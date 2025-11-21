use std::sync::Arc;

use anyhow::Result;
use async_channel::Sender;
use cpal::{SampleFormat, StreamConfig};
use log::{error, info, warn};
use parking_lot::Mutex;
use whisper_rs::{DtwModelPreset, WhisperContextParameters};

const WHISPER_MODEL_NAME: &str = "ggml-base.en.bin";
const TARGET_SAMPLE_RATE: usize = 16_000;
const HIGH_PASS_CUTOFF_HZ: f32 = 80.0;

use crate::{SpeechNotification, TranscriberThreadState};

pub fn transcription_loop_body(
    state: Arc<Mutex<TranscriberThreadState>>,
    transcription_sender: Sender<String>,
    notification_sender: Sender<SpeechNotification>,
) -> Result<()> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

    let model_path = paths::languages_dir().join(WHISPER_MODEL_NAME);
    if !model_path.exists() {
        warn!("Whisper model missing at {:?}", model_path);
        notification_sender.send_blocking(SpeechNotification::ModelNotFound(
            model_path.to_string_lossy().into(),
        ))?;
        return Err(anyhow::anyhow!(
            "whisper model not found at {:?}",
            model_path
        ));
    }

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| anyhow::anyhow!("no input device available"))?;
    let device_name = device.name().unwrap_or_else(|_| "Unknown input".into());
    info!("Using audio input device: {}", device_name);
    let config = device.default_input_config()?;
    let input_sample_rate = config.sample_rate().0 as usize;
    let input_channels = config.channels() as usize;
    info!(
        "Input sample rate: {} Hz, channels: {} (target {} Hz)",
        input_sample_rate, input_channels, TARGET_SAMPLE_RATE
    );
    let needs_downmix = input_channels >= 2;
    let needs_resample = input_sample_rate != TARGET_SAMPLE_RATE;
    let downmix_strategy = if input_channels == 1 {
        "mono"
    } else if input_channels == 2 {
        "stereo"
    } else {
        "multi-channel"
    };
    let resample_ratio = TARGET_SAMPLE_RATE as f32 / input_sample_rate as f32;
    info!(
        "speech downmix: {}, resample: {} (ratio {:.3})",
        downmix_strategy, needs_resample, resample_ratio
    );

    let min_buffer_samples = TARGET_SAMPLE_RATE / 2; // ~0.5s of audio
    let silence_trigger_samples = TARGET_SAMPLE_RATE / 4; // ~0.25s of silence
    let max_buffer_samples = TARGET_SAMPLE_RATE * 3; // hard cap ~3s

    let model_path_string = model_path.to_string_lossy().to_string();
    let mut context_params = WhisperContextParameters::default();
    context_params.dtw_parameters.mode = whisper_rs::DtwMode::ModelPreset {
        model_preset: DtwModelPreset::BaseEn,
    };
    let whisper_context = WhisperContext::new_with_params(&model_path_string, context_params)
        .map_err(|e| anyhow::anyhow!("failed to load whisper model: {}", e))?;
    let mut whisper_state = whisper_context.create_state()?;

    let (tx, rx) = std::sync::mpsc::channel();

    let stream_config: StreamConfig = config.clone().into();
    let stream = match config.sample_format() {
        SampleFormat::F32 => {
            let tx = tx.clone();
            device.build_input_stream(
                &stream_config,
                move |data: &[f32], _| {
                    if tx.send(data.to_vec()).is_err() {
                        warn!("speech audio receiver dropped (f32)");
                    }
                },
                |err| error!("error in audio stream: {}", err),
                None,
            )?
        }
        SampleFormat::I16 => {
            let tx = tx.clone();
            device.build_input_stream(
                &stream_config,
                move |data: &[i16], _| {
                    let converted: Vec<f32> = data
                        .iter()
                        .map(|sample| *sample as f32 / i16::MAX as f32)
                        .collect();
                    if tx.send(converted).is_err() {
                        warn!("speech audio receiver dropped (i16)");
                    }
                },
                |err| error!("error in audio stream: {}", err),
                None,
            )?
        }
        SampleFormat::U16 => {
            let tx = tx.clone();
            device.build_input_stream(
                &stream_config,
                move |data: &[u16], _| {
                    let midpoint = u16::MAX as f32 / 2.0;
                    let converted: Vec<f32> = data
                        .iter()
                        .map(|sample| (*sample as f32 - midpoint) / midpoint)
                        .collect();
                    if tx.send(converted).is_err() {
                        warn!("speech audio receiver dropped (u16)");
                    }
                },
                |err| error!("error in audio stream: {}", err),
                None,
            )?
        }
        SampleFormat::F64 => {
            let tx = tx.clone();
            device.build_input_stream(
                &stream_config,
                move |data: &[f64], _| {
                    if tx
                        .send(data.iter().map(|sample| *sample as f32).collect())
                        .is_err()
                    {
                        warn!("speech audio receiver dropped (f64)");
                    }
                },
                |err| error!("error in audio stream: {}", err),
                None,
            )?
        }
        _ => {
            return Err(anyhow::anyhow!(
                "unsupported sample format: {:?}",
                config.sample_format()
            ));
        }
    };

    stream.play()?;

    let mut audio_buffer = Vec::new();
    let mut accumulated_silence = 0usize;
    let mut voiced_samples = 0usize;
    let mut hp_prev_input = 0.0f32;
    let mut hp_prev_output = 0.0f32;

    loop {
        let state = (*state.lock()).clone();

        if state == TranscriberThreadState::Disabled {
            return Ok(());
        }

        if state != TranscriberThreadState::Listening {
            // If not listening, clear the buffer and sleep for a bit
            audio_buffer.clear();
            accumulated_silence = 0;
            voiced_samples = 0;
            hp_prev_input = 0.0;
            hp_prev_output = 0.0;

            std::thread::sleep(std::time::Duration::from_millis(50));
            continue;
        }

        if let Ok(chunk) = rx.try_recv() {
            let mono_chunk = if !needs_downmix || input_channels == 1 {
                chunk.clone()
            } else if input_channels == 2 {
                match whisper_rs::convert_stereo_to_mono_audio(&chunk) {
                    Ok(mono) => mono,
                    Err(err) => {
                        warn!("stereo downmix failed: {:?}", err);
                        downmix_multi_channel(&chunk, input_channels)
                    }
                }
            } else {
                downmix_multi_channel(&chunk, input_channels)
            };
            let mut processed_chunk = if needs_resample {
                resample_to_target(&mono_chunk, resample_ratio)
            } else {
                mono_chunk
            };

            if !processed_chunk.is_empty() {
                apply_high_pass_filter(
                    &mut processed_chunk,
                    &mut hp_prev_input,
                    &mut hp_prev_output,
                    HIGH_PASS_CUTOFF_HZ,
                );
            }
            let average_magnitude = if processed_chunk.is_empty() {
                0.0
            } else {
                processed_chunk
                    .iter()
                    .map(|sample| sample.abs())
                    .sum::<f32>()
                    / processed_chunk.len() as f32
            };
            if average_magnitude > 0.005 {
                accumulated_silence = 0;
                voiced_samples = voiced_samples.saturating_add(processed_chunk.len());
            } else {
                accumulated_silence = accumulated_silence.saturating_add(processed_chunk.len());
            }

            audio_buffer.extend_from_slice(&processed_chunk);

            let forced_flush = audio_buffer.len() >= max_buffer_samples;
            let heard_enough = audio_buffer.len() >= min_buffer_samples
                && accumulated_silence >= silence_trigger_samples;

            if heard_enough || forced_flush {
                let has_voice = voiced_samples >= TARGET_SAMPLE_RATE / 5; // ~200ms voiced
                if !has_voice {
                    audio_buffer.clear();
                    accumulated_silence = 0;
                    voiced_samples = 0;
                    continue;
                }

                let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
                // TODO: Make this configurable
                params.set_n_threads(8);
                params.set_language(Some("en"));

                whisper_state
                    .full(params, &audio_buffer)
                    .map_err(|e| anyhow::anyhow!("failed to run model: {}", e))?;

                let num_segments = whisper_state.full_n_segments();
                info!("Transcription produced {} segments", num_segments);
                for i in 0..num_segments {
                    if let Some(segment) = whisper_state.get_segment(i) {
                        warn!("Segment: {segment}");
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
                accumulated_silence = 0;
                voiced_samples = 0;
            }
        } else {
            std::thread::sleep(std::time::Duration::from_millis(25));
        }
    }
}

fn downmix_multi_channel(data: &[f32], channels: usize) -> Vec<f32> {
    if channels <= 1 {
        return data.to_vec();
    }
    let mut mono = Vec::with_capacity(data.len() / channels + 1);
    for frame in data.chunks(channels) {
        let sum: f32 = frame.iter().copied().sum();
        mono.push(sum / channels as f32);
    }
    mono
}

fn resample_to_target(chunk: &[f32], ratio: f32) -> Vec<f32> {
    if (ratio - 1.0).abs() < f32::EPSILON || chunk.is_empty() {
        return chunk.to_vec();
    }
    let output_len = ((chunk.len() as f32) * ratio).ceil() as usize;
    let mut output = Vec::with_capacity(output_len.max(1));
    for i in 0..output_len {
        let src_pos = i as f32 / ratio;
        let idx = src_pos.floor() as usize;
        let frac = src_pos - idx as f32;
        let next_idx = if idx + 1 < chunk.len() { idx + 1 } else { idx };
        let a = chunk[idx];
        let b = chunk[next_idx];
        output.push(a * (1.0 - frac) + b * frac);
    }
    output
}

fn apply_high_pass_filter(
    samples: &mut [f32],
    prev_input: &mut f32,
    prev_output: &mut f32,
    cutoff_hz: f32,
) {
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff_hz);
    let dt = 1.0 / TARGET_SAMPLE_RATE as f32;
    let alpha = rc / (rc + dt);
    for sample in samples.iter_mut() {
        let output = alpha * (*prev_output + *sample - *prev_input);
        *prev_input = *sample;
        *prev_output = output;
        *sample = output;
    }
}
