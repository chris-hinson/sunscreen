use crate::nes::AddrMode;
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Clone)]

pub struct InstrData {
    //printable name
    pub name: String,
    //lengths for this instruction
    pub len: usize,
    //cycles for this instruction
    pub cycles: usize,
}
impl InstrData {
    pub fn new(name: &str, len: usize, cycles: usize) -> Self {
        InstrData {
            name: name.to_string(),
            len,
            cycles,
        }
    }
}

#[derive(Clone)]

pub struct Instr {
    //hashmap of opcode to instr data
    //TODO: could instead use a vec ordered by opcode for constant lookup
    //ignore above, this should already by constant lookup
    //later note: hashmap is already constant lookup lol? might look neater as a vec tho.
    //look at peachy's crystal emulator
    pub instrs: HashMap<u8, InstrData>,
}

impl Instr {
    pub fn new() -> Self {
        let map = HashMap::from([
            (0x69, InstrData::new("ADC", 2, 2)),
            (0x65, InstrData::new("ADC", 2, 3)),
            (0x75, InstrData::new("ADC", 2, 4)),
            (0x6D, InstrData::new("ADC", 3, 4)),
            (0x7D, InstrData::new("ADC", 3, 4)),
            (0x79, InstrData::new("ADC", 3, 4)),
            (0x61, InstrData::new("ADC", 2, 6)),
            (0x71, InstrData::new("ADC", 2, 5)),
            (0x29, InstrData::new("AND", 2, 2)),
            (0x25, InstrData::new("AND", 2, 3)),
            (0x35, InstrData::new("AND", 2, 4)),
            (0x2D, InstrData::new("AND", 3, 4)),
            (0x3D, InstrData::new("AND", 3, 4)),
            (0x39, InstrData::new("AND", 3, 4)),
            (0x21, InstrData::new("AND", 2, 6)),
            (0x31, InstrData::new("AND", 2, 5)),
            (0x4C, InstrData::new("JMP", 3, 3)),
            (0x6C, InstrData::new("JMP", 3, 5)),
            (0xA2, InstrData::new("LDX", 2, 2)),
            (0xA6, InstrData::new("LDX", 2, 3)),
            (0xB6, InstrData::new("LDX", 2, 4)),
            (0xAE, InstrData::new("LDX", 3, 4)),
            (0xBE, InstrData::new("LDX", 3, 4)),
            (0x86, InstrData::new("STX", 2, 3)),
            (0x96, InstrData::new("STX", 2, 4)),
            (0x8E, InstrData::new("STX", 3, 4)),
            (0x20, InstrData::new("JSR", 3, 6)),
            (0xEA, InstrData::new("NOP", 1, 2)),
            (0x38, InstrData::new("SEC", 1, 2)),
            (0xB0, InstrData::new("BCS", 2, 2)),
            (0x18, InstrData::new("CLC", 1, 2)),
            (0x90, InstrData::new("BCC", 2, 2)),
            (0xA9, InstrData::new("LDA", 2, 2)),
            (0xA5, InstrData::new("LDA", 2, 3)),
            (0xB5, InstrData::new("LDA", 2, 4)),
            (0xAD, InstrData::new("LDA", 3, 4)),
            (0xBD, InstrData::new("LDA", 3, 4)),
            (0xB9, InstrData::new("LDA", 3, 4)),
            (0xA1, InstrData::new("LDA", 2, 6)),
            (0xB1, InstrData::new("LDA", 2, 5)),
            (0xF0, InstrData::new("BEQ", 2, 2)),
            (0xD0, InstrData::new("BNE", 2, 2)),
            (0x85, InstrData::new("STA", 2, 3)),
            (0x95, InstrData::new("STA", 2, 4)),
            (0x8D, InstrData::new("STA", 3, 4)),
            (0x9D, InstrData::new("STA", 3, 5)),
            (0x99, InstrData::new("STA", 3, 5)),
            (0x81, InstrData::new("STA", 2, 6)),
            (0x91, InstrData::new("STA", 2, 6)),
            (0x24, InstrData::new("BIT", 2, 3)),
            (0x2C, InstrData::new("BIT", 3, 4)),
            (0x70, InstrData::new("BVS", 2, 2)),
            (0x50, InstrData::new("BVC", 2, 2)),
            (0x10, InstrData::new("BPL", 2, 2)),
            (0x60, InstrData::new("RTS", 1, 6)),
            (0x00, InstrData::new("BRK", 1, 7)),
            (0x78, InstrData::new("SEI", 1, 2)),
            (0xF8, InstrData::new("SED", 1, 2)),
            (0x08, InstrData::new("PHP", 1, 3)),
            (0x68, InstrData::new("PLA", 1, 4)),
            (0x29, InstrData::new("AND", 2, 2)),
            (0x25, InstrData::new("AND", 2, 3)),
            (0x35, InstrData::new("AND", 2, 4)),
            (0x2D, InstrData::new("AND", 3, 4)),
            (0x3D, InstrData::new("AND", 3, 4)),
            (0x39, InstrData::new("AND", 3, 4)),
            (0x21, InstrData::new("AND", 2, 6)),
            (0x31, InstrData::new("AND", 2, 5)),
            (0xC9, InstrData::new("CMP", 2, 2)),
            (0xC5, InstrData::new("CMP", 2, 3)),
            (0xD5, InstrData::new("CMP", 2, 4)),
            (0xCD, InstrData::new("CMP", 3, 4)),
            (0xDD, InstrData::new("CMP", 3, 4)),
            (0xD9, InstrData::new("CMP", 3, 4)),
            (0xC1, InstrData::new("CMP", 2, 6)),
            (0xD1, InstrData::new("CMP", 2, 5)),
            (0xD8, InstrData::new("CLD", 1, 2)),
            (0x48, InstrData::new("PHA", 1, 3)),
            (0x28, InstrData::new("PLP", 1, 4)),
            (0x30, InstrData::new("BMI", 2, 2)),
            (0x09, InstrData::new("ORA", 2, 2)),
            (0x05, InstrData::new("ORA", 2, 3)),
            (0x15, InstrData::new("ORA", 2, 4)),
            (0x0D, InstrData::new("ORA", 3, 4)),
            (0x1D, InstrData::new("ORA", 3, 4)),
            (0x19, InstrData::new("ORA", 3, 4)),
            (0x01, InstrData::new("ORA", 2, 6)),
            (0x11, InstrData::new("ORA", 2, 5)),
            (0xB8, InstrData::new("CLV", 1, 2)),
            (0x49, InstrData::new("EOR", 2, 2)),
            (0x45, InstrData::new("EOR", 2, 3)),
            (0x55, InstrData::new("EOR", 2, 4)),
            (0x4D, InstrData::new("EOR", 3, 4)),
            (0x5D, InstrData::new("EOR", 3, 4)),
            (0x59, InstrData::new("EOR", 3, 4)),
            (0x41, InstrData::new("EOR", 2, 6)),
            (0x51, InstrData::new("EOR", 2, 5)),
            (0xA0, InstrData::new("LDY", 2, 2)),
            (0xA4, InstrData::new("LDY", 2, 3)),
            (0xB4, InstrData::new("LDY", 2, 4)),
            (0xAC, InstrData::new("LDY", 3, 4)),
            (0xBC, InstrData::new("LDY", 3, 4)),
            (0xE0, InstrData::new("CPX", 2, 2)),
            (0xE4, InstrData::new("CPX", 2, 3)),
            (0xEC, InstrData::new("CPX", 3, 4)),
            (0xC0, InstrData::new("CPY", 2, 2)),
            (0xC4, InstrData::new("CPY", 2, 3)),
            (0xCC, InstrData::new("CPY", 3, 4)),
            (0xE9, InstrData::new("SBC", 2, 2)),
            (0xE5, InstrData::new("SBC", 2, 3)),
            (0xF5, InstrData::new("SBC", 2, 4)),
            (0xED, InstrData::new("SBC", 3, 4)),
            (0xFD, InstrData::new("SBC", 3, 4)),
            (0xF9, InstrData::new("SBC", 3, 4)),
            (0xE1, InstrData::new("SBC", 2, 6)),
            (0xF1, InstrData::new("SBC", 2, 5)),
            (0xE8, InstrData::new("INX", 1, 2)),
            (0xC8, InstrData::new("INY", 1, 2)),
            (0xCA, InstrData::new("DEX", 1, 2)),
            (0x88, InstrData::new("DEY", 1, 2)),
            (0xAA, InstrData::new("TAX", 1, 2)),
            (0xA8, InstrData::new("TAY", 1, 2)),
            (0xBA, InstrData::new("TSX", 1, 2)),
            (0x8A, InstrData::new("TXA", 1, 2)),
            (0x9A, InstrData::new("TXS", 1, 2)),
            (0x98, InstrData::new("TYA", 1, 2)),
            (0x40, InstrData::new("RTI", 1, 6)),
            (0x4A, InstrData::new("LSR", 1, 2)),
            (0x46, InstrData::new("LSR", 2, 5)),
            (0x56, InstrData::new("LSR", 2, 6)),
            (0x4E, InstrData::new("LSR", 3, 6)),
            (0x5E, InstrData::new("LSR", 3, 7)),
            (0x0A, InstrData::new("ASL", 1, 2)),
            (0x06, InstrData::new("ASL", 2, 5)),
            (0x16, InstrData::new("ASL", 2, 6)),
            (0x0E, InstrData::new("ASL", 3, 6)),
            (0x1E, InstrData::new("ASL", 3, 7)),
            (0x58, InstrData::new("CLI", 1, 2)),
            (0x84, InstrData::new("STY", 2, 3)),
            (0x94, InstrData::new("STY", 2, 4)),
            (0x8C, InstrData::new("STY", 3, 4)),
            (0x2A, InstrData::new("ROL", 1, 2)),
            (0x26, InstrData::new("ROL", 2, 5)),
            (0x36, InstrData::new("ROL", 2, 6)),
            (0x2E, InstrData::new("ROL", 3, 6)),
            (0x3E, InstrData::new("ROL", 3, 7)),
            (0x6A, InstrData::new("ROR", 1, 2)),
            (0x66, InstrData::new("ROR", 2, 5)),
            (0x76, InstrData::new("ROR", 2, 6)),
            (0x6E, InstrData::new("ROR", 3, 6)),
            (0x7E, InstrData::new("ROR", 3, 7)),
            (0xE6, InstrData::new("INC", 2, 5)),
            (0xF6, InstrData::new("INC", 2, 6)),
            (0xEE, InstrData::new("INC", 3, 6)),
            (0xFE, InstrData::new("INC", 3, 7)),
            (0xC6, InstrData::new("DEC", 2, 5)),
            (0xD6, InstrData::new("DEC", 2, 6)),
            (0xCE, InstrData::new("DEC", 3, 6)),
            (0xDE, InstrData::new("DEC", 3, 7)),
            //here be unofficial ops.
            (0x1A, InstrData::new("*NOP", 1, 2)),
            (0x3A, InstrData::new("*NOP", 1, 2)),
            (0x5A, InstrData::new("*NOP", 1, 2)),
            (0x7A, InstrData::new("*NOP", 1, 2)),
            (0xDA, InstrData::new("*NOP", 1, 2)),
            (0xFA, InstrData::new("*NOP", 1, 2)),
            (0x80, InstrData::new("*NOP", 2, 2)),
            (0x82, InstrData::new("*NOP", 2, 2)),
            (0x89, InstrData::new("*NOP", 2, 2)),
            (0xC2, InstrData::new("*NOP", 2, 2)),
            (0xE2, InstrData::new("*NOP", 2, 2)),
            (0x04, InstrData::new("*NOP", 2, 3)),
            (0x44, InstrData::new("*NOP", 2, 3)),
            (0x64, InstrData::new("*NOP", 2, 3)),
            (0x14, InstrData::new("*NOP", 2, 4)),
            (0x34, InstrData::new("*NOP", 2, 4)),
            (0x54, InstrData::new("*NOP", 2, 4)),
            (0x74, InstrData::new("*NOP", 2, 4)),
            (0xD4, InstrData::new("*NOP", 2, 4)),
            (0xF4, InstrData::new("*NOP", 2, 4)),
            (0x0C, InstrData::new("*NOP", 3, 4)),
            (0x1C, InstrData::new("*NOP", 3, 4)),
            (0x3C, InstrData::new("*NOP", 3, 4)),
            (0x5C, InstrData::new("*NOP", 3, 4)),
            (0x7C, InstrData::new("*NOP", 3, 4)),
            (0xDC, InstrData::new("*NOP", 3, 4)),
            (0xFC, InstrData::new("*NOP", 3, 4)),
            (0xA7, InstrData::new("*LAX", 2, 3)),
            (0xB7, InstrData::new("*LAX", 2, 4)),
            (0xAF, InstrData::new("*LAX", 3, 4)),
            (0xBF, InstrData::new("*LAX", 3, 4)),
            (0xA3, InstrData::new("*LAX", 2, 6)),
            (0xB3, InstrData::new("*LAX", 2, 5)),
            (0x87, InstrData::new("*SAX", 2, 3)),
            (0x97, InstrData::new("*SAX", 2, 4)),
            (0x8F, InstrData::new("*SAX", 3, 4)),
            (0x83, InstrData::new("*SAX", 2, 6)),
            (0xEB, InstrData::new("*SBC", 2, 2)),
            (0xC7, InstrData::new("*DCP", 2, 5)),
            (0xD7, InstrData::new("*DCP", 2, 6)),
            (0xCF, InstrData::new("*DCP", 3, 6)),
            (0xDF, InstrData::new("*DCP", 3, 7)),
            (0xDB, InstrData::new("*DCP", 3, 7)),
            (0xC3, InstrData::new("*DCP", 2, 8)),
            (0xD3, InstrData::new("*DCP", 2, 8)),
            (0xE7, InstrData::new("*ISB", 2, 5)),
            (0xF7, InstrData::new("*ISB", 2, 6)),
            (0xEF, InstrData::new("*ISB", 3, 6)),
            (0xFF, InstrData::new("*ISB", 3, 7)),
            (0xFB, InstrData::new("*ISB", 3, 7)),
            (0xE3, InstrData::new("*ISB", 2, 8)),
            //TODO: ACCURATE OR NOT?
            (0xF3, InstrData::new("*ISB", 2, 8)),
            (0x07, InstrData::new("*SLO", 2, 5)),
            (0x17, InstrData::new("*SLO", 2, 6)),
            (0x0F, InstrData::new("*SLO", 3, 6)),
            (0x1F, InstrData::new("*SLO", 3, 7)),
            (0x1B, InstrData::new("*SLO", 3, 7)),
            (0x03, InstrData::new("*SLO", 2, 8)),
            (0x13, InstrData::new("*SLO", 2, 8)),
            (0x27, InstrData::new("*RLA", 2, 5)),
            (0x37, InstrData::new("*RLA", 2, 6)),
            (0x2F, InstrData::new("*RLA", 3, 6)),
            (0x3F, InstrData::new("*RLA", 3, 7)),
            (0x3B, InstrData::new("*RLA", 3, 7)),
            (0x23, InstrData::new("*RLA", 2, 8)),
            (0x33, InstrData::new("*RLA", 2, 8)),
            (0x47, InstrData::new("*SRE", 2, 5)),
            (0x57, InstrData::new("*SRE", 2, 6)),
            (0x4F, InstrData::new("*SRE", 3, 6)),
            (0x5F, InstrData::new("*SRE", 3, 7)),
            (0x5B, InstrData::new("*SRE", 3, 7)),
            (0x43, InstrData::new("*SRE", 2, 8)),
            (0x53, InstrData::new("*SRE", 2, 8)),
            (0x67, InstrData::new("*RRA", 2, 5)),
            (0x77, InstrData::new("*RRA", 2, 6)),
            (0x6F, InstrData::new("*RRA", 3, 6)),
            (0x7F, InstrData::new("*RRA", 3, 7)),
            (0x7B, InstrData::new("*RRA", 3, 7)),
            (0x63, InstrData::new("*RRA", 2, 8)),
            (0x73, InstrData::new("*RRA", 2, 8)),
        ]);

        Instr { instrs: map }
    }
}

