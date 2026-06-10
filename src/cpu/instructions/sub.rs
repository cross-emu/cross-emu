use crate::cpu::defines::Cpu;
use crate::cpu::defines::Flag;
use crate::cpu::flags::FlagsOps;
use crate::cpu_def::Reg8;
use crate::mmu::MemoryMapper;

impl<'a, M: MemoryMapper> Cpu<'a, M> {
    pub fn sub_r8_r8<Dest: Reg8, Src: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = dest.wrapping_add(src);

        Self::set_r8::<Dest>(self, result);

        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, true);
        self.flags
            .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

        self.flags
            .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
    }

    pub fn sub_r8_r8_with_carry<Src: Reg8, Dest: Reg8>(&mut self, _bus: &mut M) {
        let src = Self::get_r8::<Src>(self);
        let dest = Self::get_r8::<Dest>(self);

        let result = src - dest - self.flags.get_flag(Flag::Zero) as u8;
        Self::set_r8::<Dest>(self, result);

        self.flags.set_flag(Flag::Zero, result == 0);
        self.flags.set_flag(Flag::Subtract, true);
        self.flags
            .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

        self.flags
            .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
    }
}
