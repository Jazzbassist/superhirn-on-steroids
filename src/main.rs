// main.rs
mod game;
mod gameloop;
mod ui;

use gameloop::*;

fn main() {
    GameLoop::struct_game_loop()
}
