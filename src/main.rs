//need this for carrying add function in ADC
#![feature(bigint_helper_methods)]

use std::fs;
use std::time::Duration;

use my_views::CpuView;

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

    thread::spawn(move || nes.run(log, pair2));

    //NOTE: you will need this lifetime for the sdl loop. adding it lets us break out of our main program loop
    //'running: loop {
    let mut tui_running = true;
    loop {
        /*let mut step_running = false;
        tui.with_user_data(|s: &mut crate::tui::AppState| {
            if s.is_running {
                step_running = true;
            }
        });*/

        //only render cpu data if we are CURRENTLY IN A BREAK STATE
        //if step_running {
        //get all pending log lines and append them to the buffer view
        let mut pending_logs: Vec<String> = log_rx.try_iter().collect();
        tui.call_on_name("log", |view: &mut crate::my_views::BufferView| {
            view.update(&mut pending_logs)
        });

        /*
        //we will only be getting a cpu snapshot if we are halted
        match cpu_rx.try_recv() {
            Ok(v) => tui
                .call_on_name("cpu", |view: &mut CpuView| view.update(v))
                .unwrap(),
            Err(_e) => {}
        }

        //read any pending ram updates into a vector
        let pending_vram_data: Vec<(usize, u8)> = mem_rx.try_iter().collect();
        tui.call_on_name("ram_view", |view: &mut crate::my_views::UltraHexaView| {
            view.update_data(pending_vram_data);
        });*/
        //TODO: we also need to get -
        //ppu data
        //apu data
        //}

        let _tui_event_received = tui.step();
        tui.refresh();

        //if our local running flag has been set to false,
        if !tui_running {
            //halt until our runner thread gives us the ok
            let _guard = pair
                .1
                .wait_while(pair.0.lock().unwrap(), |pending| *pending)
                .unwrap();
            tui_running = true;

            //as soon as we get a wakeup, block until we receive a new system state to inspect
            /*let nes_state = nes_rx.recv_timeout(Duration::new(1, 0)).unwrap();
            tui.with_user_data(|s: &mut crate::tui::AppState| {
                s.nes_state = nes_state;
            });*/
        } else {
            //make sure we havent gotten a resume callback, before we go to next main loop iteration

            tui.with_user_data(|s: &mut crate::tui::AppState| {
                if !s.is_running {
                    tui_running = false;
                } else {
                    let mut running = pair.0.lock().unwrap();
                    *running = true;
                    drop(running);
                    pair.2.notify_all();
                }
            });
        }
    }
}
