use crate::db::Db;
use crate::export::ExportRow;

impl Db {
    pub fn export_rows(&self, deck_id: Option<i64>) -> rusqlite::Result<Vec<ExportRow>> {
        let mut sql = String::from(
            "SELECT d.name, c.front, c.back, c.ease, c.interval, c.repetitions,
                    c.due_at, c.updated_at
             FROM cards c JOIN decks d ON d.id = c.deck_id",
        );
        if let Some(id) = deck_id {
            sql.push_str(&format!(" WHERE c.deck_id = {id}"));
        }
        sql.push_str(" ORDER BY d.id, c.id");
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map([], |r| {
            Ok(ExportRow {
                deck: r.get(0)?,
                front: r.get(1)?,
                back: r.get(2)?,
                ease: r.get(3)?,
                interval: r.get(4)?,
                repetitions: r.get(5)?,
                due_at: r.get(6)?,
                updated_at: r.get(7)?,
            })
        })?;
        rows.collect()
    }
}
