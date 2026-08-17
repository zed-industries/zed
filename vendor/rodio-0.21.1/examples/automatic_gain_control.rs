use rodio::source::Source;
use rodio::Decoder;
use std::error::Error;
use std::fs::File;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());

    // Decode the sound file into a source
    let file = File::open("assets/music.flac")?;
    let source = Decoder::try_from(file)?;

    // Apply automatic gain control to the source
    let agc_source = source.automatic_gain_control(Default::default());

    // Make it so that the source checks if automatic gain control should be
    // enabled or disabled every 5 milliseconds. We must clone `agc_enabled`,
    // or we would lose it when we move it into the periodic access.
    let agc_enabled = Arc::new(AtomicBool::new(true));
    let agc_enabled_clone = agc_enabled.clone();
    let controlled = agc_source.periodic_access(Duration::from_millis(5), move |agc_source| {
        agc_source.set_enabled(agc_enabled_clone.load(Ordering::Relaxed));
    });

    // Add the source now equipped with automatic gain control and controlled via
    // periodic_access to the sink for the playback.
    player.append(controlled);

    // After 5 seconds of playback disable automatic gain control using the
    // shared AtomicBool `agc_enabled`. You could do this from another part
    // of the program since `agc_enabled` is of type Arc<AtomicBool> which
    // is freely clone-able and move-able.
    //
    // Note that disabling the AGC takes up to 5 millis because periodic_access
    // controls the source every 5 millis.
    thread::sleep(Duration::from_secs(5));
    agc_enabled.store(false, Ordering::Relaxed);

    // Keep the program running until the playback is complete.
    player.sleep_until_end();
    Ok(())
}
