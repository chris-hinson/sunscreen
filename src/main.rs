mod cpu;
use cpu::Cpu;
mod cart;
use cart::Cart;

use std::fs;

fn main() {
    println!("rust emu go brrrr");

    //beep boop we just want to pass nestest
    let filename = "./test-roms/nestest/nestest.nes";
    let rom_file = fs::read(filename).expect("file not found!");

    //make our cpu :D
    let cpu = Cpu::new();
    //make our "cart"
    let cart = Cart::new(rom_file[0x10..0x400f].to_vec());
}
