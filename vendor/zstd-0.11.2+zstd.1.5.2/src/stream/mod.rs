//! Compress and decompress Zstd streams.
//!
//! Zstd streams are the main way to compress and decompress data.
//! They are compatible with the `zstd` command-line tool.
//!
//! This module provides both `Read` and `Write` interfaces to compressing and
//! decompressing.

pub mod read;
pub mod write;

mod functions;
pub mod zio;

#[cfg(test)]
mod tests;

pub mod raw;

pub use self::functions::{copy_decode, copy_encode, decode_all, encode_all};
pub use self::read::Decoder;
pub use self::write::{AutoFinishEncoder, Encoder};

#[doc(hidden)]
#[macro_export]
/// Common functions for the decoder, both in read and write mode.
macro_rules! decoder_parameters {
    () => {
        /// Sets the maximum back-reference distance.
        ///
        /// The actual maximum distance is going to be `2^log_distance`.
        ///
        /// This will need to at least match the value set when compressing.
        pub fn window_log_max(&mut self, log_distance: u32) -> io::Result<()> {
            self.set_parameter(zstd_safe::DParameter::WindowLogMax(
                log_distance,
            ))
        }

        #[cfg(feature = "experimental")]
        #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
        /// Enables or disabled expecting the 4-byte magic header
        ///
        /// Only available with the `experimental` feature.
        ///
        /// This will need to match the settings used when compressing.
        pub fn include_magicbytes(
            &mut self,
            include_magicbytes: bool,
        ) -> io::Result<()> {
            self.set_parameter(zstd_safe::DParameter::Format(
                if include_magicbytes {
                    zstd_safe::FrameFormat::One
                } else {
                    zstd_safe::FrameFormat::Magicless
                },
            ))
        }
    };
}

#[doc(hidden)]
#[macro_export]
/// Common functions for the decoder, both in read and write mode.
macro_rules! decoder_common {
    ($readwrite:ident) => {
        /// Sets a decompression parameter on the decompression stream.
        pub fn set_parameter(
            &mut self,
            parameter: zstd_safe::DParameter,
        ) -> io::Result<()> {
            self.$readwrite.operation_mut().set_parameter(parameter)
        }

        $crate::decoder_parameters!();
    };
}

#[doc(hidden)]
#[macro_export]
/// Parameter-setters for the encoder. Relies on a `set_parameter` method.
macro_rules! encoder_parameters {
    () => {
        /// Controls whether zstd should include a content checksum at the end
        /// of each frame.
        pub fn include_checksum(
            &mut self,
            include_checksum: bool,
        ) -> io::Result<()> {
            self.set_parameter(zstd_safe::CParameter::ChecksumFlag(
                include_checksum,
            ))
        }

        /// Enables multithreaded compression
        ///
        /// * If `n_workers == 0` (default), then multithreaded will be
        ///   disabled.
        /// * If `n_workers >= 1`, then compression will be done in separate
        ///   threads.
        ///
        /// So even `n_workers = 1` may increase performance by separating
        /// IO and compression.
        ///
        /// Note: This is only available if the `zstdmt` cargo feature is activated.
        #[cfg(feature = "zstdmt")]
        #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "zstdmt")))]
        pub fn multithread(&mut self, n_workers: u32) -> io::Result<()> {
            self.set_parameter(zstd_safe::CParameter::NbWorkers(n_workers))
        }

        /// Enables or disables storing of the dict id.
        ///
        /// Defaults to true. If false, the behaviour of decoding with a wrong
        /// dictionary is undefined.
        pub fn include_dictid(
            &mut self,
            include_dictid: bool,
        ) -> io::Result<()> {
            self.set_parameter(zstd_safe::CParameter::DictIdFlag(
                include_dictid,
            ))
        }

        /// Enables or disabled storing of the contentsize.
        ///
        /// Note that this only has an effect if the size is given with `set_pledged_src_size`.
        pub fn include_contentsize(
            &mut self,
            include_contentsize: bool,
        ) -> io::Result<()> {
            self.set_parameter(zstd_safe::CParameter::ContentSizeFlag(
                include_contentsize,
            ))
        }
        /// Enables or disables long-distance matching
        pub fn long_distance_matching(
            &mut self,
            long_distance_matching: bool,
        ) -> io::Result<()> {
            self.set_parameter(
                zstd_safe::CParameter::EnableLongDistanceMatching(
                    long_distance_matching,
                ),
            )
        }

        /// Sets the maximum back-reference distance.
        ///
        /// The actual maximum distance is going to be `2^log_distance`.
        ///
        /// Note that decompression will need to use at least the same setting.
        pub fn window_log(&mut self, log_distance: u32) -> io::Result<()> {
            self.set_parameter(zstd_safe::CParameter::WindowLog(log_distance))
        }

        #[cfg(feature = "experimental")]
        #[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "experimental")))]
        /// Enables or disable the magic bytes at the beginning of each frame.
        ///
        /// If disabled, include_magicbytes must also be called on the decoder.
        ///
        /// Only available with the `experimental` feature.
        ///
        /// Note that decompression will need to use the same setting.
        pub fn include_magicbytes(
            &mut self,
            include_magicbytes: bool,
        ) -> io::Result<()> {
            self.set_parameter(zstd_safe::CParameter::Format(
                if include_magicbytes {
                    zstd_safe::FrameFormat::One
                } else {
                    zstd_safe::FrameFormat::Magicless
                },
            ))
        }
    };
}

#[doc(hidden)]
#[macro_export]
/// Common functions for the encoder, both in read and write mode.
macro_rules! encoder_common {
    ($readwrite:ident) => {
        /// Sets the given zstd compression parameter.
        pub fn set_parameter(
            &mut self,
            parameter: zstd_safe::CParameter,
        ) -> io::Result<()> {
            self.$readwrite.operation_mut().set_parameter(parameter)
        }

        /// Sets the expected size of the input.
        ///
        /// This affects the compression effectiveness.
        ///
        /// It is an error to give an incorrect size (an error will be returned when closing the
        /// stream if the size does not match what was pledged).
        ///
        /// Giving a `None` size means the size is unknown (this is the default).
        pub fn set_pledged_src_size(
            &mut self,
            size: Option<u64>,
        ) -> io::Result<()> {
            match size {
                Some(size) => {
                    self.$readwrite.operation_mut().set_pledged_src_size(size)
                }
                None => self
                    .$readwrite
                    .operation_mut()
                    .set_pledged_src_size(zstd_safe::CONTENTSIZE_UNKNOWN),
            }
        }

        $crate::encoder_parameters!();
    };
}
