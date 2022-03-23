use crate::cart::Cart;
use crate::cpu::Cpu;

pub struct NES {
    pub cycles: u128,
    pub cpu: Cpu,
    pub cart: Cart,
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
        //println!("fetching {:04X}, found {instr:02X}", self.cpu.PC);

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
                //push processor status onto stack

                //php puts 11 in na/b
                let mut push_val = self.cpu.SR.decode();
                push_val |= 0b11 << 4;

                self.cpu.write(self.cpu.SP as u16, push_val);

                println!(
                    "{:04X}  08        PHP                             {}             CYC:{}",
                    self.cpu.PC, self.cpu, self.cycles
                );

                self.cpu.SP -= 1;

                self.cycles += 3;
                self.cpu.PC += 1;
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
                //branch on res plus (N = 0)
                //relative addressing

                //add 1 to cycles if branch occurs on same page
                //add 2 to cycles if branch occurs to different page

                let targ_offset = self.cpu.read(self.cpu.PC + 1, &mut self.cart);
                //target addr is the offsett plus the width of the instruction(2)
                let targ_addr = self.cpu.PC as i16 + (targ_offset as i16) + 2;
                println!("{:04X}  10 {targ_offset:02X}     BPL ${targ_addr:4X}                       {}             CYC:{}",self.cpu.PC,self.cpu,self.cycles);

                if !self.cpu.SR.N {
                    let boundary_change = self.cpu.PC % 256 != (targ_addr % 256) as u16;
                    self.cpu.PC = targ_addr as u16;
                    if boundary_change {
                        self.cycles += 3
                    } else {
                        self.cycles += 2
                    };
                } else {
                    self.cpu.PC += 2;
                    self.cycles += 2;
                }
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
                //clear carry
                println!(
                    "{:04X}  18        CLC                             {}             CYC:{}",
                    self.cpu.PC, self.cpu, self.cycles
                );

                self.cpu.SR.C = false;
                self.cpu.PC += 1;
                self.cycles += 2;
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
                //JSR jump and save return on stack
                let target_lo = self.cpu.read(self.cpu.PC + 1, &mut self.cart);
                let target_hi = self.cpu.read(self.cpu.PC + 2, &mut self.cart);
                let target_addr: u16 = (target_hi as u16) << 8 | target_lo as u16;
                println!("{:04X}  20 {target_lo:2X} {target_hi:2X}  JSR ${target_addr:4X}                       {}             CYC:{}",self.cpu.PC,self.cpu,self.cycles);

                //push ret addr
                let ret_addr = self.cpu.PC + 2;
                self.cpu.write(self.cpu.SP as u16, ret_addr as u8);
                self.cpu
                    .write((self.cpu.SP + 1) as u16, (ret_addr >> 8) as u8);
                self.cpu.SP -= 0x2;

                //jump
                self.cpu.PC = target_addr;

                self.cycles += 6;
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
                //test bits in memory with accumulator
                //zeropage

                let addr = self.cpu.read(self.cpu.PC + 1, &mut self.cart);
                let value = self.cpu.read(addr as u16, &mut self.cart);

                println!("{:04X}  24 {addr:02X}     BIT ${addr:02X} = {value:02X}                    {}             CYC:{}",self.cpu.PC,self.cpu,self.cycles);

                /*println!("value is: {value:08b}");
                println!("bit 7 is: {}", ((value & 0b1000_0000) >> 7) == 0x1);
                println!("bit 6 is: {}", ((value & 0b0100_0000) >> 6) == 0x1);*/

                //set zero flag if acc AND val =self.cpu.SP += 0x2; 0
                self.cpu.SR.Z = self.cpu.ACC & value == 0x0;

                //set bits 7 and 6 of SR to bits 7 and 6 of value read
                self.cpu.SR.N = ((value & 0b1000_0000) >> 7) == 0x1;
                self.cpu.SR.V = ((value & 0b0100_0000) >> 6) == 0x1;

                self.cycles += 3;
                self.cpu.PC += 2;
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
                //AND mem with acc
                //imm addressing
                let imm = self.cpu.read((self.cpu.PC + 1) as u16, &mut self.cart);

