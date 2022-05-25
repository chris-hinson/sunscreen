use crate::cart::Cart;
use crate::cpu::Cpu;
use crate::instr::Instr;

use std::fmt::Write;

pub enum AddrMode {
    ACC,
    ABS,
    ABSX,
    ABSY,
    IMM,
    IND,
    INDX,
    INDY,
    REL,
    ZPG,
    ZPGX,
    ZPGY,
}

pub struct NES {
    pub cycles: u128,
    pub cpu: Cpu,
    pub cart: Cart,
    pub instr_data: Instr,
    //apu
    //ppu
    //VRAM
}

impl NES {
    pub fn new(cpu: Cpu, cart: Cart) -> NES {
        NES {
            cpu,
            cart,
            cycles: 7, //from intial reset vector
            instr_data: Instr::new(),
        }
    }

    /*
    A	Accumulator	OPC A	operand is AC (implied single byte instruction)
    abs	absolute	OPC $LLHH	operand is address $HHLL *
    abs,X	absolute, X-indexed	OPC $LLHH,X	operand is address; effective address is address incremented by X with carry **
    abs,Y	absolute, Y-indexed	OPC $LLHH,Y	operand is address; effective address is address incremented by Y with carry **
    #	immediate	OPC #$BB	operand is byte BB
    impl	implied	OPC	operand implied
    ind	indirect	OPC ($LLHH)	operand is address; effective address is contents of word at address: C.w($HHLL)
    X,ind	X-indexed, indirect	OPC ($LL,X)	operand is zeropage address; effective address is word in (LL + X, LL + X + 1), inc. without carry: C.w($00LL + X)
    ind,Y	indirect, Y-indexed	OPC ($LL),Y	operand is zeropage address; effective address is word in (LL, LL + 1) incremented by Y with carry: C.w($00LL) + Y
    rel	relative	OPC $BB	branch target is PC + signed offset BB ***
    zpg	zeropage	OPC $LL	operand is zeropage address (hi-byte is zero, address = $00LL)
    zpg,X	zeropage, X-indexed	OPC $LL,X	operand is zeropage address; effective address is address incremented by X without carry **
    zpg,Y	zeropage, Y-indexed	OPC $LL,Y	operand is zeropage address; effective address is address incremented by Y without carry **
    */

    pub fn calc_addr(&mut self, bytes: &Vec<u8>, mode: AddrMode) -> u16 {
        return match mode {
            AddrMode::ABS => {
                let addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                addr
            }
            AddrMode::ABSX => {
                let base_addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                let addr = base_addr + self.cpu.X as u16;

                //must incur a cycle penalty if we cross pages
                let base_page = base_addr & 0xFF00;
                let final_page = addr & 0xFF00;
                if base_page != final_page {
                    self.cycles += 1;
                }

                addr
            }
            AddrMode::ABSY => {
                let base_addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                let addr = base_addr + self.cpu.Y as u16;

                //must incur a cycle penalty if we cross pages
                let base_page = base_addr & 0xFF00;
                let final_page = addr & 0xFF00;
                if base_page != final_page {
                    self.cycles += 1;
                }

                addr
            }
            AddrMode::IND => {
                unimplemented!("INDIRECT ADDRESSING NOT IMPLEMENTED")
            }
            AddrMode::INDX => {
                unimplemented!("INDIRECT X-INDEXED ADDRESSING NOT IMPLEMENTED")
            }
            AddrMode::INDY => {
                unimplemented!("Y-INDEXED INDIRECT ADDRESSING NOT IMPLEMENTED")
            }
            AddrMode::REL => {
                unimplemented!("RELATIVE ADDRESSING NOT IMPLEMENTED")
            }
            AddrMode::ZPG => {
                let addr = bytes[1] as u16;
                addr
            }
            AddrMode::ZPGX => {
                let base_addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                let addr = base_addr + self.cpu.X as u16;

                //might incur a cycle penalty if we cross pages
                let base_page = base_addr & 0xFF00;
                let final_page = addr & 0xFF00;
                if base_page != final_page {
                    self.cycles += 1;
                }

                addr
            }
            AddrMode::ZPGY => {
                let base_addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                let addr = base_addr + self.cpu.Y as u16;

                //might incur a cycle penalty if we cross pages
                let base_page = base_addr & 0xFF00;
                let final_page = addr & 0xFF00;
                if base_page != final_page {
                    self.cycles += 1;
                }

                addr
            }
            _ => panic!("TRIED TO CALCULATE ADDRESS FOR AN ADDRESSING MODE WITH NO ADDRESS"),
        };
    }

