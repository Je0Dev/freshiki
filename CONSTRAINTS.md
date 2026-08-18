# Constraints

These rules apply to every commit in this repository. Reviews must enforce them.

## Git workflow

- `main` is protected: **never commit directly to it**.
- **Every feature, test, or fix gets its own branch** created from an up-to-date
  `main`, then merged with a merge commit. Example:
  - `feat/<name>` for app features,
  - `test/<name>` for new or updated tests,
  - `fix/<name>`, `docs/<name>`, `chore/<name>` for the rest.
- Commit messages use Conventional Commits (`feat:`, `test:`, `fix:`, `docs:`,
  `chore:`). See [CONTRIBUTING.md](CONTRIBUTING.md) for the full workflow.
- Tests must pass on a branch before it is merged into `main`.

## Merge approval

- **When changes are pushed to GitHub and the user has not explicitly said
  "merge", never merge to `main` automatically.**
- Instead, push the work on its own branch and leave it for the user to review
  and approve the merge on GitHub.
- If the user asks the agent to merge on their behalf, that explicit request
  is approval enough - the agent may then merge and push `main`.

## Code size

- Every source file must stay within **100-120 lines maximum**.
- If a file grows past the limit, split it (e.g. separate `db.rs` / `db_cards.rs`
  for persistence concerns, or a dedicated `state.rs` for UI state).

## Functions

- Keep functions small, focused, and single-purpose. **No monolithic functions**
  (a function must not load data, run the SRS math, and render at once).
- Prefer several small helpers over one large function.
- A function that becomes hard to read should be split into named helpers.

## Comments

- Comment *why*, not what - the code should already show what it does.
- Every public item (`pub fn`, `pub struct`, `pub enum`) gets a short doc comment.
- Keep comments brief and to the point.
- **No `TODO:` markers or unfinished code may be committed.**

## Simplicity

- Keep things simple and direct. Avoid clever one-liners and unnecessary abstraction.
- Prefer plain data structures over over-engineered generic layers.
- No dead code: remove unused fields, methods, and imports before committing.

## Tests

- Tests are required for **important features**: the SM-2 scheduler, due-date
  logic, and persistence. Not every function needs a test - small glue helpers
  and trivial UI wrappers do not.
- Pure logic lives in testable modules (`model.rs`, `srs.rs`), separate from
  egui input/render code.
- Run `cargo test` and all tests must pass before merging.

## Maintainability & scalability

- Code must be maintainable, scalable, and optimal where it matters:
  - no accidental `O(n^2)` scans of card lists in hot paths,
  - SQL filtering pushed into queries instead of post-filtering in Rust where
    reasonable,
  - deterministic, side-effect-free helper functions where possible.
- Build cleanly with `cargo fmt --check` and `cargo clippy -- -D warnings`.

## Style

- Rust edition 2024, `cargo fmt` formatting (rustfmt defaults).
- `snake_case` for functions and variables, `UpperCamelCase` for types and
  enum variants, `SCREAMING_SNAKE` for constants.
- Prefer `use` paths in the same module (no absolute `crate::` re-export spam).
- Order helpers top-down: private helpers above the public items that use them.