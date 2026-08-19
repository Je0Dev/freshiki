use eframe::egui;

use crate::app::FreshikiApp;
use crate::model::now;

pub fn show(app: &mut FreshikiApp, ui: &mut egui::Ui) {
    ui.heading("Decks");
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut app.new_deck_name);
        if ui.button("Add Deck").clicked() {
            let name = app.new_deck_name.trim().to_string();
            if !name.is_empty() {
                let _ = app.db.create_deck(&name, now());
                app.new_deck_name.clear();
                app.refresh_decks();
            }
        }
    });
    ui.separator();

    let mut study: Option<i64> = None;
    let mut edit: Option<i64> = None;
    let mut rename: Option<i64> = None;
    let mut delete: Option<i64> = None;
    let mut export: Option<i64> = None;
    egui::ScrollArea::vertical().show(ui, |ui| {
        for deck in &app.decks {
            let count = app.db.card_count(deck.id).unwrap_or(0);
            let due = app.db.due_count(deck.id, now()).unwrap_or(0);
            ui.horizontal(|ui| {
                ui.strong(&deck.name);
                ui.label(format!("{count} cards, {due} due"));
                if ui.button("Study").clicked() {
                    study = Some(deck.id);
                }
                if ui.button("Edit").clicked() {
                    edit = Some(deck.id);
                }
                if ui.button("Export").clicked() {
                    export = Some(deck.id);
                }
                if ui.button("Rename").clicked() {
                    rename = Some(deck.id);
                }
                if ui.button("Delete").clicked() {
                    delete = Some(deck.id);
                }
            });
        }
    });

    if let Some(id) = study {
        app.start_study(id);
    }
    if let Some(id) = edit {
        app.start_editor(id);
    }
    if let Some(id) = export {
        let name = app
            .decks
            .iter()
            .find(|d| d.id == id)
            .map(|d| d.name.clone())
            .unwrap_or_default();
        app.open_export(Some(id), &name);
    }
    if let Some(id) = rename {
        app.rename_target = Some(id);
        app.rename_name = app
            .decks
            .iter()
            .find(|d| d.id == id)
            .map(|d| d.name.clone())
            .unwrap_or_default();
    }
    if let Some(id) = delete {
        let _ = app.db.delete_deck(id);
        app.refresh_decks();
    }

    if let Some(id) = app.rename_target {
        egui::Window::new("Rename Deck").show(ui.ctx(), |ui| {
            ui.text_edit_singleline(&mut app.rename_name);
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    let _ = app.db.rename_deck(id, app.rename_name.trim());
                    app.rename_target = None;
                    app.refresh_decks();
                }
                if ui.button("Cancel").clicked() {
                    app.rename_target = None;
                }
            });
        });
    }
}
