use crate::{Cpu, implemenation::Reg16};

//Some ops effectively use 2 cycles but work on one (i.e. LD (HL), r) so that we put a nothing op so it stills takes two cycles and fetch accordingly
pub fn noop(_cpu: &mut Cpu) {}

pub fn halt(_cpu: &mut Cpu) {
    todo!();
}

pub fn decrement_r16<Reg: Reg16>(cpu: &mut Cpu) {
    cpu.set_r16::<Reg>(cpu.get_r16::<Reg>().wrapping_sub(1));
}

// pub fn increment_r16<Reg: Reg16>(cpu: &mut Cpu) {
//     cpu.set_r16::<Reg>(cpu.get_r16::<Reg>().wrapping_add(1));
// }
