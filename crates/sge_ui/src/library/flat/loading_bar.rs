use sge_color::Color;

use crate::UiRef;

pub struct LoadingBar;

impl LoadingBar {
    pub fn new(color: Color) -> UiRef {
        crate::base::LoadingBar::custom(
            color.lighten_oklch(0.07),
            color.darken_oklch(0.07),
            100.0,
            40.0,
        )
    }

    pub fn new_with_speed(color: Color, speed: f32) -> UiRef {
        crate::base::LoadingBar::custom(
            color.lighten_oklch(0.07),
            color.darken_oklch(0.07),
            speed,
            40.0,
        )
    }
}
