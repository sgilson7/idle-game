//! tool_core: every rule of the idle game.
//!
//! Builder requirement: this crate performs no input or output. It takes
//! values and returns values, so `cargo test` reaches every rule without a
//! browser. `tests/boundary.rs` enforces that, along with the dependency
//! allowlist and the reference ban on floats, hash maps and clocks.

pub mod api;
pub mod content;
pub mod engines;
pub mod game;
pub mod glossary;
pub mod rng;
pub mod text;
pub mod view;
pub mod view_engines;
pub mod xp;
