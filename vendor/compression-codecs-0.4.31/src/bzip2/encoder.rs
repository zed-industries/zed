use crate::{bzip2::params::Bzip2EncoderParams, Encode};
use bzip2::{Action, Compress, Compression, Status};
use compression_core::util::PartialBuffer;
use std::{fmt, io};

pub struct BzEncoder {
    compress: Compress,
}

impl fmt::Debug for BzEncoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BzEncoder {{total_in: {}, total_out: {}}}",
            self.compress.total_in(),
            self.compress.total_out()
        )
    }
}

impl BzEncoder {
    /// Creates a new stream prepared for compression.
    ///
    /// The `work_factor` parameter controls how the compression phase behaves
    /// when presented with worst case, highly repetitive, input data. If
    /// compression runs into difficulties caused by repetitive data, the
    /// library switches from the standard sorting algorithm to a fallback
    /// algorithm. The fallback is slower than the standard algorithm by perhaps
    /// a factor of three, but always behaves reasonably, no matter how bad the
    /// input.
    ///
    /// Lower values of `work_factor` reduce the amount of effort the standard
    /// algorithm will expend before resorting to the fallback. You should set
    /// this parameter carefully; too low, and many inputs will be handled by
    /// the fallback algorithm and so compress rather slowly, too high, and your
    /// average-to-worst case compression times can become very large. The
    /// default value of 30 gives reasonable behaviour over a wide range of
    /// circumstances.
    ///
    /// Allowable values range from 0 to 250 inclusive. 0 is a special case,
    /// equivalent to using the default value of 30.
    pub fn new(params: Bzip2EncoderParams, work_factor: u32) -> Self {
        let params = Compression::from(params);
        Self {
            compress: Compress::new(params, work_factor),
        }
    }

    fn encode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
        action: Action,
    ) -> io::Result<Status> {
        let prior_in = self.compress.total_in();
        let prior_out = self.compress.total_out();

        let status = self
            .compress
            .compress(input.unwritten(), output.unwritten_mut(), action)
            .map_err(io::Error::other)?;

        input.advance((self.compress.total_in() - prior_in) as usize);
        output.advance((self.compress.total_out() - prior_out) as usize);

        Ok(status)
    }
}

impl Encode for BzEncoder {
    fn encode(
        &mut self,
        input: &mut PartialBuffer<impl AsRef<[u8]>>,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<()> {
        match self.encode(input, output, Action::Run)? {
            // Decompression went fine, nothing much to report.
            Status::Ok => Ok(()),

            // The Flush action on a compression went ok.
            Status::FlushOk => unreachable!(),

            // The Run action on compression went ok.
            Status::RunOk => Ok(()),

            // The Finish action on compression went ok.
            Status::FinishOk => unreachable!(),

            // The stream's end has been met, meaning that no more data can be input.
            Status::StreamEnd => unreachable!(),

            // There was insufficient memory in the input or output buffer to complete
            // the request, but otherwise everything went normally.
            Status::MemNeeded => Err(io::ErrorKind::OutOfMemory.into()),
        }
    }

    fn flush(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        match self.encode(&mut PartialBuffer::new(&[][..]), output, Action::Flush)? {
            // Decompression went fine, nothing much to report.
            Status::Ok => unreachable!(),

            // The Flush action on a compression went ok.
            Status::FlushOk => Ok(false),

            // The Run action on compression went ok.
            Status::RunOk => Ok(true),

            // The Finish action on compression went ok.
            Status::FinishOk => unreachable!(),

            // The stream's end has been met, meaning that no more data can be input.
            Status::StreamEnd => unreachable!(),

            // There was insufficient memory in the input or output buffer to complete
            // the request, but otherwise everything went normally.
            Status::MemNeeded => Err(io::ErrorKind::OutOfMemory.into()),
        }
    }

    fn finish(
        &mut self,
        output: &mut PartialBuffer<impl AsRef<[u8]> + AsMut<[u8]>>,
    ) -> io::Result<bool> {
        match self.encode(&mut PartialBuffer::new(&[][..]), output, Action::Finish)? {
            // Decompression went fine, nothing much to report.
            Status::Ok => Ok(false),

            // The Flush action on a compression went ok.
            Status::FlushOk => unreachable!(),

            // The Run action on compression went ok.
            Status::RunOk => unreachable!(),

            // The Finish action on compression went ok.
            Status::FinishOk => Ok(false),

            // The stream's end has been met, meaning that no more data can be input.
            Status::StreamEnd => Ok(true),

            // There was insufficient memory in the input or output buffer to complete
            // the request, but otherwise everything went normally.
            Status::MemNeeded => Err(io::ErrorKind::OutOfMemory.into()),
        }
    }
}
