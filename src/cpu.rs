pub struct CPU {
    PC: u16,
    ACC: u8,
    X: u8,
    Y: u8,
    SR: SR,
}

//status register struct
struct SR {
    N: bool,
    V: bool,
    NA: bool,
    B: bool,
    D: bool,
    I: bool,
    Z: bool,
    C: bool,
}

//functions to encode and decode the SR
impl SR {
    //turn the struct into a u8
    fn encode(&self) -> u8 {
        let mut ret_field: u8 = 0b00000000;
        if self.N {
            ret_field |= 0x1 << 7;
        } else {
            ret_field |= 0x0 << 7;
        }
        if self.V {
            ret_field |= 0x1 << 6;
        } else {
            ret_field |= 0x0 << 6;
        }
        //dont touch the NA reg
        if self.B {
            ret_field |= 0x1 << 4;
        } else {
            ret_field |= 0x0 << 4;
        }
        if self.D {
            ret_field |= 0x1 << 3;
        } else {
            ret_field |= 0x0 << 3;
        }
        if self.I {
            ret_field |= 0x1 << 2;
        } else {
            ret_field |= 0x0 << 2;
        }
        if self.Z {
            ret_field |= 0x1 << 1;
        } else {
            ret_field |= 0x0 << 1;
        }
        if self.C {
            ret_field |= 0x1;
        } else {
            ret_field |= 0x0;
        }

        ret_field
    }
    //turn a u8 into the SR
    fn decode(&mut self, val: u8) {
        self.N = if (val & 0b01000000) >> 7 == 0x1 {
            true
        } else {
            false
        };
        self.V = if (val & 0b00100000) >> 6 == 0x1 {
            true
        } else {
            false
        };
        //dont touch the NA
        self.B = if (val & 0b00010000) >> 7 == 0x1 {
            true
        } else {
            false
        };
    }
}
