use compression_core::Level;

use ::proptest::{
    arbitrary::any,
    prop_oneof,
    strategy::{Just, Strategy},
};

mod utils;

#[allow(dead_code)]
fn any_level() -> impl Strategy<Value = Level> {
    prop_oneof![
        Just(Level::Fastest),
        Just(Level::Best),
        Just(Level::Default),
        any::<i32>().prop_map(Level::Precise),
    ]
}

#[allow(unused_macros)]
macro_rules! io_tests {
    ($impl:ident, $variant:ident) => {
        mod $impl {
            mod bufread {
                use crate::utils::{algos::$variant::{$impl::{read, bufread}, sync}, InputStream};
                use proptest::{prelude::{any, ProptestConfig}, proptest};
                use std::iter::FromIterator;

                proptest! {
                    #[test]
                    fn compress(ref input in any::<InputStream>()) {
                        let compressed = bufread::compress(bufread::from(input));
                        let output = sync::decompress(&compressed);
                        assert_eq!(output, input.bytes());
                    }

                    #[test]
                    fn decompress(
                        ref bytes in any::<Vec<u8>>(),
                        chunk_size in 1..20usize,
                    ) {
                        let compressed = sync::compress(bytes);
                        let input = InputStream::from(Vec::from_iter(compressed.chunks(chunk_size).map(Vec::from)));
                        let output = bufread::decompress(bufread::from(&input));
                        assert_eq!(&output, bytes);
                    }
                }

                proptest! {
                    #![proptest_config(ProptestConfig::with_cases(32))]

                    #[test]
                    fn compress_with_level(
                        ref input in any::<InputStream>(),
                        level in crate::any_level(),
                    ) {
                        let encoder = bufread::Encoder::with_quality(bufread::from(input), level);
                        let compressed = read::to_vec(encoder);
                        let output = sync::decompress(&compressed);
                        assert_eq!(output, input.bytes());
                    }
                }
            }

            mod write {
                use crate::utils::{algos::$variant::{$impl::write, sync}, InputStream};
                use proptest::{prelude::{any, ProptestConfig}, proptest};

                proptest! {
                    #[test]
                    fn compress(
                        ref input in any::<InputStream>(),
                        limit in 1..20usize,
                    ) {
                        let compressed = write::compress(input.as_ref(), limit);
                        let output = sync::decompress(&compressed);
                        assert_eq!(output, input.bytes());
                    }
                }

                proptest! {
                    #![proptest_config(ProptestConfig::with_cases(32))]

                    #[test]
                    fn compress_with_level(
                        ref input in any::<InputStream>(),
                        limit in 1..20usize,
                        level in crate::any_level(),
                    ) {
                        let compressed = write::to_vec(
                            input.as_ref(),
                            |input| Box::pin(write::Encoder::with_quality(input, level)),
                            limit,
                        );
                        let output = sync::decompress(&compressed);
                        assert_eq!(output, input.bytes());
                    }
                }
            }
        }
    }
}

#[allow(unused_macros)]
macro_rules! tests {
    ($variant:ident) => {
        mod $variant {
            #[cfg(feature = "futures-io")]
            io_tests!(futures, $variant);

            #[cfg(feature = "tokio")]
            io_tests!(tokio, $variant);
        }
    };
}

mod proptest {
    #[cfg(feature = "brotli")]
    tests!(brotli);

    #[cfg(feature = "bzip2")]
    tests!(bzip2);

    #[cfg(feature = "deflate")]
    tests!(deflate);

    #[cfg(feature = "gzip")]
    tests!(gzip);

    #[cfg(feature = "lz4")]
    tests!(lz4);

    #[cfg(feature = "lzma")]
    tests!(lzma);

    #[cfg(feature = "xz")]
    tests!(xz);

    #[cfg(feature = "zlib")]
    tests!(zlib);

    #[cfg(feature = "zstd")]
    tests!(zstd);
}
