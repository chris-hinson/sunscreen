use crate::cart::Cart;
use std::collections::HashMap;
use std::fmt;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Cpu {
    pub PC: u16,
    pub ACC: u8,
    pub X: u8,
    pub Y: u8,
    pub SR: SR,
    pub SP: u8,
    //TODO: should this be part of the cpu or should we put it somewhere else?
    WRAM: [u8; 2048],
}

impl fmt::Display for Cpu {
    //A:00 X:00 Y:00 P:  N:0 V:0 B:10 D:0 I:1 Z:0 C:0  SP:FD  CYC:7
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "A:{:02X} X:{:02X} Y:{:02X} P:  N:{} V:{} B:{}{} D:{} I:{} Z:{} C:{}  SP:{:02X} ",
            self.ACC,
            self.X,
            self.Y,
            self.SR.N as i32,
            self.SR.V as i32,
            self.SR.BH as i32,
            self.SR.BL as i32,
            self.SR.D as i32,
            self.SR.I as i32,
            self.SR.Z as i32,
            self.SR.C as i32,
            self.SP
        )
    }
}

#[allow(non_snake_case)]
#[derive(Debug)]
//status register struct
pub struct SR {
    pub N: bool,
    pub V: bool,
    pub BH: bool,
    pub BL: bool,
    pub D: bool,
    pub I: bool,
    pub Z: bool,
    pub C: bool,
}

//functions to encode and decode the SR
impl SR {
    //turn the struct into a u8
    pub fn decode(&self) -> u8 {
        let mut ret_field: u8 = 0b00000000;
        if self.N {
            ret_field |= 0x1 << 7
        };
        if self.V {
            ret_field |= 0x1 << 6
        };
        if self.BH {
            ret_field |= 0x1 << 5;
        }
        if self.BL {
            ret_field |= 0x1 << 4;
        };
        if self.D {
            ret_field |= 0x1 << 3;
        };
        if self.I {
            ret_field |= 0x1 << 2;
        };
        if self.Z {
            ret_field |= 0x1 << 1;
        };
        if self.C {
            ret_field |= 0x1;
        };

        ret_field
    }
    //turn a u8 into an SR
    pub fn encode(&mut self, val: u8) {
        self.N = (val & 0b1000_0000) >> 7 == 0x1;
        self.V = (val & 0b0100_0000) >> 6 == 0x1;
        self.BH = (val & 0b0010_0000) >> 5 == 0x1;
        self.BL = (val & 0b0001_0000) >> 4 == 0x1;
        self.D = (val & 0b0000_1000) >> 3 == 0x1;
        self.I = (val & 0b0000_0100) >> 2 == 0x1;
        self.Z = (val & 0b0000_0010) >> 1 == 0x1;
        self.C = (val & 0b0000_0001) >> 0 == 0x1;
    }
    fn new() -> Self {
        SR {
            N: false,
            V: false,
            BH: false,
            BL: false,
            D: false,
            I: false,
            Z: false,
            C: false,
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        let mut our_cpu = Cpu {
            PC: 0x0,
            ACC: 0x0,
            X: 0x0,
            Y: 0x0,
            SR: SR::new(),
            SP: 0xFD,
            WRAM: [0; 2048],
        };

        our_cpu.SR.encode(0b0010_0100);
        return our_cpu;
    }

    //memory operations

    /*CPU Memory Map (16bit buswidth, 0-FFFFh)

    0000h-07FFh   Internal 2K Work RAM (mirrored to 800h-1FFFh)
    2000h-2007h   Internal PPU Registers (mirrored to 2008h-3FFFh)
    4000h-4017h   Internal APU Registers
    4018h-5FFFh   Cartridge Expansion Area almost 8K
    6000h-7FFFh   Cartridge SRAM Area 8K
    8000h-FFFFh   Cartridge PRG-ROM Area 32K*/

    //so when the cpu reads or writes to an address, these functions should dispatch the rw to
    //the appropriate part

    //TODO: to handle reads to other parts of the system, we must pass in mutable refrences to every other component
    //read n bytes from address a
    pub fn read(&mut self, addr: u16, cart: &mut Cart, length: usize) -> Vec<u8> {
        match addr {
            //WRAM(2kb) + 3 mirrors
            0x0000..=0x1FFF => {
                //mod by 2048 since we have 3 mirrors
                let final_addr = addr % 2048;

                return self.WRAM[final_addr as usize..final_addr as usize + length].into();
                //return 0;
            }
            //PPU control regs a PM at gs (8 bytes) + a fuckton of mirrors
            0x2000..=0x3FFF => {
                return [].into();
            }
            //registers (apu and io)
            0x4000..=0x4017 => {
                return [].into();
            }
            //cart expansion
            0x4018..=0x5FFF => {
                return [].into();
            }
            //cart SRAM (8k)
            0x6000..=0x7FFF => {
                return [].into();
            }
            //PRG-ROM (32K)
            0x8000..=0xFFFF => cart.read(addr, length).into(),
        }
    }
    pub fn write(&mut self, addr: u16, byte: u8) {
        match addr {
            //WRAM(2kb) + 3 mirrors
            0x0000..=0x1FFF => {
                let final_addr = addr % 2048;
                self.WRAM[final_addr as usize] = byte;
                //return 0;
            }
            //PPU control regs (8 bytes) + a fuckton of mirrors
            0x2000..=0x3FFF => {}
            //registers (apu and io)
            0x4000..=0x4017 => {}
            //cart expansion
            0x4018..=0x5FFF => {}
            //cart SRAM (8k)
            0x6000..=0x7FFF => {}
            //PRG-ROM (32K)
            0x8000..=0xFFFF => {}
        }
    }
}
