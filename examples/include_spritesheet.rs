use engine_4::prelude::*;
use engine_4_macros::include_spritesheet;

fn main() -> anyhow::Result<()> {
    init("Title")?;

    include_spritesheet!(spritesheet, "assets/textures/pixel");

    loop {
        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
