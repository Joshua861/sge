use sge_color::Color;

pub const BUTTON_COLOR: Color = Color::NEUTRAL_600;
pub const BUTTON_HOVER_COLOR: Color = Color::NEUTRAL_500;
pub const PRIMARY_TEXT_COLOR: Color = Color::NEUTRAL_200;
pub const BG0: Color = Color::NEUTRAL_900;
pub const BG1: Color = Color::NEUTRAL_800;
pub const BG2: Color = Color::NEUTRAL_700;
pub const BG3: Color = Color::NEUTRAL_600;
pub const BG4: Color = Color::NEUTRAL_500;

pub mod progress_bar;
pub use progress_bar::*;

pub mod button;
pub use button::*;

pub mod card;
pub use card::*;

pub mod loading_bar;
pub use loading_bar::*;
