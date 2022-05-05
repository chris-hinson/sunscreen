use std::collections::HashMap;

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
        return InstrData {
            name: name.to_string(),
            len,
            cycles,
        };
    }
}

pub struct Instr {
    //hashmap of opcode to instr data
    //TODO: could instead use a vec ordered by opcode for constant lookup
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
        ]);

        Instr { instrs: map }
    }
}
