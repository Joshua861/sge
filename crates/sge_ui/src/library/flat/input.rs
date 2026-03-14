use sge_text::SANS;

use crate::prelude::*;

use super::PRIMARY_TEXT_COLOR;

pub struct TextInput;

impl TextInput {
    pub fn new(bg: Color, id: usize) -> UiRef {
        Fit::new(BoxFill::new(
            bg,
            base::TextInput::new(id, None, Some(SANS), 16, PRIMARY_TEXT_COLOR, 10.0, true),
        ))
    }

    pub fn with_prompt(bg: Color, prompt: impl ToString, id: usize) -> UiRef {
        Fit::new(BoxFill::new(
            bg,
            base::TextInput::new(
                id,
                Some(prompt.to_string()),
                Some(SANS),
                16,
                PRIMARY_TEXT_COLOR,
                10.0,
                true,
            ),
        ))
    }
}
