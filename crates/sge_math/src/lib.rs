pub mod collision;
pub mod transform;
pub mod usize_rect;

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
