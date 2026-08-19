use std::path::PathBuf;

use crate::db::Db;

pub fn media_root() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("freshiki")
        .join("media")
}

pub fn images_dir() -> PathBuf {
    media_root().join("images")
}

pub fn audio_dir() -> PathBuf {
    media_root().join("audio")
}

/// Create the media folders, ignoring errors if they already exist.
pub fn ensure_media_dirs() {
    let _ = std::fs::create_dir_all(images_dir());
    let _ = std::fs::create_dir_all(audio_dir());
}

/// Open a folder in the OS file manager.
pub fn open_folder(path: &PathBuf) {
    let _ = std::process::Command::new(open_cmd()).arg(path).spawn();
}

/// Extract every media id referenced as `[[media:N]]` in a card field.
pub fn media_ids(text: &str) -> Vec<i64> {
    let mut ids = Vec::new();
    for token in text.split("[[").skip(1) {
        let Some(rest) = token.strip_prefix("media:") else {
            continue;
        };
        let num = rest.split(']').next().unwrap_or_default();
        if let Ok(id) = num.parse::<i64>() {
            ids.push(id);
        }
    }
    ids
}

pub fn mime_from_path(path: &str) -> &'static str {
    let lower = path.to_ascii_lowercase();
    if lower.ends_with(".png") {
        "image/png"
    } else if lower.ends_with(".jpg") || lower.ends_with(".jpeg") {
        "image/jpeg"
    } else if lower.ends_with(".gif") {
        "image/gif"
    } else if lower.ends_with(".mp3") {
        "audio/mpeg"
    } else if lower.ends_with(".wav") {
        "audio/wav"
    } else if lower.ends_with(".ogg") || lower.ends_with(".oga") {
        "audio/ogg"
    } else if lower.ends_with(".flac") {
        "audio/flac"
    } else {
        "application/octet-stream"
    }
}

/// Read a file and store it as a media blob, returning the new id.
pub fn attach(db: &Db, path: &str, now: i64) -> Result<i64, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    if bytes.is_empty() {
        return Err("file is empty".to_string());
    }
    db.insert_media(mime_from_path(path), &bytes, now)
        .map_err(|e| e.to_string())
}

pub fn mime_is_audio(mime: &str) -> bool {
    mime.starts_with("audio/")
}

pub fn ext_from_mime(mime: &str) -> &'static str {
    match mime {
        "image/png" => "png",
        "image/jpeg" => "jpg",
        "image/gif" => "gif",
        "audio/mpeg" => "mp3",
        "audio/wav" => "wav",
        "audio/ogg" => "ogg",
        "audio/flac" => "flac",
        _ => "bin",
    }
}

/// Write audio bytes to a temp file and open it with the OS default handler.
pub fn play_audio(id: i64, mime: &str, data: &[u8]) {
    let path = std::env::temp_dir().join(format!("freshiki_{id}.{}", ext_from_mime(mime)));
    if std::fs::write(&path, data).is_err() {
        return;
    }
    let _ = std::process::Command::new(open_cmd()).arg(path).spawn();
}

fn open_cmd() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "start"
    }
    #[cfg(target_os = "macos")]
    {
        "open"
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        "xdg-open"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_media_references() {
        assert_eq!(media_ids("see [[media:3]] here"), vec![3]);
        assert_eq!(media_ids("no refs"), Vec::<i64>::new());
        assert_eq!(media_ids("[[media:1]][[media:2]]"), vec![1, 2]);
        assert_eq!(media_ids("[[media:abc]]"), Vec::<i64>::new());
    }

    #[test]
    fn detects_mime_by_extension() {
        assert_eq!(mime_from_path("a.PNG"), "image/png");
        assert_eq!(mime_from_path("clip.mp3"), "audio/mpeg");
        assert_eq!(mime_from_path("x.xyz"), "application/octet-stream");
    }
}
