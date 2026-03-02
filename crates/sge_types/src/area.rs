use bevy_math::Vec2;
use sge_window::window_size;

#[derive(Copy, Clone, Debug)]
pub struct Area {
    pub top_left: Vec2,
    pub size: Vec2,
}

impl Area {
    pub fn new(top_left: Vec2, size: Vec2) -> Self {
        Self { top_left, size }
    }

    pub fn to_rect(self) -> glium::Rect {
        let window_size = window_size();

        let bottom_y = window_size.y - (self.top_left.y + self.size.y);

        glium::Rect {
            left: self.top_left.x as u32,
            bottom: bottom_y as u32,
            width: self.size.x as u32,
            height: self.size.y as u32,
        }
    }

    pub fn top(&self) -> f32 {
        self.top_left.y
    }

    pub fn bottom(&self) -> f32 {
        self.top_left.y + self.size.y
    }

    pub fn left(&self) -> f32 {
        self.top_left.x
    }

    pub fn right(&self) -> f32 {
        self.top_left.x + self.size.x
    }

    pub fn top_left(&self) -> Vec2 {
        self.top_left
    }

    pub fn bottom_right(&self) -> Vec2 {
        self.top_left + self.size
    }

    pub fn bottom_left(&self) -> Vec2 {
        Vec2::new(self.top_left.x, self.top_left.y + self.size.y)
    }

    pub fn top_right(&self) -> Vec2 {
        Vec2::new(self.top_left.x + self.size.x, self.top_left.y)
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn center(&self) -> Vec2 {
        self.top_left + self.size / 2.0
    }

    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }
}
