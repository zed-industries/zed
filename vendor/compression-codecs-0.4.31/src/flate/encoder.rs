use crate::{flate::params::FlateEncoderParams, Encode};
use compression_core::util::PartialBuffer;
use flate2::{Compress, FlushCompress, Status};
use std::io;

#[derive(Debug)]
pub struct FlateEncoder {
    compress: Compress,
    flushed: bool,
}

impl FlateEncoder {
    pub fn new(level: FlateEncoderParams, zlib_header: bool) -> Self {
        let level = flate2::Compression::from(level);
        Self {
            compress: Compress::new(level, zlib_header),
            flushed: true,
        }
    }

    pub fn get_ref(&self) -> &Compress {
        &self.compress
    }

    fn encode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
        flush: FlushCompress,
    ) -> io::Result<Status> {
        let prior_in = self.compress.total_in();
        let prior_out = self.compress.total_out();

        let status = self
            .compress
            .compress(input.unwritten(), output.unwritten_mut(), flush)?;

        input.advance((self.compress.total_in() - prior_in) as usize);
        output.advance((self.compress.total_out() - prior_out) as usize);

        Ok(status)
    }
}

impl Encode for FlateEncoder {
    fn encode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<()> {
        self.flushed = false;
        match self.encode(input, output, FlushCompress::None)? {
            Status::Ok => Ok(()),
            Status::StreamEnd => unreachable!(),
            Status::BufError => Err(io::Error::other("unexpected BufError")),
        }
    }

    fn flush(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        // We need to keep track of whether we've already flushed otherwise we'll just keep writing
        // out sync blocks continuously and probably never complete flushing.
        if self.flushed {
            return Ok(true);
        }

        self.encode(
            &mut PartialBuffer::new(&[][..]),
            output,
            FlushCompress::Sync,
        )?;

        loop {
            let old_len = output.written().len();
            self.encode(
                &mut PartialBuffer::new(&[][..]),
                output,
                FlushCompress::None,
            )?;
            if output.written().len() == old_len {
                break;
            }
        }

        let internal_flushed = !output.unwritten().is_empty();
        self.flushed = internal_flushed;
        Ok(internal_flushed)
    }

    fn finish(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        self.flushed = false;
        match self.encode(
            &mut PartialBuffer::new(&[][..]),
            output,
            FlushCompress::Finish,
        )? {
            Status::Ok => Ok(false),
            Status::StreamEnd => Ok(true),
            Status::BufError => Err(io::Error::other("unexpected BufError")),
        }
    }
}
