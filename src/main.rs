//need this for carrying add function in ADC
#![feature(bigint_helper_methods)]

use std::fs;

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
    //reverse the log so we can pop values from it
    log = log.into_iter().rev().collect();
    log.remove(0);

    //load our rom
    let filename = "./test-roms/nestest/nestest.nes";
    let rom_file = fs::read(filename).expect("file not found!");

    //make our cpu :D
    let mut cpu = Cpu::new();
    //make our wram
    let wram = Wram::new();
    //make our "cart"
    let cart = Cart::new(rom_file[0x10..=0x400f].to_vec());
    //set PC to 0xc000
    cpu.PC = 0xc000;

    //make our full system and add a breakpoint at the test rom entry address
    let mut nes = NES::new(cpu, cart, wram);
    nes.add_breakpoint(0xC000);

    let runner_handle = thread::Builder::new()
        .name("runner".to_string())
        .spawn(move || nes.run(log.clone()))
        .unwrap();

    runner_handle.join().expect("runner thread panicked");
}
