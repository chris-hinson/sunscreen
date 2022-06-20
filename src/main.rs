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

    //our channel for log data between threads
    let (log_tx, log_rx): (Sender<String>, Receiver<String>) = channel();

    //our multithreading model now consists of two parts
    //a channel for passing an NES snapshot
    let (nes_tx, nes_rx): (Sender<NES>, Receiver<NES>) = channel();
    //and a boolean predicate condvar pair
    let pair = Arc::new((Mutex::new(false), Condvar::new(), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    let lock = &pair.0;
    let tui_condvar = &pair.1;
    let nes_condvar = &pair.2;

    //make our cpu :D
    let mut cpu = Cpu::new();
    //make our wram
    let wram = Wram::new();
    //make our "cart"
    let cart = Cart::new(rom_file[0x10..=0x400f].to_vec());
    //set PC to 0xc000
    cpu.PC = 0xc000;

    //make our full system
    let mut nes = NES::new(cpu, cart, wram, log_tx, nes_tx);
    nes.add_breakpoint(0xC000);
    //nes.add_breakpoint(0xc002);

    let mut tui = crate::tui::setup_tui(&mut nes);

    thread::Builder::new()
        .name("runner".to_string())
        .spawn(move || nes.run(log.clone(), pair2))
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
            let mut running = lock.lock().unwrap();
            *running = true;
            drop(running);
            //also let the system know that it can start running
            nes_condvar.notify_all();

            //halt until condvar 1 receives a notification and running is false, meaning we can debug
            let guard = tui_condvar
                .wait_while(lock.lock().unwrap(), |nes_running| *nes_running)
                .unwrap();
            drop(guard);
            drop(lock);

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
