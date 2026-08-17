use rodio::source::{SineWave, Source};
use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    // Open the default output stream and get the mixer
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let mixer = stream_handle.mixer();

    // Create a sine wave source and apply distortion
    let distorted = SineWave::new(440.0)
        .amplify(0.2)
        .distortion(4.0, 0.3)
        .take_duration(Duration::from_secs(3));

    // Play the distorted sound
    mixer.add(distorted);

    println!("Playing distorted sine wave for 3 seconds...");
    thread::sleep(Duration::from_secs(3));
    println!("Done.");

    Ok(())
}
