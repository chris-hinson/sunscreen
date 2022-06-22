use crate::cart::Cart;
use crate::cpu::Cpu;
use crate::instr::Instr;
use crate::wram::Wram;

//use pretty_assertions::{assert_eq, assert_ne};
//use pretty_assertions::Comparison;
//use sdl2::render::SurfaceCanvas;

use std::fmt::Write;
use std::fs::OpenOptions;

//TODO: remove this allow once we finish implementing all addressing modes
#[allow(dead_code)]
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

#[derive(Clone)]
pub struct NES {
    //components
    pub cpu: Cpu,
    pub cart: Cart,
    pub instr_data: Instr,
    pub wram: Wram,
    //apu
    //ppu
    //VRAM??

    //data about the system
    pub cycles: u128,

    //breakpoints halt execution when our PC equals that value
    pub breakpoints: Vec<usize>,
    //watchpoints halt execution when a write goes to that address
    pub watchpoints: Vec<usize>,
}

#[allow(dead_code)]
impl NES {
    pub fn new(cpu: Cpu, cart: Cart, wram: Wram) -> NES {
        NES {
            cpu,
            cart,
            wram,
            cycles: 7, //from intial reset vector
            instr_data: Instr::new(),
            breakpoints: Vec::new(),
            watchpoints: Vec::new(),
        }
    }

    pub fn add_watchpoint(&mut self, addr: usize) {
        self.watchpoints.push(addr);
    }
    pub fn add_breakpoint(&mut self, addr: usize) {
        self.breakpoints.push(addr);
    }

