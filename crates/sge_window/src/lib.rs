#![allow(deprecated)]

use std::num::NonZeroU32;

use bevy_math::Vec2;
use error_union::ErrorUnion;
use glium::{
    backend::glutin::Display,
    glutin::{
        config::ConfigTemplateBuilder,
        context::ContextAttributesBuilder,
        display::GetGlDisplay,
        prelude::*,
        surface::{SurfaceAttributesBuilder, SwapInterval, WindowSurface},
    },
    winit::{
        dpi::PhysicalSize,
        event_loop::EventLoop,
        raw_window_handle::HasRawWindowHandle,
        window::{Window, WindowAttributes},
    },
};
use glutin_winit::{DisplayBuilder, GlWindow};

pub type SgeDisplay = Display<WindowSurface>;

pub struct WindowState {
    pub window: Window,
    pub display: SgeDisplay,
    pub event_loop: EventLoop<()>,
    pub window_size: Vec2,
}

global::global!(WindowState, window_state);

pub struct WindowOptions {
    pub template: ConfigTemplateBuilder,
    pub surface_attributes: SurfaceAttributesBuilder<WindowSurface>,
    pub context_attributes: ContextAttributesBuilder,
    pub swap_interval: SwapInterval,
    pub window_attributes: WindowAttributes,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            template: ConfigTemplateBuilder::new().with_multisampling(4),
            surface_attributes: Default::default(),
            context_attributes: ContextAttributesBuilder::new(),
            // swap_interval: SwapInterval::DontWait,
            swap_interval: SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
            window_attributes: Window::default_attributes().with_transparent(true),
        }
    }
}

#[derive(ErrorUnion, Debug)]
pub enum WindowCreationError {
    EventLoop(glium::winit::error::EventLoopError),
    Handle(glium::winit::raw_window_handle::HandleError),
    Glutin(glium::glutin::error::Error),
    IncompatibleOpenGl(glium::IncompatibleOpenGl),
}

pub fn init(opts: WindowOptions) -> Result<(), WindowCreationError> {
    let event_loop = EventLoop::builder().build()?;

    let window_attributes = opts.window_attributes;

    let (window, gl_config) = DisplayBuilder::new()
        .with_preference(glutin_winit::ApiPreference::FallbackEgl)
        .with_window_attributes(Some(window_attributes))
        .build(&event_loop, opts.template, |configs| {
            // Find the config with the maximum number of samples
            configs
                .reduce(|accum, config| {
                    if config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

    let window = window.unwrap();

    let context_attributes = opts
        .context_attributes
        .build(Some(window.raw_window_handle()?));

    let gl_display = gl_config.display();
    let not_current_context =
        unsafe { gl_display.create_context(&gl_config, &context_attributes)? };

    let surface_attributes = window
        .build_surface_attributes(opts.surface_attributes)
        .expect("Failed to build surface attributes");

    let gl_surface = unsafe { gl_display.create_window_surface(&gl_config, &surface_attributes)? };

    let gl_context = not_current_context.make_current(&gl_surface)?;

    gl_surface
        .set_swap_interval(&gl_context, opts.swap_interval)
        .expect("Failed to set swap interval");

    let display = Display::from_context_surface(gl_context, gl_surface)?;

    let state = WindowState {
        window_size: physical_size_to_vec2(window.inner_size()),
        display,
        window,
        event_loop,
    };

    set_window_state(state);

    Ok(())
}

fn physical_size_to_vec2(size: PhysicalSize<u32>) -> Vec2 {
    Vec2::new(size.width as f32, size.height as f32)
}

pub fn window_size() -> Vec2 {
    get_window_state().window_size
}

pub fn window_center() -> Vec2 {
    window_size() / 2.0
}

pub fn window_size_u32() -> PhysicalSize<u32> {
    get_window_state().window.inner_size()
}

pub fn window_height() -> f32 {
    get_window_state().window_size.y
}

pub fn window_width() -> f32 {
    get_window_state().window_size.x
}

pub fn handle_window_resize(size: PhysicalSize<u32>) {
    let window_state = get_window_state();
    window_state.display.resize(size.into());
    let size = window_state.window.inner_size();
    window_state.window_size = physical_size_to_vec2(size);
}

pub fn dpi_scaling() -> f32 {
    get_window_state().window.scale_factor() as f32
}

pub fn get_display() -> &'static SgeDisplay {
    &get_window_state().display
}

pub fn get_display_mut() -> &'static mut SgeDisplay {
    &mut get_window_state().display
}

pub fn max_window_dimension() -> f32 {
    window_height().max(window_width())
}

pub fn min_window_dimension() -> f32 {
    window_height().min(window_width())
}
