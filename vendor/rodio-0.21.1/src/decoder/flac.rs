use std::io::{Read, Seek, SeekFrom};
use std::mem;
use std::time::Duration;

use crate::source::SeekError;
use crate::Source;

use crate::common::{ChannelCount, Sample, SampleRate};

use claxon::FlacReader;
use dasp_sample::Sample as _;
use dasp_sample::I24;

/// Decoder for the FLAC format.
pub struct FlacDecoder<R>
where
    R: Read + Seek,
{
    reader: FlacReader<R>,
    current_block: Vec<i32>,
    current_block_channel_len: usize,
    current_block_off: usize,
    bits_per_sample: u32,
    sample_rate: SampleRate,
    channels: ChannelCount,
    total_duration: Option<Duration>,
}

impl<R> FlacDecoder<R>
where
    R: Read + Seek,
{
    /// Attempts to decode the data as FLAC.
    pub fn new(mut data: R) -> Result<FlacDecoder<R>, R> {
        if !is_flac(data.by_ref()) {
            return Err(data);
        }

        let reader = FlacReader::new(data).expect("should still be flac");

        let spec = reader.streaminfo();
        let sample_rate = spec.sample_rate;

        // `samples` in FLAC means "inter-channel samples" aka frames
        // so we do not divide by `self.channels` here.
        let total_duration = spec.samples.map(|s| {
            // Calculate duration as (samples * 1_000_000) / sample_rate
            // but do the division first to avoid overflow
            let sample_rate = sample_rate as u64;
            let secs = s / sample_rate;
            let nanos = ((s % sample_rate) * 1_000_000_000) / sample_rate;
            Duration::new(secs, nanos as u32)
        });

        Ok(FlacDecoder {
            reader,
            current_block: Vec::with_capacity(
                spec.max_block_size as usize * spec.channels as usize,
            ),
            current_block_channel_len: 1,
            current_block_off: 0,
            bits_per_sample: spec.bits_per_sample,
            sample_rate: SampleRate::new(sample_rate)
                .expect("flac data should never have a zero sample rate"),
            channels: ChannelCount::new(
                spec.channels
                    .try_into()
                    .expect("rodio supports only up to u16::MAX (65_535) channels"),
            )
            .expect("flac should never have zero channels"),
            total_duration,
        })
    }

    #[inline]
    pub fn into_inner(self) -> R {
        self.reader.into_inner()
    }
}

impl<R> Source for FlacDecoder<R>
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
        self.total_duration
    }

    #[inline]
    fn try_seek(&mut self, _: Duration) -> Result<(), SeekError> {
        Err(SeekError::NotSupported {
            underlying_source: std::any::type_name::<Self>(),
        })
    }
}

impl<R> Iterator for FlacDecoder<R>
where
    R: Read + Seek,
{
    type Item = Sample;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_block_off < self.current_block.len() {
                // Read from current block.
                let real_offset = (self.current_block_off % self.channels.get() as usize)
                    * self.current_block_channel_len
                    + self.current_block_off / self.channels.get() as usize;
                let raw_val = self.current_block[real_offset];
                self.current_block_off += 1;
                let bits = self.bits_per_sample;
                let real_val = match bits {
                    8 => (raw_val as i8).to_sample(),
                    16 => (raw_val as i16).to_sample(),
                    24 => I24::new(raw_val)
                        .unwrap_or(dasp_sample::Sample::EQUILIBRIUM)
                        .to_sample(),
                    32 => raw_val.to_sample(),
                    _ => {
                        // FLAC also supports 12 and 20 bits per sample. We use bit
                        // shifts to convert them to 32 bits, because:
                        // - I12 does not exist as a type
                        // - I20 exists but does not have `ToSample` implemented
                        (raw_val << (32 - bits)).to_sample()
                    }
                };
                return Some(real_val);
            }

            // Load the next block.
            self.current_block_off = 0;
            let buffer = mem::take(&mut self.current_block);
            match self.reader.blocks().read_next_or_eof(buffer) {
                Ok(Some(block)) => {
                    self.current_block_channel_len = (block.len() / block.channels()) as usize;
                    self.current_block = block.into_buffer();
                }
                _ => return None,
            }
        }
    }
}

/// Returns true if the stream contains FLAC data, then tries to rewind it to where it was.
fn is_flac<R>(mut data: R) -> bool
where
    R: Read + Seek,
{
    let stream_pos = data.stream_position().unwrap_or_default();
    let result = FlacReader::new(data.by_ref()).is_ok();
    let _ = data.seek(SeekFrom::Start(stream_pos));
    result
}
