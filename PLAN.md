# Freshiki Feature Plan - Next Core Features

Roadmap for the next set of core features. Each section lists goals, UX,
storage/DB changes, new modules and dependencies, and the test approach.

- `[x]` = implemented
- `[ ]` = planned

## 1. [x] Keyboard Navigation

**Status:** done. See `keymap.rs`, `db_settings.rs`, `ui/settings.rs`. Bindings
persist in the `settings` table and are remappable with conflict rejection.

### Goals

- Drive the study flow entirely from the keyboard.
- Remap every binding through a settings menu, with the new layout persisted.

### UX

- `Enter` / `Space` - flip the current card (show/hide answer).
- `Left` / `Right` - move to the previous / next card.
- `Enter` while a card is focused in the editor - open that card for editing.
- A **Settings** view listing every action with its current key, letting the
  user reassign each one. Duplicate or conflicting bindings are rejected.

### Implementation

- New pure module `keymap.rs`: an `Action` enum, a `KeyBindings` map with
  defaults, and remap/conflict-resolution helpers. Kept side-effect-free so it
  is unit-testable, per CONSTRAINTS.md.
- Key input captured in `app.rs` and the study/editor views via
  `ctx.input(|i| ...)` using egui's `Key` enum.
- Bindings persisted in SQLite (new `settings` table) and seeded with defaults
  in a migration on first run.
- `app.rs` must stay under the file-size limit; if it grows past it, split the
  settings view into `ui/settings.rs`.

### Tests

- Keymap defaults, remapping, and duplicate-binding rejection in `keymap.rs`.

## 2. [x] Export Functionality

**Status:** done. See `export.rs`, `db_export.rs`, `ui/export.rs`. The `rfd`
file picker was skipped (no GTK dev headers in the build environment); a path
text field is used instead.

### Goals

- Export decks to portable formats so cards are never locked in the app.

### Formats

- **CSV** - header row: deck, front, back, ease, interval, repetitions,
  due_at, updated_at. UTF-8 encoded.
- **JSON** - a structured document: `{ "deck": "...", "cards": [ ... ] }` with
  the same fields.

### UX

- An "Export" button in the decks view and the editor view.
- Destination chosen via a file picker (`rfd` crate) or a path text field.

### Implementation

- New pure module `export.rs` with `to_csv(...)` and `to_json(...)` functions;
  serialization is deterministic and side-effect-free.
- New `Db` query returning full card rows joined with their deck name.
- `rfd` added as a dependency for native file dialogs.

### Tests

- CSV/JSON output for a sample card list, including escaping and Unicode.

## 3. [x] Image & Audio Support

**Status:** mostly done. See `db_media.rs`, `media.rs`, `ui/media.rs`. Audio is
played by the OS default handler today (in-app playback is section 9). The
`rfd` picker was replaced with path fields / drag-and-drop (section 4).

### Goals

- Embed images and audio on card fronts and/or backs.

### Storage

- **Blobs in SQLite** so the app stays a single portable file.
- New `media` table: `id`, `mime_type`, `data`, `created_at` (migration
  required).
- Front/back text references media with markup, e.g. `[[media:id]]`, resolved
  at render time.

### UX

- "Attach Image" / "Attach Audio" buttons in the editor, backed by the same
  `rfd` file picker.
- Images rendered with egui's `Image` (via `egui_extras` for byte-based
  loading); audio played with a native `audio` widget.

### Implementation

- [x] `Db` gains `insert_media` / `get_media`.
- [ ] `delete_media` (cleanup of unused blobs).
- [ ] Media included in JSON export as base64; CSV references IDs only.

### Tests

- Media insert/load round-trip and markup parsing helpers.

## 4. [x] Media Folders & Drag-and-Drop

**Status:** done. See `media.rs`, `ui/media.rs`, `ui/editor.rs`. Folders live
under `<data>/freshiki/media/` and are created at startup; duck.png and
quack.mp3 ship as test media.

### Goals

- Give the user managed folders for their image and audio source files so
  media can be linked by path or dropped onto a card field directly.
- Ship test media (duck.png, quack.mp3) in those folders for trying it out.

### Media folders

- Managed under the app data dir: `<data>/freshiki/media/images/` and
  `<data>/freshiki/media/audio/`.
- Created automatically at startup; "Open Images" / "Open Audio" buttons in
  the editor reveal them in the OS file manager.
- Duck and quack test files copied into the folders so they are ready to use.

### Drag-and-drop

- The editor shows two drop zones, one for **Front** and one for **Back**.
- Dropping an image/audio file onto a zone reads its bytes, stores them as a
  media blob, and appends `[[media:id]]` to that field (no path dependency).
