use sge_shapes::d2::Shape2D;
use shapes_2d::{Shape2DExt, draw_shape, draw_shape_world};

pub mod area;
pub mod shapes_2d;

pub trait Drawable {
    fn draw(&self);
    fn draw_word(&self);
}

impl<T: Shape2D + Shape2DExt> Drawable for T {
    fn draw(&self) {
        draw_shape(self);
    }

    fn draw_word(&self) {
        draw_shape_world(self);
    }
}

pub fn draw<T: Drawable>(o: T) {
    o.draw();
}

pub fn draw_world<T: Drawable>(o: T) {
    o.draw_word();
}
