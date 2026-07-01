pub struct Wram {
    bank0: [u8; 0x100],
    bank1: [u8; 0x100],
    bank2: [u8; 0x100],
    bank3: [u8; 0x100],
    bank4: [u8; 0x100],
    bank5: [u8; 0x100],
    bank6: [u8; 0x100],
    bank7: [u8; 0x100],
    svbk_wbk: u8, //0xFF70
}

impl Wram {
    pub fn new() -> Self {
        Self {
            bank0: [0x00; 0x100],
            bank1: [0x00; 0x100],
            bank2: [0x00; 0x100],
            bank3: [0x00; 0x100],
            bank4: [0x00; 0x100],
            bank5: [0x00; 0x100],
            bank6: [0x00; 0x100],
            bank7: [0x00; 0x100],
            svbk_wbk: 0x00,
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match self.svbk_wbk() {
            0x00 => self.bank0[addr as usize],
            0x01 => self.bank1[addr as usize],
            0x02 => self.bank2[addr as usize],
            0x03 => self.bank3[addr as usize],
            0x04 => self.bank4[addr as usize],
            0x05 => self.bank5[addr as usize],
            0x06 => self.bank6[addr as usize],
            0x07 => self.bank7[addr as usize],
            _ => 0x00,
        }
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match self.svbk_wbk() {
            0x00 => self.bank0[addr as usize] = data,
            0x01 => self.bank1[addr as usize] = data,
            0x02 => self.bank2[addr as usize] = data,
            0x03 => self.bank3[addr as usize] = data,
            0x04 => self.bank4[addr as usize] = data,
            0x05 => self.bank5[addr as usize] = data,
            0x06 => self.bank6[addr as usize] = data,
            0x07 => self.bank7[addr as usize] = data,
            _ => {}
        }
    }

    pub fn svbk_wbk(&self) -> u8 {
        self.svbk_wbk & 0b00000111
    }

    pub fn set_svbk_wbk(&mut self, wbk: u8) {
        self.svbk_wbk = wbk;
    }
}
