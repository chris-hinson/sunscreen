use crate::cart::Cart;
use crate::vram::Vram;

#[allow(dead_code)]
#[derive(Clone)]
#[allow(non_snake_case)]
//NOTE: the *general* idea for our cpu, is that we want to build a full frame pixel by pixel
//and then only send a full frame to sdl. this WILL need some fancy timing stuff but thats a later issue
pub struct Ppu {
    //256B OAM
    pub OAM: [u8; 256],
    //OAM DMA? - this *technically* goes through the ppuregs via OAMADDR and OAMDMA
    //Palette??
    //Control Registers - THESE ARE MEMORY MAPPED IN CPU'S MEM SPACE
    pub regs: PPUREGS,

    pub vram: Vram,
    pub cart: Cart,

    //some metadata about where we are in the rendering process
    pub cur_dot: usize,
    pub cur_line: usize,

    //total cycles from boot
    pub cycles: usize,

    //number of frames from boot
    pub frams: usize,
}

impl Ppu {
    pub fn new(cart: Cart) -> Self {
        Ppu {
            OAM: [0; 256],
            regs: PPUREGS::new(),
            vram: Vram::new(),
            cart,
            cur_dot: 0,
            cur_line: 0,
            cycles: 0,
            frams: 0,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Copy)]
pub struct PPUREGS {
    //PPUCTRL: 0x2000
    pub PPUCTRL: PPUCTRL,
    //PPUMASK: 0x2001
    pub PPUMASK: PPUMASK,
    //PPUSTATUS: 0X2002
    pub PPUSTATUS: PPUSTATUS,
    //OAMADDR: 0x2003
    pub OAMADDR: u8,
    //OAMDATA: 0x2004
    pub OAMDATA: u8,
    //PPUSCROLL: 0x2005
    pub PPUSCROLL: u8,
    //PPUADDR: 0x2006
    pub PPUADDR: u8,
    //PPUDATA: 0x2007
    pub PPUDATA: u8,
    //OAMDMA: 0x4014
    pub OAMDMA: u8,
}
impl PPUREGS {
    fn new() -> Self {
        Self {
            PPUCTRL: 0.into(),
            PPUMASK: 0.into(),
            PPUSTATUS: 0.into(),
            OAMADDR: 0,
            OAMDATA: 0,
            PPUSCROLL: 0,
            PPUADDR: 0,
            PPUDATA: 0,
            OAMDMA: 0,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Copy)]
pub struct PPUCTRL {
    //VPHB SINN

