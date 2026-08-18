use crate::model::{Card, SECS_PER_DAY};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    Again,
    Hard,
    Good,
    Easy,
}

impl Grade {
    pub fn label(self) -> &'static str {
        match self {
            Grade::Again => "Again",
            Grade::Hard => "Hard",
            Grade::Good => "Good",
            Grade::Easy => "Easy",
        }
    }
}

pub fn review(card: &Card, grade: Grade, now: i64) -> Card {
    let mut c = card.clone();
    match grade {
        Grade::Again => {
            c.repetitions = 0;
            c.interval = 1;
            c.ease = (c.ease - 0.20).max(1.3);
        }
        Grade::Hard => {
            c.repetitions += 1;
            c.interval = if c.repetitions == 1 {
                1
            } else {
                (c.interval as f64 * 1.2).round() as i64
            };
            c.ease = (c.ease - 0.15).max(1.3);
        }
        Grade::Good => {
            c.repetitions += 1;
            c.interval = match c.repetitions {
                1 => 1,
                2 => 6,
                _ => (c.interval as f64 * c.ease).round() as i64,
            };
        }
        Grade::Easy => {
            c.repetitions += 1;
            c.interval = match c.repetitions {
                1 => 4,
                2 => 7,
                _ => (c.interval as f64 * c.ease * 1.3).round() as i64,
            };
            c.ease += 0.15;
        }
    }
    c.due_at = now + c.interval * SECS_PER_DAY;
    c.updated_at = now;
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_card() -> Card {
        Card {
            id: 1,
            front: String::new(),
            back: String::new(),
            ease: 2.5,
            interval: 0,
            repetitions: 0,
            due_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn good_grows_interval() {
        let now = 1_000_000;
        let mut c = new_card();
        for _ in 0..3 {
            c = review(&c, Grade::Good, now);
        }
        assert_eq!(c.repetitions, 3);
        assert!(c.interval >= 6);
    }

    #[test]
    fn again_resets_and_lowers_ease() {
        let mut c = new_card();
        c = review(&c, Grade::Good, 0);
        c = review(&c, Grade::Good, 0);
        c = review(&c, Grade::Again, 0);
        assert_eq!(c.repetitions, 0);
        assert_eq!(c.interval, 1);
        assert!(c.ease < 2.5);
    }

    #[test]
    fn ease_floor_is_1_3() {
        let mut c = new_card();
        for _ in 0..20 {
            c = review(&c, Grade::Again, 0);
        }
        assert!((c.ease - 1.3).abs() < 1e-9);
    }

    #[test]
    fn due_is_now_plus_interval() {
        let now = 1_000_000;
        let c = review(&new_card(), Grade::Good, now);
        assert_eq!(c.due_at, now + SECS_PER_DAY);
    }
}
