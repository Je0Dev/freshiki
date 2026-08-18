use eframe::egui;

use crate::db::Db;
use crate::model::{Deck, now};
use crate::state::{EditorState, SearchState, StudyState};
use crate::ui;

#[derive(Clone, Copy)]
pub enum View {
    Decks,
    Study,
    Editor { deck_id: i64 },
    Search,
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
        };
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
}

impl eframe::App for FreshikiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("topbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Freshiki");
                ui.separator();
                if ui.button("Decks").clicked() {
                    self.show_decks();
                }
                if ui.button("Search").clicked() {
                    self.show_search();
                }
                if matches!(self.view, View::Study | View::Editor { .. })
                    && ui.button("Back to Decks").clicked()
                {
                    self.show_decks();
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui| match self.view {
            View::Decks => ui::decks::show(self, ui),
            View::Study => ui::study::show(self, ui),
            View::Editor { deck_id } => ui::editor::show(self, ui, deck_id),
            View::Search => ui::search::show(self, ui),
        });
    }
}
