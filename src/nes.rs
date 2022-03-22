use crate::cart::Cart;
use crate::cpu::Cpu;

pub struct NES {
    cycles: u128,
    cpu: Cpu,
    cart: Cart,
    //apu
    //ppu
    //VRAM
}

impl NES {
    pub fn new(cpu: Cpu, cart: Cart) -> NES {
        NES {
            cpu,
            cart,
            cycles: 7,
        }
    }

    pub fn step(&mut self) {
        let instr: u8 = self.cpu.read(self.cpu.PC, &mut self.cart);

        //simulates full opcode space, including illegal instructions
        match instr {
            0x00 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x01 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x02 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x03 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x04 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x05 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x06 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x07 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x08 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x09 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x0A => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x0B => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x0C => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x0D => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x0E => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x0F => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0x10 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x11 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x12 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x13 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x14 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x15 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x16 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x17 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x18 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x19 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x1A => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x1B => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x1C => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x1D => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x1E => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x1F => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0x20 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x21 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x22 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x23 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x24 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x25 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x26 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x27 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x28 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x29 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x2A => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x2B => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x2C => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x2D => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x2E => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x2F => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0x30 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x31 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x32 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x33 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x34 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x35 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x36 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x37 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x38 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x39 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x3A => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x3B => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x3C => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x3D => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x3E => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x3F => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0x40 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x41 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x42 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x43 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x44 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x45 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x46 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x47 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x48 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x49 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x4A => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x4B => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x4C => {
                //panic!("unimplemented op {:#02x}", instr)
                let new_addr_lo = self.cpu.read(self.cpu.PC + 1, &mut self.cart);
                let new_addr_hi = self.cpu.read(self.cpu.PC + 2, &mut self.cart);
                let new_addr: u16 = (new_addr_hi as u16) << 8 | new_addr_lo as u16;
                println!(
                    "{:04X}  {} {:02x} {:02X}  JMP ${:04X}",
                    self.cpu.PC, "4C", new_addr_lo, new_addr_hi, new_addr
                );
                self.cpu.PC = new_addr;
                self.cycles += 3;
            }
            0x4D => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x4E => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x4F => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0x50 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x51 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x52 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x53 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x54 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x55 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x56 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x57 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x58 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x59 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x5A => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x5B => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x5C => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x5D => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x5E => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x5F => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0x60 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x61 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x62 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x63 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x64 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x65 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x66 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x67 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x68 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x69 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x6A => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x6B => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x6C => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x6D => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x6E => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x6F => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0x70 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x71 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x72 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x73 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x74 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x75 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x76 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x77 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x78 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x79 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x7A => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x7B => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x7C => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x7D => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x7E => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x7F => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0x80 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x81 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x82 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x83 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x84 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x85 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x86 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x87 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x88 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x89 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x8A => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x8B => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x8C => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x8D => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x8E => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x8F => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0x90 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x91 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x92 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x93 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x94 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x95 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x96 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x97 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x98 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x99 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x9A => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x9B => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x9C => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x9D => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x9E => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0x9F => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0xA0 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xA1 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xA2 => {
                //imm to be loaded comes immeadiately after op in memory
                let imm = self.cpu.read(self.cpu.PC + 1, &mut self.cart);
                self.cpu.SR.Z = imm == 0;
                self.cpu.SR.N = (imm as i8) < 0;

                self.cpu.X = imm;

                println!(
                    "{:04X}  {} {:02x}     {} #${:02x}",
                    self.cpu.PC, "A2", imm, "LDX", imm
                );
                self.cpu.PC += 2;
                self.cycles += 2;
            }
            0xA3 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xA4 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xA5 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xA6 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xA7 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xA8 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xA9 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xAA => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xAB => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xAC => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xAD => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xAE => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xAF => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0xB0 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xB1 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xB2 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xB3 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xB4 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xB5 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xB6 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xB7 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xB8 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xB9 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xBA => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xBB => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xBC => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xBD => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xBE => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xBF => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0xC0 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xC1 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xC2 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xC3 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xC4 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xC5 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xC6 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xC7 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xC8 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xC9 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xCA => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xCB => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xCC => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xCD => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xCE => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xCF => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0xD0 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xD1 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xD2 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xD3 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xD4 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xD5 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xD6 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xD7 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xD8 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xD9 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xDA => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xDB => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xDC => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xDD => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xDE => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xDF => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0xE0 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xE1 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xE2 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xE3 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xE4 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xE5 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xE6 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xE7 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xE8 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xE9 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xEA => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xEB => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xEC => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xED => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xEE => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xEF => {
                panic!("unimplemented op {:#02x}", instr)
            }

            0xF0 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xF1 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xF2 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xF3 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xF4 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xF5 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xF6 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xF7 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xF8 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xF9 => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xFA => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xFB => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xFC => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xFD => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xFE => {
                panic!("unimplemented op {:#02x}", instr)
            }
            0xFF => {
                panic!("unimplemented op {:#02x}", instr)
            }
        }
    }
}
