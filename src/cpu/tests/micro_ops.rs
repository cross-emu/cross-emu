#[cfg(test)]
mod tests {
    use crate::cpu::defines::Flag;
    use crate::cpu::flags::FlagsOps;
    use crate::cpu_def::*;
    use crate::gameboy::GameBoy;
    use crate::mmu::GbaMmu;
    use crate::mmu::MemoryMapper;
    use crate::mmu::mbc;
    use crate::mmu::mbc::*;

    // Creates a GameBoy with the given first opcode and pads with NOPs so post-instruction
    // fetch never goes out of bounds. Tests operate on the GameBoy and its inner CPU (`gb.cpu`).
    fn gb<M>(opcode: u8) -> GameBoy<GbaMmu<RomOnly>> {
        let list = vec![opcode];
        let mut gb: GameBoy<GbaMmu<RomOnly>> =
            GameBoy::new(None, list.clone(), None).expect("Failed to create gb");
        let bus = GbaMmu::new(None, list, None).expect("Failed to create bus");
        gb.bus = bus;
        // Pre-fetch the first instruction so tests can call `tick()` immediately.
        gb.cpu.set_r16::<PC>(0x0);
        gb
    }

    // Sets the immediate byte at PC=1 (after the opcode is fetched at PC=0).
    fn cpu_n<M>(opcode: u8, n: u8) -> GameBoy<GbaMmu<mbc::RomOnly>> {
        let mut c = gb::<M>(opcode);
        c.bus.write_byte(1, n);
        c
    }

    // Sets the 16-bit immediate at PC=1 (lo) and PC=2 (hi).
    fn cpu_nn<M>(opcode: u8, lo: u8, hi: u8) -> GameBoy<GbaMmu<mbc::RomOnly>> {
        let mut c = gb::<M>(opcode);
        c.bus.write_byte(1, lo);
        c.bus.write_byte(2, hi);
        c
    }

    fn ticks(gb: &mut GameBoy<GbaMmu<mbc::RomOnly>>, n: usize) {
        for _ in 0..n {
            gb.cpu.tick(&mut gb.bus);
        }
    }


    #[test]
    fn op_80_add_a_b() {
        let mut c = gb::<GbaMmu<mbc::RomOnly>>(0x80);
        c.cpu.set_r8::<A>(5);
        c.cpu.set_r8::<B>(3);
        ticks(&mut c, 1);
        assert_eq!(c.cpu.get_r8::<A>(), 8);
        assert!(!c.cpu.flags.get_flag(Flag::Zero));
        assert!(!c.cpu.flags.get_flag(Flag::Subtract));
        assert!(!c.cpu.flags.get_flag(Flag::Carry));
    }

}