#[allow(non_snake_case, unused_variables)]
impl crate::NES {
    pub fn ADC(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let val: u8 = match instr {
            0x69 => self.get_val(&bytes, AddrMode::IMM, stepstring, false),
            0x65 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0x75 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0x6D => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0x7D => self.get_val(&bytes, AddrMode::ABSX, stepstring, true),
            0x79 => self.get_val(&bytes, AddrMode::ABSY, stepstring, true),
            0x61 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0x71 => self.get_val(&bytes, AddrMode::INDY, stepstring, true),
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
    pub fn AND(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let val: u8 = match instr {
            0x29 => self.get_val(&bytes, AddrMode::IMM, stepstring, false),
            0x25 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0x35 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0x2D => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0x3D => self.get_val(&bytes, AddrMode::ABSX, stepstring, true),
            0x39 => self.get_val(&bytes, AddrMode::ABSY, stepstring, true),
            0x21 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0x31 => self.get_val(&bytes, AddrMode::INDY, stepstring, true),
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
    pub fn ASL(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let mut val = match instr {
            0x0A => self.get_val(&bytes, AddrMode::ACC, stepstring, false),
            0x06 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0x16 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0x0E => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0x1E => self.get_val(&bytes, AddrMode::ABSX, stepstring, false),
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
    pub fn BCC(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn BCS(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn BEQ(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn BIT(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let val: u8 = if instr == 0x24 {
            self.get_val(&bytes, AddrMode::ZPG, stepstring, false)
        } else {
            self.get_val(&bytes, AddrMode::ABS, stepstring, false)
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
    pub fn BMI(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn BNE(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn BPL(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn BRK(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn BVC(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn BVS(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn CLC(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.SR.C = false;
        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn CLD(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.SR.D = false;
        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn CLI(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.SR.I = false;
        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn CLV(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.SR.V = false;
        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn CMP(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let val: u8 = match instr {
            0xC9 => self.get_val(&bytes, AddrMode::IMM, stepstring, false),
            0xC5 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0xD5 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0xCD => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0xDD => self.get_val(&bytes, AddrMode::ABSX, stepstring, true),
            0xD9 => self.get_val(&bytes, AddrMode::ABSY, stepstring, true),
            0xC1 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0xD1 => self.get_val(&bytes, AddrMode::INDY, stepstring, true),
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
    pub fn CPX(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let val: u8 = match instr {
            0xE0 => self.get_val(&bytes, AddrMode::IMM, stepstring, false),
            0xE4 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0xEC => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
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
    pub fn CPY(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let val: u8 = match instr {
            0xC0 => self.get_val(&bytes, AddrMode::IMM, stepstring, false),
            0xC4 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0xCC => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
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
    pub fn DEC(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let addr = match instr {
            0xC6 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPG, true);
                self.get_val(&bytes, AddrMode::ZPG, stepstring, false);
                addr
            }
            0xD6 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPGX, true);
                self.get_val(&bytes, AddrMode::ZPGX, stepstring, false);
                addr
            }
            0xCE => {
                let addr = self.calc_addr(&bytes, AddrMode::ABS, true);
                self.get_val(&bytes, AddrMode::ABS, stepstring, false);
                addr
            }
            0xDE => {
                let addr = self.calc_addr(&bytes, AddrMode::ABSX, true);
                self.get_val(&bytes, AddrMode::ABSX, stepstring, false);
                addr
            }
            _ => unreachable!("IN INC, BUT GOT BAD OP"),
        };

        let mut val = self.read(addr, 1)[0];

        val = val.wrapping_sub(1);
        self.write(addr, &vec![val]);

        //NZ
        self.cpu.SR.N = (val as i8) < 0;
        self.cpu.SR.Z = val == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn DEX(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.X = self.cpu.X.wrapping_sub(1);

        self.cpu.SR.N = (self.cpu.X as i8) < 0;
        self.cpu.SR.Z = self.cpu.X == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn DEY(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.Y = self.cpu.Y.wrapping_sub(1);

        self.cpu.SR.N = (self.cpu.Y as i8) < 0;
        self.cpu.SR.Z = self.cpu.Y == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn EOR(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let val: u8 = match instr {
            0x49 => self.get_val(&bytes, AddrMode::IMM, stepstring, false),
            0x45 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0x55 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0x4D => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0x5D => self.get_val(&bytes, AddrMode::ABSX, stepstring, true),
            0x59 => self.get_val(&bytes, AddrMode::ABSY, stepstring, true),
            0x41 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0x51 => self.get_val(&bytes, AddrMode::INDY, stepstring, true),
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
    pub fn INC(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let addr = match instr {
            0xE6 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPG, true);
                //write!(stepstring, "${addr:02X} = ").unwrap();
                self.get_val(&bytes, AddrMode::ZPG, stepstring, false);
                addr
            }
            0xF6 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPGX, true);
                self.get_val(&bytes, AddrMode::ZPGX, stepstring, false);
                addr
            }
            0xEE => {
                let addr = self.calc_addr(&bytes, AddrMode::ABS, true);
                self.get_val(&bytes, AddrMode::ABS, stepstring, false);
                addr
            }
            0xFE => {
                let addr = self.calc_addr(&bytes, AddrMode::ABSX, true);
                self.get_val(&bytes, AddrMode::ABSX, stepstring, false);
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
    pub fn INX(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.X = self.cpu.X.wrapping_add(1);

        self.cpu.SR.N = (self.cpu.X as i8) < 0;
        self.cpu.SR.Z = self.cpu.X == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn INY(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.Y = self.cpu.Y.wrapping_add(1);

        self.cpu.SR.N = (self.cpu.Y as i8) < 0;
        self.cpu.SR.Z = self.cpu.Y == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn JMP(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn JSR(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn LDA(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let val: u8 = match instr {
            0xA9 => self.get_val(&bytes, AddrMode::IMM, stepstring, false),
            0xA5 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0xB5 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0xAD => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0xBD => self.get_val(&bytes, AddrMode::ABSX, stepstring, true),
            0xB9 => self.get_val(&bytes, AddrMode::ABSY, stepstring, true),
            0xA1 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0xB1 => self.get_val(&bytes, AddrMode::INDY, stepstring, true),
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
    pub fn LDX(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let val = match instr {
            0xA2 => self.get_val(&bytes, AddrMode::IMM, stepstring, false),
            0xA6 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0xB6 => self.get_val(&bytes, AddrMode::ZPGY, stepstring, false),
            0xAE => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0xBE => self.get_val(&bytes, AddrMode::ABSY, stepstring, true),
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
    pub fn LDY(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let val = match instr {
            0xA0 => self.get_val(&bytes, AddrMode::IMM, stepstring, false),
            0xA4 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0xB4 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0xAC => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0xBC => self.get_val(&bytes, AddrMode::ABSX, stepstring, true),
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
    pub fn LSR(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let mut val = match instr {
            0x4A => self.get_val(&bytes, AddrMode::ACC, stepstring, false),
            0x46 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0x56 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0x4E => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0x5E => self.get_val(&bytes, AddrMode::ABSX, stepstring, false),
            _ => unreachable!("IN LSR BUT GOT BAD OPCOODE"),
        };

        self.cpu.SR.C = (val & 0x1) == 1;
        val >>= 1;

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
    pub fn NOP(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.PC += 1;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn ORA(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let val: u8 = match instr {
            0x09 => self.get_val(&bytes, AddrMode::IMM, stepstring, false),
            0x05 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0x15 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0x0D => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0x1D => self.get_val(&bytes, AddrMode::ABSX, stepstring, true),
            0x19 => self.get_val(&bytes, AddrMode::ABSY, stepstring, true),
            0x01 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0x11 => self.get_val(&bytes, AddrMode::INDY, stepstring, true),
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
    pub fn PHA(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        //push ACC (B =  11)
        //self.write(self.cpu.SP as u16 + 0x100 as u16, &vec![self.cpu.ACC]);
        //self.cpu.SP -= 1;
        self.cpu.push(&mut self.wram, self.cpu.ACC);

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn PHP(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn PLA(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        //self.cpu.SP += 1;
        //self.cpu.ACC = self.read(self.cpu.SP as u16 + 0x100 as u16, 1)[0];

        self.cpu.ACC = self.cpu.pop(&mut self.wram);

        self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
        self.cpu.SR.Z = self.cpu.ACC == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn PLP(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn ROL(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let mut val: u8 = match instr {
            0x2A => self.get_val(&bytes, AddrMode::ACC, stepstring, false),
            0x26 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0x36 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0x2E => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0x3E => self.get_val(&bytes, AddrMode::ABSX, stepstring, false),
            _ => unreachable!("IN ROL BUT GOT BAD OP"),
        };

        //ROL shifts all bits left one position.
        //The Carry is shifted into bit 0 and the original bit 7 is shifted into the Carry.
        let new_c: bool = (val & 0x80) == 0x80;
        val = val.rotate_left(1) & 0xFE;
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
    pub fn ROR(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let mut val: u8 = match instr {
            0x6A => self.get_val(&bytes, AddrMode::ACC, stepstring, false),
            0x66 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0x76 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0x6E => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0x7E => self.get_val(&bytes, AddrMode::ABSX, stepstring, false),
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
    pub fn RTI(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let new_sr = self.cpu.pop(&mut self.wram);
        self.cpu.SR.encode(new_sr);
        //TODO: THIS IS A FUCKING HACK BC I THINK WE SHOULD ONLY BE ABLE TO GET HERE FROM AN IRQ
        self.cpu.SR.BH = true;

        let new_pc_lo = self.cpu.pop(&mut self.wram) as u16;
        let new_pc_hi = self.cpu.pop(&mut self.wram) as u16;
        self.cpu.PC = new_pc_lo | new_pc_hi << 8;

        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn RTS(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
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
    pub fn SBC(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let val: u8 = match instr {
            0xE9 => self.get_val(&bytes, AddrMode::IMM, stepstring, false),
            0xE5 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0xF5 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0xED => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0xFD => self.get_val(&bytes, AddrMode::ABSX, stepstring, true),
            0xF9 => self.get_val(&bytes, AddrMode::ABSY, stepstring, true),
            0xE1 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0xF1 => self.get_val(&bytes, AddrMode::INDY, stepstring, true),
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
        self.cpu.SR.V =
            (((self.cpu.ACC ^ val) & 0x80) == 0x80) && (((self.cpu.ACC ^ sub2.0) & 0x80) == 0x80);

        self.cpu.ACC = sub2.0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn SEC(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.SR.C = true;
        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn SED(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.SR.D = true;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn SEI(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.SR.I = true;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn STA(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        //NOTE: what a stupid fucking log
        //LDA $addr = val is WHAT IS CURRENTLY AT THAT ADDR BEFORE OUR STORE???
        let addr: u16 = match instr {
            0x85 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPG, false);
                self.get_val(&bytes, AddrMode::ZPG, stepstring, false);
                addr
            }
            0x95 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPGX, false);
                self.get_val(&bytes, AddrMode::ZPGX, stepstring, false);
                addr
            }
            0x8D => {
                let addr = self.calc_addr(&bytes, AddrMode::ABS, false);
                self.get_val(&bytes, AddrMode::ABS, stepstring, false);
                addr
            }
            0x9D => {
                let addr = self.calc_addr(&bytes, AddrMode::ABSX, false);
                self.get_val(&bytes, AddrMode::ABSX, stepstring, false);
                addr
            }
            0x99 => {
                let addr = self.calc_addr(&bytes, AddrMode::ABSY, false);
                self.get_val(&bytes, AddrMode::ABSY, stepstring, false);
                addr
            }
            0x81 => {
                let addr = self.calc_addr(&bytes, AddrMode::INDX, false);

                self.get_val(&bytes, AddrMode::INDX, stepstring, false);
                addr
            }
            0x91 => {
                let addr = self.calc_addr(&bytes, AddrMode::INDY, false);
                self.get_val(&bytes, AddrMode::INDY, stepstring, false);
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
    pub fn STX(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        //note: for some fucking reason, the log wants us to say what is at that address
        //BEFORE we write to it
        let addr = if instr == 0x86 {
            let addr = self.calc_addr(&bytes, AddrMode::ZPG, false);
            self.get_val(&bytes, AddrMode::ZPG, stepstring, false);
            addr
        } else if instr == 0x96 {
            let addr = self.calc_addr(&bytes, AddrMode::ZPGY, false);
            self.get_val(&bytes, AddrMode::ZPGY, stepstring, false);

            addr
        } else {
            let addr = self.calc_addr(&bytes, AddrMode::ABS, false);
            self.get_val(&bytes, AddrMode::ABS, stepstring, false);
            addr
        };

        self.write(addr, &[self.cpu.X].to_vec());

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn STY(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        let addr = if instr == 0x84 {
            let addr = self.calc_addr(&bytes, AddrMode::ZPG, false);
            self.get_val(&bytes, AddrMode::ZPG, stepstring, false);
            addr
        } else if instr == 0x94 {
            let addr = self.calc_addr(&bytes, AddrMode::ZPGX, false);
            self.get_val(&bytes, AddrMode::ZPGX, stepstring, false);
            addr
        } else {
            let addr = self.calc_addr(&bytes, AddrMode::ABS, false);
            self.get_val(&bytes, AddrMode::ABS, stepstring, false);

            addr
        };

        self.write(addr, &[self.cpu.Y].to_vec());

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn TAX(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.X = self.cpu.ACC;

        self.cpu.SR.N = (self.cpu.X as i8) < 0;
        self.cpu.SR.Z = self.cpu.X == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn TAY(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.Y = self.cpu.ACC;

        self.cpu.SR.N = (self.cpu.Y as i8) < 0;
        self.cpu.SR.Z = self.cpu.Y == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn TSX(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.X = self.cpu.SP;

        self.cpu.SR.N = (self.cpu.X as i8) < 0;
        self.cpu.SR.Z = self.cpu.X == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn TXA(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.ACC = self.cpu.X;

        self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
        self.cpu.SR.Z = self.cpu.ACC == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn TXS(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        //NO FLAGS SET

        self.cpu.SP = self.cpu.X;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn TYA(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        self.cpu.ACC = self.cpu.Y;

        self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
        self.cpu.SR.Z = self.cpu.ACC == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn INOP(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        //these are literally all nops are you fucking kidding me
        //ARE YOU FUCKING KIDDING ME NESTEST FORMATS THEM ALL DIFFERENTLY??
        stepstring.remove(stepstring.len() - 6);

        //TODO: we have to write to the stepstring manually because reading from memory might access an address we are not allowed to
        match instr {
            //implied
            0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => {}
            //imm
            0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => write!(stepstring, "#${:02X}", bytes[1]).unwrap(),
            //zpg
            0x04 | 0x44 | 0x64 => {
                self.get_val(&bytes, AddrMode::ZPG, stepstring, false);
            }
            //zpgx
            0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 => {
                self.get_val(&bytes, AddrMode::ZPGX, stepstring, false);
            }
            //absolute
            0x0C => {
                self.get_val(&bytes, AddrMode::ABS, stepstring, false);
            }
            //absolute X
            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => {
                self.get_val(&bytes, AddrMode::ABSX, stepstring, true);
            }
            _ => unreachable!("go fuck yourself {instr:02X}"),
        }
        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn ILAX(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        stepstring.remove(stepstring.len() - 6);
        let val = match instr {
            0xA7 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0xB7 => self.get_val(&bytes, AddrMode::ZPGY, stepstring, false),
            0xAF => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0xBF => self.get_val(&bytes, AddrMode::ABSY, stepstring, true),
            0xA3 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0xB3 => self.get_val(&bytes, AddrMode::INDY, stepstring, true),
            _ => unreachable!("IN LAX BUT GOT BAD OP"),
        };

        self.cpu.ACC = val;
        self.cpu.X = val;

        self.cpu.SR.N = (val as i8) < 0;
        self.cpu.SR.Z = val == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn ISAX(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        stepstring.remove(stepstring.len() - 6);
        let addr = match instr {
            0x87 => {
                self.get_val(&bytes, AddrMode::ZPG, stepstring, false);

                self.calc_addr(&bytes, AddrMode::ZPG, false)
            }
            0x97 => {
                self.get_val(&bytes, AddrMode::ZPGY, stepstring, false);

                self.calc_addr(&bytes, AddrMode::ZPGY, false)
            }
            0x8F => {
                self.get_val(&bytes, AddrMode::ABS, stepstring, false);

                self.calc_addr(&bytes, AddrMode::ABS, false)
            }
            0x83 => {
                self.get_val(&bytes, AddrMode::INDX, stepstring, false);

                self.calc_addr(&bytes, AddrMode::INDX, false)
            }
            _ => unreachable!("IN LAX BUT GOT BAD OP"),
        };

        let val = self.cpu.ACC & self.cpu.X;
        self.write(addr, &vec![val]);

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn IUSBC(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        stepstring.remove(stepstring.len() - 6);

        let val = self.get_val(&bytes, AddrMode::IMM, stepstring, false);
        //A - M - C -> A
        //NZCV

        //shoutout kamiyaowl's rust nes emulator for this one
        let sub1 = self.cpu.ACC.overflowing_sub(val);
        let sub2 = sub1.0.overflowing_sub(if self.cpu.SR.C { 0 } else { 1 });

        self.cpu.SR.Z = sub2.0 == 0;
        self.cpu.SR.N = (sub2.0 as i8) < 0;
        self.cpu.SR.C = !(sub1.1 || sub2.1);
        self.cpu.SR.V =
            (((self.cpu.ACC ^ val) & 0x80) == 0x80) && (((self.cpu.ACC ^ sub2.0) & 0x80) == 0x80);

        self.cpu.ACC = sub2.0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn IDCP(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        stepstring.remove(stepstring.len() - 6);
        //DEC oper + CMP oper

        ///////DEC
        let addr = match instr {
            0xC7 => self.calc_addr(&bytes, AddrMode::ZPG, false),
            0xD7 => self.calc_addr(&bytes, AddrMode::ZPGX, false),
            0xCF => self.calc_addr(&bytes, AddrMode::ABS, false),
            0xDF => self.calc_addr(&bytes, AddrMode::ABSX, false),
            0xDB => self.calc_addr(&bytes, AddrMode::ABSY, false),
            0xC3 => self.calc_addr(&bytes, AddrMode::INDX, false),
            0xD3 => self.calc_addr(&bytes, AddrMode::INDY, false),
            _ => unreachable!("IN IDCP_INC, BUT GOT BAD OP"),
        };

        let old_val = self.read(addr, 1)[0];

        let val = old_val.wrapping_sub(1);

        //NZ
        self.cpu.SR.N = (val as i8) < 0;
        self.cpu.SR.Z = val == 0;
        ///////

        ///////CMP
        //this is only for debug prints
        match instr {
            0xC7 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0xD7 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0xCF => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0xDF => self.get_val(&bytes, AddrMode::ABSX, stepstring, false),
            0xDB => self.get_val(&bytes, AddrMode::ABSY, stepstring, false),
            0xC3 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0xD3 => self.get_val(&bytes, AddrMode::INDY, stepstring, false),
            _ => unimplemented!("IN IDCP_CMP BUT GOT ILLEGAL OPCODE"),
        };

        //NOTE: WE ONLY WAIT TILL NOW TO WRITE IT SO THE PRINT LOOKS RIGHT
        self.write(addr, &vec![val]);

        //ACC - M (DO NOT SAVE)
        let res = self.cpu.ACC.wrapping_sub(val);
        //just affects NZC flags
        self.cpu.SR.N = (res as i8) < 0;
        self.cpu.SR.Z = res == 0;

        //TODO: shouldnt this be the other way around????
        self.cpu.SR.C = self.cpu.ACC >= val;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn IISB(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        stepstring.remove(stepstring.len() - 6);

        //INC oper + SBC oper
        let addr = match instr {
            0xE7 => self.calc_addr(&bytes, AddrMode::ZPG, false),
            0xF7 => self.calc_addr(&bytes, AddrMode::ZPGX, false),
            0xEF => self.calc_addr(&bytes, AddrMode::ABS, false),
            0xFF => self.calc_addr(&bytes, AddrMode::ABSX, false),
            0xFB => self.calc_addr(&bytes, AddrMode::ABSY, false),
            0xE3 => self.calc_addr(&bytes, AddrMode::INDX, false),
            0xF3 => self.calc_addr(&bytes, AddrMode::INDY, false),

            _ => unreachable!("IN IISB_INC, BUT GOT BAD OP"),
        };

        let mut val = self.read(addr, 1)[0];

        val = val.wrapping_add(1);

        //NZ
        self.cpu.SR.N = (val as i8) < 0;
        self.cpu.SR.Z = val == 0;

        /////////
        match instr {
            0xE7 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0xF7 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0xEF => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0xFF => self.get_val(&bytes, AddrMode::ABSX, stepstring, false),
            0xFB => self.get_val(&bytes, AddrMode::ABSY, stepstring, false),
            0xE3 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0xF3 => self.get_val(&bytes, AddrMode::INDY, stepstring, false),
            _ => panic!("IN IISB_SBC BUT GOT BAD OPCODE"),
        };

        self.write(addr, &vec![val]);

        //A - M - C -> A
        //NZCV

        //shoutout kamiyaowl's rust nes emulator for this one
        let sub1 = self.cpu.ACC.overflowing_sub(val);
        let sub2 = sub1.0.overflowing_sub(if self.cpu.SR.C { 0 } else { 1 });

        self.cpu.SR.Z = sub2.0 == 0;
        self.cpu.SR.N = (sub2.0 as i8) < 0;
        self.cpu.SR.C = !(sub1.1 || sub2.1);
        self.cpu.SR.V =
            (((self.cpu.ACC ^ val) & 0x80) == 0x80) && (((self.cpu.ACC ^ sub2.0) & 0x80) == 0x80);

        self.cpu.ACC = sub2.0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn ISLO(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        stepstring.remove(stepstring.len() - 6);

        //ASL oper + ORA oper

        //ASL
        let mut val = match instr {
            0x07 => self.get_val_silent(&bytes, AddrMode::ZPG, false),
            0x17 => self.get_val_silent(&bytes, AddrMode::ZPGX, false),
            0x0F => self.get_val_silent(&bytes, AddrMode::ABS, false),
            0x1F => self.get_val_silent(&bytes, AddrMode::ABSX, false),
            0x1B => self.get_val_silent(&bytes, AddrMode::ABSY, false),
            0x03 => self.get_val_silent(&bytes, AddrMode::INDX, false),
            0x13 => self.get_val_silent(&bytes, AddrMode::INDY, false),
            _ => unreachable!("IN ISLO_ASL BUT GOT BAD OPCOODE"),
        };

        self.cpu.SR.C = (val & 0x80) >> 7 == 1;
        val = (val << 1) & 0b1111_1110;

        self.cpu.SR.N = (val as i8) < 0;
        self.cpu.SR.Z = val == 0;

        //ORA
        match instr {
            0x07 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0x17 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0x0F => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0x1F => self.get_val(&bytes, AddrMode::ABSX, stepstring, false),
            0x1B => self.get_val(&bytes, AddrMode::ABSY, stepstring, false),
            0x03 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0x13 => self.get_val(&bytes, AddrMode::INDY, stepstring, false),
            _ => {
                panic!("IN OR, BUT GOT INVALID OPCODE")
            }
        };

        match instr {
            0x07 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPG, false);
                self.write(addr, &vec![val])
            }
            0x17 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPGX, false);
                self.write(addr, &vec![val])
            }
            0x0F => {
                let addr = self.calc_addr(&bytes, AddrMode::ABS, false);
                self.write(addr, &vec![val])
            }
            0x1F => {
                let addr = self.calc_addr(&bytes, AddrMode::ABSX, false);
                self.write(addr, &vec![val])
            }
            0x1B => {
                let addr = self.calc_addr(&bytes, AddrMode::ABSY, false);
                self.write(addr, &vec![val])
            }
            0x03 => {
                let addr = self.calc_addr(&bytes, AddrMode::INDX, false);
                self.write(addr, &vec![val])
            }
            0x13 => {
                let addr = self.calc_addr(&bytes, AddrMode::INDY, false);
                self.write(addr, &vec![val])
            }
            _ => unreachable!("IN LSR BUT GOT BAD OPCOODE"),
        };

        self.cpu.ACC |= val;

        self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
        self.cpu.SR.Z = self.cpu.ACC == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn IRLA(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        stepstring.remove(stepstring.len() - 6);

        //ROL oper + AND oper
        let mut val: u8 = match instr {
            0x27 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0x37 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0x2F => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0x3F => self.get_val(&bytes, AddrMode::ABSX, stepstring, false),
            0x3B => self.get_val(&bytes, AddrMode::ABSY, stepstring, false),
            0x23 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0x33 => self.get_val(&bytes, AddrMode::INDY, stepstring, false),
            _ => unreachable!("IN IRLA_ROL BUT GOT BAD OP"),
        };

        //ROL shifts all bits left one position.
        //The Carry is shifted into bit 0 and the original bit 7 is shifted into the Carry.
        let new_c: bool = (val & 0x80) == 0x80;
        let new_l = if self.cpu.SR.C { 1 } else { 0 };
        val = val.rotate_left(1) & 0xFE;
        val |= new_l;

        self.cpu.SR.N = (val as i8) < 0;
        self.cpu.SR.Z = val == 0;
        self.cpu.SR.C = new_c;

        match instr {
            0x27 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPG, false);
                self.write(addr, &vec![val])
            }
            0x37 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPGX, false);
                self.write(addr, &vec![val])
            }
            0x2F => {
                let addr = self.calc_addr(&bytes, AddrMode::ABS, false);
                self.write(addr, &vec![val])
            }
            0x3F => {
                let addr = self.calc_addr(&bytes, AddrMode::ABSX, false);
                self.write(addr, &vec![val])
            }
            0x3B => {
                let addr = self.calc_addr(&bytes, AddrMode::ABSY, false);
                self.write(addr, &vec![val])
            }
            0x23 => {
                let addr = self.calc_addr(&bytes, AddrMode::INDX, false);
                self.write(addr, &vec![val])
            }
            0x33 => {
                let addr = self.calc_addr(&bytes, AddrMode::INDY, false);
                self.write(addr, &vec![val])
            }
            _ => unreachable!("IN IRLA_ROL BUT GOT BAD OPCOODE"),
        };

        self.cpu.ACC &= val;

        self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
        self.cpu.SR.Z = self.cpu.ACC == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn ISRE(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        stepstring.remove(stepstring.len() - 6);

        //LSR oper + EOR oper
        let mut val = match instr {
            0x47 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0x57 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0x4F => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0x5F => self.get_val(&bytes, AddrMode::ABSX, stepstring, false),
            0x5B => self.get_val(&bytes, AddrMode::ABSY, stepstring, false),
            0x43 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0x53 => self.get_val(&bytes, AddrMode::INDY, stepstring, false),
            _ => unreachable!("IN ISRE_LSR BUT GOT BAD OPCOODE"),
        };

        self.cpu.SR.C = (val & 0x1) == 1;
        val >>= 1;

        self.cpu.SR.N = false;
        self.cpu.SR.Z = val == 0;

        match instr {
            0x47 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPG, false);
                self.write(addr, &vec![val])
            }
            0x57 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPGX, false);
                self.write(addr, &vec![val])
            }
            0x4F => {
                let addr = self.calc_addr(&bytes, AddrMode::ABS, false);
                self.write(addr, &vec![val])
            }
            0x5F => {
                let addr = self.calc_addr(&bytes, AddrMode::ABSX, false);
                self.write(addr, &vec![val])
            }
            0x5B => {
                let addr = self.calc_addr(&bytes, AddrMode::ABSY, false);
                self.write(addr, &vec![val])
            }
            0x43 => {
                let addr = self.calc_addr(&bytes, AddrMode::INDX, false);
                self.write(addr, &vec![val])
            }
            0x53 => {
                let addr = self.calc_addr(&bytes, AddrMode::INDY, false);
                self.write(addr, &vec![val])
            }
            _ => unreachable!("IN ISRE_LSR BUT GOT BAD OPCOODE"),
        };

        //EOR
        self.cpu.ACC ^= val;

        self.cpu.SR.N = (self.cpu.ACC as i8) < 0;
        self.cpu.SR.Z = self.cpu.ACC == 0;

        self.cpu.PC += self.instr_data.instrs[&instr].len as u16;
        self.cycles += self.instr_data.instrs[&instr].cycles as u128;
    }
    pub fn IRRA(&mut self, instr: u8, bytes: Vec<u8>, stepstring: &mut String) {
        stepstring.remove(stepstring.len() - 6);

        //ROR oper + ADC oper
        let mut val: u8 = match instr {
            0x67 => self.get_val(&bytes, AddrMode::ZPG, stepstring, false),
            0x77 => self.get_val(&bytes, AddrMode::ZPGX, stepstring, false),
            0x6F => self.get_val(&bytes, AddrMode::ABS, stepstring, false),
            0x7F => self.get_val(&bytes, AddrMode::ABSX, stepstring, false),
            0x7B => self.get_val(&bytes, AddrMode::ABSY, stepstring, false),
            0x63 => self.get_val(&bytes, AddrMode::INDX, stepstring, false),
            0x73 => self.get_val(&bytes, AddrMode::INDY, stepstring, false),

            _ => unreachable!("IN IRRA_ROR BUT GOT BAD OP"),
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
            0x67 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPG, false);
                self.write(addr, &vec![val])
            }
            0x77 => {
                let addr = self.calc_addr(&bytes, AddrMode::ZPGX, false);
                self.write(addr, &vec![val])
            }
            0x6F => {
                let addr = self.calc_addr(&bytes, AddrMode::ABS, false);
                self.write(addr, &vec![val])
            }
            0x7F => {
                let addr = self.calc_addr(&bytes, AddrMode::ABSX, false);
                self.write(addr, &vec![val])
            }
            0x7B => {
                let addr = self.calc_addr(&bytes, AddrMode::ABSY, false);
                self.write(addr, &vec![val])
            }
            0x63 => {
                let addr = self.calc_addr(&bytes, AddrMode::INDX, false);
                self.write(addr, &vec![val])
            }
            0x73 => {
                let addr = self.calc_addr(&bytes, AddrMode::INDY, false);
                self.write(addr, &vec![val])
            }
            _ => unreachable!("IN IRRA_ROR BUT GOT BAD OPCOODE"),
        };

        //ADC

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
}
