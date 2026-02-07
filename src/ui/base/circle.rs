use bevy_math::vec2;

use crate::prelude::{Drawable, draw_ellipse};

use super::*;

#[derive(Debug)]
pub struct CircleFill {
    color: Color,
    child: Child,
}

impl CircleFill {
    pub fn new(color: Color) -> UiRef {
        Self {
            color,
            child: EMPTY,
        }
        .to_ref()
    }

    pub fn with_child(color: Color, child: Child) -> UiRef {
        Self { color, child }.to_ref()
    }
}

impl UiNode for CircleFill {
    fn preferred_dimensions(&self) -> Vec2 {
        vec2(50.0, 50.0)
    }

    fn draw(&self, area: Area, _: &UiState) -> Vec2 {
        crate::prelude::Circle::from_top_left(area.top_left, area.size / 2.0, self.color).draw();

        area.size
    }
}
