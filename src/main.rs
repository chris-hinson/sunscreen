//need this for carrying add function in ADC
#![feature(bigint_helper_methods)]

use std::fs;

use my_views::CpuView;
use pretty_assertions::{assert_eq, assert_ne};

mod cart;
mod cpu;
mod instr;
mod my_views;
mod nes;
mod tui;

use cart::Cart;
use cpu::Cpu;
use nes::NES;

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
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
    //our channel for sending and receiving any updates to cpu memory
    let (mem_tx, mem_rx): (Sender<(usize, u8)>, Receiver<(usize, u8)>) = channel();
    //our channnel for sending and recieving cpu snapshots AS A SIMPLE STRING
    let (cpu_tx, cpu_rx): (Sender<Vec<String>>, Receiver<Vec<String>>) = channel();

    //make our cpu :D
    let mut cpu = Cpu::new(mem_tx);
    //make our "cart"
    let cart = Cart::new(rom_file[0x10..=0x400f].to_vec());
    //set PC to 0xc000
    cpu.PC = 0xc000;

    //declare our channels
    let channels = nes::Channels {
        log_channel: log_tx,
        cpu_channel: cpu_tx,
    };

    //make our full system
    let mut nes = NES::new(cpu, cart, channels);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    let mut tui = crate::tui::setup_tui(&mut nes);

    ///////////////////////////////////////////////////////////////////////////////////////////////

    thread::spawn(move || nes.run(log));

    'running: loop {
        let mut step_running = false;
        tui.with_user_data(|s: &mut crate::tui::AppState| {
            if s.is_running {
                step_running = true;
            }
        });

        //only actually do stuff if we are currently running
        if step_running {
            //get all pending log lines and append them to the buffer view
            let mut pending_logs: Vec<String> = log_rx.try_iter().collect();
            tui.call_on_name("log", |view: &mut crate::my_views::BufferView| {
                view.update(&mut pending_logs)
            });

            //we will only be getting a cpu snapshot if we are halted
            match cpu_rx.try_recv() {
                Ok(v) => tui
                    .call_on_name("cpu", |view: &mut CpuView| view.update(v))
                    .unwrap(),
                Err(e) => {}
            }

            //read any pending ram updates into a vector
            let pending_vram_data: Vec<(usize, u8)> = mem_rx.try_iter().collect();
            tui.call_on_name("ram_view", |view: &mut crate::my_views::UltraHexaView| {
                view.update_data(pending_vram_data);
            });
            //TODO: we also need to get -
            //ppu data
            //apu data
        }

        let _tui_event_received = tui.step();
        tui.refresh();
    }
}
