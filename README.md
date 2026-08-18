# Freshiki

A GUI flashcard app with spaced repetition, built in Rust with [egui] and
SQLite.

## Features

- **Deck management** - create, rename, and delete decks; see card and due counts.
- **Study mode** - flip cards and grade them with Anki-style buttons
  (Again / Hard / Good / Easy).
- **Spaced repetition** - SM-2-inspired scheduler that grows review intervals
  based on your answers and tracks an ease factor per card.
- **Card editor** - add, edit, and delete cards (front/back) per deck.
- **Search & filter** - text search across cards, filtered by deck and status
  (New / Learning / Due / Known).

## Requirements

- Rust (edition 2024). Install with [rustup] if needed.
- On Linux, the usual build essentials (`gcc`/`cc`) plus X11/Wayland and GL
  libraries for the egui window.

## Getting started

```bash
cargo run
```

The database is created automatically at `~/.local/share/freshiki/app.db`
on Linux.

## Project layout

```
src/
  main.rs      - eframe entry point and window setup
  app.rs       - top-level app state and navigation
  model.rs     - Deck / Card types and card status helpers
  srs.rs       - SM-2 spaced-repetition scheduler (unit-tested)
  db.rs        - SQLite connection, migrations, deck CRUD
  db_cards.rs  - card CRUD and search queries
  state.rs     - study / editor / search UI state
  ui/          - egui views: decks, study, editor, search
```

## Development

```bash
cargo fmt --check   # formatting
cargo clippy -- -D warnings
cargo test          # scheduler unit tests
cargo run           # launch the app
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the git workflow and
[CONSTRAINTS.md](CONSTRAINTS.md) for code rules.

## License

MIT. See [LICENSE](LICENSE).

[egui]: https://github.com/emilk/egui
[rustup]: https://rustup.rs/