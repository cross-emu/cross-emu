
use crate::communications::GameCT;
use crate::ppu::lcd_control::LcdControl;
use crate::ppu::{ObjectManager, PixelProcessor, Ppu, PpuMode};
use crate::ppu::colors_palette::{ColorType, DmgColor};
use crate::ppu::lcd_status::LcdStatus;
use crate::ppu::pixel::Pixel;
use crate::ppu::pixel_fetcher::PFetcher;
use crate::ppu::vram::{DmgVram, Vram};

impl<P: PFetcher<DmgVram, DmgColor>, O: ObjectManager> PixelProcessor
    for Ppu<DmgVram, P, O, DmgColor>
{
    fn read_wram_value(&mut self, addr: u16) -> u8 {
        self.wram.read(addr)
    }

    fn write_wram_value(&mut self, addr: u16, value: u8) {
        self.wram.write(addr, value)
    }

    fn lcd_status(&self) -> &LcdStatus {
        &self.lcd_status
    }
    fn tick(&mut self, ct: &mut Box<dyn GameCT>, boot_enable: bool) {
        self.boot_enable = boot_enable;
        self.check_lyc_equals_ly();

        if !self.read_lcdc().is_ppu_enabled() {
            self.reset_when_ppu_disabled();
            return;
        }

        if !self.lcd_was_enabled {
            self.is_first_scanline_after_lcd_on = true;
            self.lcd_was_enabled = true;
            self.frame_blanked = true;
        }

        self.dots += 1;

        if self.wy == self.ly {
            self.wy_equal_ly_condition_met = true;
        }

        match self.lcd_status.get_ppu_mode() {
            PpuMode::OamSearch => self.mode_oam_search(),
            PpuMode::PixelTransfer => self.mode_pixel_transfer(ct),
            PpuMode::HBlank => self.mode_hblank(),
            PpuMode::VBlank => self.mode_vblank(),
        };

        self.evaluate_stat_interrupt();
    }

    fn mode_oam_search(&mut self) {
        if self.dots == 1 {
            self.oam.set_accessed_oam_row(0);
            self.oam_scan_index = 0;
            self.visible_sprites_count = 0;
            self.visible_sprites = [None; 10];
            self.current_obj_height = if self.read_lcdc().is_obj_size_8x16() {
                16
            } else {
                8
            };
        }

        if self.dots.is_multiple_of(2) && self.oam_scan_index < 40 {
            let sprite = self.oam.sprite(self.oam_scan_index);

            if sprite.is_visible(self.ly, self.current_obj_height)
                && self.visible_sprites_count < 10
            {
                sprite.oam_index = self.oam_scan_index;
                let visible_sprites_count = self.visible_sprites_count;
                self.visible_sprites[visible_sprites_count as usize] = Some(*sprite);
                self.visible_sprites_count += 1;
            }
            self.oam_scan_index += 1;
        }

        if self.dots.is_multiple_of(4) {
            self.oam.update_accessed_oam_row(8);
        }

        if self.dots >= OAM_DOTS {
            let sorted = self.sort_sprites_by_x();
            self.visible_sprites = [None; 10];

            for (i, sprite) in sorted.into_iter().enumerate() {
                self.visible_sprites[i] = Some(sprite);
            }

            self.update_ppu_mode(PpuMode::PixelTransfer);
            self.oam.set_accessed_oam_row(0xFF);
        }
    }

    fn step_oam_fetcher(&mut self) {
        let height: u8 = if LcdControl::from_byte(self.lcdc_byte).is_obj_size_8x16() {
            16
        } else {
            8
        };

        if self.fetching_sprite {
            if let Some(index) = self.current_sprite_to_fetch
                && let Some(sprite) = self.visible_sprites[index]
            {
                self.fetching_sprite = !self.oam_fetcher.tick(
                    &self.vram,
                    &sprite,
                    &mut self.obj_piso,
                    self.ly,
                    height,
                    self.x,
                    self.obp0,
                    self.obp1,
                    false,
                );

                if !self.fetching_sprite {
                    self.visible_sprites[index] = None;

                    let remaining_pixels = self.bg_fifo.len() as u8;
                    if remaining_pixels < 6 {
                        self.stall_dots = 6 - remaining_pixels;
                    }
                }
            };
        } else {
            if !LcdControl::from_byte(self.lcdc_byte).is_obj_enabled() {
                return;
            }

            for (index, sprite_opt) in self.visible_sprites.iter_mut().enumerate() {
                if let Some(sprite) = sprite_opt
                    && sprite.x as usize <= self.x + 8
                {
                    self.current_sprite_to_fetch = Some(index);
                    self.pixel_fetcher.reset_to_state_1();

                    self.fetching_sprite = !self.oam_fetcher.tick(
                        &self.vram,
                        sprite,
                        &mut self.obj_piso,
                        self.ly,
                        height,
                        self.x,
                        self.obp0,
                        self.obp1,
                        false,
                    );

                    if !self.fetching_sprite {
                        *sprite_opt = None;
                    }

                    break;
                }
            }
        }
    }
    fn handle_window_switch(&mut self, use_window: bool) {
        if !self.use_window && use_window {
            self.pixel_fetcher.reset_for_window();
            self.bg_fifo.clear();
            let wx = self.wx;
            self.wx_at_window_start = wx;
            self.pixels_to_discard = 0;
        }

        self.use_window = use_window;

        if self.use_window
            && self.wx != self.wx_at_window_start
            && self.x + 7 >= self.wx as usize
            && !self.is_wx_glitch_happened
        {
            let glitched_pixel = Pixel::new_bg(
                DmgColor::apply_background_palette_bgp(0, self.bgp),
                false,
                0,
            );
            self.bg_fifo.push(glitched_pixel);
            self.is_wx_glitch_happened = true;
        }
    }

    fn push_pixel_to_screen(&mut self, ct: &mut Box<dyn GameCT>) {
        if let Some(bg_pixel) = self.bg_fifo.pop() {
            if self.pixels_to_discard > 0 {
                self.pixels_to_discard -= 1;
            } else {
                let obj_pixel = self.obj_piso.shift_out();

                let bg_color_index: u8;
                let bg_color: DmgColor;

                if !self.read_lcdc().is_bg_window_enabled() {
                    bg_color_index = 0;
                    bg_color = DmgColor::apply_background_palette_bgp(0, self.bgp);
                } else {
                    bg_color_index = bg_pixel.get_color_base_index();
                    bg_color = *bg_pixel.get_color();
                }

                let obj_color_index = obj_pixel.get_color_base_index();

                let mut final_color = if obj_color_index == 0 {
                    bg_color
                } else {
                    let priority = obj_pixel.get_priority();

                    if priority && bg_color_index != 0 {
                        bg_color
                    } else {
                        *obj_pixel.get_color()
                    }
                };

                let ly = self.ly as usize;
                let offset = ly * WIN_SIZE_X + self.x;

                final_color = if self.frame_blanked {
                    LCD_OFF_COLOR_DMG
                } else {
                    final_color
                };
                ct.put_pixel_to_frame(offset, final_color.rgb());
                let x = self.x;
                self.x = x + 1;
            }
        }
    }

    fn mode_pixel_transfer(&mut self, ct: &mut Box<dyn GameCT>) {
        if self.ly < WIN_SIZE_Y as u8 {
            let wx = self.wx;

            let use_window = self.read_lcdc().is_window_enabled()
                && self.wy_equal_ly_condition_met
                && (self.x + 7 >= wx as usize);

            self.step_oam_fetcher();

            if !self.fetching_sprite {
                self.step_pixel_fetcher(use_window);
                if self.stall_dots > 0 {
                    self.stall_dots -= 1;
                } else {
                    self.handle_window_switch(use_window);
                    self.push_pixel_to_screen(ct);
                }
            }
        }

        if self.x == 160 {
            self.update_ppu_mode(PpuMode::HBlank);
        }
    }

    fn new(compatibility: bool) -> Self
    where
        Self: Sized,
    {
        Self::new(compatibility)
    }

    fn read_vram(&self, addr: u16) -> u8 {
        self.read_vram(addr)
    }

    fn read_register(&self, addr: u16) -> u8 {
        self.read_register(addr)
    }

    fn write_vram(&mut self, addr: u16, val: u8) {
        self.write_vram(addr, val)
    }

    fn write_register(&mut self, addr: u16, val: u8) {
        self.write_register(addr, val);
    }

    fn read_oam(&mut self, addr: u16) -> u8 {
        self.read_oam(addr)
    }

    fn write_oam(&mut self, addr: u16, value: u8) {
        self.write_oam(addr, value);
    }

    fn pending_vblank(&self) -> bool {
        self.pending_vblank()
    }

    fn set_pending_vblank(&mut self, value: bool) {
        self.set_pending_vblank(value)
    }

    fn pending_stat(&self) -> bool {
        self.pending_stat()
    }

    fn set_pending_stat(&mut self, value: bool) {
        self.set_pending_stat(value)
    }
    fn hdma1(&self) -> u8 {
        0
    }
    fn hdma2(&self) -> u8 {
        0
    }
    fn hdma3(&self) -> u8 {
        0
    }
    fn hdma4(&self) -> u8 {
        0
    }
    fn write_hdma_value(&mut self, _addr: u16, _value: u8) {
        todo!()
    }
}
