//need this for carrying add function in ADC
#![feature(bigint_helper_methods)]
#![feature(mixed_integer_ops)]
use std::fs;

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

    //load our rom
    let filename = "./test-roms/nestest/nestest.nes";
    let rom_file = fs::read(filename).expect("file not found!");

    //make our cpu :D
    let mut cpu = Cpu::new();
    //set PC to 0xc000
    cpu.PC = 0xc000;
    //make our wram
    let wram = Wram::new();
    //make our "cart"
    let cart = Cart::new(rom_file[0x10..=0x400f].to_vec());
    //make our ppu
    let ppu = Ppu::new(cart);

    //make our full system and add a breakpoint at the test rom entry address
    let mut nes = NES::new(cpu, wram, ppu);
    nes.add_breakpoint(0xC000);

    //nes.add_breakpoint(0xC689);
    //nes.add_breakpoint(0xC6C8);

    //nes.add_breakpoint(0xC5FD);

    let runner_handle = thread::Builder::new()
        .name("runner".to_string())
        .spawn(move || nes.run(log.clone()))
        .unwrap();

    let window_handle = thread::Builder::new()
        .name("app".to_string())
        .spawn(move || crate::app::run())
        .unwrap();

    runner_handle.join().expect("runner thread panicked");
    window_handle.join().expect("app window panicked?").unwrap();
}
