
use crate::defines::{Flag};
use crate::Cpu;
use crate::flags::FlagsOps;


#[test]
pub fn test_flags_initial_state() {
    let instr: Vec<u8> = vec![0x00];
    let cpu = Cpu::new(instr);

    assert!(!cpu.registers.flags.get_flag(Flag::Zero));
    assert!(!cpu.registers.flags.get_flag(Flag::Subtract));
    assert!(!cpu.registers.flags.get_flag(Flag::HalfCarry));
    assert!(!cpu.registers.flags.get_flag(Flag::Carry));
}

#[test]
pub fn test_flags_isolation() {

    let instr: Vec<u8> = vec![0x00];
    let mut cpu = Cpu::new(instr);

    cpu.registers.flags.set_flag(Flag::Zero, true);
    assert!(!cpu.registers.flags.get_flag(Flag::Subtract),  "Zero corrupted Subtract");
    assert!(!cpu.registers.flags.get_flag(Flag::HalfCarry), "Zero corrupted HalfCarry");
    assert!(!cpu.registers.flags.get_flag(Flag::Carry),     "Zero corrupted Carry");

    cpu.registers.flags.set_flag(Flag::Carry, true);
    assert!(cpu.registers.flags.get_flag(Flag::Zero),       "Carry overwrote Zero");
    assert!(!cpu.registers.flags.get_flag(Flag::Subtract),  "Carry corrupted Subtract");
    assert!(!cpu.registers.flags.get_flag(Flag::HalfCarry), "Carry corrupted HalfCarry");
}

#[test]
pub fn test_flags_clear_isolation() {

    let instr: Vec<u8> = vec![0x00];
    let mut cpu = Cpu::new(instr);

    // Turn everything on
    cpu.registers.flags.set_flag(Flag::Zero,      true);
    cpu.registers.flags.set_flag(Flag::Subtract,  true);
    cpu.registers.flags.set_flag(Flag::HalfCarry, true);
    cpu.registers.flags.set_flag(Flag::Carry,     true);

    // Clear only Zero
    cpu.registers.flags.set_flag(Flag::Zero, false);

    assert!(!cpu.registers.flags.get_flag(Flag::Zero),     "Zero should be cleared");
    assert!(cpu.registers.flags.get_flag(Flag::Subtract),  "Subtract was cleared by mistake");
    assert!(cpu.registers.flags.get_flag(Flag::HalfCarry), "HalfCarry was cleared by mistake");
    assert!(cpu.registers.flags.get_flag(Flag::Carry),     "Carry was cleared by mistake");
}

#[test]
pub fn test_flags_idempotent_set_true() {

    let instr: Vec<u8> = vec![0x00];
    let mut cpu = Cpu::new(instr);

    cpu.registers.flags.set_flag(Flag::HalfCarry, true);
    cpu.registers.flags.set_flag(Flag::HalfCarry, true); // intentional duplicate

    assert!(cpu.registers.flags.get_flag(Flag::HalfCarry));
    // All other flags must remain untouched
    assert!(!cpu.registers.flags.get_flag(Flag::Zero));
    assert!(!cpu.registers.flags.get_flag(Flag::Subtract));
    assert!(!cpu.registers.flags.get_flag(Flag::Carry));
}

#[test]
pub fn test_flags_idempotent_set_false() {
    let instr: Vec<u8> = vec![0x00];
    let mut cpu = Cpu::new(instr);

    cpu.registers.flags.set_flag(Flag::Zero, true); // only Zero is active

    cpu.registers.flags.set_flag(Flag::Carry,    false); // Carry already clear
    cpu.registers.flags.set_flag(Flag::Subtract, false); // Subtract already clear

    assert!(cpu.registers.flags.get_flag(Flag::Zero), "Zero was cleared for no reason");
}

#[test]
pub fn test_flags_all_set_then_all_cleared() {
    let instr: Vec<u8> = vec![0x00];
    let mut cpu = Cpu::new(instr);

    let all_flags = [
        Flag::Zero,
        Flag::Subtract,
        Flag::HalfCarry,
        Flag::Carry,
    ];

    for &f in &all_flags {
        cpu.registers.flags.set_flag(f, true);
    }
    for &f in &all_flags {
        assert!(cpu.registers.flags.get_flag(f), "Flag {:?} should be set", f);
    }

    for &f in &all_flags {
        cpu.registers.flags.set_flag(f, false);
    }
    for &f in &all_flags {
        assert!(!cpu.registers.flags.get_flag(f), "Flag {:?} should be clear", f);
    }

    assert_eq!(cpu.registers.flags, 0x00, "F register is not cleanly zeroed out");
}

#[test]
pub fn test_flags_raw_value_consistency() {

    let instr: Vec<u8> = vec![0x00];
    let mut cpu = Cpu::new(instr);

    cpu.registers.flags.set_flag(Flag::Zero, true);
    assert_eq!(cpu.registers.flags & 0b10000000, 0b10000000, "Zero must sit at bit 7");

    cpu.registers.flags.set_flag(Flag::Zero, false);
    cpu.registers.flags.set_flag(Flag::Subtract, true);
    assert_eq!(cpu.registers.flags & 0b01000000, 0b01000000, "Subtract must sit at bit 6");

    cpu.registers.flags.set_flag(Flag::Subtract, false);
    cpu.registers.flags.set_flag(Flag::HalfCarry, true);
    assert_eq!(cpu.registers.flags & 0b00100000, 0b00100000, "HalfCarry must sit at bit 5");

    cpu.registers.flags.set_flag(Flag::HalfCarry, false);
    cpu.registers.flags.set_flag(Flag::Carry, true);
    assert_eq!(cpu.registers.flags & 0b00010000, 0b00010000, "Carry must sit at bit 4");
}

