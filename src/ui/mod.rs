use crate::{get_state, input::Input, prelude::TextureRef, utils::EngineCreate};
use engine_4_macros::gen_ref_type;
use engine_color::Color;
use error_union::Union;

pub struct SomeNode {
    node: Box<dyn UiNode>,
}

gen_ref_type!(SomeNode, UiRef, ui_nodes);

pub trait UiNode {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn draw(&mut self);
    fn update(&mut self, info: UiInfo);
}

pub trait ToGenericUiNode: UiNode + Sized {
    fn to_generic(self) -> SomeNode;
    fn to_ref(self) -> UiRef {
        self.to_generic().create()
    }
}

impl<T: UiNode + 'static> ToGenericUiNode for T {
    fn to_generic(self) -> SomeNode {
        SomeNode {
            node: Box::new(self),
        }
    }
}

#[derive(Union)]
pub enum UiContents {
    Texture(TextureRef),
}

pub struct Button {
    color: Color,
    hover_color: Color,
    held_color: Color,
    contents: UiContents,
}

pub struct UiInfo {
    pub input: Input,
}

pub struct Ui {
    root: UiRef,
}
