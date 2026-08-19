#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod db;
mod db_cards;
mod db_export;
mod db_media;
mod db_settings;
mod export;
mod frame;
mod keymap;
mod media;
mod model;
mod srs;
mod state;
mod ui;

use eframe::egui;

use crate::app::FreshikiApp;
use crate::db::Db;

fn main() -> eframe::Result {
    let db_path = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("freshiki")
        .join("app.db");
    if let Some(parent) = db_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    crate::media::ensure_media_dirs();
    let db = Db::open(&db_path).expect("failed to open database");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Freshiki",
        options,
        Box::new(|_cc| Ok(Box::new(FreshikiApp::new(db)))),
    )
}
