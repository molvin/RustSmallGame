use std::ops;

#[derive(Clone, Debug, Copy)]
pub struct Point
{
    pub x: i32,
    pub y: i32
}
impl Point
{
    pub fn zero() -> Point
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