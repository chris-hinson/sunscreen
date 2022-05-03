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
        ]);

        Instr { instrs: map }
    }
}
