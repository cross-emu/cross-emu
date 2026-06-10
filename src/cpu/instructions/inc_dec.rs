use crate::mmu::MemoryMapper;
use crate::{cpu::defines::Cpu, cpu_def::Reg8, cpu_def::Reg16};

impl<'a, M: MemoryMapper> Cpu<'a, M> {
    pub fn inc_r8<Dest: Reg8>(&mut self, _bus: &mut M) {
        Self::set_r8::<Dest>(self, Self::get_r8::<Dest>(self).wrapping_add(1));
    }

    //Increment the value pointed BY the r16 and update it
    pub fn inc_addr<Addr: Reg16, Value: Reg8>(&mut self, bus: &mut M) {
        Self::inc_r8::<Value>(self, bus);
        Self::write_memory::<Addr, Value>(self, bus);
    }

    pub fn dec_r8<Reg: Reg8>(&mut self, _bus: &mut M) {
        Self::set_r8::<Reg>(self, Self::get_r8::<Reg>(self).wrapping_sub(1));
    }

    //Decrement the value pointed BY the r16 and update it
    pub fn dec_addr<Addr: Reg16, Value: Reg8>(&mut self, bus: &mut M) {
        Self::dec_r8::<Value>(self, bus);
        Self::write_memory::<Addr, Value>(self, bus);
    }

    pub fn inc_r16<Dest: Reg16>(&mut self, _bus: &mut M) {
        Self::set_r16::<Dest>(self, Self::get_r16::<Dest>(self).wrapping_add(1));
    }

    pub fn dec_r16<Dest: Reg16>(&mut self, _bus: &mut M) {
        Self::set_r16::<Dest>(self, Self::get_r16::<Dest>(self).wrapping_sub(1));
    }
}
