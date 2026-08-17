use crate::Decode;
use bzip2::{Decompress, Status};
use compression_core::util::PartialBuffer;
use std::{fmt, io};

pub struct BzDecoder {
    decompress: Decompress,
}

impl fmt::Debug for BzDecoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BzDecoder {{total_in: {}, total_out: {}}}",
            self.decompress.total_in(),
            self.decompress.total_out()
        )
    }
}

impl Default for BzDecoder {
    fn default() -> Self {
        Self {
            decompress: Decompress::new(false),
        }
    }
}

impl BzDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    fn decode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<Status> {
        let prior_in = self.decompress.total_in();
        let prior_out = self.decompress.total_out();

        let status = self
            .decompress
            .decompress(input.unwritten(), output.unwritten_mut())
            .map_err(io::Error::other)?;

        input.advance((self.decompress.total_in() - prior_in) as usize);
        output.advance((self.decompress.total_out() - prior_out) as usize);

        Ok(status)
    }
}

impl Decode for BzDecoder {
    fn reinit(&mut self) -> io::Result<()> {
        self.decompress = Decompress::new(false);
        Ok(())
    }

    fn decode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        match self.decode(input, output)? {
            // Decompression went fine, nothing much to report.
            Status::Ok => Ok(false),

            // The Flush action on a compression went ok.
            Status::FlushOk => unreachable!(),

            // THe Run action on compression went ok.
            Status::RunOk => unreachable!(),

            // The Finish action on compression went ok.
            Status::FinishOk => unreachable!(),

            // The stream's end has been met, meaning that no more data can be input.
            Status::StreamEnd => Ok(true),

            // There was insufficient memory in the input or output buffer to complete
            // the request, but otherwise everything went normally.
            Status::MemNeeded => Err(io::ErrorKind::OutOfMemory.into()),
        }
    }

    fn flush(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        self.decode(&mut PartialBuffer::new(&[][..]), output)?;

        loop {
            let old_len = output.written().len();
            self.decode(&mut PartialBuffer::new(&[][..]), output)?;
            if output.written().len() == old_len {
                break;
            }
        }

        Ok(!output.unwritten().is_empty())
    }

    fn finish(
        &mut self,
        _output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        Ok(true)
    }
}
