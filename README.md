# Freshiki

A GUI flashcard app with spaced repetition, built in Rust with
[egui](https://github.com/emilk/egui) and SQLite.

## Features

- **Deck management** - create, rename, and delete decks; see card and due
  counts.
- **Study mode** - flip cards and grade them with Anki-style buttons
  (Again / Hard / Good / Easy).
- **Spaced repetition** - SM-2-inspired scheduler that grows review intervals.
- **Card editor** - add, edit, and delete cards (front/back) per deck.
- **Search & filter** - text search across cards, filtered by deck and status.
- **Keyboard navigation** - flip, move between, and edit cards with the
  keyboard; every shortcut is remappable in Settings.
- **Export** - export any deck to CSV or JSON.
- **Media support** - attach images and audio to cards (stored in SQLite),
  with managed media folders and drag-and-drop.

## Requirements

- Rust (edition 2024). Install with [rustup] if needed.
- On Linux, the usual build essentials (`gcc`/`cc`) plus X11/Wayland and GL
  libraries for the egui window.

## Clone & build

```bash
git clone https://github.com/Je0Dev/freshiki.git
cd freshiki
cargo run --release
```

The database and media folders are created at `~/.local/share/freshiki/` on
Linux (and the platform equivalent on Windows/macOS).

## Install the binary

- **Linux**: `cargo build --release` and run `target/release/freshiki`.
  Or install it globally with `cargo install --path .`.
- **Windows**: `cargo build --release`, then run
  `target\release\freshiki.exe`.
- **macOS**: `cargo build --release`, then run `target/release/freshiki`.

## Development

```bash
cargo fmt --check   # formatting
cargo clippy -- -D warnings
cargo test          # unit tests
cargo run           # launch the app
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the git workflow and
[CONSTRAINTS.md](CONSTRAINTS.md) for code rules. Upcoming features live in
[PLAN.md](PLAN.md).

## License

MIT. See [LICENSE](LICENSE).

[egui]: https://github.com/emilk/egui
[rustup]: https://rustup.rs/