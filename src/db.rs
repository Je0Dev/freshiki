use std::path::Path;

use rusqlite::{Connection, params};

use crate::model::Deck;

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        let db = Db { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> rusqlite::Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS decks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS cards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                deck_id INTEGER NOT NULL REFERENCES decks(id) ON DELETE CASCADE,
                front TEXT NOT NULL,
                back TEXT NOT NULL,
                ease REAL NOT NULL DEFAULT 2.5,
                interval INTEGER NOT NULL DEFAULT 0,
                repetitions INTEGER NOT NULL DEFAULT 0,
                due_at INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );",
        )
    }

    pub fn list_decks(&self) -> rusqlite::Result<Vec<Deck>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM decks ORDER BY name")?;
        let rows = stmt.query_map([], |r| {
            Ok(Deck {
                id: r.get(0)?,
                name: r.get(1)?,
            })
        })?;
        rows.collect()
    }

    pub fn create_deck(&self, name: &str, now: i64) -> rusqlite::Result<i64> {
        self.conn.execute(
            "INSERT INTO decks (name, created_at) VALUES (?1, ?2)",
            params![name, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn rename_deck(&self, id: i64, name: &str) -> rusqlite::Result<()> {
        self.conn.execute(
            "UPDATE decks SET name = ?1 WHERE id = ?2",
            params![name, id],
        )?;
        Ok(())
    }

    pub fn delete_deck(&self, id: i64) -> rusqlite::Result<()> {
        self.conn
            .execute("DELETE FROM decks WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn card_count(&self, deck_id: i64) -> rusqlite::Result<i64> {
        self.conn.query_row(
            "SELECT COUNT(*) FROM cards WHERE deck_id = ?1",
            params![deck_id],
            |r| r.get(0),
        )
    }

    pub fn due_count(&self, deck_id: i64, now: i64) -> rusqlite::Result<i64> {
        self.conn.query_row(
            "SELECT COUNT(*) FROM cards WHERE deck_id = ?1 AND due_at <= ?2 AND repetitions > 0",
            params![deck_id, now],
            |r| r.get(0),
        )
    }
}
