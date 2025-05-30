use crate::{config::AudioConfig, Result, SpeechError, error::AudioError};
use futures::Stream;
use hound::WavReader;
use std::io::Cursor;
use std::path::Path;
use std::pin::Pin;

pub type AudioStream = Pin<Box<dyn Stream<Item = Result<Vec<f32>>> + Send>>;

#[derive(Debug, Clone)]
pub struct AudioFormat {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
}

impl From<AudioConfig> for AudioFormat {
    fn from(config: AudioConfig) -> Self {
        Self {
            sample_rate: config.sample_rate,
            channels: config.channels,
            bits_per_sample: config.bits_per_sample,
        }
    }
}

pub struct AudioProcessor {
    config: AudioConfig,
    target_format: AudioFormat,
}

impl AudioProcessor {
    pub fn new(config: AudioConfig) -> Result<Self> {
        let target_format = AudioFormat::from(config.clone());
        
        Ok(Self {
            config,
            target_format,
        })
    }

    /// Process audio samples for STT input
    pub fn process_for_stt(
        &self,
        samples: &[f32],
        sample_rate: u32,
        channels: u16,
    ) -> Result<Vec<f32>> {
        let mut processed = samples.to_vec();

        // Convert to mono if needed
        if channels > 1 {
            processed = self.convert_to_mono(&processed, channels)?;
        }

        // Resample to target sample rate if needed
        if sample_rate != self.target_format.sample_rate {
            processed = self.resample(&processed, sample_rate, self.target_format.sample_rate)?;
        }

        // Normalize to [-1.0, 1.0] range
        self.normalize(&mut processed);

        // Apply pre-emphasis filter for better recognition
        self.apply_preemphasis(&mut processed, 0.97);

        Ok(processed)
    }

    /// Load audio from file
    pub fn load_audio_file<P: AsRef<Path>>(&self, path: P) -> Result<(Vec<f32>, u32, u16)> {
        let path = path.as_ref();
        
        // Try to load as WAV
        if let Ok(reader) = WavReader::open(path) {
            return self.load_wav(reader);
        }

        Err(SpeechError::Audio(AudioError::UnsupportedFormat))
    }

    /// Load audio from raw bytes
    pub fn load_from_bytes(&self, data: &[u8]) -> Result<(Vec<f32>, u32, u16)> {
        let cursor = Cursor::new(data);
        let reader = WavReader::new(cursor)
            .map_err(|e| AudioError::ParseError(e.to_string()))?;
        
        self.load_wav(reader)
    }

    /// Decode audio from raw bytes (static method for convenience)
    pub fn decode_audio(data: &[u8]) -> Result<(Vec<f32>, u32, u16)> {
        let cursor = Cursor::new(data);
        let reader = WavReader::new(cursor)
            .map_err(|e| AudioError::ParseError(e.to_string()))?;
        
        // Create a temporary processor with default config
        let default_config = AudioConfig::default();
        let processor = AudioProcessor::new(default_config)?;
        processor.load_wav(reader)
    }

    /// Convert i16 samples to f32
    pub fn i16_to_f32(&self, samples: &[i16]) -> Vec<f32> {
        samples.iter()
            .map(|&sample| sample as f32 / i16::MAX as f32)
            .collect()
    }

