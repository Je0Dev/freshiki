use eframe::egui;

use crate::app::FreshikiApp;
use crate::state::EditorState;

pub fn show(app: &mut FreshikiApp, ui: &mut egui::Ui, deck_id: i64) {
    let needs_sync = app.editor.as_ref().map(|e| e.deck_id) != Some(deck_id);
    if needs_sync {
        let cards = app.db.list_cards(deck_id).unwrap_or_default();
        app.editor = Some(EditorState::new(deck_id, cards));
    }

    let Some(state) = &mut app.editor else {
        return;
    };
    let deck_name = app
        .decks
        .iter()
        .find(|d| d.id == deck_id)
        .map(|d| d.name.clone())
        .unwrap_or_default();
    ui.heading(format!("Editing: {deck_name}"));
    ui.separator();

    let mut edit: Option<i64> = None;
    let mut delete: Option<i64> = None;
    egui::ScrollArea::vertical()
        .max_height(220.0)
        .show(ui, |ui| {
            for c in &state.cards {
                ui.horizontal(|ui| {
                    ui.label(&c.front);
                    if ui.small_button("Edit").clicked() {
                        edit = Some(c.id);
                    }
                    if ui.small_button("Delete").clicked() {
                        delete = Some(c.id);
                    }
                });
            }
        });
    if let Some(id) = edit
        && let Some(c) = state.cards.iter().find(|c| c.id == id).cloned()
    {
        state.load_form(&c);
    }
    if let Some(id) = delete {
        let _ = app.db.delete_card(id);
        state.reload(&app.db);
    }

    ui.separator();
    ui.label("Front");
    ui.text_edit_multiline(&mut state.front);
    ui.label("Back");
    ui.text_edit_multiline(&mut state.back);
    ui.horizontal(|ui| {
        if state.editing.is_some() {
            if ui.button("Save").clicked() {
                state.save(&app.db);
            }
            if ui.button("Cancel").clicked() {
                state.cancel();
            }
        } else if ui.button("Add Card").clicked() {
            state.add(&app.db);
        }
    });
}
