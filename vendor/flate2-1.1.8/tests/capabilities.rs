//! Validate that certain feature-gated functionality is still available.
#[cfg(feature = "any_zlib")]
use flate2::{Compress, Compression, Decompress, FlushCompress, FlushDecompress};

// Unsupported for `miniz_oxide`.
#[cfg(feature = "any_zlib")]
#[test]
fn compress_new_with_window_bits_is_present_and_works() {
    let string = "hello world".as_bytes();

    // Test with window_bits = 9 (minimum)
    let mut encoded = Vec::with_capacity(1024);
    let mut encoder = Compress::new_with_window_bits(Compression::default(), true, 9);
    encoder
        .compress_vec(string, &mut encoded, FlushCompress::Finish)
        .unwrap();
    assert_ne!(encoded.len(), 0);

    let mut decoder = Decompress::new_with_window_bits(true, 9);
    let mut decoded = [0; 1024];
    decoder
        .decompress(&encoded, &mut decoded, FlushDecompress::Finish)
        .unwrap();
    assert_eq!(&decoded[..string.len()], string);

    // Test with window_bits = 15 (maximum)
    let mut encoded = Vec::with_capacity(1024);
    let mut encoder = Compress::new_with_window_bits(Compression::default(), false, 15);
    encoder
        .compress_vec(string, &mut encoded, FlushCompress::Finish)
        .unwrap();
    assert_ne!(encoded.len(), 0);

    let mut decoder = Decompress::new_with_window_bits(false, 15);
    let mut decoded = [0; 1024];
    decoder
        .decompress(&encoded, &mut decoded, FlushDecompress::Finish)
        .unwrap();
    assert_eq!(&decoded[..string.len()], string);
}

// Unsupported for `miniz_oxide`.
#[cfg(feature = "any_zlib")]
#[test]
fn decompress_new_gzip_window_bits_is_present_and_works() {
    let string = "hello world".as_bytes();

    // Test with different window_bits values
    for window_bits in [9, 12, 15] {
        let mut encoded = Vec::with_capacity(1024);
        let mut encoder = Compress::new_gzip(Compression::default(), window_bits);
        encoder
            .compress_vec(string, &mut encoded, FlushCompress::Finish)
            .unwrap();

        let mut decoder = Decompress::new_gzip(window_bits);
        let mut decoded = [0; 1024];
        decoder
            .decompress(&encoded, &mut decoded, FlushDecompress::Finish)
            .unwrap();
        assert_eq!(
            &decoded[..string.len()],
            string,
            "Failed with window_bits={}",
            window_bits
        );
    }
}

// Unsupported for `miniz_oxide`.
#[cfg(feature = "any_zlib")]
#[test]
#[should_panic(expected = "window_bits must be within 9 ..= 15")]
fn compress_new_with_window_bits_invalid_low() {
    let _ = Compress::new_with_window_bits(Compression::default(), true, 8);
}

// Unsupported for `miniz_oxide`.
#[cfg(feature = "any_zlib")]
#[test]
#[should_panic(expected = "window_bits must be within 9 ..= 15")]
fn compress_new_with_window_bits_invalid_high() {
    let _ = Compress::new_with_window_bits(Compression::default(), true, 16);
}

// Unsupported for `miniz_oxide`.
#[cfg(feature = "any_zlib")]
#[test]
#[should_panic(expected = "window_bits must be within 9 ..= 15")]
fn compress_new_gzip_invalid_low() {
    let _ = Compress::new_gzip(Compression::default(), 8);
}

// Unsupported for `miniz_oxide`.
#[cfg(feature = "any_zlib")]
#[test]
#[should_panic(expected = "window_bits must be within 9 ..= 15")]
fn compress_new_gzip_invalid_high() {
    let _ = Compress::new_gzip(Compression::default(), 16);
}

