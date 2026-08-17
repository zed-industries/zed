use crate::map_error_code;

#[cfg(feature = "experimental")]
use std::convert::TryInto;
use std::io;
use zstd_safe;

/// Allows to decompress independently multiple blocks of data.
///
/// This reduces memory usage compared to calling `decompress` multiple times.
#[derive(Default)]
pub struct Decompressor<'a> {
    context: zstd_safe::DCtx<'a>,
}

impl Decompressor<'static> {
    /// Creates a new zstd decompressor.
    pub fn new() -> io::Result<Self> {
        Self::with_dictionary(&[])
    }

    /// Creates a new zstd decompressor, using the given dictionary.
    pub fn with_dictionary(dictionary: &[u8]) -> io::Result<Self> {
        let mut decompressor = Self::default();

        decompressor.set_dictionary(dictionary)?;

        Ok(decompressor)
    }
}

impl<'a> Decompressor<'a> {
    /// Creates a new decompressor using an existing `DecoderDictionary`.
    ///
    /// Note that using a dictionary means that compression will need to use
    /// the same dictionary.
    pub fn with_prepared_dictionary<'b>(
        dictionary: &'a crate::dict::DecoderDictionary<'b>,
    ) -> io::Result<Self>
    where
        'b: 'a,
    {
        let mut decompressor = Self::default();

        decompressor.set_prepared_dictionary(dictionary)?;

        Ok(decompressor)
    }

    /// Changes the dictionary used by this decompressor.
    ///
    /// Will affect future compression jobs.
    ///
    /// Note that using a dictionary means that compression will need to use
    /// the same dictionary.
    pub fn set_dictionary(&mut self, dictionary: &[u8]) -> io::Result<()> {
        self.context
            .load_dictionary(dictionary)
            .map_err(map_error_code)?;

        Ok(())
    }

    /// Changes the dictionary used by this decompressor.
    ///
    /// Note that using a dictionary means that compression will need to use
    /// the same dictionary.
    pub fn set_prepared_dictionary<'b>(
        &mut self,
        dictionary: &'a crate::dict::DecoderDictionary<'b>,
    ) -> io::Result<()>
    where
        'b: 'a,
    {
        self.context
            .ref_ddict(dictionary.as_ddict())
            .map_err(map_error_code)?;

        Ok(())
    }

    /// Deompress a single block of data to the given destination buffer.
    ///
    /// Returns the number of bytes written, or an error if something happened
    /// (for instance if the destination buffer was too small).
    pub fn decompress_to_buffer<C: zstd_safe::WriteBuf + ?Sized>(
        &mut self,
        source: &[u8],
        destination: &mut C,
    ) -> io::Result<usize> {
        self.context
            .decompress(destination, source)
            .map_err(map_error_code)
    }

    /// Decompress a block of data, and return the result in a `Vec<u8>`.
    ///
    /// The decompressed data should be less than `capacity` bytes,
    /// or an error will be returned.
    pub fn decompress(
        &mut self,
        data: &[u8],
        capacity: usize,
    ) -> io::Result<Vec<u8>> {
        let capacity =
            Self::upper_bound(data).unwrap_or(capacity).min(capacity);
        let mut buffer = Vec::with_capacity(capacity);
        self.decompress_to_buffer(data, &mut buffer)?;
        Ok(buffer)
    }

    /// Sets a decompression parameter for this decompressor.
    pub fn set_parameter(
        &mut self,
        parameter: zstd_safe::DParameter,
    ) -> io::Result<()> {
        self.context
            .set_parameter(parameter)
            .map_err(map_error_code)?;
        Ok(())
    }

    crate::decoder_parameters!();

    /// Get an upper bound on the decompressed size of data, if available
    ///
    /// This can be used to pre-allocate enough capacity for `decompress_to_buffer`
    /// and is used by `decompress` to ensure that it does not over-allocate if
    /// you supply a large `capacity`.
    ///
    /// Will return `None` if the upper bound cannot be determined or is larger than `usize::MAX`
    ///
    /// Note that unless the `experimental` feature is enabled, this will always return `None`.
    pub fn upper_bound(_data: &[u8]) -> Option<usize> {
        #[cfg(feature = "experimental")]
        {
            let bound = zstd_safe::decompress_bound(_data).ok()?;
            bound.try_into().ok()
        }
        #[cfg(not(feature = "experimental"))]
        {
            None
        }
    }
}

fn _assert_traits() {
    fn _assert_send<T: Send>(_: T) {}

    _assert_send(Decompressor::new());
}
