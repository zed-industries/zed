use crate::Decode;
use brotli::{enc::StandardAlloc, BrotliDecompressStream, BrotliResult, BrotliState};
use compression_core::util::PartialBuffer;
use std::{fmt, io};

pub struct BrotliDecoder {
    // `BrotliState` is very large (over 2kb) which is why we're boxing it.
    state: Box<BrotliState<StandardAlloc, StandardAlloc, StandardAlloc>>,
}

impl Default for BrotliDecoder {
    fn default() -> Self {
        Self {
            state: Box::new(BrotliState::new(
                StandardAlloc::default(),
                StandardAlloc::default(),
                StandardAlloc::default(),
            )),
        }
    }
}

impl BrotliDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    fn decode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<BrotliResult> {
        let in_buf = input.unwritten();
        let out_buf = output.unwritten_mut();

        let mut input_len = 0;
        let mut output_len = 0;

        let status = match BrotliDecompressStream(
            &mut in_buf.len(),
            &mut input_len,
            in_buf,
            &mut out_buf.len(),
            &mut output_len,
            out_buf,
            &mut 0,
            &mut self.state,
        ) {
            BrotliResult::ResultFailure => return Err(io::Error::other("brotli error")),
            status => status,
        };

        input.advance(input_len);
        output.advance(output_len);

        Ok(status)
    }
}

impl Decode for BrotliDecoder {
    fn reinit(&mut self) -> io::Result<()> {
        self.state = Box::new(BrotliState::new(
            StandardAlloc::default(),
            StandardAlloc::default(),
            StandardAlloc::default(),
        ));
        Ok(())
    }

    fn decode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        match self.decode(input, output)? {
            BrotliResult::ResultSuccess => Ok(true),
            BrotliResult::NeedsMoreOutput | BrotliResult::NeedsMoreInput => Ok(false),
            BrotliResult::ResultFailure => unreachable!(),
        }
    }

    fn flush(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        match self.decode(&mut PartialBuffer::new(&[][..]), output)? {
            BrotliResult::ResultSuccess | BrotliResult::NeedsMoreInput => Ok(true),
            BrotliResult::NeedsMoreOutput => Ok(false),
            BrotliResult::ResultFailure => unreachable!(),
        }
    }

    fn finish(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        match self.decode(&mut PartialBuffer::new(&[][..]), output)? {
            BrotliResult::ResultSuccess => Ok(true),
            BrotliResult::NeedsMoreOutput => Ok(false),
            BrotliResult::NeedsMoreInput => Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "reached unexpected EOF",
            )),
            BrotliResult::ResultFailure => unreachable!(),
        }
    }
}

impl fmt::Debug for BrotliDecoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BrotliDecoder")
            .field("decompress", &"<no debug>")
            .finish()
    }
}