// Unsupported for `miniz_oxide`.
#[cfg(feature = "any_zlib")]
#[test]
fn set_dictionary_with_zlib_header() {
    let string = "hello, hello!".as_bytes();
    let dictionary = "hello".as_bytes();

    let mut encoded = Vec::with_capacity(1024);

    let mut encoder = Compress::new(Compression::default(), true);

    let dictionary_adler = encoder.set_dictionary(dictionary).unwrap();

    encoder
        .compress_vec(string, &mut encoded, FlushCompress::Finish)
        .unwrap();

    assert_eq!(encoder.total_in(), string.len() as u64);
    assert_eq!(encoder.total_out(), encoded.len() as u64);

    let mut decoder = Decompress::new(true);
    let mut decoded = [0; 1024];
    let decompress_error = decoder
        .decompress(&encoded, &mut decoded, FlushDecompress::Finish)
        .expect_err("decompression should fail due to requiring a dictionary");

    let required_adler = decompress_error.needs_dictionary()
        .expect("the first call to decompress should indicate a dictionary is required along with the required Adler-32 checksum");

    assert_eq!(required_adler, dictionary_adler,
               "the Adler-32 checksum should match the value when the dictionary was set on the compressor");

    let actual_adler = decoder.set_dictionary(dictionary).unwrap();

    assert_eq!(required_adler, actual_adler);

    // Decompress the rest of the input to the remainder of the output buffer
    let total_in = decoder.total_in();
    let total_out = decoder.total_out();

    let decompress_result = decoder.decompress(
        &encoded[total_in as usize..],
        &mut decoded[total_out as usize..],
        FlushDecompress::Finish,
    );
    assert!(decompress_result.is_ok());

    assert_eq!(&decoded[..decoder.total_out() as usize], string);
}

// Unsupported for `miniz_oxide`.
#[cfg(feature = "any_zlib")]
#[test]
fn set_dictionary_raw() {
    let string = "hello, hello!".as_bytes();
    let dictionary = "hello".as_bytes();

    let mut encoded = Vec::with_capacity(1024);

    let mut encoder = Compress::new(Compression::default(), false);

    encoder.set_dictionary(dictionary).unwrap();

    encoder
        .compress_vec(string, &mut encoded, FlushCompress::Finish)
        .unwrap();

    assert_eq!(encoder.total_in(), string.len() as u64);
    assert_eq!(encoder.total_out(), encoded.len() as u64);

    let mut decoder = Decompress::new(false);

    decoder.set_dictionary(dictionary).unwrap();

    let mut decoded = [0; 1024];
    let decompress_result = decoder.decompress(&encoded, &mut decoded, FlushDecompress::Finish);

    assert!(decompress_result.is_ok());

    assert_eq!(&decoded[..decoder.total_out() as usize], string);
}

// Unsupported for `miniz_oxide`.
#[cfg(feature = "any_zlib")]
#[test]
fn compression_levels_are_effective() {
    let input = b"hello hello hello hello hello hello hello hello";

    // Compress with no compression
    let mut encoded_none = Vec::new();
    Compress::new(Compression::none(), true)
        .compress_vec(input, &mut encoded_none, FlushCompress::Finish)
        .unwrap();

    // Compress with best compression
    let mut encoded_best = Vec::new();
    Compress::new(Compression::best(), true)
        .compress_vec(input, &mut encoded_best, FlushCompress::Finish)
        .unwrap();

    assert!(
        encoded_best.len() <= encoded_none.len(),
        "best compression produced larger output than no compression: best={}, none={}",
        encoded_best.len(),
        encoded_none.len(),
    );
}

// Unsupported for `miniz_oxide`.
#[cfg(feature = "any_zlib")]
#[test]
fn set_level_is_effective() {
    let input = b"hello hello hello hello hello hello hello hello";
    let no_compression = Compression::none();
    let best_compression = Compression::best();

    // Compress with no compression
    let mut encoded_none = Vec::new();
    let mut compress = Compress::new(best_compression, true);
    compress.set_level(no_compression).unwrap();
    compress
        .compress_vec(input, &mut encoded_none, FlushCompress::Finish)
        .unwrap();

    // Compress with best compression
    let mut encoded_best = Vec::new();
    let mut compress = Compress::new(no_compression, true);
    compress.set_level(best_compression).unwrap();
    compress
        .compress_vec(input, &mut encoded_best, FlushCompress::Finish)
        .unwrap();

    assert!(
        encoded_best.len() <= encoded_none.len(),
        "best compression produced larger output than no compression: best={}, none={}",
        encoded_best.len(),
        encoded_none.len(),
    );
}
