use std::fs;

mod cart;
mod cpu;
mod nes;
use cart::Cart;
use cpu::Cpu;
use nes::NES;

fn main() {
    println!("rust emu go brrrr");

    //beep boop we just want to pass nestest
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

    //run one step of our system
    nes.step();
    nes.step();
}