    //uses calc_addr func to figure out our effective address, then reads the byte at that addr
    //takes our stepstring buffer as an arg so it can write debug info into it
    pub fn get_val(&mut self, bytes: &Vec<u8>, mode: AddrMode, stepstring: &mut String) -> u8 {
        //NOTE: this is split out as a match case because we need to print different stuff based on
        // addr mode, otherwise we could just always calc addr and read a byte
        return match mode {
            AddrMode::ACC => self.cpu.ACC,
            AddrMode::ABS => {
                let addr = self.calc_addr(bytes, AddrMode::ABS);
                let val = self.cpu.read(addr, &mut self.cart, 1)[0];
                write!(stepstring, "${:04X} = {:02X}", addr, val).unwrap();
                val
            }
            AddrMode::ABSX => {
                let addr = self.calc_addr(bytes, AddrMode::ABSX);
                self.cpu.read(addr, &mut self.cart, 1)[0]
            }
            AddrMode::ABSY => {
                let addr = self.calc_addr(bytes, AddrMode::ABSY);
                self.cpu.read(addr, &mut self.cart, 1)[0]
            }
            AddrMode::IMM => {
                write!(stepstring, "#${:02X}", bytes[1]).unwrap();
                bytes[1]
            }
            AddrMode::IND => {
                let addr = self.calc_addr(bytes, AddrMode::IND);
                self.cpu.read(addr, &mut self.cart, 1)[0]
            }
            AddrMode::INDX => {
                let addr = self.calc_addr(bytes, AddrMode::INDX);
                self.cpu.read(addr, &mut self.cart, 1)[0]
            }
            AddrMode::INDY => {
                let addr = self.calc_addr(bytes, AddrMode::INDY);
                self.cpu.read(addr, &mut self.cart, 1)[0]
            }
            AddrMode::REL => {
                let addr = self.calc_addr(bytes, AddrMode::REL);
                self.cpu.read(addr, &mut self.cart, 1)[0]
            }
            AddrMode::ZPG => {
                let addr = self.calc_addr(bytes, AddrMode::ZPG);
                let val = self.cpu.read(addr, &mut self.cart, 1)[0];
                write!(stepstring, "${:02X} = {:02X}", addr as u8, val).unwrap();
                val
            }
            AddrMode::ZPGX => {
                let addr = self.calc_addr(bytes, AddrMode::ZPGX);
                let val = self.cpu.read(addr, &mut self.cart, 1)[0];
                write!(
                    stepstring,
                    "${:04X},X @ {:04X} = {:02X}",
                    (bytes[1] as u16 | (bytes[2] as u16) << 8),
                    addr,
                    val
                )
                .unwrap();

                val
            }
            AddrMode::ZPGY => {
                let addr = self.calc_addr(bytes, AddrMode::ZPGY);
                let val = self.cpu.read(addr, &mut self.cart, 1)[0];
                write!(
                    stepstring,
                    "${:04X},X @ {:04X} = {:02X}",
                    (bytes[1] as u16 | (bytes[2] as u16) << 8),
                    addr,
                    val
                )
                .unwrap();

                val
            }
        };
    }

