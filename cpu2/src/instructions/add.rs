use crate::defines::{Cpu, Flag};
use crate::flags::{self, FlagsOps};
use crate::implemenation::Reg8;

pub fn add_r8_r8<Src: Reg8, Dest: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let result = dest.wrapping_add(src);

    cpu.set_r8::<Dest>(result);

    if result == 0 {
        cpu.flags.set_flag(Flag::Zero, true);
    } else {
        cpu.flags.set_flag(Flag::Zero, false);
    }
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags
        .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

    cpu.flags
        .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
}

pub fn add_r8_r8_with_carry<Src: Reg8, Dest: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let result = src + dest + cpu.flags.get_flag(Flag::Zero) as u8;
    cpu.set_r8::<Dest>(result);

    if result == 0 {
        cpu.flags.set_flag(Flag::Zero, true);
    } else {
        cpu.flags.set_flag(Flag::Zero, false);
    }
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags
        .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

    cpu.flags
        .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
}
