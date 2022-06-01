//need this for carrying add function in ADC
#![feature(bigint_helper_methods)]

use std::fs;

use pretty_assertions::{assert_eq, assert_ne};

mod cart;
mod cpu;
mod instr;
mod nes;
use cart::Cart;
use cpu::Cpu;
use nes::NES;

fn main() {
    //loading our log
    let good_log = "./test-roms/nestest-redux/nestest_cpu_relined.log";
    let log_file = fs::read_to_string(good_log).expect("log file not found");
    let mut log = log_file.split("\n").collect::<Vec<&str>>();
    //we dont need to check initial state
    log.remove(0);

    //load our rom
    let filename = "./test-roms/nestest/nestest.nes";
    let rom_file = fs::read(filename).expect("file not found!");

    //make our cpu :D
    let mut cpu = Cpu::new();
    //make our "cart"
    let cart = Cart::new(rom_file[0x10..=0x400f].to_vec());
    //set PC to 0xc000
    cpu.PC = 0xc000;

    //make our full system
    let mut nes = NES::new(cpu, cart);
    /*println!(
        "intial state (following reset vector)           {} CYC:{}",
        nes.cpu, nes.cycles
    );*/

    //run one step of our system
    for line in log {
        assert_eq!(nes.step(), line);
    }
}
