use bevy_math::Vec2;
use glium::implement_vertex;
use sge_color::Color;

use crate::Vertex3D;

// ////////////////////////////////////////////////////////////////////////////
//                                  Rounded                                  //
///////////////////////////////////////////////////////////////////////////////

pub struct RoundedBatch {
    pub instances: Vec<RoundedInstance>,
    pub scissor: Option<glium::Rect>,
}

implement_vertex!(
    RoundedInstance,
    dimensions,
    center,
    corner_radius,
    outline_thickness,
    fill_color,
    outline_color
);
#[derive(Copy, Clone, Debug)]
pub struct RoundedInstance {
    pub dimensions: [f32; 2],
    pub center: [f32; 3],
    pub corner_radius: f32,
    pub outline_thickness: f32,
    pub fill_color: [f32; 4],
    pub outline_color: [f32; 4],
}

impl RoundedInstance {
    pub fn new(
        dimensions: Vec2,
        center: Vec2,
        z: f32,
        corner_radius: f32,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self {
            dimensions: dimensions.into(),
            center: [center.x, center.y, z],
            corner_radius,
            outline_thickness,
            fill_color: fill_color.for_gpu(),
            outline_color: outline_color.for_gpu(),
        }
    }
}

impl RoundedBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

// ////////////////////////////////////////////////////////////////////////////
//                                   Circle                                  //
///////////////////////////////////////////////////////////////////////////////

pub struct CircleBatch {
    pub instances: Vec<CircleInstance>,
    pub scissor: Option<glium::Rect>,
}

impl CircleBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            instances: Vec::new(),
            scissor,
        }
    }
}

implement_vertex!(
    CircleInstance,
    center,
    radius,
    fill_color,
    outline_thickness,
    outline_color
);
#[derive(Copy, Clone, Debug)]
pub struct CircleInstance {
    pub center: [f32; 3],
    pub radius: [f32; 2],
    pub fill_color: [f32; 4],
    pub outline_thickness: f32,
    pub outline_color: [f32; 4],
}

impl CircleInstance {
    pub fn new(center: Vec2, z: f32, radius: Vec2, fill_color: Color) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: radius.into(),
            fill_color: fill_color.for_gpu(),
            outline_thickness: 0.0,
            outline_color: fill_color.for_gpu(),
        }
    }

    pub fn new_with_outline(
        center: Vec2,
        z: f32,
        radius: Vec2,
        fill_color: Color,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self {
            center: [center.x, center.y, z],
            radius: radius.into(),
            fill_color: fill_color.for_gpu(),
            outline_thickness,
            outline_color: outline_color.for_gpu(),
        }
    }
}

// ////////////////////////////////////////////////////////////////////////////
//                                   Shape                                   //
///////////////////////////////////////////////////////////////////////////////

pub struct ShapeBatch {
    pub vertices: Vec<Vertex3D>,
    pub indices: Vec<u32>,
    pub max_index: u32,
    pub scissor: Option<glium::Rect>,
}

impl ShapeBatch {
    pub fn new(scissor: Option<glium::Rect>) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            max_index: 0,
            scissor,
        }
    }
}
