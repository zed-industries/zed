use crate::{Decode, Xz2Decoder};
use compression_core::util::PartialBuffer;
use std::{convert::TryInto, io::Result};

/// Lzma decoding stream
#[derive(Debug)]
pub struct LzmaDecoder {
    inner: Xz2Decoder,
}

impl From<Xz2Decoder> for LzmaDecoder {
    fn from(inner: Xz2Decoder) -> Self {
        Self { inner }
    }
}

impl Default for LzmaDecoder {
    fn default() -> Self {
        Self {
            inner: Xz2Decoder::new(usize::MAX.try_into().unwrap()),
        }
    }
}

impl LzmaDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_memlimit(memlimit: u64) -> Self {
        Self {
            inner: Xz2Decoder::new(memlimit),
        }
    }
}

impl Decode for LzmaDecoder {
    fn reinit(&mut self) -> Result<()> {
        self.inner.reinit()
    }

    fn decode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        self.inner.decode(input, output)
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
