use crate::zstd::params::CParameter;
use crate::Encode;
use compression_core::unshared::Unshared;
use compression_core::util::PartialBuffer;
use libzstd::stream::raw::{Encoder, Operation};
use std::io;
use std::io::Result;

#[derive(Debug)]
pub struct ZstdEncoder {
    encoder: Unshared<Encoder<'static>>,
}

impl ZstdEncoder {
    pub fn new(level: i32) -> Self {
        Self {
            encoder: Unshared::new(Encoder::new(level).unwrap()),
        }
    }

    pub fn new_with_params(level: i32, params: &[CParameter]) -> Self {
        let mut encoder = Encoder::new(level).unwrap();
        for param in params {
            encoder.set_parameter(param.as_zstd()).unwrap();
        }
        Self {
            encoder: Unshared::new(encoder),
        }
    }

    pub fn new_with_dict(level: i32, dictionary: &[u8]) -> io::Result<Self> {
        let encoder = Encoder::with_dictionary(level, dictionary)?;
        Ok(Self {
            encoder: Unshared::new(encoder),
        })
    }
}

impl Encode for ZstdEncoder {
    fn encode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<()> {
        let status = self
            .encoder
            .get_mut()
            .run_on_buffers(input.unwritten(), output.unwritten_mut())?;
        input.advance(status.bytes_read);
        output.advance(status.bytes_written);
        Ok(())
    }

    fn flush(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        let mut out_buf = zstd_safe::OutBuffer::around(output.unwritten_mut());
        let bytes_left = self.encoder.get_mut().flush(&mut out_buf)?;
        let len = out_buf.as_slice().len();
        output.advance(len);
        Ok(bytes_left == 0)
    }

    fn finish(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        let mut out_buf = zstd_safe::OutBuffer::around(output.unwritten_mut());
        let bytes_left = self.encoder.get_mut().finish(&mut out_buf, true)?;
        let len = out_buf.as_slice().len();
        output.advance(len);
        Ok(bytes_left == 0)
    }
}
