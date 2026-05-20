use crate::{Cpu};
use crate::defines::{Flag, R8};
use crate::flags::FlagsOps;


pub fn add_a_b(cpu: &mut Cpu) {
    let a = cpu.registers.r8[R8::A as usize];
    let b = cpu.registers.r8[R8::B as usize];
    let result = a.wrapping_add(b);

    cpu.registers.r8[R8::A as usize] = result;

    cpu.registers.flags.set_flag(Flag::Zero, result == 0);
    cpu.registers.flags.set_flag(Flag::Subtract, false); 
    cpu.registers.flags.set_flag(Flag::HalfCarry,(a & 0x0F) + (b & 0x0F) > 0x0F);
    cpu.registers.flags.set_flag(Flag::Carry, (a as u16) + (b as u16) > 0xFF);

    println!("Executing add_a_b");
} 