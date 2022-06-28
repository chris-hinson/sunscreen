use crate::cart::Cart;
use crate::cpu::Cpu;
use crate::instr::Instr;
use crate::wram::Wram;

use std::fmt::Write;

//TODO: remove this allow once we finish implementing all addressing modes
#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
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
#[allow(clippy::upper_case_acronyms)]
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
#[allow(clippy::upper_case_acronyms)]
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

    pub fn calc_addr(&mut self, bytes: &[u8], mode: AddrMode, penalty: bool) -> u16 {
        return match mode {
            AddrMode::ABS => (bytes[2] as u16) << 8 | bytes[1] as u16,
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
            AddrMode::ZPG => bytes[1] as u16,
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
        bytes: &[u8],
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

    pub fn get_val_silent(&mut self, bytes: &[u8], mode: AddrMode, penalty: bool) -> u8 {
        //NOTE: this is split out as a match case because we need to print different stuff based on
        // addr mode, otherwise we could just always calc addr and read a byte
        match mode {
            AddrMode::ACC => self.cpu.ACC,
            AddrMode::ABS => {
                let addr = self.calc_addr(bytes, AddrMode::ABS, penalty);
                self.read(addr, 1)[0]
            }
            AddrMode::ABSX => {
                let addr = self.calc_addr(bytes, AddrMode::ABSX, penalty);
                self.read(addr, 1)[0]
            }
            AddrMode::ABSY => {
                let addr = self.calc_addr(bytes, AddrMode::ABSY, penalty);
                self.read(addr, 1)[0]
            }
            AddrMode::IMM => bytes[1],
            AddrMode::IND => {
                let addr = self.calc_addr(bytes, AddrMode::IND, penalty);
                self.read(addr, 1)[0]
            }
            AddrMode::INDX => {
                //this is our effective(final) address
                let addr = self.calc_addr(bytes, AddrMode::INDX, penalty);
                //we read from this address to get our value
                self.read(addr, 1)[0]
            }
            AddrMode::INDY => {
                let addr = self.calc_addr(bytes, AddrMode::INDY, penalty);
                self.read(addr, 1)[0]
            }
            AddrMode::REL => {
                let addr = self.calc_addr(bytes, AddrMode::REL, penalty);
                self.read(addr, 1)[0]
            }
            AddrMode::ZPG => {
                let addr = self.calc_addr(bytes, AddrMode::ZPG, penalty);
                self.read(addr, 1)[0]
            }
            AddrMode::ZPGX => {
                let addr = self.calc_addr(bytes, AddrMode::ZPGX, penalty);
                self.read(addr, 1)[0]
            }
            AddrMode::ZPGY => {
                let addr = self.calc_addr(bytes, AddrMode::ZPGY, penalty);
                self.read(addr, 1)[0]
            }
        }
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
                self.ADC(instr, bytes, &mut stepstring);
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
                self.AND(instr, bytes, &mut stepstring)
            }

            /*
            ASL arithmetic shift left
                accumulator	ASL A	0A	1	2
                zeropage	ASL oper	06	2	5
                zeropage,X	ASL oper,X	16	2	6
                absolute	ASL oper	0E	3	6
                absolute,X	ASL oper,X	1E	3	7  */
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => self.ASL(instr, bytes, &mut stepstring),

            /*
            BCC branch on carry clear
                relative	BCC oper	90	2	2** */
            0x90 => self.BCC(instr, bytes, &mut stepstring),
            /*
            BCS branch on carry set
                relative	BCS oper	B0	2	2** */
            0xB0 => self.BCS(instr, bytes, &mut stepstring),

            /*
            BEQ branch on equal (zero set)
                relative	BEQ oper	F0	2	2** */
            0xF0 => self.BEQ(instr, bytes, &mut stepstring),

            /*
            BIT bit test
                zeropage	BIT oper	24	2	3
                absolute	BIT oper	2C	3	4  */
            0x24 | 0x2C => self.BIT(instr, bytes, &mut stepstring),
            /*
            BMI branch on minus (negative set)
                relative	BMI oper	30	2	2** */
            0x30 => self.BMI(instr, bytes, &mut stepstring),
            /*
            BNE branch on not equal (zero clear)
                relative	BNE oper	D0	2	2**  */
            0xD0 => self.BNE(instr, bytes, &mut stepstring),
            /*
            BPL branch on plus (negative clear)
                relative	BPL oper	10	2	2** */
            0x10 => self.BPL(instr, bytes, &mut stepstring),

            /*
            BRK break / interrupt
                implied	BRK	00	1	7  */
            //TODO: THIS IS UNTESTED LUL
            0x00 => self.BRK(instr, bytes, &mut stepstring),

            /*
            BVC branch on overflow clear
                relative	BVC oper	50	2	2** */
            0x50 => self.BVC(instr, bytes, &mut stepstring),

            /*
            BVS branch on overflow set
                relative	BVS oper	70	2	2** */
            0x70 => self.BVS(instr, bytes, &mut stepstring),
            /*
            CLC clear carry
                implied	CLC	18	1	2 */
            0x18 => self.CLC(instr, bytes, &mut stepstring),
            /*
            CLD clear decimal
                implied	CLD	D8	1	 2*/
            0xD8 => self.CLD(instr, bytes, &mut stepstring),
            /*
            CLI clear interrupt disable
                implied	CLI	58	1	*/
            0x58 => self.CLI(instr, bytes, &mut stepstring),
            /*
            CLV clear overflow
                implied	CLV	B8	1	2  */
            0xB8 => self.CLV(instr, bytes, &mut stepstring),
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
                self.CMP(instr, bytes, &mut stepstring)
            }

            /*
            CPX compare with X
                immediate	CPX #oper	E0	2	2
                zeropage	CPX oper	E4	2	3
                absolute	CPX oper	EC	3	4  */
            0xE0 | 0xE4 | 0xEC => self.CPX(instr, bytes, &mut stepstring),

            /*
            CPY compare with Y
                immediate	CPY #oper	C0	2	2
                zeropage	CPY oper	C4	2	3
                absolute	CPY oper	CC	3	4  */
            0xC0 | 0xC4 | 0xCC => self.CPY(instr, bytes, &mut stepstring),

            /*
            DEC decrement
                zeropage	DEC oper	C6	2	5
                zeropage,X	DEC oper,X	D6	2	6
                absolute	DEC oper	CE	3	6
                absolute,X	DEC oper,X	DE	3	7
            */
            0xC6 | 0xD6 | 0xCE | 0xDE => self.DEC(instr, bytes, &mut stepstring),
            /*
            DEX decrement X
                implied	DEX	CA	1	2   */
            0xCA => self.DEX(instr, bytes, &mut stepstring),
            /*
            DEY decrement Y
                implied	DEY	88	1	2   */
            0x88 => self.DEY(instr, bytes, &mut stepstring),

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
                self.EOR(instr, bytes, &mut stepstring)
            }

            /*
            INC increment
                zeropage	INC oper	E6	2	5
                zeropage,X	INC oper,X	F6	2	6
                absolute	INC oper	EE	3	6
                absolute,X	INC oper,X	FE	3	7  */
            0xE6 | 0xF6 | 0xEE | 0xFE => self.INC(instr, bytes, &mut stepstring),

            /*
            INX increment X
                X + 1 -> X */
            0xE8 => self.INX(instr, bytes, &mut stepstring),
            /*
            INY increment Y
                Y + 1 -> Y */
            0xC8 => self.INY(instr, bytes, &mut stepstring),
            /*
            JMP jump
                absolute	JMP oper	4C	3	3
                indirect	JMP (oper)	6C	3	5*/
            0x4C | 0x6C => self.JMP(instr, bytes, &mut stepstring),
            /*
            JSR jump subroutine
                absolute	JSR oper	20	3	6  */
            0x20 => self.JSR(instr, bytes, &mut stepstring),

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
                self.LDA(instr, bytes, &mut stepstring)
            }

            /*
            LDX load X
                immediate	LDX #oper	A2	2	2
                zeropage	LDX oper	A6	2	3
                zeropage,Y	LDX oper,Y	B6	2	4
                absolute	LDX oper	AE	3	4
                absolute,Y	LDX oper,Y	BE	3	4* */
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.LDX(instr, bytes, &mut stepstring),

            /*
            LDY load Y
                immediate	LDY #oper	A0	2	2
                zeropage	LDY oper	A4	2	3
                zeropage,X	LDY oper,X	B4	2	4
                absolute	LDY oper	AC	3	4
                absolute,X	LDY oper,X	BC	3	4*  */
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.LDY(instr, bytes, &mut stepstring),

            /*
            LSR logical shift right
                accumulator	LSR A	4A	1	2
                zeropage	LSR oper	46	2	5
                zeropage,X	LSR oper,X	56	2	6
                absolute	LSR oper	4E	3	6
                absolute,X	LSR oper,X	5E	3	7 */
            0x4A | 0x46 | 0x56 | 0x4E | 0x5E => self.LSR(instr, bytes, &mut stepstring),

            /*
            NOP no operation
                implied	NOP	EA	1	2 */
            0xEA => self.NOP(instr, bytes, &mut stepstring),
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
                self.ORA(instr, bytes, &mut stepstring)
            }
            ,/*
            PHA push accumulator
            implied	PHA	48	1	3 */
            0x48 => self.PHA(instr, bytes, &mut stepstring),

            /*
            PHP push processor status (SR)
                implied	PHP	08	1	3  */
            0x08 => self.PHP(instr, bytes, &mut stepstring),

            /*
            PLA pull accumulator
                implied	PLA	68	1	4  */
            0x68 => self.PLA(instr, bytes, &mut stepstring),

            /*
            PLP pull processor status (SR)
                implied	PLP	28	1	4 */
            0x28 => self.PLP(instr, bytes, &mut stepstring),

            /*
            ROL rotate left
                accumulator	ROL A	2A	1	2
                zeropage	ROL oper	26	2	5
                zeropage,X	ROL oper,X	36	2	6
                absolute	ROL oper	2E	3	6
                absolute,X	ROL oper,X	3E	3	7  */
            0x2A | 0x26 | 0x36 | 0x2E | 0x3E => self.ROL(instr, bytes, &mut stepstring),

            /*
            ROR rotate right
                accumulator	ROR A	6A	1	2
                zeropage	ROR oper	66	2	5
                zeropage,X	ROR oper,X	76	2	6
                absolute	ROR oper	6E	3	6
                absolute,X	ROR oper,X	7E	3	7 */
            0x6A | 0x66 | 0x76 | 0x6E | 0x7E => self.ROR(instr, bytes, &mut stepstring),

            /*
            RTI return from interrupt
                implied	RTI	40	1	6  */
            0x40 => self.RTI(instr, bytes, &mut stepstring),

            /*/
            RTS return from subroutine
                implied	RTS	60	1	6  */
            0x60 => self.RTS(instr, bytes, &mut stepstring),
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
                self.SBC(instr, bytes, &mut stepstring)
            }

            /*
            SEC set carry
                implied	SEC	38	1	2 */
            0x38 => self.SEC(instr, bytes, &mut stepstring),
            /*
            SED set decimal
                implied	SED	F8	1	2  */
            0xF8 => self.SED(instr, bytes, &mut stepstring),
            /*
            SEI set interrupt disable
                implied	SEI	78	1	2  */
            0x78 => self.SEI(instr, bytes, &mut stepstring),

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
                self.STA(instr, bytes, &mut stepstring)
            }

            /*
            STX store X
                zeropage	STX oper	86	2	3
                zeropage,Y	STX oper,Y	96	2	4
                absolute	STX oper	8E	3	4  */
            0x86 | 0x96 | 0x8E => self.STX(instr, bytes, &mut stepstring),

            /*
            STY store Y
                zeropage	STY oper	84	2	3
                zeropage,X	STY oper,X	94	2	4
                absolute	STY oper	8C	3	4  */
            0x84 | 0x94 | 0x8C => self.STY(instr, bytes, &mut stepstring),

            /*
            TAX transfer accumulator to X
                implied	TAX	AA	1	2  */
            0xAA => self.TAX(instr, bytes, &mut stepstring),
            /*
            TAY transfer accumulator to Y
                implied	TAY	A8	1	2  */
            0xA8 => self.TAY(instr, bytes, &mut stepstring),
            /*
            TSX transfer stack pointer to X
                implied	TSX	BA	1	2   */
            0xBA => self.TSX(instr, bytes, &mut stepstring),
            /*
            TXA transfer X to accumulator
                implied	TXA	8A	1	2  */
            0x8A => self.TXA(instr, bytes, &mut stepstring),
            /*
            TXS transfer X to stack pointer
                implied	TXS	9A	1	2 */
            0x9A => self.TXS(instr, bytes, &mut stepstring),
            /*
            TYA transfer Y to accumulator
                implied	TYA	98	1	2  */
            0x98 => self.TYA(instr, bytes, &mut stepstring),

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
            | 0x7C | 0xDC | 0xFC => self.INOP(instr, bytes, &mut stepstring),

            /*
            LAX
                zeropage	LAX oper	A7	2	3
                zeropage,Y	LAX oper,Y	B7	2	4
                absolute	LAX oper	AF	3	4
                absolut,Y	LAX oper,Y	BF	3	4*
                (indirect,X)	LAX (oper,X)	A3	2	6
                (indirect),Y	LAX (oper),Y	B3	2	5*
            */
            0xA7 | 0xB7 | 0xAF | 0xBF | 0xA3 | 0xB3 => self.ILAX(instr, bytes, &mut stepstring),
            /*
            SAX
                zeropage	SAX oper	87	2	3
                zeropage,Y	SAX oper,Y	97	2	4
                absolute	SAX oper	8F	3	4
                (indirect,X)	SAX (oper,X)	83	2	6
            */
            0x87 | 0x97 | 0x8F | 0x83 => self.ISAX(instr, bytes, &mut stepstring),
            /*
            USBC
                immediate	USBC #oper	EB	2	2
            */
            0xEB => self.IUSBC(instr, bytes, &mut stepstring),
            /*DCP
                zeropage	DCP oper	C7	2	5
                zeropage,X	DCP oper,X	D7	2	6
                absolute	DCP oper	CF	3	6
                absolut,X	DCP oper,X	DF	3	7
                absolut,Y	DCP oper,Y	DB	3	7
                (indirect,X)	DCP (oper,X)	C3	2	8
                (indirect),Y	DCP (oper),Y	D3	2	8
            */
            0xC7 | 0xD7 | 0xCF | 0xDF | 0xDB | 0xC3 | 0xD3 => {
                self.IDCP(instr, bytes, &mut stepstring)
            }
            /*
            ISB
                zeropage	ISC oper	E7	2	5
                zeropage,X	ISC oper,X	F7	2	6
                absolute	ISC oper	EF	3	6
                absolut,X	ISC oper,X	FF	3	7
                absolut,Y	ISC oper,Y	FB	3	7
                (indirect,X)	ISC (oper,X)	E3	2	8
                (indirect),Y	ISC (oper),Y	F3	2	4  */
            0xE7 | 0xF7 | 0xEF | 0xFF | 0xFB | 0xE3 | 0xF3 => {
                self.IISB(instr, bytes, &mut stepstring)
            }
            ,
            /*
            SLO
                zeropage	SLO oper	07	2	5
                zeropage,X	SLO oper,X	17	2	6
                absolute	SLO oper	0F	3	6
                absolut,X	SLO oper,X	1F	3	7
                absolut,Y	SLO oper,Y	1B	3	7
                (indirect,X)	SLO (oper,X)	03	2	8
                (indirect),Y	SLO (oper),Y	13	2	8  */
            0x07 | 0x17 | 0x0F | 0x1F | 0x1B | 0x03 | 0x13 => {
                self.ISLO(instr, bytes, &mut stepstring)
            }

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
            0x27 | 0x37 | 0x2F | 0x3F | 0x3B | 0x23 | 0x33 => {
                self.IRLA(instr, bytes, &mut stepstring)
            }
            ,/*
            SRE
            zeropage	SRE oper	47	2	5
            zeropage,X	SRE oper,X	57	2	6
            absolute	SRE oper	4F	3	6
            absolut,X	SRE oper,X	5F	3	7
            absolut,Y	SRE oper,Y	5B	3	7
            (indirect,X)	SRE (oper,X)	43	2	8
            (indirect),Y	SRE (oper),Y	53	2	8 */
            0x47 | 0x57 | 0x4F | 0x5F | 0x5B | 0x43 | 0x53 => {
                self.ISRE(instr, bytes, &mut stepstring)
            }
            ,            /*
            RRA
            zeropage	RRA oper	67	2	5
            zeropage,X	RRA oper,X	77	2	6
            absolute	RRA oper	6F	3	6
            absolut,X	RRA oper,X	7F	3	7
            absolut,Y	RRA oper,Y	7B	3	7
            (indirect,X)	RRA (oper,X)	63	2	8
            (indirect),Y	RRA (oper),Y	73	2	8 */
            0x67 | 0x77 | 0x6F | 0x7F | 0x7B | 0x63 | 0x73 => {
                self.IRRA(instr, bytes, &mut stepstring)
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

        Ok(stepstring)
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
                //TODO: need to halt here
            }
        }

        match addr {
            //WRAM(2kb) + 3 mirrors
            0x0000..=0x1FFF => {
                //mod by 2048 since we have 3 mirrors
                let final_addr = addr % 2048;

                self.wram.contents[final_addr as usize..final_addr as usize + length].into()
            }
            //PPU control regs a PM at gs (8 bytes) + a fuckton of mirrors
            0x2000..=0x3FFF => {
                //return [].into();
                unimplemented!("tried to read ppu control regs")
            }
            //registers (apu and io)
            0x4000..=0x4017 => {
                return vec![0; 1];
                //unimplemented!("tried to read apu/io regs")
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
            0x8000..=0xFFFF => self.cart.read(addr, length),
        }
    }
    pub fn write(&mut self, addr: u16, bytes: &Vec<u8>) {
        for a in addr as usize..=(addr as usize + bytes.len()) {
            if self.watchpoints.contains(&(a as usize)) {
                //TODO: need to halt here
            }
        }

        match addr {
            //WRAM(2kb) + 3 mirrors
            0x0000..=0x1FFF => {
                let base_addr = addr % 2048;

                for (i, b) in bytes.iter().enumerate() {
                    //write value into ram
                    self.wram.contents[(base_addr as usize) + i] = *b;
                }
            }
            //PPU control regs (8 bytes) + a fuckton of mirrors
            0x2000..=0x3FFF => {
                unimplemented!("tried to write to ppu control regs")
            }
            //registers (apu and io)
            0x4000..=0x4017 => {
                //unimplemented!("tried to wrote to apu/io regs")
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
        //ret_str.push_str(&format!("{:02X} ", b));
        write!(ret_str, "{:02X} ", b).unwrap();
    }
    ret_str
}
