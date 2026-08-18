use eframe::egui;

use crate::app::FreshikiApp;
use crate::model::{CardStatus, due_label, now};

pub fn show(app: &mut FreshikiApp, ui: &mut egui::Ui) {
    ui.heading("Search");

    let mut run = false;
    ui.horizontal(|ui| {
        let edit = ui.text_edit_singleline(&mut app.search.query);
        let enter = edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
        run = ui.button("Search").clicked() || enter;
    });

    let mut deck_changed = false;
    let mut status_changed = false;
    let mut selected_deck = app.search.deck_filter;
    egui::ComboBox::from_label("Deck")
        .selected_text(
            match app.decks.iter().find(|d| Some(d.id) == selected_deck) {
                Some(d) => d.name.clone(),
                None => "All decks".to_string(),
            },
        )
        .show_ui(ui, |ui| {
            if ui
                .selectable_value(&mut selected_deck, None, "All decks")
                .changed()
            {
                deck_changed = true;
            }
            for d in &app.decks {
                if ui
                    .selectable_value(&mut selected_deck, Some(d.id), &d.name)
                    .changed()
                {
                    deck_changed = true;
                }
            }
        });
    app.search.deck_filter = selected_deck;

    let mut selected_status = app.search.status_filter;
    egui::ComboBox::from_label("Status")
        .selected_text(match selected_status {
            Some(s) => s.label().to_string(),
            None => "Any".to_string(),
        })
        .show_ui(ui, |ui| {
            if ui
                .selectable_value(&mut selected_status, None, "Any")
                .changed()
            {
                status_changed = true;
            }
            for s in CardStatus::all() {
                if ui
                    .selectable_value(&mut selected_status, Some(s), s.label())
                    .changed()
                {
                    status_changed = true;
                }
            }
        });
    app.search.status_filter = selected_status;

    if run || deck_changed || status_changed {
        app.search.run(&app.db);
    }

    ui.separator();
    if app.search.results.is_empty() {
        ui.label("No results.");
    } else {
        ui.label(format!("{} result(s)", app.search.results.len()));
    }
    egui::ScrollArea::vertical().show(ui, |ui| {
        for c in &app.search.results {
            ui.group(|ui| {
                ui.strong(&c.front);
                ui.separator();
                ui.label(&c.back);
                ui.label(format!(
                    "{} | next: {}",
                    crate::model::status(c, now()).label(),
                    due_label(c.due_at, now())
                ));
            });
        }
    });
}
