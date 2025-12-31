#![allow(deprecated)]

use std::num::NonZeroU32;

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
        event_loop::EventLoop,
        raw_window_handle::HasRawWindowHandle,
        window::{Window, WindowAttributes},
    },
};
use glutin_winit::{DisplayBuilder, GlWindow};

use crate::EngineDisplay;

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

pub(crate) fn init_window(
    opts: WindowOptions,
) -> anyhow::Result<(Window, EngineDisplay, EventLoop<()>)> {
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

    Ok((window, display, event_loop))
}
