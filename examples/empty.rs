use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    init("Title")?;

    loop {
        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
