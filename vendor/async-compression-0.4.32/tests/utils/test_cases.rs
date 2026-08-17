macro_rules! io_test_cases {
    ($impl:ident, $variant:ident) => {
        mod $impl {
            mod bufread {
                mod compress {
                    use crate::utils::{
                        algos::$variant::{
                            sync,
                            $impl::{bufread, read},
                        },
                        one_to_six, one_to_six_stream, InputStream, Level,
                    };

                    #[test]
                    #[ntest::timeout(1000)]
                    fn empty() {
                        let mut input: &[u8] = &[];
                        let compressed = bufread::compress(&mut input);
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, &[][..]);
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn to_full_output() {
                        let mut output = [];

                        let encoder = bufread::Encoder::new(bufread::from(&one_to_six_stream()));
                        let result = read::poll_read(encoder, &mut output);
                        assert!(matches!(result, Ok(0)));
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn empty_chunk() {
                        let input = InputStream::new(vec![vec![]]);

                        let compressed = bufread::compress(bufread::from(&input));
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, input.bytes());
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn short() {
                        let compressed = bufread::compress(bufread::from(&one_to_six_stream()));
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn long() {
                        let input = InputStream::new(vec![
                            (0..32_768).map(|_| rand::random()).collect(),
                            (0..32_768).map(|_| rand::random()).collect(),
                        ]);

                        let compressed = bufread::compress(bufread::from(&input));
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, input.bytes());
                    }

                    #[test]
                    fn with_level_best() {
                        let encoder = bufread::Encoder::with_quality(
                            bufread::from(&one_to_six_stream()),
                            Level::Best,
                        );
                        let compressed = read::to_vec(encoder);
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    fn with_level_default() {
                        let encoder = bufread::Encoder::new(bufread::from(&one_to_six_stream()));
                        let compressed = read::to_vec(encoder);
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    fn with_level_0() {
                        let encoder = bufread::Encoder::with_quality(
                            bufread::from(&one_to_six_stream()),
                            Level::Precise(0),
                        );
                        let compressed = read::to_vec(encoder);
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    fn with_level_max() {
                        let encoder = bufread::Encoder::with_quality(
                            bufread::from(&one_to_six_stream()),
                            Level::Precise(i32::MAX),
                        );
                        let compressed = read::to_vec(encoder);
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, one_to_six());
                    }
                }

                mod decompress {
                    use crate::utils::{
                        algos::$variant::{
                            sync,
                            $impl::{bufread, read},
                        },
                        one_to_six, one_to_six_stream, InputStream,
                    };

                    #[test]
                    #[ntest::timeout(1000)]
                    fn empty() {
                        let compressed = sync::compress(&[]);

                        let input = InputStream::new(vec![compressed]);
                        let output = bufread::decompress(bufread::from(&input));

                        assert_eq!(output, &[][..]);
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn to_full_output() {
                        let mut output = [];

                        let decoder = bufread::Decoder::new(bufread::from(&one_to_six_stream()));
                        let result = read::poll_read(decoder, &mut output);
                        assert!(matches!(result, Ok(0)));
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn zeros() {
                        let compressed = sync::compress(&[0; 10]);

                        let input = InputStream::new(vec![compressed]);
                        let output = bufread::decompress(bufread::from(&input));

                        assert_eq!(output, &[0; 10][..]);
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn short() {
                        let compressed = sync::compress(&[1, 2, 3, 4, 5, 6]);

                        let input = InputStream::new(vec![compressed]);
                        let output = bufread::decompress(bufread::from(&input));

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn short_chunks() {
                        let compressed = sync::compress(&[1, 2, 3, 4, 5, 6]);

                        let input = InputStream::from(compressed.chunks(2));
                        let output = bufread::decompress(bufread::from(&input));

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn trailer() {
                        let mut compressed = sync::compress(&[1, 2, 3, 4, 5, 6]);

                        compressed.extend_from_slice(&[7, 8, 9, 10]);

                        let input = InputStream::new(vec![compressed]);
                        let mut reader = bufread::from(&input);
                        let output = bufread::decompress(&mut reader);
                        let trailer = read::to_vec(reader);

                        assert_eq!(output, one_to_six());
                        assert_eq!(trailer, &[7, 8, 9, 10][..]);
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn long() {
                        let bytes: Vec<u8> = (0..65_536).map(|_| rand::random()).collect();
                        let compressed = sync::compress(&bytes);

                        let input = InputStream::new(vec![compressed]);
                        let output = bufread::decompress(bufread::from(&input));

                        assert_eq!(output, bytes);
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn long_chunks() {
                        let bytes: Vec<u8> = (0..65_536).map(|_| rand::random()).collect();
                        let compressed = sync::compress(&bytes);

                        let input = InputStream::from(compressed.chunks(1024));
                        let output = bufread::decompress(bufread::from(&input));

                        assert_eq!(output, bytes);
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn multiple_members() {
                        let compressed = [
                            sync::compress(&[1, 2, 3, 4, 5, 6]),
                            sync::compress(&[6, 5, 4, 3, 2, 1]),
                        ]
                        .join(&[][..]);

                        let input = InputStream::new(vec![compressed]);

                        let mut decoder = bufread::Decoder::new(bufread::from(&input));
                        decoder.multiple_members(true);
                        let output = read::to_vec(decoder);

                        assert_eq!(output, &[1, 2, 3, 4, 5, 6, 6, 5, 4, 3, 2, 1][..]);
                    }
                }
            }

            mod write {
                mod compress {
                    use crate::utils::{
                        algos::$variant::{sync, $impl::write},
                        one_to_six, one_to_six_stream, InputStream, Level,
                    };

                    #[test]
                    #[ntest::timeout(1000)]
                    fn empty() {
                        let input = InputStream::new(vec![]);

                        let compressed = write::compress(input.as_ref(), 65_536);
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, &[][..]);
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn empty_chunk() {
                        let input = InputStream::new(vec![vec![]]);

                        let compressed = write::compress(input.as_ref(), 65_536);
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, input.bytes());
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn short() {
                        let compressed = write::compress(one_to_six_stream().as_ref(), 65_536);
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn short_chunk_output() {
                        let compressed = write::compress(one_to_six_stream().as_ref(), 2);
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn long() {
                        let input = InputStream::new(vec![
                            (0..32_768).map(|_| rand::random()).collect(),
                            (0..32_768).map(|_| rand::random()).collect(),
                        ]);

                        let compressed = write::compress(input.as_ref(), 65_536);
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, input.bytes());
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn long_chunk_output() {
                        let input = InputStream::new(vec![
                            (0..32_768).map(|_| rand::random()).collect(),
                            (0..32_768).map(|_| rand::random()).collect(),
                        ]);

                        let compressed = write::compress(input.as_ref(), 20);
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, input.bytes());
                    }

                    #[test]
                    fn with_level_best() {
                        let compressed = write::to_vec(
                            one_to_six_stream().as_ref(),
                            |input| Box::pin(write::Encoder::with_quality(input, Level::Best)),
                            65_536,
                        );
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    fn with_level_default() {
                        let compressed = write::to_vec(
                            one_to_six_stream().as_ref(),
                            |input| Box::pin(write::Encoder::new(input)),
                            65_536,
                        );
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    fn with_level_0() {
                        let compressed = write::to_vec(
                            one_to_six_stream().as_ref(),
                            |input| {
                                Box::pin(write::Encoder::with_quality(input, Level::Precise(0)))
                            },
                            65_536,
                        );
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    fn with_level_max() {
                        let compressed = write::to_vec(
                            one_to_six_stream().as_ref(),
                            |input| {
                                Box::pin(write::Encoder::with_quality(
                                    input,
                                    Level::Precise(i32::MAX),
                                ))
                            },
                            65_536,
                        );
                        let output = sync::decompress(&compressed);

                        assert_eq!(output, one_to_six());
                    }
                }

                mod decompress {
                    use crate::utils::{
                        algos::$variant::{sync, $impl::write},
                        one_to_six, InputStream,
                    };

                    #[test]
                    #[ntest::timeout(1000)]
                    fn empty() {
                        let compressed = sync::compress(&[]);

                        let input = InputStream::new(vec![compressed]);
                        let output = write::decompress(input.as_ref(), 65_536);

                        assert_eq!(output, &[][..]);
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn zeros() {
                        let compressed = sync::compress(&[0; 10]);

                        let input = InputStream::new(vec![compressed]);
                        let output = write::decompress(input.as_ref(), 65_536);

                        assert_eq!(output, &[0; 10][..]);
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn short() {
                        let compressed = sync::compress(&[1, 2, 3, 4, 5, 6]);

                        let input = InputStream::new(vec![compressed]);
                        let output = write::decompress(input.as_ref(), 65_536);

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn short_chunks() {
                        let compressed = sync::compress(&[1, 2, 3, 4, 5, 6]);

                        let input = InputStream::from(compressed.chunks(2));
                        let output = write::decompress(input.as_ref(), 65_536);

                        assert_eq!(output, one_to_six());
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn long() {
                        let bytes: Vec<u8> = (0..65_536).map(|_| rand::random()).collect();
                        let compressed = sync::compress(&bytes);

                        let input = InputStream::new(vec![compressed]);
                        let output = write::decompress(input.as_ref(), 65_536);

                        assert_eq!(output, bytes);
                    }

                    #[test]
                    #[ntest::timeout(1000)]
                    fn long_chunks() {
                        let bytes: Vec<u8> = (0..65_536).map(|_| rand::random()).collect();
                        let compressed = sync::compress(&bytes);

                        let input = InputStream::from(compressed.chunks(1024));
                        let output = write::decompress(input.as_ref(), 65_536);

                        assert_eq!(output, bytes);
                    }
                }
            }
        }
    };
}

macro_rules! test_cases {
    ($variant:ident) => {
        mod $variant {
            #[cfg(feature = "futures-io")]
            io_test_cases!(futures, $variant);

            #[cfg(feature = "tokio")]
            io_test_cases!(tokio, $variant);
        }
    };
}
