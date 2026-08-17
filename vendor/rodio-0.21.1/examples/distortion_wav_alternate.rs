use std::error::Error;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use rodio::Source;

fn main() -> Result<(), Box<dyn Error>> {
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());

    let file = std::fs::File::open("assets/music.wav")?;
    let source = rodio::Decoder::try_from(file)?;

    // Shared flag to enable/disable distortion
    let distortion_enabled = Arc::new(AtomicBool::new(true));
    let distortion_enabled_clone = distortion_enabled.clone();

    // Apply distortion and alternate the effect during playback
    let distorted =
        source
            .distortion(4.0, 0.3)
            .periodic_access(Duration::from_millis(250), move |src| {
                // src is &mut PeriodicAccess<Distortion<Decoder<...>>>
                let enable = distortion_enabled_clone.load(Ordering::Relaxed);
                // Call the setters on the distortion filter inside the source
                src.set_gain(if enable { 4.0 } else { 1.0 });
                src.set_threshold(if enable { 0.3 } else { 1.0 });
            });

    player.append(distorted);

    println!("Playing music.wav with alternating distortion effect...");
    // Alternate the distortion effect every second for 10 seconds
    for _ in 0..10 {
        thread::sleep(Duration::from_secs(1));
        let prev = distortion_enabled.load(Ordering::Relaxed);
        distortion_enabled.store(!prev, Ordering::Relaxed);
        println!("Distortion {}", if !prev { "ON" } else { "OFF" });
    }

    // Wait for playback to finish
    player.sleep_until_end();

    Ok(())
}
