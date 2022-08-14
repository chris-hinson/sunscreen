use crate::cart::Cart;
use crate::nes::NES;
use crate::ppu::Ppu;
use crate::vram::Vram;
impl NES {
    //memory operations

    //TODO: this is our cpu memory map. ppu has its own memory map
    /*CPU Memory Map (16bit buswidth, 0-FFFFh)

    0000h-07FFh   Internal 2K Work RAM (mirrored to 800h-1FFFh)
    2000h-2007h   Internal PPU Registers (mirrored to 2008h-3FFFh)
    4000h-4017h   Internal APU Registers
    4018h-5FFFh   Cartridge Expansion Area almost 8K
    6000h-7FFFh   Cartridge SRAM Area 8K
    8000h-FFFFh   Cartridge PRG-ROM Area 32K*/

    //so when the cpu reads or writes to an address, these functions should dispatch the rw to
    //the appropriate part
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
                let final_addr = (addr - 0x2000) % 8;

                return match final_addr {
                    0x0 => vec![self.ppu.regs.PPUCTRL.into()],
                    0x1 => vec![self.ppu.regs.PPUMASK.into()],
                    0x2 => vec![self.ppu.regs.PPUSTATUS.into()],
                    0x3 => vec![self.ppu.regs.OAMADDR],
                    0x4 => vec![self.ppu.regs.OAMDATA],
                    0x5 => vec![self.ppu.regs.PPUSCROLL],
                    0x6 => vec![self.ppu.regs.PPUADDR],
                    0x7 => vec![self.ppu.regs.PPUDATA],
                    _ => unreachable! {"TRIED TO READ A PPU CONTROL REG THAT DOESNT EXIST"},
                };
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
            0x8000..=0xFFFF => self.ppu.cart.cpu_read(addr, length),
        }
    }
    pub fn write(&mut self, addr: u16, bytes: &Vec<u8>) {
        /*for a in addr as usize..=(addr as usize + bytes.len()) {
            if self.watchpoints.contains(&(a as usize)) {
                //TODO: need to halt here
            }
        }*/

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
                let final_addr = (addr - 0x2000) % 8;

                match final_addr {
                    0x0 => self.ppu.regs.PPUCTRL = bytes[0].into(),
                    0x1 => self.ppu.regs.PPUMASK = bytes[0].into(),
                    0x2 => self.ppu.regs.PPUSTATUS = bytes[0].into(),
                    0x3 => self.ppu.regs.OAMADDR = bytes[0],
                    0x4 => self.ppu.regs.OAMDATA = bytes[0],
                    0x5 => self.ppu.regs.PPUSCROLL = bytes[0],
                    0x6 => self.ppu.regs.PPUADDR = bytes[0],
                    0x7 => self.ppu.regs.PPUDATA = bytes[0],
                    _ => unreachable! {"TRIED TO WRITE A PPU CONTROL REG THAT DOESNT EXIST"},
                };
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

/* PPU MEMORY MAP
$0000 	$1000 	Pattern Table 0          -cart
$1000 	$1000 	Pattern Table 1          -cart
$2000 	$3C0 	Name Table 0             -vram?
$23C0 	$40 	Attribute Table 0        -vram?
$2400 	$3C0 	Name Table 1             -vram?
$27C0 	$40 	Attribute Table 1        -vram?
$2800 	$3C0 	Name Table 2             -cart?
$2BC0 	$40 	Attribute Table 2        -cart?
$2C00 	$3C0 	Name Table 3             -cart?
$2FC0 	$40 	Attribute Table 3        -cart?
$3000 	$F00 	Mirror of 2000h-2EFFh
$3F00 	$10 	BG Palette               -internal?
$3F10 	$10 	Sprite Palette           -internal
$3F20 	$E0 	Mirror of 3F00h-3F1Fh    -internal
*/
//VRAM is 2kb, bound to $2000-2FFF (can apparently be rerouted??)

//CHRIS THE CARTRIDGE CONTROLS WHAT HARDWARE EVERYTHING GETS ROUTED TO
//"The mappings above are the fixed addresses from which the PPU uses to fetch data during rendering.
//The actual device that the PPU fetches data from, however, may be configured by the cartridge. "
/*
$0000-1FFF is normally mapped by the cartridge to a CHR-ROM or CHR-RAM, often with a bank switching mechanism.
$2000-2FFF is normally mapped to the 2kB NES internal VRAM, providing 2 nametables with a mirroring configuration controlled by the cartridge, but it can be partly or fully remapped to RAM on the cartridge, allowing up to 4 simultaneous nametables.
$3000-3EFF is usually a mirror of the 2kB region from $2000-2EFF. The PPU does not render from this address range, so this space has negligible utility.
$3F00-3FFF is not configurable, always mapped to the internal palette control. */

impl Ppu {
    pub fn read(&mut self, addr: u16, len: usize) -> Vec<u8> {
        //unimplemented!("NO READING FROM PPU YET!");
        match addr {
            0x0000..=0x1FFF => {
                //goes to cart. mapping nightmares ensue
                //panic!("ppu tried to read from cart")
                return self.cart.ppu_read(addr, len);
            }
            0x2000..=0x2FFF => {
                //VRAM! 2k

                //because addresses in vram are simply the addresses in the ppu space - 0x2000
                let final_addr = addr - 0x2000;
                return self.vram.contents
                    [final_addr as usize..(final_addr as usize + len) as usize]
                    .into();
            }
            0x3000..=0x3EFF => {
                //mirror of VRAM
                let final_addr = addr - 0x3000;
                return self.vram.contents
                    [final_addr as usize..(final_addr as usize + len) as usize]
                    .into();
            }
            0x3F00..=0x3FFF => {
                //internal palette control
                panic!("ppu tried to read from internal paletter control")
            }
            _ => panic!("reading from bad ppu addr"),
        }

        //return vec![0];
    }
    pub fn write(&mut self, addr: u16, bytes: &Vec<u8>) {
        //unimplemented!("NO WRITING FROM PPU YET!")
        match addr {
            0x0000..=0x1FFF => {
                //goes to cart. mapping nightmares ensue
                //panic!("ppu tried to write to cart")
                for byte in bytes {
                    self.cart.ppu_write(addr, *byte);
                }
            }
            0x2000..=0x2FFF => {
                //VRAM!
                let final_addr = addr - 0x2000;
                for (i, byte) in bytes.iter().enumerate() {
                    self.vram.contents[final_addr as usize + i] = *byte;
                }
            }
            0x3000..=0x3EFF => {
                //mirror of VRAM (notice youre missing the last few bytes lol)
                let final_addr = addr - 0x3000;
                for (i, byte) in bytes.iter().enumerate() {
                    self.vram.contents[final_addr as usize + i] = *byte;
                }
            }
            0x3F00..=0x3FFF => {
                //internal palette control
                panic!("ppu tried to write to internal palette control")
            }
            _ => panic!("reading from bad ppu addr"),
        }
    }
}
