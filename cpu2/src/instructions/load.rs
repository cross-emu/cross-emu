use std::fs::read;

use lcov::report::section::Value;

use crate::Cpu;
use crate::defines::{r8, r16};
use crate::implemenation::{PC, Reg8, Reg16};

pub fn load_sp_in_accu(cpu: &mut Cpu) {
    cpu.accumulator
        .accumulate_u8(cpu.bus[cpu.registers.get(R16::SP) as usize]);
    cpu.registers.incr_pc();
}

pub fn write_n_in_accu(cpu: &mut Cpu) {
    cpu.accumulator
        .accumulate_u8(cpu.bus[cpu.registers.get(R16::HL) as usize]);
}

pub fn load_pc_in_accu(cpu: &mut Cpu) {
    cpu.accumulator
        .accumulate_u8(cpu.bus[cpu.registers.get(R16::PC) as usize]);
    cpu.registers.incr_pc();
}

pub fn load_mem_in_accu(cpu: &mut Cpu) {
    let value = cpu.bus[cpu.accumulator.get_u16_at(0) as usize];
    cpu.accumulator.reset();
    cpu.accumulator.accumulate_u8(value);
}

pub fn write_a_in_c0x_ff(cpu: &mut Cpu) {
    cpu.bus[0xFF00 + cpu.registers.get(R8::C) as usize] = cpu.registers.get(R8::A);
}

pub fn load_mem0x_ff_in_accu(cpu: &mut Cpu) {
    let value = cpu.bus[0xFF00 + cpu.accumulator.get_u8_at(0) as usize];
    cpu.accumulator.reset();
    cpu.accumulator.accumulate_u8(value);
}

pub fn read_memory_from_sp(cpu: &mut Cpu) {
    let value = cpu.bus[cpu.registers.get(R16::SP) as usize];
    cpu.accumulator.accumulate_u8(value);
    sp_incr(cpu);
}

pub fn load_r8_r8<Dest: Reg8, Src: Reg8>(cpu: &mut Cpu) {
    cpu.set_r8::<Dest>(cpu.get_r8::<Src>());
}

pub fn read_memory<Addr: Reg16, Dest: Reg8>(cpu: &mut Cpu) {
    if Addr::USIZE == PC::USIZE {
        cpu.set_r16::<PC>(cpu.get_r16::<PC>().wrapping_add(1))
    };
    cpu.set_r8::<Dest>(cpu.bus[cpu.get_r16::<Addr>() as usize]);
}

pub fn read_memory_0xff<Lsb: Reg8, Dest: Reg8>(cpu: &mut Cpu) {
    let addr: u16 = (0xFF as u16) << 8 | cpu.get_r8::<Lsb>() as u16;
    cpu.set_r8::<Dest>(cpu.bus[addr as usize]);
}

pub fn write_memory_0xff<Lsb: Reg8, Value: Reg8>(cpu: &mut Cpu) {
    let addr: u16 = (0xFF as u16) << 8 | cpu.get_r8::<Lsb>() as u16;
    cpu.bus[addr as usize] = cpu.get_r8::<Value>();
}

pub fn write_memory<Addr: Reg16, Value: Reg8>(cpu: &mut Cpu) {
    cpu.bus[cpu.get_r8::<Value>() as usize];
}

pub fn load_r16_r16<Dest: Reg16, Src: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Dest>(cpu.get_r16::<Src>());
}
