use eframe::egui;

use crate::app::FreshikiApp;
use crate::keymap::Action;
use crate::model::now;
use crate::srs::{Grade, review};

pub fn show(app: &mut FreshikiApp, ui: &mut egui::Ui) {
    let flip = app.bindings.key(Action::Flip);
    let next = app.bindings.key(Action::Next);
    let prev = app.bindings.key(Action::Previous);
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

    handle_keys(state, ui, flip, next, prev);

    let card = state.cards[state.index].clone();
    ui.vertical_centered(|ui| {
        ui.heading("Study");
        ui.label(format!("{}/{}", state.index + 1, state.cards.len()));
        ui.add_space(12.0);
        ui.group(|ui| {
            ui.set_min_height(180.0);
            crate::ui::media::render_field(&app.db, &mut app.media_cache, ui, &card.front);
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
                crate::ui::media::render_field(&app.db, &mut app.media_cache, ui, &card.back);
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

fn handle_keys(
    state: &mut crate::state::StudyState,
    ui: &egui::Ui,
    flip: egui::Key,
    next: egui::Key,
    prev: egui::Key,
) {
    let ctx = ui.ctx();
    if ctx.input(|i| i.key_pressed(flip)) {
        state.flipped = !state.flipped;
    }
    if ctx.input(|i| i.key_pressed(next)) && state.index + 1 < state.cards.len() {
        state.index += 1;
        state.flipped = false;
    }
    if ctx.input(|i| i.key_pressed(prev)) && state.index > 0 {
        state.index -= 1;
        state.flipped = false;
    }
}
