macro_rules! algos {
    (@algo $algo:ident [$algo_s:expr] $decoder:ident $encoder:ident <$inner:ident>
        { @enc $($encoder_methods:tt)* }
        { @dec $($decoder_methods:tt)* }
    ) => {
        #[cfg(feature = $algo_s)]
        decoder! {
            #[doc = concat!("A ", $algo_s, " decoder, or decompressor")]
            #[cfg(feature = $algo_s)]
            $decoder<$inner>

            { $($decoder_methods)* }
        }

        #[cfg(feature = $algo_s)]
        encoder! {
            #[doc = concat!("A ", $algo_s, " encoder, or compressor.")]
            #[cfg(feature = $algo_s)]
            $encoder<$inner> {
                pub fn new(inner: $inner) -> Self {
                    Self::with_quality(inner, crate::core::Level::Default)
                }
            }

            { $($encoder_methods)* }
        }
    };

    (@algo $algo:ident [$algo_s:expr] $decoder:ident $encoder:ident <$inner:ident>
        { @dec $($decoder_methods:tt)* }
    ) => {
        #[cfg(feature = $algo_s)]
        decoder! {
            #[doc = concat!("A ", $algo_s, " decoder, or decompressor")]
            #[cfg(feature = $algo_s)]
            $decoder<$inner>

            { $($decoder_methods)* }
        }
    };

    ($($mod:ident)::+ <$inner:ident>) => {
        algos!(@algo brotli ["brotli"] BrotliDecoder BrotliEncoder <$inner>
        { @enc
            pub fn with_quality(inner: $inner, level: crate::core::Level) -> Self {
                // let params = brotli::enc::backward_references::BrotliEncoderParams::default();
                let params = crate::codecs::brotli::params::EncoderParams::default();
                let params = params.quality(level);
                Self {
                    inner: crate::$($mod::)+generic::Encoder::new(
                        inner,
                        crate::codecs::BrotliEncoder::new(params),
                    ),
                }
            }

            /// Creates a new encoder, using the specified compression level and parameters, which
            /// will read uncompressed data from the given stream and emit a compressed stream.
            pub fn with_params(
                inner: $inner,
                params: crate::codecs::brotli::params::EncoderParams,
            ) -> Self {
                Self {
                    inner: crate::$($mod::)+generic::Encoder::new(
                        inner,
                        crate::codecs::BrotliEncoder::new(params),
                    ),
                }
            }
        }
        { @dec }
        );

        algos!(@algo bzip2 ["bzip2"] BzDecoder BzEncoder <$inner>
        { @enc

            pub fn with_quality(inner: $inner, level: crate::core::Level) -> Self {
                let params = crate::codecs::bzip2::params::Bzip2EncoderParams::from(level);
                Self {
                    inner: crate::$($mod::)+generic::Encoder::new(
                        inner,
                        crate::codecs::BzEncoder::new(params, 0),
                    ),
                }
            }
        }
        { @dec }
        );

        algos!(@algo deflate ["deflate"] DeflateDecoder DeflateEncoder <$inner>
        { @enc
            pub fn with_quality(inner: $inner, level: crate::core::Level) -> Self {
                let mut params = crate::codecs::flate::params::FlateEncoderParams::from(level);

                Self {
                    inner: crate::$($mod::)+generic::Encoder::new(
                        inner,
                        crate::codecs::DeflateEncoder::new(params),
                    ),
                }
            }

            /// Returns the total number of input bytes which have been processed by this compression object.
            pub fn total_in(&self) -> u64 {
                self.inner.get_encoder_ref().get_ref().get_ref().total_in()
            }

            /// Returns the total number of output bytes which have been produced by this compression object.
            pub fn total_out(&self) -> u64 {
                self.inner.get_encoder_ref().get_ref().get_ref().total_out()
            }
        }
        { @dec }
        );

        algos!(@algo deflate ["deflate64"] Deflate64Decoder Deflate64Encoder <$inner>
        { @dec }
        );

        algos!(@algo gzip ["gzip"] GzipDecoder GzipEncoder <$inner>
        { @enc

            pub fn with_quality(inner: $inner, level: crate::core::Level) -> Self {
                let params = crate::codecs::flate::params::FlateEncoderParams::from(level);
                Self {
                    inner: crate::$($mod::)+generic::Encoder::new(
                        inner,
                        crate::codecs::GzipEncoder::new(params),
                    ),
                }
            }
        }
        { @dec }
        );

        algos!(@algo zlib ["zlib"] ZlibDecoder ZlibEncoder <$inner>
        { @enc
            pub fn with_quality(inner: $inner, level: crate::core::Level) -> Self {
                  let params = crate::codecs::flate::params::FlateEncoderParams::from(level);
                Self {
                    inner: crate::$($mod::)+generic::Encoder::new(
                        inner,
                        crate::codecs::ZlibEncoder::new(params),
                    ),
                }
            }

            /// Returns the total number of input bytes which have been processed by this compression object.
            pub fn total_in(&self) -> u64 {
                self.inner.get_encoder_ref().get_ref().get_ref().total_in()
            }

            /// Returns the total number of output bytes which have been produced by this compression object.
            pub fn total_out(&self) -> u64 {
                self.inner.get_encoder_ref().get_ref().get_ref().total_out()
            }
        }
        { @dec }
        );

        algos!(@algo zstd ["zstd"] ZstdDecoder ZstdEncoder <$inner>
        { @enc

            pub fn with_quality(inner: $inner, level: crate::core::Level) -> Self {
                let params = crate::codecs::zstd::params::CParameter::quality(level);
                Self {
                    inner: crate::$($mod::)+generic::Encoder::new(
                        inner,
                        crate::codecs::ZstdEncoder::new(params),
                    ),
                }
            }

            /// Creates a new encoder, using the specified compression level and parameters, which
            /// will read uncompressed data from the given stream and emit a compressed stream.
            ///
            /// # Panics
            ///
            /// Panics if this function is called with a [`CParameter::nb_workers()`] parameter and
            /// the `zstdmt` crate feature is _not_ enabled.
            ///
            /// [`CParameter::nb_workers()`]: crate::codecs::zstd::params::CParameter
            //
            // TODO: remove panic note on next breaking release, along with `CParameter::nb_workers`
            // change
            pub fn with_quality_and_params(inner: $inner, level: crate::core::Level, params: &[crate::codecs::zstd::params::CParameter]) -> Self {
                let level = crate::codecs::zstd::params::CParameter::quality(level);
                Self {
                    inner: crate::$($mod::)+generic::Encoder::new(
                        inner,
                        crate::codecs::ZstdEncoder::new_with_params(level, params),
                    ),
                }
            }

            /// Creates a new encoder, using the specified compression level and pre-trained
            /// dictionary, which will read uncompressed data from the given stream and emit a
            /// compressed stream.
            ///
            /// Dictionaries provide better compression ratios for small files, but are required to
            /// be present during decompression.
            ///
            /// # Errors
            ///
            /// Returns error when `dictionary` is not valid.
            pub fn with_dict(inner: $inner, level: crate::core::Level, dictionary: &[u8]) -> ::std::io::Result<Self> {
                let level = crate::codecs::zstd::params::CParameter::quality(level);
                Ok(Self {
                    inner: crate::$($mod::)+generic::Encoder::new(
                        inner,
                        crate::codecs::ZstdEncoder::new_with_dict(level, dictionary)?,
                    ),
                })
            }
        }
        { @dec
            /// Creates a new decoder, using the specified parameters, which will read compressed
            /// data from the given stream and emit a decompressed stream.
            pub fn with_params(inner: $inner, params: &[crate::codecs::zstd::params::DParameter]) -> Self {
                Self {
                    inner: crate::$($mod::)+generic::Decoder::new(
                        inner,
                        crate::codecs::ZstdDecoder::new_with_params(params),
                    ),
                }
            }

            /// Creates a new decoder, using the specified compression level and pre-trained
            /// dictionary, which will read compressed data from the given stream and emit an
            /// uncompressed stream.
            ///
            /// Dictionaries provide better compression ratios for small files, but are required to
            /// be present during decompression. The dictionary used must be the same as the one
            /// used for compression.
            ///
            /// # Errors
            ///
            /// Returns error when `dictionary` is not valid.
            pub fn with_dict(inner: $inner, dictionary: &[u8]) -> ::std::io::Result<Self> {
                Ok(Self {
                    inner: crate::$($mod::)+generic::Decoder::new(
                        inner,
                        crate::codecs::ZstdDecoder::new_with_dict(dictionary)?,
                    ),
                })
            }
        }
        );

        algos!(@algo xz ["xz"] XzDecoder XzEncoder <$inner>
        { @enc

            pub fn with_quality(inner: $inner, level: crate::core::Level) -> Self {
                Self {
                    inner: crate::$($mod::)+generic::Encoder::new(
                        inner,
                        crate::codecs::XzEncoder::new(level),
                    ),
                }
            }

            /// Creates a new multi-threaded encoder.
            ///
            /// Note that flushing will severely impact multi-threaded performance.
            #[cfg(feature = "xz-parallel")]
            pub fn parallel(inner: $inner, level: crate::core::Level, threads: std::num::NonZeroU32) -> Self {
                Self {
                    inner: crate::$($mod::)+generic::Encoder::new(
                        inner,
                        crate::codecs::XzEncoder::parallel(threads, level),
                    ),
                }
            }
        }
        { @dec
            /// Creates a new decoder with the specified limit of memory.
            ///
            /// # Errors
            ///
            /// An IO error may be returned during decoding if the specified limit is too small.
            pub fn with_mem_limit(read: $inner, memlimit: u64) -> Self {
                Self {
                    inner: crate::$($mod::)+generic::Decoder::new(
                        read,
                        crate::codecs::XzDecoder::with_memlimit(memlimit),
                    ),
                }
            }

            /// Creates a new multi-threaded decoder.
            #[cfg(feature = "xz-parallel")]
            pub fn parallel(read: $inner, threads: std::num::NonZeroU32) -> Self {
                use std::convert::TryInto;

                Self {
                    inner: crate::$($mod::)+generic::Decoder::new(
                        read,
                        crate::codecs::XzDecoder::parallel(threads, usize::MAX.try_into().unwrap()),
                    ),
                }
            }

            /// Creates a new multi-threaded decoder with the specified limit of memory.
            ///
            /// # Errors
            ///
            /// An IO error may be returned during decoding if the specified limit is too small.
            #[cfg(feature = "xz-parallel")]
            pub fn parallel_with_mem_limit(read: $inner, threads: std::num::NonZeroU32, memlimit: u64) -> Self {
                Self {
                    inner: crate::$($mod::)+generic::Decoder::new(
                        read,
                        crate::codecs::XzDecoder::parallel(threads, memlimit),
                    ),
                }
            }
        }
        );

        algos!(@algo lzma ["lzma"] LzmaDecoder LzmaEncoder <$inner>
        { @enc

            pub fn with_quality(inner: $inner, level:  crate::core::Level) -> Self {
                let encoder = crate::codecs::LzmaEncoder::new(level);
                let inner =  crate::$($mod::)+generic::Encoder::new(inner, encoder);
                Self {
                    inner
                }
            }
        }
        { @dec
            /// Creates a new decoder with the specified limit of memory.
            ///
            /// # Errors
            ///
            /// An IO error may be returned during decoding if the specified limit is too small.
            pub fn with_mem_limit(read: $inner, memlimit: u64) -> Self {
                Self {
                    inner: crate::$($mod::)+generic::Decoder::new(
                        read,
                        crate::codecs::LzmaDecoder::with_memlimit(memlimit),
                    ),
                }
            }

        }
        );

        algos!(@algo lz4 ["lz4"] Lz4Decoder Lz4Encoder <$inner>
        { @enc

            pub fn with_quality(inner: $inner, level: crate::core::Level) -> Self {
                Self::with_quality_and_params(inner, level, crate::codecs::lz4::params::EncoderParams::default())
            }

            /// Creates a new encoder, using the specified compression level and parameters, which
            /// will read uncompressed data from the given stream and emit a compressed stream.
            pub fn with_quality_and_params(
                inner: $inner,
                level: crate::core::Level,
                mut params: crate::codecs::lz4::params::EncoderParams,
            ) -> Self {
                let params = params.level(level);
                let encoder = crate::codecs::Lz4Encoder::new(params);
                let cap = encoder.buffer_size();
                Self {
                    inner: crate::$($mod::)+generic::Encoder::with_capacity(
                        inner,
                        encoder,
                        cap,
                    ),
                }
            }
        }
        { @dec }
        );

    }
}
