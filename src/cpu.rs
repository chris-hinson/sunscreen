use std::fmt;

#[allow(non_snake_case)]
#[derive(Debug, Clone)]

pub struct Cpu {
    pub PC: u16,
    pub ACC: u8,
    pub X: u8,
    pub Y: u8,
    pub SR: SR,
    pub SP: u8,
}

//this is how we print our cpu status for comparing against nestest
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
#[derive(Debug, Clone, Copy)]
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
            //WRAM: [0; 2048],
            //mem_channel,
        };

        our_cpu.SR.encode(0b0010_0100);
        return our_cpu;
    }

    //formatter function for our tui.
    //NOTE: we need this to return a vec of strings rather than one long one with newlines because
    //newlines break our tui.  :(
    pub fn fmt_for_tui(&self) -> Vec<String> {
        /*format!(
            "PC:  {:04X}\nACC: {:04X}\nX:   {:02X}\nY:   {:02X}\nSP:  {:02X}\nSR:  {}\n     N:{} V:{} BH:{} BL:{} D:{} I:{} Z:{} C:{}",
            self.PC,
            self.ACC,
            self.X,
            self.Y,
            self.SP,
            self.SR.decode(),
            self.SR.N as i32,
            self.SR.V as i32,
            self.SR.BH as i32,
            self.SR.BL as i32,
            self.SR.D as i32,
            self.SR.I as i32,
            self.SR.Z as i32,
            self.SR.C as i32,
        )*/
        let mut ret_vec: Vec<String> = Vec::new();
        ret_vec.push(format!("PC:  {:04X}", self.PC));
        ret_vec.push(format!("ACC: {:02X}", self.ACC));
        ret_vec.push(format!("X:   {:02X}", self.X));
        ret_vec.push(format!("Y:   {:02X}", self.Y));
        ret_vec.push(format!("SP:  {:02X}", self.SP));
        ret_vec.push(format!("SR:  {:02X}", self.SR.decode()));
        ret_vec.push(format!(
            "     N:{} V:{} BH:{} BL:{} D:{} I:{} Z:{} C:{}",
            self.SR.N as i32,
            self.SR.V as i32,
            self.SR.BH as i32,
            self.SR.BL as i32,
            self.SR.D as i32,
            self.SR.I as i32,
            self.SR.Z as i32,
            self.SR.C as i32,
        ));

        return ret_vec;
    }
}
