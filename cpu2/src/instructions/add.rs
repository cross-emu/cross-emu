use crate::Cpu;
use crate::defines::Flag;
use crate::flags::FlagsOps;
use crate::implemenation::{Reg8, Reg16};

pub fn add_r8_r8<Src: Reg8, Dest: Reg8>(cpu: &mut Cpu) {
    let src = cpu.get_r8::<Src>();
    let dest = cpu.get_r8::<Dest>();

    cpu.set_r8::<Dest>(dest.wrapping_add(src));

    cpu.registers.flags.set_flag(Flag::Subtract, false);
    cpu.registers
        .flags
        .set_flag(Flag::HalfCarry, (src & 0x0F) + (dest & 0x0F) > 0x0F);

    cpu.registers
        .flags
        .set_flag(Flag::Carry, (src as u16) + (dest as u16) > 0xFF);
}

pub fn add_accu_in_r8<Dest: Reg8>(cpu: &mut Cpu) {
    cpu.set_r8::<Dest>(
        cpu.get_r8::<Dest>()
            .wrapping_add(cpu.accumulator.get_u8_at(0)),
    );
}
