use std::fs::copy;

use crate::flags::FlagsOps;
use crate::{Cpu};
use crate::R8;
use crate::defines::Flag;

#[test]
pub fn test_cpu_and_register_new() {
    //Cpu::new() calls registers::new()
    let cpu = Cpu::new(vec![0x0]);

    assert!(cpu.instructions_list == [0x0]);
    assert!(cpu.queue == []);
    assert!(cpu.op_index == 0);
    //Cpu's registers
    assert!(cpu.registers.r8 == [0, 0, 0, 0, 0 ,0 ,0 ,0]);
    assert!(cpu.registers.pc == 0 && cpu.registers.sp == 0 && cpu.registers.flags == 0)

}

#[test]
pub fn test_cpu_tick() {
    //Cpu::new() calls registers::new()
    let instru = vec![0x80, 0x80, 0x80, 0x80, 0x80]; // Each 0x80 will push 3 ADD A_B in the queue, so we tick 15 times
    let mut cpu = Cpu::new(instru.clone());

    cpu.registers.r8[R8::B as usize] = 5;
    for _ in 1..15 {
        println!("{:?}", cpu);
        cpu.tick();
    }
    println!("{:?}", cpu);

    assert!(cpu.registers.r8[R8::A as usize] == 70); // 0 + (15 * 5) => 65
    assert!(cpu.registers.pc as usize == 4) // 0 counts as an instruction
}

#[test]
#[should_panic]
pub fn test_cpu_tick_invalid_opcode() {
    let mut cpu = Cpu::new(vec![0xFF]); //Unk

    cpu.registers.r8[R8::B as usize] = 5;
    for _ in 1..10 {
        cpu.tick();
        println!("{:?}", cpu);
    }
}

#[test]
#[should_panic]
pub fn test_cpu_tick_couldnt_fetch_instruction() {
    let mut cpu = Cpu::new(vec![0x80]);

    cpu.registers.r8[R8::B as usize] = 5;
    for _ in 1..10 { //Will panic at 2 because only 1 instruction
        cpu.tick();
        println!("{:?}", cpu);
    }
}

#[test]
#[should_panic]
pub fn test_cpu_tick_nope() {
    let mut cpu = Cpu::new(vec![0x0]);

    cpu.registers.r8[R8::B as usize] = 5;
    for _ in 1..10 { 
        cpu.tick();
        println!("{:?}", cpu);
    }
}



// #[test]
// pub fn test_cpu_queue() {}