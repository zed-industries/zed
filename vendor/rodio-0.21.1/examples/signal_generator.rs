//! Test signal generator example.

use std::error::Error;
use std::num::NonZero;

fn main() -> Result<(), Box<dyn Error>> {
    use rodio::source::{chirp, Function, SignalGenerator, Source};
    use std::thread;
    use std::time::Duration;

    let stream_handle = rodio::DeviceSinkBuilder::open_default_sink()?;

    let test_signal_duration = Duration::from_millis(1000);
    let interval_duration = Duration::from_millis(1500);
    let sample_rate = NonZero::new(48000).unwrap();

    println!("Playing 1000 Hz tone");
    stream_handle.mixer().add(
        SignalGenerator::new(sample_rate, 1000.0, Function::Sine)
            .amplify(0.1)
            .take_duration(test_signal_duration),
    );

    thread::sleep(interval_duration);

    println!("Playing 10,000 Hz tone");
    stream_handle.mixer().add(
        SignalGenerator::new(sample_rate, 10000.0, Function::Sine)
            .amplify(0.1)
            .take_duration(test_signal_duration),
    );

    thread::sleep(interval_duration);

    println!("Playing 440 Hz Triangle Wave");
    stream_handle.mixer().add(
        SignalGenerator::new(sample_rate, 440.0, Function::Triangle)
            .amplify(0.1)
            .take_duration(test_signal_duration),
    );

    thread::sleep(interval_duration);

    println!("Playing 440 Hz Sawtooth Wave");
    stream_handle.mixer().add(
        SignalGenerator::new(sample_rate, 440.0, Function::Sawtooth)
            .amplify(0.1)
            .take_duration(test_signal_duration),
    );

    thread::sleep(interval_duration);

    println!("Playing 440 Hz Square Wave");
    stream_handle.mixer().add(
        SignalGenerator::new(sample_rate, 440.0, Function::Square)
            .amplify(0.1)
            .take_duration(test_signal_duration),
    );

    thread::sleep(interval_duration);

    println!("Playing 20-10000 Hz Sweep");
    stream_handle.mixer().add(
        chirp(sample_rate, 20.0, 10000.0, Duration::from_secs(1))
            .amplify(0.1)
            .take_duration(test_signal_duration),
    );

    thread::sleep(interval_duration);

    Ok(())
}
