use crate::ppu::colors_palette::ColorType;

#[derive(Debug, Copy)]
pub struct Pixel<C: ColorType> {
    color: C,
    priority: bool,
}

impl<C: ColorType> Pixel<C> {
    pub fn new_bg(color: C) -> Self {
        let priority = false;
        Self::new_obj(color, priority)
    }

    pub fn new_obj(color: C, priority: bool) -> Self {
        Pixel { color, priority }
    }

    pub fn get_color(&self) -> &C {
        &self.color
    }

    pub fn get_color_value(&self) -> u16 {
        self.color.value()
    }

    pub fn get_priority(&self) -> bool {
        self.priority
    }
}

impl<C: ColorType + Copy> Default for Pixel<C> {
    fn default() -> Self {
        Pixel {
            color: ColorType::new(0),
            priority: false,
        }
    }
}

impl<C: ColorType + Clone> Clone for Pixel<C> {
    fn clone(&self) -> Self {
        Pixel {
            color: self.color.clone(),
            priority: self.priority,
        }
    }
}
