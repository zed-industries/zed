use cpal::traits::HostTrait;
use cpal::{BufferSize, SampleFormat};
use rodio::source::SineWave;
use rodio::Source;
use std::error::Error;
use std::num::NonZero;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    // You can use any other output device that can be queried from CPAL.
    let default_device = cpal::default_host()
        .default_output_device()
        .ok_or("No default audio output device is found.")?;
    let stream_handle = rodio::DeviceSinkBuilder::from_device(default_device)?
        // No need to set all parameters explicitly here,
        // the defaults were set from the device's description.
        .with_buffer_size(BufferSize::Fixed(256))
        .with_sample_rate(NonZero::new(48_000).unwrap())
        .with_sample_format(SampleFormat::F32)
        // Note that the function below still tries alternative configs if the specified one fails.
        // If you need to only use the exact specified configuration,
        // then use DeviceSinkBuilder::open_sink() instead.
        .open_sink_or_fallback()?;
    let mixer = stream_handle.mixer();

    let wave = SineWave::new(740.0)
        .amplify(0.1)
        .take_duration(Duration::from_secs(1));
    mixer.add(wave);

    println!("Beep...");
    thread::sleep(Duration::from_millis(1500));

    Ok(())
}
