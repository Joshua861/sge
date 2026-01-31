use engine_4::prelude::*;
use engine_color::schemes::ColorScheme;

const COLORS: ColorScheme = ColorScheme::GRUVBOX_DARK;

fn main() -> anyhow::Result<()> {
    init("Color schemes")?;

    let text = "Hello color schemes!";
    let text_params = TextDrawParams {
        color: COLORS.fg0,
        position: Vec2::splat(100.0),
        font_size: 50,
        ..Default::default()
    };

    loop {
        draw_rect(Vec2::ZERO, Vec2::splat(1000.0), COLORS.bg0);
        clear_screen(COLORS.bg0);

        let dimensions = measure_text_ex(text, text_params);
        draw_rect(text_params.position, dimensions.size, COLORS.bg1);
        draw_text_ex(text, text_params);

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
