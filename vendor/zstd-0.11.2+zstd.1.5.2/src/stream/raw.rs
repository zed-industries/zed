//! Raw in-memory stream compression/decompression.
//!
//! This module defines a `Decoder` and an `Encoder` to decode/encode streams
//! of data using buffers.
//!
//! They are mostly thin wrappers around `zstd_safe::{DCtx, CCtx}`.
use std::io;

pub use zstd_safe::{CParameter, DParameter, InBuffer, OutBuffer, WriteBuf};

use crate::dict::{DecoderDictionary, EncoderDictionary};
use crate::map_error_code;

/// Represents an abstract compression/decompression operation.
///
/// This trait covers both `Encoder` and `Decoder`.
pub trait Operation {
    /// Performs a single step of this operation.
    ///
    /// Should return a hint for the next input size.
    ///
    /// If the result is `Ok(0)`, it may indicate that a frame was just
    /// finished.
    fn run<C: WriteBuf + ?Sized>(
        &mut self,
        input: &mut InBuffer<'_>,
        output: &mut OutBuffer<'_, C>,
    ) -> io::Result<usize>;

    /// Performs a single step of this operation.
    ///
    /// This is a comvenience wrapper around `Operation::run` if you don't
    /// want to deal with `InBuffer`/`OutBuffer`.
    fn run_on_buffers(
        &mut self,
        input: &[u8],
        output: &mut [u8],
    ) -> io::Result<Status> {
        let mut input = InBuffer::around(input);
        let mut output = OutBuffer::around(output);

        let remaining = self.run(&mut input, &mut output)?;

        Ok(Status {
            remaining,
            bytes_read: input.pos(),
            bytes_written: output.pos(),
        })
    }

    /// Flushes any internal buffer, if any.
    ///
    /// Returns the number of bytes still in the buffer.
    /// To flush entirely, keep calling until it returns `Ok(0)`.
    fn flush<C: WriteBuf + ?Sized>(
        &mut self,
        output: &mut OutBuffer<'_, C>,
    ) -> io::Result<usize> {
        let _ = output;
        Ok(0)
    }

    /// Prepares the operation for a new frame.
    ///
    /// This is hopefully cheaper than creating a new operation.
    fn reinit(&mut self) -> io::Result<()> {
        Ok(())
    }

    /// Finishes the operation, writing any footer if necessary.
    ///
    /// Returns the number of bytes still to write.
    ///
    /// Keep calling this method until it returns `Ok(0)`,
    /// and then don't ever call this method.
    fn finish<C: WriteBuf + ?Sized>(
        &mut self,
        output: &mut OutBuffer<'_, C>,
        finished_frame: bool,
    ) -> io::Result<usize> {
        let _ = output;
        let _ = finished_frame;
        Ok(0)
    }
}

/// Dummy operation that just copies its input to the output.
pub struct NoOp;

impl Operation for NoOp {
    fn run<C: WriteBuf + ?Sized>(
        &mut self,
        input: &mut InBuffer<'_>,
        output: &mut OutBuffer<'_, C>,
    ) -> io::Result<usize> {
        // Skip the prelude
        let src = &input.src[input.pos..];
        // Safe because `output.pos() <= output.dst.capacity()`.
        let dst = unsafe { output.dst.as_mut_ptr().add(output.pos()) };

        // Ignore anything past the end
        let len = usize::min(src.len(), output.dst.capacity());
        let src = &src[..len];

        // Safe because:
        // * `len` is less than either of the two lengths
        // * `src` and `dst` do not overlap because we have `&mut` to each.
        unsafe { std::ptr::copy_nonoverlapping(src.as_ptr(), dst, len) };
        input.set_pos(input.pos() + len);
        unsafe { output.set_pos(output.pos() + len) };

        Ok(0)
    }
}

/// Describes the result of an operation.
pub struct Status {
    /// Number of bytes expected for next input.
    ///
    /// This is just a hint.
    pub remaining: usize,

    /// Number of bytes read from the input.
    pub bytes_read: usize,

    /// Number of bytes written to the output.
    pub bytes_written: usize,
}

/// An in-memory decoder for streams of data.
pub struct Decoder<'a> {
    context: zstd_safe::DCtx<'a>,
}

impl Decoder<'static> {
    /// Creates a new decoder.
    pub fn new() -> io::Result<Self> {
        Self::with_dictionary(&[])
    }

    /// Creates a new decoder initialized with the given dictionary.
    pub fn with_dictionary(dictionary: &[u8]) -> io::Result<Self> {
        let mut context = zstd_safe::DCtx::create();
        context.init();
        context
            .load_dictionary(dictionary)
            .map_err(map_error_code)?;
        Ok(Decoder { context })
    }
}

impl<'a> Decoder<'a> {
    /// Creates a new decoder, using an existing `DecoderDictionary`.
    pub fn with_prepared_dictionary<'b>(
        dictionary: &DecoderDictionary<'b>,
    ) -> io::Result<Self>
    where
        'b: 'a,
    {
        let mut context = zstd_safe::DCtx::create();
        context
            .ref_ddict(dictionary.as_ddict())
            .map_err(map_error_code)?;
        Ok(Decoder { context })
    }

    /// Sets a decompression parameter for this decoder.
    pub fn set_parameter(&mut self, parameter: DParameter) -> io::Result<()> {
        self.context
            .set_parameter(parameter)
            .map_err(map_error_code)?;
        Ok(())
    }
}