                println!(
                    "{:04X}  29 {imm:02X}     AND #${imm:02X}                        {}             CYC:{}",
                    self.cpu.PC, self.cpu, self.cycles
                );

                let result = imm & self.cpu.ACC;
                self.cpu.SR.Z = result == 0;
                self.cpu.SR.N = (result as i8) < 0;

                self.cpu.ACC = result;

                self.cpu.PC += 2;
                self.cycles += 2;
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
                //SEC set carry

                println!(
                    "{:04X}  38        SEC                             {}             CYC:{}",
                    self.cpu.PC, self.cpu, self.cycles
                );

                self.cpu.SR.C = true;

                self.cpu.PC += 1;
                self.cycles += 2;
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
                //JMP absolute
                let new_addr_lo = self.cpu.read(self.cpu.PC + 1, &mut self.cart);
                let new_addr_hi = self.cpu.read(self.cpu.PC + 2, &mut self.cart);
                let new_addr: u16 = (new_addr_hi as u16) << 8 | new_addr_lo as u16;
                println!(
                    "{:04X}  {} {:02X} {:02X}  JMP ${:04X}                       {}             CYC:{}",
                    self.cpu.PC, "4C", new_addr_lo, new_addr_hi, new_addr, self.cpu, self.cycles
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
                //branch on overflow clear
                //relative addressing

                //add 1 to cycles if branch occurs on same page
                //add 2 to cycles if branch occurs to different page

                let targ_offset = self.cpu.read(self.cpu.PC + 1, &mut self.cart);
                //target addr is the offsett plus the width of the instruction(2)
                let targ_addr = self.cpu.PC as i16 + (targ_offset as i16) + 2;
                println!("{:04X}  50 {targ_offset:02X}     BVC ${targ_addr:4X}                       {}             CYC:{}",self.cpu.PC,self.cpu,self.cycles);

                if !self.cpu.SR.V {
                    let boundary_change = self.cpu.PC % 256 != (targ_addr % 256) as u16;
                    self.cpu.PC = targ_addr as u16;
                    if boundary_change {
                        self.cycles += 3
                    } else {
                        self.cycles += 2
                    };
                } else {
                    self.cpu.PC += 2;
                    self.cycles += 2;
                }
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
                //return from subroutine

                //NOTE: BECAUSE WE ARE READING OFF THE TOP OF THE STACK WE ACTUALLY READ 3 AND 2 BYTES ABOVE THE SP SINCE ITS POINTING AT EMPTY SPACE
                let pc_lo = self.cpu.read((self.cpu.SP + 2) as u16, &mut self.cart);
                let pc_hi = self.cpu.read((self.cpu.SP + 3) as u16, &mut self.cart);
                let pc = (pc_hi as u16) << 8 | pc_lo as u16;
                //println!("popped PC: {pc:04X}");

                println!(
                    "{:04X}  60        RTS                             {}             CYC:{}",
                    self.cpu.PC, self.cpu, self.cycles
                );

                self.cpu.SP += 0x2;

                self.cpu.PC = pc;
                self.cpu.PC += 1;
                self.cycles += 6;
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
                //pull acc from stack

                println!(
                    "{:04X}  68        PLA                             {}             CYC:{}",
                    self.cpu.PC, self.cpu, self.cycles
                );

                self.cpu.SP += 1;
                let pulled_val = self.cpu.read(self.cpu.SP as u16, &mut self.cart);

                self.cpu.SR.N = (pulled_val as i8) < 0;
                self.cpu.SR.Z = pulled_val == 0;
                self.cpu.ACC = pulled_val;

                self.cycles += 4;
                self.cpu.PC += 1;
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
                //branch on overflow set
                //relative addressing

                //add 1 to cycles if branch occurs on same page
                //add 2 to cycles if branch occurs to different page

                let targ_offset = self.cpu.read(self.cpu.PC + 1, &mut self.cart);
                //target addr is the offsett plus the width of the instruction(2)
                let targ_addr = self.cpu.PC as i16 + (targ_offset as i16) + 2;
                println!("{:04X}  70 {targ_offset:02X}     BVS ${targ_addr:4X}                       {}             CYC:{}",self.cpu.PC,self.cpu,self.cycles);

                if self.cpu.SR.V {
                    let boundary_change = self.cpu.PC % 256 != (targ_addr % 256) as u16;
                    self.cpu.PC = targ_addr as u16;
                    if boundary_change {
                        self.cycles += 3
                    } else {
                        self.cycles += 2
                    };
                } else {
                    self.cpu.PC += 2;
                    self.cycles += 2;
                }
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
                //set interrupt disable
                println!(
                    "{:04X}  78        SEI                             {}             CYC:{}",
                    self.cpu.PC, self.cpu, self.cycles
                );
                self.cpu.SR.I = true;
                self.cycles += 2;
                self.cpu.PC += 1;
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
                //STA zeropage
                let addr: u16 = self.cpu.read(self.cpu.PC + 1, &mut self.cart) as u16;
                let cur_val = self.cpu.read(addr, &mut self.cart);
                println!(
                    "{:04X}  85 {:02X}     STA ${:02X} = {:02X}                    {}             CYC:{}",
                    self.cpu.PC, addr, addr,cur_val, self.cpu, self.cycles
                );

                self.cpu.write(addr, self.cpu.ACC);

                self.cycles += 3;
                self.cpu.PC += 2;
            }
            0x86 => {
                //STX zeropage
                let addr: u16 = self.cpu.read(self.cpu.PC + 1, &mut self.cart) as u16;
                let cur_val = self.cpu.read(addr, &mut self.cart);

                println!(
                    "{:04X}  86 {:02X}     STX ${:02X} = {:02X}                    {}             CYC:{}",
                    self.cpu.PC, addr, addr, cur_val, self.cpu, self.cycles
                );

                self.cpu.write(addr, self.cpu.X);

                self.cycles += 3;
                self.cpu.PC += 2;
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
                //branch on cary clear
                //relative addressing

                //add 1 to cycles if branch occurs on same page
                //add 2 to cycles if branch occurs to different page

                let targ_offset = self.cpu.read(self.cpu.PC + 1, &mut self.cart);
                //target addr is the offsett plus the width of the instruction(2)
                let targ_addr = self.cpu.PC as i16 + (targ_offset as i16) + 2;
                println!("{:04X}  90 {targ_offset:02X}     BCC ${targ_addr:4X}                       {}             CYC:{}",self.cpu.PC,self.cpu,self.cycles);

                if !self.cpu.SR.C {
                    let boundary_change = self.cpu.PC % 256 != (targ_addr % 256) as u16;
                    self.cpu.PC = targ_addr as u16;
                    if boundary_change {
                        self.cycles += 3
                    } else {
                        self.cycles += 2
                    };
                } else {
                    self.cpu.PC += 2;
                    self.cycles += 2;
                }
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
                //LDX imm
                //imm to be loaded comes immeadiately after op in memory
                let imm = self.cpu.read(self.cpu.PC + 1, &mut self.cart);

                println!(
                    "{:04X}  {} {:02x}     {} #${:02x}                        {}             CYC:{}",
                    self.cpu.PC, "A2", imm, "LDX", imm, self.cpu, self.cycles
                );

                self.cpu.SR.Z = imm == 0;
                self.cpu.SR.N = (imm as i8) < 0;

                self.cpu.X = imm;

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
                //LDA imm
                //imm to be loaded comes immeadiately after op in memory
                let imm = self.cpu.read(self.cpu.PC + 1, &mut self.cart);

                println!(
                    "{:04X}  A9 {imm:02X}     LDA #${:02X}                        {}             CYC:{}",
                    self.cpu.PC,  imm, self.cpu, self.cycles
                );

                self.cpu.SR.Z = imm == 0;
                self.cpu.SR.N = (imm as i8) < 0;

                self.cpu.ACC = imm;

                self.cpu.PC += 2;
                self.cycles += 2;
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
                //branch on cary set
                //relative addressing

                //add 1 to cycles if branch occurs on same page
                //add 2 to cycles if branch occurs to different page

                let targ_offset = self.cpu.read(self.cpu.PC + 1, &mut self.cart);
                //target addr is the offsett plus the width of the instruction(2)
                let targ_addr = self.cpu.PC as i16 + (targ_offset as i16) + 2;
                println!("{:04X}  B0 {targ_offset:02X}     BCS ${targ_addr:4X}                       {}             CYC:{}",self.cpu.PC,self.cpu,self.cycles);

                if self.cpu.SR.C {
                    let boundary_change = self.cpu.PC % 256 != (targ_addr % 256) as u16;
                    self.cpu.PC = targ_addr as u16;
                    if boundary_change {
                        self.cycles += 3
                    } else {
                        self.cycles += 2
                    };
                } else {
                    self.cpu.PC += 2;
                    self.cycles += 2;
                }
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
                //CMP mem with acc
                let val = self.cpu.read((self.cpu.PC + 1) as u16, &mut self.cart);
                println!(
                    "{:04X}  C9 {val:02X}     CMP #${val:02X}                        {}             CYC:{}",
                    self.cpu.PC, self.cpu, self.cycles
                );

                let res = self.cpu.ACC as i8 - val as i8;

                self.cpu.SR.N = res < 0;
                self.cpu.SR.Z = res == 0;
                //we need to carry if we subtract a value greater than the acc
                self.cpu.SR.C = self.cpu.ACC <= val;

                self.cpu.PC += 2;
                self.cycles += 2;
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
                //branch on res not 0
                //branch if z flag is not set lol

                //add 1 to cycles if branch occurs on same page
                //add 2 to cycles if branch occurs to different page

                let targ_offset = self.cpu.read(self.cpu.PC + 1, &mut self.cart);
                //target addr is the offsett plus the width of the instruction(2)
                let targ_addr = self.cpu.PC as i16 + (targ_offset as i16) + 2;
                println!("{:04X}  D0 {targ_offset:02X}     BNE ${targ_addr:4X}                       {}             CYC:{}",self.cpu.PC,self.cpu,self.cycles);

                if !self.cpu.SR.Z {
                    let boundary_change = self.cpu.PC % 256 != (targ_addr % 256) as u16;
                    self.cpu.PC = targ_addr as u16;
                    if boundary_change {
                        self.cycles += 3
                    } else {
                        self.cycles += 2
                    };
                } else {
                    self.cpu.PC += 2;
                    self.cycles += 2;
                }
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
                //clear decimal mode
                println!(
                    "{:04X}  D8        CLD                             {}             CYC:{}",
                    self.cpu.PC, self.cpu, self.cycles
                );
                self.cpu.SR.D = false;
                self.cycles += 2;
                self.cpu.PC += 1;
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
                //nop
                println!(
                    "{:04X}  EA        NOP                             {}             CYC:{}",
                    self.cpu.PC, self.cpu, self.cycles
                );
                //since its only one byte long, only increment pc by one byte
                self.cpu.PC += 1;
                self.cycles += 2;
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
                //branch on res 0
                //branch if z flag is set lol

                //add 1 to cycles if branch occurs on same page
                //add 2 to cycles if branch occurs to different page

                let targ_offset = self.cpu.read(self.cpu.PC + 1, &mut self.cart);
                //target addr is the offsett plus the width of the instruction(2)
                let targ_addr = self.cpu.PC as i16 + (targ_offset as i16) + 2;
                println!("{:04X}  F0 {targ_offset:02X}     BEQ ${targ_addr:4X}                       {}             CYC:{}",self.cpu.PC,self.cpu,self.cycles);

                if self.cpu.SR.Z {
                    let boundary_change = self.cpu.PC % 256 != (targ_addr % 256) as u16;
                    self.cpu.PC = targ_addr as u16;
                    if boundary_change {
                        self.cycles += 3
                    } else {
                        self.cycles += 2
                    };
                } else {
                    self.cpu.PC += 2;
                    self.cycles += 2;
                }
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
                //set decimal
                println!(
                    "{:04X}  F8        SED                             {}             CYC:{}",
                    self.cpu.PC, self.cpu, self.cycles
                );
                self.cpu.SR.D = true;
                self.cycles += 2;
                self.cpu.PC += 1;
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
