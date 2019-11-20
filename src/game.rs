use ggez::{Context, GameResult, input, timer};
use ggez::graphics::{self, DrawMode};
use ggez::event::{EventHandler, KeyCode};
use rand::{seq::SliceRandom, thread_rng};
pub mod tetromino;
use tetromino::Piece;

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
    const ORIGIN_OFFSET: (f32, f32) = (150.0, 15.0);
    const CELL_SIZE: f32 = 20.0;

    fn new(width: u32, height: u32) -> Board
    {
        let mut v = Vec::new();
        for _i in 0..width*height
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

pub struct Game
{
    board: Board,
    active_piece: Piece,
    input_timer: f32,
    tick_timer: f32,
    tetromino_hat: [Piece; Game::NUM_OF_TETROMINOS],
    current_tetromino_index: usize,
}
impl Game
{
    const INPUT_DELAY: f32 = 0.3;
    const TICK_DELAY: f32 = 0.2;
    const NUM_OF_TETROMINOS: usize = 7;
    const NUM_OF_NEXT_PIECES: usize = 4;

    pub fn new(_context: &mut Context) -> Game
    {
        let mut hat: [Piece; Game::NUM_OF_TETROMINOS] = 
        [
            Piece::i(),
            Piece::j(),
            Piece::l(),
            Piece::o(),
            Piece::s(),
            Piece::t(),
            Piece::z()
        ];
        hat.shuffle(&mut thread_rng());
        Game 
        { 
            board: Board::new(10, 20), 
            active_piece: hat[0].clone(), 
            input_timer: 0.0, 
            tick_timer: 0.0,
            tetromino_hat: hat,
            current_tetromino_index: 0
        }
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

            //New piece
            self.current_tetromino_index = (self.current_tetromino_index + 1) % Game::NUM_OF_TETROMINOS;
            if self.current_tetromino_index == 0
            {
                self.tetromino_hat.shuffle(&mut thread_rng());
            }
            self.active_piece = self.tetromino_hat[self.current_tetromino_index].clone();

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

        //Draw board, TODO: put mesh in board, no point remaking all the time      
        let board_rect = graphics::Rect
        {
            x: Board::ORIGIN_OFFSET.0, 
            y: Board::ORIGIN_OFFSET.1, 
            w: self.board.width as f32 * (Board::CELL_SIZE + Board::CELL_SPACING) + Board::CELL_SPACING,
            h: self.board.height as f32 * (Board::CELL_SIZE + Board::CELL_SPACING) + Board::CELL_SPACING
        };
        let board_mesh = graphics::Mesh::new_rectangle(context, DrawMode::stroke(1.0), board_rect, graphics::WHITE).unwrap();
        graphics::draw(context, &board_mesh, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
        
        //Draw hold piece        
        let size = 4.0 * (Board::CELL_SIZE + Board::CELL_SPACING) + Board::CELL_SPACING;
        let hold_rect = graphics::Rect
        {
            x: board_rect.x - size,
            y: Board::ORIGIN_OFFSET.1,
            w: size,
            h: size
        };
        let hold_mesh = graphics::Mesh::new_rectangle(context, DrawMode::stroke(1.0), hold_rect, graphics::WHITE).unwrap();
        graphics::draw(context, &hold_mesh, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
        
        //Draw next pieces
        for i in 0..Game::NUM_OF_NEXT_PIECES
        {
            let hold_rect = graphics::Rect
            {
                x: board_rect.x + board_rect.w,
                y: Board::ORIGIN_OFFSET.1 + size * i as f32,
                w: size,
                h: size
            };
            let hold_mesh = graphics::Mesh::new_rectangle(context, DrawMode::stroke(1.0), hold_rect, graphics::WHITE).unwrap();
            graphics::draw(context, &hold_mesh, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;


            for point in self.tetromino_hat[(self.current_tetromino_index + i + 1) % Game::NUM_OF_NEXT_PIECES].points.iter()
            {
                //TODO: operator overloading for more clean code?
                let x_pos = hold_rect.x + Board::CELL_SPACING + point.x as f32 * (Board::CELL_SIZE + Board::CELL_SPACING);  
                let y_pos = hold_rect.y + Board::CELL_SPACING + point.y as f32 * (Board::CELL_SIZE + Board::CELL_SPACING);
                
                let rect = graphics::Rect{ x: x_pos, y: y_pos, w: Board::CELL_SIZE, h: Board::CELL_SIZE};
                let square = graphics::Mesh::new_rectangle(context, DrawMode::fill(), rect, graphics::WHITE).unwrap();
                graphics::draw(context, &square, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
            }
        }

        //Draw cells
        for y in 0..self.board.height
        {
            for x in 0..self.board.width
            {
                match self.board.cells[(x + y * self.board.width) as usize]
                {
                    Cell::Occupied => 
                    {
                        //TODO: operator overloading for more clean code?   caching rect or mesh?
                        let x_pos = Board::ORIGIN_OFFSET.0 + Board::CELL_SPACING + (x as f32 * (Board::CELL_SIZE + Board::CELL_SPACING));  
                        let y_pos = Board::ORIGIN_OFFSET.1 + Board::CELL_SPACING + (y as f32 * (Board::CELL_SIZE + Board::CELL_SPACING));  
        
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
            let x_pos = Board::ORIGIN_OFFSET.0 + Board::CELL_SPACING + ((point.x + center.x) as f32 * (Board::CELL_SIZE + Board::CELL_SPACING));  
            let y_pos = Board::ORIGIN_OFFSET.1 + Board::CELL_SPACING + ((point.y + center.y) as f32 * (Board::CELL_SIZE + Board::CELL_SPACING));
            
            let rect = graphics::Rect{ x: x_pos, y: y_pos, w: Board::CELL_SIZE, h: Board::CELL_SIZE};
            let square = graphics::Mesh::new_rectangle(context, DrawMode::fill(), rect, graphics::WHITE).unwrap();
            graphics::draw(context, &square, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
        }
        //TODO: draw ghost piece

        graphics::present(context)
    }
}