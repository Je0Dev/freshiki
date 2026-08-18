use rusqlite::{Row, params};

use crate::db::Db;
use crate::model::{Card, CardStatus, status};

impl Db {
    pub fn list_cards(&self, deck_id: i64) -> rusqlite::Result<Vec<Card>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, front, back, ease, interval, repetitions,
                    due_at, updated_at
             FROM cards WHERE deck_id = ?1 ORDER BY id",
        )?;
        let rows = stmt.query_map(params![deck_id], row_to_card)?;
        rows.collect()
    }

    pub fn create_card(
        &self,
        deck_id: i64,
        front: &str,
        back: &str,
        now: i64,
    ) -> rusqlite::Result<i64> {
        self.conn.execute(
            "INSERT INTO cards (deck_id, front, back, due_at, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?4, ?4)",
            params![deck_id, front, back, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn update_card(&self, id: i64, front: &str, back: &str, now: i64) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE cards SET front = ?1, back = ?2, updated_at = ?3 WHERE id = ?4",
            params![front, back, now, id],
        )?;
        Ok(())
    }

    pub fn delete_card(&self, id: i64) -> rusqlite::Result<()> {
        self.conn
            .execute("DELETE FROM cards WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn save_review(&self, card: &Card) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE cards SET ease = ?1, interval = ?2, repetitions = ?3,
                    due_at = ?4, updated_at = ?5
             WHERE id = ?6",
            params![
                card.ease,
                card.interval,
                card.repetitions,
                card.due_at,
                card.updated_at,
                card.id
            ],
        )?;
        Ok(())
    }

    pub fn search(
        &self,
        query: &str,
        deck: Option<i64>,
        status_filter: Option<CardStatus>,
        now: i64,
    ) -> rusqlite::Result<Vec<Card>> {
        let mut sql = String::from(
            "SELECT id, front, back, ease, interval, repetitions,
                    due_at, updated_at
             FROM cards WHERE (front LIKE ?1 OR back LIKE ?1)",
        );
        if let Some(id) = deck {
            sql.push_str(&format!(" AND deck_id = {id}"));
        }
        sql.push_str(" ORDER BY id");
        let pattern = format!("%{query}%");
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(params![pattern], row_to_card)?;
        let cards = rows.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(match status_filter {
            Some(expected) => cards
                .into_iter()
                .filter(|c| status(c, now) == expected)
                .collect(),
            None => cards,
        })
    }
}

fn row_to_card(row: &Row) -> rusqlite::Result<Card> {
    Ok(Card {
        id: row.get(0)?,
        front: row.get(1)?,
        back: row.get(2)?,
        ease: row.get(3)?,
        interval: row.get(4)?,
        repetitions: row.get(5)?,
        due_at: row.get(6)?,
        updated_at: row.get(7)?,
    })
}
