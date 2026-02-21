use crate::ui::prelude::*;

pub struct Button;

impl Button {
    pub fn new(bg: Color, hover: Color, id: usize, child: Child) -> UiRef {
        Fit::new(HoverBoxFill::new(
            bg,
            hover,
            base::Button::new(id, Padding::xy(40.0, 10.0, child)),
        ))
    }

    pub fn primary(id: usize, child: Child) -> UiRef {
        Self::new(Color::NEUTRAL_600, Color::NEUTRAL_500, id, child)
    }

    pub fn text(bg: Color, hover: Color, id: usize, text: impl ToString) -> UiRef {
        Fit::new(ActiveFill::new(
            bg,
            hover,
            bg,
            0.0,
            base::Button::new(id, Padding::xy(40.0, 10.0, Text::no_wrap(text))),
        ))
    }

    pub fn primary_text(id: usize, text: impl ToString) -> UiRef {
        Self::text(Color::NEUTRAL_600, Color::NEUTRAL_500, id, text)
    }
}
