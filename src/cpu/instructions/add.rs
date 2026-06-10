use crate::cpu::defines::Flag;
use crate::cpu::flags::FlagsOps;
use crate::cpu_def::*;
use crate::mmu::MemoryMapper;
use crate::{cpu::defines::Cpu, cpu_def::Reg8};

impl<'a, M: MemoryMapper> Cpu<'a, M> {
    pub fn add_r8_r8<Src: Reg8, Dest: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = dest.wrapping_add(src);

        Self::set_r8::<Dest>(self, result);

        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags
            .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

        self.flags
            .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
    }

    pub fn add_r8_r8_with_carry<Src: Reg8, Dest: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = src + dest + self.flags.get_flag(Flag::Zero) as u8;
        Self::set_r8::<Dest>(self, result);

        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags
            .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

        self.flags
            .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
    }

    pub fn add_r8_r8_no_zero_flag<Src: Reg8, Dest: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = dest.wrapping_add(src);

        self.set_r8::<Dest>(result);

        self.flags.set_flag(Flag::Subtract, false);
        self.flags
            .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

        self.flags
            .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
    }

    pub fn add_r8_r8_with_carry_and_no_zero_flag<Src: Reg8, Dest: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = src + dest + self.flags.get_flag(Flag::Zero) as u8;
        self.set_r8::<Dest>(result);

        self.flags.set_flag(Flag::Subtract, false);
        self.flags
            .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

        self.flags
            .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
    }

    pub fn add_hl_sp_e_low(&mut self, _bus: &mut M) {
        let sp_low = Self::get_r8::<P>(self);
        let e = Self::get_r8::<Z>(self);
        let result = sp_low.wrapping_add(e);
        Self::set_r8::<Z>(self, result);

        let h = (sp_low & 0x0F) + (e & 0x0F) > 0x0F;
        let c = (sp_low as u16) + (e as u16) > 0xFF;

        self.flags.set_flag(Flag::Zero, false);
        self.flags.set_flag(Flag::Subtract, false);
        self.flags.set_flag(Flag::HalfCarry, h);
        self.flags.set_flag(Flag::Carry, c);
    }

    pub fn add_hl_sp_e_high(&mut self, _bus: &mut M) {
        let sp_high = Self::get_r8::<S>(self);
        let e = Self::get_r8::<Z>(self);
        let adj: u8 = if e & 0x80 != 0 { 0xFF } else { 0x00 };
        let carry: u8 = self.flags.get_flag(Flag::Carry) as u8;

        self.set_r8::<W>(sp_high.wrapping_add(adj).wrapping_add(carry));
    }
}
