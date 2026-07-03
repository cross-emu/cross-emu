#[cfg(test)]
mod tests {
    use crate::cpu::cb_instructions::build_cb_instructions;
    use crate::mmu::DmgMmu;
    use crate::mmu::mbc::*;
    use crate::mmu::timers::DmgTimers;
    use crate::ppu::DmgPpu;
    use crate::{cpu::instructions::build_instructions, gameboy::GameBoy};

    pub fn get_new_gb() -> GameBoy<DmgMmu<RomOnly, DmgTimers, DmgPpu>> {
        let gb: GameBoy<DmgMmu<RomOnly, DmgTimers, DmgPpu>> =
            GameBoy::new(None, vec![], None, false).expect("Failed to create gb");
        gb
    }

    #[test]
    fn test_instructions_order_and_completeness() {
        let instructions = build_instructions::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>();

        for (expected_opcode, instruction) in instructions.iter().enumerate() {
            assert_eq!(
                instruction.opcode as usize, expected_opcode,
                "Opcode incorrect à l'index {} : trouvé 0x{:02X}, attendu 0x{:02X}",
                expected_opcode, instruction.opcode, expected_opcode
            );
        }

        assert_eq!(
            instructions.len(),
            256,
            "Nombre d'instructions incorrect : {} trouvée(s), 256 attendues",
            instructions.len()
        );
    }

    #[test]
    fn test_cb_instructions_order_and_completeness() {
        let instructions = build_cb_instructions::<DmgMmu<RomOnly, DmgTimers, DmgPpu>>();

        for (expected_opcode, instruction) in instructions.iter().enumerate() {
            assert_eq!(
                instruction.opcode as usize, expected_opcode,
                "Opcode incorrect à l'index {} : trouvé 0x{:02X}, attendu 0x{:02X}",
                expected_opcode, instruction.opcode, expected_opcode
            );
        }

        assert_eq!(
            instructions.len(),
            256,
            "Nombre d'instructions incorrect : {} trouvée(s), 256 attendues",
            instructions.len()
        );
    }
}
