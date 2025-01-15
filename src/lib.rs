pub fn init(variant: String) {
    //let mut gameloop = GameLoop::new(Variant::Curtail);
    let mut gameloop = GameLoop::new(Variant::ChangeSecret);
    gameloop.run();
}