//need this for carrying add function in ADC
#![feature(bigint_helper_methods)]

use std::fs;
use std::time::Duration;

use my_views::{CpuView, UltraHexaView};

mod cart;
mod cpu;
mod instr;
mod my_views;
mod nes;
mod tui;
mod wram;

use cart::Cart;
use cpu::Cpu;
use nes::NES;
use wram::Wram;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    //loading our log
    let good_log = "./test-roms/nestest-redux/nestest_cpu_relined.log";
    let log_file = fs::read_to_string(good_log).expect("log file not found");
    let mut log = log_file
        .split("\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    //we dont need to check initial state
    log.remove(0);
    //reverse the log so we can pop values from it
    log = log.into_iter().rev().collect();
    assert_ne!(log.len(), 0);

    //load our rom
    let filename = "./test-roms/nestest/nestest.nes";
    let rom_file = fs::read(filename).expect("file not found!");

    //construct our channels for sending data to the tui
    //our channel for log data between threads
    let (log_tx, log_rx): (Sender<String>, Receiver<String>) = channel();
    //our channel for sending and receiving any updates to cpu memory(wram)
    //let (mem_tx, mem_rx): (Sender<(usize, u8)>, Receiver<(usize, u8)>) = channel();
    //our channnel for sending and recieving cpu snapshots AS A SIMPLE STRING
    //let (cpu_tx, cpu_rx): (Sender<Vec<String>>, Receiver<Vec<String>>) = channel();

    //our multithreading model now consists of two parts
    //a channel for passing a reference to our NES
    let (nes_tx, nes_rx): (Sender<NES>, Receiver<NES>) = channel();
    //and a boolean predicate condvar pair
    let pair = Arc::new((Mutex::new(false), Condvar::new(), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    //make our cpu :D
    let mut cpu = Cpu::new();
    //make our wram
    let wram = Wram::new();
    //make our "cart"
    let cart = Cart::new(rom_file[0x10..=0x400f].to_vec());
    //set PC to 0xc000
    cpu.PC = 0xc000;

    //declare our channels
    /*let channels = nes::Channels {
        log_channel: log_tx,
        cpu_channel: cpu_tx,
        wram_channel: mem_tx,
    };*/

    //make our full system
    let mut nes = NES::new(cpu, cart, wram, log_tx, nes_tx);
    nes.add_breakpoint(0xC000);

    let mut tui = crate::tui::setup_tui(&mut nes);

    thread::Builder::new()
        .name("runner".to_string())
        .spawn(move || nes.run(log, pair2))
        .unwrap();

    let mut cur_nes: NES = tui
        .with_user_data(|s: &mut crate::tui::AppState| s.nes_state.clone())
        .unwrap();

    loop {
        //only do our draw loop if the appstate is NOT running
        if !tui
            .with_user_data(|s: &mut crate::tui::AppState| s.is_running)
            .unwrap()
        {
            //get all pending log lines and append them to the buffer view
            let mut pending_logs: Vec<String> = log_rx.try_iter().collect();
            tui.call_on_name("log", |view: &mut crate::my_views::BufferView| {
                view.update(&mut pending_logs)
            });

            //cpu
            tui.call_on_name("cpu", |view: &mut CpuView| {
                view.update(cur_nes.cpu.fmt_for_tui())
            });
            //wram
            tui.call_on_name("wram", |view: &mut UltraHexaView| {
                view.set_data(&mut cur_nes.wram.contents.to_vec());
            });
            //ppu
            //apu

            //refresh the view
            let _tui_event_received = tui.step();
            tui.refresh();
        } else {
            //set the predicate to be true so the system begins running
            let mut is_running = pair.0.lock().unwrap();
            *is_running = true;
            drop(is_running);
            //also let the system know that it can start running
            pair.2.notify_all();

            //halt until condvar 1 receives a notification and running is false, meaning we can debug
            let _guard = pair
                .1
                .wait_while(pair.0.lock().unwrap(), |nes_running| *nes_running)
                .unwrap();
            //now we have returned to this thread
            tui.with_user_data(|s: &mut crate::tui::AppState| s.is_running = true);
        }

        //as soon as we get a wakeup, block until we receive a new system state to inspect
        /*let nes_state = nes_rx.try_recv().unwrap();
        tui.with_user_data(|s: &mut crate::tui::AppState| {
            s.nes_state = nes_state.clone();
            cur_nes = nes_state;
        });*/
    }
}
