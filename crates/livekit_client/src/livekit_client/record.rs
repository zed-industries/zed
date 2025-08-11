use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, StreamTrait};
use rodio::{buffer::SamplesBuffer, conversions::SampleTypeConverter};
use util::ResultExt;

use crate::livekit_client::playback::{default_device, get_sample_data};

pub fn capture_input() -> Result<()> {
    let (device, config) = default_device(true)?;
    let name = device.name().unwrap_or("<unknown>".to_string());
    log::info!("Using microphone: {}", name);

    let samples = Arc::new(Mutex::new(Vec::new()));
    let stream = start_capture(device, config.clone(), samples.clone());
    thread::sleep(Duration::from_secs(10));
    drop(stream);

    let mut path = env::current_dir().context("Could not get current dir")?;
    path.push(&format!("test_recording_{name}.wav"));
    log::info!("Test recording written to: {}", path.display());
    write_out(samples, config, path)
}

fn start_capture(
    device: cpal::Device,
    config: cpal::SupportedStreamConfig,
    samples: Arc<Mutex<Vec<i16>>>,
) -> Result<cpal::Stream> {
    let stream = device
        .build_input_stream_raw(
            &config.config(),
            config.sample_format(),
            move |data, _: &_| {
                let data = get_sample_data(config.sample_format(), data).log_err();
                let Some(data) = data else {
                    return;
                };
                samples
                    .try_lock()
                    .expect("Only locked after stream ends")
                    .extend_from_slice(&data);
            },
            |err| log::error!("error capturing audio track: {:?}", err),
            Some(Duration::from_millis(100)),
        )
        .context("failed to build input stream")?;

    stream.play()?;
    Ok(stream)
}

fn write_out(
    samples: Arc<Mutex<Vec<i16>>>,
    config: cpal::SupportedStreamConfig,
    path: PathBuf,
) -> Result<()> {
    let samples = std::mem::take(
        &mut *samples
            .try_lock()
            .expect("Stream has ended, callback cant hold the lock"),
    );
    let samples: Vec<f32> = SampleTypeConverter::<_, f32>::new(samples.into_iter())
        .into_iter()
        .collect();
    let mut samples = SamplesBuffer::new(config.channels(), config.sample_rate().0, samples);
    match rodio::output_to_wav(&mut samples, path) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!("Failed to write wav file: {}", e)),
    }
}
