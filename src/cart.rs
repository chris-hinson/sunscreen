pub struct Cart {
    prg_rom: Vec<u8>,
}

impl Cart {
    pub fn new(bank0: Vec<u8>) -> Self {
        return Cart { prg_rom: bank0 };
    }

    fn read(&mut self, addr: u16) -> u8 {
        return self.prg_rom[addr as usize];
    }

    fn write(&mut self, addr: u16, byte: u8) {
        self.prg_rom[addr as usize] = byte;
    }
}
