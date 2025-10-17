use denoise::{Denoiser, DenoiserError};
use rodio::Sample;
use std::time::Duration;

use super::RodioExt;
use super::resample::FixedResampler;
use rodio::ChannelCount;
use rodio::SampleRate;
use rodio::Source;

#[derive(Default)]
enum InnerRSD<S: Source> {
    Transparent(S),
    Denoised(FixedResampler<Denoiser<FixedResampler<S>>>),
    #[default]
    ShouldNotExist,
}

pub struct ResamplingDenoiser<S: Source> {
    inner: InnerRSD<S>,
}

impl<S: Source> ResamplingDenoiser<S> {
    pub fn new(source: S) -> Result<Self, DenoiserError> {
        Ok(ResamplingDenoiser {
            inner: InnerRSD::Transparent(source),
        })
    }
    pub fn set_enabled(&mut self, enabled: bool) -> Result<(), DenoiserError> {
        self.inner = match std::mem::take(&mut self.inner) {
            InnerRSD::Transparent(s) => {
                if enabled {
                    let sr = s.sample_rate();
                    InnerRSD::Denoised(
                        Denoiser::try_new(s.constant_samplerate(denoise::SUPPORTED_SAMPLE_RATE))?
                            .constant_samplerate(sr),
                    )
                } else {
                    InnerRSD::Transparent(s)
                }
            }
            InnerRSD::Denoised(s) => {
                if !enabled {
                    InnerRSD::Transparent(s.into_inner().into_inner().into_inner())
                } else {
                    InnerRSD::Denoised(s)
                }
            }
            InnerRSD::ShouldNotExist => unreachable!(),
        };
        Ok(())
    }
}

impl<S: Source> Source for ResamplingDenoiser<S> {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> ChannelCount {
        match &self.inner {
            // Different types, can't unify :c
            InnerRSD::Transparent(s) => s.channels(),
            InnerRSD::Denoised(s) => s.channels(),
            InnerRSD::ShouldNotExist => unreachable!(),
        }
    }

    fn sample_rate(&self) -> SampleRate {
        match &self.inner {
            // Different types, can't unify :c
            InnerRSD::Transparent(s) => s.sample_rate(),
            InnerRSD::Denoised(s) => s.sample_rate(),
            InnerRSD::ShouldNotExist => unreachable!(),
        }
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

impl<S: Source> Iterator for ResamplingDenoiser<S> {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            InnerRSD::Denoised(denoiser) => denoiser.next(),
            InnerRSD::Transparent(source) => source.next(),
            InnerRSD::ShouldNotExist => unreachable!(),
        }
    }
}
