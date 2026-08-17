use std::io::{Read, Seek, SeekFrom};
use std::sync::Arc;
use std::time::Duration;

use crate::source::SeekError;
use crate::{Sample, Source};

use crate::common::{ChannelCount, SampleRate};

use dasp_sample::Sample as _;
use dasp_sample::I24;
use hound::{SampleFormat, WavReader};

/// Decoder for the WAV format.
pub struct WavDecoder<R>
where
    R: Read + Seek,
{
    reader: SamplesIterator<R>,
    total_duration: Duration,
    sample_rate: SampleRate,
    channels: ChannelCount,
}

impl<R> WavDecoder<R>
where
    R: Read + Seek,
{
    /// Attempts to decode the data as WAV.
    pub fn new(mut data: R) -> Result<WavDecoder<R>, R> {
        if !is_wave(data.by_ref()) {
            return Err(data);
        }

        let reader = WavReader::new(data).expect("should still be wav");
        let spec = reader.spec();
        let len = reader.len() as u64;
        let reader = SamplesIterator {
            reader,
            samples_read: 0,
        };

        let sample_rate = spec.sample_rate;
        let channels = spec.channels;
        assert!(channels > 0);

        let total_duration = {
            let data_rate = sample_rate as u64 * channels as u64;
            let secs = len / data_rate;
            let nanos = ((len % data_rate) * 1_000_000_000) / data_rate;
            Duration::new(secs, nanos as u32)
        };

        Ok(WavDecoder {
            reader,
            total_duration,
            sample_rate: SampleRate::new(sample_rate)
                .expect("wav should have a sample rate higher then zero"),
            channels: ChannelCount::new(channels).expect("wav should have a least one channel"),
        })
    }

    #[inline]
    pub fn into_inner(self) -> R {
        self.reader.reader.into_inner()
    }
}

struct SamplesIterator<R>
where
    R: Read + Seek,
{
    reader: WavReader<R>,
    samples_read: u32, // wav header is u32 so this suffices
}

impl<R> Iterator for SamplesIterator<R>
where
    R: Read + Seek,
{
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.samples_read += 1;
        let spec = self.reader.spec();
        let next_sample: Option<Self::Item> =
            match (spec.sample_format, spec.bits_per_sample as u32) {
                (SampleFormat::Float, bits) => {
                    if bits == 32 {
                        let next_f32: Option<Result<f32, _>> = self.reader.samples().next();
                        next_f32.and_then(|value| value.ok())
                    } else {
                        #[cfg(feature = "tracing")]
                        tracing::error!("Unsupported WAV float bit depth: {}", bits);
                        #[cfg(not(feature = "tracing"))]
                        eprintln!("Unsupported WAV float bit depth: {}", bits);
                        None
                    }
                }

                (SampleFormat::Int, 8) => {
                    let next_i8: Option<Result<i8, _>> = self.reader.samples().next();
                    next_i8.and_then(|value| value.ok().map(|value| value.to_sample()))
                }
                (SampleFormat::Int, 16) => {
                    let next_i16: Option<Result<i16, _>> = self.reader.samples().next();
                    next_i16.and_then(|value| value.ok().map(|value| value.to_sample()))
                }
                (SampleFormat::Int, 24) => {
                    let next_i24_in_i32: Option<Result<i32, _>> = self.reader.samples().next();
                    next_i24_in_i32.and_then(|value| {
                        value.ok().and_then(I24::new).map(|value| value.to_sample())
                    })
                }
                (SampleFormat::Int, 32) => {
                    let next_i32: Option<Result<i32, _>> = self.reader.samples().next();
                    next_i32.and_then(|value| value.ok().map(|value| value.to_sample()))
                }
                (SampleFormat::Int, bits) => {
                    // Unofficial WAV integer bit depth, try to handle it anyway
                    let next_i32: Option<Result<i32, _>> = self.reader.samples().next();
                    if bits <= 32 {
                        next_i32.and_then(|value| {
                            value.ok().map(|value| (value << (32 - bits)).to_sample())
                        })
                    } else {
                        #[cfg(feature = "tracing")]
                        tracing::error!("Unsupported WAV integer bit depth: {}", bits);
                        #[cfg(not(feature = "tracing"))]
                        eprintln!("Unsupported WAV integer bit depth: {}", bits);
                        None
                    }
                }
            };
        next_sample
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.reader.len() - self.samples_read) as usize;
        (len, Some(len))
    }
}

impl<R> ExactSizeIterator for SamplesIterator<R> where R: Read + Seek {}

impl<R> Source for WavDecoder<R>
where
    R: Read + Seek,
{
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        self.channels
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        Some(self.total_duration)
    }

    #[inline]
    fn try_seek(&mut self, pos: Duration) -> Result<(), SeekError> {
        let file_len = self.reader.reader.duration();

        let new_pos = pos.as_secs_f32() * self.sample_rate().get() as f32;
        let new_pos = new_pos as u32;
        let new_pos = new_pos.min(file_len); // saturate pos at the end of the source

        // make sure the next sample is for the right channel
        let to_skip = self.reader.samples_read % self.channels().get() as u32;

        self.reader
            .reader
            .seek(new_pos)
            .map_err(Arc::new)
            .map_err(SeekError::HoundDecoder)?;
        self.reader.samples_read = new_pos * self.channels().get() as u32;

        for _ in 0..to_skip {
            self.next();
        }

        Ok(())
    }
}

impl<R> Iterator for WavDecoder<R>
where
    R: Read + Seek,
{
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.reader.size_hint()
    }
}

impl<R> ExactSizeIterator for WavDecoder<R> where R: Read + Seek {}

/// Returns true if the stream contains WAV data, then resets it to where it was.
fn is_wave<R>(mut data: R) -> bool
where
    R: Read + Seek,
{
    let stream_pos = data.stream_position().unwrap_or_default();
    let result = WavReader::new(data.by_ref()).is_ok();
    let _ = data.seek(SeekFrom::Start(stream_pos));
    result
}
