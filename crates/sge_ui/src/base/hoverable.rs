use super::*;
use sge_time::frame_count;

pub struct Hoverable {
    id: usize,
    child: Child,
}

impl Debug for Hoverable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hoverable")
            .field("child", &self.child)
            .finish()
    }
}

impl Hoverable {
    pub fn new(id: usize, child: Child) -> UiRef {
        Self { id, child }.to_ref()
    }
}

impl UiNode for Hoverable {
    fn preferred_dimensions(&self) -> Vec2 {
        self.child.node.preferred_dimensions()
    }

    fn draw(&self, area: Area, ui: &UiState) -> Vec2 {
        if ui.is_hovered(area) {
            get_ui_storage()
                .elements_interacted
                .insert(self.id, frame_count());

            get_ui_storage()
                .elements_interacted_this_frame
                .push(self.id);
        }

        self.child.node.draw(area, ui)
    }
}

pub fn hovered_this_frame(id: usize) -> bool {
    get_ui_storage()
        .elements_interacted
        .get(&id)
        .is_some_and(|&n| n == frame_count())
}

pub fn hovered_last_frame(id: usize) -> bool {
    get_ui_storage()
        .elements_interacted
        .get(&id)
        .is_some_and(|&n| n == frame_count() - 1)
}
