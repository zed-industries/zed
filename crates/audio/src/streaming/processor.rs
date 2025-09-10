#[cfg(all(feature = "aec-processing", not(feature = "webrtc-processing")))]
use aec_rs::{Aec, AecConfig};
use anyhow::{Result, anyhow};
use log::{debug, info};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering},
    },
    time::Duration,
};
#[cfg(feature = "webrtc-processing")]
use webrtc_audio_processing::{
    Config, EchoCancellation, EchoCancellationSuppressionLevel, GainControl, GainControlMode,
    InitializationConfig, NoiseSuppression, NoiseSuppressionLevel,
};

/// Trait implemented by audio processors used by the streaming pipeline.
/// Implementors should apply in-place echo cancellation/noise handling on
/// interleaved f32 audio frames in the engine format.
pub trait AudioProcessor: Clone + Send + Sync + 'static {
    fn new(enabled: bool) -> Result<Self>
    where
        Self: Sized;

    fn is_enabled(&self) -> bool;

    fn init_capture(&self, channels: usize) -> Result<()>;
    fn init_playback(&self, channels: usize) -> Result<()>;

    fn process_capture_frame(&self, frame: &mut [f32]) -> Result<()>;
    fn process_render_frame(&self, frame: &mut [f32]) -> Result<()>;

    fn set_capture_delay(&self, _stream_delay: Duration) {}
    fn set_playback_delay(&self, _stream_delay: Duration) {}
}

#[cfg(feature = "webrtc-processing")]
#[derive(Clone)]
pub struct WebrtcAudioProcessor(Arc<Inner>);

#[cfg(all(feature = "aec-processing", not(feature = "webrtc-processing")))]
#[derive(Clone)]
pub struct AECAudioProcessor {
    enabled: AtomicBool,
    aec: Mutex<Aec>,
    last_render_i16: Mutex<Vec<i16>>,
    capture_channels: AtomicUsize,
    render_channels: AtomicUsize,
}

#[cfg(all(feature = "aec-processing", not(feature = "webrtc-processing")))]
impl AECAudioProcessor {
    const AEC_SAMPLE_RATE: usize = 16_000;
    const AEC_FRAME_SAMPLES: usize = Self::AEC_SAMPLE_RATE / 100; // 10ms

    fn downsample_to_16k_mono_i16(
        &self,
        input_interleaved_f32: &[f32],
        from_sample_rate: usize,
        channels: usize,
    ) -> Vec<i16> {
        use fixed_resample::{FixedResampler, ResampleQuality};

        let mut resampled = Vec::<f32>::new();
        let mut rs = FixedResampler::<f32, 2>::new(
            std::num::NonZeroUsize::new(channels).unwrap(),
            from_sample_rate,
            Self::AEC_SAMPLE_RATE,
            ResampleQuality::High,
            true,
        );
        rs.process_interleaved(
            input_interleaved_f32,
            |chunk| resampled.extend_from_slice(chunk),
            None,
            false,
        );

        // Downmix to mono if needed
        let mut mono = Vec::<f32>::with_capacity(resampled.len() / channels.max(1));
        if channels > 1 {
            for frame in resampled.chunks_exact(channels) {
                let sum: f32 = frame.iter().copied().sum();
                mono.push(sum / (channels as f32));
            }
        } else {
            mono = resampled;
        }

        // Ensure exactly the AEC frame size (pad with zeros if needed)
        let mut mono = if mono.len() >= Self::AEC_FRAME_SAMPLES {
            mono[0..Self::AEC_FRAME_SAMPLES].to_vec()
        } else {
            let mut tmp = mono;
            tmp.resize(Self::AEC_FRAME_SAMPLES, 0.0);
            tmp
        };

        mono.into_iter()
            .map(|s| (s * 32767.0).clamp(-32768.0, 32767.0) as i16)
            .collect()
    }

    fn upsample_16k_mono_to_engine_interleaved(
        &self,
        mono_i16: &[i16],
        to_sample_rate: usize,
        out_channels: usize,
    ) -> Vec<f32> {
        use fixed_resample::{FixedResampler, ResampleQuality};
        let mono_f32: Vec<f32> = mono_i16.iter().map(|v| (*v as f32) / 32768.0).collect();

        let mut up = Vec::<f32>::new();
        let mut rs = FixedResampler::<f32, 2>::new(
            std::num::NonZeroUsize::new(1).unwrap(),
            Self::AEC_SAMPLE_RATE,
            to_sample_rate,
            ResampleQuality::High,
            true,
        );
        rs.process_interleaved(&mono_f32, |chunk| up.extend_from_slice(chunk), None, false);

        if out_channels <= 1 {
            return up;
        }
        let mut interleaved = Vec::<f32>::with_capacity(up.len() * out_channels);
        for sample in up {
            for _ in 0..out_channels {
                interleaved.push(sample);
            }
        }
        interleaved
    }
}

