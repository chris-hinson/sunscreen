#[allow(dead_code)]
#[derive(Clone)]
#[allow(non_snake_case)]
pub struct Ppu {
    //256B OAM
    OAM: [u8; 256],
    //OAM DMA?
    //Palette
    //Control Registers (these need to be memory mapped)?
    regs: PPUREGS,
}

/* PPU MEMORY MAP
$0000 	$1000 	Pattern Table 0
$1000 	$1000 	Pattern Table 1
$2000 	$3C0 	Name Table 0
$23C0 	$40 	Attribute Table 0
$2400 	$3C0 	Name Table 1
$27C0 	$40 	Attribute Table 1
$2800 	$3C0 	Name Table 2
$2BC0 	$40 	Attribute Table 2
$2C00 	$3C0 	Name Table 3
$2FC0 	$40 	Attribute Table 3
$3000 	$F00 	Mirror of 2000h-2EFFh
$3F00 	$10 	BG Palette
$3F10 	$10 	Sprite Palette
$3F20 	$E0 	Mirror of 3F00h-3F1Fh */
impl Ppu {
    pub fn new() -> Self {
        Ppu {
            OAM: [0; 256],
            regs: PPUREGS::new(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct PPUREGS {
    //PPUCTRL
    PPUCTRL: PPUCTRL,
    //PPUMASK
    //PPUSTATUS
    //OAMADDR
    //OAMDATA
    //PPUSCROLL
    //PPUADDR
    //PPUDATA
    //OAMDMA
}
impl PPUREGS {
    fn new() -> Self {
        Self {
            PPUCTRL: PPUCTRL::new(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone)]
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
impl PPUCTRL {
    pub fn new() -> Self {
        PPUCTRL::from(0)
    }
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
