use bevy_math::{Mat4, Vec2, Vec3};
use glium::winit::window::Window;

#[derive(Clone, Copy, Debug)]
pub struct Camera3D {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub isometric: bool,
    window_size: Vec2,
    view_proj: Mat4,
    view_matrix: Mat4,
    proj_matrix: Mat4,
    needs_update: bool,
}

impl Camera3D {
    pub fn new(window_width: u32, window_height: u32) -> Self {
        let mut camera = Self {
            eye: Vec3::ZERO,
            target: Vec3::NEG_Z,
            up: Vec3::Y,
            window_size: Vec2::new(window_width as f32, window_height as f32),
            fovy: 100.0,
            znear: 0.1,
            zfar: 1000.0,
            isometric: false,
            needs_update: true,
            view_proj: Mat4::ZERO,
            view_matrix: Mat4::ZERO,
            proj_matrix: Mat4::ZERO,
        };
        camera.update_matrices();
        camera
    }

    /// Does nothing if self.needs_update is false
    pub fn update_matrices(&mut self) {
        if !self.needs_update {
            return;
        }
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);

        let proj = if self.isometric {
            let distance = (self.eye - self.target).length();
            let height = distance * (self.fovy.to_radians() / 2.0).tan();
            let width = height * self.window_aspect_ratio();

            Mat4::orthographic_rh(-width, width, -height, height, self.znear, self.zfar)
        } else {
            Mat4::perspective_rh(
                self.fovy.to_radians(),
                self.window_aspect_ratio(),
                self.znear,
                self.zfar,
            )
        };

        self.view_matrix = view;
        self.proj_matrix = proj;
        self.view_proj = proj * view;
        self.needs_update = false;
    }

    pub fn view_proj(&mut self) -> Mat4 {
        self.update_matrices();
        self.view_proj
    }

    pub fn window_aspect_ratio(&self) -> f32 {
        self.window_size.x / self.window_size.y
    }

    pub fn from_window(window: &Window) -> Self {
        let size = window.inner_size();
        Self::new(size.width, size.height)
    }

    pub fn mark_dirty(&mut self) {
        self.needs_update = true;
    }

    pub fn update_sizes(&mut self, window_width: u32, window_height: u32) {
        self.window_size = Vec2::new(window_width as f32, window_height as f32);
        self.needs_update = true;
    }
}
