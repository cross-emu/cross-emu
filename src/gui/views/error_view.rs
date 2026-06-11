use crate::gui::{AppState, ErrorDevice, SelectionDevice};
use eframe::egui;

impl ErrorDevice {
    pub fn error_view(self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) -> AppState {
        let mut go_to_selection = false;

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Error");
                    ui.add_space(16.0);
                    ui.label(&self.formated_error);
                    ui.add_space(24.0);
                    if ui.button("Back to selection").clicked() {
                        go_to_selection = true;
                    }
                });
            });
        });

        if go_to_selection {
            AppState::SelectionHub(SelectionDevice::default())
        } else {
            AppState::Error(self)
        }
    }
}
