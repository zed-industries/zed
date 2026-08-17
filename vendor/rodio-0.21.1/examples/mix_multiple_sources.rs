use rodio::mixer;
use rodio::source::{SineWave, Source};
use rodio::Float;
use std::error::Error;
use std::num::NonZero;
use std::time::Duration;

const NOTE_DURATION: Duration = Duration::from_secs(1);
const NOTE_AMPLITUDE: Float = 0.20;

fn main() -> Result<(), Box<dyn Error>> {
    // Construct a dynamic controller and mixer, stream_handle, and player.
    let (controller, mixer) = mixer::mixer(NonZero::new(2).unwrap(), NonZero::new(44_100).unwrap());
    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(stream_handle.mixer());

    // Create four unique sources. The frequencies used here correspond
    // notes in the key of C and in octave 4: C4, or middle C on a piano,
    // E4, G4, and A4 respectively.
    let source_c = SineWave::new(261.63)
        .take_duration(NOTE_DURATION)
        .amplify(NOTE_AMPLITUDE);
    let source_e = SineWave::new(329.63)
        .take_duration(NOTE_DURATION)
        .amplify(NOTE_AMPLITUDE);
    let source_g = SineWave::new(392.0)
        .take_duration(NOTE_DURATION)
        .amplify(NOTE_AMPLITUDE);
    let source_a = SineWave::new(440.0)
        .take_duration(NOTE_DURATION)
        .amplify(NOTE_AMPLITUDE);

    // Add sources C, E, G, and A to the mixer controller.
    controller.add(source_c);
    controller.add(source_e);
    controller.add(source_g);
    controller.add(source_a);

    // Append the dynamic mixer to the sink to play a C major 6th chord.
    player.append(mixer);

    // Sleep the thread until sink is empty.
    player.sleep_until_end();

    Ok(())
}
