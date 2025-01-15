use wasm_bindgen::prelude::wasm_bindgen;
mod game;
mod gameloop;
mod ui;

use gameloop::*;

#[wasm_bindgen]
pub fn init(_variant: String) {
    //let mut gameloop = GameLoop::new(Variant::Curtail);
    let mut gameloop = GameLoop::new(Variant::ChangeSecret);
    gameloop.run();
}

#[wasm_bindgen]
pub fn add (a: usize, b: usize) -> usize {
    a+b
}


#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}