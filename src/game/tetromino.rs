use crate::utility::{ Point };
use std::cmp;
use ggez::graphics::Color;
use crate::game::Board;

#[derive(Debug, Clone)]
pub struct Tetromino
{
    pub position: Point,
    pub points: [Point; 4],
    rotation_center: (f32, f32),
    pub color: Color
    
}
impl Tetromino
{
    pub fn i() -> Tetromino
    {
        Tetromino
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 0}, Point{x: 1, y: 0}, Point{x: 2, y: 0}, Point{x: 3, y: 0}],
            rotation_center: (2.0, 0.0),
            color: Color::new(0.27, 0.96, 0.95, 1.0)
        }
    }
    pub fn j() -> Tetromino
    {
        Tetromino
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 0}, Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 2, y: 1}],
            rotation_center: (1.0, 1.0),
            color: Color::new(0.18, 0.0, 0.84, 1.0)
        }
    }
    pub fn l() -> Tetromino
    {
        Tetromino
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 2, y: 1}, Point{x: 2, y: 0}],
            rotation_center: (1.0, 1.0),
            color: Color::new(0.91, 0.65, 0.05, 1.0)
        }
    }
    pub fn o() -> Tetromino
    {
        Tetromino
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 0}, Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 1, y: 0}],
            rotation_center: (0.5, 0.5),
            color: Color::new(0.92, 0.96, 0.06, 1.0)
        }
    }
    pub fn s() -> Tetromino
    {
        Tetromino
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 1, y: 0}, Point{x: 2, y: 0}],
            rotation_center: (1.0, 1.0),
            color: Color::new(0.18, 0.96, 0.0, 1.0)
        }
    }
    pub fn t() -> Tetromino
    {
        Tetromino
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 2, y: 1}, Point{x: 1, y: 0}],
            rotation_center: (1.0, 1.0),
            color: Color::new(0.63, 0.0, 0.94, 1.0)
        }
    }
    pub fn z() -> Tetromino
    {
        Tetromino
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 0}, Point{x: 1, y: 0}, Point{x: 1, y: 1}, Point{x: 2, y: 1}],
            rotation_center: (1.0, 1.0),
            color: Color::new(0.96, 0.05, 0.07, 1.0)
        }
    }
    pub fn generate_bounds(&self) -> (Point, Point)
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
    pub fn rotate(&mut self, board: &Board)
    {
        let safe_points = self.points.clone();
        let origin = self.position.clone();
        let mut direction = 1;

        for _i in 0..3
        {
            for i in 0..4
            {
                self.points[i] = Point
                {
                    x: (self.rotation_center.0 - (self.points[i].y as f32 - self.rotation_center.1)).round() as i32,
                    y: ((self.points[i].x as f32 - self.rotation_center.0) + self.rotation_center.1).round() as i32
                };
            }
            
            if board.check_collision(&self)
            {
                self.points = safe_points;
                self.position = origin + Point{x: direction, y: 0};
                direction *= -1;
                continue;   
            }
            return;
        }
        self.points = safe_points;
        self.position = origin;            
    }
}