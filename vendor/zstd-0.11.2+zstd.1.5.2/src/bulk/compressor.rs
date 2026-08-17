use crate::map_error_code;

use std::io;
use zstd_safe;

/// Allows to compress independently multiple chunks of data.
///
/// Each job will be processed entirely in-memory without streaming, so this
/// is most fitting for many small jobs. To compress larger volume that don't
/// easily fit in memory, a streaming compression may be more appropriate.
///
/// It is more efficient than a streaming compressor for 2 reasons:
/// * It re-uses the zstd context between jobs to avoid re-allocations
/// * It avoids copying data from a `Read` into a temporary buffer before compression.
#[derive(Default)]
pub struct Compressor<'a> {
    context: zstd_safe::CCtx<'a>,
}

impl Compressor<'static> {
    /// Creates a new zstd compressor
    pub fn new(level: i32) -> io::Result<Self> {
        Self::with_dictionary(level, &[])
    }

    /// Creates a new zstd compressor, using the given dictionary.
    ///
    /// Note that using a dictionary means that decompression will need to use
    /// the same dictionary.
    pub fn with_dictionary(level: i32, dictionary: &[u8]) -> io::Result<Self> {
        let mut compressor = Self::default();

        compressor.set_dictionary(level, dictionary)?;

        Ok(compressor)
    }
}

impl<'a> Compressor<'a> {
    /// Creates a new compressor using an existing `EncoderDictionary`.
    ///
    /// The compression level will be the one specified when creating the dictionary.
    ///
    /// Note that using a dictionary means that decompression will need to use
    /// the same dictionary.
    pub fn with_prepared_dictionary<'b>(
        dictionary: &'a crate::dict::EncoderDictionary<'b>,
    ) -> io::Result<Self>
    where
        'b: 'a,
    {
        let mut compressor = Self::default();

        compressor.set_prepared_dictionary(dictionary)?;

        Ok(compressor)
    }

    /// Changes the compression level used by this compressor.
    ///
    /// *This will clear any dictionary previously registered.*
    ///
    /// If you want to keep the existing dictionary, you will need to pass it again to
    /// `Self::set_dictionary` instead of using this method.
    pub fn set_compression_level(&mut self, level: i32) -> io::Result<()> {
        self.set_dictionary(level, &[])
    }

    /// Changes the dictionary and compression level used by this compressor.
    ///
    /// Will affect future compression jobs.
    ///
    /// Note that using a dictionary means that decompression will need to use
    /// the same dictionary.
    pub fn set_dictionary(
        &mut self,
        level: i32,
        dictionary: &[u8],
    ) -> io::Result<()> {
        self.context
            .set_parameter(zstd_safe::CParameter::CompressionLevel(level))
            .map_err(map_error_code)?;

        self.context
            .load_dictionary(dictionary)
            .map_err(map_error_code)?;

        Ok(())
    }

    /// Changes the dictionary used by this compressor.
    ///
    /// The compression level used when preparing the dictionary will be used.
    ///
    /// Note that using a dictionary means that decompression will need to use
    /// the same dictionary.
    pub fn set_prepared_dictionary<'b>(
        &mut self,
        dictionary: &'a crate::dict::EncoderDictionary<'b>,
    ) -> io::Result<()>
    where
        'b: 'a,
    {
        self.context
            .ref_cdict(dictionary.as_cdict())
            .map_err(map_error_code)?;

        Ok(())
    }

    /// Compress a single block of data to the given destination buffer.
    ///
    /// Returns the number of bytes written, or an error if something happened
    /// (for instance if the destination buffer was too small).
    ///
    /// A level of `0` uses zstd's default (currently `3`).
    pub fn compress_to_buffer<C: zstd_safe::WriteBuf + ?Sized>(
        &mut self,
        source: &[u8],
        destination: &mut C,
    ) -> io::Result<usize> {
        self.context
            .compress2(destination, source)
            .map_err(map_error_code)
    }

    /// Compresses a block of data and returns the compressed result.
    ///
    /// A level of `0` uses zstd's default (currently `3`).
    pub fn compress(&mut self, data: &[u8]) -> io::Result<Vec<u8>> {
        // We allocate a big buffer, slightly larger than the input data.
        let buffer_len = zstd_safe::compress_bound(data.len());
        let mut buffer = Vec::with_capacity(buffer_len);

        self.compress_to_buffer(data, &mut buffer)?;

        // Should we shrink the vec? Meh, let the user do it if he wants.
        Ok(buffer)
    }

    /// Gives mutable access to the internal context.
    pub fn context_mut(&mut self) -> &mut zstd_safe::CCtx<'a> {
        &mut self.context
    }

    /// Sets a compression parameter for this compressor.
    pub fn set_parameter(
        &mut self,
        parameter: zstd_safe::CParameter,
    ) -> io::Result<()> {
        self.context
            .set_parameter(parameter)
            .map_err(map_error_code)?;
        Ok(())
    }

    crate::encoder_parameters!();
}

fn _assert_traits() {
    fn _assert_send<T: Send>(_: T) {}

    _assert_send(Compressor::new(0));
}
