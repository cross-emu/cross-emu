use crate::cpu::defines::Flag;
use crate::cpu::flags::FlagsOps;
use crate::{cpu::defines::Cpu, cpu_def::Reg8};

pub fn cp_r8_r8<Dest: Reg8, Src: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    let result = dest.wrapping_sub(src);

    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, true);
    cpu.flags
        .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

    cpu.flags
        .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
}
