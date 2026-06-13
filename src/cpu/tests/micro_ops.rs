#[cfg(test)]
mod tests {
    use crate::cpu::defines::Flag;
    use crate::cpu::flags::FlagsOps;
    use crate::cpu_def::*;
    use crate::gameboy::GameBoy;
    use crate::mmu::GbaMmu;
    use crate::mmu::mbc;
    use crate::mmu::mbc::*;

    // Creates a GameBoy with the given first opcode and pads with NOPs so post-instruction
    // fetch never goes out of bounds. Tests operate on the GameBoy and its inner CPU (`gb.cpu`).
    fn gb<M>(opcode: u8) -> GameBoy<GbaMmu<RomOnly>> {
        let mut gb: GameBoy<GbaMmu<RomOnly>> =
            GameBoy::new(None, Vec::new(), None).expect("Failed to create gb");
        gb.bus.write_byte(0x8000, opcode);
        gb.cpu.set_r16::<PC>(0x8000);
        gb
    }

    fn ticks(gb: &mut GameBoy<GbaMmu<mbc::RomOnly>>, n: usize) {
        for _ in 0..n {
            gb.cpu.tick(&mut gb.bus);
        }
    }

    #[test]
    fn op_00_noop() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x00);
        c.cpu.first_read(&mut c.bus);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0);
        assert_eq!(c.cpu.get_r8::<B>(), 0);
        assert_eq!(c.cpu.get_r8::<C>(), 0);
        assert_eq!(c.cpu.get_r8::<D>(), 0);
        assert_eq!(c.cpu.get_r8::<E>(), 0);
        assert_eq!(c.cpu.get_r8::<H>(), 0);
        assert_eq!(c.cpu.get_r8::<L>(), 0);
        assert_eq!(c.cpu.get_r16::<SP>(), 0);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8002);
        assert_eq!(c.cpu.get_r16::<AF>(), 0x0000);
        assert_eq!(c.cpu.get_r16::<BC>(), 0x0000);
        assert_eq!(c.cpu.get_r16::<DE>(), 0x0000);
        assert_eq!(c.cpu.get_r16::<HL>(), 0x0000);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

    #[test]
    fn op_01_ld_bc_d16() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x01);
        c.cpu.first_read(&mut c.bus);
        c.bus.write_byte(0x8001, 0x34);
        c.bus.write_byte(0x8002, 0x12);
        
        ticks(&mut c, 3);
        assert_eq!(c.cpu.get_r16::<BC>(), 0x1234);
        assert_eq!(c.cpu.get_r16::<PC>(), 0x8004);
    }

    #[test]
    fn op_02_ld_a_bc() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x02);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0xAB);
        c.cpu.set_r16::<BC>(0x8003);
        ticks(&mut c, 2);
        assert_eq!(c.bus.read_byte(0x8003), c.cpu.get_r8::<A>());
    }

    #[test]
    fn op_03_inc_bc() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x03);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<BC>(0x1234);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r16::<BC>(), 0x1235);
    }

    #[test]
    fn op_04_inc_b_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x04);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<B>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<B>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_04_inc_b_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x04);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<B>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<B>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }


    #[test]
    fn op_0c_inc_c_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x0C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<C>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<C>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_0c_inc_c_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x0C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<C>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<C>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_14_inc_d_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x14);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<D>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<D>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_14_inc_d_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x14);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<D>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<D>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_1c_inc_e_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x1C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<E>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<E>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_1c_inc_e_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x1C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<E>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<E>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_24_inc_h_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x24);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<H>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<H>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_24_inc_h_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x24);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<H>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<H>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_2c_inc_l_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x2C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<L>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<L>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_2c_inc_l_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x2C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<L>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<L>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_3c_inc_a_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x3C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0xFE);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_3c_inc_a_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x3C);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0x0F);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_34_inc_hl_mem_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x34);
        c.cpu.first_read(&mut c.bus);
        
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0x0F); 
        
        ticks(&mut c, 3);
        assert_eq!(c.bus.read_byte(0xC000), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_34_inc_hl_mem_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x34);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0xFE);
        ticks(&mut c, 3);
        assert_eq!(c.bus.read_byte(0xC000), 0xFF);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_05_dec_b_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x05);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<B>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<B>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_05_dec_b_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x05);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<B>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<B>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_0d_dec_c_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x0D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<C>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<C>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_0d_dec_c_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x0D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<C>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<C>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_15_dec_d_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x15);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<D>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<D>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_15_dec_d_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x15);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<D>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<D>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_1d_dec_e_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x1D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<E>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<E>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_1d_dec_e_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x1D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<E>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<E>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_25_dec_h_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x25);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<H>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<H>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_25_dec_h_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x25);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<H>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<H>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_2d_dec_l_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x2D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<L>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<L>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_2d_dec_l_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x2D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<L>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<L>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_3d_dec_a_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x3D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0x11);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0x10);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_3d_dec_a_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x3D);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(0x10);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 0x0F);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_35_dec_hl_mem_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x35);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0xFE);
        ticks(&mut c, 3);
        assert_eq!(c.bus.read_byte(0xC000), 0xFD);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }


    #[test]
    fn op_35_dec_hl_mem_no_half_carry() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x35);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r16::<HL>(0xC000);
        c.bus.write_byte(0xC000, 0x0F);
        ticks(&mut c, 3);
        assert_eq!(c.bus.read_byte(0xC000), 0x0E);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::HalfCarry));
    }

    #[test]
    fn op_80_add_a_b() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x80);
        c.cpu.first_read(&mut c.bus);
        c.cpu.set_r8::<A>(5);
        c.cpu.set_r8::<B>(3);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 8);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }
}
