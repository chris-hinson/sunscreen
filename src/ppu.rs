use crate::cart::Cart;
use crate::vram::Vram;
use std::fmt::Write;
use std::sync::mpsc::Sender;

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
    pub frames: usize,
    //mpsc channel for sending a frame to the app thread
    pub channel: Sender<Vec<u8>>,
    pub frame: Vec<u8>,
}

impl Ppu {
    pub fn new(cart: Cart, channel: Sender<Vec<u8>>) -> Self {
        Ppu {
            OAM: [0; 256],
            regs: PPUREGS::new(),
            vram: Vram::new(),
            cart,
            cur_dot: 0,
            cur_line: 0,
            cycles: 0,
            frames: 0,
            channel,
            frame: vec![200; 184_320],
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
    //tick our ppu one CLOCK CYCLE
    //TODO: this is our remnants of a per-pixel renderer
    /*
    pub fn step(&mut self) {
        match self.cur_line {
            //visible lines
            0..=239 => {
                match self.cur_dot {
                    //idle cycle
                    0 => {
                        //we literally do nothing on this cycle
                    }
                    //data fetch AND next line sprite eval
                    1..=256 => {
                        let phase = (self.cur_dot - 1) % 8;
                        match phase {
                            //1.Nametable byte
                            0 => {
                                //fetch from $2000
                                self.read(0x2000, 1);
                            }
                            1 => {
                            }
                            //2.Attribute table byte
                            2 => {}
                            3 => {}
                            //3.Pattern table tile low
                            4 => {}
                            5 => {}
                            //4.Pattern table tile high (+8 bytes from pattern table tile low)
                            6 => {}
                            7 => {}
                            _ => unreachable!("we are in a visible line data fetch phase, but modded an invalid value")
                        }
                    }
                    //NEXT LINE sprite fetch
                    257..=320 => {}
                    //NEXT LINE first two tiles
                    321..=336 => {
                        //send the frame data on the last dot of the last visible line
                        if self.cur_line == 239 && self.cur_dot == 336 {
                            self.channel.send(self.frame.clone()).unwrap();
                            self.frames += 1;
                        }
                    }
                    //unknown bytes fetch
                    337..=340 => {}
                    _ => {
                        unreachable!("WE ARE ON A SCANLINE DOT THAT DOES NOT EXIST")
                    }
                }
            }

            //post-render scanline
            240 => {}
            //vblanking
            241..=260 => {}
            //dummy line
            261 => {
                //During pixels 280 through 304 of this scanline,
                //the vertical scroll bits are reloaded if rendering is enabled.
            }
            _ => unreachable!("IN A SCANLINE THAT DOESNT EXIST"),
        }

        self.tick_beam();
    }*/

    pub fn step(&mut self) -> Result<String, String> {
        let mut log_line = String::new();
        match self.cur_line {
            //visible lines
            0..=239 => {
                //For each pixel in the background buffer,
                //the corresponding sprite pixel replaces it
                //only if the sprite pixel is opaque and front priority
                //or if the background pixel is transparent.

                //what row of tiles are we on
                let tile_row = self.cur_line / 8;

                //TODO: how do we know what nametable to read from?
                //since even with mirroring there are still 2 to choose from
                let nametable_base = 0x2000;
                let addr = nametable_base + tile_row * 32;

                //fetch 32 tiles (32 bytes) these bytes are actually indexes into our pattern table
                let tile_data = self.read(addr as u16, 32);

                //now we need to get our patterns
                for (i, tile) in tile_data.iter().enumerate() {
                    let pattern_table_base = 0x0000;
                    let pattern_addr = pattern_table_base + tile * 16;
                    let pattern_raw = self.read(pattern_addr as u16, 16);
                    let pattern_hi_plane = &pattern_raw[0..8];
                    let pattern_lo_plane = &pattern_raw[8..16];

                    //bitwise or every byte in the pattern planes to yield our final pattern
                    let pattern_final: Vec<u8> = pattern_hi_plane
                        .iter()
                        .enumerate()
                        .map(|(i, v)| *v | pattern_lo_plane[i])
                        .collect();

                    let pixels = pattern_final[self.cur_line % 8];
                    let pixels_rgb = self.u8_to_rgb(pixels);
                    let framebuffer_addr = self.cur_line * 256 * 3 + i * 8 * 3;
                    for i in 0..24 {
                        self.frame[framebuffer_addr + i] = pixels_rgb[i];
                    }
                    write!(
                        &mut log_line,
                        "PPU: line is {}, RGB data is: {:?}",
                        self.cur_line, pixels_rgb
                    )
                    .unwrap();
                }
            }
            //post-render scanline
            240 => {
                //literally do nothing. safe to access ppu memory, but no vblank flag has been raised
            }
            //vblanking
            241..=260 => {}
            //dummy line
            261 => {
                //During pixels 280 through 304 of this scanline,
                //the vertical scroll bits are reloaded if rendering is enabled.

                if self.cur_dot == 341 {
                    //self.channel.send(vec![255; 184_320]).unwrap();
                    self.channel.send(self.frame.clone()).unwrap();
                    self.tick_beam();
                    return Err("just sent a frame".to_string());
                };
            }
            _ => unreachable!("IN A SCANLINE THAT DOESNT EXIST"),
        }

        self.tick_beam();
        return Ok(log_line);
    }

    fn tick_beam(&mut self) {
        //increase the dot we're on, wrapping to 0 the end of line
        if self.cur_dot >= 341 {
            self.cur_dot = 0;
            //if we're wrapping the dot, we need to increase our line,
            //wrapping to 0 after all lines
            if self.cur_line >= 261 {
                self.cur_line = 0;
            } else {
                self.cur_line += 1;
            }
        } else {
            self.cur_dot += 1
        };
    }

    fn u8_to_rgb(&self, pixels: u8) -> Vec<u8> {
        let mut ret_vec = vec![0; 24];
        for i in 7..=0 {
            let bit = (pixels >> i & 0x1) != 0;
            if bit {
                ret_vec[(7 - i) * 3] = 255;
                ret_vec[((7 - i) * 3) + 1] = 255;
                ret_vec[((7 - i) * 3) + 2] = 255;
            }
        }

        return ret_vec;
    }
}
