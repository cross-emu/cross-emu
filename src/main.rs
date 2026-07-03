#![allow(dead_code)]
mod cli;
mod communications;
mod cpu;
mod file;
mod gameboy;
mod gui;
mod mmu;
mod ppu;
mod sound;

use crate::{cli::EmulatorArguments, file::GbmuFile, gui::EmulationAppOptions};
use gui::GraphicalApp;
use std::sync::{LazyLock, Mutex};

static GBMU_FILE: LazyLock<Mutex<GbmuFile>> =
    LazyLock::new(|| Mutex::new(GbmuFile::get_existing_or_new()));

static ROM_COMPTABILITY: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));
static IS_BOOT_ROM_FINISHED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

#[tokio::main]
async fn main() {
    let arguments = match EmulatorArguments::get() {
        Ok(args) => args,
        Err(errors) => {
            eprintln!("Enable to open emulator : {errors}");
            return;
        }
    };

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_min_inner_size([800.0, 600.0])
            .with_resizable(true),
        ..Default::default()
    };

    let app = if let Some(rom_path) = arguments.rom_path {
        let options =
            EmulationAppOptions::new(None, rom_path, arguments.boot_rom, arguments.gb_type);
        GraphicalApp::create_emulation_app(options)
    } else {
        GraphicalApp::default()
    };

    let _ = eframe::run_native("GBMU", options, Box::new(|_cc| Ok(Box::new(app))));
}
