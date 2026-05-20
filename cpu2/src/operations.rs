use crate::defines::Instruction;
use crate::ops::add_a_b;
use crate::defines::MicroOp;

//We build that shit so that we can just define in INSTRUCTIONS what instructions are implemented
//But it'll eventually get deleted once everything is done since we won't need to build an array
pub static DISPATCH: [Option<&'static [MicroOp]>; 256] = build_dispatch();


pub static INSTRUCTIONS: &[Instruction] = &[
    Instruction { opcode: 0x00, micro_ops: &[] },
    Instruction { opcode: 0x80, micro_ops: &[add_a_b, add_a_b, add_a_b] },
    Instruction { opcode: 0x41, micro_ops: &[add_a_b] },
    Instruction { opcode: 0x14, micro_ops: &[add_a_b] },
];


#[coverage(off)]
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

