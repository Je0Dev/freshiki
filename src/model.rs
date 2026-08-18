use chrono::Utc;

pub const SECS_PER_DAY: i64 = 86_400;

pub fn now() -> i64 {
    Utc::now().timestamp()
}

#[derive(Clone)]
pub struct Deck {
    pub id: i64,
    pub name: String,
}

#[derive(Clone)]
pub struct Card {
    pub id: i64,
    pub front: String,
    pub back: String,
    pub ease: f64,
    pub interval: i64,
    pub repetitions: i64,
    pub due_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardStatus {
    New,
    Learning,
    Due,
    Known,
}

impl CardStatus {
    pub fn label(self) -> &'static str {
        match self {
            CardStatus::New => "New",
            CardStatus::Learning => "Learning",
            CardStatus::Due => "Due",
            CardStatus::Known => "Known",
        }
    }

    pub fn all() -> [CardStatus; 4] {
        [
            CardStatus::New,
            CardStatus::Learning,
            CardStatus::Due,
            CardStatus::Known,
        ]
    }
}

pub fn status(card: &Card, now: i64) -> CardStatus {
    if card.repetitions == 0 {
        CardStatus::New
    } else if card.due_at <= now {
        CardStatus::Due
    } else if card.repetitions < 2 {
        CardStatus::Learning
    } else {
        CardStatus::Known
    }
}

pub fn due_label(due_at: i64, now: i64) -> String {
    let secs = due_at - now;
    if secs <= 0 {
        "due".to_string()
    } else if secs < SECS_PER_DAY {
        format!("{}h", secs / 3_600)
    } else {
        format!("{}d", secs / SECS_PER_DAY)
    }
}
