use std::{
    env,
    num::NonZero,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, StreamTrait};
use rodio::{buffer::SamplesBuffer, conversions::SampleTypeConverter};
use util::ResultExt;

pub struct CaptureInput {
    pub name: String,
    config: cpal::SupportedStreamConfig,
    samples: Arc<Mutex<Vec<i16>>>,
    _stream: cpal::Stream,
}

impl CaptureInput {
    pub fn start() -> anyhow::Result<Self> {
        let (device, config) = crate::default_device(true)?;
        let name = device.name().unwrap_or("<unknown>".to_string());
        log::info!("Using microphone: {}", name);

        let samples = Arc::new(Mutex::new(Vec::new()));
        let stream = start_capture(device, config.clone(), samples.clone())?;

        Ok(Self {
            name,
            _stream: stream,
            config,
            samples,
        })
    }

    pub fn finish(self) -> Result<PathBuf> {
        let name = self.name;
        let mut path = env::current_dir().context("Could not get current dir")?;
        path.push(&format!("test_recording_{name}.wav"));
        log::info!("Test recording written to: {}", path.display());
        write_out(self.samples, self.config, &path)?;
        Ok(path)
    }
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
                let data = crate::get_sample_data(config.sample_format(), data).log_err();
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
    path: &Path,
) -> Result<()> {
    let samples = std::mem::take(
        &mut *samples
            .try_lock()
            .expect("Stream has ended, callback cant hold the lock"),
    );
    let samples: Vec<f32> = SampleTypeConverter::<_, f32>::new(samples.into_iter()).collect();
    let mut samples = SamplesBuffer::new(
        NonZero::new(config.channels()).expect("config channel is never zero"),
        NonZero::new(config.sample_rate().0).expect("config sample_rate is never zero"),
        samples,
    );
    match rodio::wav_to_file(&mut samples, path) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!("Failed to write wav file: {}", e)),
    }
}
