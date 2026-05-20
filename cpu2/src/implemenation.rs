
use crate::defines::{Cpu, Registers};
use crate::operations::{INSTRUCTIONS, DISPATCH};
use std::fmt;

impl Registers {

    pub fn new() -> Self {

        Registers { 
            r8: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            pc: 0,
            sp : 0,
            flags : 0
         } 

    }
}

impl Cpu {

    pub fn new(test : Vec<u8>) -> Self {
        {
            let first_opcode = test.first().expect("Error in the fetch of the first instruction");
            let first_instr = INSTRUCTIONS.iter()
            .find(|e| e.opcode == *first_opcode)
            .expect("Unknown opcode");
            println!("First fetch");

            Cpu { 
                registers: Registers::new(),
                instructions_list: test.clone(),
                queue: first_instr.micro_ops,
                op_index: 0
            }
        }
    }

    pub fn tick(&mut self) {
        
        if self.op_index < self.queue.len() {
            let micro_op = &self.queue[self.op_index];
            self.op_index += 1;
            micro_op(self);

            if self.op_index == self.queue.len() {

                println!("Fetching...");
                let opcode = self.instructions_list
                    .get(self.registers.pc as usize)
                    .expect("Could not fetch instructions");
                self.registers.pc = self.registers.pc.wrapping_add(1);

                self.queue = DISPATCH[*opcode as usize]
                    .expect("Unknown opcode");

                self.op_index = 0;
            }
        } else {
            panic!("No instruction left!");
        }
    }
}


impl fmt::Debug for Cpu {
   #[coverage(off)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cpu")
         .field("Registers", &self.registers)
         .field("op_index", &self.op_index)
         .field("queue", &self.queue)
         .finish()
    }
}

impl fmt::Debug for Registers {
    #[coverage(off)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Registers")
         .field("r8", &self.r8)
         .field("pc", &self.pc)
         .field("sp", &self.sp)
         .field("flags", &format!("{:08b}", self.flags))
         .finish()
    }
}
