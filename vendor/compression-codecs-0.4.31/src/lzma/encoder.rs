use crate::{Encode, Xz2Encoder, Xz2FileFormat};
use compression_core::{util::PartialBuffer, Level};
use std::io::Result;

/// Lzma encoding stream
#[derive(Debug)]
pub struct LzmaEncoder {
    inner: Xz2Encoder,
}

impl LzmaEncoder {
    pub fn new(level: Level) -> Self {
        Self {
            inner: Xz2Encoder::new(Xz2FileFormat::Lzma, level),
        }
    }
}

impl From<Xz2Encoder> for LzmaEncoder {
    fn from(inner: Xz2Encoder) -> Self {
        Self { inner }
    }
}

impl Encode for LzmaEncoder {
    fn encode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<()> {
        self.inner.encode(input, output)
    }

    fn flush(
        &mut self,
        _output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        // Flush on LZMA 1 is not supported
        Ok(true)
    }

    fn finish(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        self.inner.finish(output)
    }
}
