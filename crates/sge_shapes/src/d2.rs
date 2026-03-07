use bevy_math::Vec2;
use sge_color::Color;
use sge_math::collision::{Aabb2d, HasBounds2D};
use sge_types::Vertex2D;
use std::f32::consts::TAU;

pub trait Shape2D: HasBounds2D {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>);
    fn is_visible_in_world(&self) -> bool {
        self.bounds().is_visible_in_world()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub center: Vec2,
    pub radius: Vec2,
    pub color: Color,
}

impl Shape2D for Circle {
    fn points(&self, _starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        unimplemented!();
    }
}

impl HasBounds2D for Circle {
    fn bounds(&self) -> Aabb2d {
        Aabb2d::from_center_size(self.center, self.radius * 2.0)
    }
}

impl Circle {
    pub fn encompassing_radius(&self) -> f32 {
        self.radius.x.max(self.radius.y)
    }

    pub fn from_top_left(top_left: Vec2, radius: Vec2, color: Color) -> Self {
        Self {
            center: top_left + radius,
            radius,
            color,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CircleOutline {
    pub center: Vec2,
    pub radius: Vec2,
    pub color: Color,
    pub thickness: f32,
}

impl HasBounds2D for CircleOutline {
    fn bounds(&self) -> Aabb2d {
        let total_radius = self.radius + Vec2::splat(self.thickness);
        Aabb2d::from_center_size(self.center, total_radius * 2.0)
    }
}

impl Shape2D for CircleOutline {
    fn points(&self, _starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        unimplemented!();
    }
}

pub struct RoundedRectangle {
    pub top_left: Vec2,
    pub size: Vec2,
    pub fill_color: Color,
    pub corner_radius: f32,
    pub outline_thickness: f32,
    pub outline_color: Color,
}

impl HasBounds2D for RoundedRectangle {
    fn bounds(&self) -> Aabb2d {
        Aabb2d::new(self.top_left, self.top_left + self.size)
    }
}

impl Shape2D for RoundedRectangle {
    fn points(&self, _: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        unimplemented!();
    }
}

impl RoundedRectangle {
    pub fn new(top_left: Vec2, size: Vec2, color: Color, corner_radius: f32) -> Self {
        Self {
            top_left,
            size,
            fill_color: color,
            corner_radius,
            outline_color: color,
            outline_thickness: 0.0,
        }
    }

    pub fn square(top_left: Vec2, size: f32, color: Color, corner_radius: f32) -> Self {
        Self::new(top_left, Vec2::splat(size), color, corner_radius)
    }

    pub fn square_with_outline(
        top_left: Vec2,
        size: f32,
        color: Color,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self::with_outline(
            top_left,
            Vec2::splat(size),
            color,
            corner_radius,
            outline_thickness,
            outline_color,
        )
    }

    pub fn with_outline(
        top_left: Vec2,
        size: Vec2,
        color: Color,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self {
            top_left,
            size,
            fill_color: color,
            corner_radius,
            outline_thickness,
            outline_color,
        }
    }

    pub fn from_center(center: Vec2, size: Vec2, color: Color, corner_radius: f32) -> Self {
        Self::new(center - size / 2.0, size, color, corner_radius)
    }

    pub fn from_center_with_outline(
        center: Vec2,
        size: Vec2,
        color: Color,
        corner_radius: f32,
        outline_thickness: f32,
        outline_color: Color,
    ) -> Self {
        Self::with_outline(
            center - size / 2.0,
            size,
            color,
            corner_radius,
            outline_thickness,
            outline_color,
        )
    }

    pub fn center(&self) -> Vec2 {
        self.top_left + self.size / 2.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub top_left: Vec2,
    pub size: Vec2,
    pub color: Color,
}

impl HasBounds2D for Rect {
    fn bounds(&self) -> Aabb2d {
        Aabb2d::new(self.top_left, self.top_left + self.size)
    }
}

impl Rect {
    pub fn new(top_left: Vec2, size: Vec2, color: Color) -> Self {
        Self {
            top_left,
            size,
            color,
        }
    }

    pub fn from_center(center: Vec2, size: Vec2, color: Color) -> Self {
        Self::new(center - size / 2.0, size, color)
    }

    pub fn center(&self) -> Vec2 {
        self.top_left + self.size / 2.0
    }

    pub fn new_square(top_left: Vec2, size: f32, color: Color) -> Self {
        Self::new(top_left, Vec2::splat(size), color)
    }

    pub fn from_square_center(center: Vec2, size: f32, color: Color) -> Self {
        Self::from_center(center, Vec2::splat(size), color)
    }

    fn gen_quad(&self) -> Vec<Vertex2D> {
        let tl = self.top_left;
        let br = self.top_left + self.size;

        vec![
            Vertex2D::new(tl.x, tl.y, self.color),
            Vertex2D::new(br.x, tl.y, self.color),
            Vertex2D::new(tl.x, br.y, self.color),
            Vertex2D::new(br.x, br.y, self.color),
        ]
    }
}

impl Shape2D for Rect {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        let quad = self.gen_quad();
        let indices = QUAD_INDICES.map(|n| n + starting_index).to_vec();
        (indices, quad)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub points: [Vec2; 3],
    pub color: Color,
}

impl HasBounds2D for Triangle {
    fn bounds(&self) -> Aabb2d {
        let min = self.points[0].min(self.points[1]).min(self.points[2]);
        let max = self.points[0].max(self.points[1]).max(self.points[2]);
        Aabb2d::new(min, max)
    }
}

impl Shape2D for Triangle {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        let tri = self.points.map(|p| Vertex2D::new(p.x, p.y, self.color));
        let indices = starting_index..starting_index + 3;
        (indices.collect(), tri.to_vec())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line2D {
    pub start: Vec2,
    pub end: Vec2,
    pub thickness: f32,
    pub color: Color,
}

impl HasBounds2D for Line2D {
    fn bounds(&self) -> Aabb2d {
        let half_thick = self.thickness * 0.5;
        Aabb2d::new(
            self.start.min(self.end) - Vec2::splat(half_thick),
            self.start.max(self.end) + Vec2::splat(half_thick),
        )
    }
}

impl Line2D {
    fn gen_mesh(&self) -> Option<Vec<Vertex2D>> {
        let direction = self.end - self.start;
        let length = direction.length();

        if length == 0.0 {
            return None;
        }

        let normalized = direction / length;
        let perpendicular = Vec2::new(-normalized.y, normalized.x) * self.thickness / 2.0;

        Some(vec![
            Vertex2D::new(
                self.start.x - perpendicular.x,
                self.start.y - perpendicular.y,
                self.color,
            ),
            Vertex2D::new(
                self.end.x - perpendicular.x,
                self.end.y - perpendicular.y,
                self.color,
            ),
            Vertex2D::new(
                self.start.x + perpendicular.x,
                self.start.y + perpendicular.y,
                self.color,
            ),
            Vertex2D::new(
                self.end.x + perpendicular.x,
                self.end.y + perpendicular.y,
                self.color,
            ),
        ])
    }
}

impl Shape2D for Line2D {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        if let Some(mesh) = self.gen_mesh() {
            (QUAD_INDICES.map(|n| n + starting_index).to_vec(), mesh)
        } else {
            (vec![], vec![])
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Poly {
    pub sides: usize,
    pub radius: f32,
    pub center: Vec2,
    pub rotation: f32,
    pub color: Color,
}

impl HasBounds2D for Poly {
    fn bounds(&self) -> Aabb2d {
        Aabb2d::from_center_size(self.center, Vec2::splat(self.radius * 2.0))
    }
}

impl Poly {
    pub fn gen_points(&self) -> Vec<Vec2> {
        let mut points = Vec::with_capacity(self.sides);
        let angle_step = TAU / self.sides as f32;

        for i in 0..self.sides {
            let angle = angle_step * i as f32 + self.rotation;
            let x = self.center.x + self.radius * angle.cos();
            let y = self.center.y + self.radius * angle.sin();
            points.push(Vec2::new(x, y));
        }

        points
    }

    pub fn gen_mesh(&self) -> (Vec<Vertex2D>, Vec<u32>) {
        let points = self.gen_points();
        gen_mesh_from_points(&points, self.color)
    }
}

impl Shape2D for Poly {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        let (vertices, indices) = self.gen_mesh();
        let indices = indices.iter().map(|n| n + starting_index).collect();
        (indices, vertices)
    }
}

#[derive(Clone, Debug)]
pub struct CustomShape {
    pub points: Vec<Vec2>,
    pub color: Color,
}

impl HasBounds2D for CustomShape {
    fn bounds(&self) -> Aabb2d {
        if self.points.is_empty() {
            return Aabb2d::new(Vec2::ZERO, Vec2::ZERO);
        }

        let mut min = self.points[0];
        let mut max = self.points[0];

        for point in &self.points[1..] {
            min = min.min(*point);
            max = max.max(*point);
        }

        Aabb2d::new(min, max)
    }
}

impl Shape2D for CustomShape {
    fn points(&self, starting_index: u32) -> (Vec<u32>, Vec<Vertex2D>) {
        let (vertices, indices) = gen_mesh_from_points(&self.points, self.color);
        let indices = indices.iter().map(|n| n + starting_index).collect();
        (indices, vertices)
    }
}

pub const QUAD_INDICES: [u32; 6] = [0, 1, 2, 1, 2, 3];

pub const UNIT_QUAD: [Vertex2D; 4] = [
    Vertex2D {
        position: [-1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex2D {
        position: [1.0, -1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex2D {
        position: [-1.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
    Vertex2D {
        position: [1.0, 1.0],
        color: [1.0, 1.0, 1.0, 1.0],
    },
];

fn gen_mesh_from_points(points: &[Vec2], color: Color) -> (Vec<Vertex2D>, Vec<u32>) {
    if points.len() < 3 {
        return (vec![], vec![]);
    }

    let mut polygon_builder = lyon::tessellation::path::Path::builder();
    polygon_builder.begin(lyon::math::point(points[0].x, points[0].y));
    for point in &points[1..] {
        polygon_builder.line_to(lyon::math::point(point.x, point.y));
    }
    polygon_builder.end(false);
    let polygon = polygon_builder.build();

    struct VertexConstructor {
        color: Color,
    }

    impl lyon::tessellation::FillVertexConstructor<Vertex2D> for VertexConstructor {
        fn new_vertex(&mut self, vertex: lyon::tessellation::FillVertex) -> Vertex2D {
            let pos = vertex.position();
            Vertex2D::new(pos.x, pos.y, self.color)
        }
    }

    let mut tessellator = lyon::tessellation::FillTessellator::new();
    let mut buffers = lyon::tessellation::VertexBuffers::<Vertex2D, u32>::new();

    tessellator
        .tessellate_path(
            &polygon,
            &lyon::tessellation::FillOptions::non_zero(),
            &mut lyon::tessellation::BuffersBuilder::new(&mut buffers, VertexConstructor { color }),
        )
        .unwrap();

    let vertices = buffers.vertices;
    let indices = buffers.indices;

    (vertices, indices)
}
