use eframe::egui;

use crate::app::FreshikiApp;
use crate::export::{ExportRow, to_csv, to_json};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Csv,
    Json,
}

pub struct ExportDialog {
    pub deck_id: Option<i64>,
    pub path: String,
    pub format: ExportFormat,
    pub msg: Option<String>,
}

impl ExportDialog {
    pub fn new(deck_id: Option<i64>, name: &str) -> Self {
        ExportDialog {
            deck_id,
            path: format!("{name}.csv"),
            format: ExportFormat::Csv,
            msg: None,
        }
    }
}

pub fn show(app: &mut FreshikiApp, ui: &mut egui::Ui) {
    let Some(dialog) = app.export.take() else {
        return;
    };
    let mut dialog = dialog;
    let mut close = false;
    egui::Window::new("Export").show(ui.ctx(), |ui| {
        ui.label("Destination file");
        ui.text_edit_singleline(&mut dialog.path);
        ui.horizontal(|ui| {
            ui.radio_value(&mut dialog.format, ExportFormat::Csv, "CSV");
            ui.radio_value(&mut dialog.format, ExportFormat::Json, "JSON");
        });
        if let Some(msg) = &dialog.msg {
            ui.colored_label(egui::Color32::RED, msg);
        }
        ui.horizontal(|ui| {
            if ui.button("Export").clicked() {
                let path = dialog.path.trim().to_string();
                if path.is_empty() {
                    dialog.msg = Some("Enter a destination path".to_string());
                } else {
                    let deck_id = dialog.deck_id;
                    let format = dialog.format;
                    let result: Result<(), String> = app
                        .db
                        .export_rows(deck_id)
                        .map_err(|e| e.to_string())
                        .and_then(|rows| write_export(&rows, &path, format));
                    match result {
                        Ok(()) => close = true,
                        Err(msg) => dialog.msg = Some(msg),
                    }
                }
            }
            if ui.button("Cancel").clicked() {
                close = true;
            }
        });
    });
    if !close {
        app.export = Some(dialog);
    }
}

fn write_export(rows: &[ExportRow], path: &str, format: ExportFormat) -> Result<(), String> {
    let content = match format {
        ExportFormat::Csv => to_csv(rows),
        ExportFormat::Json => {
            serde_json::to_string_pretty(&to_json(rows)).map_err(|e| e.to_string())?
        }
    };
    std::fs::write(path, content).map_err(|e| e.to_string())
}
