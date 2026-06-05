use crate::defines::Instruction;
use crate::defines::MicroOp;
use crate::implemenation::A;
use crate::implemenation::B;
use crate::implemenation::BC;
use crate::implemenation::C;
use crate::implemenation::D;
use crate::implemenation::DE;
use crate::implemenation::E;
use crate::implemenation::H;
use crate::implemenation::HL;
use crate::implemenation::L;
use crate::implemenation::PC;
use crate::implemenation::W;
use crate::implemenation::WZ;
use crate::implemenation::Z;
use crate::instructions;
use crate::instructions::add::add_r8_r8;
use crate::instructions::load::load_r8_r8;

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
        micro_ops: &[add_r8_r8::<A, B>],
    },
    Instruction {
        opcode: 0x81,
        micro_ops: &[add_r8_r8::<A, C>],
    },
    Instruction {
        opcode: 0x82,
        micro_ops: &[add_r8_r8::<A, D>],
    },
    Instruction {
        opcode: 0x83,
        micro_ops: &[add_r8_r8::<A, E>],
    },
    Instruction {
        opcode: 0x84,
        micro_ops: &[add_r8_r8::<A, H>],
    },
    Instruction {
        opcode: 0x85,
        micro_ops: &[add_r8_r8::<A, L>],
    },
    Instruction {
        opcode: 0x86,
        micro_ops: &[instructions::other::noop],
    },
    //LD (HL), n
    Instruction {
        opcode: 0x36,
        micro_ops: &[
            instructions::load::read_memory::<PC, Z>,
            instructions::load::write_memory::<HL, Z>,
            instructions::other::noop,
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
            instructions::load::read_memory::<PC, Z>,
            instructions::load::read_memory::<PC, W>,
            instructions::load::read_memory::<WZ, Z>,
            instructions::load::load_r8_r8::<A, Z>,
        ],
    },
    //LD (nn), A
    Instruction {
        opcode: 0xea,
        micro_ops: &[
            instructions::load::read_memory::<PC, Z>,
            instructions::load::read_memory::<PC, W>,
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
            instructions::load::load_pc_in_accu,
            instructions::load::load_mem0x_ff_in_accu,
            instructions::load_a::load_a_accu,
        ],
    },
    //LD A, (HL-)
    Instruction {
        opcode: 0x3A,
        micro_ops: &[
            instructions::load_r16::load_tmp_hl_decr,
            instructions::load_a::load_a_accu,
        ],
    },
    //LD (HL-), A
    Instruction {
        opcode: 0x32,
        micro_ops: &[
            instructions::load_r16::write_a_in_mem_decr,
            instructions::other::noop,
        ],
    },
    //LD A, (HL+)
    Instruction {
        opcode: 0x2A,
        micro_ops: &[
            instructions::load_r16::load_tmp_hl_incr,
            instructions::load_a::load_a_accu,
        ],
    },
    //LD (HL+), A
    Instruction {
        opcode: 0x22,
        micro_ops: &[
            instructions::load_r16::write_a_in_mem_incr,
            instructions::other::noop,
        ],
    },
    //LD BC, NN
    Instruction {
        opcode: 0x01,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_pc_in_accu,
            instructions::load_r16::write_tmp_in_bc,
        ],
    },
    //LD DE, NN
    Instruction {
        opcode: 0x11,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_pc_in_accu,
            instructions::load_r16::write_tmp_in_de,
        ],
    },
    //LD HL, NN
    Instruction {
        opcode: 0x21,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_pc_in_accu,
            instructions::load_r16::write_tmp_in_hl,
        ],
    },
    //LD SP, NN
    Instruction {
        opcode: 0x31,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_pc_in_accu,
            instructions::load_r16::write_tmp_in_sp,
        ],
    },
    //LD NN, SP
    Instruction {
        opcode: 0x08,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load::load_pc_in_accu,
            instructions::load_r16::write_lsb_sp_in_mem,
            instructions::load_r16::write_msb_sp_in_mem,
        ],
    },
    //LD SP, HL
    Instruction {
        opcode: 0xF9,
        micro_ops: &[
            instructions::load_r16::load_hl_in_sp,
            instructions::other::noop,
        ],
    },
    //PUSH, rr
    Instruction {
        opcode: 0xC5,
        micro_ops: &[
            instructions::other::decremenent_sp,
            instructions::load_r16::write_msb_bc_in_mem,
            instructions::load_r16::write_lsb_bc_in_mem,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xD5,
        micro_ops: &[
            instructions::other::decremenent_sp,
            instructions::load_r16::write_msb_de_in_mem,
            instructions::load_r16::write_lsb_de_in_mem,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xE5,
        micro_ops: &[
            instructions::other::decremenent_sp,
            instructions::load_r16::write_msb_hl_in_mem,
            instructions::load_r16::write_lsb_hl_in_mem,
            instructions::other::noop,
        ],
    },
    Instruction {
        opcode: 0xF5,
        micro_ops: &[
            instructions::other::decremenent_sp,
            instructions::load_r16::write_msb_af_in_mem,
            instructions::load_r16::write_lsb_af_in_mem,
            instructions::other::noop,
        ],
    },
    //POP, rr
    Instruction {
        opcode: 0xC1,
        micro_ops: &[
            instructions::load::read_memory_from_sp,
            instructions::load::read_memory_from_sp,
            instructions::load_r16::write_tmp_in_bc,
        ],
    },
    Instruction {
        opcode: 0xD1,
        micro_ops: &[
            instructions::load::read_memory_from_sp,
            instructions::load::read_memory_from_sp,
            instructions::load_r16::write_tmp_in_de,
        ],
    },
    Instruction {
        opcode: 0xE1,
        micro_ops: &[
            instructions::load::read_memory_from_sp,
            instructions::load::read_memory_from_sp,
            instructions::load_r16::write_tmp_in_hl,
        ],
    },
    Instruction {
        opcode: 0xF1,
        micro_ops: &[
            instructions::load::read_memory_from_sp,
            instructions::load::read_memory_from_sp,
            instructions::load_r16::write_tmp_in_af,
        ],
    },
    //LD HL,SP+e
    Instruction {
        opcode: 0xF8,
        micro_ops: &[
            instructions::load::load_pc_in_accu,
            instructions::load_r16::put_spe_in_h,
            instructions::load_r16::write_tmp_in_hl,
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
            instructions::load::read_mem_in_accu_from_pc,
            instructions::load::load_accu_in_r8::<B>,
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
            instructions::load::read_mem_in_accu_from_pc,
            instructions::load::load_accu_in_r8::<C>,
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
            instructions::load::read_mem_in_accu_from_pc,
            instructions::load::load_accu_in_r8::<D>,
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
            instructions::load::read_mem_in_accu_from_pc,
            instructions::load::load_accu_in_r8::<E>,
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
            instructions::load::read_mem_in_accu_from_pc,
            instructions::load::load_accu_in_r8::<H>,
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
            instructions::load::read_mem_in_accu_from_pc,
            instructions::load::load_accu_in_r8::<L>,
        ],
    },
    Instruction {
        opcode: 0x6f,
        micro_ops: &[instructions::load::load_r8_r8::<L, A>],
    },
    //LD HL, r
    Instruction {
        opcode: 0x70,
        micro_ops: &[instructions::load_b::load_hl_b, instructions::other::noop],
    },
    Instruction {
        opcode: 0x71,
        micro_ops: &[instructions::load_c::load_hl_c, instructions::other::noop],
    },
    Instruction {
        opcode: 0x72,
        micro_ops: &[instructions::load_d::load_hl_d, instructions::other::noop],
    },
    Instruction {
        opcode: 0x73,
        micro_ops: &[instructions::load_e::load_hl_e, instructions::other::noop],
    },
    Instruction {
        opcode: 0x74,
        micro_ops: &[instructions::load_h::load_hl_h, instructions::other::noop],
    },
    Instruction {
        opcode: 0x75,
        micro_ops: &[instructions::load_l::load_hl_l, instructions::other::noop],
    },
    Instruction {
        opcode: 0x77,
        micro_ops: &[instructions::load_a::load_hl_a, instructions::other::noop],
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
            instructions::load::read_mem_in_accu_from_pc,
            instructions::load::load_accu_in_r8::<A>,
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
