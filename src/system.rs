use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::mmu::mbc::Mbc;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HardwareKind {
    Dmg,
    Cgb,
}

pub struct Myppu<S: SystemSpecific> {
    p: PhantomData<S>,
}

pub struct Mymmu<S: SystemSpecific, M: Mbc> {
    p: PhantomData<S>,
    cart: M,
}

pub struct Mytimer<S: SystemSpecific> {
    p: PhantomData<S>,
}

pub struct Mygb<S: SystemSpecific, M: Mbc> {
    p: PhantomData<S>,
    mmu: Mymmu<S, M>,
}

pub trait SystemSpecific where Self: Sized {
    const KIND: HardwareKind;
    
    fn simulate_boot_rom<M: Mbc>(gb: &mut Mygb<Self, M>);

    fn read_vram(ppu: &mut Myppu<Self>, addr: u16) -> u8;
    fn write_vram(ppu: &mut Myppu<Self>, addr: u16, val: u8);

    fn tick_timer(timer: &mut Mytimer<Self>) -> bool;
    fn write_timer<M: Mbc>(mmu: &mut Mymmu<Self, M>);
    fn read_timer<M: Mbc>(mmu: &mut Mymmu<Self, M>);

    fn addr_is_in_boot_rom(addr: u16) -> bool;

    fn set_speed_reg<M: Mbc>(mmu: &mut Mymmu<Self, M>);
    fn speed_switch_is_requested<M: Mbc>(mmu: &mut Mymmu<Self, M>) -> bool;

    fn write_registers(&mut self, addr: u16, value: u8);
    fn read_registers(&mut self, addr: u16) -> u8;

    fn update_sprite_attribute(/* to define*/);

    fn push_pixel(); 

    fn get_tile_data_high() -> u8;
    fn get_tile_data_low() -> u8;
}
