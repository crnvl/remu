use bitflags::bitflags;

bitflags! {
    pub struct ControlRegister: u8 {
        const NAMETABLE1                 = 0b00000001;
        const NAMETABLE2                 = 0b00000010;
        const VRAM_ADD_INCREMENT         = 0b00000100;
        const SPRITE_PATTERN_ADDR        = 0b00001000;
        const BACKGROUND_PATTERN_ADDR    = 0b00010000;
        const SPRITE_SIZE                = 0b00100000;
        const MASTER_SLAVE_SELECT        = 0b01000000;
        const GENERATE_NMI               = 0b10000000;
    }
}

impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister::from_bits_truncate(0b0000_0000)
    }

    pub fn nametable_addr(&self) -> u16 {
        match self.bits & 0b11 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => panic!("not possible"),
        }
    }

    pub fn vram_addr_increment(&self) -> u8 {
        if !self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
            1
        } else {
            32
        }
    }

    pub fn sprt_pattern_addr(&self) -> u16 {
        if !self.contains(ControlRegister::SPRITE_PATTERN_ADDR) {
            0
        } else {
            0x1000
        }
    }

    pub fn bknd_pattern_addr(&self) -> u16 {
        if !self.contains(ControlRegister::BACKGROUND_PATTERN_ADDR) {
            0
        } else {
            0x1000
        }
    }

    pub fn sprite_size(&self) -> u8 {
        if !self.contains(ControlRegister::SPRITE_SIZE) {
            8
        } else {
            16
        }
    }

    pub fn master_slave_select(&self) -> u8 {
        if !self.contains(ControlRegister::MASTER_SLAVE_SELECT) {
            0
        } else {
            1
        }
    }

    pub fn generate_vblank_nmi(&self) -> bool {
        return self.contains(ControlRegister::GENERATE_NMI);
    }

    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }
}
