# Repository Guidelines

## Project Structure & Module Organization
- `src/` contains all Rust code.
  - `src/main.rs` is the binary entrypoint.
  - `src/lib.rs` wires modules: `app`, `state`, `combat`, `ui`, `data`.
  - `src/app.rs` owns Bevy app setup and plugin wiring.
  - `src/state.rs` defines game states and transitions.
  - `src/combat.rs` holds the pure combat simulation logic.
  - `src/ui.rs` contains Bevy UI views and interaction wiring.
  - `src/data.rs` handles TOML loading and validation.
- `assets/` will hold Bevy assets (images, audio) and TOML content (e.g., `assets/data/cards.toml`). Create it as needed.
- `tasks.md` lists the current milestone tasks and tickets.

## Build, Test, and Development Commands
- `cargo run` — build and launch the game window.
- `cargo check` — fast compile check without producing a binary.
- `cargo test` — run unit tests.
- `cargo fmt` — format code with rustfmt (use before PRs).

## Coding Style & Naming Conventions
- Rust edition: 2024 (see `Cargo.toml`).
- Use rustfmt defaults (4‑space indentation, standard formatting).
- Naming: `snake_case` for functions/modules, `CamelCase` for types, `SCREAMING_SNAKE_CASE` for consts.
- Keep combat logic UI‑agnostic and deterministic for testability.

## Testing Guidelines
- Prefer unit tests alongside the module under test using `#[cfg(test)]`.
- Name tests for behavior, e.g., `test_draws_five_cards`.
- Keep tests deterministic (use fixed seeds if RNG is introduced).
- Run `cargo test` before opening a PR.

## Commit & Pull Request Guidelines
- This repo has no commit history yet; use a simple convention going forward:
  - `feat: …`, `fix: …`, `chore: …`, `docs: …`.
- PRs should include:
  - A concise summary of changes and the task/ticket.
  - The commands run (e.g., `cargo test`).
  - Screenshots or short clips for UI changes.

## Content & Configuration Notes
- Card, enemy, and encounter data should be TOML‑driven once the schema exists.
- Keep content files small and composable to support fast iteration.
