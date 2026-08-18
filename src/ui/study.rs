use eframe::egui;

use crate::app::FreshikiApp;
use crate::model::now;
use crate::srs::{Grade, review};

pub fn show(app: &mut FreshikiApp, ui: &mut egui::Ui) {
    let Some(state) = &mut app.study else {
        ui.label("No study session.");
        return;
    };

    if state.cards.is_empty() {
        ui.label("No cards due right now. Great job!");
        return;
    }
    if state.index >= state.cards.len() {
        ui.label("Session complete! All due cards reviewed.");
        return;
    }

    let card = state.cards[state.index].clone();
    ui.vertical_centered(|ui| {
        ui.heading("Study");
        ui.label(format!("{}/{}", state.index + 1, state.cards.len()));
        ui.add_space(12.0);
        ui.group(|ui| {
            ui.set_min_height(180.0);
            ui.label(&card.front);
            if !state.flipped {
                ui.add_space(8.0);
                if ui.button("Show Answer").clicked() {
                    state.flipped = true;
                }
            }
        });
        if state.flipped {
            ui.add_space(8.0);
            ui.group(|ui| {
                ui.set_min_height(120.0);
                ui.label(&card.back);
            });
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                for grade in [Grade::Again, Grade::Hard, Grade::Good, Grade::Easy] {
                    if ui.button(grade.label()).clicked() {
                        let updated = review(&card, grade, now());
                        let _ = app.db.save_review(&updated);
                        state.index += 1;
                        state.flipped = false;
                    }
                }
            });
        }
    });
}
