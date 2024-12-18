// main.rs
mod game;
mod gameloop;
mod ui;

use gameloop::*;

fn main() {
    //let mut gameloop = GameLoop::new(Variant::Curtail);
    let mut gameloop = GameLoop::new(Variant::Curtail);
    gameloop.run();
}
