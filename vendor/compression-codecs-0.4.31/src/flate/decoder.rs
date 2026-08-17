use crate::Decode;
use compression_core::util::PartialBuffer;
use flate2::{Decompress, FlushDecompress, Status};
use std::io;

#[derive(Debug)]
pub struct FlateDecoder {
    zlib_header: bool,
    decompress: Decompress,
}

impl FlateDecoder {
    pub(crate) fn new(zlib_header: bool) -> Self {
        Self {
            zlib_header,
            decompress: Decompress::new(zlib_header),
        }
    }

    fn decode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
        flush: FlushDecompress,
    ) -> io::Result<Status> {
        let prior_in = self.decompress.total_in();
        let prior_out = self.decompress.total_out();

        let status =
            self.decompress
                .decompress(input.unwritten(), output.unwritten_mut(), flush)?;

        input.advance((self.decompress.total_in() - prior_in) as usize);
        output.advance((self.decompress.total_out() - prior_out) as usize);

        Ok(status)
    }
}

impl Decode for FlateDecoder {
    fn reinit(&mut self) -> io::Result<()> {
        self.decompress.reset(self.zlib_header);
        Ok(())
    }

    fn decode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        match self.decode(input, output, FlushDecompress::None)? {
            Status::Ok => Ok(false),
            Status::StreamEnd => Ok(true),
            Status::BufError => Err(io::Error::other("unexpected BufError")),
        }
    }

    fn flush(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        self.decode(
            &mut PartialBuffer::new(&[][..]),
            output,
            FlushDecompress::Sync,
        )?;

        loop {
            let old_len = output.written().len();
            self.decode(
                &mut PartialBuffer::new(&[][..]),
                output,
                FlushDecompress::None,
            )?;
            if output.written().len() == old_len {
                break;
            }
        }

        Ok(!output.unwritten().is_empty())
    }

    fn finish(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        match self.decode(
            &mut PartialBuffer::new(&[][..]),
            output,
            FlushDecompress::Finish,
        )? {
            Status::Ok => Ok(false),
            Status::StreamEnd => Ok(true),
            Status::BufError => Err(io::Error::other("unexpected BufError")),
        }
    }
}