    /// Convert f32 samples to i16
    pub fn f32_to_i16(&self, samples: &[f32]) -> Vec<i16> {
        samples.iter()
            .map(|&sample| (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
            .collect()
    }

    /// Detect voice activity in audio samples
    pub fn detect_voice_activity(&self, samples: &[f32]) -> bool {
        if samples.is_empty() {
            return false;
        }

        // Calculate RMS (Root Mean Square) energy
        let rms = (samples.iter()
            .map(|&s| s * s)
            .sum::<f32>() / samples.len() as f32)
            .sqrt();

        rms > self.config.voice_activation_threshold
    }

    /// Split audio into chunks for streaming processing
    pub fn chunk_audio(&self, samples: &[f32], chunk_size: usize) -> Vec<Vec<f32>> {
        samples.chunks(chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    /// Get audio duration in seconds
    pub fn get_duration(&self, samples: &[f32]) -> f32 {
        samples.len() as f32 / self.target_format.sample_rate as f32
    }

    // Private helper methods

    fn load_wav<R: std::io::Read>(&self, mut reader: WavReader<R>) -> Result<(Vec<f32>, u32, u16)> {
        let spec = reader.spec();
        let sample_rate = spec.sample_rate;
        let channels = spec.channels;

        let samples: Result<Vec<f32>> = match spec.sample_format {
            hound::SampleFormat::Float => {
                reader.samples::<f32>()
                    .collect::<std::result::Result<Vec<_>, _>>()
                    .map_err(|e| AudioError::ParseError(e.to_string()).into())
            }
            hound::SampleFormat::Int => {
                match spec.bits_per_sample {
                    16 => {
                        let int_samples: std::result::Result<Vec<i16>, _> = reader.samples().collect();
                        int_samples
                            .map(|samples| self.i16_to_f32(&samples))
                            .map_err(|e| AudioError::ParseError(e.to_string()).into())
                    }
                    32 => {
                        let int_samples: std::result::Result<Vec<i32>, _> = reader.samples().collect();
                        int_samples
                            .map(|samples| {
                                samples.iter()
                                    .map(|&s| s as f32 / i32::MAX as f32)
                                    .collect()
                            })
                            .map_err(|e| AudioError::ParseError(e.to_string()).into())
                    }
                    _ => Err(AudioError::UnsupportedFormat.into()),
                }
            }
        };

        samples.map(|s| (s, sample_rate, channels))
    }

    fn convert_to_mono(&self, samples: &[f32], channels: u16) -> Result<Vec<f32>> {
        if channels == 1 {
            return Ok(samples.to_vec());
        }

        let mono_samples: Vec<f32> = samples
            .chunks_exact(channels as usize)
            .map(|chunk| chunk.iter().sum::<f32>() / channels as f32)
            .collect();

        Ok(mono_samples)
    }

    fn resample(&self, samples: &[f32], from_rate: u32, to_rate: u32) -> Result<Vec<f32>> {
        if from_rate == to_rate {
            return Ok(samples.to_vec());
        }

        // Simple linear interpolation resampling
        let ratio = to_rate as f64 / from_rate as f64;
        let output_len = (samples.len() as f64 * ratio) as usize;
        let mut resampled = Vec::with_capacity(output_len);

        for i in 0..output_len {
            let src_index = i as f64 / ratio;
            let src_index_floor = src_index.floor() as usize;
            let src_index_ceil = (src_index_floor + 1).min(samples.len() - 1);
            let fraction = src_index - src_index_floor as f64;

            if src_index_floor < samples.len() {
                let sample = if src_index_ceil == src_index_floor {
                    samples[src_index_floor]
                } else {
                    let a = samples[src_index_floor];
                    let b = samples[src_index_ceil];
                    a + (b - a) * fraction as f32
                };
                resampled.push(sample);
            }
        }

        Ok(resampled)
    }

    fn normalize(&self, samples: &mut [f32]) {
        if samples.is_empty() {
            return;
        }

        let max_abs = samples.iter()
            .map(|&s| s.abs())
            .fold(0.0f32, f32::max);

        if max_abs > 0.0 && max_abs != 1.0 {
            let scale = 1.0 / max_abs;
            for sample in samples.iter_mut() {
                *sample *= scale;
            }
        }
    }

    fn apply_preemphasis(&self, samples: &mut [f32], alpha: f32) {
        if samples.len() < 2 {
            return;
        }

        // Apply pre-emphasis filter: y[n] = x[n] - alpha * x[n-1]
        for i in (1..samples.len()).rev() {
            samples[i] -= alpha * samples[i - 1];
        }
    }
}

/// Audio capture utilities
pub mod capture {
    use super::*;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use futures::channel::mpsc;
    use std::sync::Arc;
    use parking_lot::Mutex;

    #[allow(dead_code)]
    pub struct AudioCapture {
        config: AudioConfig,
        stream: Option<cpal::Stream>,
    }

    impl AudioCapture {
        pub fn new(config: AudioConfig) -> Result<Self> {
            Ok(Self {
                config,
                stream: None,
            })
        }

        pub fn start_capture(&mut self) -> Result<AudioStream> {
            let host = cpal::default_host();
            let device = host
                .default_input_device()
                .ok_or(AudioError::DeviceNotAvailable)?;

            let config = device
                .default_input_config()
                .map_err(|e| AudioError::StreamError(e.to_string()))?;

            let (sender, receiver) = mpsc::unbounded();
            let sender = Arc::new(Mutex::new(sender));

            let stream = device
                .build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        if let Some(sender) = sender.try_lock() {
                            let _ = sender.unbounded_send(Ok(data.to_vec()));
                        }
                    },
                    |err| {
                        log::error!("Audio input stream error: {}", err);
                    },
                    None,
                )?;

            stream.play()?;
            self.stream = Some(stream);

            Ok(Box::pin(receiver))
        }
    }
}

/// Audio playback utilities
pub mod playback {
    use super::*;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use std::sync::Arc;
    use parking_lot::Mutex;

    #[allow(dead_code)]
    pub struct AudioPlayback {
        config: AudioConfig,
        stream: Option<cpal::Stream>,
    }

    impl AudioPlayback {
        pub fn new(config: AudioConfig) -> Result<Self> {
            Ok(Self { config, stream: None })
        }

        pub async fn play_samples(&self, samples: Vec<f32>) -> Result<()> {
            let host = cpal::default_host();
            let device = host
                .default_output_device()
                .ok_or(AudioError::DeviceNotAvailable)?;

            let config = device
                .default_output_config()
                .map_err(|e| AudioError::StreamError(e.to_string()))?;

            let samples = Arc::new(Mutex::new(samples));
            let sample_index = Arc::new(Mutex::new(0));

            let stream = device
                .build_output_stream(
                    &config.into(),
                    {
                        let samples = samples.clone();
                        let sample_index = sample_index.clone();
                        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                            let samples = samples.lock();
                            let mut index = sample_index.lock();

                            for sample in data.iter_mut() {
                                if *index < samples.len() {
                                    *sample = samples[*index];
                                    *index += 1;
                                } else {
                                    *sample = 0.0;
                                }
                            }
                        }
                    },
                    |err| {
                        log::error!("Audio output stream error: {}", err);
                    },
                    None,
                )?;

            stream.play()?;

            // Wait for playback to complete
            while *sample_index.lock() < samples.lock().len() {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }

            Ok(())
        }
    }
}

/// Audio processing utilities for speech recognition
pub mod utils {
    /// Convert audio bytes (from i16 samples) to f32 samples for STT processing
    pub fn bytes_to_f32_samples(audio_bytes: &[u8]) -> Vec<f32> {
        audio_bytes
            .chunks_exact(2)
            .map(|chunk| {
                let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
                sample as f32 / i16::MAX as f32
            })
            .collect()
    }
    
