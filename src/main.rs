// main.rs
mod game;
mod gameloop;
mod ui;

use gameloop::*;

fn main() {
    let mut gameloop = GameLoop::new_buffer();
    //let mut gameloop = GameLoop::new(Variant::ChangeSecret);
    gameloop.run();
}
