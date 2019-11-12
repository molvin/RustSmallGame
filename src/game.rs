use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawMode};
use ggez::event::EventHandler;
use ggez::event::KeyCode;
use ggez::input;
use ggez::timer;

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
    const CELL_SPACING: f32 = 3.0;
    const ORIGIN_OFFSET: f32 = 10.0;
    const CELL_SIZE: f32 = 20.0;

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
    position: (i32, i32),
    points: [(i32, i32); 4],
    top_left: (i32, i32),
    bot_right: (i32, i32)
}
impl Piece
{
    fn I() -> Piece
    {
        Piece
        {
            position: (0,0),
            points: [(0, 2), (0, 1), (0, 0), (0, -1)],
            top_left: (0, -1),
            bot_right: (0, 2)
        }
    }
    fn L() -> Piece
    {
        Piece
        {
            position: (0,0),
            points: [(0,-1), (0,0), (0, 1), (1,1)],
            top_left: (0, -1),
            bot_right: (1,1)

        }
    }

    
    fn top_left(&self) -> (i32, i32)
    {
        (self.top_left.0 + self.position.0 as i32, self.top_left.1 + self.position.1 as i32)
    }
    fn bot_right(&self) -> (i32, i32)
    {
        (self.bot_right.0 + self.position.0 as i32, self.bot_right.1 + self.position.1 as i32)
    }
}

pub struct Game
{
    board: Board,
    active_piece: Piece,
    input_timer: f32,
    tick_timer: f32,
}
impl Game
{
    const INPUT_DELAY: f32 = 0.3;
    const TICK_DELAY: f32 = 0.5;

    pub fn new(context: &mut Context) -> Game
    {
        Game { board: Board::new(10, 20), active_piece: Piece::L(), input_timer: 0.0, tick_timer: 0.0 }
    }
}
impl EventHandler for Game
{
    fn update(&mut self, context: &mut Context) -> GameResult<()>
    {
        let delta_time = timer::delta(context).as_secs_f32();
        self.input_timer += delta_time;
        self.tick_timer += delta_time;

        //Tick
        if self.tick_timer > Game::TICK_DELAY
        {
            self.tick_timer -= Game::TICK_DELAY;
            
            self.active_piece.position.1 += 1;
        }

        //Input
        if input::keyboard::is_key_pressed(context, KeyCode::D) && self.input_timer > Game::INPUT_DELAY
        {
            self.active_piece.position.0 += 1;
            self.input_timer = 0.0;
        }
        if input::keyboard::is_key_pressed(context, KeyCode::A) && self.input_timer > Game::INPUT_DELAY
        {
            self.active_piece.position.0 -= 1;
            self.input_timer = 0.0;
        }

        //Collision
        let x = self.active_piece.top_left().0;
        if x < 0
        {
            self.active_piece.position.0 += -x;
        }
        let x = self.active_piece.bot_right().0;
        if x >= (self.board.width) as i32
        {
            self.active_piece.position.0 -= x - (self.board.width as i32 - 1);
        }

        Ok(())
    }
    fn draw(&mut self, context: &mut Context) -> GameResult<()>
    {
        graphics::clear(context, graphics::BLACK);

        //Draw board      
        let board_rect = graphics::Rect
        {
            x: Board::ORIGIN_OFFSET, 
            y: Board::ORIGIN_OFFSET, 
            w: self.board.width as f32 * (Board::CELL_SIZE + Board::CELL_SPACING) + Board::CELL_SPACING,
            h: self.board.height as f32 * (Board::CELL_SIZE + Board::CELL_SPACING) + Board::CELL_SPACING
        };
        let board_mesh = graphics::Mesh::new_rectangle(context, DrawMode::stroke(1.0), board_rect, graphics::WHITE).unwrap();
        graphics::draw(context, &board_mesh, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;

        //Draw cells
        for y in 0..self.board.height
        {
            for x in 0..self.board.width
            {
                match self.board.cells[(x + y * self.board.width) as usize]
                {
                    Cell::Occupied => 
                    {
                        let x_pos = Board::ORIGIN_OFFSET + Board::CELL_SPACING + (x as f32 * (Board::CELL_SIZE + Board::CELL_SPACING));  
                        let y_pos = Board::ORIGIN_OFFSET + Board::CELL_SPACING + (y as f32 * (Board::CELL_SIZE + Board::CELL_SPACING));  
        
                        let rect = graphics::Rect{ x: x_pos, y: y_pos, w: Board::CELL_SIZE, h: Board::CELL_SIZE};
                        let square = graphics::Mesh::new_rectangle(context, DrawMode::fill(), rect, graphics::WHITE).unwrap();
                        graphics::draw(context, &square, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
                    }
                    _ => continue
                }                
            }
        }
        //TODO: draw active piece
        for (x,y) in self.active_piece.points.iter()
        {
            let (cx, cy) = self.active_piece.position;
            let x_pos = Board::ORIGIN_OFFSET + Board::CELL_SPACING + ((*x + cx as i32) as f32 * (Board::CELL_SIZE + Board::CELL_SPACING));  
            let y_pos = Board::ORIGIN_OFFSET + Board::CELL_SPACING + ((*y + cy as i32) as f32 * (Board::CELL_SIZE + Board::CELL_SPACING));
            
            let rect = graphics::Rect{ x: x_pos, y: y_pos, w: Board::CELL_SIZE, h: Board::CELL_SIZE};
            let square = graphics::Mesh::new_rectangle(context, DrawMode::fill(), rect, graphics::WHITE).unwrap();
            graphics::draw(context, &square, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
        }
        //TODO: draw ghost piece

        graphics::present(context)
    }
}