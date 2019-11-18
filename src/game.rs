use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawMode};
use ggez::event::{EventHandler, KeyCode};
use ggez::{input, timer};
use std::{cmp, ops};

///TODO
/// Make all pieces
/// Random hat pieces
/// Space to drop
/// Down to reduce time between ticks
/// Colors for pieces
/// Colors for cells
/// Rotation
/// Kicking
/// Clearing
/// Score
/// Holding 
/// Show next pieces

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
            //v.push(if i < width * (height - 1) { Cell::Empty } else { Cell::Occupied });
            v.push(Cell::Empty);
        }

        Board{width: width, height: height, cells: v}
    }
    fn check_collision(&self, piece: &Piece) -> bool
    {   
        let (tl, br) = piece.generate_bounds();
        if tl.x < 0 || br.x >= (self.width as i32) || br.y >= (self.height as i32)        
        {
            return true;
        }

        for point in piece.points.iter()
        {
            let index = point.x + piece.position.x + ((point.y + piece.position.y) * self.width as i32);
            if index < 0
            {
                continue;
            }
            match self.cells[index as usize]
            {
                Cell::Occupied => { return true; }
                _=> continue
            }
        }
        false
    }

}
#[derive(Clone, Debug, Copy)]
struct Point
{
    x: i32,
    y: i32
}
impl Point
{
    fn zero() -> Point
    {
        Point { x: 0, y: 0}
    }
}
impl ops::Add<Point> for Point
{
    type Output = Point;

    fn add(self, rhs: Point) -> Point
    {
        Point { x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

#[derive(Debug)]
struct Piece
{
    position: Point,
    points: [Point; 4],
    rotation_center: (f32, f32)
    //TODO: don't need top_left and top_right, just index into points array
}
impl Piece
{
    fn i() -> Piece
    {
        Piece
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 2, y: 1}, Point{x: 3, y: 1}],
            rotation_center: (2.0, 1.0)
        }
    }
    fn j() -> Piece
    {
        Piece
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 0}, Point{x: 1, y: 0}, Point{x: 2, y: 0}, Point{x: 3, y: 0}],
            rotation_center: (0.0, 0.0)
        }
    }
    fn l() -> Piece
    {
        Piece
        {
            position: Point::zero(),
            points: [Point{x: 0, y: -1}, Point{x: 0, y: 0}, Point{x: 0, y: 1}, Point{x: 1, y: 1}],
            rotation_center: (0.0, 0.0)
        }
    }
    


    fn generate_bounds(&self) -> (Point, Point)
    {
        let mut top_left = Point{x: 100000, y: 100000};
        let mut bot_right = Point{x: -100000, y: -100000};

        for i in 0..4
        {
            top_left.x = cmp::min(self.points[i].x, top_left.x);
            bot_right.x = cmp::max(self.points[i].x, bot_right.x);          
            top_left.y = cmp::min(self.points[i].y, top_left.y);
            bot_right.y = cmp::max(self.points[i].y, bot_right.y);
        }

        (top_left + self.position, bot_right + self.position)
    }
    fn rotate(&mut self)
    {
        let angle : f32 = std::f32::consts::PI * 0.5;
        let c = angle.cos();
        let s = angle.sin();
        for i in 0..4
        {
            let (cx, cy) = self.rotation_center;
            let (px, py) = (self.points[i].x as f32 - cx, self.points[i].y as f32 - cy);
            let rotated_x = ((px * c - py * s) + cx).round() as i32;
            let rotated_y = ((px * s + py * c) + cy).round() as i32;
            self.points[i] = Point{x: rotated_x, y: rotated_y};
            println!("index: {}, before: {}, {}, after: {}, {}", i, px, py, rotated_x, rotated_y);
        }
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
        Game { board: Board::new(10, 20), active_piece: Piece::i(), input_timer: 0.0, tick_timer: 0.0 }
    }   
}
impl EventHandler for Game
{
    fn update(&mut self, context: &mut Context) -> GameResult<()>
    {
        let delta_time = timer::delta(context).as_secs_f32();
        self.input_timer += delta_time;
        self.tick_timer += delta_time;

        let previous_position = self.active_piece.position.clone();
        //Tick
        if self.tick_timer > Game::TICK_DELAY
        {
            self.tick_timer -= Game::TICK_DELAY;
            
            self.active_piece.position.y += 1;
        }
        if self.board.check_collision(&self.active_piece)
        {
            self.active_piece.position = previous_position;
            for point in self.active_piece.points.iter()
            {
                let index = (point.x + self.active_piece.position.x + ((point.y + self.active_piece.position.y) * self.board.width as i32)) as usize;
                self.board.cells[index] = Cell::Occupied;
            }

            self.active_piece = Piece::l();

            return Ok(())
        }    
        
        let previous_position = self.active_piece.position.clone();
        //Input, TODO: allow multiple inputs ? also get key down, input system
        if input::keyboard::is_key_pressed(context, KeyCode::D) && self.input_timer > Game::INPUT_DELAY
        {
            self.active_piece.position.x += 1;
            self.input_timer = 0.0;
        }
        if input::keyboard::is_key_pressed(context, KeyCode::A) && self.input_timer > Game::INPUT_DELAY
        {
            self.active_piece.position.x -= 1;
            self.input_timer = 0.0;
        }
        if input::keyboard::is_key_pressed(context, KeyCode::W) && self.input_timer > Game::INPUT_DELAY
        {
            self.active_piece.rotate();
            self.input_timer = 0.0;
        }
        //Collision
        if self.board.check_collision(&self.active_piece)
        {
            self.active_piece.position = previous_position;
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
                        //TODO: operator overloading for more clean code?
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
        //Draw active piece
        for point in self.active_piece.points.iter()
        {
            //TODO: operator overloading for more clean code?
            let center = &self.active_piece.position;
            let x_pos = Board::ORIGIN_OFFSET + Board::CELL_SPACING + ((point.x + center.x) as f32 * (Board::CELL_SIZE + Board::CELL_SPACING));  
            let y_pos = Board::ORIGIN_OFFSET + Board::CELL_SPACING + ((point.y + center.y) as f32 * (Board::CELL_SIZE + Board::CELL_SPACING));
            
            let rect = graphics::Rect{ x: x_pos, y: y_pos, w: Board::CELL_SIZE, h: Board::CELL_SIZE};
            let square = graphics::Mesh::new_rectangle(context, DrawMode::fill(), rect, graphics::WHITE).unwrap();
            graphics::draw(context, &square, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
        }
        //TODO: draw ghost piece

        graphics::present(context)
    }
}