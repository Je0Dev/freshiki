use eframe::egui;

use crate::app::FreshikiApp;
use crate::keymap::Action;
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
    let mut export_clicked = false;
    ui.horizontal(|ui| {
        ui.heading(format!("Editing: {deck_name}"));
        if ui.button("Export").clicked() {
            export_clicked = true;
        }
    });
    ui.separator();

    let mut edit: Option<i64> = None;
    let mut delete: Option<i64> = None;
    let mut select: Option<i64> = None;
    egui::ScrollArea::vertical()
        .max_height(220.0)
        .show(ui, |ui| {
            for c in &state.cards {
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(state.selected == Some(c.id), &c.front)
                        .clicked()
                    {
                        select = Some(c.id);
                    }
                    if ui.small_button("Edit").clicked() {
                        edit = Some(c.id);
                    }
                    if ui.small_button("Delete").clicked() {
                        delete = Some(c.id);
                    }
                });
            }
        });
    if let Some(id) = select {
        state.selected = Some(id);
    }
    if let Some(id) = edit
        && let Some(c) = state.cards.iter().find(|c| c.id == id).cloned()
    {
        state.load_form(&c);
    }
    if let Some(id) = delete {
        let _ = app.db.delete_card(id);
        if state.selected == Some(id) {
            state.selected = None;
        }
        state.reload(&app.db);
    }

    if state.editing.is_none()
        && !ui.ctx().text_edit_focused()
        && ui
            .ctx()
            .input(|i| i.key_pressed(app.bindings.key(Action::Edit)))
        && let Some(id) = state.selected
        && let Some(c) = state.cards.iter().find(|c| c.id == id).cloned()
    {
        state.load_form(&c);
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

    ui.separator();
    let dropped = ui.input(|i| i.raw.dropped_files.clone());
    let hovering = ui.input(|i| i.raw.hovered_files.clone());
    ui.label("Media folders");
    ui.horizontal(|ui| {
        if ui.button("Open Images").clicked() {
            crate::media::open_folder(&crate::media::images_dir());
        }
        if ui.button("Open Audio").clicked() {
            crate::media::open_folder(&crate::media::audio_dir());
        }
    });
    ui.label("Link by path, or drag & drop a file onto a field below:");
    ui.horizontal(|ui| {
        ui.label("Path:");
        ui.text_edit_singleline(&mut state.media_path);
        if ui.button("to Front").clicked() {
            crate::ui::media::attach(state, &app.db, true);
        }
        if ui.button("to Back").clicked() {
            crate::ui::media::attach(state, &app.db, false);
        }
    });
    let pointer = pointer_pos(ui);
    let front_rect =
        crate::ui::media::drop_zone(ui, "Drop image/audio on Front", !hovering.is_empty());
    for file in &dropped {
        if front_rect.contains(pointer) {
            crate::ui::media::attach_dropped(state, &app.db, true, file.as_ref());
        }
    }
    let back_rect =
        crate::ui::media::drop_zone(ui, "Drop image/audio on Back", !hovering.is_empty());
    for file in &dropped {
        if back_rect.contains(pointer) {
            crate::ui::media::attach_dropped(state, &app.db, false, file.as_ref());
        }
    }
    if let Some(msg) = &state.media_error {
        ui.colored_label(egui::Color32::RED, msg);
    }
    ui.separator();
    ui.label("Preview");
    crate::ui::media::render_field(&app.db, &mut app.media_cache, ui, &state.front);
    crate::ui::media::render_field(&app.db, &mut app.media_cache, ui, &state.back);

    if export_clicked {
        app.open_export(Some(deck_id), &deck_name);
    }
}

fn pointer_pos(ui: &egui::Ui) -> egui::Pos2 {
    ui.ctx()
        .input(|i| i.pointer.hover_pos())
        .unwrap_or(egui::Pos2::NAN)
}
