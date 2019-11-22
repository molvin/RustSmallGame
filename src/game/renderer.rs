use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawMode, Color};
use crate::utility::Point;

pub struct Renderer
{

}
impl Renderer
{
    //TODO: cache mesh? or rect?
    //TODO: replace f32 tuple with struct for operator overloading
    pub fn draw_tetromino(context: &mut Context, points: &[Point; 4], origin: (f32, f32), position: (f32, f32), cell_size: f32, cell_spacing: f32, color: Color) -> GameResult<()>
    {
        for point in points.iter()
        {
            let x_pos = origin.0 + cell_spacing + point.x as f32 * (cell_size + cell_spacing);
            let y_pos = origin.1 + cell_spacing + point.y as f32 * (cell_size + cell_spacing);
            let rect = graphics::Rect{ x: x_pos, y: y_pos, w: cell_size, h: cell_size};
            let square = graphics::Mesh::new_rectangle(context, DrawMode::fill(), rect, color).unwrap();
            graphics::draw(context, &square, (ggez::nalgebra::Point2::new(position.0, position.1) * (cell_size + cell_spacing),))?;
        }
        Ok(())
    }
    pub fn draw_frame(context: &mut Context, position: (f32, f32), size: (f32, f32)) -> GameResult<()>
    {
        let hold_rect = graphics::Rect
            {
                x: position.0,
                y: position.1,
                w: size.0,
                h: size.1
            };
            let hold_mesh = graphics::Mesh::new_rectangle(context, DrawMode::stroke(1.0), hold_rect, graphics::WHITE).unwrap();
            graphics::draw(context, &hold_mesh, (ggez::nalgebra::Point2::new(0.0, 0.0),))
    }
}