use crate::cart::Cart;

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
//status register struct
pub struct SR {
    pub N: bool,
    pub V: bool,
    pub NA: bool,
    pub B: bool,
    pub D: bool,
    pub I: bool,
    pub Z: bool,
    pub C: bool,
}

//functions to encode and decode the SR
impl SR {
    //turn the struct into a u8
    fn decode(&self) -> u8 {
        let mut ret_field: u8 = 0b00000000;
        if self.N {
            ret_field |= 0x1 << 7
        };
        if self.V {
            ret_field |= 0x1 << 6
        };
        //dont touch the NA reg
        if self.B {
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
    fn encode(&mut self, val: u8) {
        self.N = (val & 0b1000_0000) >> 7 == 0x1;
        self.V = (val & 0b0100_0000) >> 6 == 0x1;
        //dont touch the NA
        self.B = (val & 0b0001_0000) >> 4 == 0x1;
        self.D = (val & 0b0000_1000) >> 3 == 0x1;
        self.I = (val & 0b0000_0100) >> 2 == 0x1;
        self.Z = (val & 0b0000_0010) >> 1 == 0x1;
        self.C = (val & 0b0000_0001) >> 0 == 0x1;
    }
    fn new() -> Self {
        SR {
            N: false,
            V: false,
            NA: false,
            B: false,
            D: false,
            I: false,
            Z: false,
            C: false,
        }
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            PC: 0x0,
            ACC: 0x0,
            X: 0x0,
            Y: 0x0,
            SR: SR::new(),
            SP: 0x0,
            WRAM: [0; 2048],
        }
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
    pub fn read(&mut self, addr: u16, cart: &mut Cart) -> u8 {
        match addr {
            //WRAM(2kb) + 3 mirrors
            0x0000..=0x1FFF => {
                let final_addr = addr % 2048;
                self.WRAM[final_addr as usize]
                //return 0;
            }
            //PPU control regs (8 bytes) + a fuckton of mirrors
            0x2000..=0x3FFF => {
                return 0;
            }
            //registers (apu and io)
            0x4000..=0x4017 => {
                return 0;
            }
            //cart expansion
            0x4018..=0x5FFF => {
                return 0;
            }
            //cart SRAM (8k)
            0x6000..=0x7FFF => {
                return 0;
            }
            //PRG-ROM (32K)
            0x8000..=0xFFFF => cart.read(addr),
        }
    }
    fn write(&mut self, addr: u16, byte: u8) {
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
