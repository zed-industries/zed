use std::io::{Read, Seek, SeekFrom};
use std::num::NonZero;
use std::time::Duration;

use crate::common::{ChannelCount, Sample, SampleRate};
use crate::source::SeekError;
use crate::Source;

use dasp_sample::Sample as _;

use minimp3::Decoder;
use minimp3::Frame;
use minimp3_fixed as minimp3;

pub struct Mp3Decoder<R>
where
    R: Read + Seek,
{
    // decoder: SeekDecoder<R>,
    decoder: Decoder<R>,
    // what minimp3 calls frames rodio calls spans
    current_span: Frame,
    current_span_offset: usize,
}

impl<R> Mp3Decoder<R>
where
    R: Read + Seek,
{
    pub fn new(mut data: R) -> Result<Self, R> {
        if !is_mp3(data.by_ref()) {
            return Err(data);
        }
        // let mut decoder = SeekDecoder::new(data)
        let mut decoder = Decoder::new(data);
        // parameters are correct and minimp3 is used correctly
        // thus if we crash here one of these invariants is broken:
        // .expect("should be able to allocate memory, perform IO");
        // let current_span = decoder.decode_frame()
        let current_span = decoder.next_frame().expect("should still be mp3");

        Ok(Mp3Decoder {
            decoder,
            current_span,
            current_span_offset: 0,
        })
    }

    #[inline]
    pub fn into_inner(self) -> R {
        self.decoder.into_inner()
    }
}

impl<R> Source for Mp3Decoder<R>
where
    R: Read + Seek,
{
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        Some(self.current_span.data.len())
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        NonZero::new(self.current_span.channels as _).expect("mp3's have at least one channel")
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        NonZero::new(self.current_span.sample_rate as _).expect("mp3's have a non zero sample rate")
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }

    fn try_seek(&mut self, _pos: Duration) -> Result<(), SeekError> {
        // TODO waiting for PR in minimp3_fixed or minimp3

        // let pos = (pos.as_secs_f32() * self.sample_rate().get() as f32) as u64;
        // // do not trigger a sample_rate, channels and frame/span len update
        // // as the seek only takes effect after the current frame/span is done
        // self.decoder.seek_samples(pos)?;
        // Ok(())

        Err(SeekError::NotSupported {
            underlying_source: std::any::type_name::<Self>(),
        })
    }
}

impl<R> Iterator for Mp3Decoder<R>
where
    R: Read + Seek,
{
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        let current_span_len = self.current_span_len()?;
        if self.current_span_offset == current_span_len {
            if let Ok(span) = self.decoder.next_frame() {
                // if let Ok(span) = self.decoder.decode_frame() {
                self.current_span = span;
                self.current_span_offset = 0;
            } else {
                return None;
            }
        }

        let v = self.current_span.data[self.current_span_offset];
        self.current_span_offset += 1;

        Some(v.to_sample())
    }
}

/// Returns true if the stream contains mp3 data, then resets it to where it was.
fn is_mp3<R>(mut data: R) -> bool
where
    R: Read + Seek,
{
    let stream_pos = data.stream_position().unwrap_or_default();
    let mut decoder = Decoder::new(data.by_ref());
    let result = decoder.next_frame().is_ok();
    let _ = data.seek(SeekFrom::Start(stream_pos));
    result
}
