use crate::Decode;
use compression_core::util::PartialBuffer;
use deflate64::InflaterManaged;
use std::io::{Error, ErrorKind, Result};

#[derive(Debug)]
pub struct Deflate64Decoder {
    inflater: Box<InflaterManaged>,
}

impl Default for Deflate64Decoder {
    fn default() -> Self {
        Self {
            inflater: Box::new(InflaterManaged::new()),
        }
    }
}

impl Deflate64Decoder {
    pub fn new() -> Self {
        Self::default()
    }

    fn decode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        let result = self
            .inflater
            .inflate(input.unwritten(), output.unwritten_mut());

        input.advance(result.bytes_consumed);
        output.advance(result.bytes_written);

        if result.data_error {
            Err(Error::new(ErrorKind::InvalidData, "invalid data"))
        } else {
            Ok(self.inflater.finished() && self.inflater.available_output() == 0)
        }
    }
}

impl Decode for Deflate64Decoder {
    fn reinit(&mut self) -> Result<()> {
        self.inflater = Box::new(InflaterManaged::new());
        Ok(())
    }

    fn decode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        self.decode(input, output)
    }

    fn flush(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        self.decode(&mut PartialBuffer::new([]), output)?;

        loop {
            let old_len = output.written().len();
            self.decode(&mut PartialBuffer::new([]), output)?;
            if output.written().len() == old_len {
                break;
            }
        }

        Ok(!output.unwritten().is_empty())
    }

    fn finish(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> Result<bool> {
        self.decode(&mut PartialBuffer::new([]), output)
    }
}
