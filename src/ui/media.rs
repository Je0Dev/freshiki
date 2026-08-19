use std::collections::HashMap;

use eframe::egui;

use crate::db::Db;
use crate::media::{ext_from_mime, media_ids, mime_is_audio, play_audio};

const MAX_IMAGE: egui::Vec2 = egui::Vec2::new(320.0, 240.0);

/// Render a card field, replacing `[[media:N]]` tokens with image/audio widgets.
pub fn render_field(
    db: &Db,
    cache: &mut HashMap<i64, egui::TextureHandle>,
    ui: &mut egui::Ui,
    text: &str,
) {
    if media_ids(text).is_empty() {
        ui.label(text);
        return;
    }
    for part in split_text(text) {
        match part {
            MediaPart::Text(s) => {
                if !s.is_empty() {
                    ui.label(s);
                }
            }
            MediaPart::Media(id) => render_media(db, cache, ui, id),
        }
    }
}

enum MediaPart {
    Text(String),
    Media(i64),
}

fn split_text(text: &str) -> Vec<MediaPart> {
    let mut parts = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("[[media:") {
        parts.push(MediaPart::Text(rest[..start].to_string()));
        let tail = &rest[start + "[[media:".len()..];
        let Some(end) = tail.find("]]") else {
            parts.push(MediaPart::Text(rest.to_string()));
            return parts;
        };
        let id = tail[..end].parse::<i64>().unwrap_or(0);
        parts.push(MediaPart::Media(id));
        rest = &tail[end + 2..];
    }
    parts.push(MediaPart::Text(rest.to_string()));
    parts
}

fn render_media(
    db: &Db,
    cache: &mut HashMap<i64, egui::TextureHandle>,
    ui: &mut egui::Ui,
    id: i64,
) {
    let Ok(Some(media)) = db.get_media(id) else {
        return;
    };
    if mime_is_audio(&media.mime_type) {
        if ui
            .button(format!("Play audio ({})", ext_from_mime(&media.mime_type)))
            .clicked()
        {
            play_audio(id, &media.mime_type, &media.data);
        }
    } else if let Some(texture) = cached_texture(cache, ui.ctx(), id, &media.data) {
        let size = texture.size_vec2().min(MAX_IMAGE);
        ui.image((texture.id(), size));
    }
}

fn cached_texture(
    cache: &mut HashMap<i64, egui::TextureHandle>,
    ctx: &egui::Context,
    id: i64,
    data: &[u8],
) -> Option<egui::TextureHandle> {
    if let Some(texture) = cache.get(&id) {
        return Some(texture.clone());
    }
    let Ok(img) = image::load_from_memory(data).map(to_color_image) else {
        return None;
    };
    let texture = ctx.load_texture(format!("media-{id}"), img, egui::TextureOptions::LINEAR);
    cache.insert(id, texture.clone());
    Some(texture)
}

fn to_color_image(img: image::DynamicImage) -> egui::ColorImage {
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &rgba)
}

/// Attach a file to the front or back field, appending a `[[media:N]]` token.
pub fn attach(state: &mut crate::state::EditorState, db: &Db, front: bool) {
    let path = state.media_path.trim().to_string();
    if path.is_empty() {
        state.media_error = Some("Enter a file path".to_string());
        return;
    }
    match crate::media::attach(db, &path, crate::model::now()) {
        Ok(id) => {
            let field = if front {
                &mut state.front
            } else {
                &mut state.back
            };
            field.push_str(&format!("[[media:{id}]]"));
            state.media_path.clear();
            state.media_error = None;
        }
        Err(msg) => state.media_error = Some(msg),
    }
}

/// Render a drop target; highlights while files hover over the app.
pub fn drop_zone(ui: &mut egui::Ui, label: &str, hovered: bool) -> egui::Rect {
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 44.0), egui::Sense::hover());
    let (fill, stroke) = if hovered {
        (
            egui::Color32::from_rgb(40, 60, 90),
            egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
        )
    } else {
        (
            egui::Color32::from_rgb(25, 25, 25),
            egui::Stroke::new(1.0, egui::Color32::GRAY),
        )
    };
    ui.painter()
        .rect(rect, 4.0, fill, stroke, egui::StrokeKind::Inside);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(14.0),
        egui::Color32::GRAY,
    );
    rect
}

/// Attach a dropped file to the front or back field as a media blob.
pub fn attach_dropped(
    state: &mut crate::state::EditorState,
    db: &Db,
    front: bool,
    file: &dyn egui::DroppedFile,
) {
    let path = file.path();
    let mime = crate::media::mime_from_path(&path.to_string_lossy());
    if !mime.starts_with("image/") && !mime.starts_with("audio/") {
        state.media_error = Some(format!("Unsupported file: {}", path.display()));
        return;
    }
    let bytes = file.bytes().or_else(|_| std::fs::read(path));
    match bytes {
        Ok(bytes) if !bytes.is_empty() => {
            match db.insert_media(mime, &bytes, crate::model::now()) {
                Ok(id) => {
                    let field = if front {
                        &mut state.front
                    } else {
                        &mut state.back
                    };
                    field.push_str(&format!("[[media:{id}]]"));
                    state.media_error = None;
                }
                Err(e) => state.media_error = Some(e.to_string()),
            }
        }
        _ => state.media_error = Some("Could not read dropped file".to_string()),
    }
}
