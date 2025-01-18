// main.rs
mod game;
mod gameloop;
mod ui;
mod score;

use gameloop::*;

fn main() {
    //let mut gameloop = GameLoop::new(Variant::Curtail);
    let mut gameloop = GameLoop::new(Variant::ChangeSecret);
    gameloop.run();
}