    /// Convert stereo samples to mono by averaging channels
    pub fn stereo_to_mono(samples: &[f32]) -> Vec<f32> {
        samples
            .chunks_exact(2)
            .map(|chunk| (chunk[0] + chunk[1]) / 2.0)
            .collect()
    }
    
    /// Simple downsampling by skipping samples (not ideal but works for basic resampling)
    pub fn downsample(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
        if from_rate <= to_rate {
            return samples.to_vec();
        }
        
        let downsample_ratio = from_rate as f32 / to_rate as f32;
        let target_len = (samples.len() as f32 / downsample_ratio) as usize;
        let mut resampled = Vec::with_capacity(target_len);
        
        for i in 0..target_len {
            let src_idx = (i as f32 * downsample_ratio) as usize;
            if src_idx < samples.len() {
                resampled.push(samples[src_idx]);
            }
        }
        
        resampled
    }
    
    /// Prepare audio data for Whisper STT (converts to mono 16kHz f32 samples)
    pub fn prepare_for_whisper(
        audio_bytes: &[u8], 
        sample_rate: u32, 
        channels: u32
    ) -> Vec<f32> {
        // Convert bytes to f32 samples
        let mut samples = bytes_to_f32_samples(audio_bytes);
        
        // Convert stereo to mono if needed
        if channels == 2 {
            samples = stereo_to_mono(&samples);
        }
        
        // Resample to 16kHz if needed (Whisper's expected rate)
        if sample_rate != 16000 {
            samples = downsample(&samples, sample_rate, 16000);
        }
        
        samples
    }
} 