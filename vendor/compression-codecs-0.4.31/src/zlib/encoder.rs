use crate::{flate::params::FlateEncoderParams, Encode, FlateEncoder};
use compression_core::util::PartialBuffer;
use std::io::Result;

#[derive(Debug)]
pub struct ZlibEncoder {
    inner: FlateEncoder,
}

impl ZlibEncoder {
    pub fn new(level: FlateEncoderParams) -> Self {
        Self {
            inner: FlateEncoder::new(level, true),
        }
    }

    pub fn get_ref(&self) -> &FlateEncoder {
        &self.inner
    }
}

impl Encode for ZlibEncoder {
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
