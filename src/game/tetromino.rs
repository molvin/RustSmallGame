use crate::utility::{ Point };
use std::cmp;

#[derive(Debug, Clone)]
pub struct Piece
{
    pub position: Point,
    pub points: [Point; 4],
    rotation_center: (f32, f32)
}
impl Piece
{
    pub fn i() -> Piece
    {
        Piece
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 2, y: 1}, Point{x: 3, y: 1}],
            rotation_center: (2.0, 1.0)
        }
    }
    pub fn j() -> Piece
    {
        Piece
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 0}, Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 2, y: 1}],
            rotation_center: (1.0, 1.0)
        }
    }
    pub fn l() -> Piece
    {
        Piece
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 2, y: 1}, Point{x: 2, y: 0}],
            rotation_center: (1.0, 1.0)
        }
    }
    pub fn o() -> Piece
    {
        Piece
        {
            position: Point::zero(),
            points: [Point{x: 1, y: 0}, Point{x: 1, y: 1}, Point{x: 2, y: 1}, Point{x: 2, y: 0}],
            rotation_center: (1.5, 0.5)
        }
    }
    pub fn s() -> Piece
    {
        Piece
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 1, y: 0}, Point{x: 2, y: 0}],
            rotation_center: (1.0, 1.0)
        }
    }
    pub fn t() -> Piece
    {
        Piece
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 2, y: 1}, Point{x: 1, y: 0}],
            rotation_center: (1.0, 1.0)
        }
    }
    pub fn z() -> Piece
    {
        Piece
        {
            position: Point::zero(),
            points: [Point{x: 0, y: 0}, Point{x: 1, y: 0}, Point{x: 1, y: 1}, Point{x: 2, y: 1}],
            rotation_center: (1.0, 1.0)
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
    pub fn rotate(&mut self)
    {
        let angle : f32 = std::f32::consts::PI * 0.5;
        let c = angle.cos();    // 0
        let s = angle.sin();    // 1
        for i in 0..4
        {
            let (cx, cy) = self.rotation_center;
            let (px, py) = (self.points[i].x as f32 - cx, self.points[i].y as f32 - cy);
            let rotated_x = ((px * c - py * s) + cx).round() as i32;
            let rotated_y = ((px * s + py * c) + cy).round() as i32;
            self.points[i] = Point{x: rotated_x, y: rotated_y};
            println!("index: {}, before: {}, {}, after: {}, {}, rotation center: {}, {}", i, px + cx, py + cy, rotated_x, rotated_y, cx, cy);
        }
    }
}