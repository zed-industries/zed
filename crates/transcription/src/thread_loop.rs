use std::{num::NonZero, sync::Arc, thread::sleep, time::Duration};

use anyhow::{Context, Ok, Result};
use async_channel::Sender;
use audio::RodioExt;
use log::{error, info, warn};
use parking_lot::Mutex;
use rodio::nz;
use rodio::source::UniformSourceIterator;
use rodio::microphone::MicrophoneBuilder;
use whisper_rs::{
    DtwModelPreset, FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters, WhisperState,
};

const WHISPER_MODEL_NAME: &str = "ggml-base.en.bin";
const TARGET_SAMPLE_RATE: NonZero<u32> = nz!(16_000);
pub const BUFFER_SIZE: usize = TARGET_SAMPLE_RATE.get() as usize / 10;

use crate::{TranscriptionNotification, TranscriptionThreadState};

fn open_mic() -> Result<UniformSourceIterator<impl rodio::Source>> {
    let stream = MicrophoneBuilder::new()
        .default_device()?
        .default_config()?
        .prefer_sample_rates([
            TARGET_SAMPLE_RATE, // sample rates trivially resamplable to `SAMPLE_RATE`
            TARGET_SAMPLE_RATE.saturating_mul(nz!(2)),
            TARGET_SAMPLE_RATE.saturating_mul(nz!(3)),
            TARGET_SAMPLE_RATE.saturating_mul(nz!(4)),
        ])
        .prefer_channel_counts([nz!(1), nz!(2), nz!(3), nz!(4)])
        .prefer_buffer_sizes(512..)
        .open_stream()?;

    info!("Opened transcription microphone: {:?}", stream.config());

    let stream = stream
        .possibly_disconnected_channels_to_mono()
        .constant_samplerate(TARGET_SAMPLE_RATE)
        .denoise()
        .context("Could not set up transcription denoiser")?
        .constant_params(nz!(1), TARGET_SAMPLE_RATE);

    Ok(stream)
}

pub fn load_whisper_model(notification_sender: Sender<TranscriptionNotification>) -> Result<WhisperState> {
    let model_path = paths::languages_dir().join(WHISPER_MODEL_NAME);
    if !model_path.exists() {
        warn!("Whisper model missing at {:?}", model_path);
        notification_sender.send_blocking(TranscriptionNotification::ModelNotFound(
            model_path.to_string_lossy().into(),
        ))?;
        return Err(anyhow::anyhow!(
            "whisper model not found at {:?}",
            model_path
        ));
    }
    let model_path_string = model_path.to_string_lossy().to_string();
    let mut context_params = WhisperContextParameters::default();
    context_params.dtw_parameters.mode = whisper_rs::DtwMode::ModelPreset {
        model_preset: DtwModelPreset::BaseEn,
    };
    let whisper_context = WhisperContext::new_with_params(&model_path_string, context_params)
        .map_err(|e| anyhow::anyhow!("failed to load whisper model: {}", e))?;
    Ok(whisper_context.create_state()?)
}

pub fn transcription_loop_body(
    state: Arc<Mutex<TranscriptionThreadState>>,
    transcription_sender: Sender<String>,
    notification_sender: Sender<TranscriptionNotification>,
) -> Result<()> {
    let stream = open_mic()?;

    // Load the model
    let mut whisper_state = load_whisper_model(notification_sender.clone())?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
    // TODO: Make this configurable
    params.set_n_threads(8);
    params.set_language(Some("en"));

    let mut audio_buffer = Vec::new();

    for sample in stream {
        let state = (*state.lock()).clone();

        if state == TranscriptionThreadState::Disabled {
            info!("Stopping the transcription thread");
            return Ok(());
        }

        if state != TranscriptionThreadState::Listening {
            // If not listening, clear the buffer and sleep for a bit
            audio_buffer.clear();

            info!("Not listening...");

            sleep(Duration::from_millis(5000));
            continue;
        }

        audio_buffer.push(sample);

        if audio_buffer.len() < BUFFER_SIZE {
            continue;
        }

        whisper_state
            .full(params.clone(), &audio_buffer)
            .map_err(|e| anyhow::anyhow!("failed to run model: {}", e))?;

        let num_segments = whisper_state.full_n_segments();
        info!("Transcription produced {} segments", num_segments);

        for i in 0..num_segments {
            if let Some(segment) = whisper_state.get_segment(i) {
                let text = segment
                    .to_str()
                    .map_err(|e| anyhow::anyhow!("failed to get segment text: {}", e))?
                    .trim()
                    .to_owned();

                if text.is_empty() {
                    continue;
                }

                transcription_sender.send_blocking(text)?;
            }
        }

        audio_buffer.clear();
    }

    error!("Transcription thread exited - this should never happen");

    Ok(())
}
