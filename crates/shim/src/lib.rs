//! tool_shim: holds one `tool_core::api::Game` and forwards the page's calls.
//!
//! Builder requirement: the shim decides nothing. Each method is one call into
//! the core and a conversion of its arguments: the page's clock arrives as a
//! float of milliseconds and becomes the core's whole number. The game lives
//! here, in the module, so its state is not copied out and back on every tick
//! (after Level Twenty). `crates/core/tests/boundary.rs` fails if a branch, a
//! loop or the `?` operator appears here.

use tool_core::api;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    inner: api::Game,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(strings: &str, glossary: &str, saved: &str, device: &str, now: f64) -> Result<Game, JsValue> {
        api::Game::new(strings, glossary, saved, device, now as u64)
            .map(|inner| Game { inner })
            .map_err(|e| JsValue::from_str(&e))
    }

    pub fn tick(&mut self, now: f64) -> String {
        self.inner.tick(now as u64)
    }

    pub fn act(&mut self, name: &str, value: &str, now: f64) -> String {
        self.inner.act(name, value, now as u64)
    }

    pub fn save(&self) -> String {
        self.inner.save()
    }

    pub fn export(&mut self, now: f64) -> String {
        self.inner.export(now as u64)
    }

    pub fn import(&mut self, text: &str, now: f64) -> String {
        self.inner.import(text, now as u64)
    }

    pub fn playtest(&mut self, now: f64) -> String {
        self.inner.playtest(now as u64)
    }
}
