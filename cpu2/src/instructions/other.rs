use crate::{Cpu, implemenation::PC, implemenation::SP};

//Some ops effectively use 2 cycles but work on one (i.e. LD (HL), r) so that we put a nothing op so it stills takes two cycles and fetch accordingly
pub fn noop(_cpu: &mut Cpu) {}

pub fn halt(_cpu: &mut Cpu) {
    todo!();
}

pub fn decremenent_sp(cpu: &mut Cpu) {
    cpu.set_r16::<SP>(cpu.get_r16::<SP>().wrapping_add(1));
}

pub fn decremenent_pc(cpu: &mut Cpu) {
    cpu.set_r16::<PC>(cpu.get_r16::<PC>().wrapping_add(1));
}

pub fn increment_sp(cpu: &mut Cpu) {
    cpu.set_r16::<SP>(cpu.get_r16::<SP>().wrapping_add(1));
}

pub fn increment_pc(cpu: &mut Cpu) {
    cpu.set_r16::<PC>(cpu.get_r16::<PC>().wrapping_add(1));
}
