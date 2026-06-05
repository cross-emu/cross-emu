use crate::defines::Cpu;
use crate::implemenation::A;
mod defines;
mod flags;
mod implemenation;
mod instructions;
mod operations;

fn main() {
    let instr: Vec<u8> = vec![0x80, 0x80, 0x80, 0x80, 0x80];
    let mut cpu = Cpu::new(instr);

    cpu.set_r8::<A>(5);
    for _ in 1..6 {
        cpu.tick();
        println!("{:?}", cpu);
    }
}
