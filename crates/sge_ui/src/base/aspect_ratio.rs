use super::*;

#[derive(Debug)]
pub struct AspectRatio {
    ratio: f32,
    child: Child,
}

impl AspectRatio {
    pub fn new(ratio: f32, child: Child) -> UiRef {
        Self { ratio, child }.to_ref()
    }

    fn ratio(&self, dimensions: Vec2) -> Vec2 {
        let current_ratio = dimensions.x / dimensions.y;

        if current_ratio > self.ratio {
            Vec2::new(dimensions.y * self.ratio, dimensions.y)
        } else {
            Vec2::new(dimensions.x, dimensions.x / self.ratio)
        }
    }
}

impl UiNode for AspectRatio {
    fn preferred_dimensions(&self) -> Vec2 {
        self.ratio(self.child.node.preferred_dimensions())
    }

    fn draw(&self, mut area: Area, ui: &UiState) -> Vec2 {
        area.size = self.ratio(area.size).min(self.preferred_dimensions());

        self.child.node.draw(area, ui);

        area.size
    }
}
