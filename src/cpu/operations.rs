use crate::cpu::defines::{Cpu, Instruction};
use crate::cpu::instructions::cond::*;
use crate::cpu_def::*;
use crate::mmu::MemoryMapper;
use std::marker::PhantomData;

pub struct InstrListe<M> {
    a: PhantomData<M>,
}

impl<M: MemoryMapper + 'a> InstrListe<M> {
    pub fn prout() -> &'static [Instruction<M>] {
        &Self::LALALALA
    }

    pub const LALALALA: [Instruction<'_, M>; 245] = [
        Instruction {
            opcode: 0x00,
            micro_ops: &[Cpu::noop],
        },
        Instruction {
            opcode: 0x10, //TaBONNE GROSSE DARONNE LA PUTE,
            micro_ops: &[Cpu::stop],
        },
        Instruction {
            opcode: 0x80,
            micro_ops: &[Cpu::add_r8_r8::<A, B>],
        },
        Instruction {
            opcode: 0x81,
            micro_ops: &[Cpu::add_r8_r8::<A, C>],
        },
        Instruction {
            opcode: 0x82,
            micro_ops: &[Cpu::add_r8_r8::<A, D>],
        },
        Instruction {
            opcode: 0x83,
            micro_ops: &[Cpu::add_r8_r8::<A, E>],
        },
        Instruction {
            opcode: 0x84,
            micro_ops: &[Cpu::add_r8_r8::<A, H>],
        },
        Instruction {
            opcode: 0x85,
            micro_ops: &[Cpu::add_r8_r8::<A, L>],
        },
        Instruction {
            opcode: 0x86,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::add_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0x87,
            micro_ops: &[Cpu::add_r8_r8::<A, A>],
        },
        Instruction {
            opcode: 0xC6,
            micro_ops: &[Cpu::read_memory_incr::<PC, Z>, Cpu::add_r8_r8::<A, Z>],
        },
        //LD (HL), n
        Instruction {
            opcode: 0x36,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::write_memory::<HL, Z>,
                Cpu::noop,
            ],
        },
        //ADC B
        Instruction {
            opcode: 0x88,
            micro_ops: &[Cpu::add_r8_r8_with_carry::<A, B>],
        },
        //ADC C
        Instruction {
            opcode: 0x89,
            micro_ops: &[Cpu::add_r8_r8_with_carry::<A, C>],
        },
        //Sub A, B
        Instruction {
            opcode: 0x90,
            micro_ops: &[Cpu::sub_r8_r8::<A, B>],
        },
        //Sub A, C
        Instruction {
            opcode: 0x91,
            micro_ops: &[Cpu::sub_r8_r8::<A, C>],
        },
        //Sub A, D
        Instruction {
            opcode: 0x92,
            micro_ops: &[Cpu::sub_r8_r8::<A, D>],
        },
        //Sub A, E
        Instruction {
            opcode: 0x93,
            micro_ops: &[Cpu::sub_r8_r8::<A, D>],
        },
        //Sub A, H
        Instruction {
            opcode: 0x94,
            micro_ops: &[Cpu::sub_r8_r8::<A, H>],
        },
        //Sub A, L
        Instruction {
            opcode: 0x95,
            micro_ops: &[Cpu::sub_r8_r8::<A, L>],
        },
        //Sub A, HL
        Instruction {
            opcode: 0x96,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::sub_r8_r8::<A, Z>],
        },
        //Sub A, A
        Instruction {
            opcode: 0x97,
            micro_ops: &[Cpu::sub_r8_r8::<A, A>],
        },
        //SUB n
        Instruction {
            opcode: 0xD6,
            micro_ops: &[Cpu::read_memory_incr::<PC, Z>, Cpu::sub_r8_r8::<A, Z>],
        },
        //                              SBC
        //SBC A, B
        Instruction {
            opcode: 0x98,
            micro_ops: &[Cpu::sub_r8_r8_with_carry::<A, B>],
        },
        //SBC A, C
        Instruction {
            opcode: 0x99,
            micro_ops: &[Cpu::sub_r8_r8_with_carry::<A, C>],
        },
        //SBC A, D
        Instruction {
            opcode: 0x9A,
            micro_ops: &[Cpu::sub_r8_r8_with_carry::<A, D>],
        },
        //SBC A, E
        Instruction {
            opcode: 0x9B,
            micro_ops: &[Cpu::sub_r8_r8_with_carry::<A, D>],
        },
        //SBC A, H
        Instruction {
            opcode: 0x9C,
            micro_ops: &[Cpu::sub_r8_r8_with_carry::<A, H>],
        },
        //SBC A, L
        Instruction {
            opcode: 0x9D,
            micro_ops: &[Cpu::sub_r8_r8_with_carry::<A, L>],
        },
        //SBC A, HL
        Instruction {
            opcode: 0x9E,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::sub_r8_r8_with_carry::<A, Z>],
        },
        //SCB A, A
        Instruction {
            opcode: 0x9F,
            micro_ops: &[Cpu::sub_r8_r8::<A, A>],
        },
        //SBC n
        Instruction {
            opcode: 0xDE,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::sub_r8_r8_with_carry::<A, Z>,
            ],
        },
        //                      ADC
        // ADC D
        Instruction {
            opcode: 0x8A,
            micro_ops: &[Cpu::add_r8_r8_with_carry::<A, D>],
        },
        //ADC E
        Instruction {
            opcode: 0x8B,
            micro_ops: &[Cpu::add_r8_r8_with_carry::<A, E>],
        },
        //ADC H
        Instruction {
            opcode: 0x8C,
            micro_ops: &[Cpu::add_r8_r8_with_carry::<A, H>],
        },
        //ADC L
        Instruction {
            opcode: 0x8D,
            micro_ops: &[Cpu::add_r8_r8_with_carry::<A, L>],
        },
        //ADC HL
        Instruction {
            opcode: 0x8E,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::add_r8_r8_with_carry::<A, Z>],
        },
        //ADC A
        Instruction {
            opcode: 0x8F,
            micro_ops: &[Cpu::add_r8_r8_with_carry::<A, A>],
        },
        //ADC n
        Instruction {
            opcode: 0xCE,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::add_r8_r8_with_carry::<A, Z>,
            ],
        },
        //                  CP
        //CP A, B
        Instruction {
            opcode: 0xB8,
            micro_ops: &[Cpu::cp_r8_r8::<A, B>],
        },
        //CP A, C
        Instruction {
            opcode: 0xB9,
            micro_ops: &[Cpu::cp_r8_r8::<A, C>],
        },
        //CP A, D
        Instruction {
            opcode: 0xBA,
            micro_ops: &[Cpu::cp_r8_r8::<A, D>],
        },
        //CP A, E
        Instruction {
            opcode: 0xBB,
            micro_ops: &[Cpu::cp_r8_r8::<A, D>],
        },
        //CP A, H
        Instruction {
            opcode: 0xBC,
            micro_ops: &[Cpu::cp_r8_r8::<A, H>],
        },
        //CP A, L
        Instruction {
            opcode: 0xBD,
            micro_ops: &[Cpu::cp_r8_r8::<A, L>],
        },
        //CP A, HL
        Instruction {
            opcode: 0xBE,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::cp_r8_r8::<A, Z>],
        },
        //CP A, A
        Instruction {
            opcode: 0xBF,
            micro_ops: &[Cpu::cp_r8_r8::<A, A>],
        },
        //CP n
        Instruction {
            opcode: 0xFE,
            micro_ops: &[Cpu::read_memory_incr::<PC, Z>, Cpu::cp_r8_r8::<A, Z>],
        },
        //                      INC
        //INC B
        Instruction {
            opcode: 0x04,
            micro_ops: &[Cpu::inc_r8::<B>],
        },
        //INC, C
        Instruction {
            opcode: 0x0C,
            micro_ops: &[Cpu::inc_r8::<C>],
        },
        //INC, D
        Instruction {
            opcode: 0x14,
            micro_ops: &[Cpu::inc_r8::<D>],
        },
        //INC, E
        Instruction {
            opcode: 0x1C,
            micro_ops: &[Cpu::inc_r8::<E>],
        },
        //INC, H
        Instruction {
            opcode: 0x24,
            micro_ops: &[Cpu::inc_r8::<H>],
        },
        //INC, L
        Instruction {
            opcode: 0x2C,
            micro_ops: &[Cpu::inc_r8::<L>],
        },
        //INC, HL
        Instruction {
            opcode: 0x34,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::inc_addr::<HL, Z>, Cpu::noop],
        },
        //INC A, A
        Instruction {
            opcode: 0x3C,
            micro_ops: &[Cpu::inc_r8::<A>],
        },
        //                      DEC
        //DEC B
        Instruction {
            opcode: 0x05,
            micro_ops: &[Cpu::dec_r8::<B>],
        },
        //DEC, C
        Instruction {
            opcode: 0x0D,
            micro_ops: &[Cpu::dec_r8::<C>],
        },
        //DEC, D
        Instruction {
            opcode: 0x15,
            micro_ops: &[Cpu::dec_r8::<D>],
        },
        //DEC, E
        Instruction {
            opcode: 0x1D,
            micro_ops: &[Cpu::dec_r8::<E>],
        },
        //DEC, H
        Instruction {
            opcode: 0x25,
            micro_ops: &[Cpu::dec_r8::<H>],
        },
        //DEC, L
        Instruction {
            opcode: 0x2D,
            micro_ops: &[Cpu::dec_r8::<L>],
        },
        //DEC, HL
        Instruction {
            opcode: 0x35,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::dec_addr::<HL, Z>, Cpu::noop],
        },
        //DEC A, A
        Instruction {
            opcode: 0x3D,
            micro_ops: &[Cpu::dec_r8::<A>],
        },
        //                      AND
        //AND A, B
        Instruction {
            opcode: 0xA0,
            micro_ops: &[Cpu::and_r8_r8::<A, B>],
        },
        //AND A, C
        Instruction {
            opcode: 0xA1,
            micro_ops: &[Cpu::and_r8_r8::<A, C>],
        },
        //AND A, D
        Instruction {
            opcode: 0xA2,
            micro_ops: &[Cpu::and_r8_r8::<A, D>],
        },
        //AND A, E
        Instruction {
            opcode: 0xA3,
            micro_ops: &[Cpu::and_r8_r8::<A, D>],
        },
        //AND A, H
        Instruction {
            opcode: 0xA4,
            micro_ops: &[Cpu::and_r8_r8::<A, H>],
        },
        //AND A, L
        Instruction {
            opcode: 0xA5,
            micro_ops: &[Cpu::and_r8_r8::<A, L>],
        },
        //AND A, HL
        Instruction {
            opcode: 0xA6,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::and_r8_r8::<A, Z>],
        },
        //AND A, A
        Instruction {
            opcode: 0xA7,
            micro_ops: &[Cpu::and_r8_r8::<A, A>],
        },
        //AND n
        Instruction {
            opcode: 0xE6,
            micro_ops: &[Cpu::read_memory_incr::<PC, Z>, Cpu::and_r8_r8::<A, Z>],
        },
        //                      OR
        //OR A, B
        Instruction {
            opcode: 0xB0,
            micro_ops: &[Cpu::or_r8_r8::<A, B>],
        },
        //OR A, C
        Instruction {
            opcode: 0xB1,
            micro_ops: &[Cpu::or_r8_r8::<A, C>],
        },
        //OR A, D
        Instruction {
            opcode: 0xB2,
            micro_ops: &[Cpu::or_r8_r8::<A, D>],
        },
        //OR A, E
        Instruction {
            opcode: 0xB3,
            micro_ops: &[Cpu::or_r8_r8::<A, D>],
        },
        //OR A, H
        Instruction {
            opcode: 0xB4,
            micro_ops: &[Cpu::or_r8_r8::<A, H>],
        },
        //OR A, L
        Instruction {
            opcode: 0xB5,
            micro_ops: &[Cpu::or_r8_r8::<A, L>],
        },
        //OR A, HL
        Instruction {
            opcode: 0xB6,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::or_r8_r8::<A, Z>],
        },
        //OR A, A
        Instruction {
            opcode: 0xB7,
            micro_ops: &[Cpu::or_r8_r8::<A, A>],
        },
        //OR n
        Instruction {
            opcode: 0xF6,
            micro_ops: &[Cpu::read_memory_incr::<PC, Z>, Cpu::xor_r8_r8::<A, Z>],
        },
        //                      XOR
        //XOR A, B
        Instruction {
            opcode: 0xA8,
            micro_ops: &[Cpu::xor_r8_r8::<A, B>],
        },
        //XOR A, C
        Instruction {
            opcode: 0xA9,
            micro_ops: &[Cpu::xor_r8_r8::<A, C>],
        },
        //XOR A, D
        Instruction {
            opcode: 0xAA,
            micro_ops: &[Cpu::xor_r8_r8::<A, D>],
        },
        //XOR A, E
        Instruction {
            opcode: 0xAB,
            micro_ops: &[Cpu::xor_r8_r8::<A, D>],
        },
        //XOR A, H
        Instruction {
            opcode: 0xAC,
            micro_ops: &[Cpu::xor_r8_r8::<A, H>],
        },
        //XOR A, L
        Instruction {
            opcode: 0xAD,
            micro_ops: &[Cpu::xor_r8_r8::<A, L>],
        },
        //XOR A, HL
        Instruction {
            opcode: 0xAE,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::xor_r8_r8::<A, Z>],
        },
        //XOR A, A
        Instruction {
            opcode: 0xAF,
            micro_ops: &[Cpu::xor_r8_r8::<A, A>],
        },
        //XOR n
        Instruction {
            opcode: 0xEE,
            micro_ops: &[Cpu::read_memory_incr::<PC, Z>, Cpu::cp_r8_r8::<A, Z>],
        },
        //                      LD
        //LD A, (BC)
        Instruction {
            opcode: 0x0a,
            micro_ops: &[Cpu::read_memory::<BC, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        //LD A, (DE)
        Instruction {
            opcode: 0x1a,
            micro_ops: &[Cpu::read_memory::<DE, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        //LD (BC), A
        Instruction {
            opcode: 0x02,
            micro_ops: &[Cpu::write_memory::<BC, A>, Cpu::noop],
        },
        //LD (DE), A
        Instruction {
            opcode: 0x12,
            micro_ops: &[Cpu::write_memory::<DE, A>, Cpu::noop],
        },
        //LD A, (nn)
        Instruction {
            opcode: 0xfa,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::read_memory::<WZ, Z>,
                Cpu::load_r8_r8::<A, Z>,
            ],
        },
        //LD (nn), A
        Instruction {
            opcode: 0xea,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::write_memory::<WZ, A>,
                Cpu::noop,
            ],
        },
        //LDH A, (C)
        Instruction {
            opcode: 0xF2,
            micro_ops: &[Cpu::read_memory_0xff::<C, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        //LDH (C), A
        Instruction {
            opcode: 0xE2,
            micro_ops: &[Cpu::write_memory_0xff::<C, Z>, Cpu::noop],
        },
        //LDH A, (n)
        Instruction {
            opcode: 0xF0,
            micro_ops: &[Cpu::read_memory_0xff::<C, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0xE0,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::write_memory_0xff::<A, Z>,
                Cpu::load_r8_r8::<A, Z>,
            ],
        },
        //LD A, (HL-)
        Instruction {
            opcode: 0x3A,
            micro_ops: &[Cpu::read_memory_decr::<HL, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        //LD (HL-), A
        Instruction {
            opcode: 0x32,
            micro_ops: &[Cpu::read_memory_decr::<HL, Z>, Cpu::noop],
        },
        //LD A, (HL+)
        Instruction {
            opcode: 0x2A,
            micro_ops: &[Cpu::read_memory_incr::<HL, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        //LD (HL+), A
        Instruction {
            opcode: 0x22,
            micro_ops: &[Cpu::write_memory_incr::<HL, Z>, Cpu::noop],
        },
        //LD BC, NN
        Instruction {
            opcode: 0x01,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<BC, WZ>,
            ],
        },
        //LD DE, NN
        Instruction {
            opcode: 0x11,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<DE, WZ>,
            ],
        },
        //LD HL, NN
        Instruction {
            opcode: 0x21,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<HL, WZ>,
            ],
        },
        //LD SP, NN
        Instruction {
            opcode: 0x31,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<SP, WZ>,
            ],
        },
        //LD NN, SP
        Instruction {
            opcode: 0x08,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::write_memory_incr::<WZ, P>,
                Cpu::write_memory_incr::<WZ, S>,
                Cpu::noop,
            ],
        },
        //LD SP, HL
        Instruction {
            opcode: 0xF9,
            micro_ops: &[Cpu::load_r16_r16::<SP, HL>, Cpu::noop],
        },
        //PUSH, rr
        Instruction {
            opcode: 0xC5,
            micro_ops: &[
                Cpu::decrement_r16::<PC>,
                Cpu::write_memory_decr::<SP, B>,
                Cpu::write_memory::<SP, C>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD5,
            micro_ops: &[
                Cpu::decrement_r16::<PC>,
                Cpu::write_memory_decr::<SP, D>,
                Cpu::write_memory::<SP, E>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xE5,
            micro_ops: &[
                Cpu::decrement_r16::<PC>,
                Cpu::write_memory_decr::<SP, H>,
                Cpu::write_memory::<SP, L>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xF5,
            micro_ops: &[
                Cpu::decrement_r16::<PC>,
                Cpu::write_memory_decr::<SP, A>,
                Cpu::write_memory::<SP, F>,
                Cpu::noop,
            ],
        },
        //POP, rr
        Instruction {
            opcode: 0xC1,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<BC, WZ>,
            ],
        },
        Instruction {
            opcode: 0xD1,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<DE, WZ>,
            ],
        },
        Instruction {
            opcode: 0xE1,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<HL, WZ>,
            ],
        },
        Instruction {
            opcode: 0xF1,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<AF, WZ>,
            ],
        },
        //LD HL,SP+e
        Instruction {
            opcode: 0xF8,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::ld_hl_sp_e_low,
                Cpu::ld_hl_sp_e_high,
            ],
        },
        //LD B, r
        Instruction {
            opcode: 0x40,
            micro_ops: &[Cpu::load_r8_r8::<B, B>],
        },
        Instruction {
            opcode: 0x41,
            micro_ops: &[Cpu::load_r8_r8::<B, C>],
        },
        Instruction {
            opcode: 0x42,
            micro_ops: &[Cpu::load_r8_r8::<B, D>],
        },
        Instruction {
            opcode: 0x43,
            micro_ops: &[Cpu::load_r8_r8::<B, E>],
        },
        Instruction {
            opcode: 0x44,
            micro_ops: &[Cpu::load_r8_r8::<B, H>],
        },
        Instruction {
            opcode: 0x45,
            micro_ops: &[Cpu::load_r8_r8::<B, L>],
        },
        Instruction {
            opcode: 0x46,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<B, Z>],
        },
        Instruction {
            opcode: 0x47,
            micro_ops: &[Cpu::load_r8_r8::<B, A>],
        },
        Instruction {
            opcode: 0x06,
            micro_ops: &[Cpu::write_memory_incr::<HL, Z>, Cpu::load_r8_r8::<B, Z>],
        },
        //LD C, r
        Instruction {
            opcode: 0x48,
            micro_ops: &[Cpu::load_r8_r8::<C, B>],
        },
        Instruction {
            opcode: 0x49,
            micro_ops: &[Cpu::load_r8_r8::<C, C>],
        },
        Instruction {
            opcode: 0x4a,
            micro_ops: &[Cpu::load_r8_r8::<C, D>],
        },
        Instruction {
            opcode: 0x4b,
            micro_ops: &[Cpu::load_r8_r8::<C, E>],
        },
        Instruction {
            opcode: 0x4c,
            micro_ops: &[Cpu::load_r8_r8::<C, H>],
        },
        Instruction {
            opcode: 0x4d,
            micro_ops: &[Cpu::load_r8_r8::<C, L>],
        },
        Instruction {
            opcode: 0x4e,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<C, Z>],
        },
        Instruction {
            opcode: 0x4f,
            micro_ops: &[Cpu::load_r8_r8::<C, A>],
        },
        Instruction {
            opcode: 0x0E,
            micro_ops: &[Cpu::write_memory_incr::<HL, Z>, Cpu::load_r8_r8::<C, Z>],
        },
        //LD D, r
        Instruction {
            opcode: 0x50,
            micro_ops: &[Cpu::load_r8_r8::<D, B>],
        },
        Instruction {
            opcode: 0x51,
            micro_ops: &[Cpu::load_r8_r8::<D, C>],
        },
        Instruction {
            opcode: 0x52,
            micro_ops: &[Cpu::load_r8_r8::<D, D>],
        },
        Instruction {
            opcode: 0x53,
            micro_ops: &[Cpu::load_r8_r8::<D, E>],
        },
        Instruction {
            opcode: 0x54,
            micro_ops: &[Cpu::load_r8_r8::<D, H>],
        },
        Instruction {
            opcode: 0x55,
            micro_ops: &[Cpu::load_r8_r8::<D, L>],
        },
        Instruction {
            opcode: 0x56,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<D, Z>],
        },
        Instruction {
            opcode: 0x57,
            micro_ops: &[Cpu::load_r8_r8::<D, A>],
        },
        Instruction {
            opcode: 0x16,
            micro_ops: &[Cpu::write_memory_incr::<HL, Z>, Cpu::load_r8_r8::<D, Z>],
        },
        //LD E, r
        Instruction {
            opcode: 0x58,
            micro_ops: &[Cpu::load_r8_r8::<E, B>],
        },
        Instruction {
            opcode: 0x59,
            micro_ops: &[Cpu::load_r8_r8::<E, C>],
        },
        Instruction {
            opcode: 0x5a,
            micro_ops: &[Cpu::load_r8_r8::<E, D>],
        },
        Instruction {
            opcode: 0x5b,
            micro_ops: &[Cpu::load_r8_r8::<E, E>],
        },
        Instruction {
            opcode: 0x5c,
            micro_ops: &[Cpu::load_r8_r8::<E, H>],
        },
        Instruction {
            opcode: 0x5d,
            micro_ops: &[Cpu::load_r8_r8::<E, L>],
        },
        Instruction {
            opcode: 0x5e,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<E, Z>],
        },
        Instruction {
            opcode: 0x5f,
            micro_ops: &[Cpu::load_r8_r8::<E, A>],
        },
        Instruction {
            opcode: 0x1E,
            micro_ops: &[Cpu::write_memory_incr::<HL, Z>, Cpu::load_r8_r8::<E, Z>],
        },
        //LD H, r
        Instruction {
            opcode: 0x60,
            micro_ops: &[Cpu::load_r8_r8::<H, B>],
        },
        Instruction {
            opcode: 0x61,
            micro_ops: &[Cpu::load_r8_r8::<H, C>],
        },
        Instruction {
            opcode: 0x62,
            micro_ops: &[Cpu::load_r8_r8::<H, D>],
        },
        Instruction {
            opcode: 0x63,
            micro_ops: &[Cpu::load_r8_r8::<H, E>],
        },
        Instruction {
            opcode: 0x64,
            micro_ops: &[Cpu::load_r8_r8::<H, H>],
        },
        Instruction {
            opcode: 0x65,
            micro_ops: &[Cpu::load_r8_r8::<H, L>],
        },
        Instruction {
            opcode: 0x66,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<H, Z>],
        },
        Instruction {
            opcode: 0x67,
            micro_ops: &[Cpu::load_r8_r8::<H, A>],
        },
        Instruction {
            opcode: 0x26,
            micro_ops: &[Cpu::write_memory_incr::<HL, Z>, Cpu::load_r8_r8::<H, Z>],
        },
        //LD L, r
        Instruction {
            opcode: 0x68,
            micro_ops: &[Cpu::load_r8_r8::<L, B>],
        },
        Instruction {
            opcode: 0x69,
            micro_ops: &[Cpu::load_r8_r8::<L, C>],
        },
        Instruction {
            opcode: 0x6a,
            micro_ops: &[Cpu::load_r8_r8::<L, D>],
        },
        Instruction {
            opcode: 0x6b,
            micro_ops: &[Cpu::load_r8_r8::<L, E>],
        },
        Instruction {
            opcode: 0x6c,
            micro_ops: &[Cpu::load_r8_r8::<L, H>],
        },
        Instruction {
            opcode: 0x6d,
            micro_ops: &[Cpu::load_r8_r8::<L, L>],
        },
        Instruction {
            opcode: 0x6e,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<L, Z>],
        },
        Instruction {
            opcode: 0x6f,
            micro_ops: &[Cpu::load_r8_r8::<L, A>],
        },
        Instruction {
            opcode: 0x2E,
            micro_ops: &[Cpu::write_memory_incr::<HL, Z>, Cpu::load_r8_r8::<L, Z>],
        },
        //LD HL, r
        Instruction {
            opcode: 0x70,
            micro_ops: &[Cpu::write_memory::<HL, B>, Cpu::noop],
        },
        Instruction {
            opcode: 0x71,
            micro_ops: &[Cpu::write_memory::<HL, C>, Cpu::noop],
        },
        Instruction {
            opcode: 0x72,
            micro_ops: &[Cpu::write_memory::<HL, D>, Cpu::noop],
        },
        Instruction {
            opcode: 0x73,
            micro_ops: &[Cpu::write_memory::<HL, E>, Cpu::noop],
        },
        Instruction {
            opcode: 0x74,
            micro_ops: &[Cpu::write_memory::<HL, H>, Cpu::noop],
        },
        Instruction {
            opcode: 0x75,
            micro_ops: &[Cpu::write_memory::<HL, L>, Cpu::noop],
        },
        Instruction {
            opcode: 0x77,
            micro_ops: &[Cpu::write_memory::<HL, A>, Cpu::noop],
        },
        //HALT
        Instruction {
            opcode: 0x76,
            micro_ops: &[Cpu::halt],
        },
        //LD A, r
        Instruction {
            opcode: 0x78,
            micro_ops: &[Cpu::load_r8_r8::<A, B>],
        },
        Instruction {
            opcode: 0x79,
            micro_ops: &[Cpu::load_r8_r8::<A, C>],
        },
        Instruction {
            opcode: 0x7a,
            micro_ops: &[Cpu::load_r8_r8::<A, D>],
        },
        Instruction {
            opcode: 0x7b,
            micro_ops: &[Cpu::load_r8_r8::<A, E>],
        },
        Instruction {
            opcode: 0x7c,
            micro_ops: &[Cpu::load_r8_r8::<A, H>],
        },
        Instruction {
            opcode: 0x7d,
            micro_ops: &[Cpu::load_r8_r8::<A, L>],
        },
        Instruction {
            opcode: 0x7e,
            micro_ops: &[Cpu::read_memory::<HL, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0x7f,
            micro_ops: &[Cpu::load_r8_r8::<A, A>],
        },
        Instruction {
            opcode: 0x3E,
            micro_ops: &[Cpu::write_memory_incr::<HL, Z>, Cpu::load_r8_r8::<A, Z>],
        },
        Instruction {
            opcode: 0x3F,
            micro_ops: &[Cpu::ccf],
        },
        Instruction {
            opcode: 0x37,
            micro_ops: &[Cpu::scf],
        },
        Instruction {
            opcode: 0x27,
            micro_ops: &[Cpu::daa],
        },
        Instruction {
            opcode: 0x2F,
            micro_ops: &[Cpu::cpl],
        },
        Instruction {
            opcode: 0x03,
            micro_ops: &[Cpu::inc_r16::<BC>, Cpu::noop],
        },
        Instruction {
            opcode: 0x13,
            micro_ops: &[Cpu::inc_r16::<DE>, Cpu::noop],
        },
        Instruction {
            opcode: 0x23,
            micro_ops: &[Cpu::inc_r16::<HL>, Cpu::noop],
        },
        Instruction {
            opcode: 0x33,
            micro_ops: &[Cpu::inc_r16::<SP>, Cpu::noop],
        },
        Instruction {
            opcode: 0x0B,
            micro_ops: &[Cpu::dec_r16::<BC>, Cpu::noop],
        },
        Instruction {
            opcode: 0x1B,
            micro_ops: &[Cpu::dec_r16::<DE>, Cpu::noop],
        },
        Instruction {
            opcode: 0x2B,
            micro_ops: &[Cpu::dec_r16::<HL>, Cpu::noop],
        },
        Instruction {
            opcode: 0x3B,
            micro_ops: &[Cpu::dec_r16::<SP>, Cpu::noop],
        },
        Instruction {
            opcode: 0x09,
            micro_ops: &[
                Cpu::add_r8_r8_no_zero_flag::<C, L>,
                Cpu::add_r8_r8_with_carry_and_no_zero_flag::<B, H>,
            ],
        },
        Instruction {
            opcode: 0x19,
            micro_ops: &[
                Cpu::add_r8_r8_no_zero_flag::<E, L>,
                Cpu::add_r8_r8_with_carry_and_no_zero_flag::<D, H>,
            ],
        },
        Instruction {
            opcode: 0x29,
            micro_ops: &[
                Cpu::add_r8_r8_no_zero_flag::<L, L>,
                Cpu::add_r8_r8_with_carry_and_no_zero_flag::<H, H>,
            ],
        },
        Instruction {
            opcode: 0x39,
            micro_ops: &[
                Cpu::add_r8_r8_no_zero_flag::<P, L>,
                Cpu::add_r8_r8_with_carry_and_no_zero_flag::<S, H>,
            ],
        },
        Instruction {
            opcode: 0xE8,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::add_hl_sp_e_low,
                Cpu::add_hl_sp_e_high,
                Cpu::load_r16_r16::<SP, WZ>,
            ],
        },
        Instruction {
            opcode: 0x07,
            micro_ops: &[Cpu::rlca],
        },
        Instruction {
            opcode: 0x0F,
            micro_ops: &[Cpu::rrca],
        },
        Instruction {
            opcode: 0x17,
            micro_ops: &[Cpu::rla],
        },
        Instruction {
            opcode: 0x1F,
            micro_ops: &[Cpu::rra],
        },
        Instruction {
            opcode: 0x20,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::check_cond::<CondNZ>,
                Cpu::relative_jump,
            ],
        },
        Instruction {
            opcode: 0x30,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::check_cond::<CondNC>,
                Cpu::relative_jump,
            ],
        },
        Instruction {
            opcode: 0x28,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::check_cond::<CondZ>,
                Cpu::relative_jump,
            ],
        },
        Instruction {
            opcode: 0x38,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::check_cond::<CondC>,
                Cpu::relative_jump,
            ],
        },
        Instruction {
            opcode: 0xCD,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::dec_r16::<SP>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_reassign_pc::<SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD4,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::dec_r16::<SP>,
                Cpu::check_cond::<CondNZ>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_reassign_pc::<SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC4,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::dec_r16::<SP>,
                Cpu::check_cond::<CondNC>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_reassign_pc::<SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xDC,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::dec_r16::<SP>,
                Cpu::check_cond::<CondC>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_reassign_pc::<SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xCC,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::dec_r16::<SP>,
                Cpu::check_cond::<CondZ>,
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_reassign_pc::<SP, PcC>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC3,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xE9,
            micro_ops: &[Cpu::load_r16_r16::<PC, HL>],
        },
        Instruction {
            opcode: 0xC2,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::check_cond::<CondNZ>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD2,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::check_cond::<CondNC>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xCA,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::check_cond::<CondZ>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xDA,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::read_memory_incr::<PC, W>,
                Cpu::check_cond::<CondC>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0x18,
            micro_ops: &[
                Cpu::read_memory_incr::<PC, Z>,
                Cpu::relative_jump,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC9,
            micro_ops: &[
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC0,
            micro_ops: &[
                Cpu::check_cond::<CondNZ>,
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD0,
            micro_ops: &[
                Cpu::check_cond::<CondNC>,
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC8,
            micro_ops: &[
                Cpu::check_cond::<CondZ>,
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD8,
            micro_ops: &[
                Cpu::check_cond::<CondC>,
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD9,
            micro_ops: &[
                Cpu::read_memory_incr::<SP, Z>,
                Cpu::read_memory_incr::<SP, W>,
                Cpu::load_r16_r16_and_ime::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xCF,
            micro_ops: &[
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x08, SP, PcC>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xC7,
            micro_ops: &[
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x00, SP, PcC>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xD7,
            micro_ops: &[
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x10, SP, PcC>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xDF,
            micro_ops: &[
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x18, SP, PcC>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xE7,
            micro_ops: &[
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x20, SP, PcC>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xEF,
            micro_ops: &[
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x28, SP, PcC>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xF7,
            micro_ops: &[
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x30, SP, PcC>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xFF,
            micro_ops: &[
                Cpu::write_memory_decr::<SP, PcP>,
                Cpu::write_memory_rst::<0x38, SP, PcC>,
                Cpu::load_r16_r16::<PC, WZ>,
                Cpu::noop,
            ],
        },
        Instruction {
            opcode: 0xF3,
            micro_ops: &[Cpu::set_ime_0],
        },
        Instruction {
            opcode: 0xFB,
            micro_ops: &[Cpu::set_ime_1],
        },
        Instruction {
            opcode: 0xCB,
            micro_ops: &[Cpu::decode_cb],
        },
    ];
}
