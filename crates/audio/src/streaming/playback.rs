use std::{
    num::NonZeroUsize,
    ops::ControlFlow,
    time::{Duration, Instant},
};

use anyhow::{Result, anyhow};
use fixed_resample::{FixedResampler, ResampleQuality};
use log::{debug, error, info, warn};
use ringbuf::{
    HeapCons as Consumer, HeapProd as Producer,
    traits::{Consumer as _, Observer as _, Producer as _, Split},
};
use rodio::cpal::{
    self, Device, Sample, SampleFormat,
    traits::{DeviceTrait, StreamTrait},
};
use tokio::sync::{mpsc, oneshot};

use crate::streaming::processor::AudioProcessor;

use super::{
    AudioFormat, DURATION_10MS, DURATION_20MS, DefaultAudioProcessor, ENGINE_FORMAT, SAMPLE_RATE,
    device::{Direction, StreamConfigWithFormat, find_device, find_output_stream_config},
};

pub trait AudioSource: Send + 'static {
    fn tick(&mut self, buf: &mut [f32]) -> Result<ControlFlow<(), usize>>;
}

#[derive(Debug, Clone)]
pub struct AudioPlayback {
    source_sender: mpsc::Sender<Box<dyn AudioSource>>,
}

impl AudioPlayback {
    pub async fn build(
        host: &cpal::Host,
        device: Option<&str>,
        processor: DefaultAudioProcessor,
    ) -> Result<Self> {
        let device = find_device(host, Direction::Playback, device)?;
        let stream_config = find_output_stream_config(&device, &ENGINE_FORMAT)?;

        let buffer_size = ENGINE_FORMAT.sample_count(DURATION_20MS) * 32;
        let (producer, consumer) = ringbuf::HeapRb::<f32>::new(buffer_size).split();

        let (source_sender, source_receiver) = mpsc::channel(16);
        let (init_tx, init_rx) = oneshot::channel();

        std::thread::spawn(move || {
            if let Err(err) = audio_thread_priority::promote_current_thread_to_real_time(
                buffer_size as u32,
                ENGINE_FORMAT.sample_rate.get(),
            ) {
                warn!("failed to set playback thread to realtime priority: {err:?}");
            }
            let stream = match start_playback_stream(&device, &stream_config, processor, consumer) {
                Ok(stream) => {
                    init_tx.send(Ok(())).unwrap();
                    stream
                }
                Err(err) => {
                    init_tx.send(Err(err)).unwrap();
                    return;
                }
            };
            playback_loop(producer, source_receiver);
            drop(stream);
        });

        init_rx.await??;
        Ok(Self { source_sender })
    }

    pub async fn add_source(&self, source: impl AudioSource) -> Result<()> {
        self.source_sender
            .send(Box::new(source))
            .await
            .map_err(|_| anyhow!("failed to add audio source: playback loop dead"))?;
        Ok(())
    }
}

fn playback_loop(
    mut producer: Producer<f32>,
    mut source_receiver: mpsc::Receiver<Box<dyn AudioSource>>,
) {
    info!("playback loop start");

    let tick_duration = DURATION_20MS;
    let buffer_size = ENGINE_FORMAT.sample_count(tick_duration);
    let mut work_buf = vec![0.; buffer_size];
    let mut out_buf = vec![0.; buffer_size];
    let mut sources: Vec<Box<dyn AudioSource>> = vec![];

    // todo: do we want this?
    let initial_latency = ENGINE_FORMAT.sample_count(DURATION_20MS);
    let initial_silence = vec![0.; initial_latency];
    let n = producer.push_slice(&initial_silence);
    debug_assert_eq!(n, initial_silence.len());

    let mut tick = 0;
    loop {
        let start = Instant::now();

        // pull incoming sources
        loop {
            match source_receiver.try_recv() {
                Ok(source) => {
                    info!("add new track to decoder");
                    sources.push(source);
                }
                Err(mpsc::error::TryRecvError::Empty) => break,
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    info!("stop playback mixer loop: channel closed");
                    return;
                }
            }
        }

        out_buf.fill(0.);
        sources.retain_mut(|source| match source.tick(&mut work_buf) {
            Ok(ControlFlow::Continue(count)) => {
                for i in 0..count {
                    out_buf[i] += work_buf[i];
                }
                if count < work_buf.len() {
                    debug!(
                        "audio source xrun: missing {} of {}",
                        work_buf.len() - count,
                        work_buf.len()
                    );
                }
                true
            }
            Ok(ControlFlow::Break(())) => {
                debug!("remove decoder: closed");
                false
            }
            Err(err) => {
                warn!("remove decoder: failed {err:?}");
                false
            }
        });

        let len = producer.push_slice(&out_buf[..]);
        if len < out_buf.len() {
            warn!(
                "xrun: failed to push {} of {}",
                out_buf.len() - len,
                out_buf.len()
            );
        }

        debug!("tick {tick} took {:?} pushed {len}", start.elapsed());
        if start.elapsed() > tick_duration {
            warn!(
                "playback thread tick exceeded interval (took {:?})",
                start.elapsed()
            );
        } else {
            let sleep_time = tick_duration.saturating_sub(start.elapsed());
            spin_sleep::sleep(sleep_time);
        }
        tick += 1;
    }
}

