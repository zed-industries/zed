use anyhow::{anyhow, Result};
use hound::{WavReader, WavSpec};
use std::io::Cursor;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct AudioFormat {
    pub sample_rate: u32,
    pub channels: u16,
    pub bits_per_sample: u16,
}

impl Default for AudioFormat {
    fn default() -> Self {
        Self {
            sample_rate: 16000, // Whisper expects 16kHz
            channels: 1,        // Mono
            bits_per_sample: 16,
        }
    }
}

pub struct AudioProcessor {
    target_format: AudioFormat,
}

impl AudioProcessor {
    pub fn new() -> Self {
        Self {
            target_format: AudioFormat::default(),
        }
    }

    /// Process raw audio samples for Whisper input
    /// Whisper expects: 16kHz, mono, f32 samples in range [-1.0, 1.0]
    pub fn process_for_whisper(
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

        // Resample to 16kHz if needed
        if sample_rate != self.target_format.sample_rate {
            processed = self.resample(&processed, sample_rate, self.target_format.sample_rate)?;
        }

        // Normalize to [-1.0, 1.0] range
        self.normalize(&mut processed);

        // Apply pre-emphasis filter (optional, can improve recognition)
        self.apply_preemphasis(&mut processed, 0.97);

        Ok(processed)
    }

    /// Load audio from file and return samples, sample rate, and channels
    pub fn load_audio_file<P: AsRef<Path>>(&self, path: P) -> Result<(Vec<f32>, u32, u16)> {
        let path = path.as_ref();
        
        // Try to load as WAV first
        if let Ok(reader) = WavReader::open(path) {
            return self.load_wav(reader);
        }

        // Could add support for other formats here using symphonia
        Err(anyhow!("Unsupported audio format: {:?}", path))
    }

    /// Load audio from raw bytes (e.g., from VoiceRecording)
    pub fn load_from_bytes(&self, data: &[u8]) -> Result<(Vec<f32>, u32, u16)> {
        let cursor = Cursor::new(data);
        let reader = WavReader::new(cursor)
            .map_err(|e| anyhow!("Failed to parse audio data: {}", e))?;
        
        self.load_wav(reader)
    }

    fn load_wav<R: std::io::Read>(&self, mut reader: WavReader<R>) -> Result<(Vec<f32>, u32, u16)> {
        let spec = reader.spec();
        let sample_rate = spec.sample_rate;
        let channels = spec.channels;
        
        let samples: Result<Vec<f32>> = match spec.sample_format {
            hound::SampleFormat::Float => {
                reader.samples::<f32>()
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| anyhow!("Failed to read float samples: {}", e))
            }
            hound::SampleFormat::Int => {
                match spec.bits_per_sample {
                    16 => {
                        let int_samples: Result<Vec<i16>, _> = reader.samples().collect();
                        Ok(int_samples?
                            .into_iter()
                            .map(|s| s as f32 / i16::MAX as f32)
                            .collect())
                    }
                    24 => {
                        let int_samples: Result<Vec<i32>, _> = reader.samples().collect();
                        Ok(int_samples?
                            .into_iter()
                            .map(|s| s as f32 / (1 << 23) as f32)
                            .collect())
                    }
                    32 => {
                        let int_samples: Result<Vec<i32>, _> = reader.samples().collect();
                        Ok(int_samples?
                            .into_iter()
                            .map(|s| s as f32 / i32::MAX as f32)
                            .collect())
                    }
                    _ => Err(anyhow!("Unsupported bit depth: {}", spec.bits_per_sample)),
                }
            }
        };

        Ok((samples?, sample_rate, channels))
    }

    fn convert_to_mono(&self, samples: &[f32], channels: u16) -> Result<Vec<f32>> {
        if channels == 1 {
            return Ok(samples.to_vec());
        }

        let channels = channels as usize;
        let mono_len = samples.len() / channels;
        let mut mono = Vec::with_capacity(mono_len);

        for chunk in samples.chunks_exact(channels) {
            let sum: f32 = chunk.iter().sum();
            mono.push(sum / channels as f32);
        }

        Ok(mono)
    }

    fn resample(&self, samples: &[f32], from_rate: u32, to_rate: u32) -> Result<Vec<f32>> {
        if from_rate == to_rate {
            return Ok(samples.to_vec());
        }

        // Simple linear interpolation resampling
        let ratio = from_rate as f64 / to_rate as f64;
        let output_len = (samples.len() as f64 / ratio) as usize;
        let mut resampled = Vec::with_capacity(output_len);

        for i in 0..output_len {
            let src_index = i as f64 * ratio;
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

        // Find the maximum absolute value
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

    /// Convert processed audio back to WAV format for debugging
    pub fn save_as_wav<P: AsRef<Path>>(&self, samples: &[f32], path: P) -> Result<()> {
        let spec = WavSpec {
            channels: 1,
            sample_rate: self.target_format.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(path, spec)?;
        
        for &sample in samples {
            let sample_i16 = (sample.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
            writer.write_sample(sample_i16)?;
        }
        
        writer.finalize()?;
        Ok(())
    }

    /// Get audio duration in seconds
    pub fn get_duration(&self, samples: &[f32]) -> f32 {
        samples.len() as f32 / self.target_format.sample_rate as f32
    }

    /// Extract audio features for analysis (RMS, zero-crossing rate, etc.)
    pub fn extract_features(&self, samples: &[f32]) -> AudioFeatures {
        let rms = self.calculate_rms(samples);
        let zcr = self.calculate_zero_crossing_rate(samples);
        let energy = self.calculate_energy(samples);
        
        AudioFeatures {
            rms,
            zero_crossing_rate: zcr,
            energy,
            duration: self.get_duration(samples),
        }
    }

    fn calculate_rms(&self, samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }
        
        let sum_squares: f32 = samples.iter().map(|&s| s * s).sum();
        (sum_squares / samples.len() as f32).sqrt()
    }

    fn calculate_zero_crossing_rate(&self, samples: &[f32]) -> f32 {
        if samples.len() < 2 {
            return 0.0;
        }
        
        let crossings = samples.windows(2)
            .filter(|window| (window[0] >= 0.0) != (window[1] >= 0.0))
            .count();
        
        crossings as f32 / (samples.len() - 1) as f32
    }

    fn calculate_energy(&self, samples: &[f32]) -> f32 {
        samples.iter().map(|&s| s * s).sum()
    }
}

#[derive(Debug, Clone)]
pub struct AudioFeatures {
    pub rms: f32,
    pub zero_crossing_rate: f32,
    pub energy: f32,
    pub duration: f32,
}

impl Default for AudioProcessor {
    fn default() -> Self {
        Self::new()
    }
} 