    //function to run this system in its own thread, takes a SENDER channel to return logs on to the rendering thread
    pub fn run(&mut self, mut good_log: Vec<String>) {
        /*let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("./file")
        .unwrap();*/

        let mut tui = crate::tui::setup_tui(self);

        let mut halt = false;
        let mut pending_logs: Vec<String> = Vec::new();
        //endless running loop
        loop {
            let good_line = match good_log.pop() {
                Some(v) => v,
                None => panic!("log file is empty???"),
            };
            match self.step() {
                //Ok means that we didnt encounter anything out of the ordinary in our step
                Ok(our_line) => {
                    //file.write("{our_line}");
                    //write!(file, "{our_line}");
                    std::fs::write("./err", our_line.clone())
                        .expect("please just shut the fuck up");
                    //always push the line we just got back
                    if good_line.ne(&our_line) {
                        //panic!("mismatch");
                        pending_logs.push(format!(
                            "halting on line mismatch, PC = {:04X}",
                            self.cpu.PC
                        ));

                        pending_logs.push(format!("G: {}", good_line.clone()));
                        pending_logs.push(format!("B: {}", our_line.clone()));

                        halt = true;
                    } else {
                        pending_logs.push(our_line.clone());
                    }
                }
                //append our error and halt
                Err(error_string) => {
                    std::fs::write("./err", error_string.clone())
                        .expect("please just shut the fuck up");

                    pending_logs.push(error_string);
                    halt = true;
                }
            }

            if halt {
                //if we're halting on this step, call our tui runner function
                crate::tui::run(&mut tui, &mut pending_logs, self);
                //clear our pending logs before we continue
                pending_logs = Vec::new();
                //make sure to turn halting off
                halt = false;
            }
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

    pub fn calc_addr(&mut self, bytes: &Vec<u8>, mode: AddrMode, penalty: bool) -> u16 {
        return match mode {
            AddrMode::ABS => {
                let addr = (bytes[2] as u16) << 8 | bytes[1] as u16;
                addr
            }
            AddrMode::ABSX => {
                let base_addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                let addr = base_addr.wrapping_add(self.cpu.X as u16);

                //must incur a cycle penalty if we cross pages
                let base_page = base_addr & 0xFF00;
                let final_page = addr & 0xFF00;
                if base_page != final_page && penalty {
                    self.cycles += 1;
                }

                addr
            }
            AddrMode::ABSY => {
                let base_addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                let addr = base_addr.wrapping_add(self.cpu.Y as u16);

                //must incur a cycle penalty if we cross pages
                let base_page = base_addr & 0xFF00;
                let final_page = addr & 0xFF00;
                if base_page != final_page && penalty {
                    self.cycles += 1;
                }

                addr
            }
            AddrMode::IND => {
                unimplemented!("INDIRECT ADDRESSING NOT IMPLEMENTED")
            }
            AddrMode::INDX => {
                //OPC ($LL,X)
                //operand is zeropage address;
                let mut zpg_addr: u16 = bytes[1] as u16;
                //effective address is word in (LL + X, LL + X + 1), inc. without carry: C.w($00LL + X)
                zpg_addr = zpg_addr.wrapping_add(self.cpu.X as u16) & 0xFF;
                //we read from this zeropage address to get our effective address
                let ea_l = self.read(zpg_addr, 1)[0];
                //make sure we mask to keep this as a zpg addr
                zpg_addr = zpg_addr.wrapping_add(1) & 0xFF;
                let ea_h = self.read(zpg_addr, 1)[0];
                let addr: u16 = ea_l as u16 | (ea_h as u16) << 8;
                //returning the effective address
                addr
            }
            AddrMode::INDY => {
                //OPC ($LL),Y
                //operand is zeropage address; effective address is word in (LL, LL + 1) incremented by Y with carry:
                //C.w($00LL) + Y
                let mut zpg_addr: u16 = bytes[1] as u16;
                let ea_l = self.read(zpg_addr, 1)[0];
                zpg_addr = zpg_addr.wrapping_add(1) & 0xFF;
                let ea_h = self.read(zpg_addr, 1)[0];
                let base_addr: u16 = ea_l as u16 | (ea_h as u16) << 8;
                let addr = base_addr.wrapping_add(self.cpu.Y as u16);

                //must incur a cycle penalty if we cross pages
                let base_page = base_addr & 0xFF00;
                let final_page = addr & 0xFF00;
                if base_page != final_page && penalty {
                    self.cycles += 1;
                }
                addr
            }
            AddrMode::REL => {
                //offset is signed so it can be forward or backwards
                let offset = bytes[1] as i8;
                let target = self.cpu.PC.wrapping_add_signed(offset as i16);

                //rel addressing may incur a penalty if crossing page boundary
                //AND IF THE BRANCH IS TAKEN
                if ((self.cpu.PC & 0xFF00) != (target & 0xFF00)) && penalty {
                    self.cycles += 1;
                }

                target
            }
            AddrMode::ZPG => {
                let addr = bytes[1] as u16;
                addr
            }
            AddrMode::ZPGX => {
                //let base_addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                let base_addr = bytes[1] as u16;
                let addr = base_addr.wrapping_add(self.cpu.X as u16);

                //might incur a cycle penalty if we cross pages
                let base_page = base_addr & 0xFF00;
                let final_page = addr & 0xFF00;
                if base_page != final_page && penalty {
                    self.cycles += 1;
                }

                addr & 0xFF
            }
            AddrMode::ZPGY => {
                let base_addr = bytes[1] as u16;
                let addr = base_addr.wrapping_add(self.cpu.Y as u16);

                //might incur a cycle penalty if we cross pages
                let base_page = base_addr & 0xFF00;
                let final_page = addr & 0xFF00;
                if base_page != final_page && penalty {
                    self.cycles += 1;
                }

                addr & 0xFF
            }
            _ => panic!("TRIED TO CALCULATE ADDRESS FOR AN ADDRESSING MODE WITH NO ADDRESS"),
        };
    }

    //uses calc_addr func to figure out our effective address, then reads the byte at that addr
    //takes our stepstring buffer as an arg so it can write debug info into it
    pub fn get_val(
        &mut self,
        bytes: &Vec<u8>,
        mode: AddrMode,
        stepstring: &mut String,
        penalty: bool,
    ) -> u8 {
        //NOTE: this is split out as a match case because we need to print different stuff based on
        // addr mode, otherwise we could just always calc addr and read a byte
        return match mode {
            AddrMode::ACC => {
                write!(stepstring, "A").unwrap();
                self.cpu.ACC
            }
            AddrMode::ABS => {
                let addr = self.calc_addr(bytes, AddrMode::ABS, penalty);
                let val = self.read(addr, 1)[0];
                write!(stepstring, "${:04X} = {:02X}", addr, val).unwrap();
                val
            }
            AddrMode::ABSX => {
                let addr = self.calc_addr(bytes, AddrMode::ABSX, penalty);
                let val = self.read(addr, 1)[0];

                write!(
                    stepstring,
                    "${:04X},X @ {addr:04X} = {val:02X}",
                    bytes[1] as u16 | (bytes[2] as u16) << 8
                )
                .unwrap();

                val
            }
            AddrMode::ABSY => {
                let addr = self.calc_addr(bytes, AddrMode::ABSY, penalty);
                let val = self.read(addr, 1)[0];

                //$0300,Y @ 0300 = 89
                write!(
                    stepstring,
                    "${:04X},Y @ {addr:04X} = {val:02X}",
                    bytes[1] as u16 | (bytes[2] as u16) << 8
                )
                .unwrap();

                val
            }
            AddrMode::IMM => {
                write!(stepstring, "#${:02X}", bytes[1]).unwrap();
                bytes[1]
            }
            AddrMode::IND => {
                let addr = self.calc_addr(bytes, AddrMode::IND, penalty);
                self.read(addr, 1)[0]
            }
            AddrMode::INDX => {
                //this is our effective(final) address
                let addr = self.calc_addr(bytes, AddrMode::INDX, penalty);
                //we read from this address to get our value
                let val = self.read(addr, 1)[0];

                //bytes , bytes+x, ea, final val
                //($80,X) @ 80 = 0200 = 5A
                write!(
                    stepstring,
                    "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                    bytes[1],
                    bytes[1].wrapping_add(self.cpu.X),
                    addr,
                    val
                )
                .unwrap();

                val
            }
            AddrMode::INDY => {
                let addr = self.calc_addr(bytes, AddrMode::INDY, penalty);
                let val = self.read(addr, 1)[0];

                // bytes, ea,    +y, val
                //LDA ($89),Y = 0300 @ 0300 = 89
                write!(
                    stepstring,
                    "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                    bytes[1],
                    self.read(bytes[1] as u16, 1)[0] as u16
                        | (self.read((bytes[1] as u16).wrapping_add(1) & 0xFF, 1)[0] as u16) << 8,
                    addr,
                    val
                )
                .unwrap();

                val
            }
            AddrMode::REL => {
                let addr = self.calc_addr(bytes, AddrMode::REL, penalty);
                self.read(addr, 1)[0]
            }
            AddrMode::ZPG => {
                let addr = self.calc_addr(bytes, AddrMode::ZPG, penalty);
                let val = self.read(addr, 1)[0];
                write!(stepstring, "${:02X} = {:02X}", addr as u8, val).unwrap();
                val
            }
            AddrMode::ZPGX => {
                let addr = self.calc_addr(bytes, AddrMode::ZPGX, penalty);
                let val = self.read(addr, 1)[0];
                write!(
                    stepstring,
                    "${:02X},X @ {:02X} = {:02X}",
                    bytes[1] as u16, addr, val
                )
                .unwrap();

                val
            }
            AddrMode::ZPGY => {
                let addr = self.calc_addr(bytes, AddrMode::ZPGY, penalty);
                let val = self.read(addr, 1)[0];
                write!(
                    stepstring,
                    "${:02X},Y @ {:02X} = {:02X}",
                    bytes[1] as u16, addr, val
                )
                .unwrap();

                val
            }
        };
    }

    //stepping our system can either return an Ok(log string) or an Err(step_error)
    pub fn step(&mut self) -> Result<String, String> {
        //if we are at a breakpoint, take no action, and set our running flag to false
        if self.breakpoints.contains(&(self.cpu.PC as usize)) {
            self.breakpoints.retain(|v| *v != (self.cpu.PC as usize));
            return Err(format!("Hit breakpoint at PC = {:04X}", self.cpu.PC));
        }

        //for debugging, lets build a string to output this step
        let mut stepstring = String::new();

        //print exactly 16 characters composed of:
        // our current addr,
        // the bytes that make up this instr,
        // padding out to 16 chars
        let instr: u8 = self.read(self.cpu.PC, 1)[0];

        //DEBUG
        match self.instr_data.instrs.get(&instr) {
            Some(_v) => {}
            None => {
                unimplemented!(
                    "crashing on unimplemented op: {instr:02x} at PC = {:04X}, cyc = {}",
                    self.cpu.PC,
                    self.cycles
                )
            }
        }

        let bytes = self.read(self.cpu.PC, self.instr_data.instrs[&instr].len);
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
                    0x69 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring, false),
                    0x65 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0x75 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false),
                    0x6D => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0x7D => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, true),
                    0x79 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring, true),
                    0x61 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring, false),
                    0x71 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring, true),
                    _ => {
                        unreachable!("IN ADC, BUT GOT INVALID OPCODE")
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
                    0x29 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring, false),
                    0x25 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0x35 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false),
                    0x2D => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0x3D => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, true),
                    0x39 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring, true),
                    0x21 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring, false),
                    0x31 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring, true),
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
                accumulator	ASL A	0A	1	2
                zeropage	ASL oper	06	2	5
                zeropage,X	ASL oper,X	16	2	6
                absolute	ASL oper	0E	3	6
                absolute,X	ASL oper,X	1E	3	7  */
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                let mut val = match instr {
                    0x0A => self.get_val(&bytes, AddrMode::ACC, &mut stepstring, false),
                    0x06 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0x16 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false),
                    0x0E => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0x1E => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, false),
                    _ => unreachable!("IN ASL BUT GOT BAD OPCOODE"),
                };

                self.cpu.SR.C = (val & 0x80) >> 7 == 1;
                val = (val << 1) & 0b1111_1110;

                self.cpu.SR.N = (val as i8) < 0;
                self.cpu.SR.Z = val == 0;

                match instr {
                    0x0A => self.cpu.ACC = val,
                    0x06 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPG, true);
                        self.write(addr, &vec![val])
                    }
                    0x16 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPGX, true);
                        self.write(addr, &vec![val])
                    }
                    0x0E => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABS, true);
                        self.write(addr, &vec![val])
                    }
                    0x1E => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABSX, true);
                        self.write(addr, &vec![val])
                    }
                    _ => unreachable!("IN LSR BUT GOT BAD OPCOODE"),
                };

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            //TODO: combine all branch ops into one case
            BCC branch on carry clear
                relative	BCC oper	90	2	2** */
            0x90 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                let target = self.calc_addr(&bytes, AddrMode::REL, false);
                write!(stepstring, "${target:04X}").unwrap();

                if !self.cpu.SR.C {
                    //call with true BEFORE UPDATING PC to see if we cross a page
                    self.calc_addr(&bytes, AddrMode::REL, true);
                    //now we update the PC
                    self.cpu.PC = target;
                    //branching always adds a cycle
                    self.cycles += 1;
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            BCS branch on carry set
                relative	BCS oper	B0	2	2** */
            0xB0 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                let target = self.calc_addr(&bytes, AddrMode::REL, false);
                write!(stepstring, "${target:04X}").unwrap();

                if self.cpu.SR.C {
                    //call with true BEFORE UPDATING PC to see if we cross a page
                    self.calc_addr(&bytes, AddrMode::REL, true);
                    //now we update the PC
                    self.cpu.PC = target;
                    //branching always adds a cycle
                    self.cycles += 1;
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            BEQ branch on equal (zero set)
                relative	BEQ oper	F0	2	2** */
            0xF0 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                let target = self.calc_addr(&bytes, AddrMode::REL, false);
                write!(stepstring, "${target:04X}").unwrap();

                if self.cpu.SR.Z {
                    //call with true BEFORE UPDATING PC to see if we cross a page
                    self.calc_addr(&bytes, AddrMode::REL, true);
                    //now we update the PC
                    self.cpu.PC = target;
                    //branching always adds a cycle
                    self.cycles += 1;
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            BIT bit test
                zeropage	BIT oper	24	2	3
                absolute	BIT oper	2C	3	4  */
            0x24 | 0x2C => {
                let val: u8 = if instr == 0x24 {
                    self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false)
                } else {
                    self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false)
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
                let target = self.calc_addr(&bytes, AddrMode::REL, false);
                write!(stepstring, "${target:04X}").unwrap();

                if self.cpu.SR.N {
                    //call with true BEFORE UPDATING PC to see if we cross a page
                    self.calc_addr(&bytes, AddrMode::REL, true);
                    //now we update the PC
                    self.cpu.PC = target;
                    //branching always adds a cycle
                    self.cycles += 1;
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            BNE branch on not equal (zero clear)
                relative	BNE oper	D0	2	2**  */
            0xD0 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                //let target = self.cpu.PC.wrapping_add_signed(bytes[1] as i16);
                //call with false bc we dont know if we're taking it yet
                let target = self.calc_addr(&bytes, AddrMode::REL, false);
                write!(stepstring, "${target:04X}").unwrap();

                if !self.cpu.SR.Z {
                    //call with true BEFORE UPDATING PC to see if we cross a page
                    self.calc_addr(&bytes, AddrMode::REL, true);
                    //now we update the PC
                    self.cpu.PC = target;
                    //branching always adds a cycle
                    self.cycles += 1;
                }
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            BPL branch on plus (negative clear)
                relative	BPL oper	10	2	2** */
            0x10 => {
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                //we always need to compute the target for logging prints
                let target = self.calc_addr(&bytes, AddrMode::REL, false);
                write!(stepstring, "${target:04X}").unwrap();

                if !self.cpu.SR.N {
                    //call with true BEFORE UPDATING PC to see if we cross a page
                    self.calc_addr(&bytes, AddrMode::REL, true);
                    //now we update the PC
                    self.cpu.PC = target;
                    //branching always adds a cycle
                    self.cycles += 1;
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
                self.write(
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
                self.write(self.cpu.SP as u16, &vec![saved_sr.decode()]);
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
                implied	CLI	58	1	*/
            0x58 => {
                self.cpu.SR.I = false;
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
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
                    0xC9 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring, false),
                    0xC5 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0xD5 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false),
                    0xCD => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0xDD => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, true),
                    0xD9 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring, true),
                    0xC1 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring, false),
                    0xD1 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring, true),
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
                    0xE0 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring, false),
                    0xE4 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0xEC => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
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
                    0xC0 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring, false),
                    0xC4 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0xCC => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
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
                zeropage	DEC oper	C6	2	5
                zeropage,X	DEC oper,X	D6	2	6
                absolute	DEC oper	CE	3	6
                absolute,X	DEC oper,X	DE	3	7
            */
            0xC6 | 0xD6 | 0xCE | 0xDE => {
                let addr = match instr {
                    0xC6 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPG, true);
                        self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false);
                        addr
                    }
                    0xD6 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPGX, true);
                        self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false);
                        addr
                    }
                    0xCE => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABS, true);
                        self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false);
                        addr
                    }
                    0xDE => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABSX, true);
                        self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, false);
                        addr
                    }
                    _ => unreachable!("IN INC, BUT GOT BAD OP"),
                };

                let mut val = self.read(addr, 1)[0];
                //write!(stepstring, "{val:02X}").unwrap();

                val = val.wrapping_sub(1);
                self.write(addr, &vec![val]);

                //NZ
                self.cpu.SR.N = (val as i8) < 0;
                self.cpu.SR.Z = val == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
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
                    0x49 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring, false),
                    0x45 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0x55 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false),
                    0x4D => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0x5D => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, true),
                    0x59 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring, true),
                    0x41 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring, false),
                    0x51 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring, true),
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
                zeropage	INC oper	E6	2	5
                zeropage,X	INC oper,X	F6	2	6
                absolute	INC oper	EE	3	6
                absolute,X	INC oper,X	FE	3	7  */
            0xE6 | 0xF6 | 0xEE | 0xFE => {
                let addr = match instr {
                    0xE6 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPG, true);
                        //write!(stepstring, "${addr:02X} = ").unwrap();
                        self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false);
                        addr
                    }
                    0xF6 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPGX, true);
                        self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false);
                        addr
                    }
                    0xEE => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABS, true);
                        self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false);
                        addr
                    }
                    0xFE => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABSX, true);
                        self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, false);
                        addr
                    }
                    _ => unreachable!("IN INC, BUT GOT BAD OP"),
                };

                let mut val = self.read(addr, 1)[0];

                val = val.wrapping_add(1);
                self.write(addr, &vec![val]);

                //NZ
                self.cpu.SR.N = (val as i8) < 0;
                self.cpu.SR.Z = val == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
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
                    //base addr is given by the opcode
                    let base_addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                    //we read the first byte of the target address
                    let new_lo = self.read(base_addr, 1)[0];

                    //we have to make sure we wrap around page boundaries when adding 1
                    let addr_hi = ((base_addr & 0xFF00) >> 8) as u8;
                    let mut addr_lo = (base_addr & 0xFF) as u8;
                    addr_lo = addr_lo.wrapping_add(1);
                    let addr2 = addr_lo as u16 | (addr_hi as u16) << 8;
                    //read the second byte of the target address
                    let new_hi = self.read(addr2, 1)[0];

                    let new_pc = new_lo as u16 | (new_hi as u16) << 8;
                    write!(stepstring, "(${base_addr:04X}) = {new_pc:04X}").unwrap();
                    new_pc
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
                //let addr = self.cpu.SP as u16 + 0x100 as u16;
                let ra = self.cpu.PC + 2;
                let ral = (ra & 0xFF) as u8;
                let rah = ((ra >> 8) & 0xFF) as u8;

                //push high then low
                self.cpu.push(&mut self.wram, rah);
                self.cpu.push(&mut self.wram, ral);

                //self.write(addr, &[ral, rah].to_vec());
                //self.cpu.SP -= 2;
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
                    0xA9 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring, false),
                    0xA5 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0xB5 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false),
                    0xAD => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0xBD => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, true),
                    0xB9 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring, true),
                    0xA1 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring, false),
                    0xB1 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring, true),
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
                    0xA2 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring, false),
                    0xA6 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0xB6 => self.get_val(&bytes, AddrMode::ZPGY, &mut stepstring, false),
                    0xAE => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0xBE => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring, true),
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
                    0xA0 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring, false),
                    0xA4 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0xB4 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false),
                    0xAC => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0xBC => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, true),
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
                accumulator	LSR A	4A	1	2
                zeropage	LSR oper	46	2	5
                zeropage,X	LSR oper,X	56	2	6
                absolute	LSR oper	4E	3	6
                absolute,X	LSR oper,X	5E	3	7 */
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => {
                let mut val = match instr {
                    0x4A => self.get_val(&bytes, AddrMode::ACC, &mut stepstring, false),
                    0x46 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0x56 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false),
                    0x4E => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0x5E => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, false),
                    _ => unreachable!("IN LSR BUT GOT BAD OPCOODE"),
                };

                self.cpu.SR.C = (val & 0x1) == 1;
                val = val >> 1;

                self.cpu.SR.N = false;
                self.cpu.SR.Z = val == 0;

                match instr {
                    0x4A => self.cpu.ACC = val,
                    0x46 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPG, true);
                        self.write(addr, &vec![val])
                    }
                    0x56 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPGX, true);
                        self.write(addr, &vec![val])
                    }
                    0x4E => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABS, true);
                        self.write(addr, &vec![val])
                    }
                    0x5E => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABSX, true);
                        self.write(addr, &vec![val])
                    }
                    _ => unreachable!("IN LSR BUT GOT BAD OPCOODE"),
                };

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
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
                    0x09 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring, false),
                    0x05 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0x15 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false),
                    0x0D => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0x1D => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, true),
                    0x19 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring, true),
                    0x01 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring, false),
                    0x11 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring, true),
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
                //self.write(self.cpu.SP as u16 + 0x100 as u16, &vec![self.cpu.ACC]);
                //self.cpu.SP -= 1;
                self.cpu.push(&mut self.wram, self.cpu.ACC);

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

                //self.write(self.cpu.SP as u16 + 0x100 as u16, &vec![saved_sr.decode()]);
                //self.cpu.SP -= 1;
                self.cpu.push(&mut self.wram, saved_sr.decode());

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            PLA pull accumulator
                implied	PLA	68	1	4  */
            0x68 => {
                //self.cpu.SP += 1;
                //self.cpu.ACC = self.read(self.cpu.SP as u16 + 0x100 as u16, 1)[0];

                self.cpu.ACC = self.cpu.pop(&mut self.wram);

                self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
                self.cpu.SR.Z = self.cpu.ACC == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            PLP pull processor status (SR)
                implied	PLP	28	1	4 */
            0x28 => {
                //self.cpu.SP += 1;
                //let pulled = self.read(self.cpu.SP as u16 + 0x100 as u16, 1)[0];
                let pulled = self.cpu.pop(&mut self.wram);
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
                accumulator	ROL A	2A	1	2
                zeropage	ROL oper	26	2	5
                zeropage,X	ROL oper,X	36	2	6
                absolute	ROL oper	2E	3	6
                absolute,X	ROL oper,X	3E	3	7  */
            0x2A | 0x26 | 0x36 | 0x2E | 0x3E => {
                let mut val: u8 = match instr {
                    0x2A => self.get_val(&bytes, AddrMode::ACC, &mut stepstring, false),
                    0x26 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0x36 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false),
                    0x2E => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0x3E => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, false),
                    _ => unreachable!("IN ROL BUT GOT BAD OP"),
                };

                //ROL shifts all bits left one position.
                //The Carry is shifted into bit 0 and the original bit 7 is shifted into the Carry.
                let new_c: bool = (val & 0x80) == 0x80;
                val = val.rotate_left(1);
                let new_l = if self.cpu.SR.C { 1 } else { 0 };
                val |= new_l;

                self.cpu.SR.N = (val as i8) < 0;
                self.cpu.SR.Z = val == 0;
                self.cpu.SR.C = new_c;

                match instr {
                    0x2A => self.cpu.ACC = val,
                    0x26 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPG, true);
                        self.write(addr, &vec![val])
                    }
                    0x36 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPGX, true);
                        self.write(addr, &vec![val])
                    }
                    0x2E => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABS, true);
                        self.write(addr, &vec![val])
                    }
                    0x3E => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABSX, true);
                        self.write(addr, &vec![val])
                    }
                    _ => unreachable!("IN ROL BUT GOT BAD OPCOODE"),
                };

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            ROR rotate right
                accumulator	ROR A	6A	1	2
                zeropage	ROR oper	66	2	5
                zeropage,X	ROR oper,X	76	2	6
                absolute	ROR oper	6E	3	6
                absolute,X	ROR oper,X	7E	3	7 */
            0x6A | 0x66 | 0x76 | 0x6E | 0x7E => {
                let mut val: u8 = match instr {
                    0x6A => self.get_val(&bytes, AddrMode::ACC, &mut stepstring, false),
                    0x66 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0x76 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false),
                    0x6E => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0x7E => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, false),
                    _ => unreachable!("IN ROR BUT GOT BAD OP"),
                };

                //ROR shifts all bits right one position.
                //The Carry is shifted into bit 0 and the original bit 7 is shifted into the Carry.
                let new_c: bool = (val & 0x1) == 0x1;
                val = val.rotate_right(1);
                let new_hi = if self.cpu.SR.C { 0x80 } else { 0 };
                val &= 0x7F;
                val |= new_hi;

                self.cpu.SR.N = (val as i8) < 0;
                self.cpu.SR.Z = val == 0;
                self.cpu.SR.C = new_c;

                match instr {
                    0x6A => self.cpu.ACC = val,
                    0x66 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPG, true);
                        self.write(addr, &vec![val])
                    }
                    0x76 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPGX, true);
                        self.write(addr, &vec![val])
                    }
                    0x6E => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABS, true);
                        self.write(addr, &vec![val])
                    }
                    0x7E => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABSX, true);
                        self.write(addr, &vec![val])
                    }
                    _ => unreachable!("IN ROR BUT GOT BAD OPCOODE"),
                };

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            RTI return from interrupt
                implied	RTI	40	1	6  */
            0x40 => {
                let new_sr = self.cpu.pop(&mut self.wram);
                self.cpu.SR.encode(new_sr);
                //TODO: THIS IS A FUCKING HACK BC I THINK WE SHOULD ONLY BE ABLE TO GET HERE FROM AN IRQ
                self.cpu.SR.BH = true;

                let new_pc_lo = self.cpu.pop(&mut self.wram) as u16;
                let new_pc_hi = self.cpu.pop(&mut self.wram) as u16;
                self.cpu.PC = new_pc_lo | new_pc_hi << 8;

                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*/
            RTS return from subroutine
                implied	RTS	60	1	6  */
            0x60 => {
                //pull PC, PC+1 -> PC
                //self.cpu.SP += 2;
                //let pull = self.read(self.cpu.SP as u16 + 0x100 as u16, 2);
                let new_pc_lo = self.cpu.pop(&mut self.wram) as u16;
                let new_pc_hi = self.cpu.pop(&mut self.wram) as u16;
                let new_pc: u16 = new_pc_lo | new_pc_hi << 8;
                self.cpu.PC = new_pc;

                self.cpu.PC = self.cpu.PC.wrapping_add(1);

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
                    0xE9 => self.get_val(&bytes, AddrMode::IMM, &mut stepstring, false),
                    0xE5 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0xF5 => self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false),
                    0xED => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0xFD => self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, true),
                    0xF9 => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring, true),
                    0xE1 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring, false),
                    0xF1 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring, true),
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
                        let addr = self.calc_addr(&bytes, AddrMode::ZPG, false);
                        self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false);

                        addr
                    }
                    0x95 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ZPGX, false);
                        self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false);

                        addr
                    }
                    0x8D => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABS, false);
                        self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false);

                        addr
                    }
                    0x9D => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABSX, false);
                        self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, false);

                        addr
                    }
                    0x99 => {
                        let addr = self.calc_addr(&bytes, AddrMode::ABSY, false);
                        self.get_val(&bytes, AddrMode::ABSY, &mut stepstring, false);
                        addr
                    }
                    0x81 => {
                        let addr = self.calc_addr(&bytes, AddrMode::INDX, false);

                        self.get_val(&bytes, AddrMode::INDX, &mut stepstring, false);
                        addr
                    }
                    0x91 => {
                        let addr = self.calc_addr(&bytes, AddrMode::INDY, false);
                        self.get_val(&bytes, AddrMode::INDY, &mut stepstring, false);
                        addr
                    }
                    _ => {
                        panic!("IN STA BUT INVALID OP")
                    }
                };

                self.write(addr, &vec![self.cpu.ACC]);

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
                    let addr = self.calc_addr(&bytes, AddrMode::ZPG, false);
                    let cur_val = self.read(addr, 1)[0];
                    self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false);
                    addr
                } else if instr == 0x96 {
                    let addr = self.calc_addr(&bytes, AddrMode::ZPGY, false);
                    self.get_val(&bytes, AddrMode::ZPGY, &mut stepstring, false);

                    addr
                } else {
                    let addr = self.calc_addr(&bytes, AddrMode::ABS, false);
                    let cur_val = self.read(addr, 1)[0];
                    self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false);
                    addr
                };

                self.write(addr, &[self.cpu.X].to_vec());

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            STY store Y
                zeropage	STY oper	84	2	3
                zeropage,X	STY oper,X	94	2	4
                absolute	STY oper	8C	3	4  */
            0x84 | 0x94 | 0x8C => {
                let addr = if instr == 0x84 {
                    let addr = self.calc_addr(&bytes, AddrMode::ZPG, false);
                    let cur_val = self.read(addr, 1)[0];
                    self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false);
                    addr
                } else if instr == 0x94 {
                    let addr = self.calc_addr(&bytes, AddrMode::ZPGX, false);
                    self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false);
                    addr
                } else {
                    let addr = self.calc_addr(&bytes, AddrMode::ABS, false);
                    let cur_val = self.read(addr, 1)[0];
                    self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false);

                    addr
                };

                self.write(addr, &[self.cpu.Y].to_vec());

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
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

                self.cpu.SR.N = (self.cpu.X as i8) < 0;
                self.cpu.SR.Z = self.cpu.X == 0;

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

            ///////////////////////////////////////////////////////////////////////////////////////
            //here be unofficial ops
            /* *NOP
                1A	implied	1	2
                3A	implied	1	2
                5A	implied	1	2
                7A	implied	1	2
                DA	implied	1	2
                FA	implied	1	2
                80	immediate	2	2
                82	immediate	2	2
                89	immediate	2	2
                C2	immediate	2	2
                E2	immediate	2	2
                04	zeropage	2	3
                44	zeropage	2	3
                64	zeropage	2	3
                14	zeropage,X	2	4
                34	zeropage,X	2	4
                54	zeropage,X	2	4
                74	zeropage,X	2	4
                D4	zeropage,X	2	4
                F4	zeropage,X	2	4
                0C	absolute	3	4
                1C	absolut,X	3	4*
                3C	absolut,X	3	4*
                5C	absolut,X	3	4*
                7C	absolut,X	3	4*
                DC	absolut,X	3	4*
                FC	absolut,X	3	4*
            */
            0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA | 0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 | 0x04
            | 0x44 | 0x64 | 0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 | 0x0C | 0x1C | 0x3C | 0x5C
            | 0x7C | 0xDC | 0xFC => {
                //these are literally all nops are you fucking kidding me
                //ARE YOU FUCKING KIDDING ME NESTEST FORMATS THEM ALL DIFFERENTLY??
                stepstring.remove(stepstring.len() - 6);

                //TODO: we have to write to the stepstring manually because reading from memory might access an address we are not allowed to
                match instr {
                    //implied
                    0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => {}
                    //imm
                    0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => {
                        write!(stepstring, "#${:02X}", bytes[1]).unwrap()
                    }
                    //zpg
                    0x04 | 0x44 | 0x64 => {
                        self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false);
                    }
                    //zpgx
                    0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 => {
                        self.get_val(&bytes, AddrMode::ZPGX, &mut stepstring, false);
                    }
                    //absolute
                    0x0C => {
                        self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false);
                    }
                    //absolute X
                    0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => {
                        self.get_val(&bytes, AddrMode::ABSX, &mut stepstring, true);
                    }
                    _ => unreachable!("go fuck yourself {instr:02X}"),
                }
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }

            /*
            LAX
                zeropage	LAX oper	A7	2	3
                zeropage,Y	LAX oper,Y	B7	2	4
                absolute	LAX oper	AF	3	4
                absolut,Y	LAX oper,Y	BF	3	4*
                (indirect,X)	LAX (oper,X)	A3	2	6
                (indirect),Y	LAX (oper),Y	B3	2	5*
            */
            0xA7 | 0xB7 | 0xAF | 0xBF | 0xA3 | 0xB3 => {
                stepstring.remove(stepstring.len() - 6);
                let val = match instr {
                    0xA7 => self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false),
                    0xB7 => self.get_val(&bytes, AddrMode::ZPGY, &mut stepstring, false),
                    0xAF => self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false),
                    0xBF => self.get_val(&bytes, AddrMode::ABSY, &mut stepstring, true),
                    0xA3 => self.get_val(&bytes, AddrMode::INDX, &mut stepstring, false),
                    0xB3 => self.get_val(&bytes, AddrMode::INDY, &mut stepstring, true),
                    _ => unreachable!("IN LAX BUT GOT BAD OP"),
                };

                self.cpu.ACC = val;
                self.cpu.X = val;

                self.cpu.SR.N = (val as i8) < 0;
                self.cpu.SR.Z = val == 0;

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            SAX
                zeropage	SAX oper	87	2	3
                zeropage,Y	SAX oper,Y	97	2	4
                absolute	SAX oper	8F	3	4
                (indirect,X)	SAX (oper,X)	83	2	6
            */
            0x87 | 0x97 | 0x8F | 0x83 => {
                stepstring.remove(stepstring.len() - 6);
                let addr = match instr {
                    0x87 => {
                        self.get_val(&bytes, AddrMode::ZPG, &mut stepstring, false);

                        self.calc_addr(&bytes, AddrMode::ZPG, false)
                    }
                    0x97 => {
                        self.get_val(&bytes, AddrMode::ZPGY, &mut stepstring, false);

                        self.calc_addr(&bytes, AddrMode::ZPGY, false)
                    }
                    0x8F => {
                        self.get_val(&bytes, AddrMode::ABS, &mut stepstring, false);

                        self.calc_addr(&bytes, AddrMode::ABS, false)
                    }
                    0x83 => {
                        self.get_val(&bytes, AddrMode::INDX, &mut stepstring, false);

                        self.calc_addr(&bytes, AddrMode::INDX, false)
                    }
                    _ => unreachable!("IN LAX BUT GOT BAD OP"),
                };

                let val = self.cpu.ACC & self.cpu.X;
                self.write(addr, &vec![val]);

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
            }
            /*
            USBC
                immediate	USBC #oper	EB	2	2
            */
            0xEB => {
                stepstring.remove(stepstring.len() - 6);

                let val = self.get_val(&bytes, AddrMode::IMM, &mut stepstring, false);
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
            /*DCP
                zeropage	DCP oper	C7	2	5
                zeropage,X	DCP oper,X	D7	2	6
                absolute	DCP oper	CF	3	6
                absolut,X	DCP oper,X	DF	3	7
                absolut,Y	DCP oper,Y	DB	3	7
                (indirect,X)	DCP (oper,X)	C3	2	8
                (indirect),Y	DCP (oper),Y	D3	2	8
            */
            0xC7 | 0xD7 | 0xCF | 0xDF | 0xDB | 0xC3 | 0xD3 => {}
            /*
            ISB
                zeropage	ISC oper	E7	2	5
                zeropage,X	ISC oper,X	F7	2	6
                absolute	ISC oper	EF	3	6
                absolut,X	ISC oper,X	FF	3	7
                absolut,Y	ISC oper,Y	FB	3	7
                (indirect,X)	ISC (oper,X)	E3	2	8
                (indirect),Y	ISC (oper),Y	F3	2	4  */
            0xE7 | 0xF7 | 0xEF | 0xFF | 0xFB | 0xE3 | 0xF3 => {}
            /*
            SLO
                zeropage	SLO oper	07	2	5
                zeropage,X	SLO oper,X	17	2	6
                absolute	SLO oper	0F	3	6
                absolut,X	SLO oper,X	1F	3	7
                absolut,Y	SLO oper,Y	1B	3	7
                (indirect,X)	SLO (oper,X)	03	2	8
                (indirect),Y	SLO (oper),Y	13	2	8  */
            0x07 | 0x17 | 0x0F | 0x1F | 0x1B | 0x03 | 0x13 => {}

            /*
            RLA
                zeropage	RLA oper	27	2	5
                zeropage,X	RLA oper,X	37	2	6
                absolute	RLA oper	2F	3	6
                absolut,X	RLA oper,X	3F	3	7
                absolut,Y	RLA oper,Y	3B	3	7
                (indirect,X)	RLA (oper,X)	23	2	8
                (indirect),Y	RLA (oper),Y	33	2	8
            */
            0x27 | 0x37 | 0x2F | 0x3F | 0x3B | 0x23 | 0x33 => {}
            /*
            SRE
                zeropage	SRE oper	47	2	5
                zeropage,X	SRE oper,X	57	2	6
                absolute	SRE oper	4F	3	6
                absolut,X	SRE oper,X	5F	3	7
                absolut,Y	SRE oper,Y	5B	3	7
                (indirect,X)	SRE (oper,X)	43	2	8
                (indirect),Y	SRE (oper),Y	53	2	8 */
            0x47 | 0x57 | 0x4F | 0x5F | 0x5B | 0x43 | 0x53 => {}
            /*
            RRA
                zeropage	RRA oper	67	2	5
                zeropage,X	RRA oper,X	77	2	6
                absolute	RRA oper	6F	3	6
                absolut,X	RRA oper,X	7F	3	7
                absolut,Y	RRA oper,Y	7B	3	7
                (indirect,X)	RRA (oper,X)	63	2	8
                (indirect),Y	RRA (oper),Y	73	2	8 */
            0x67 | 0x77 | 0x6F | 0x7F | 0x7B | 0x63 | 0x73 => {}
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

        return Ok(stepstring);
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

    //to handle reads to other parts of the system, we must pass in refs to every other component
    //read n bytes from address a
    pub fn read(&mut self, addr: u16, length: usize) -> Vec<u8> {
        for a in addr as usize..=(addr as usize + length) {
            if self.watchpoints.contains(&(a as usize)) {
                //self.running = false;
                /*self.channels
                .log_channel
                .send("HALTING BC WE HIT A MEMORY WATCHPOINT".to_string())
                .unwrap();*/
            }
        }

        match addr {
            //WRAM(2kb) + 3 mirrors
            0x0000..=0x1FFF => {
                //mod by 2048 since we have 3 mirrors
                let final_addr = addr % 2048;

                return self.wram.contents[final_addr as usize..final_addr as usize + length]
                    .into();
                //return 0;
            }
            //PPU control regs a PM at gs (8 bytes) + a fuckton of mirrors
            0x2000..=0x3FFF => {
                //return [].into();
                unimplemented!("tried to read ppu control regs")
            }
            //registers (apu and io)
            0x4000..=0x4017 => {
                //return [].into();
                unimplemented!("tried to read apu/io regs")
            }
            //cart expansion
            0x4018..=0x5FFF => {
                //return [].into();
                unimplemented!("tried to read cart expansion")
            }
            //cart SRAM (8k)
            0x6000..=0x7FFF => {
                //return [].into();
                unimplemented!("tried to read cart SRAM")
            }
            //PRG-ROM (32K)
            0x8000..=0xFFFF => self.cart.read(addr, length).into(),
        }
    }
    pub fn write(&mut self, addr: u16, bytes: &Vec<u8>) {
        for a in addr as usize..=(addr as usize + bytes.len()) {
            if self.watchpoints.contains(&(a as usize)) {
                //self.running = false;
                /*self.channels
                .log_channel
                .send("HALTING BC WE HIT A MEMORY WATCHPOINT".to_string())
                .unwrap();*/
            }
        }

        match addr {
            //WRAM(2kb) + 3 mirrors
            0x0000..=0x1FFF => {
                let base_addr = addr % 2048;

                for (i, b) in bytes.iter().enumerate() {
                    //write value into ram
                    self.wram.contents[(base_addr as usize) + i] = *b;
                    //make sure we also send this value to the frontend
                    /*self.channels
                    .wram_channel
                    .send((((base_addr as usize) + i), *b))
                    .unwrap();*/
                }
            }
            //PPU control regs (8 bytes) + a fuckton of mirrors
            0x2000..=0x3FFF => {
                unimplemented!("tried to write to ppu control regs")
            }
            //registers (apu and io)
            0x4000..=0x4017 => {
                unimplemented!("tried to wrote to apu/io regs")
            }
            //cart expansion
            0x4018..=0x5FFF => {
                unimplemented!("tried to write to cart expansion?")
            }
            //cart SRAM (8k)
            0x6000..=0x7FFF => {
                unimplemented!("tried to write to cart SRAM")
            }
            //PRG-ROM (32K)
            0x8000..=0xFFFF => {
                unimplemented!("tried to write to prg-rom??")
            }
        }
    }
}

pub fn print_bytes(bytes: &Vec<u8>) -> String {
    let mut ret_str = String::new();

    for b in bytes {
        ret_str.push_str(&format!("{:02X} ", b));
    }
    return ret_str;
}
