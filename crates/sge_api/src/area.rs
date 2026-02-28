use easy_ext::ext;
use sge_color::Color;
use sge_rendering::api::draw_texture_scaled;
use sge_textures::TextureRef;
use sge_types::Area;

use crate::shapes_2d::draw_rect;

#[ext(AreaExt)]
pub impl Area {
    fn fill(&self, color: Color) {
        draw_rect(self.top_left, self.size, color);
    }

    fn draw_texture(&self, texture: TextureRef) {
        draw_texture_scaled(texture, self.top_left, self.size);
    }
}
