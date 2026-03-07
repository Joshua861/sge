use sge::prelude::*;

const PLAYER_RADIUS: f32 = 30.0;

struct Player {
    color: Color,
    controller: PlayerController,
}

fn main() -> anyhow::Result<()> {
    init("2D Platformer")?;

    let mut world = World::new();

    let mut player = Player {
        color: Color::RED_500,
        controller: world
            .create_player_controller(Bounds::Circle(PLAYER_RADIUS))
            .with_position(Vec2::new(0.0, -200.0)),
    };

    const PREV_LEN: usize = 100;
    const PREV_REPEAT: usize = 10;
    let mut prev_pos = [Vec2::ZERO; PREV_LEN];
    let mut write_head: usize = 0;

    player
        .controller
        .set_binds()
        .jump(KeyCode::Space)
        .right(KeyCode::KeyF)
        .left(KeyCode::KeyS);

    player.controller.set_move_speed(5.0);
    player.controller.set_double_jumps(1);
    player.controller.set_jump_velocity(13.0);
    world.set_gravity(50.0);

    let platforms = vec![
        platform(-500.0, 400.0, 1000.0),
        platform(-400.0, 250.0, 200.0),
        platform(200.0, 250.0, 200.0),
        platform(-100.0, 100.0, 200.0),
    ];

    for p in &platforms {
        world
            .create_fixed(Bounds::Rect(p.size))
            .with_position(p.center());
    }

    let mut debug_mode = false;

    loop {
        clear_screen(Color::NEUTRAL_900);

        if key_pressed(KeyCode::KeyD) {
            debug_mode = !debug_mode;
        }

        if key_pressed(KeyCode::KeyR) {
            player.controller.set_position(Vec2::new(0.0, -200.0));
        }

        if debug_mode {
            world.draw_colliders_world();
        }

        if player.controller.position().y > 1000.0 {
            player.controller.add_impulse(Vec2::Y * 15.0);
        }

        {
            let ppos = player.controller.position();
            let cpos = get_camera_2d().translation;
            let (normal, len) = (ppos - cpos).normalize_and_length();

            if len > 300.0 {
                let pos = -normal * 300.0 + ppos;
                mutate_camera_2d(|c| c.translation = pos);
            }
        }

        for i in 0..PREV_REPEAT {
            let ratio = (i + 1) as f32 / PREV_REPEAT as f32;
            let pos = player
                .controller
                .position_last_frame()
                .lerp(player.controller.position(), ratio);
            prev_pos[write_head % PREV_LEN] = pos;
            write_head += 1;
        }

        if !debug_mode {
            for i in 0..PREV_LEN {
                let idx = (write_head + i) % PREV_LEN;
                let age = i as f32 / PREV_LEN as f32;
                draw_circle_world(
                    prev_pos[idx],
                    PLAYER_RADIUS * age,
                    player.color.darken(0.2).desaturate(0.5),
                );
            }

            draw_circle_world(player.controller.position(), PLAYER_RADIUS, player.color);

            for p in &platforms {
                draw_world(*p);
            }
        }

        world.update();

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}

fn platform(x: f32, y: f32, w: f32) -> Rect {
    Rect::new(Vec2::new(x, y), Vec2::new(w, 20.0), Color::NEUTRAL_800)
}