- Zones highlight while a file is hovered over them (egui
  `hovered_files` / `dropped_files`).

### Implementation

- `media.rs` gains `images_dir` / `audio_dir` / `ensure_media_dirs` and an
  `open_folder` helper.
- Editor drop handling attaches via `DroppedFile::bytes()` (with
  `std::fs::read` fallback) and MIME detection from the file extension.
- Unsupported files (not image/audio) are rejected with a message.

### Tests

- Existing media round-trip and markup tests continue to cover the blob path;
  drop/zone glue is thin egui code.

## 5. [ ] Cloze Prefix / Suffix Support (Anki-style)

### Goals

- Cloze deletion just like Anki: hide a span of text, reveal it on flip.
- Prefix and suffix strings configurable from Settings (Anki defaults).

### UX

- Cloze syntax `{{c1::answer}}` and hinted `{{c1::answer::hint}}`.
- Settings expose `cloze_prefix` / `cloze_suffix` (defaults `{{c1::` / `}}`).
- Editor helper "Insert Cloze" wraps the selected text.
- Study mode: each cloze in a card becomes its own review card (Anki
  behavior); the answer shows as a blank (hint below) until flipped.

### Implementation

- Pure module `cloze.rs`: parse a field into cloze segments, expand N clozes
  into N virtual review cards; hint extraction; unit-tested.
- Settings persisted in the existing `settings` table.
- Render blanks in `ui/study.rs` / `ui/media.rs`.

### Tests

- Parsing, multi-cloze expansion, hint extraction, prefix/suffix overrides.

## 6. [ ] Clock & Session Timer

### Goals

- A clean, minimalistic clock and a timer showing how long the user has been
  in the app.

### UX

- Small monospace clock (current time) plus a count-up session timer in the
  topbar; unobtrusive, no clutter.

### Implementation

- App stores `started_at: std::time::Instant`; clock from `chrono` (already a
  dependency).
- `format_duration` helper (mm:ss, then hh:mm:ss) in a testable module.

### Tests

- `format_duration` boundary formatting.

## 7. [ ] Chinese / Pinyin Ruby Support

### Goals

- Strong Chinese support for language learning: pinyin rendered as ruby above
  each hanzi character, exactly like HTML `<ruby>`.

### UX

- Any hanzi in a card field gets pinyin annotations above the characters,
  auto-generated offline; no manual input.
- Optional tone-colored pinyin.

### Implementation

- Add the pure-Rust `pinyin` crate (no system dependencies).
- New module `ruby.rs`: segment text into hanzi runs, map each character to
  pinyin, build ruby pairs; unit-tested.
- Custom rendering with `ui.painter()` (egui has no native ruby) in
  `ui/media.rs`, shared by study and editor preview.

### Tests

- Hanzi segmentation, pinyin mapping, ruby pair building.

## 8. [ ] Basic UI Customization (Settings)

### Goals

- Let the user tune the look of the app without touching config files.

### UX

- Settings gains an "Appearance" section: Dark / Light / System theme, accent
  color, and base font size.

### Implementation

- egui `ctx.set_theme(...)`, style overrides for fonts/spacing, accent color.
- Persisted through the existing `settings` table (`load_bindings` /
  `save_bindings` generalize to all settings).

### Tests

- Persistence round-trip via the existing settings storage.

## 9. [ ] Native In-App Audio Playback

### Goals

- Play card audio inside the app instead of opening an external player
  (today: `xdg-open`).

### UX

- Play button streams the stored blob; play/stop state shown on the card.

### Implementation

- Add `rodio` (with a decoder feature) playing from in-memory bytes.
- Replace `media::play_audio`'s external open with an in-app sink.
- New Linux build dependency at build time: ALSA dev headers
  (`libasound2-dev`) - document in README/CONTRIBUTING.

### Tests

- Thin; decode a tiny sample clip if practical.

## 10. [ ] README Install Guide, AppImage & CI

### Goals

- Clear install instructions for Windows, macOS, and Linux, plus a packaged
  Linux AppImage and CI-built release artifacts.

### UX / docs

- README: per-OS prerequisites (Rust edition 2024; Linux X11/Wayland + GL
  libs and `libasound2-dev`), `cargo build --release`, binary location,
  optional `cargo install --path .`.
- `scripts/build-appimage.sh` using linuxdeploy/appimagetool.
- GitHub Actions workflow (`.github/workflows/release.yml`) building Windows,
  macOS, and Linux binaries plus an AppImage, attached to a GitHub Release.

### Implementation

- Workflow YAML + packaging script; no application code changes.

### Tests

- N/A (docs and packaging).

## Out of scope (for now)

- Anki `.apkg` import/export.
- Cloud sync.
- Media editing tools (crop, trim, recolor).
