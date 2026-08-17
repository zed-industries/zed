use compression_core::{util::PartialBuffer, Level};
use liblzma::stream::{Action, Check, Status, Stream};
use std::{
    convert::{TryFrom, TryInto},
    fmt, io,
};

use crate::{
    lzma::params::{LzmaEncoderParams, LzmaOptions},
    Encode, Xz2FileFormat,
};

/// Xz2 encoding stream
pub struct Xz2Encoder {
    stream: Stream,
    params: LzmaEncoderParams,
}

impl fmt::Debug for Xz2Encoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Xz2Encoder").finish_non_exhaustive()
    }
}

impl TryFrom<LzmaEncoderParams> for Xz2Encoder {
    type Error = liblzma::stream::Error;

    fn try_from(params: LzmaEncoderParams) -> Result<Self, Self::Error> {
        let stream = Stream::try_from(&params)?;
        Ok(Self {
            stream,
            params: params.clone(),
        })
    }
}

fn xz2_level(level: Level) -> u32 {
    match level {
        Level::Fastest => 0,
        Level::Best => 9,
        Level::Precise(quality) => quality.try_into().unwrap_or(0).clamp(0, 9),
        _ => 5,
    }
}

impl Xz2Encoder {
    pub fn new(format: Xz2FileFormat, level: Level) -> Self {
        let preset = xz2_level(level);
        let params = match format {
            Xz2FileFormat::Xz => LzmaEncoderParams::Easy {
                preset,
                check: Check::Crc64,
            },
            Xz2FileFormat::Lzma => {
                let options = LzmaOptions::default().preset(preset);
                LzmaEncoderParams::Lzma { options }
            }
        };

        Self::try_from(params).unwrap()
    }

    #[cfg(feature = "xz-parallel")]
    pub fn xz_parallel(level: Level, threads: std::num::NonZeroU32) -> Self {
        use crate::lzma::params::MtStreamBuilder;

        let preset = xz2_level(level);
        let mut builder = MtStreamBuilder::default();
        builder
            .threads(threads)
            .timeout_ms(300)
            .preset(preset)
            .check(Check::Crc64);
        let params = LzmaEncoderParams::MultiThread { builder };
        Self::try_from(params).unwrap()
    }
}

impl Encode for Xz2Encoder {
    fn encode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<()> {
        let previous_in = self.stream.total_in() as usize;
        let previous_out = self.stream.total_out() as usize;

        let status = self
            .stream
            .process(input.unwritten(), output.unwritten_mut(), Action::Run)?;

        input.advance(self.stream.total_in() as usize - previous_in);
        output.advance(self.stream.total_out() as usize - previous_out);

        match status {
            Status::Ok | Status::StreamEnd => Ok(()),
            Status::GetCheck => Err(io::Error::other("Unexpected lzma integrity check")),
            Status::MemNeeded => Err(io::ErrorKind::OutOfMemory.into()),
        }
    }

    fn flush(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        let previous_out = self.stream.total_out() as usize;

        let action = match &self.params {
            // Multi-threaded streams don't support SyncFlush, use FullFlush instead
            #[cfg(feature = "xz-parallel")]
            LzmaEncoderParams::MultiThread { builder: _ } => Action::FullFlush,
            _ => Action::SyncFlush,
        };

        let status = self.stream.process(&[], output.unwritten_mut(), action)?;

        output.advance(self.stream.total_out() as usize - previous_out);

        match status {
            Status::Ok => Ok(false),
            Status::StreamEnd => Ok(true),
            Status::GetCheck => Err(io::Error::other("Unexpected lzma integrity check")),
            Status::MemNeeded => Err(io::ErrorKind::OutOfMemory.into()),
        }
    }

    fn finish(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        let previous_out = self.stream.total_out() as usize;

        let status = self
            .stream
            .process(&[], output.unwritten_mut(), Action::Finish)?;

        output.advance(self.stream.total_out() as usize - previous_out);

        match status {
            Status::Ok => Ok(false),
            Status::StreamEnd => Ok(true),
            Status::GetCheck => Err(io::Error::other("Unexpected lzma integrity check")),
            Status::MemNeeded => Err(io::ErrorKind::OutOfMemory.into()),
        }
    }
}