    //NMI ENABLE
    nmi_enable: bool,
    //PPU MASTER/SLAVE
    ppu_master_slave: bool,
    //SPRITE HEIGHT
    sprite_height: bool,
    //BACKGROUND TILE SELECT
    background_tile_select: bool,
    //SPRITE TILE SELECT
    sprite_tile_select: bool,
    //INCREMENT MODE
    increment_mode: bool,
    //NAMETABLE SELECT
    nametable_select: u8,
}
impl From<u8> for PPUCTRL {
    fn from(val: u8) -> Self {
        PPUCTRL {
            nmi_enable: (val & 0b1000_0000) >> 7 == 1,
            ppu_master_slave: (val & 0b0100_0000) >> 6 == 1,
            sprite_height: (val & 0b0010_0000) >> 5 == 1,
            background_tile_select: (val & 0b0001_0000) >> 4 == 1,
            sprite_tile_select: (val & 0b0000_1000) >> 3 == 1,
            increment_mode: (val & 0b0000_0100) >> 2 == 1,
            nametable_select: (val & 0b0000_0011),
        }
    }
}
impl Into<u8> for PPUCTRL {
    fn into(self) -> u8 {
        let mut ret: u8 = 0;
        if self.nmi_enable {
            ret |= 0b1000_0000
        }
        if self.ppu_master_slave {
            ret |= 0b0100_0000
        }
        if self.sprite_height {
            ret |= 0b0010_0000
        }
        if self.background_tile_select {
            ret |= 0b0001_0000
        }
        if self.sprite_tile_select {
            ret |= 0b0000_1000
        }
        if self.increment_mode {
            ret |= 0b0000_0100
        }
        ret |= self.nametable_select;

        ret
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Copy)]
pub struct PPUMASK {
    BLUE: bool,
    GREEN: bool,
    RED: bool,
    SPRITE_ENABLE: bool,
    BACKGROUND_EABLE: bool,
    LEFT_SPRITE_HIDE: bool,
    LEFT_BACKGROUND_HIDE: bool,
    GREYSCALE: bool,
}
impl From<u8> for PPUMASK {
    fn from(val: u8) -> Self {
        PPUMASK {
            BLUE: (val & 0b1000_0000) != 0,
            GREEN: (val & 0b0100_0000) != 0,
            RED: (val & 0b0010_0000) != 0,
            SPRITE_ENABLE: (val & 0b0001_0000) != 0,
            BACKGROUND_EABLE: (val & 0b0000_1000) != 0,
            LEFT_SPRITE_HIDE: (val & 0b0000_0100) != 0,
            LEFT_BACKGROUND_HIDE: (val & 0b0000_0010) != 0,
            GREYSCALE: (val & 0b0000_0001) != 0,
        }
    }
}
impl Into<u8> for PPUMASK {
    fn into(self) -> u8 {
        let mut ret: u8 = 0;
        if self.BLUE {
            ret |= 0b1000_0000;
        }
        if self.GREEN {
            ret |= 0b0100_0000;
        }
        if self.RED {
            ret |= 0b0010_0000;
        }
        if self.SPRITE_ENABLE {
            ret |= 0b0001_0000;
        }
        if self.BACKGROUND_EABLE {
            ret |= 0b0000_1000;
        }
        if self.LEFT_SPRITE_HIDE {
            ret |= 0b0000_0100;
        }
        if self.LEFT_BACKGROUND_HIDE {
            ret |= 0b0000_0010;
        }
        if self.GREYSCALE {
            ret |= 0b0000_0001;
        }

        ret
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Copy)]
pub struct PPUSTATUS {
    VBLANK: bool,
    SPRITE_0_HIT: bool,
    SPRITE_OVERFLOW: bool,
    //5-BIT STALE PPU BUS CONTENTS?
}
impl From<u8> for PPUSTATUS {
    fn from(val: u8) -> Self {
        PPUSTATUS {
            VBLANK: (val & 0b1000_0000) != 0,
            SPRITE_0_HIT: (val & 0b0100_0000) != 0,
            SPRITE_OVERFLOW: (val & 0b0010_0000) != 0,
        }
    }
}
impl Into<u8> for PPUSTATUS {
    fn into(self) -> u8 {
        let mut ret: u8 = 0;
        if self.VBLANK {
            ret |= 0b1000_0000;
        }
        if self.SPRITE_0_HIT {
            ret |= 0b0100_0000;
        }
        if self.SPRITE_OVERFLOW {
            ret |= 0b0010_0000;
        }

        ret
    }
}

impl Ppu {
    //tick our ppu one step
    pub fn step(&mut self) {
        match self.cur_line {
            //visible lines
            0..=239 => match self.cur_dot {
                //idle cycle
                0 => {}
                //data fetch AND next line sprite eval
                1..=256 => {
                    //1.Nametable byte
                    //fetch from $2000
                    self.read(0x2000, 1);
                    //2.Attribute table byte
                    //3.Pattern table tile low
                    //4.Pattern table tile high (+8 bytes from pattern table tile low)
                }
                //NEXT LINE sprite fetch
                257..=320 => {}
                //NEXT LINE first two tiles
                321..=336 => {}
                _ => {
                    unreachable!("WE ARE ON A SCANLINE DOT THAT DOES NOT EXIST")
                }
            },
            //post-render scanline
            240..=240 => {}
            //vblanking
            241..=260 => {}
            //dummy line
            261 => {
                //During pixels 280 through 304 of this scanline, the vertical scroll bits are reloaded if rendering is enabled.
            }
            _ => unreachable!("IN A SCANLINE THAT DOESNT EXIST"),
        }
    }
}