impl Operation for Decoder<'_> {
    fn run<C: WriteBuf + ?Sized>(
        &mut self,
        input: &mut InBuffer<'_>,
        output: &mut OutBuffer<'_, C>,
    ) -> io::Result<usize> {
        self.context
            .decompress_stream(output, input)
            .map_err(map_error_code)
    }

    fn reinit(&mut self) -> io::Result<()> {
        self.context.reset().map_err(map_error_code)?;
        Ok(())
    }

    fn finish<C: WriteBuf + ?Sized>(
        &mut self,
        _output: &mut OutBuffer<'_, C>,
        finished_frame: bool,
    ) -> io::Result<usize> {
        if finished_frame {
            Ok(0)
        } else {
            Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "incomplete frame",
            ))
        }
    }
}

/// An in-memory encoder for streams of data.
pub struct Encoder<'a> {
    context: zstd_safe::CCtx<'a>,
}

impl Encoder<'static> {
    /// Creates a new encoder.
    pub fn new(level: i32) -> io::Result<Self> {
        Self::with_dictionary(level, &[])
    }

    /// Creates a new encoder initialized with the given dictionary.
    pub fn with_dictionary(level: i32, dictionary: &[u8]) -> io::Result<Self> {
        let mut context = zstd_safe::CCtx::create();

        context
            .set_parameter(CParameter::CompressionLevel(level))
            .map_err(map_error_code)?;

        context
            .load_dictionary(dictionary)
            .map_err(map_error_code)?;

        Ok(Encoder { context })
    }
}

impl<'a> Encoder<'a> {
    /// Creates a new encoder using an existing `EncoderDictionary`.
    pub fn with_prepared_dictionary<'b>(
        dictionary: &EncoderDictionary<'b>,
    ) -> io::Result<Self>
    where
        'b: 'a,
    {
        let mut context = zstd_safe::CCtx::create();
        context
            .ref_cdict(dictionary.as_cdict())
            .map_err(map_error_code)?;
        Ok(Encoder { context })
    }

    /// Sets a compression parameter for this encoder.
    pub fn set_parameter(&mut self, parameter: CParameter) -> io::Result<()> {
        self.context
            .set_parameter(parameter)
            .map_err(map_error_code)?;
        Ok(())
    }

    /// Sets the size of the input expected by zstd.
    ///
    /// May affect compression ratio.
    ///
    /// It is an error to give an incorrect size (an error _will_ be returned when closing the
    /// stream).
    pub fn set_pledged_src_size(
        &mut self,
        pledged_src_size: u64,
    ) -> io::Result<()> {
        self.context
            .set_pledged_src_size(pledged_src_size)
            .map_err(map_error_code)?;
        Ok(())
    }
}

impl<'a> Operation for Encoder<'a> {
    fn run<C: WriteBuf + ?Sized>(
        &mut self,
        input: &mut InBuffer<'_>,
        output: &mut OutBuffer<'_, C>,
    ) -> io::Result<usize> {
        self.context
            .compress_stream(output, input)
            .map_err(map_error_code)
    }

    fn flush<C: WriteBuf + ?Sized>(
        &mut self,
        output: &mut OutBuffer<'_, C>,
    ) -> io::Result<usize> {
        self.context.flush_stream(output).map_err(map_error_code)
    }

    fn finish<C: WriteBuf + ?Sized>(
        &mut self,
        output: &mut OutBuffer<'_, C>,
        _finished_frame: bool,
    ) -> io::Result<usize> {
        self.context.end_stream(output).map_err(map_error_code)
    }

    fn reinit(&mut self) -> io::Result<()> {
        self.context
            .reset(zstd_safe::ResetDirective::ZSTD_reset_session_only)
            .map_err(map_error_code)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    // This requires impl for [u8; N] which is currently behind a feature.
    #[cfg(feature = "arrays")]
    #[test]
    fn test_cycle() {
        use super::{Decoder, Encoder, InBuffer, Operation, OutBuffer};

        let mut encoder = Encoder::new(1).unwrap();
        let mut decoder = Decoder::new().unwrap();

        // Step 1: compress
        let mut input = InBuffer::around(b"AbcdefAbcdefabcdef");

        let mut output = [0u8; 128];
        let mut output = OutBuffer::around(&mut output);

        loop {
            encoder.run(&mut input, &mut output).unwrap();

            if input.pos == input.src.len() {
                break;
            }
        }
        encoder.finish(&mut output, true).unwrap();

        let initial_data = input.src;

        // Step 2: decompress
        let mut input = InBuffer::around(output.as_slice());
        let mut output = [0u8; 128];
        let mut output = OutBuffer::around(&mut output);

        loop {
            decoder.run(&mut input, &mut output).unwrap();

            if input.pos == input.src.len() {
                break;
            }
        }

        assert_eq!(initial_data, output.as_slice());
    }
}
