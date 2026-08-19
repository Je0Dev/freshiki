use crate::db::Db;
use crate::model::{Card, CardStatus, now};

pub struct StudyState {
    pub cards: Vec<Card>,
    pub index: usize,
    pub flipped: bool,
}

impl StudyState {
    pub fn new(cards: Vec<Card>) -> Self {
        StudyState {
            cards,
            index: 0,
            flipped: false,
        }
    }
}

pub struct EditorState {
    pub deck_id: i64,
    pub cards: Vec<Card>,
    pub front: String,
    pub back: String,
    pub editing: Option<i64>,
    pub selected: Option<i64>,
    pub media_path: String,
    pub media_error: Option<String>,
}

impl EditorState {
    pub fn new(deck_id: i64, cards: Vec<Card>) -> Self {
        EditorState {
            deck_id,
            cards,
            front: String::new(),
            back: String::new(),
            editing: None,
            selected: None,
            media_path: String::new(),
            media_error: None,
        }
    }

    pub fn load_form(&mut self, c: &Card) {
        self.front = c.front.clone();
        self.back = c.back.clone();
        self.editing = Some(c.id);
        self.selected = Some(c.id);
    }

    pub fn reload(&mut self, db: &Db) {
        self.cards = db.list_cards(self.deck_id).unwrap_or_default();
    }

    pub fn add(&mut self, db: &Db) {
        let front = self.front.trim().to_string();
        let back = self.back.trim().to_string();
        if front.is_empty() || back.is_empty() {
            return;
        }
        let _ = db.create_card(self.deck_id, &front, &back, now());
        self.front.clear();
        self.back.clear();
        self.reload(db);
    }

    pub fn save(&mut self, db: &Db) {
        if let Some(id) = self.editing {
            let _ = db.update_card(id, self.front.trim(), self.back.trim(), now());
            self.editing = None;
            self.front.clear();
            self.back.clear();
            self.reload(db);
        }
    }

    pub fn cancel(&mut self) {
        self.editing = None;
        self.front.clear();
        self.back.clear();
    }
}

pub struct SearchState {
    pub query: String,
    pub deck_filter: Option<i64>,
    pub status_filter: Option<CardStatus>,
    pub results: Vec<Card>,
}

impl SearchState {
    pub fn new() -> Self {
        SearchState {
            query: String::new(),
            deck_filter: None,
            status_filter: None,
            results: Vec::new(),
        }
    }

    pub fn run(&mut self, db: &Db) {
        self.results = db
            .search(&self.query, self.deck_filter, self.status_filter, now())
            .unwrap_or_default();
    }
}
