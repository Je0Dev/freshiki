use eframe::egui;
use eframe::egui::Key;

use crate::app::FreshikiApp;
use crate::keymap::{Action, KeyBindings};

pub fn show(app: &mut FreshikiApp, ui: &mut egui::Ui) {
    ui.heading("Settings");
    ui.separator();
    ui.label("Keyboard shortcuts");
    for action in Action::ALL {
        ui.horizontal(|ui| {
            ui.label(action.label());
            if app.remapping == Some(action) {
                ui.strong("Press a key...");
                capture_key(app, ui, action);
            } else {
                ui.label(app.bindings.key(action).name());
                if ui.small_button("Change").clicked() {
                    app.remapping = Some(action);
                    app.remap_error = None;
                }
            }
        });
    }
    if ui.button("Reset defaults").clicked() {
        app.bindings = KeyBindings::defaults();
        let _ = app.db.save_bindings(&app.bindings);
    }
    if let Some(err) = &app.remap_error {
        ui.colored_label(egui::Color32::RED, err);
    }
}

fn capture_key(app: &mut FreshikiApp, ui: &mut egui::Ui, action: Action) {
    let Some(key) = pressed_key(ui) else {
        return;
    };
    if key == Key::Escape {
        app.remapping = None;
        return;
    }
    match app.bindings.bind(action, key) {
        Ok(()) => {
            let _ = app.db.save_bindings(&app.bindings);
            app.remapping = None;
            app.remap_error = None;
        }
        Err(msg) => app.remap_error = Some(msg),
    }
}

fn pressed_key(ui: &egui::Ui) -> Option<Key> {
    ui.ctx().input(|i| {
        i.events.iter().find_map(|event| match event {
            egui::Event::Key {
                key, pressed: true, ..
            } => Some(*key),
            _ => None,
        })
    })
}
