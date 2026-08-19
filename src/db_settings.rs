use rusqlite::params;

use crate::db::Db;
use crate::keymap::{Action, KeyBindings, key_from_name};

impl Db {
    pub fn load_bindings(&self) -> KeyBindings {
        let mut pairs = Vec::new();
        if let Ok(mut stmt) = self.conn.prepare("SELECT name, value FROM settings")
            && let Ok(rows) =
                stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
        {
            for row in rows.flatten() {
                if let (Some(action), Some(key)) =
                    (Action::from_name(&row.0), key_from_name(&row.1))
                {
                    pairs.push((action, key));
                }
            }
        }
        KeyBindings::from_pairs(pairs)
    }

    pub fn save_bindings(&self, bindings: &KeyBindings) -> rusqlite::Result<()> {
        self.conn.execute("DELETE FROM settings", [])?;
        for (action, key) in bindings.entries() {
            self.conn.execute(
                "INSERT INTO settings (name, value) VALUES (?1, ?2)",
                params![action.name(), key.name()],
            )?;
        }
        Ok(())
    }
}
