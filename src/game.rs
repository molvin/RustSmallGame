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
    origin: (i32, i32),
    cell_size: (u32, u32),
    buffer: (u32, u32)
}
impl Board
{
    fn new(width: u32, height: u32, origin: (i32, i32), cell_size: (u32, u32), buffer: (u32, u32)) -> Board
    {
        let mut v = Vec::new();
        for i in 0..width*height
        {
            v.push(Cell::Empty);
        }
        Board{width: width, height: height, cells: v, origin: origin, cell_size: cell_size, buffer: buffer}
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
            points: [(0, 2), (0, 1), (0, 0), (0, -1)]
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
        Game { board: Board::new(10, 20, (10,10), (10, 10), (1, 1)), active_piece: Piece::i() }
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
        fn get_draw_position(board: &mut Board, x: u32, y: u32) -> (f32, f32)
        {
            ((board.origin.0 + x as i32 * board.cell_size.0 as i32 + x as i32 * board.buffer.0 as i32) as f32, 
            (board.origin.1 +  y as i32 * board.cell_size.1 as i32 + y as i32 * board.buffer.1 as i32) as f32)
        }

        graphics::clear(context, graphics::BLACK);

        self.active_piece.position = (0, 1);
        //Draw piece
        for p in self.active_piece.points.iter()
        {
            let (x,y) = get_draw_position(&mut self.board, (p.0 + self.active_piece.position.0 as i32) as u32, (p.1 + self.active_piece.position.1 as i32) as u32);
            let rectangle = graphics::Mesh::new_rectangle(
                context,
                DrawMode::fill(),
                graphics::Rect {
                    x: x,
                    y: y,
                    w: 10.0,
                    h: 10.0,
                },
                graphics::WHITE,
                )?;
                graphics::draw(context, &rectangle, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
        }

        //Draw board
        let count = (self.board.width * self.board.height);
        for i in 0..count
        {
            match self.board.cells[i as usize]
            {
                Cell::Empty => { continue; }
                _ => {

                    let (xi, yi) = ((i % self.board.height), (i / self.board.width)); 
                    let (x, y) = get_draw_position(&mut self.board, xi, yi);

                    let rectangle = graphics::Mesh::new_rectangle(
                        context,
                        DrawMode::fill(),
                        graphics::Rect {
                            x: x as f32,
                            y: y as f32,
                            w: 10.0,
                            h: 10.0,
                        },
                        graphics::WHITE,
                        )?;
                        graphics::draw(context, &rectangle, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
                 }
            }          
        }

        graphics::present(context)
    }
}