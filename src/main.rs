//need this for carrying add function in ADC
#![feature(bigint_helper_methods)]
#![feature(mixed_integer_ops)]
use std::{
    fs,
    sync::mpsc::{channel, Receiver, Sender},
};

mod app;
mod bus;
mod cart;
mod cpu;
mod instr;
mod my_views;
mod nes;
mod ppu;
mod tui;
mod vram;
mod wram;

use cart::Cart;
use cpu::Cpu;
use nes::NES;
use ppu::Ppu;
use wram::Wram;

//use pretty_assertions::Comparison;
use std::thread;

fn main() {
    //loading our log
    let good_log = "./test-roms/nestest-redux/nestest_cpu_relined.log";
    let log_file = fs::read_to_string(good_log).expect("log file not found");
    let mut log = log_file
        .split('\n')
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    //we dont need to check initial state
    //reverse the log so we can pop values from it
    log = log.into_iter().rev().collect();
    log.remove(0);

    //make our cpu :D
    let cpu = Cpu::new();

    //make our wram
    let wram = Wram::new();
    //make our "cart"
    let filename = "./test-roms/nestest/nestest.nes";
    let cart = Cart::new(filename);
    //ppu and app need a channel to send frame data
    let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = channel();
    //make our ppu
    let ppu = Ppu::new(cart, tx);

    //make our full system and add a breakpoint at the test rom entry address
    let mut nes = NES::new(cpu, wram, ppu);
    let reset_addr = nes.ppu.cart.cpu_read(0xFFFC, 2);
    nes.cpu.PC = reset_addr[0] as u16 | (reset_addr[1] as u16) << 8;
    //panic!("reset addr is {:04X}", nes.cpu.PC);
    nes.add_breakpoint(nes.cpu.PC as usize);
    //nes.add_breakpoint(0xC689);
    //nes.add_breakpoint(0xC6C8);
    //nes.add_breakpoint(0xC5FD);

    let runner_handle = thread::Builder::new()
        .name("runner".to_string())
        .spawn(move || nes.run(log.clone()))
        .unwrap();

    let window_handle = thread::Builder::new()
        .name("app".to_string())
        .spawn(move || crate::app::run(rx))
        .unwrap();

    runner_handle.join().expect("runner thread panicked");
    window_handle.join().expect("app window panicked?").unwrap();
}
