use crate::{Encode, Xz2Encoder, Xz2FileFormat};
use compression_core::{util::PartialBuffer, Level};
use std::io::Result;

/// Xz encoding stream
#[derive(Debug)]
pub struct XzEncoder {
    inner: Xz2Encoder,
}

impl XzEncoder {
    pub fn new(level: Level) -> Self {
        Self {
            inner: Xz2Encoder::new(Xz2FileFormat::Xz, level),
        }
    }

    #[cfg(feature = "xz-parallel")]
    pub fn parallel(threads: std::num::NonZeroU32, level: Level) -> Self {
        Self {
            inner: Xz2Encoder::xz_parallel(level, threads),
        }
    }
}

impl Encode for XzEncoder {
    fn encode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<()> {
        self.inner.encode(input, output)
    }

    fn flush(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        self.inner.flush(output)
    }

    fn finish(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        self.inner.finish(output)
    }
}
