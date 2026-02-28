use tunes::{engine::AudioEngine, error::TunesError};

global::global!(AudioState, audio_state);

pub struct AudioState {
    engine: AudioEngine,
}

pub fn play_sound(path: &str) -> tunes::prelude::SamplePlaybackBuilder<'_> {
    get_audio_state().engine.play_sample(path)
}

pub fn audio() -> &'static mut AudioEngine {
    &mut get_audio_state().engine
}

pub fn init() -> Result<(), TunesError> {
    let engine = AudioEngine::new()?;
    set_audio_state(AudioState { engine });
    log::info!("Initialized audio");
    Ok(())
}
