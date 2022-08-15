use core::panic;
use std::fs;

#[derive(Clone)]
pub struct Cart {
    //technically the cart may contain literally anything, but these are the three most common things
    pub prg_rom: Vec<u8>,
    pub prg_ram: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

//TODO: remove me when we implement not terrible carts lmao
//NROM128 for now?

//NOTE: we should turn this into a trait at some point. a useful abstraction of what
//a cart really is is simple a cpu read/write mapping, and a ppu read/write mapping
#[allow(dead_code)]
impl Cart {
    pub fn new(filename: &str) -> Self {
        //TODO: finish actually parsing ines!!

        //load our rom
        let rom_raw = fs::read(filename).expect("file not found!");

        let header = rom_raw[0..16].to_vec();
        //size of prg_rom in 16kb units
        let prg_rom_size = header[4];
        //size of chr_rom in 8kb units
        let chr_rom_size = header[5];

        let flags_6 = header[6];

        let prg_rom = if (flags_6 & 0b0000_0100) != 0 {
            //we have a trainer
            panic!("this rom contains a trainer and we dont know how to deal with that yet");
        } else {
            rom_raw[17..(16 + prg_rom_size as usize * 16384) as usize].to_vec()
        };

        let chr_rom = if (flags_6 & 0b0000_0100) != 0 {
            //we have a trainer
            panic!("this rom contains a trainer and we dont know how to deal with that yet");
        } else {
            rom_raw[(16 + prg_rom_size as usize * 16384) + 1 as usize..].to_vec()
        };

        /*let mut init_contents: Vec<u8> = vec![0; 16384];
        for (i, v) in bank0.iter().enumerate() {
            init_contents[i] = *v;
        }*/
        Cart {
            //prg_rom: bank0,
            prg_rom: prg_rom,
            prg_ram: vec![0; 2048],
            chr_rom: vec![0; 8192],
        }
    }

    pub fn cpu_read(&mut self, addr: u16, length: usize) -> Vec<u8> {
        //two mirrors of 16KB each
        let eff_addr = addr % 16384;
        self.prg_rom[eff_addr as usize..eff_addr as usize + length].into()
    }
    pub fn cpu_write(&mut self, addr: u16, byte: u8) {
        let eff_addr = addr % 16384;
        //self.prg_rom[(addr - 0xc000) as usize] = byte;
        self.prg_rom[eff_addr as usize] = byte;
    }

    pub fn ppu_read(&mut self, addr: u16, length: usize) -> Vec<u8> {
        return self.chr_rom[addr as usize..addr as usize + length].to_vec();
    }
    pub fn ppu_write(&mut self, addr: u16, byte: u8) {
        self.chr_rom[addr as usize] = byte;
    }
}
