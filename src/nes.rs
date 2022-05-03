use crate::cart::Cart;
use crate::cpu::Cpu;
use crate::instr::Instr;

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

    pub fn step(&mut self) {
        //print exactly 16 characters composed of:
        // our current addr,
        // the bytes that make up this instr,
        // padding out to 16 chars
        let instr: u8 = self.cpu.read(self.cpu.PC, &mut self.cart, 1)[0];

        //println!("fetching {:04X}, found {instr:02X}", self.cpu.PC);

        let bytes = self.cpu.read(
            self.cpu.PC,
            &mut self.cart,
            self.instr_data.instrs[&instr].len,
        );
        let bytes_string = print_bytes(&bytes);
        let padding: String = vec![" "; 16 - (bytes_string.len() + 6)].join("");

        print!("{:04X}  {bytes_string}{padding}", self.cpu.PC);

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
                unimplemented!("unimplemented op ADC")
            }

            /*
            AND and (with accumulator)
            ASL arithmetic shift left
            BCC branch on carry clear
            BCS branch on carry set
            BEQ branch on equal (zero set)
            BIT bit test
            BMI branch on minus (negative set)
            BNE branch on not equal (zero clear)
            BPL branch on plus (negative clear)
            BRK break / interrupt
            BVC branch on overflow clear
            BVS branch on overflow set
            CLC clear carry
            CLD clear decimal
            CLI clear interrupt disable
            CLV clear overflow
            CMP compare (with accumulator)
            CPX compare with X
            CPY compare with Y
            DEC decrement
            DEX decrement X
            DEY decrement Y
            EOR exclusive or (with accumulator)
            INC increment
            INX increment X
            INY increment Y
            JMP jump
                absolute	JMP oper	4C	3	3
                indirect	JMP (oper)	6C	3	5*/
            0x4C | 0x6C => {
                //print opcode name
                print!("{} ", self.instr_data.instrs[&instr].name);
                let target: u16 = if instr == 0x4C {
                    let imm = bytes[1] as u16 | (bytes[2] as u16) << 8;
                    print!("${imm:04X}                       ");
                    imm
                } else {
                    let addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                    let bytes = self.cpu.read(addr, &mut self.cart, 2);
                    let val = bytes[1] as u16 | (bytes[2] as u16) << 8;
                    //JMP ($0200) = DB7E
                    print!("(${addr:04X}) = {val:04X}              ");
                    val
                };

                self.cpu.PC = target;
                self.cycles += self.instr_data.instrs[&instr].cycles as u128;
                println!("{} CYC:{}", self.cpu, self.cycles);
            }
            /*
            JSR jump subroutine
            LDA load accumulator
            LDX load X
                immediate	LDX #oper	A2	2	2
                zeropage	LDX oper	A6	2	3
                zeropage,Y	LDX oper,Y	B6	2	4
                absolute	LDX oper	AE	3	4
                absolute,Y	LDX oper,Y	BE	3	4* */
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                print!("{} ", self.instr_data.instrs[&instr].name);
                let val = match instr {
                    0xA2 => {
                        print!("#${:02X}                        ", bytes[1]);
                        bytes[1]
                    }
                    0xA6 => {
                        let addr = 0 as u16 | bytes[1] as u16;
                        self.cpu.read(addr, &mut self.cart, 1)[0]
                    }
                    0xB6 => {
                        let mut addr = 0 as u16 | bytes[1] as u16;
                        addr += self.cpu.Y as u16;
                        self.cpu.read(addr, &mut self.cart, 1)[0]
                    }
                    0xAE => {
                        let addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                        self.cpu.read(addr, &mut self.cart, 1)[0]
                    }
                    0xBE => {
                        let mut addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                        let init_page = addr % 256;
                        addr += self.cpu.Y as u16;
                        let final_page = addr % 256;
                        //if we cross a page boundary with indexing, add a cycle
                        if init_page != final_page {
                            self.cycles += 1;
                        }
                        self.cpu.read(addr, &mut self.cart, 1)[0]
                    }
                    _ => 0,
                };

                //actual loading
                self.cpu.X = val;

                //set flags
                self.cpu.SR.N = (val as i8) < 0;
                self.cpu.SR.Z = val == 0;

                //update PC
                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                self.cycles += self.instr_data.instrs[&instr].cycles as u128;

                //print cpu state after executing this operation
                println!("{} CYC:{}", self.cpu, self.cycles);
            }

            /*
            LDY load Y
            LSR logical shift right
            NOP no operation
            ORA or with accumulator
            PHA push accumulator
            PHP push processor status (SR)
            PLA pull accumulator
            PLP pull processor status (SR)
            ROL rotate left
            ROR rotate right
            RTI return from interrupt
            RTS return from subroutine
            SBC subtract with carry
            SEC set carry
            SED set decimal
            SEI set interrupt disable
            STA store accumulator
            STX store X
                zeropage	STX oper	86	2	3
                zeropage,Y	STX oper,Y	96	2	4
                absolute	STX oper	8E	3	4  */
            0x86 | 0x96 | 0x8E => {
                print!("{} ", self.instr_data.instrs[&instr].name);
                let addr = if instr == 0x86 {
                    let addr = 0 as u16 | bytes[1] as u16;
                    print!("${:02X} = {:02X}                    ", addr, self.cpu.X);
                    addr
                } else if instr == 0x96 {
                    let mut addr = 0 as u16 | bytes[1] as u16;
                    addr += self.cpu.Y as u16;
                    //STX $80,Y @ 7F = 00
                    //print!("$")
                    addr
                } else {
                    let addr = bytes[1] as u16 | (bytes[2] as u16) << 8;
                    addr
                };

                self.cpu.write(addr, &[self.cpu.X].to_vec());

                self.cpu.PC += self.instr_data.instrs[&instr].len as u16;

                self.cycles += self.instr_data.instrs[&instr].cycles as u128;

                println!("{} CYC:{}", self.cpu, self.cycles);
            }

            /*
            STY store Y
            TAX transfer accumulator to X
            TAY transfer accumulator to Y
            TSX transfer stack pointer to X
            TXA transfer X to accumulator
            TXS transfer X to stack pointer
            TYA transfer Y to accumulator
            */
            _ => {
                panic!("unimplemented op {:#02x}", instr)
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