fn start_playback_stream(
    device: &Device,
    stream_config: &StreamConfigWithFormat,
    processor: DefaultAudioProcessor,
    consumer: Consumer<f32>,
) -> Result<cpal::Stream> {
    let config = &stream_config.config;
    let format = stream_config.audio_format();
    processor.init_playback(config.channels as usize)?;
    let resampler = FixedResampler::new(
        NonZeroUsize::new(format.channel_count.get() as usize).unwrap(),
        SAMPLE_RATE.get(),
        format.sample_rate.get(),
        ResampleQuality::High,
        true,
    );
    let state = PlaybackState {
        consumer,
        format,
        processor,
        resampler,
    };
    let stream = match stream_config.sample_format {
        SampleFormat::I8 => build_playback_stream::<i8>(device, config, state),
        SampleFormat::I16 => build_playback_stream::<i16>(device, config, state),
        SampleFormat::I32 => build_playback_stream::<i32>(device, config, state),
        SampleFormat::F32 => build_playback_stream::<f32>(device, config, state),
        sample_format => {
            log::error!("Unsupported sample format '{sample_format}'");
            Err(cpal::BuildStreamError::StreamConfigNotSupported)
        }
    }?;
    info!(
        "start playback stream on {} with {format:?}",
        device.name()?
    );
    stream.play()?;
    Ok(stream)
}

struct PlaybackState {
    format: AudioFormat,
    resampler: FixedResampler<f32, 2>,
    #[allow(unused)]
    processor: DefaultAudioProcessor,
    consumer: Consumer<f32>,
}

fn build_playback_stream<S: dasp_sample::FromSample<f32> + cpal::SizedSample + Default>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut state: PlaybackState,
) -> Result<cpal::Stream, cpal::BuildStreamError> {
    let frame_size = state.format.sample_count(DURATION_10MS);
    let mut unprocessed: Vec<f32> = Vec::with_capacity(frame_size);
    let mut processed: Vec<f32> = Vec::with_capacity(frame_size);
    let mut resampled: Vec<f32> = Vec::with_capacity(frame_size);
    let mut tick = 0;
    let mut last_warning = Instant::now();
    let mut underflows = 0;

    device.build_output_stream::<S, _, _>(
        config,
        move |data: &mut [S], info: &_| {
            let delay = {
                let output_delay = info
                    .timestamp()
                    .callback
                    .duration_since(&info.timestamp().playback)
                    .unwrap_or_default();
                let resampler_delay = Duration::from_secs_f32(state.resampler.output_delay() as f32 / state.format.sample_rate.get() as f32);
                output_delay + resampler_delay
            };

            if tick % 100 == 0 {
                debug!("callback tick {tick} len={} delay={delay:?}", data.len());
            }


            //#[cfg(feature = "audio-processing")]
            state.processor.set_playback_delay(delay);

            // pop from channel
            unprocessed.extend(state.consumer.pop_iter());

            // process
            let mut chunks = unprocessed.chunks_exact_mut(frame_size);
            for chunk in &mut chunks {
                //#[cfg(feature = "audio-processing")]
                state.processor.process_render_frame(chunk).unwrap();
                processed.extend_from_slice(chunk);
            }
            // cleanup
            let remainder_len = chunks.into_remainder().len();
            let end = unprocessed.len() - remainder_len;
            unprocessed.copy_within(end.., 0);
            unprocessed.truncate(remainder_len);

            // resample
            state.resampler.process_interleaved(&processed, |samples|{
                resampled.extend_from_slice(samples);
            } , None, false);
            processed.clear();


            // copy to out
            let out_len = resampled.len().min(data.len());
            let remaining = resampled.len() - out_len;
            for (i, sample) in data[..out_len].iter_mut().enumerate() {
                *sample = resampled[i].to_sample()
            }
            resampled.copy_within(out_len.., 0);
            resampled.truncate(remaining);

            // debug!("out_len {out_len} resampled_remaining {} processed_remaining {}", resampled.len(), processed.len());
            if out_len < data.len() {
                let now = Instant::now();
                if now.duration_since(last_warning) > Duration::from_secs(1) {
                    warn!(
                        "[tick {tick}] playback xrun: {} of {} samples missing (buffered {}) (+ {} previous)",
                        data.len() - out_len,
                        data.len(),
                        unprocessed.len() + state.consumer.occupied_len(),
                        underflows
                    );
                    underflows += 1;
                    last_warning = now;
                }
            }
            tick += 1;
        },
        |err| {
            error!("an error occurred on output stream: {}", err);
        },
        None,
    )
}
