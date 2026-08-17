use crate::{Decode, FlateDecoder};
use compression_core::util::PartialBuffer;
use std::io::Result;

#[derive(Debug)]
pub struct DeflateDecoder {
    inner: FlateDecoder,
}

impl Default for DeflateDecoder {
    fn default() -> Self {
        Self {
            inner: FlateDecoder::new(false),
        }
    }
}

impl DeflateDecoder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Decode for DeflateDecoder {
    fn reinit(&mut self) -> Result<()> {
        self.inner.reinit()?;
        Ok(())
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