    pub fn step(&mut self) -> String {
        //for debugging, lets build a string to output this step
        let mut stepstring = String::new();

        //print exactly 16 characters composed of:
        // our current addr,
        // the bytes that make up this instr,
        // padding out to 16 chars
        let instr: u8 = self.cpu.read(self.cpu.PC, &mut self.cart, 1)[0];

        //DEBUG
        match self.instr_data.instrs.get(&instr) {
            Some(_v) => {}
            None => {
                unimplemented!("crashing on unimplemented op: {instr:02x}")
            }
        }

        let bytes = self.cpu.read(
            self.cpu.PC,
            &mut self.cart,
            self.instr_data.instrs[&instr].len,
        );
        let bytes_string = print_bytes(&bytes);
        let padding: String = vec![" "; 16 - (bytes_string.len() + 6)].join("");

        write!(
            stepstring,
            "{:04X}  {bytes_string}{padding}{} ",
            self.cpu.PC, self.instr_data.instrs[&instr].name,
        )
        .unwrap();

        //simulates full opcode space, including illegal instructions
        match instr {
            /*
            ADC add with carry
                immediate	ADC #oper	69	2	2
                zeropage	ADC oper	65	2	3
                zeropage,X	ADC oper,X	75	2	4
                absolute	ADC oper	6D	3	4
                absolute,X	ADC oper,X	7D	3	4*
                absolute,Y	ADC oper,Y	79	3	4*
                (indirect,X)	ADC (oper,X)	61	2	6
                (indirect),Y	ADC (oper),Y	71	2	5* */
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                let val: u8 = match instr {
                    0x69 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring),
                    0x65 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring),
                    0x75 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring),
                    0x7D => self.get_val(&bytes, AddrMode::ABS, &mut stepstring),
                    0x6D => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring),
                    0x79 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring),
                    0x61 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring),
                    0x71 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring),
                    _ => {
                        unimplemented!("IN ADC, BUT GOT INVALID OPCODE")
                    }
                };

                let old_acc = self.cpu.ACC;

                //ACC = ACC + M + C
                //NZCV
                let c: u8 = if self.cpu.SR.C { 1 } else { 0 };

                let res_one = (self.cpu.ACC as i8).overflowing_add(val as i8);
                //println!("res one: {:02X} + {:02X} = {res_one:?}", self.cpu.ACC, val);
                let res_two = res_one.0.overflowing_add(c as i8);
                //println!("res two: {:02X} + {:02X} =  {res_two:?}", res_one.0, c);

                //set the actual Result
                self.cpu.ACC = res_two.0 as u8;

                self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
                self.cpu.SR.Z = self.cpu.ACC == 0;
                //NOTE: THIS METHOD REQUIRES NIGHTLY. FIND A WAY TO DO WITHOUT?
                //self.cpu.SR.C = (self.cpu.ACC as u8) < (old_acc as u8);
                self.cpu.SR.C = old_acc.carrying_add(val, self.cpu.SR.C).1;
                self.cpu.SR.V = res_one.1 || res_two.1;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            AND and (with accumulator)
                immediate	AND #oper	29	2	2
                zeropage	AND oper	25	2	3
                zeropage,X	AND oper,X	35	2	4
                absolute	AND oper	2D	3	4
                absolute,X	AND oper,X	3D	3	4*
                absolute,Y	AND oper,Y	39	3	4*
                (indirect,X)	AND (oper,X)	21	2	6
                (indirect),Y	AND (oper),Y	31	2	5* */
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                let val: u8 = match instr {
                    0x29 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring),
                    0x25 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring),
                    0x35 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring),
                    0x2D => self.get_val(&bytes, AddrMode::ABS, &mut stepstring),
                    0x3D => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring),
                    0x39 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring),
                    0x21 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring),
                    0x31 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring),
                    _ => {
                        panic!("IN AND, BUT GOT INVALID OPCODE")
                    }
                };

                self.cpu.ACC &= val;

                self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
                self.cpu.SR.Z = self.cpu.ACC == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            ASL arithmetic shift left

            //TODO: combine all branch ops into one case
            BCC branch on carry clear
                relative	BCC oper	90	2	2** */
            0x90 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                let target = self.cpu.PC as i16 + bytes[1] as i16;
                write!(stepstring, "${target:04X}").unwrap();

                if !self.cpu.SR.C {
                    let cur_page = self.cpu.PC & 0xFF00;
                    let target_page = (target as u16) & 0xFF00;
                    self.cpu.PC = target as u16;
                    //branch taken always adds a cycle
                    self.cycles += 1;
                    //add another cycle if we cross page boundary
                    if cur_page != target_page {
                        //println!("branch across page: {cur_page} to {target_page}");
                        self.cycles += 1
                    }
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            BCS branch on carry set
                relative	BCS oper	B0	2	2** */
            0xB0 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                let target = self.cpu.PC as i16 + bytes[1] as i16;
                write!(stepstring, "${target:04X}").unwrap();

                if self.cpu.SR.C {
                    let cur_page = self.cpu.PC & 0xFF00;
                    let target_page = (target as u16) & 0xFF00;
                    self.cpu.PC = target as u16;
                    //branching always adds a cycle
                    self.cycles += 1;
                    //add another cycle if we cross page boundary
                    if cur_page != target_page {
                        //println!("branch across page: {cur_page} to {target_page}");
                        self.cycles += 1
                    }
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            BEQ branch on equal (zero set)
                relative	BEQ oper	F0	2	2** */
            0xF0 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                let target = self.cpu.PC as i16 + bytes[1] as i16;
                write!(stepstring, "${target:04X}").unwrap();

                if self.cpu.SR.Z {
                    let cur_page = self.cpu.PC & 0xFF00;
                    let target_page = (target as u16) & 0xFF00;
                    self.cpu.PC = target as u16;
                    //branching always adds a cycle
                    self.cycles += 1;
                    //add another cycle if we cross page boundary
                    if cur_page != target_page {
                        //println!("branch across page: {cur_page} to {target_page}");
                        self.cycles += 1
                    }
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            BIT bit test
                zeropage	BIT oper	24	2	3
                absolute	BIT oper	2C	3	4  */
            0x24 | 0x2C => {
                let val: u8 = if instr == 0x24 {
                    self.get_val(&bytes, AddrMode::ZPG, &mut stepstring)
                } else {
                    self.get_val(&bytes, AddrMode::ABS, &mut stepstring)
                };

                //N = m7
                self.cpu.SR.N = (val & 0b10000000) >> 7 == 1;
                //V = m6
                self.cpu.SR.V = (val & 0b01000000) >> 6 == 1;
                //Z = ACC and VAL
                self.cpu.SR.Z = (self.cpu.ACC & val) == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            BMI branch on minus (negative set)
                relative	BMI oper	30	2	2** */
            0x30 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                let target = self.cpu.PC as i16 + bytes[1] as i16;
                write!(stepstring, "${target:04X}").unwrap();

                if self.cpu.SR.N {
                    let cur_page = self.cpu.PC & 0xFF00;
                    let target_page = (target as u16) & 0xFF00;
                    self.cpu.PC = target as u16;
                    //branching always adds a cycle
                    self.cycles += 1;
                    //add another cycle if we cross page boundary
                    if cur_page != target_page {
                        self.cycles += 1
                    }
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            BNE branch on not equal (zero clear)
                relative	BNE oper	D0	2	2**  */
            0xD0 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                let target = self.cpu.PC as i16 + bytes[1] as i16;
                write!(stepstring, "${target:04X}").unwrap();

                if !self.cpu.SR.Z {
                    let cur_page = self.cpu.PC & 0xFF00;
                    let target_page = (target as u16) & 0xFF00;
                    self.cpu.PC = target as u16;
                    //branching always adds a cycle
                    self.cycles += 1;
                    //add another cycle if we cross page boundary
                    if cur_page != target_page {
                        //println!("branch across page: {cur_page} to {target_page}");
                        self.cycles += 1
                    }
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            BPL branch on plus (negative clear)
                relative	BPL oper	10	2	2** */
            0x10 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                let target = self.cpu.PC as i16 + bytes[1] as i16;
                write!(stepstring, "${target:04X}").unwrap();

                if !self.cpu.SR.N {
                    let cur_page = self.cpu.PC & 0xFF00;
                    let target_page = (target as u16) & 0xFF00;
                    self.cpu.PC = target as u16;
                    //branching always adds a cycle
                    self.cycles += 1;
                    //add another cycle if we cross page boundary
                    if cur_page != target_page {
                        self.cycles += 1
                    }
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            BRK break / interrupt
                implied	BRK	00	1	7  */
            //TODO: THIS IS UNTESTED LUL
            0x00 => {
                //interrupt,
                //push PC+2
                let ra = self.cpu.PC + 2;
                self.cpu.write(
                    self.cpu.SP as u16,
                    &vec![(ra & 0xFF) as u8, (ra & 0xFF00 >> 8) as u8],
                );
                self.cpu.SP -= 2;
                //SR.I = true
                self.cpu.SR.I = true;
                //push SR(B = 11)
                let mut saved_sr = self.cpu.SR;
                saved_sr.BH = true;
                saved_sr.BL = true;
                self.cpu.write(self.cpu.SP as u16, &vec![saved_sr.decode()]);
                self.cpu.SP -= 1;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            BVC branch on overflow clear
                relative	BVC oper	50	2	2** */
            0x50 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                let target = self.cpu.PC as i16 + bytes[1] as i16;
                write!(stepstring, "${target:04X}").unwrap();

                if !self.cpu.SR.V {
                    let cur_page = self.cpu.PC & 0xFF00;
                    let target_page = (target as u16) & 0xFF00;
                    self.cpu.PC = target as u16;
                    //branching always adds a cycle
                    self.cycles += 1;
                    //add another cycle if we cross page boundary
                    if cur_page != target_page {
                        //println!("branch across page: {cur_page} to {target_page}");
                        self.cycles += 1
                    }
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            BVS branch on overflow set
                relative	BVS oper	70	2	2** */
            0x70 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                let target = self.cpu.PC as i16 + bytes[1] as i16;
                write!(stepstring, "${target:04X}").unwrap();

                if self.cpu.SR.V {
                    let cur_page = self.cpu.PC & 0xFF00;
                    let target_page = (target as u16) & 0xFF00;
                    self.cpu.PC = target as u16;
                    //branching always adds a cycle
                    self.cycles += 1;
                    //add another cycle if we cross page boundary
                    if cur_page != target_page {
                        //println!("branch across page: {cur_page} to {target_page}");
                        self.cycles += 1
                    }
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            CLC clear carry
                implied	CLC	18	1	2 */
            0x18 => {
                self.cpu.SR.C = false;
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            CLD clear decimal
                implied	CLD	D8	1	 2*/
            0xD8 => {
                self.cpu.SR.D = false;
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            CLI clear interrupt disable
            CLV clear overflow
                implied	CLV	B8	1	2  */
            0xB8 => {
                self.cpu.SR.V = false;
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            CMP compare (with accumulator)
                immediate	CMP #oper	C9	2	2
                zeropage	CMP oper	C5	2	3
                zeropage,X	CMP oper,X	D5	2	4
                absolute	CMP oper	CD	3	4
                absolute,X	CMP oper,X	DD	3	4*
                absolute,Y	CMP oper,Y	D9	3	4*
                (indirect,X)	CMP (oper,X)	C1	2	6
                (indirect),Y	CMP (oper),Y	D1	2	5* */
            0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => {
                let val: u8 = match instr {
                    //imm
                    0xC9 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring),
                    //zpg
                    0xC5 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring),
                    //zpg,x
                    0xD5 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring),
                    0xCD => self.get_val(&bytes, AddrMode::ABS, &mut stepstring),
                    0xDD => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring),
                    0xD9 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring),
                    0xC1 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring),
                    0xD1 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring),
                    _ => {
                        unimplemented!("IN CMP BUT GOT ILLEGAL OPCODE")
                    }
                };

                //ACC - M (DO NOT SAVE)
                let res = self.cpu.ACC.wrapping_sub(val);
                //just affects NZC flags
                self.cpu.SR.N = (res as i8) < 0;
                self.cpu.SR.Z = res == 0;

                //TODO: shouldnt this be the other way around????
                //println!("CMP: {:02X} - {:02X} = {res:02X}", self.cpu.ACC, val);
                self.cpu.SR.C = self.cpu.ACC >= val;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            CPX compare with X
                immediate	CPX #oper	E0	2	2
                zeropage	CPX oper	E4	2	3
                absolute	CPX oper	EC	3	4  */
            0xE0 | 0xE4 | 0xEC => {
                let val: u8 = match instr {
                    0xE0 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring),
                    0xE4 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring),
                    0xEC => self.get_val(&bytes, AddrMode::ABS, &mut stepstring),
                    _ => panic!("IN CPX BUT GOT BAD OPCODE"),
                };

                //X - M (DO NOT SAVE)
                let res = self.cpu.X.wrapping_sub(val);
                //just affects NZC flags
                self.cpu.SR.N = (res as i8) < 0;
                self.cpu.SR.Z = res == 0;

                //TODO: shouldnt this be the other way around????
                //println!("CMP: {:02X} - {:02X} = {res:02X}", self.cpu.ACC, val);
                self.cpu.SR.C = self.cpu.X >= val;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            CPY compare with Y
                immediate	CPY #oper	C0	2	2
                zeropage	CPY oper	C4	2	3
                absolute	CPY oper	CC	3	4  */
            0xC0 | 0xC4 | 0xCC => {
                let val: u8 = match instr {
                    0xC0 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring),
                    0xC4 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring),
                    0xCC => self.get_val(&bytes, AddrMode::ABS, &mut stepstring),
                    _ => panic!("IN CPX BUT GOT BAD OPCODE"),
                };

                //X - M (DO NOT SAVE)
                let res = self.cpu.Y.wrapping_sub(val);
                //just affects NZC flags
                self.cpu.SR.N = (res as i8) < 0;
                self.cpu.SR.Z = res == 0;

                //TODO: shouldnt this be the other way around????
                //println!("CMP: {:02X} - {:02X} = {res:02X}", self.cpu.ACC, val);
                self.cpu.SR.C = self.cpu.Y >= val;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            DEC decrement
            DEX decrement X
                implied	DEX	CA	1	2   */
            0xCA => {
                self.cpu.X = self.cpu.X.wrapping_sub(1);

                self.cpu.SR.N = (self.cpu.X as i8) < 0;
                self.cpu.SR.Z = self.cpu.X == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            DEY decrement Y
                implied	DEY	88	1	2   */
            0x88 => {
                self.cpu.Y = self.cpu.Y.wrapping_sub(1);

                self.cpu.SR.N = (self.cpu.Y as i8) < 0;
                self.cpu.SR.Z = self.cpu.Y == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            EOR exclusive or (with accumulator)
                immediate	EOR #oper	49	2	2
                zeropage	EOR oper	45	2	3
                zeropage,X	EOR oper,X	55	2	4
                absolute	EOR oper	4D	3	4
                absolute,X	EOR oper,X	5D	3	4*
                absolute,Y	EOR oper,Y	59	3	4*
                (indirect,X)	EOR (oper,X)	41	2	6
                (indirect),Y	EOR (oper),Y	51	2	5* */
            0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => {
                let val: u8 = match instr {
                    0x49 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring),
                    0x45 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring),
                    0x55 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring),
                    0x4D => self.get_val(&bytes, AddrMode::ABS, &mut stepstring),
                    0x5D => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring),
                    0x59 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring),
                    0x41 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring),
                    0x51 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring),
                    _ => {
                        panic!("IN EOR, BUT INVALID OPCODE")
                    }
                };

                self.cpu.ACC ^= val;

                self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
                self.cpu.SR.Z = self.cpu.ACC == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            INC increment
            INX increment X
                X + 1 -> X */
            0xE8 => {
                self.cpu.X = self.cpu.X.wrapping_add(1);

                self.cpu.SR.N = (self.cpu.X as i8) < 0;
                self.cpu.SR.Z = self.cpu.X == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            INY increment Y
                Y + 1 -> Y */
            0xC8 => {
                self.cpu.Y = self.cpu.Y.wrapping_add(1);

                self.cpu.SR.N = (self.cpu.Y as i8) < 0;
                self.cpu.SR.Z = self.cpu.Y == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            JMP jump
                absolute	JMP oper	4C	3	3
                indirect	JMP (oper)	6C	3	5*/
            0x4C | 0x6C => {
                let target: u16 = if instr == 0x4C {
                    let imm = bytes[1] as u16 | (bytes[2] as u16) << 8;
                    write!(stepstring, "${imm:04X}").unwrap();
                    imm
                } else {
                    let addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                    let bytes = self.cpu.read(addr, &mut self.cart, 2);
                    let val = bytes[1] as u16 | (bytes[2] as u16) << 8;
                    write!(stepstring, "(${addr:04X}) = {val:04X}").unwrap();
                    val
                };

                self.cpu.PC = target;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            JSR jump subroutine
                absolute	JSR oper	20	3	6  */
            0x20 => {
                //why the fuck does the 6502 use an empty stack >:(
                //push ra
                let addr = self.cpu.SP as u16;
                let ra = self.cpu.PC + 2;
                let ral = (ra & 0xFF) as u8;
                let rah = ((ra >> 8) & 0xFF) as u8;
                self.cpu.write(addr, &[ral, rah].to_vec());
                self.cpu.SP -= 2;
                //set pc to target addr
                let target = bytes[1] as u16 | (bytes[2] as u16) << 8;
                self.cpu.PC = target;

                write!(stepstring, "${target:04X}").unwrap();
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            LDA load accumulator
                immediate	LDA #oper	A9	2	2
                zeropage	LDA oper	A5	2	3
                zeropage,X	LDA oper,X	B5	2	4
                absolute	LDA oper	AD	3	4
                absolute,X	LDA oper,X	BD	3	4*
                absolute,Y	LDA oper,Y	B9	3	4*
                (indirect,X)	LDA (oper,X)	A1	2	6
                (indirect),Y	LDA (oper),Y	B1	2	5* */
            0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                let val: u8 = match instr {
                    //imm
                    0xA9 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring),
                    //zeropage
                    0xA5 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring),
                    //zeropage,x
                    0xB5 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring),
                    //abs
                    0xAD => self.get_val(&bytes, AddrMode::ABS, &mut stepstring),
                    //abs,x
                    0xBD => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring),
                    //abs,y
                    0xB9 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring),
                    //(indirect,X)
                    0xA1 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring),
                    //(indirect),Y
                    0xB1 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring),
                    _ => {
                        panic!("IN LDA, BUT SOMEHOW GOT INVALID OP")
                    }
                };

                self.cpu.ACC = val;

                self.cpu.SR.N = (val as i8) < 0;
                self.cpu.SR.Z = val == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            LDX load X
                immediate	LDX #oper	A2	2	2
                zeropage	LDX oper	A6	2	3
                zeropage,Y	LDX oper,Y	B6	2	4
                absolute	LDX oper	AE	3	4
                absolute,Y	LDX oper,Y	BE	3	4* */
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                let val = match instr {
                    0xA2 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring),
                    0xA6 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring),
                    0xB6 => self.get_val(&bytes, AddrMode::ZPGY, &mut stepstring),
                    0xAE => self.get_val(&bytes, AddrMode::ABS, &mut stepstring),
                    0xBE => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring),
                    _ => panic!("IN LDX, BUT GOT BAD OP"),
                };

                //actual loading
                self.cpu.X = val;

                //set flags
                self.cpu.SR.N = (val as i8) < 0;
                self.cpu.SR.Z = val == 0;

                //update PC
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            LDY load Y
                immediate	LDY #oper	A0	2	2
                zeropage	LDY oper	A4	2	3
                zeropage,X	LDY oper,X	B4	2	4
                absolute	LDY oper	AC	3	4
                absolute,X	LDY oper,X	BC	3	4*  */
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                let val = match instr {
                    0xA0 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring),
                    0xA4 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring),
                    0xB4 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring),
                    0xAC => self.get_val(&bytes, AddrMode::ABS, &mut stepstring),
                    0xBC => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring),
                    _ => panic!("IN LDY BUT GOT BAD OPCODE"),
                };

                //actual loading
                self.cpu.Y = val;

                //set flags
                self.cpu.SR.N = (val as i8) < 0;
                self.cpu.SR.Z = val == 0;

                //update PC
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            LSR logical shift right
            NOP no operation
                implied	NOP	EA	1	2 */
            0xEA => {
                self.cpu.PC += 1;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            ORA or with accumulator
                immediate	ORA #oper	09	2	2
                zeropage	ORA oper	05	2	3
                zeropage,X	ORA oper,X	15	2	4
                absolute	ORA oper	0D	3	4
                absolute,X	ORA oper,X	1D	3	4*
                absolute,Y	ORA oper,Y	19	3	4*
                (indirect,X)	ORA (oper,X)	01	2	6
                (indirect),Y	ORA (oper),Y	11	2	5* */
            0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => {
                let val: u8 = match instr {
                    0x09 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring),
                    0x05 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring),
                    //zeropage,x
                    0x15 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring),
                    //abs
                    0x0D => self.get_val(&bytes, AddrMode::ABS, &mut stepstring),
                    //abs,x
                    0x1D => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring),
                    //abs y
                    0x19 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring),
                    0x01 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring),
                    0x11 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring),
                    _ => {
                        panic!("IN OR, BUT GOT INVALID OPCODE")
                    }
                };

                self.cpu.ACC |= val;

                self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
                self.cpu.SR.Z = self.cpu.ACC == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            PHA push accumulator
                implied	PHA	48	1	3 */
            0x48 => {
                //push ACC (B =  11)
                self.cpu.write(self.cpu.SP as u16, &vec![self.cpu.ACC]);
                self.cpu.SP -= 1;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            PHP push processor status (SR)
                implied	PHP	08	1	3  */
            0x08 => {
                //push SR (B =  11)
                let mut saved_sr = self.cpu.SR;
                saved_sr.BH = true;
                saved_sr.BL = true;

                self.cpu.write(self.cpu.SP as u16, &vec![saved_sr.decode()]);
                self.cpu.SP -= 1;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            PLA pull accumulator
                implied	PLA	68	1	4  */
            0x68 => {
                self.cpu.SP += 1;
                self.cpu.ACC = self.cpu.read(self.cpu.SP as u16, &mut self.cart, 1)[0];

                self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
                self.cpu.SR.Z = self.cpu.ACC == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            PLP pull processor status (SR)
                implied	PLP	28	1	4 */
            0x28 => {
                self.cpu.SP += 1;
                let pulled = self.cpu.read(self.cpu.SP as u16, &mut self.cart, 1)[0];
                let old_bh = self.cpu.SR.BH;
                let old_bl = self.cpu.SR.BL;

                self.cpu.SR.encode(pulled);
                self.cpu.SR.BH = old_bh;
                self.cpu.SR.BL = old_bl;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            ROL rotate left
            ROR rotate right
            RTI return from interrupt
            RTS return from subroutine
                implied	RTS	60	1	6  */
            0x60 => {
                //pull PC, PC+1 -> PC
                self.cpu.SP += 2;
                let pull = self.cpu.read(self.cpu.SP as u16, &mut self.cart, 2);
                let new_pc = pull[0] as u16 | (pull[1] as u16) << 8;
                self.cpu.PC = new_pc;

                self.cpu.PC += 1;

                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            SBC subtract with carry
                immediate	SBC #oper	E9	2	2
                zeropage	SBC oper	E5	2	3
                zeropage,X	SBC oper,X	F5	2	4
                absolute	SBC oper	ED	3	4
                absolute,X	SBC oper,X	FD	3	4*
                absolute,Y	SBC oper,Y	F9	3	4*
                (indirect,X)	SBC (oper,X)	E1	2	6
                (indirect),Y	SBC (oper),Y	F1	2	5* */
            0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                let val: u8 = match instr {
                    0xE9 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring),
                    0xE5 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring),
                    0xF5 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring),
                    0xED => self.get_val(&bytes, AddrMode::ABS, &mut stepstring),
                    0xFD => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring),
                    0xF9 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring),
                    0xE1 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring),
                    0xF1 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring),
                    _ => panic!("IN SBC BUT GOT BAD OPCODE"),
                };

                //A - M - C -> A
                //NZCV

                //shoutout kamiyaowl's rust nes emulator for this one
                let sub1 = self.cpu.ACC.overflowing_sub(val);
                let sub2 = sub1.0.overflowing_sub(if self.cpu.SR.C { 0 } else { 1 });

                self.cpu.SR.Z = sub2.0 == 0;
                self.cpu.SR.N = (sub2.0 as i8) < 0;
                self.cpu.SR.C = !(sub1.1 || sub2.1);
                self.cpu.SR.V = (((self.cpu.ACC ^ val) & 0x80) == 0x80)
                    && (((self.cpu.ACC ^ sub2.0) & 0x80) == 0x80);

                self.cpu.ACC = sub2.0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            SEC set carry
                implied	SEC	38	1	2 */
            0x38 => {
                self.cpu.SR.C = true;
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            SED set decimal
                implied	SED	F8	1	2  */
            0xF8 => {
                self.cpu.SR.D = true;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            SEI set interrupt disable
                implied	SEI	78	1	2  */
            0x78 => {
                self.cpu.SR.I = true;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            STA store accumulator
                zeropage	STA oper	85	2	3
                zeropage,X	STA oper,X	95	2	4
                absolute	STA oper	8D	3	4
                absolute,X	STA oper,X	9D	3	5
                absolute,Y	STA oper,Y	99	3	5
                (indirect,X)	STA (oper,X)	81	2	6
                (indirect),Y	STA (oper),Y	91	2	6  */
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                //NOTE: what a stupid fucking log
                //LDA $addr = val is WHAT IS CURRENTLY AT THAT ADDR BEFORE OUR STORE???
                let addr: u16 = match instr {
                    0x85 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPG);
                        write!(
                            stepstring,
                            "${:02x} = {:02X}",
                            addr,
                            self.cpu.read(addr, &mut self.cart, 1)[0]
                        )
                        .unwrap();
                        addr
                    }
                    0x95 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPGX);
                        addr
                    }
                    0x8D => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABS);
                        addr
                    }
                    0x9D => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABSX);
                        addr
                    }
                    0x99 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABSY);
                        addr
                    }
                    0x81 => {
                        let addr = self.calc_addr(&bytes, AddrMode::INDX);
                        addr
                    }
                    0x91 => {
                        let addr = self.calc_addr(&bytes, AddrMode::INDY);
                        addr
                    }
                    _ => {
                        panic!("IN STA BUT INVALID OP")
                    }
                };

                self.cpu.write(addr, &vec![self.cpu.ACC]);

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            STX store X
                zeropage	STX oper	86	2	3
                zeropage,Y	STX oper,Y	96	2	4
                absolute	STX oper	8E	3	4  */
            0x86 | 0x96 | 0x8E => {
                //note: for some fucking reason, the log wants us to say what is at that address
                //BEFORE we write to it
                let addr = if instr == 0x86 {
                    let addr = self.calc_addr(&bytes, AddrMode::ZPG);
                    let cur_val = self.cpu.read(addr, &mut self.cart, 1)[0];
                    write!(stepstring, "${:02X} = {:02X}", addr, cur_val).unwrap();
                    addr
                } else if instr == 0x96 {
                    let addr = self.calc_addr(&bytes, AddrMode::ZPGY);
                    addr
                } else {
                    let addr = self.calc_addr(&bytes, AddrMode::ABS);
                    let cur_val = self.cpu.read(addr, &mut self.cart, 1)[0];
                    write!(stepstring, "${:04X} = {:02X}", addr, cur_val).unwrap();
                    addr
                };

                self.cpu.write(addr, &[self.cpu.X].to_vec());

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            STY store Y
            TAX transfer accumulator to X
                implied	TAX	AA	1	2  */
            0xAA => {
                self.cpu.X = self.cpu.ACC;

                self.cpu.SR.N = (self.cpu.X as i8) < 0;
                self.cpu.SR.Z = self.cpu.X == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            TAY transfer accumulator to Y
                implied	TAY	A8	1	2  */
            0xA8 => {
                self.cpu.Y = self.cpu.ACC;

                self.cpu.SR.N = (self.cpu.Y as i8) < 0;
                self.cpu.SR.Z = self.cpu.Y == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            TSX transfer stack pointer to X
                implied	TSX	BA	1	2   */
            0xBA => {
                self.cpu.X = self.cpu.SP;

                self.cpu.SR.N = (self.cpu.SP as i8) < 0;
                self.cpu.SR.Z = self.cpu.SP == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            TXA transfer X to accumulator
                implied	TXA	8A	1	2  */
            0x8A => {
                self.cpu.ACC = self.cpu.X;

                self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
                self.cpu.SR.Z = self.cpu.ACC == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            TXS transfer X to stack pointer
                implied	TXS	9A	1	2 */
            0x9A => {
                //NO FLAGS SET

                self.cpu.SP = self.cpu.X;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            TYA transfer Y to accumulator
                implied	TYA	98	1	2  */
            0x98 => {
                self.cpu.ACC = self.cpu.Y;

                self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
                self.cpu.SR.Z = self.cpu.ACC == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            _ => {
                panic!("unimplemented op {:#02x}", instr)
            }
        }
        //print padding and then cpu state and cycles
        let final_padding = vec![" "; 48 - stepstring.len()].join("");
        write!(
            stepstring,
            "{final_padding}{} CYC:{}",
            self.cpu, self.cycles
        )
        .unwrap();
        println!("{stepstring}");
        return stepstring;
    }
}

pub fn print_bytes(bytes: &Vec<u8>) -> String {
    let mut ret_str = String::new();

    for b in bytes {
        ret_str.push_str(&format!("{:02X} ", b));
    }
    return ret_str;
}
