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
        let bytes = self.cpu.read(
            self.cpu.PC,
            &mut self.cart,
            self.instr_data.instrs[&instr].len,
        );
        let bytes_string = print_bytes(&bytes);
        let padding: String = vec![" "; 16 - bytes_string.len()].join("");
        //println!("fetching {:04X}, found {instr:02X}", self.cpu.PC);

        print!("{:04X} {bytes_string}{padding}", self.cpu.PC);

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
            }
            /*
            JSR jump subroutine
            LDA load accumulator
            LDX load X
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
