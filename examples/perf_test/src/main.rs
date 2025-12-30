use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("perf_test")?;

    let texture = include_texture!("../../../assets/textures/guy.jpg");
    let mesh = cube(Vec3::ZERO, 10.0);
    let cube = Object3D::from_mesh_and_material(mesh, create_textured_material(texture));

    loop {
        for x in 0..1000 {
            for y in 0..1000 {
                draw_circle(ivec2(x, y).as_vec2(), 1.0, Color::WHITE);
            }
        }

        cube.draw();

        blur_screen(10.0);

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
