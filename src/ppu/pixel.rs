use crate::ppu::colors_palette::ColorType;

#[derive(Debug, Copy)]
pub struct Pixel<C: ColorType> {
    color: C,
    oam_index: u8,
    priority: bool,
}

impl<C: ColorType> Pixel<C> {
    pub fn new_bg(color: C, priority: bool, oam_index: u8) -> Self {
        Self::new_obj(color, priority, oam_index)
    }

    pub fn new_obj(color: C, priority: bool, oam_index: u8) -> Self {
        Pixel {
            color,
            priority,
            oam_index,
        }
    }

    pub fn get_color(&self) -> &C {
        &self.color
    }

    pub fn get_color_base_index(&self) -> u8 {
        self.color.base_index()
    }

    pub fn get_priority(&self) -> bool {
        self.priority
    }
    pub fn get_oam_index(&self) -> u8 {
        self.oam_index
    }
}

impl<C: ColorType + Copy> Default for Pixel<C> {
    fn default() -> Self {
        Pixel {
            color: ColorType::new(0, 0),
            oam_index: 0,
            priority: false,
        }
    }
}

impl<C: ColorType + Clone> Clone for Pixel<C> {
    fn clone(&self) -> Self {
        Pixel {
            color: self.color.clone(),
            oam_index: self.oam_index,
            priority: self.priority,
        }
    }
}
