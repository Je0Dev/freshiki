use eframe::egui;

use crate::app::{FreshikiApp, View};
use crate::ui;

impl eframe::App for FreshikiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("topbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Freshiki");
                ui.separator();
                if ui.button("Decks").clicked() {
                    self.show_decks();
                }
                if ui.button("Search").clicked() {
                    self.show_search();
                }
                if ui.button("Settings").clicked() {
                    self.show_settings();
                }
                if matches!(self.view, View::Study | View::Editor { .. })
                    && ui.button("Back to Decks").clicked()
                {
                    self.show_decks();
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui| match self.view {
            View::Decks => ui::decks::show(self, ui),
            View::Study => ui::study::show(self, ui),
            View::Editor { deck_id } => ui::editor::show(self, ui, deck_id),
            View::Search => ui::search::show(self, ui),
            View::Settings => ui::settings::show(self, ui),
        });
        ui::export::show(self, ui);
    }
}
