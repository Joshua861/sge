use engine_4::prelude::*;

fn main() -> anyhow::Result<()> {
    let opts = EngineCreationOptions {
        window: WindowOptions {
            title: "Logging",
            ..Default::default()
        },
        min_log_level: LevelFilter::Trace,
        ..Default::default()
    };

    init_custom(opts)?;

    trace!("Hello");
    debug!("Hello!");
    info!("Hello!!");
    warn!("Hello!!!");
    error!("Hello!!!!");

    loop {
        draw_logs();

        if should_quit() {
            break;
        }

        next_frame();
    }

    Ok(())
}
