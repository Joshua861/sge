use sge_shapes::d2::Shape2D;
use shapes_2d::{Shape2DExt, draw_shape};

pub mod area;
pub mod shapes_2d;

pub trait Drawable {
    fn draw(&self);
}

impl<T: Shape2D + Shape2DExt> Drawable for T {
    fn draw(&self) {
        draw_shape(self);
    }
}

pub fn draw<T: Drawable>(o: T) {
    o.draw();
}
