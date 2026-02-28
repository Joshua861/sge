// use glium::winit::keyboard::{Key, NamedKey};

// use crate::{input::input_text, prelude::FontRef};

// use super::*;

// #[derive(Debug)]
// pub struct Input {
//     state: State<Data>,

//     prompt: Option<String>,
//     font: FontRef,
//     font_size: usize,
//     color: Color,
//     /// scale the font size by the DPI scaling of your monitor
//     do_dpi_scaling: bool,
// }

// #[derive(Default, Debug)]
// struct Data {
//     value: String,
//     is_active: bool,
// }

// impl UiNode for Input {
//     fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
//         let state = self.state.get_or_default();

//         if state.is_active {
//             for key in input_text() {
//                 if let Key::Character(s) = key {
//                     state.value.push_str(s.as_str());
//                 }

//                 if let Key::Named(NamedKey::Space) = key {
//                     state.value.push(' ');
//                 }

//                 if let Key::Named(NamedKey::Enter) = key {
//                     state.value.push('\n');
//                 }

//                 if let Key::Named(NamedKey::Backspace) = key {
//                     state.value.pop();
//                 }
//             }
//         }

//         let text = if state.value.is_empty()
//             && let Some(prompt) = self.prompt
//         {
//             prompt
//         } else {
//             state.value
//         };

//         Text::no_wrap(text)
//     }
// }
