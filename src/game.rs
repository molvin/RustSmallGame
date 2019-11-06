use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, DrawMode};
use ggez::event::{self, EventHandler};
use ggez::event::{Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};
use ggez::input;

enum Cell
{
    Empty,
    Occupied
}
struct Board
{
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}
impl Board
{
    fn new(width: u32, height: u32) -> Board
    {
        let mut v = Vec::new();
        for i in 0..width*height
        {
            v.push(Cell::Empty);
        }
        Board{width: width, height: height, cells: v}
    }
}
struct Piece
{
    position: (u32, u32),
    points: [(i32, i32); 4],
}
impl Piece
{
    fn i() -> Piece
    {
        Piece
        {
            position: (0,0),
            points: [(0, 2), (0, 1), (0, 0), (-1, 0)]
        }
    }
}

pub struct Game
{
    board: Board,
    active_piece: Piece,
}
impl Game
{
    pub fn new(context: &mut Context) -> Game
    {
        Game { board: Board::new(10, 20), active_piece: Piece::i() }
    }
}
impl EventHandler for Game
{
    fn update(&mut self, context: &mut Context) -> GameResult<()>
    {
        let mut v = vec![vec![0; 10]; 10];

        if input::keyboard::is_key_pressed(context, KeyCode::Q)
        {
            println!("Q pressed");
        }
        Ok(())
    }
    fn draw(&mut self, context: &mut Context) -> GameResult<()>
    {
        graphics::clear(context, graphics::BLACK);

        let rectangle = graphics::Mesh::new_rectangle(
            context,
            DrawMode::fill(),
            graphics::Rect {
                x: 10.0,
                y: 10.0,
                w: 400.0,
                h: 300.0,
            },
            graphics::WHITE,
        )?;
        graphics::draw(context, &rectangle, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;

        graphics::present(context)
    }
}