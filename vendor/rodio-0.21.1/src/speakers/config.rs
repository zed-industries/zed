use std::num::NonZero;

use crate::{math::nz, stream::DeviceSinkConfig, ChannelCount, SampleRate};

/// Describes the output stream's configuration
#[derive(Copy, Clone, Debug)]
pub struct OutputConfig {
    /// The number of channels
    pub channel_count: ChannelCount,
    /// The sample rate the audio card will be playing back at
    pub sample_rate: SampleRate,
    /// The buffersize, see a thorough explanation in SpeakerBuilder::with_buffer_size
    pub buffer_size: cpal::BufferSize,
    /// The sample format used by the audio card.
    /// Note we will always convert to this from f32
    pub sample_format: cpal::SampleFormat,
}
impl OutputConfig {
    pub(crate) fn supported_given(&self, supported: &cpal::SupportedStreamConfigRange) -> bool {
        let buffer_ok = match (self.buffer_size, supported.buffer_size()) {
            (cpal::BufferSize::Default, _) | (_, cpal::SupportedBufferSize::Unknown) => true,
            (
                cpal::BufferSize::Fixed(n_frames),
                cpal::SupportedBufferSize::Range {
                    min: min_samples,
                    max: max_samples,
                },
            ) => {
                let n_samples = n_frames * self.channel_count.get() as u32;
                (*min_samples..=*max_samples).contains(&n_samples)
            }
        };

        buffer_ok
            && self.channel_count.get() == supported.channels()
            && self.sample_format == supported.sample_format()
            && self.sample_rate.get() <= supported.max_sample_rate()
            && self.sample_rate.get() >= supported.min_sample_rate()
    }

    pub(crate) fn with_f32_samples(&self) -> Self {
        let mut this = *self;
        this.sample_format = cpal::SampleFormat::F32;
        this
    }

    pub(crate) fn into_cpal_config(self) -> crate::stream::DeviceSinkConfig {
        DeviceSinkConfig {
            channel_count: self.channel_count,
            sample_rate: self.sample_rate,
            buffer_size: self.buffer_size,
            sample_format: self.sample_format,
        }
    }
}

impl From<cpal::SupportedStreamConfig> for OutputConfig {
    fn from(value: cpal::SupportedStreamConfig) -> Self {
        let buffer_size = match value.buffer_size() {
            cpal::SupportedBufferSize::Range { .. } => cpal::BufferSize::Default,
            cpal::SupportedBufferSize::Unknown => cpal::BufferSize::Default,
        };
        Self {
            channel_count: NonZero::new(value.channels())
                .expect("A supported config never has 0 channels"),
            sample_rate: NonZero::new(value.sample_rate())
                .expect("A supported config produces samples"),
            buffer_size,
            sample_format: value.sample_format(),
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            channel_count: nz!(1),
            sample_rate: nz!(44_100),
            buffer_size: cpal::BufferSize::Default,
            sample_format: cpal::SampleFormat::F32,
        }
    }
}
