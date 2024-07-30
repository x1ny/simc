use rogue::Rogue;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod common;

pub mod rogue;

pub fn test() {
    let mut _a: WebSimc = WebSimc::new();
    // a.combat_points += 3;
    // a.combat_points.value()
}

#[wasm_bindgen]
pub struct WebSimc {
    rogue: Rogue
}

#[wasm_bindgen]
impl WebSimc {
    pub fn new () -> Self {
        Self {
            rogue: Rogue::new()
        }
    }

    pub fn say_hi() -> String {
        "Hello World".to_string()
    }
}
