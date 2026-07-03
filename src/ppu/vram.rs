pub struct CgbVram {
    bank0: [u8; 0x2000],
    bank1: [u8; 0x2000],
    vbk: u8, // 0xFF4F
}

impl Default for CgbVram {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DmgVram {
    bank0: [u8; 0x2000],
}

impl Default for DmgVram {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Vram {
    fn new() -> Self
    where
        Self: Sized;
    fn write(&mut self, addr: u16, byte: u8);
    fn read(&self, addr: u16) -> u8;
    fn set_vbk(&mut self, value: u8);
    fn get_vbk(&self, attribute: u8) -> u8;
    fn vbk(&self) -> u8;
    fn read_with_custom_vbk(&self, _addr: u16, _custom_vbk: u8) -> u8 {
        0
    }
    fn write_with_custom_vbk(&mut self, addr: u16, value: u8, custom_vbk: u8);
}

impl Vram for DmgVram {
    fn new() -> Self {
        Self { bank0: [0; 0x2000] }
    }
    fn write(&mut self, addr: u16, byte: u8) {
        let addr_in_bank = addr - 0x8000;
        self.bank0[addr_in_bank as usize] = byte;
    }
    fn read(&self, addr: u16) -> u8 {
        let addr_in_bank = addr - 0x8000;
        self.bank0[addr_in_bank as usize]
    }
    fn set_vbk(&mut self, _value: u8) {}

    fn vbk(&self) -> u8 {
        0
    }
    fn write_with_custom_vbk(&mut self, _addr: u16, _value: u8, _custom_vbk: u8) {}

    fn get_vbk(&self, _attribute: u8) -> u8 {
        0
    }
}

impl Vram for CgbVram {
    fn new() -> Self {
        Self {
            bank0: [0x00; 0x2000],
            bank1: [0x00; 0x2000],
            vbk: 0xFF,
        }
    }
    fn write(&mut self, addr: u16, byte: u8) {
        let addr_in_bank = addr - 0x8000;
        match self.vbk() {
            0x00 => self.bank0[addr_in_bank as usize] = byte,
            0x01 => self.bank1[addr_in_bank as usize] = byte,
            _ => {}
        }
    }
    fn read(&self, addr: u16) -> u8 {
        let addr_in_bank = addr - 0x8000;
        match self.vbk() {
            0x00 => self.bank0[addr_in_bank as usize],
            0x01 => self.bank1[addr_in_bank as usize],
            _ => unreachable!(),
        }
    }

    fn set_vbk(&mut self, value: u8) {
        self.vbk = value & 0x01;
    }

    fn get_vbk(&self, attribute: u8) -> u8 {
        (attribute >> 3) & 1
    }

    fn vbk(&self) -> u8 {
        self.vbk & 0b00000001
    }

    fn read_with_custom_vbk(&self, addr: u16, custom_vbk: u8) -> u8 {
        let addr_in_bank = addr - 0x8000;
        match custom_vbk {
            0x00 => self.bank0[addr_in_bank as usize],
            0x01 => self.bank1[addr_in_bank as usize],
            _ => unreachable!(),
        }
    }

    fn write_with_custom_vbk(&mut self, addr: u16, byte: u8, custom_vbk: u8) {
        let addr_in_bank = addr - 0x8000;
        match custom_vbk {
            0x00 => self.bank0[addr_in_bank as usize] = byte,
            0x01 => self.bank1[addr_in_bank as usize] = byte,
            _ => {}
        }
    }
}
