use crate::defines::Instruction;
use crate::defines::MicroOp;
use crate::implemenation::*;
use crate::instructions;

//We build that shit so that we can just define in INSTRUCTIONS what instructions are implemented
//But it'll eventually get deleted once everything is done since we won't need to build an array
pub static DISPATCH: [Option<&'static [MicroOp]>; 256] = build_dispatch();

pub static INSTRUCTIONS: &[Instruction] = &[
    Instruction {
        opcode: 0x00,
        micro_ops: &[],
    },
    Instruction {
        opcode: 0x80,
        micro_ops: &[instructions::add::add_r8_r8::<A, B>],
    },
    Instruction {
        opcode: 0x81,
        micro_ops: &[instructions::add::add_r8_r8::<A, C>],
    },
    Instruction {
        opcode: 0x82,
        micro_ops: &[instructions::add::add_r8_r8::<A, D>],
    },
    Instruction {
        opcode: 0x83,
        micro_ops: &[instructions::add::add_r8_r8::<A, E>],
    },
    Instruction {
        opcode: 0x84,
        micro_ops: &[instructions::add::add_r8_r8::<A, H>],
    },
    Instruction {
        opcode: 0x85,
        micro_ops: &[instructions::add::add_r8_r8::<A, L>],
    },
    Instruction {
        opcode: 0x86,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::add::add_r8_r8::<A, Z>,
        ],
    },
    Instruction {
        opcode: 0xC6,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::add::add_r8_r8::<A, Z>,
        ],
    },
    //LD (HL), n
    Instruction {
        opcode: 0x36,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::write_memory::<HL, Z>,
            instructions::other::noop,
        ],
    },
    //ADC B
    Instruction {
        opcode: 0x88,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, B>],
    },
    //ADC C
    Instruction {
        opcode: 0x89,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, C>],
    },
    //ADC D
    Instruction {
        opcode: 0x8A,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, D>],
    },
    //ADC E
    Instruction {
        opcode: 0x8B,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, E>],
    },
    //ADC H
    Instruction {
        opcode: 0x8C,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, H>],
    },
    //ADC L
    Instruction {
        opcode: 0x8D,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, L>],
    },
    //ADC HL
    Instruction {
        opcode: 0x8E,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::add::add_r8_r8_with_carry::<A, Z>,
        ],
    },
    //ADC A
    Instruction {
        opcode: 0x8F,
        micro_ops: &[instructions::add::add_r8_r8_with_carry::<A, A>],
    },
    //ADC n
    Instruction {
        opcode: 0xCE,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::add::add_r8_r8_with_carry::<A, Z>,
        ],
    },
    //LD A, (BC)
    Instruction {
        opcode: 0x0a,
        micro_ops: &[
            instructions::load::read_memory::<BC, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LD A, (DE)
    Instruction {
        opcode: 0x1a,
        micro_ops: &[
            instructions::load::read_memory::<DE, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LD (BC), A
    Instruction {
        opcode: 0x02,
        micro_ops: &[
            instructions::load::write_memory::<BC, A>,
            instructions::other::noop,
        ],
    },
    //LD (DE), A
    Instruction {
        opcode: 0x12,
        micro_ops: &[
            instructions::load::write_memory::<DE, A>,
            instructions::other::noop,
        ],
    },
    //LD A, (nn)
    Instruction {
        opcode: 0xfa,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::read_memory::<WZ, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LD (nn), A
    Instruction {
        opcode: 0xea,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::write_memory::<WZ, A>,
            instructions::other::noop,
        ],
    },
    //LDH A, (C)
    Instruction {
        opcode: 0xF2,
        micro_ops: &[
            instructions::load::read_memory_0xff::<C, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LDH (C), A
    Instruction {
        opcode: 0xE2,
        micro_ops: &[
            instructions::load::write_memory_0xff::<C, Z>,
            instructions::other::noop,
        ],
    },
    //LDH A, (n)
    Instruction {
        opcode: 0xF0,
        micro_ops: &[
            instructions::load::read_memory_0xff::<C, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LD A, (HL-)
    Instruction {
        opcode: 0x3A,
        micro_ops: &[
            instructions::load::read_memory_decr::<HL, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LD (HL-), A
    Instruction {
        opcode: 0x32,
        micro_ops: &[
            instructions::load::read_memory_decr::<HL, Z>,
            instructions::other::noop,
        ],
    },
    //LD A, (HL+)
    Instruction {
        opcode: 0x2A,
        micro_ops: &[
            instructions::load::read_memory_incr::<HL, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LD (HL+), A
    Instruction {
        opcode: 0x22,
        micro_ops: &[
            instructions::load::write_memory_incr::<HL, Z>,
            instructions::other::noop,
        ],
    },
    //LD BC, NN
    Instruction {
        opcode: 0x01,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<BC, WZ>,
        ],
    },
    //LD DE, NN
    Instruction {
        opcode: 0x11,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<DE, WZ>,
        ],
    },
    //LD HL, NN
    Instruction {
        opcode: 0x21,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<HL, WZ>,
        ],
    },
    //LD SP, NN
    Instruction {
        opcode: 0x31,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<SP, WZ>,
        ],
    },
    //LD NN, SP
    Instruction {
        opcode: 0x08,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::write_memory_incr::<WZ, P>,
            instructions::load::write_memory_incr::<WZ, S>,
            instructions::other::noop,
        ],
    },
    //LD SP, HL
    Instruction {
        opcode: 0xF9,
        micro_ops: &[
            instructions::load::load_r16_r16::<SP, HL>,
            instructions::other::noop,
        ],
    },
    //PUSH, rr
    Instruction {
        opcode: 0xC5,
        micro_ops: &[
            instructions::other::decrement_r16::<PC>,
            instructions::load::write_memory_decr::<SP, B>,
            instructions::load::write_memory::<SP, C>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xD5,
        micro_ops: &[
            instructions::other::decrement_r16::<PC>,
            instructions::load::write_memory_decr::<SP, D>,
            instructions::load::write_memory::<SP, E>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xE5,
        micro_ops: &[
            instructions::other::decrement_r16::<PC>,
            instructions::load::write_memory_decr::<SP, H>,
            instructions::load::write_memory::<SP, L>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xF5,
        micro_ops: &[
            instructions::other::decrement_r16::<PC>,
            instructions::load::write_memory_decr::<SP, A>,
            instructions::load::write_memory::<SP, F>,
            instructions::other::noop,
        ],
    },
    //POP, rr
    Instruction {
        opcode: 0xC1,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<BC, WZ>,
        ],
    },
    Instruction {
        opcode: 0xD1,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<DE, WZ>,
        ],
    },
    Instruction {
        opcode: 0xE1,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<HL, WZ>,
        ],
    },
    Instruction {
        opcode: 0xF1,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::read_memory_incr::<PC, W>,
            instructions::load::load_r16_r16::<AF, WZ>,
        ],
    },
    //LD HL,SP+e
    Instruction {
        opcode: 0xF8,
        micro_ops: &[
            instructions::load::read_memory_incr::<PC, Z>,
            instructions::load::ld_hl_sp_e_low,
            instructions::load::ld_hl_sp_e_high,
        ],
    },
    //LD B, r
    Instruction {
        opcode: 0x40,
        micro_ops: &[instructions::load::load_r8_r8::<B, B>],
    },
    Instruction {
        opcode: 0x41,
        micro_ops: &[instructions::load::load_r8_r8::<B, C>],
    },
    Instruction {
        opcode: 0x42,
        micro_ops: &[instructions::load::load_r8_r8::<B, D>],
    },
    Instruction {
        opcode: 0x43,
        micro_ops: &[instructions::load::load_r8_r8::<B, E>],
    },
    Instruction {
        opcode: 0x44,
        micro_ops: &[instructions::load::load_r8_r8::<B, H>],
    },
    Instruction {
        opcode: 0x45,
        micro_ops: &[instructions::load::load_r8_r8::<B, L>],
    },
    Instruction {
        opcode: 0x46,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<B, Z>,
        ],
    },
    Instruction {
        opcode: 0x47,
        micro_ops: &[instructions::load::load_r8_r8::<B, A>],
    },
    //LD C, r
    Instruction {
        opcode: 0x48,
        micro_ops: &[instructions::load::load_r8_r8::<C, B>],
    },
    Instruction {
        opcode: 0x49,
        micro_ops: &[instructions::load::load_r8_r8::<C, C>],
    },
    Instruction {
        opcode: 0x4a,
        micro_ops: &[instructions::load::load_r8_r8::<C, D>],
    },
    Instruction {
        opcode: 0x4b,
        micro_ops: &[instructions::load::load_r8_r8::<C, E>],
    },
    Instruction {
        opcode: 0x4c,
        micro_ops: &[instructions::load::load_r8_r8::<C, H>],
    },
    Instruction {
        opcode: 0x4d,
        micro_ops: &[instructions::load::load_r8_r8::<C, L>],
    },
    Instruction {
        opcode: 0x4e,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<C, Z>,
        ],
    },
    Instruction {
        opcode: 0x4f,
        micro_ops: &[instructions::load::load_r8_r8::<C, A>],
    },
    //LD D, r
    Instruction {
        opcode: 0x50,
        micro_ops: &[instructions::load::load_r8_r8::<D, B>],
    },
    Instruction {
        opcode: 0x51,
        micro_ops: &[instructions::load::load_r8_r8::<D, C>],
    },
    Instruction {
        opcode: 0x52,
        micro_ops: &[instructions::load::load_r8_r8::<D, D>],
    },
    Instruction {
        opcode: 0x53,
        micro_ops: &[instructions::load::load_r8_r8::<D, E>],
    },
    Instruction {
        opcode: 0x54,
        micro_ops: &[instructions::load::load_r8_r8::<D, H>],
    },
    Instruction {
        opcode: 0x55,
        micro_ops: &[instructions::load::load_r8_r8::<D, L>],
    },
    Instruction {
        opcode: 0x56,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<D, Z>,
        ],
    },
    Instruction {
        opcode: 0x57,
        micro_ops: &[instructions::load::load_r8_r8::<D, A>],
    },
    //LD E, r
    Instruction {
        opcode: 0x58,
        micro_ops: &[instructions::load::load_r8_r8::<E, B>],
    },
    Instruction {
        opcode: 0x59,
        micro_ops: &[instructions::load::load_r8_r8::<E, C>],
    },
    Instruction {
        opcode: 0x5a,
        micro_ops: &[instructions::load::load_r8_r8::<E, D>],
    },
    Instruction {
        opcode: 0x5b,
        micro_ops: &[instructions::load::load_r8_r8::<E, E>],
    },
    Instruction {
        opcode: 0x5c,
        micro_ops: &[instructions::load::load_r8_r8::<E, H>],
    },
    Instruction {
        opcode: 0x5d,
        micro_ops: &[instructions::load::load_r8_r8::<E, L>],
    },
    Instruction {
        opcode: 0x5e,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<E, Z>,
        ],
    },
    Instruction {
        opcode: 0x5f,
        micro_ops: &[instructions::load::load_r8_r8::<E, A>],
    },
    //LD H, r
    Instruction {
        opcode: 0x60,
        micro_ops: &[instructions::load::load_r8_r8::<H, B>],
    },
    Instruction {
        opcode: 0x61,
        micro_ops: &[instructions::load::load_r8_r8::<H, C>],
    },
    Instruction {
        opcode: 0x62,
        micro_ops: &[instructions::load::load_r8_r8::<H, D>],
    },
    Instruction {
        opcode: 0x63,
        micro_ops: &[instructions::load::load_r8_r8::<H, E>],
    },
    Instruction {
        opcode: 0x64,
        micro_ops: &[instructions::load::load_r8_r8::<H, H>],
    },
    Instruction {
        opcode: 0x65,
        micro_ops: &[instructions::load::load_r8_r8::<H, L>],
    },
    Instruction {
        opcode: 0x66,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<H, Z>,
        ],
    },
    Instruction {
        opcode: 0x67,
        micro_ops: &[instructions::load::load_r8_r8::<H, A>],
    },
    //LD L, r
    Instruction {
        opcode: 0x68,
        micro_ops: &[instructions::load::load_r8_r8::<L, B>],
    },
    Instruction {
        opcode: 0x69,
        micro_ops: &[instructions::load::load_r8_r8::<L, C>],
    },
    Instruction {
        opcode: 0x6a,
        micro_ops: &[instructions::load::load_r8_r8::<L, D>],
    },
    Instruction {
        opcode: 0x6b,
        micro_ops: &[instructions::load::load_r8_r8::<L, E>],
    },
    Instruction {
        opcode: 0x6c,
        micro_ops: &[instructions::load::load_r8_r8::<L, H>],
    },
    Instruction {
        opcode: 0x6d,
        micro_ops: &[instructions::load::load_r8_r8::<L, L>],
    },
    Instruction {
        opcode: 0x6e,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<L, Z>,
        ],
    },
    Instruction {
        opcode: 0x6f,
        micro_ops: &[instructions::load::load_r8_r8::<L, A>],
    },
    //LD HL, r
    Instruction {
        opcode: 0x70,
        micro_ops: &[
            instructions::load::write_memory::<HL, B>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x71,
        micro_ops: &[
            instructions::load::write_memory::<HL, C>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x72,
        micro_ops: &[
            instructions::load::write_memory::<HL, D>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x73,
        micro_ops: &[
            instructions::load::write_memory::<HL, E>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x74,
        micro_ops: &[
            instructions::load::write_memory::<HL, H>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x75,
        micro_ops: &[
            instructions::load::write_memory::<HL, L>,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0x77,
        micro_ops: &[
            instructions::load::write_memory::<HL, A>,
            instructions::other::noop,
        ],
    },
    //HALT
    Instruction {
        opcode: 0x76,
        micro_ops: &[instructions::other::halt],
    },
    //LD A, r
    Instruction {
        opcode: 0x78,
        micro_ops: &[instructions::load::load_r8_r8::<A, B>],
    },
    Instruction {
        opcode: 0x79,
        micro_ops: &[instructions::load::load_r8_r8::<A, C>],
    },
    Instruction {
        opcode: 0x7a,
        micro_ops: &[instructions::load::load_r8_r8::<A, D>],
    },
    Instruction {
        opcode: 0x7b,
        micro_ops: &[instructions::load::load_r8_r8::<A, E>],
    },
    Instruction {
        opcode: 0x7c,
        micro_ops: &[instructions::load::load_r8_r8::<A, H>],
    },
    Instruction {
        opcode: 0x7d,
        micro_ops: &[instructions::load::load_r8_r8::<A, L>],
    },
    Instruction {
        opcode: 0x7e,
        micro_ops: &[
            instructions::load::read_memory::<HL, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    Instruction {
        opcode: 0x7f,
        micro_ops: &[instructions::load::load_r8_r8::<A, A>],
    },
];

const fn build_dispatch() -> [Option<&'static [MicroOp]>; 256] {
    let mut table = [None; 256];
    let mut i = 0;
    while i < INSTRUCTIONS.len() {
        let opcode = INSTRUCTIONS[i].opcode as usize;
        table[opcode] = Some(INSTRUCTIONS[i].micro_ops);
        i += 1;
    }
    table
}
