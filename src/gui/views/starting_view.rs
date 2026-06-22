use eframe::egui::{Direction, Layout};
use crate::gui::{AppState, SelectionDevice, StartingHubDevice};


impl StartingHubDevice {
    pub fn starting_view(
    self,
    ui: &mut egui::Ui,
    _frame: &mut eframe::Frame,
) -> AppState {

    egui::CentralPanel::default()
        .show_inside(ui, |ui| {

            ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
                if ui.button("Start").clicked() {
                    AppState::SelectionHub(SelectionDevice::default())
                } else {
                    AppState::StartingHub(StartingHubDevice {})
                }
            })
            .inner
        })
        .inner
    }
}