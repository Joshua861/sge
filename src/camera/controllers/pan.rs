use crate::prelude::*;

pub struct PanningCameraController {
    pub allow_panning: bool,
    pub allow_zooming: bool,
    pub pan_button: Button,
}

impl PanningCameraController {
    pub fn update(&mut self) {
        if self.allow_panning && button_held(self.pan_button) {
            let diff: Vec2 = mouse_diff();

            mutate_camera_2d(|camera| {
                camera.translation -= diff / camera.scale;
            });
        }

        if self.allow_zooming && scroll_diff().y != 0.0 {
            let diff = scroll_diff().y;
            let diff = (diff * 0.1) + 1.0;

            camera2d_zoom_at(cursor_pos(), diff);
        }
    }

    pub fn new() -> Self {
        Self {
            allow_panning: true,
            allow_zooming: true,
            pan_button: MouseButton::Left.into(),
        }
    }

    pub fn set_pan_button(&mut self, pan_button: impl Into<Button>) {
        self.pan_button = pan_button.into();
    }
}

impl Default for PanningCameraController {
    fn default() -> Self {
        Self::new()
    }
}