#[cfg(all(feature = "aec-processing", not(feature = "webrtc-processing")))]
impl AudioProcessor for AECAudioProcessor {
    fn new(enabled: bool) -> Result<Self> {
        let cfg = AecConfig {
            sample_rate: Self::AEC_SAMPLE_RATE as u32,
            filter_length: (Self::AEC_SAMPLE_RATE / 10) as i32, // 100ms
            frame_size: Self::AEC_FRAME_SAMPLES,
            enable_preprocess: true,
        };
        let aec = Aec::new(&cfg);
        info!("init aec_rs audio processor (enabled={enabled})");
        Ok(Self {
            enabled: AtomicBool::new(enabled),
            aec: Mutex::new(aec),
            last_render_i16: Mutex::new(Vec::with_capacity(Self::AEC_FRAME_SAMPLES)),
            capture_channels: AtomicUsize::new(0),
            render_channels: AtomicUsize::new(0),
        })
    }

    fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::SeqCst);
    }

    fn init_capture(&self, channels: usize) -> Result<()> {
        self.capture_channels.store(channels, Ordering::SeqCst);
        Ok(())
    }

    fn init_playback(&self, channels: usize) -> Result<()> {
        self.render_channels.store(channels, Ordering::SeqCst);
        Ok(())
    }

    fn process_render_frame(&self, frame: &mut [f32]) -> Result<()> {
        if !self.is_enabled() {
            return Ok(());
        }
        let channels = self.render_channels.load(Ordering::SeqCst).max(1);
        let from_sr = super::ENGINE_FORMAT.sample_rate.get();
        let mono_i16 = self.downsample_to_16k_mono_i16(frame, from_sr, channels);
        if let Ok(mut buf) = self.last_render_i16.lock() {
            *buf = mono_i16;
        }
        Ok(())
    }

    fn process_capture_frame(&self, frame: &mut [f32]) -> Result<()> {
        if !self.is_enabled() {
            return Ok(());
        }
        let capture_channels = self.capture_channels.load(Ordering::SeqCst).max(1);
        let from_sr = super::ENGINE_FORMAT.sample_rate.get();

        let mut cap_i16 = self.downsample_to_16k_mono_i16(frame, from_sr, capture_channels);
        let ref_i16 = {
            let guard = self.last_render_i16.lock().unwrap();
            if guard.len() == Self::AEC_FRAME_SAMPLES {
                guard.clone()
            } else {
                vec![0i16; Self::AEC_FRAME_SAMPLES]
            }
        };

        let mut out_i16 = vec![0i16; Self::AEC_FRAME_SAMPLES];
        {
            let mut aec = self.aec.lock().unwrap();
            aec.cancel_echo(&mut cap_i16, &mut ref_i16.clone(), &mut out_i16);
        }

        let to_sr = super::ENGINE_FORMAT.sample_rate.get();
        let processed =
            self.upsample_16k_mono_to_engine_interleaved(&out_i16, to_sr, capture_channels);

        if processed.len() >= frame.len() {
            frame.copy_from_slice(&processed[..frame.len()]);
        } else {
            frame[..processed.len()].copy_from_slice(&processed);
            for s in frame.iter_mut().skip(processed.len()) {
                *s = 0.0;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "webrtc-processing")]
struct Inner {
    inner: Mutex<Option<webrtc_audio_processing::Processor>>,
    config: Mutex<Config>,
    capture_delay: AtomicU64,
    playback_delay: AtomicU64,
    enabled: AtomicBool,
    capture_channels: AtomicUsize,
    playback_channels: AtomicUsize,
}

#[cfg(feature = "webrtc-processing")]
impl WebrtcAudioProcessor {
    fn init(&self) -> Result<()> {
        let playback_channels = self.0.playback_channels.load(Ordering::SeqCst);
        let capture_channels = self.0.playback_channels.load(Ordering::SeqCst);
        let mut processor = webrtc_audio_processing::Processor::new(&InitializationConfig {
            num_capture_channels: capture_channels as i32,
            num_render_channels: playback_channels as i32,
            ..InitializationConfig::default()
        })?;
        processor.set_config(self.0.config.lock().unwrap().clone());
        *self.0.inner.lock().unwrap() = Some(processor);
        Ok(())
    }

    fn update_stream_delay(&self) {
        let playback = self.0.playback_delay.load(Ordering::Relaxed);
        let capture = self.0.capture_delay.load(Ordering::Relaxed);
        let total = playback + capture;
        let mut config = self.0.config.lock().unwrap();
        config.echo_cancellation.as_mut().unwrap().stream_delay_ms = Some(total as i32);
        if let Some(processor) = self.0.inner.lock().unwrap().as_mut() {
            processor.set_config(config.clone());
        }
    }
}

// Implement AudioProcessor for WebrtcAudioProcessor by delegating to the inner processor.
#[cfg(feature = "webrtc-processing")]
impl AudioProcessor for WebrtcAudioProcessor {
    fn new(enabled: bool) -> Result<Self> {
        let suppression_level = EchoCancellationSuppressionLevel::High;
        let config = Config {
            echo_cancellation: Some(EchoCancellation {
                suppression_level,
                stream_delay_ms: None,
                enable_delay_agnostic: true,
                enable_extended_filter: true,
            }),
            enable_high_pass_filter: true,
            enable_transient_suppressor: true,
            gain_control: Some(GainControl {
                mode: GainControlMode::AdaptiveDigital,
                target_level_dbfs: 3,
                compression_gain_db: 15,
                enable_limiter: true,
            }),
            noise_suppression: Some(NoiseSuppression {
                suppression_level: NoiseSuppressionLevel::VeryHigh,
            }),
            ..Config::default()
        };
        info!("init webrtc audio processor (enabled={enabled})");
        Ok(Self(Arc::new(Inner {
            inner: Mutex::new(None),
            config: Mutex::new(config),
            capture_delay: Default::default(),
            playback_delay: Default::default(),
            enabled: AtomicBool::new(enabled),
            capture_channels: Default::default(),
            playback_channels: Default::default(),
        })))
    }

    fn is_enabled(&self) -> bool {
        self.0.enabled.load(Ordering::SeqCst)
    }

    fn init_capture(&self, channels: usize) -> Result<()> {
        self.0.capture_channels.store(channels, Ordering::SeqCst);
        if self.0.playback_channels.load(Ordering::SeqCst) > 0 {
            self.init()?;
        }
        Ok(())
    }

    fn init_playback(&self, channels: usize) -> Result<()> {
        self.0.playback_channels.store(channels, Ordering::SeqCst);
        if self.0.capture_channels.load(Ordering::SeqCst) > 0 {
            self.init()?;
        }
        Ok(())
    }

    fn process_capture_frame(&self, frame: &mut [f32]) -> Result<()> {
        if !self.is_enabled() {
            return Ok(());
        }
        if let Some(processor) = self.0.inner.lock().unwrap().as_mut() {
            processor
                .process_capture_frame(frame)
                .map_err(|err| anyhow!("Error processing capture frame: {}", err))
        } else {
            Ok(())
        }
    }

    fn process_render_frame(&self, frame: &mut [f32]) -> Result<()> {
        if !self.is_enabled() {
            return Ok(());
        }
        if let Some(processor) = self.0.inner.lock().unwrap().as_mut() {
            processor
                .process_render_frame(frame)
                .map_err(|err| anyhow!("Error processing render frame: {}", err))
        } else {
            Ok(())
        }
    }

    fn set_capture_delay(&self, stream_delay: Duration) {
        let new_val = stream_delay.as_millis() as u64;
        if let Ok(old_val) =
            self.0
                .capture_delay
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |val| {
                    if new_val.abs_diff(val) > 1 {
                        Some(new_val)
                    } else {
                        None
                    }
                })
        {
            debug!("changing capture delay from {old_val} to {new_val}");
            self.update_stream_delay();
        }
    }

    fn set_playback_delay(&self, stream_delay: Duration) {
        let new_val = stream_delay.as_millis() as u64;
        if let Ok(old_val) =
            self.0
                .playback_delay
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |val| {
                    if new_val.abs_diff(val) > 1 {
                        Some(new_val)
                    } else {
                        None
                    }
                })
        {
            debug!("changing playback delay from {old_val} to {new_val}");
            self.update_stream_delay();
        }
    }
}
