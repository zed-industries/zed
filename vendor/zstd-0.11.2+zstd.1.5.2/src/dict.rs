//! Train a dictionary from various sources.
//!
//! A dictionary can help improve the compression of small files.
//! The dictionary must be present during decompression,
//! but can be shared accross multiple "similar" files.
//!
//! Creating a dictionary using the `zstd` C library,
//! using the `zstd` command-line interface, using this library,
//! or using the `train` binary provided, should give the same result,
//! and are therefore completely compatible.
//!
//! To use, see [`Encoder::with_dictionary`] or [`Decoder::with_dictionary`].
//!
//! [`Encoder::with_dictionary`]: ../struct.Encoder.html#method.with_dictionary
//! [`Decoder::with_dictionary`]: ../struct.Decoder.html#method.with_dictionary

#[cfg(feature = "zdict_builder")]
use std::io::{self, Read};

pub use zstd_safe::{CDict, DDict};

/// Prepared dictionary for compression
///
/// A dictionary can include its own copy of the data (if it is `'static`), or it can merely point
/// to a separate buffer (if it has another lifetime).
pub struct EncoderDictionary<'a> {
    cdict: CDict<'a>,
}

impl EncoderDictionary<'static> {
    /// Creates a prepared dictionary for compression.
    ///
    /// This will copy the dictionary internally.
    pub fn copy(dictionary: &[u8], level: i32) -> Self {
        Self {
            cdict: zstd_safe::create_cdict(dictionary, level),
        }
    }
}

impl<'a> EncoderDictionary<'a> {
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    /// Create prepared dictionary for compression
    ///
    /// A level of `0` uses zstd's default (currently `3`).
    ///
    /// Only available with the `experimental` feature. Use `EncoderDictionary::copy` otherwise.
    pub fn new(dictionary: &'a [u8], level: i32) -> Self {
        Self {
            cdict: zstd_safe::create_cdict_by_reference(dictionary, level),
        }
    }

    /// Returns reference to `CDict` inner object
    pub fn as_cdict(&self) -> &CDict<'a> {
        &self.cdict
    }
}

/// Prepared dictionary for decompression
pub struct DecoderDictionary<'a> {
    ddict: DDict<'a>,
}

impl DecoderDictionary<'static> {
    /// Create a prepared dictionary for decompression.
    ///
    /// This will copy the dictionary internally.
    pub fn copy(dictionary: &[u8]) -> Self {
        Self {
            ddict: zstd_safe::DDict::create(dictionary),
        }
    }
}

impl<'a> DecoderDictionary<'a> {
    #[cfg(feature = "experimental")]
    #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
    /// Create prepared dictionary for decompression
    ///
    /// Only available with the `experimental` feature. Use `DecoderDictionary::copy` otherwise.
    pub fn new(dict: &'a [u8]) -> Self {
        Self {
            ddict: zstd_safe::create_ddict_by_reference(dict),
        }
    }

    /// Returns reference to `DDict` inner object
    pub fn as_ddict(&self) -> &DDict<'a> {
        &self.ddict
    }
}

/// Train a dictionary from a big continuous chunk of data.
///
/// This is the most efficient way to train a dictionary,
/// since this is directly fed into `zstd`.
#[cfg(feature = "zdict_builder")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "zdict_builder")))]
pub fn from_continuous(
    sample_data: &[u8],
    sample_sizes: &[usize],
    max_size: usize,
) -> io::Result<Vec<u8>> {
    use crate::map_error_code;

    // Complain if the lengths don't add up to the entire data.
    if sample_sizes.iter().sum::<usize>() != sample_data.len() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "sample sizes don't add up".to_string(),
        ));
    }

    let mut result = Vec::with_capacity(max_size);
    zstd_safe::train_from_buffer(&mut result, sample_data, sample_sizes)
        .map_err(map_error_code)?;
    Ok(result)
}

/// Train a dictionary from multiple samples.
///
/// The samples will internaly be copied to a single continuous buffer,
/// so make sure you have enough memory available.
///
/// If you need to stretch your system's limits,
/// [`from_continuous`] directly uses the given slice.
///
/// [`from_continuous`]: ./fn.from_continuous.html
#[cfg(feature = "zdict_builder")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "zdict_builder")))]
pub fn from_samples<S: AsRef<[u8]>>(
    samples: &[S],
    max_size: usize,
) -> io::Result<Vec<u8>> {
    // Copy every sample to a big chunk of memory
    let data: Vec<_> =
        samples.iter().flat_map(|s| s.as_ref()).cloned().collect();
    let sizes: Vec<_> = samples.iter().map(|s| s.as_ref().len()).collect();

    from_continuous(&data, &sizes, max_size)
}

/// Train a dict from a list of files.
#[cfg(feature = "zdict_builder")]
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "zdict_builder")))]
pub fn from_files<I, P>(filenames: I, max_size: usize) -> io::Result<Vec<u8>>
where
    P: AsRef<std::path::Path>,
    I: IntoIterator<Item = P>,
{
    use std::fs;

    let mut buffer = Vec::new();
    let mut sizes = Vec::new();

    for filename in filenames {
        let mut file = fs::File::open(filename)?;
        let len = file.read_to_end(&mut buffer)?;
        sizes.push(len);
    }

    from_continuous(&buffer, &sizes, max_size)
}

#[cfg(test)]
#[cfg(feature = "zdict_builder")]
mod tests {
    use std::fs;
    use std::io;
    use std::io::Read;

    use walkdir;

    #[test]
    fn test_dict_training() {
        // Train a dictionary
        let paths: Vec<_> = walkdir::WalkDir::new("src")
            .into_iter()
            .map(|entry| entry.unwrap())
            .map(|entry| entry.into_path())
            .filter(|path| path.to_str().unwrap().ends_with(".rs"))
            .collect();

        let dict = super::from_files(&paths, 4000).unwrap();

        for path in paths {
            let mut buffer = Vec::new();
            let mut file = fs::File::open(path).unwrap();
            let mut content = Vec::new();
            file.read_to_end(&mut content).unwrap();
            io::copy(
                &mut &content[..],
                &mut crate::stream::Encoder::with_dictionary(
                    &mut buffer,
                    1,
                    &dict,
                )
                .unwrap()
                .auto_finish(),
            )
            .unwrap();

            let mut result = Vec::new();
            io::copy(
                &mut crate::stream::Decoder::with_dictionary(
                    &buffer[..],
                    &dict[..],
                )
                .unwrap(),
                &mut result,
            )
            .unwrap();

            assert_eq!(&content, &result);
        }
    }
}
