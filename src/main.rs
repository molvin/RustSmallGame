use ggez::ContextBuilder;
use ggez::event;

mod game;

fn main() {
    
    let window_setup = ggez::conf::WindowSetup::default().title("Tetris");
    let window_mode = ggez::conf::WindowMode::default().resizable(true);
    let (mut context, mut event_loop) = ContextBuilder::new("game", "molvin")
    .window_setup(window_setup)
    .window_mode(window_mode)
    .build().expect("failed to create ggez context");
    
    let mut game = game::Game::new(&mut context);
    
    match event::run(&mut context, &mut event_loop, &mut game)
    {
        Ok(_) => println!("Exited cleanly"),
        Err(e) => println!("Error: {}", e)
    }
}
