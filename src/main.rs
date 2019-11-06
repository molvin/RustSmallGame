use ggez::ContextBuilder;
use ggez::event;

mod game;

fn main() {
    
    let (mut context, mut event_loop) = ContextBuilder::new("game", "molvin").build().expect("failed to create ggez context");
    
    let mut game = game::Game::new(&mut context);
    
    match event::run(&mut context, &mut event_loop, &mut game)
    {
        Ok(_) => println!("Exited cleanly"),
        Err(e) => println!("Error: {}", e)
    }
}
