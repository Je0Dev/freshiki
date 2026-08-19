use rusqlite::params;

use crate::db::Db;

pub struct Media {
    pub mime_type: String,
    pub data: Vec<u8>,
}

impl Db {
    pub fn insert_media(&self, mime_type: &str, data: &[u8], now: i64) -> rusqlite::Result<i64> {
        self.conn.execute(
            "INSERT INTO media (mime_type, data, created_at) VALUES (?1, ?2, ?3)",
            params![mime_type, data, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_media(&self, id: i64) -> rusqlite::Result<Option<Media>> {
        let mut stmt = self
            .conn
            .prepare("SELECT mime_type, data FROM media WHERE id = ?1")?;
        let mut rows = stmt.query_map(params![id], |r| {
            Ok(Media {
                mime_type: r.get(0)?,
                data: r.get(1)?,
            })
        })?;
        rows.next().transpose()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::db::Db;

    #[test]
    fn media_round_trip() {
        let path = PathBuf::from(format!(
            "{}/freshiki_test_{}.db",
            std::env::temp_dir().display(),
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let db = Db::open(&path).unwrap();
        let id = db.insert_media("image/png", &[1, 2, 3, 4], 0).unwrap();
        let media = db.get_media(id).unwrap().unwrap();
        assert_eq!(media.mime_type, "image/png");
        assert_eq!(media.data, vec![1, 2, 3, 4]);
        let _ = std::fs::remove_file(&path);
    }
}
