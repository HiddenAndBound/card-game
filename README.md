# Card Roguelite (Bevy)

A small card‑based roguelite prototype inspired by Slay the Spire. Built in Rust with Bevy. The MVP targets a 3‑encounter run with a deterministic, testable combat core and Bevy UI.

## Goals
- Fast iteration for a 2‑person team new to Bevy.
- Data‑driven content via TOML.
- Local‑only LLM integration planned for later.

## Quickstart
- `cargo run` — build and launch the game window.
- `cargo check` — fast compile check.
- `cargo test` — run unit tests.
- `cargo fmt` — format code.

## Project Structure
- `src/main.rs` — binary entrypoint.
- `src/lib.rs` — module wiring.
- `src/app.rs` — Bevy app setup and plugins.
- `src/state.rs` — game states and transitions.
- `src/combat.rs` — combat simulation logic.
- `src/ui.rs` — Bevy UI layer.
- `src/data.rs` — TOML data loading and validation.
- `tasks.md` — current milestone tasks and tickets.
- `plan.md` — high‑level iteration plan.

## Development Notes
- Keep combat logic UI‑agnostic and deterministic.
- Prefer data‑driven content once TOML schemas land.
- Add assets under `assets/` as needed.

## Roadmap
See `tasks.md` for the current work breakdown and `plan.md` for the iteration plan.
