use crate::implemenation::{self, A, HL, Reg8};
use crate::instructions::load::write_memory;
use crate::{Cpu, defines::Flag, flags::FlagsOps, implemenation::Reg16};

//Some ops effectively use 2 cycles but work on one (i.e. LD (HL), r) so that we put a nothing op so it stills takes two cycles and fetch accordingly
pub fn noop(_cpu: &mut Cpu) {}

pub fn halt(_cpu: &mut Cpu) {
    todo!();
}

pub fn decrement_r16<Reg: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Reg>(cpu.get_r16::<Reg>().wrapping_sub(1));
}

pub fn set_ime_0(cpu: &mut Cpu) {
    todo!()
}

pub fn set_ime_1(cpu: &mut Cpu) {
    todo!()
}

pub fn cpl(cpu: &mut Cpu) {
    let a = cpu.get_r8::<A>();
    cpu.set_r8::<A>(!a);

    cpu.flags.set_flag(Flag::Subtract, true);
    cpu.flags.set_flag(Flag::HalfCarry, true);
}

pub fn scf(cpu: &mut Cpu) {
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, true);
}

pub fn ccf(cpu: &mut Cpu) {
    let c = cpu.flags.get_flag(Flag::Carry);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, !c);
}

pub fn daa(cpu: &mut Cpu) {
    let mut a = cpu.get_r8::<A>();
    let mut adjust = 0;
    let mut carry = false;

    let n = cpu.flags.get_flag(Flag::Subtract);
    let h = cpu.flags.get_flag(Flag::HalfCarry);
    let c = cpu.flags.get_flag(Flag::Carry);

    if h || (!n && (a & 0x0F) > 0x09) {
        adjust |= 0x06;
    }

    if c || (!n && a > 0x99) {
        adjust |= 0x60;
        carry = true;
    }

    if n {
        a = a.wrapping_sub(adjust);
    } else {
        a = a.wrapping_add(adjust);
    }

    cpu.set_r8::<A>(a);
    cpu.flags.set_flag(Flag::Zero, a == 0);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, carry);
}

pub fn rlca(cpu: &mut Cpu) {
    let a = cpu.get_r8::<A>();
    let bit7 = (a & 0x80) >> 7;
    let result = (a << 1) | bit7;

    cpu.set_r8::<A>(result);
    cpu.flags.set_flag(Flag::Zero, false);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit7 == 1);
}

pub fn rrca(cpu: &mut Cpu) {
    let a = cpu.get_r8::<A>();
    let bit0 = a & 0x01;
    let result = (a >> 1) | (bit0 << 7);

    cpu.set_r8::<A>(result);
    cpu.flags.set_flag(Flag::Zero, false);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit0 == 1);
}

pub fn rla(cpu: &mut Cpu) {
    let a = cpu.get_r8::<A>();
    let old_carry = if cpu.flags.get_flag(Flag::Carry) {
        1
    } else {
        0
    };
    let bit7 = (a & 0x80) >> 7;
    let result = (a << 1) | old_carry;

    cpu.set_r8::<A>(result);
    cpu.flags.set_flag(Flag::Zero, false);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit7 == 1);
}

pub fn rra(cpu: &mut Cpu) {
    let a = cpu.get_r8::<A>();
    let old_carry = if cpu.flags.get_flag(Flag::Carry) {
        1
    } else {
        0
    };
    let bit0 = a & 0x01;
    let result = (a >> 1) | (old_carry << 7);

    cpu.set_r8::<A>(result);
    cpu.flags.set_flag(Flag::Zero, false);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit0 == 1);
}

pub fn rlc<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let bit7 = (val & 0x80) >> 7;
    let result = (val << 1) | bit7;

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit7 == 1);
}

pub fn write_rlc_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    rlc::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_rrc_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    rrc::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_rl_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    rl::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_rr_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    rr::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_sla_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    sla::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_sra_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    sra::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn rl<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let old_carry = if cpu.flags.get_flag(Flag::Carry) {
        1
    } else {
        0
    };
    let bit7 = (val & 0x80) >> 7;
    let result = (val << 1) | old_carry;

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit7 == 1);
}

pub fn rrc<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let bit0 = val & 0x01;
    let result = (val >> 1) | (bit0 << 7);

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit0 == 1);
}

pub fn rr<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let old_carry = if cpu.flags.get_flag(Flag::Carry) {
        1
    } else {
        0
    };
    let bit0 = val & 0x01;
    let result = (val >> 1) | (old_carry << 7);

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit0 == 1);
}

pub fn sla<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let bit7 = (val & 0x80) >> 7;
    let result = val << 1;

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit7 == 1);
}

pub fn sra<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let bit0 = val & 0x01;
    let bit7 = val & 0x80;
    let result = (val >> 1) | bit7;

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit0 == 1);
}

pub fn swap<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let result = (val >> 4) | (val << 4);

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, false);
}

pub fn srl<Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let bit0 = val & 0x01;
    let result = val >> 1;

    cpu.set_r8::<Reg>(result);
    cpu.flags.set_flag(Flag::Zero, result == 0);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, false);
    cpu.flags.set_flag(Flag::Carry, bit0 == 1);
}


pub fn bit<const B: u8, Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let is_bit_zero = (val & (1 << B)) == 0;

    cpu.flags.set_flag(Flag::Zero, is_bit_zero);
    cpu.flags.set_flag(Flag::Subtract, false);
    cpu.flags.set_flag(Flag::HalfCarry, true);
}

pub fn res<const B: u8, Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let result = val & !(1 << B);
    cpu.set_r8::<Reg>(result);
}

pub fn set<const B: u8, Reg: Reg8>(cpu: &mut Cpu) {
    let val = cpu.get_r8::<Reg>();
    let result = val | (1 << B);
    cpu.set_r8::<Reg>(result);
}


pub fn write_swap_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    swap::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_srl_mem<Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    srl::<Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_res_mem<const B: u8, Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    res::<B, Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

pub fn write_set_mem<const B: u8, Addr: Reg16, Reg: Reg8>(cpu: &mut Cpu) {
    set::<B, Reg>(cpu);
    write_memory::<HL, Reg>(cpu);
}

// pub fn increment_r16<Reg: Reg16>(cpu: &mut Cpu) {
//     cpu.set_r16::<Reg>(cpu.get_r16::<Reg>().wrapping_add(1));
// }
