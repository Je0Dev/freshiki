use std::collections::HashMap;

use eframe::egui;

use crate::db::Db;
use crate::keymap::{Action, KeyBindings};
use crate::model::{Deck, now};
use crate::state::{EditorState, SearchState, StudyState};
use crate::ui::export::ExportDialog;

#[derive(Clone, Copy)]
pub enum View {
    Decks,
    Study,
    Editor { deck_id: i64 },
    Search,
    Settings,
}

pub struct FreshikiApp {
    pub db: Db,
    pub view: View,
    pub decks: Vec<Deck>,
    pub study: Option<StudyState>,
    pub editor: Option<EditorState>,
    pub search: SearchState,
    pub new_deck_name: String,
    pub rename_target: Option<i64>,
    pub rename_name: String,
    pub bindings: KeyBindings,
    pub remapping: Option<Action>,
    pub remap_error: Option<String>,
    pub export: Option<ExportDialog>,
    pub media_cache: HashMap<i64, egui::TextureHandle>,
}

impl FreshikiApp {
    pub fn new(db: Db) -> Self {
        let mut app = FreshikiApp {
            db,
            view: View::Decks,
            decks: Vec::new(),
            study: None,
            editor: None,
            search: SearchState::new(),
            new_deck_name: String::new(),
            rename_target: None,
            rename_name: String::new(),
            bindings: KeyBindings::defaults(),
            remapping: None,
            remap_error: None,
            export: None,
            media_cache: HashMap::new(),
        };
        app.bindings = app.db.load_bindings();
        app.refresh_decks();
        app
    }

    pub fn refresh_decks(&mut self) {
        self.decks = self.db.list_decks().unwrap_or_default();
    }

    pub fn start_study(&mut self, deck_id: i64) {
        let cards = self
            .db
            .list_cards(deck_id)
            .unwrap_or_default()
            .into_iter()
            .filter(|c| c.due_at <= now())
            .collect();
        self.study = Some(StudyState::new(cards));
        self.view = View::Study;
    }

    pub fn start_editor(&mut self, deck_id: i64) {
        self.view = View::Editor { deck_id };
    }

    pub fn show_decks(&mut self) {
        self.refresh_decks();
        self.view = View::Decks;
    }

    pub fn show_search(&mut self) {
        self.view = View::Search;
    }

    pub fn show_settings(&mut self) {
        self.remapping = None;
        self.view = View::Settings;
    }

    pub fn open_export(&mut self, deck_id: Option<i64>, name: &str) {
        self.export = Some(ExportDialog::new(deck_id, name));
    }
}
