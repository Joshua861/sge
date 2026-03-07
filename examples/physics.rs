use sge::prelude::*;

const BOUNDS_SIZE: Vec2 = Vec2::new(1000.0, 1000.0);
const BOUNDS_THICKNESS: f32 = 50.0;
const FORCE_RADIUS: f32 = 250.0;
const FORCE_STRENGTH: f32 = 100.0;

#[derive(Clone, Copy)]
enum ShapeType {
    Circle,
    Square,
    Rectangle,
}

impl ShapeType {
    fn from_index(i: usize) -> Self {
        match i % 3 {
            0 => Self::Circle,
            1 => Self::Square,
            _ => Self::Rectangle,
        }
    }

    fn bounds(&self) -> Bounds {
        match self {
            Self::Circle => Bounds::Circle(15.0),
            Self::Square => Bounds::Rect(Vec2::splat(30.0)),
            Self::Rectangle => Bounds::Rect(Vec2::new(40.0, 20.0)),
        }
    }

    fn draw(&self, pos: Vec2, color: Color) {
        match self {
            Self::Circle => draw_circle(pos, 15.0, color),
            Self::Square => draw_square(pos - Vec2::splat(15.0), 30.0, color),
            Self::Rectangle => draw_rect(pos - Vec2::new(20.0, 10.0), Vec2::new(40.0, 20.0), color),
        }
    }
}

fn speed_color(speed: f32) -> Color {
    Color::from_oklch(
        0.8,
        0.1 + (speed / 100.0).clamp(0.0, 0.1),
        142.94 - (speed / 5.0).clamp(0.0, 142.94 - 26.17),
    )
}

fn main() -> anyhow::Result<()> {
    init("Physics Showcase")?;

    let mut world = World::new();

    let wall_rects = [
        (
            Vec2::new(BOUNDS_THICKNESS * 0.5, BOUNDS_SIZE.y * 0.5),
            Vec2::new(BOUNDS_THICKNESS, BOUNDS_SIZE.y),
        ),
        (
            Vec2::new(BOUNDS_SIZE.x * 0.5, BOUNDS_THICKNESS * 0.5),
            Vec2::new(BOUNDS_SIZE.x, BOUNDS_THICKNESS),
        ),
        (
            Vec2::new(BOUNDS_SIZE.x * 0.5, BOUNDS_SIZE.y - BOUNDS_THICKNESS * 0.5),
            Vec2::new(BOUNDS_SIZE.x, BOUNDS_THICKNESS),
        ),
        (
            Vec2::new(BOUNDS_SIZE.x - BOUNDS_THICKNESS * 0.5, BOUNDS_SIZE.y * 0.5),
            Vec2::new(BOUNDS_THICKNESS, BOUNDS_SIZE.y),
        ),
    ];

    for (pos, size) in wall_rects {
        world.create_fixed(Bounds::Rect(size)).set_position(pos);
    }

    let mut objects: Vec<(ObjectRef, ShapeType)> = Vec::new();

    for i in 0..50 {
        let pos = Vec2::new(
            rand::<f32>() * (BOUNDS_SIZE.x - BOUNDS_THICKNESS * 2.0) + BOUNDS_THICKNESS,
            rand::<f32>() * (BOUNDS_SIZE.y - BOUNDS_THICKNESS * 2.0) + BOUNDS_THICKNESS,
        );
        let velocity = Vec2::new(rand::<f32>() * 500.0 - 250.0, rand::<f32>() * 500.0 - 250.0);
        let shape_type = ShapeType::from_index(i);

        let mut collider = world.create_dynamic(shape_type.bounds()).with_ccd();
        collider.set_position(pos);
        collider.set_velocity(velocity);
        objects.push((collider, shape_type));
    }

    loop {
        world.update();
        clear_screen(Color::NEUTRAL_900);

        draw_rect(
            Vec2::ZERO,
            Vec2::new(BOUNDS_THICKNESS, BOUNDS_SIZE.y),
            Color::NEUTRAL_800,
        );
        draw_rect(
            Vec2::ZERO,
            Vec2::new(BOUNDS_SIZE.x, BOUNDS_THICKNESS),
            Color::NEUTRAL_800,
        );
        draw_rect(
            Vec2::new(0.0, BOUNDS_SIZE.y - BOUNDS_THICKNESS),
            Vec2::new(BOUNDS_SIZE.x, BOUNDS_THICKNESS),
            Color::NEUTRAL_800,
        );
        draw_rect(
            Vec2::new(BOUNDS_SIZE.x - BOUNDS_THICKNESS, 0.0),
            Vec2::new(BOUNDS_THICKNESS, BOUNDS_SIZE.y),
            Color::NEUTRAL_800,
        );

        let ui = {
            use ui::prelude::*;

            Fit::new(Fill::new(
                Color::NEUTRAL_800,
                Padding::all(
                    50.0,
                    Col::new([
                        Text::title_nowrap("Physics showcase"),
                        Text::mono(format!("Objects: {}", objects.len())),
                        Text::mono(format!("FPS: {:.2}", avg_fps())),
                        Text::h2("Controls"),
                        Text::body("• Left Click: Spawn object"),
                        Text::body("• Right Click (hold): Apply force"),
                    ]),
                ),
            ))
        };
        ui::draw_ui(ui, vec2(0.0, BOUNDS_SIZE.y - BOUNDS_THICKNESS));

        let cursor_pos = last_cursor_pos();

        if mouse_pressed(MouseButton::Left) {
            let velocity = Vec2::new(rand::<f32>() * 100.0 - 50.0, rand::<f32>() * 100.0 - 50.0);
            let shape_type = ShapeType::from_index(objects.len());
            let mut collider = world.create_dynamic(shape_type.bounds());
            collider.set_position(cursor_pos);
            collider.set_velocity(velocity);
            objects.push((collider, shape_type));
        }

        for (collider, shape_type) in &objects {
            let pos = collider.get_position();
            let speed = collider.get_velocity().length();
            shape_type.draw(pos, speed_color(speed));
        }

        if mouse_held(MouseButton::Right) {
            for (collider, _) in &mut objects {
                let pos = collider.get_position();
                let to_cursor = cursor_pos - pos;
                let distance = to_cursor.length();

                if distance < FORCE_RADIUS && distance > 0.0 {
                    let strength = (1.0 - distance.powi(2) / FORCE_RADIUS.powi(2)) * FORCE_STRENGTH;
                    collider.add_velocity(to_cursor.normalize() * strength);
                }
            }

            draw_circle_with_outline(
                cursor_pos,
                FORCE_RADIUS,
                Color::CYAN_500.with_alpha(0.2),
                Color::CYAN_500.with_alpha(0.5),
                3.0,
            );
            draw_circle_with_outline(cursor_pos, 10.0, Color::CYAN_400, Color::WHITE, 3.0);
        }

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
