use crate::{lzma::params::LzmaDecoderParams, Decode};
use compression_core::util::PartialBuffer;
use liblzma::stream::{Action, Status, Stream};
use std::{convert::TryFrom, fmt, io};

/// Xz2 decoding stream
pub struct Xz2Decoder {
    stream: Stream,
    params: LzmaDecoderParams,
}

impl fmt::Debug for Xz2Decoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Xz2Decoder").finish_non_exhaustive()
    }
}

impl TryFrom<LzmaDecoderParams> for Xz2Decoder {
    type Error = liblzma::stream::Error;

    fn try_from(params: LzmaDecoderParams) -> Result<Self, Self::Error> {
        let stream = Stream::try_from(&params)?;
        Ok(Self { stream, params })
    }
}

impl Xz2Decoder {
    pub fn new(mem_limit: u64) -> Self {
        let params = LzmaDecoderParams::Auto {
            mem_limit,
            flags: 0,
        };
        Self::try_from(params).unwrap()
    }

    #[cfg(feature = "xz-parallel")]
    pub fn parallel(threads: std::num::NonZeroU32, mem_limit: u64) -> Self {
        use crate::lzma::params::MtStreamBuilder;

        let mut builder = MtStreamBuilder::default();

        builder
            .threads(threads)
            .timeout_ms(300)
            .mem_limit_stop(mem_limit);

        let params = LzmaDecoderParams::MultiThread { builder };

        Self::try_from(params).unwrap()
    }
}

impl Decode for Xz2Decoder {
    fn reinit(&mut self) -> io::Result<()> {
        *self = Self::try_from(self.params.clone())?;
        Ok(())
    }

    fn decode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        let previous_in = self.stream.total_in() as usize;
        let previous_out = self.stream.total_out() as usize;

        let status = self
            .stream
            .process(input.unwritten(), output.unwritten_mut(), Action::Run)?;

        input.advance(self.stream.total_in() as usize - previous_in);
        output.advance(self.stream.total_out() as usize - previous_out);

        match status {
            Status::Ok => Ok(false),
            Status::StreamEnd => Ok(true),
            Status::GetCheck => Err(io::Error::other("Unexpected lzma integrity check")),
            Status::MemNeeded => Err(io::ErrorKind::OutOfMemory.into()),
        }
    }

    fn flush(
        &mut self,
        _output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        // While decoding flush is a noop
        Ok(true)
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

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::{
        lzma::params::{LzmaDecoderParams, LzmaFilter, LzmaFilters, LzmaOptions},
        Xz2Decoder,
    };

    #[test]
    fn test_lzma_decoder_from_params() {
        let filters = LzmaFilters::default().add_filter(LzmaFilter::Lzma2(LzmaOptions::default()));
        let params = LzmaDecoderParams::Raw { filters };
        Xz2Decoder::try_from(params).unwrap();
    }
}
