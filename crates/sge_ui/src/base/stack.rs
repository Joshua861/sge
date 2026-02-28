use super::*;

#[derive(Debug)]
pub struct Stack {
    children: Vec<Child>,
}

impl Stack {
    pub fn new(children: impl Into<Vec<Child>>) -> UiRef {
        Self {
            children: children.into(),
        }
        .to_ref()
    }
}

impl UiNode for Stack {
    fn preferred_dimensions(&self) -> Vec2 {
        Vec2::ZERO
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        for child in &self.children {
            child.node.draw(area, ui);
        }

        area.size
    }
}
