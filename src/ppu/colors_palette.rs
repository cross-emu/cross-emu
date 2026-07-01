#![allow(unused_variables)]

use crate::ppu::Cram;

pub trait ColorType {
    fn get_rgb(value: u16) -> [u8; 3];
    fn new(value: u16) -> Self;
    fn apply_background_palette_bgp(color_index: u8, bgp: u8) -> Self;
    fn apply_background_palette_cram(bg_cram: &Cram, bg_attributes: u8, color_index: u8) -> Self;
    fn value(&self) -> u16;
    fn rgb(&self) -> [u8; 3];
}

#[derive(Default, Copy, Clone)]
pub struct DmgColor {
    pub value: u16,
    pub rgb: [u8; 3],
}

#[derive(Default, Copy, Clone)]
pub struct CgbColor {
    pub value: u16,
    pub rgb: [u8; 3],
}

// Use arrays (Copy) so we can return them by value easily.
const WHITE: [u8; 3] = [255, 255, 255];
const LIGHTGRAY: [u8; 3] = [192, 192, 192];
const DARKGRAY: [u8; 3] = [96, 96, 96];
const BLACK: [u8; 3] = [0, 0, 0];

impl ColorType for DmgColor {
    fn new(value: u16) -> Self {
        let rgb = Self::get_rgb(value);
        Self { value, rgb }
    }
    fn get_rgb(value: u16) -> [u8; 3] {
        match value {
            0 => WHITE,
            1 => LIGHTGRAY,
            2 => DARKGRAY,
            3 => BLACK,
            _ => unreachable!(),
        }
    }

    fn apply_background_palette_bgp(color_index: u8, bgp: u8) -> Self {
        let index = (bgp >> (color_index * 2)) & 0b11;
        Self::new(index as u16)
    }
    fn value(&self) -> u16 {
        self.value
    }
    fn rgb(&self) -> [u8; 3] {
        self.rgb
    }

    fn apply_background_palette_cram(bg_cram: &Cram, bg_attributes: u8, color_index: u8) -> Self {
        Self::new(0)
    }
}

impl ColorType for CgbColor {
    fn new(value: u16) -> Self {
        let rgb = Self::get_rgb(value);
        Self { value, rgb }
    }
    fn get_rgb(value: u16) -> [u8; 3] {
        let color = value;
        let mut b = ((color >> 10) & 0x1F) as u8;
        b = (b << 3) | 0b111;
        let mut g = ((color >> 5) & 0x1F) as u8;
        g = (g << 3) | 0b001;
        let mut r = (color & 0x1F) as u8;
        r = (r << 3) | 0b100;
        [r, g, b]
    }

    fn apply_background_palette_cram(
        bg_cram: &Cram,
        palette_index: u8,
        color_index: u8,
    ) -> CgbColor {
        let offset = palette_index as usize * 8 + color_index as usize * 2;
        let low = bg_cram.data[offset];
        let high = bg_cram.data[offset + 1];
        let color_raw = (high as u16) << 8 | low as u16;
        CgbColor::new(color_raw)
    }

    fn apply_background_palette_bgp(color_index: u8, bgp: u8) -> Self {
        Self::new(0)
    }

    fn value(&self) -> u16 {
        self.value
    }
    fn rgb(&self) -> [u8; 3] {
        self.rgb
    }
}
