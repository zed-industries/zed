// rstest_reuse template functions have unused variables
#![allow(unused_variables)]

use rand::{
    self,
    distributions::{self, Distribution as _},
    rngs, Rng as _, SeedableRng as _,
};
use rstest::rstest;
use rstest_reuse::{apply, template};
use std::{collections, fmt, io::Read as _};

use crate::{
    alphabet::{Alphabet, STANDARD},
    encode::add_padding,
    encoded_len,
    engine::{
        general_purpose, naive, Config, DecodeEstimate, DecodeMetadata, DecodePaddingMode, Engine,
    },
    read::DecoderReader,
    tests::{assert_encode_sanity, random_alphabet, random_config},
    DecodeError, PAD_BYTE,
};

// the case::foo syntax includes the "foo" in the generated test method names
#[template]
#[rstest(engine_wrapper,
case::general_purpose(GeneralPurposeWrapper {}),
case::naive(NaiveWrapper {}),
case::decoder_reader(DecoderReaderEngineWrapper {}),
)]
fn all_engines<E: EngineWrapper>(engine_wrapper: E) {}

/// Some decode tests don't make sense for use with `DecoderReader` as they are difficult to
/// reason about or otherwise inapplicable given how DecoderReader slice up its input along
/// chunk boundaries.
#[template]
#[rstest(engine_wrapper,
case::general_purpose(GeneralPurposeWrapper {}),
case::naive(NaiveWrapper {}),
)]
fn all_engines_except_decoder_reader<E: EngineWrapper>(engine_wrapper: E) {}

#[apply(all_engines)]
fn rfc_test_vectors_std_alphabet<E: EngineWrapper>(engine_wrapper: E) {
    let data = vec![
        ("", ""),
        ("f", "Zg=="),
        ("fo", "Zm8="),
        ("foo", "Zm9v"),
        ("foob", "Zm9vYg=="),
        ("fooba", "Zm9vYmE="),
        ("foobar", "Zm9vYmFy"),
    ];

    let engine = E::standard();
    let engine_no_padding = E::standard_unpadded();

    for (orig, encoded) in &data {
        let encoded_without_padding = encoded.trim_end_matches('=');

        // unpadded
        {
            let mut encode_buf = [0_u8; 8];
            let mut decode_buf = [0_u8; 6];

            let encode_len =
                engine_no_padding.internal_encode(orig.as_bytes(), &mut encode_buf[..]);
            assert_eq!(
                &encoded_without_padding,
                &std::str::from_utf8(&encode_buf[0..encode_len]).unwrap()
            );
            let decode_len = engine_no_padding
                .decode_slice_unchecked(encoded_without_padding.as_bytes(), &mut decode_buf[..])
                .unwrap();
            assert_eq!(orig.len(), decode_len);

            assert_eq!(
                orig,
                &std::str::from_utf8(&decode_buf[0..decode_len]).unwrap()
            );

            // if there was any padding originally, the no padding engine won't decode it
            if encoded.as_bytes().contains(&PAD_BYTE) {
                assert_eq!(
                    Err(DecodeError::InvalidPadding),
                    engine_no_padding.decode(encoded)
                )
            }
        }

        // padded
        {
            let mut encode_buf = [0_u8; 8];
            let mut decode_buf = [0_u8; 6];

            let encode_len = engine.internal_encode(orig.as_bytes(), &mut encode_buf[..]);
            assert_eq!(
                // doesn't have padding added yet
                &encoded_without_padding,
                &std::str::from_utf8(&encode_buf[0..encode_len]).unwrap()
            );
            let pad_len = add_padding(encode_len, &mut encode_buf[encode_len..]);
            assert_eq!(encoded.as_bytes(), &encode_buf[..encode_len + pad_len]);

            let decode_len = engine
                .decode_slice_unchecked(encoded.as_bytes(), &mut decode_buf[..])
                .unwrap();
            assert_eq!(orig.len(), decode_len);

            assert_eq!(
                orig,
                &std::str::from_utf8(&decode_buf[0..decode_len]).unwrap()
            );

            // if there was (canonical) padding, and we remove it, the standard engine won't decode
            if encoded.as_bytes().contains(&PAD_BYTE) {
                assert_eq!(
                    Err(DecodeError::InvalidPadding),
                    engine.decode(encoded_without_padding)
                )
            }
        }
    }
}

#[apply(all_engines)]
fn roundtrip_random<E: EngineWrapper>(engine_wrapper: E) {
    let mut rng = seeded_rng();

    let mut orig_data = Vec::<u8>::new();
    let mut encode_buf = Vec::<u8>::new();
    let mut decode_buf = Vec::<u8>::new();

    let len_range = distributions::Uniform::new(1, 1_000);

    for _ in 0..10_000 {
        let engine = E::random(&mut rng);

        orig_data.clear();
        encode_buf.clear();
        decode_buf.clear();

        let (orig_len, _, encoded_len) = generate_random_encoded_data(
            &engine,
            &mut orig_data,
            &mut encode_buf,
            &mut rng,
            &len_range,
        );

        // exactly the right size
        decode_buf.resize(orig_len, 0);

        let dec_len = engine
            .decode_slice_unchecked(&encode_buf[0..encoded_len], &mut decode_buf[..])
            .unwrap();

        assert_eq!(orig_len, dec_len);
        assert_eq!(&orig_data[..], &decode_buf[..dec_len]);
    }
}

#[apply(all_engines)]
fn encode_doesnt_write_extra_bytes<E: EngineWrapper>(engine_wrapper: E) {
    let mut rng = seeded_rng();

    let mut orig_data = Vec::<u8>::new();
    let mut encode_buf = Vec::<u8>::new();
    let mut encode_buf_backup = Vec::<u8>::new();

    let input_len_range = distributions::Uniform::new(0, 1000);

    for _ in 0..10_000 {
        let engine = E::random(&mut rng);
        let padded = engine.config().encode_padding();

        orig_data.clear();
        encode_buf.clear();
        encode_buf_backup.clear();

        let orig_len = fill_rand(&mut orig_data, &mut rng, &input_len_range);

        let prefix_len = 1024;
        // plenty of prefix and suffix
        fill_rand_len(&mut encode_buf, &mut rng, prefix_len * 2 + orig_len * 2);
        encode_buf_backup.extend_from_slice(&encode_buf[..]);

        let expected_encode_len_no_pad = encoded_len(orig_len, false).unwrap();

        let encoded_len_no_pad =
            engine.internal_encode(&orig_data[..], &mut encode_buf[prefix_len..]);
        assert_eq!(expected_encode_len_no_pad, encoded_len_no_pad);

        // no writes past what it claimed to write
        assert_eq!(&encode_buf_backup[..prefix_len], &encode_buf[..prefix_len]);
        assert_eq!(
            &encode_buf_backup[(prefix_len + encoded_len_no_pad)..],
            &encode_buf[(prefix_len + encoded_len_no_pad)..]
        );

        let encoded_data = &encode_buf[prefix_len..(prefix_len + encoded_len_no_pad)];
        assert_encode_sanity(
            std::str::from_utf8(encoded_data).unwrap(),
            // engines don't pad
            false,
            orig_len,
        );

        // pad so we can decode it in case our random engine requires padding
        let pad_len = if padded {
            add_padding(
                encoded_len_no_pad,
                &mut encode_buf[prefix_len + encoded_len_no_pad..],
            )
        } else {
            0
        };

        assert_eq!(
            orig_data,
            engine
                .decode(&encode_buf[prefix_len..(prefix_len + encoded_len_no_pad + pad_len)],)
                .unwrap()
        );
    }
}

#[apply(all_engines)]
fn encode_engine_slice_fits_into_precisely_sized_slice<E: EngineWrapper>(engine_wrapper: E) {
    let mut orig_data = Vec::new();
    let mut encoded_data = Vec::new();
    let mut decoded = Vec::new();

    let input_len_range = distributions::Uniform::new(0, 1000);

    let mut rng = rngs::SmallRng::from_entropy();

    for _ in 0..10_000 {
        orig_data.clear();
        encoded_data.clear();
        decoded.clear();

        let input_len = input_len_range.sample(&mut rng);

        for _ in 0..input_len {
            orig_data.push(rng.gen());
        }

        let engine = E::random(&mut rng);

        let encoded_size = encoded_len(input_len, engine.config().encode_padding()).unwrap();

        encoded_data.resize(encoded_size, 0);

        assert_eq!(
            encoded_size,
            engine.encode_slice(&orig_data, &mut encoded_data).unwrap()
        );

        assert_encode_sanity(
            std::str::from_utf8(&encoded_data[0..encoded_size]).unwrap(),
            engine.config().encode_padding(),
            input_len,
        );

        engine
            .decode_vec(&encoded_data[0..encoded_size], &mut decoded)
            .unwrap();
        assert_eq!(orig_data, decoded);
    }
}

#[apply(all_engines)]
fn decode_doesnt_write_extra_bytes<E>(engine_wrapper: E)
where
    E: EngineWrapper,
    <<E as EngineWrapper>::Engine as Engine>::Config: fmt::Debug,
{
    let mut rng = seeded_rng();

    let mut orig_data = Vec::<u8>::new();
    let mut encode_buf = Vec::<u8>::new();
    let mut decode_buf = Vec::<u8>::new();
    let mut decode_buf_backup = Vec::<u8>::new();

    let len_range = distributions::Uniform::new(1, 1_000);

    for _ in 0..10_000 {
        let engine = E::random(&mut rng);

        orig_data.clear();
        encode_buf.clear();
        decode_buf.clear();
        decode_buf_backup.clear();

        let orig_len = fill_rand(&mut orig_data, &mut rng, &len_range);
        encode_buf.resize(orig_len * 2 + 100, 0);

        let encoded_len = engine
            .encode_slice(&orig_data[..], &mut encode_buf[..])
            .unwrap();
        encode_buf.truncate(encoded_len);

        // oversize decode buffer so we can easily tell if it writes anything more than
        // just the decoded data
        let prefix_len = 1024;
        // plenty of prefix and suffix
        fill_rand_len(&mut decode_buf, &mut rng, prefix_len * 2 + orig_len * 2);
        decode_buf_backup.extend_from_slice(&decode_buf[..]);

        let dec_len = engine
            .decode_slice_unchecked(&encode_buf, &mut decode_buf[prefix_len..])
            .unwrap();

        assert_eq!(orig_len, dec_len);
        assert_eq!(
            &orig_data[..],
            &decode_buf[prefix_len..prefix_len + dec_len]
        );
        assert_eq!(&decode_buf_backup[..prefix_len], &decode_buf[..prefix_len]);
        assert_eq!(
            &decode_buf_backup[prefix_len + dec_len..],
            &decode_buf[prefix_len + dec_len..]
        );
    }
}

#[apply(all_engines)]
fn decode_detect_invalid_last_symbol<E: EngineWrapper>(engine_wrapper: E) {
    // 0xFF -> "/w==", so all letters > w, 0-9, and '+', '/' should get InvalidLastSymbol
    let engine = E::standard();

    assert_eq!(Ok(vec![0x89, 0x85]), engine.decode("iYU="));
    assert_eq!(Ok(vec![0xFF]), engine.decode("/w=="));

    for (suffix, offset) in vec![
        // suffix, offset of bad byte from start of suffix
        ("/x==", 1_usize),
        ("/z==", 1_usize),
        ("/0==", 1_usize),
        ("/9==", 1_usize),
        ("/+==", 1_usize),
        ("//==", 1_usize),
        // trailing 01
        ("iYV=", 2_usize),
        // trailing 10
        ("iYW=", 2_usize),
        // trailing 11
        ("iYX=", 2_usize),
    ] {
        for prefix_quads in 0..256 {
            let mut encoded = "AAAA".repeat(prefix_quads);
            encoded.push_str(suffix);

            assert_eq!(
                Err(DecodeError::InvalidLastSymbol(
                    encoded.len() - 4 + offset,
                    suffix.as_bytes()[offset],
                )),
                engine.decode(encoded.as_str())
            );
        }
    }
}

#[apply(all_engines)]
fn decode_detect_invalid_last_symbol_when_length_is_also_invalid<E: EngineWrapper>(
    engine_wrapper: E,
) {
    let mut rng = seeded_rng();

    // check across enough lengths that it would likely cover any implementation's various internal
    // small/large input division
    for len in (0_usize..256).map(|len| len * 4 + 1) {
        let engine = E::random_alphabet(&mut rng, &STANDARD);

        let mut input = vec![b'A'; len];

        // with a valid last char, it's InvalidLength
        assert_eq!(Err(DecodeError::InvalidLength), engine.decode(&input));
        // after mangling the last char, it's InvalidByte
        input[len - 1] = b'"';
        assert_eq!(
            Err(DecodeError::InvalidByte(len - 1, b'"')),
            engine.decode(&input)
        );
    }
}

#[apply(all_engines)]
fn decode_detect_invalid_last_symbol_every_possible_two_symbols<E: EngineWrapper>(
    engine_wrapper: E,
) {
    let engine = E::standard();

    let mut base64_to_bytes = collections::HashMap::new();

    for b in 0_u8..=255 {
        let mut b64 = vec![0_u8; 4];
        assert_eq!(2, engine.internal_encode(&[b], &mut b64[..]));
        let _ = add_padding(2, &mut b64[2..]);

        assert!(base64_to_bytes.insert(b64, vec![b]).is_none());
    }

    // every possible combination of trailing symbols must either decode to 1 byte or get InvalidLastSymbol, with or without any leading chunks

    let mut prefix = Vec::new();
    for _ in 0..256 {
        let mut clone = prefix.clone();

        let mut symbols = [0_u8; 4];
        for &s1 in STANDARD.symbols.iter() {
            symbols[0] = s1;
            for &s2 in STANDARD.symbols.iter() {
                symbols[1] = s2;
                symbols[2] = PAD_BYTE;
                symbols[3] = PAD_BYTE;

                // chop off previous symbols
                clone.truncate(prefix.len());
                clone.extend_from_slice(&symbols[..]);
                let decoded_prefix_len = prefix.len() / 4 * 3;

                match base64_to_bytes.get(&symbols[..]) {
                    Some(bytes) => {
                        let res = engine
                            .decode(&clone)
                            // remove prefix
                            .map(|decoded| decoded[decoded_prefix_len..].to_vec());

                        assert_eq!(Ok(bytes.clone()), res);
                    }
                    None => assert_eq!(
                        Err(DecodeError::InvalidLastSymbol(1, s2)),
                        engine.decode(&symbols[..])
                    ),
                }
            }
        }

        prefix.extend_from_slice(b"AAAA");
    }
}

#[apply(all_engines)]
fn decode_detect_invalid_last_symbol_every_possible_three_symbols<E: EngineWrapper>(
    engine_wrapper: E,
) {
    let engine = E::standard();

    let mut base64_to_bytes = collections::HashMap::new();

    let mut bytes = [0_u8; 2];
    for b1 in 0_u8..=255 {
        bytes[0] = b1;
        for b2 in 0_u8..=255 {
            bytes[1] = b2;
            let mut b64 = vec![0_u8; 4];
            assert_eq!(3, engine.internal_encode(&bytes, &mut b64[..]));
            let _ = add_padding(3, &mut b64[3..]);

            let mut v = Vec::with_capacity(2);
            v.extend_from_slice(&bytes[..]);

            assert!(base64_to_bytes.insert(b64, v).is_none());
        }
    }

    // every possible combination of symbols must either decode to 2 bytes or get InvalidLastSymbol, with or without any leading chunks

    let mut prefix = Vec::new();
    for _ in 0..256 {
        let mut input = prefix.clone();

        let mut symbols = [0_u8; 4];
        for &s1 in STANDARD.symbols.iter() {
            symbols[0] = s1;
            for &s2 in STANDARD.symbols.iter() {
                symbols[1] = s2;
                for &s3 in STANDARD.symbols.iter() {
                    symbols[2] = s3;
                    symbols[3] = PAD_BYTE;

                    // chop off previous symbols
                    input.truncate(prefix.len());
                    input.extend_from_slice(&symbols[..]);
                    let decoded_prefix_len = prefix.len() / 4 * 3;

                    match base64_to_bytes.get(&symbols[..]) {
                        Some(bytes) => {
                            let res = engine
                                .decode(&input)
                                // remove prefix
                                .map(|decoded| decoded[decoded_prefix_len..].to_vec());

                            assert_eq!(Ok(bytes.clone()), res);
                        }
                        None => assert_eq!(
                            Err(DecodeError::InvalidLastSymbol(2, s3)),
                            engine.decode(&symbols[..])
                        ),
                    }
                }
            }
        }
        prefix.extend_from_slice(b"AAAA");
    }
}

#[apply(all_engines)]
fn decode_invalid_trailing_bits_ignored_when_configured<E: EngineWrapper>(engine_wrapper: E) {
    let strict = E::standard();
    let forgiving = E::standard_allow_trailing_bits();

    fn assert_tolerant_decode<E: Engine>(
        engine: &E,
        input: &mut String,
        b64_prefix_len: usize,
        expected_decode_bytes: Vec<u8>,
        data: &str,
    ) {
        let prefixed = prefixed_data(input, b64_prefix_len, data);
        let decoded = engine.decode(prefixed);
        // prefix is always complete chunks
        let decoded_prefix_len = b64_prefix_len / 4 * 3;
        assert_eq!(
            Ok(expected_decode_bytes),
            decoded.map(|v| v[decoded_prefix_len..].to_vec())
        );
    }

    let mut prefix = String::new();
    for _ in 0..256 {
        let mut input = prefix.clone();

        // example from https://github.com/marshallpierce/rust-base64/issues/75
        assert!(strict
            .decode(prefixed_data(&mut input, prefix.len(), "/w=="))
            .is_ok());
        assert!(strict
            .decode(prefixed_data(&mut input, prefix.len(), "iYU="))
            .is_ok());
        // trailing 01
        assert_tolerant_decode(&forgiving, &mut input, prefix.len(), vec![255], "/x==");
        assert_tolerant_decode(&forgiving, &mut input, prefix.len(), vec![137, 133], "iYV=");
        // trailing 10
        assert_tolerant_decode(&forgiving, &mut input, prefix.len(), vec![255], "/y==");
        assert_tolerant_decode(&forgiving, &mut input, prefix.len(), vec![137, 133], "iYW=");
        // trailing 11
        assert_tolerant_decode(&forgiving, &mut input, prefix.len(), vec![255], "/z==");
        assert_tolerant_decode(&forgiving, &mut input, prefix.len(), vec![137, 133], "iYX=");

        prefix.push_str("AAAA");
    }
}

#[apply(all_engines)]
fn decode_invalid_byte_error<E: EngineWrapper>(engine_wrapper: E) {
    let mut rng = seeded_rng();

    let mut orig_data = Vec::<u8>::new();
    let mut encode_buf = Vec::<u8>::new();
    let mut decode_buf = Vec::<u8>::new();

    let len_range = distributions::Uniform::new(1, 1_000);

    for _ in 0..100_000 {
        let alphabet = random_alphabet(&mut rng);
        let engine = E::random_alphabet(&mut rng, alphabet);

        orig_data.clear();
        encode_buf.clear();
        decode_buf.clear();

        let (orig_len, encoded_len_just_data, encoded_len_with_padding) =
            generate_random_encoded_data(
                &engine,
                &mut orig_data,
                &mut encode_buf,
                &mut rng,
                &len_range,
            );

        // exactly the right size
        decode_buf.resize(orig_len, 0);

        // replace one encoded byte with an invalid byte
        let invalid_byte: u8 = loop {
            let byte: u8 = rng.gen();

            if alphabet.symbols.contains(&byte) || byte == PAD_BYTE {
                continue;
            } else {
                break byte;
            }
        };

        let invalid_range = distributions::Uniform::new(0, orig_len);
        let invalid_index = invalid_range.sample(&mut rng);
        encode_buf[invalid_index] = invalid_byte;

        assert_eq!(
            Err(DecodeError::InvalidByte(invalid_index, invalid_byte)),
            engine.decode_slice_unchecked(
                &encode_buf[0..encoded_len_with_padding],
                &mut decode_buf[..],
            )
        );
    }
}

/// Any amount of padding anywhere before the final non padding character = invalid byte at first
/// pad byte.
/// From this, we know padding must extend to the end of the input.
// DecoderReader pseudo-engine detects InvalidLastSymbol instead of InvalidLength because it
// can end a decode on the quad that happens to contain the start of the padding
#[apply(all_engines_except_decoder_reader)]
fn decode_padding_before_final_non_padding_char_error_invalid_byte<E: EngineWrapper>(
    engine_wrapper: E,
) {
    let mut rng = seeded_rng();

    // the different amounts of proper padding, w/ offset from end for the last non-padding char
    let suffixes = [("/w==", 2), ("iYu=", 1), ("zzzz", 0)];

    let prefix_quads_range = distributions::Uniform::from(0..=256);

    for mode in all_pad_modes() {
        // we don't encode so we don't care about encode padding
        let engine = E::standard_with_pad_mode(true, mode);

        for _ in 0..100_000 {
            for (suffix, offset) in suffixes.iter() {
                let mut s = "ABCD".repeat(prefix_quads_range.sample(&mut rng));
                s.push_str(suffix);
                let mut encoded = s.into_bytes();

                // calculate a range to write padding into that leaves at least one non padding char
                let last_non_padding_offset = encoded.len() - 1 - offset;

                // don't include last non padding char as it must stay not padding
                let padding_end = rng.gen_range(0..last_non_padding_offset);

                // don't use more than 100 bytes of padding, but also use shorter lengths when
                // padding_end is near the start of the encoded data to avoid biasing to padding
                // the entire prefix on short lengths
                let padding_len = rng.gen_range(1..=usize::min(100, padding_end + 1));
                let padding_start = padding_end.saturating_sub(padding_len);

                encoded[padding_start..=padding_end].fill(PAD_BYTE);

                assert_eq!(
                    Err(DecodeError::InvalidByte(padding_start, PAD_BYTE)),
                    engine.decode(&encoded),
                );
            }
        }
    }
}

/// Any amount of padding before final chunk that crosses over into final chunk with 2-4 bytes =
/// invalid byte at first pad byte.
/// From this and [decode_padding_starts_before_final_chunk_error_invalid_length] we know the
/// padding must start in the final chunk.
// DecoderReader pseudo-engine detects InvalidLastSymbol instead of InvalidLength because it
// can end a decode on the quad that happens to contain the start of the padding
#[apply(all_engines_except_decoder_reader)]
fn decode_padding_starts_before_final_chunk_error_invalid_byte<E: EngineWrapper>(
    engine_wrapper: E,
) {
    let mut rng = seeded_rng();

    // must have at least one prefix quad
    let prefix_quads_range = distributions::Uniform::from(1..256);
    // excluding 1 since we don't care about invalid length in this test
    let suffix_pad_len_range = distributions::Uniform::from(2..=4);
    for mode in all_pad_modes() {
        // we don't encode so we don't care about encode padding
        let engine = E::standard_with_pad_mode(true, mode);
        for _ in 0..100_000 {
            let suffix_len = suffix_pad_len_range.sample(&mut rng);
            let mut encoded = "ABCD"
                .repeat(prefix_quads_range.sample(&mut rng))
                .into_bytes();
            encoded.resize(encoded.len() + suffix_len, PAD_BYTE);

            // amount of padding must be long enough to extend back from suffix into previous
            // quads
            let padding_len = rng.gen_range(suffix_len + 1..encoded.len());
            // no non-padding after padding in this test, so padding goes to the end
            let padding_start = encoded.len() - padding_len;
            encoded[padding_start..].fill(PAD_BYTE);

            assert_eq!(
                Err(DecodeError::InvalidByte(padding_start, PAD_BYTE)),
                engine.decode(&encoded),
                "suffix_len: {}, padding_len: {}, b64: {}",
                suffix_len,
                padding_len,
                std::str::from_utf8(&encoded).unwrap()
            );
        }
    }
}

/// Any amount of padding before final chunk that crosses over into final chunk with 1 byte =
/// invalid length.
/// From this we know the padding must start in the final chunk.
// DecoderReader pseudo-engine detects InvalidByte instead of InvalidLength because it starts by
// decoding only the available complete quads
#[apply(all_engines_except_decoder_reader)]
fn decode_padding_starts_before_final_chunk_error_invalid_length<E: EngineWrapper>(
    engine_wrapper: E,
) {
    let mut rng = seeded_rng();

    // must have at least one prefix quad
    let prefix_quads_range = distributions::Uniform::from(1..256);
    for mode in all_pad_modes() {
        // we don't encode so we don't care about encode padding
        let engine = E::standard_with_pad_mode(true, mode);
        for _ in 0..100_000 {
            let mut encoded = "ABCD"
                .repeat(prefix_quads_range.sample(&mut rng))
                .into_bytes();
            encoded.resize(encoded.len() + 1, PAD_BYTE);

            // amount of padding must be long enough to extend back from suffix into previous
            // quads
            let padding_len = rng.gen_range(1 + 1..encoded.len());
            // no non-padding after padding in this test, so padding goes to the end
            let padding_start = encoded.len() - padding_len;
            encoded[padding_start..].fill(PAD_BYTE);

            assert_eq!(Err(DecodeError::InvalidLength), engine.decode(&encoded),);
        }
    }
}

/// 0-1 bytes of data before any amount of padding in final chunk = invalid byte, since padding
/// is not valid data (consistent with error for pad bytes in earlier chunks).
/// From this we know there must be 2-3 bytes of data before padding
#[apply(all_engines)]
fn decode_too_little_data_before_padding_error_invalid_byte<E: EngineWrapper>(engine_wrapper: E) {
    let mut rng = seeded_rng();

    // want to test no prefix quad case, so start at 0
    let prefix_quads_range = distributions::Uniform::from(0_usize..256);
    let suffix_data_len_range = distributions::Uniform::from(0_usize..=1);
    for mode in all_pad_modes() {
        // we don't encode so we don't care about encode padding
        let engine = E::standard_with_pad_mode(true, mode);
        for _ in 0..100_000 {
            let suffix_data_len = suffix_data_len_range.sample(&mut rng);
            let prefix_quad_len = prefix_quads_range.sample(&mut rng);

            // ensure there is a suffix quad
            let min_padding = usize::from(suffix_data_len == 0);

            // for all possible padding lengths
            for padding_len in min_padding..=(4 - suffix_data_len) {
                let mut encoded = "ABCD".repeat(prefix_quad_len).into_bytes();
                encoded.resize(encoded.len() + suffix_data_len, b'A');
                encoded.resize(encoded.len() + padding_len, PAD_BYTE);

                if suffix_data_len + padding_len == 1 {
                    assert_eq!(Err(DecodeError::InvalidLength), engine.decode(&encoded),);
                } else {
                    assert_eq!(
                        Err(DecodeError::InvalidByte(
                            prefix_quad_len * 4 + suffix_data_len,
                            PAD_BYTE,
                        )),
                        engine.decode(&encoded),
                        "suffix data len {} pad len {}",
                        suffix_data_len,
                        padding_len
                    );
                }
            }
        }
    }
}

// https://eprint.iacr.org/2022/361.pdf table 2, test 1
#[apply(all_engines)]
fn decode_malleability_test_case_3_byte_suffix_valid<E: EngineWrapper>(engine_wrapper: E) {
    assert_eq!(
        b"Hello".as_slice(),
        &E::standard().decode("SGVsbG8=").unwrap()
    );
}

// https://eprint.iacr.org/2022/361.pdf table 2, test 2
#[apply(all_engines)]
fn decode_malleability_test_case_3_byte_suffix_invalid_trailing_symbol<E: EngineWrapper>(
    engine_wrapper: E,
) {
    assert_eq!(
        DecodeError::InvalidLastSymbol(6, 0x39),
        E::standard().decode("SGVsbG9=").unwrap_err()
    );
}

// https://eprint.iacr.org/2022/361.pdf table 2, test 3
#[apply(all_engines)]
fn decode_malleability_test_case_3_byte_suffix_no_padding<E: EngineWrapper>(engine_wrapper: E) {
    assert_eq!(
        DecodeError::InvalidPadding,
        E::standard().decode("SGVsbG9").unwrap_err()
    );
}

// https://eprint.iacr.org/2022/361.pdf table 2, test 4
#[apply(all_engines)]
fn decode_malleability_test_case_2_byte_suffix_valid_two_padding_symbols<E: EngineWrapper>(
    engine_wrapper: E,
) {
    assert_eq!(
        b"Hell".as_slice(),
        &E::standard().decode("SGVsbA==").unwrap()
    );
}

// https://eprint.iacr.org/2022/361.pdf table 2, test 5
#[apply(all_engines)]
fn decode_malleability_test_case_2_byte_suffix_short_padding<E: EngineWrapper>(engine_wrapper: E) {
    assert_eq!(
        DecodeError::InvalidPadding,
        E::standard().decode("SGVsbA=").unwrap_err()
    );
}

// https://eprint.iacr.org/2022/361.pdf table 2, test 6
#[apply(all_engines)]
fn decode_malleability_test_case_2_byte_suffix_no_padding<E: EngineWrapper>(engine_wrapper: E) {
    assert_eq!(
        DecodeError::InvalidPadding,
        E::standard().decode("SGVsbA").unwrap_err()
    );
}

// https://eprint.iacr.org/2022/361.pdf table 2, test 7
// DecoderReader pseudo-engine gets InvalidByte at 8 (extra padding) since it decodes the first
// two complete quads correctly.
#[apply(all_engines_except_decoder_reader)]
fn decode_malleability_test_case_2_byte_suffix_too_much_padding<E: EngineWrapper>(
    engine_wrapper: E,
) {
    assert_eq!(
        DecodeError::InvalidByte(6, PAD_BYTE),
        E::standard().decode("SGVsbA====").unwrap_err()
    );
}

/// Requires canonical padding -> accepts 2 + 2, 3 + 1, 4 + 0 final quad configurations
#[apply(all_engines)]
fn decode_pad_mode_requires_canonical_accepts_canonical<E: EngineWrapper>(engine_wrapper: E) {
    assert_all_suffixes_ok(
        E::standard_with_pad_mode(true, DecodePaddingMode::RequireCanonical),
        vec!["/w==", "iYU=", "AAAA"],
    );
}

/// Requires canonical padding -> rejects 2 + 0-1, 3 + 0 final chunk configurations
#[apply(all_engines)]
fn decode_pad_mode_requires_canonical_rejects_non_canonical<E: EngineWrapper>(engine_wrapper: E) {
    let engine = E::standard_with_pad_mode(true, DecodePaddingMode::RequireCanonical);

    let suffixes = ["/w", "/w=", "iYU"];
    for num_prefix_quads in 0..256 {
        for &suffix in suffixes.iter() {
            let mut encoded = "AAAA".repeat(num_prefix_quads);
            encoded.push_str(suffix);

            let res = engine.decode(&encoded);

            assert_eq!(Err(DecodeError::InvalidPadding), res);
        }
    }
}

/// Requires no padding -> accepts 2 + 0, 3 + 0, 4 + 0 final chunk configuration
#[apply(all_engines)]
fn decode_pad_mode_requires_no_padding_accepts_no_padding<E: EngineWrapper>(engine_wrapper: E) {
    assert_all_suffixes_ok(
        E::standard_with_pad_mode(true, DecodePaddingMode::RequireNone),
        vec!["/w", "iYU", "AAAA"],
    );
}

/// Requires no padding -> rejects 2 + 1-2, 3 + 1 final chunk configuration
#[apply(all_engines)]
fn decode_pad_mode_requires_no_padding_rejects_any_padding<E: EngineWrapper>(engine_wrapper: E) {
    let engine = E::standard_with_pad_mode(true, DecodePaddingMode::RequireNone);

    let suffixes = ["/w=", "/w==", "iYU="];
    for num_prefix_quads in 0..256 {
        for &suffix in suffixes.iter() {
            let mut encoded = "AAAA".repeat(num_prefix_quads);
            encoded.push_str(suffix);

            let res = engine.decode(&encoded);

            assert_eq!(Err(DecodeError::InvalidPadding), res);
        }
    }
}

/// Indifferent padding accepts 2 + 0-2, 3 + 0-1, 4 + 0 final chunk configuration
#[apply(all_engines)]
fn decode_pad_mode_indifferent_padding_accepts_anything<E: EngineWrapper>(engine_wrapper: E) {
    assert_all_suffixes_ok(
        E::standard_with_pad_mode(true, DecodePaddingMode::Indifferent),
        vec!["/w", "/w=", "/w==", "iYU", "iYU=", "AAAA"],
    );
}

//this is a MAY in the rfc: https://tools.ietf.org/html/rfc4648#section-3.3
// DecoderReader pseudo-engine finds the first padding, but doesn't report it as an error,
// because in the next decode it finds more padding, which is reported as InvalidByte, just
// with an offset at its position in the second decode, rather than being linked to the start
// of the padding that was first seen in the previous decode.
#[apply(all_engines_except_decoder_reader)]
fn decode_pad_byte_in_penultimate_quad_error<E: EngineWrapper>(engine_wrapper: E) {
    for mode in all_pad_modes() {
        // we don't encode so we don't care about encode padding
        let engine = E::standard_with_pad_mode(true, mode);

        for num_prefix_quads in 0..256 {
            // leave room for at least one pad byte in penultimate quad
            for num_valid_bytes_penultimate_quad in 0..4 {
                // can't have 1 or it would be invalid length
                for num_pad_bytes_in_final_quad in 2..=4 {
                    let mut s: String = "ABCD".repeat(num_prefix_quads);

                    // varying amounts of padding in the penultimate quad
                    for _ in 0..num_valid_bytes_penultimate_quad {
                        s.push('A');
                    }
                    // finish penultimate quad with padding
                    for _ in num_valid_bytes_penultimate_quad..4 {
                        s.push('=');
                    }
                    // and more padding in the final quad
                    for _ in 0..num_pad_bytes_in_final_quad {
                        s.push('=');
                    }

                    // padding should be an invalid byte before the final quad.
                    // Could argue that the *next* padding byte (in the next quad) is technically the first
                    // erroneous one, but reporting that accurately is more complex and probably nobody cares
                    assert_eq!(
                        DecodeError::InvalidByte(
                            num_prefix_quads * 4 + num_valid_bytes_penultimate_quad,
                            b'=',
                        ),
                        engine.decode(&s).unwrap_err(),
                    );
                }
            }
        }
    }
}

#[apply(all_engines)]
fn decode_bytes_after_padding_in_final_quad_error<E: EngineWrapper>(engine_wrapper: E) {
    for mode in all_pad_modes() {
        // we don't encode so we don't care about encode padding
        let engine = E::standard_with_pad_mode(true, mode);

        for num_prefix_quads in 0..256 {
            // leave at least one byte in the quad for padding
            for bytes_after_padding in 1..4 {
                let mut s: String = "ABCD".repeat(num_prefix_quads);

                // every invalid padding position with a 3-byte final quad: 1 to 3 bytes after padding
                for _ in 0..(3 - bytes_after_padding) {
                    s.push('A');
                }
                s.push('=');
                for _ in 0..bytes_after_padding {
                    s.push('A');
                }

                // First (and only) padding byte is invalid.
                assert_eq!(
                    DecodeError::InvalidByte(
                        num_prefix_quads * 4 + (3 - bytes_after_padding),
                        b'='
                    ),
                    engine.decode(&s).unwrap_err()
                );
            }
        }
    }
}

#[apply(all_engines)]
fn decode_absurd_pad_error<E: EngineWrapper>(engine_wrapper: E) {
    for mode in all_pad_modes() {
        // we don't encode so we don't care about encode padding
        let engine = E::standard_with_pad_mode(true, mode);

        for num_prefix_quads in 0..256 {
            let mut s: String = "ABCD".repeat(num_prefix_quads);
            s.push_str("==Y=Wx===pY=2U=====");

            // first padding byte
            assert_eq!(
                DecodeError::InvalidByte(num_prefix_quads * 4, b'='),
                engine.decode(&s).unwrap_err()
            );
        }
    }
}

// DecoderReader pseudo-engine detects InvalidByte instead of InvalidLength because it starts by
// decoding only the available complete quads
#[apply(all_engines_except_decoder_reader)]
fn decode_too_much_padding_returns_error<E: EngineWrapper>(engine_wrapper: E) {
    for mode in all_pad_modes() {
        // we don't encode so we don't care about encode padding
        let engine = E::standard_with_pad_mode(true, mode);

        for num_prefix_quads in 0..256 {
            // add enough padding to ensure that we'll hit all decode stages at the different lengths
            for pad_bytes in 1..=64 {
                let mut s: String = "ABCD".repeat(num_prefix_quads);
                let padding: String = "=".repeat(pad_bytes);
                s.push_str(&padding);

                if pad_bytes % 4 == 1 {
                    assert_eq!(DecodeError::InvalidLength, engine.decode(&s).unwrap_err());
                } else {
                    assert_eq!(
                        DecodeError::InvalidByte(num_prefix_quads * 4, b'='),
                        engine.decode(&s).unwrap_err()
                    );
                }
            }
        }
    }
}

// DecoderReader pseudo-engine detects InvalidByte instead of InvalidLength because it starts by
// decoding only the available complete quads
#[apply(all_engines_except_decoder_reader)]
fn decode_padding_followed_by_non_padding_returns_error<E: EngineWrapper>(engine_wrapper: E) {
    for mode in all_pad_modes() {
        // we don't encode so we don't care about encode padding
        let engine = E::standard_with_pad_mode(true, mode);

        for num_prefix_quads in 0..256 {
            for pad_bytes in 0..=32 {
                let mut s: String = "ABCD".repeat(num_prefix_quads);
                let padding: String = "=".repeat(pad_bytes);
                s.push_str(&padding);
                s.push('E');

                if pad_bytes % 4 == 0 {
                    assert_eq!(DecodeError::InvalidLength, engine.decode(&s).unwrap_err());
                } else {
                    assert_eq!(
                        DecodeError::InvalidByte(num_prefix_quads * 4, b'='),
                        engine.decode(&s).unwrap_err()
                    );
                }
            }
        }
    }
}

#[apply(all_engines)]
fn decode_one_char_in_final_quad_with_padding_error<E: EngineWrapper>(engine_wrapper: E) {
    for mode in all_pad_modes() {
        // we don't encode so we don't care about encode padding
        let engine = E::standard_with_pad_mode(true, mode);

        for num_prefix_quads in 0..256 {
            let mut s: String = "ABCD".repeat(num_prefix_quads);
            s.push_str("E=");

            assert_eq!(
                DecodeError::InvalidByte(num_prefix_quads * 4 + 1, b'='),
                engine.decode(&s).unwrap_err()
            );

            // more padding doesn't change the error
            s.push('=');
            assert_eq!(
                DecodeError::InvalidByte(num_prefix_quads * 4 + 1, b'='),
                engine.decode(&s).unwrap_err()
            );

            s.push('=');
            assert_eq!(
                DecodeError::InvalidByte(num_prefix_quads * 4 + 1, b'='),
                engine.decode(&s).unwrap_err()
            );
        }
    }
}

#[apply(all_engines)]
fn decode_too_few_symbols_in_final_quad_error<E: EngineWrapper>(engine_wrapper: E) {
    for mode in all_pad_modes() {
        // we don't encode so we don't care about encode padding
        let engine = E::standard_with_pad_mode(true, mode);

        for num_prefix_quads in 0..256 {
            // <2 is invalid
            for final_quad_symbols in 0..2 {
                for padding_symbols in 0..=(4 - final_quad_symbols) {
                    let mut s: String = "ABCD".repeat(num_prefix_quads);

                    for _ in 0..final_quad_symbols {
                        s.push('A');
                    }
                    for _ in 0..padding_symbols {
                        s.push('=');
                    }

                    match final_quad_symbols + padding_symbols {
                        0 => continue,
                        1 => {
                            assert_eq!(DecodeError::InvalidLength, engine.decode(&s).unwrap_err());
                        }
                        _ => {
                            // error reported at first padding byte
                            assert_eq!(
                                DecodeError::InvalidByte(
                                    num_prefix_quads * 4 + final_quad_symbols,
                                    b'=',
                                ),
                                engine.decode(&s).unwrap_err()
                            );
                        }
                    }
                }
            }
        }
    }
}

// DecoderReader pseudo-engine can't handle DecodePaddingMode::RequireNone since it will decode
// a complete quad with padding in it before encountering the stray byte that makes it an invalid
// length
#[apply(all_engines_except_decoder_reader)]
fn decode_invalid_trailing_bytes<E: EngineWrapper>(engine_wrapper: E) {
    for mode in all_pad_modes() {
        do_invalid_trailing_byte(E::standard_with_pad_mode(true, mode), mode);
    }
}

#[apply(all_engines)]
fn decode_invalid_trailing_bytes_all_modes<E: EngineWrapper>(engine_wrapper: E) {
    // excluding no padding mode because the DecoderWrapper pseudo-engine will fail with
    // InvalidPadding because it will decode the last complete quad with padding first
    for mode in pad_modes_allowing_padding() {
        do_invalid_trailing_byte(E::standard_with_pad_mode(true, mode), mode);
    }
}

#[apply(all_engines)]
fn decode_invalid_trailing_padding_as_invalid_length<E: EngineWrapper>(engine_wrapper: E) {
    // excluding no padding mode because the DecoderWrapper pseudo-engine will fail with
    // InvalidPadding because it will decode the last complete quad with padding first
    for mode in pad_modes_allowing_padding() {
        do_invalid_trailing_padding_as_invalid_length(E::standard_with_pad_mode(true, mode), mode);
    }
}

// DecoderReader pseudo-engine can't handle DecodePaddingMode::RequireNone since it will decode
// a complete quad with padding in it before encountering the stray byte that makes it an invalid
// length
#[apply(all_engines_except_decoder_reader)]
fn decode_invalid_trailing_padding_as_invalid_length_all_modes<E: EngineWrapper>(
    engine_wrapper: E,
) {
    for mode in all_pad_modes() {
        do_invalid_trailing_padding_as_invalid_length(E::standard_with_pad_mode(true, mode), mode);
    }
}

#[apply(all_engines)]
fn decode_wrong_length_error<E: EngineWrapper>(engine_wrapper: E) {
    let engine = E::standard_with_pad_mode(true, DecodePaddingMode::Indifferent);

    for num_prefix_quads in 0..256 {
        // at least one token, otherwise it wouldn't be a final quad
        for num_tokens_final_quad in 1..=4 {
            for num_padding in 0..=(4 - num_tokens_final_quad) {
                let mut s: String = "IIII".repeat(num_prefix_quads);
                for _ in 0..num_tokens_final_quad {
                    s.push('g');
                }
                for _ in 0..num_padding {
                    s.push('=');
                }

                let res = engine.decode(&s);
                if num_tokens_final_quad >= 2 {
                    assert!(res.is_ok());
                } else if num_tokens_final_quad == 1 && num_padding > 0 {
                    // = is invalid if it's too early
                    assert_eq!(
                        Err(DecodeError::InvalidByte(
                            num_prefix_quads * 4 + num_tokens_final_quad,
                            61
                        )),
                        res
                    );
                } else if num_padding > 2 {
                    assert_eq!(Err(DecodeError::InvalidPadding), res);
                } else {
                    assert_eq!(Err(DecodeError::InvalidLength), res);
                }
            }
        }
    }
}

#[apply(all_engines)]
fn decode_into_slice_fits_in_precisely_sized_slice<E: EngineWrapper>(engine_wrapper: E) {
    let mut orig_data = Vec::new();
    let mut encoded_data = String::new();
    let mut decode_buf = Vec::new();

    let input_len_range = distributions::Uniform::new(0, 1000);
    let mut rng = rngs::SmallRng::from_entropy();

    for _ in 0..10_000 {
        orig_data.clear();
        encoded_data.clear();
        decode_buf.clear();

        let input_len = input_len_range.sample(&mut rng);

        for _ in 0..input_len {
            orig_data.push(rng.gen());
        }

        let engine = E::random(&mut rng);
        engine.encode_string(&orig_data, &mut encoded_data);
        assert_encode_sanity(&encoded_data, engine.config().encode_padding(), input_len);

        decode_buf.resize(input_len, 0);

        // decode into the non-empty buf
        let decode_bytes_written = engine
            .decode_slice_unchecked(encoded_data.as_bytes(), &mut decode_buf[..])
            .unwrap();

        assert_eq!(orig_data.len(), decode_bytes_written);
        assert_eq!(orig_data, decode_buf);
    }
}

#[apply(all_engines)]
fn inner_decode_reports_padding_position<E: EngineWrapper>(engine_wrapper: E) {
    let mut b64 = String::new();
    let mut decoded = Vec::new();
    let engine = E::standard();

    for pad_position in 1..10_000 {
        b64.clear();
        decoded.clear();
        // plenty of room for original data
        decoded.resize(pad_position, 0);

        for _ in 0..pad_position {
            b64.push('A');
        }
        // finish the quad with padding
        for _ in 0..(4 - (pad_position % 4)) {
            b64.push('=');
        }

        let decode_res = engine.internal_decode(
            b64.as_bytes(),
            &mut decoded[..],
            engine.internal_decoded_len_estimate(b64.len()),
        );
        if pad_position % 4 < 2 {
            // impossible padding
            assert_eq!(
                Err(DecodeError::InvalidByte(pad_position, PAD_BYTE)),
                decode_res
            );
        } else {
            let decoded_bytes = pad_position / 4 * 3
                + match pad_position % 4 {
                    0 => 0,
                    2 => 1,
                    3 => 2,
                    _ => unreachable!(),
                };
            assert_eq!(
                Ok(DecodeMetadata::new(decoded_bytes, Some(pad_position))),
                decode_res
            );
        }
    }
}

#[apply(all_engines)]
fn decode_length_estimate_delta<E: EngineWrapper>(engine_wrapper: E) {
    for engine in [E::standard(), E::standard_unpadded()] {
        for &padding in &[true, false] {
            for orig_len in 0..1000 {
                let encoded_len = encoded_len(orig_len, padding).unwrap();

                let decoded_estimate = engine
                    .internal_decoded_len_estimate(encoded_len)
                    .decoded_len_estimate();
                assert!(decoded_estimate >= orig_len);
                assert!(
                    decoded_estimate - orig_len < 3,
                    "estimate: {}, encoded: {}, orig: {}",
                    decoded_estimate,
                    encoded_len,
                    orig_len
                );
            }
        }
    }
}

#[apply(all_engines)]
fn estimate_via_u128_inflation<E: EngineWrapper>(engine_wrapper: E) {
    // cover both ends of usize
    (0..1000)
        .chain(usize::MAX - 1000..=usize::MAX)
        .for_each(|encoded_len| {
            // inflate to 128 bit type to be able to safely use the easy formulas
            let len_128 = encoded_len as u128;

            let estimate = E::standard()
                .internal_decoded_len_estimate(encoded_len)
                .decoded_len_estimate();

            // This check is a little too strict: it requires using the (len + 3) / 4 * 3 formula
            // or equivalent, but until other engines come along that use a different formula
            // requiring that we think more carefully about what the allowable criteria are, this
            // will do.
            assert_eq!(
                ((len_128 + 3) / 4 * 3) as usize,
                estimate,
                "enc len {}",
                encoded_len
            );
        })
}

fn do_invalid_trailing_byte(engine: impl Engine, mode: DecodePaddingMode) {
    for num_prefix_quads in 0..256 {
        let mut s: String = "ABCD".repeat(num_prefix_quads);
        s.push_str("Cg==\n");

        // The case of trailing newlines is common enough to warrant a test for a good error
        // message.
        assert_eq!(
            Err(DecodeError::InvalidByte(num_prefix_quads * 4 + 4, b'\n')),
            engine.decode(&s),
            "mode: {:?}, input: {}",
            mode,
            s
        );
    }
}

fn do_invalid_trailing_padding_as_invalid_length(engine: impl Engine, mode: DecodePaddingMode) {
    for num_prefix_quads in 0..256 {
        let mut s: String = "ABCD".repeat(num_prefix_quads);
        s.push_str("Cg===");

        assert_eq!(
            Err(DecodeError::InvalidLength),
            engine.decode(&s),
            "mode: {:?}, input: {}",
            mode,
            s
        );
    }
}

/// Returns a tuple of the original data length, the encoded data length (just data), and the length including padding.
///
/// Vecs provided should be empty.
fn generate_random_encoded_data<E: Engine, R: rand::Rng, D: distributions::Distribution<usize>>(
    engine: &E,
    orig_data: &mut Vec<u8>,
    encode_buf: &mut Vec<u8>,
    rng: &mut R,
    length_distribution: &D,
) -> (usize, usize, usize) {
    let padding: bool = engine.config().encode_padding();

    let orig_len = fill_rand(orig_data, rng, length_distribution);
    let expected_encoded_len = encoded_len(orig_len, padding).unwrap();
    encode_buf.resize(expected_encoded_len, 0);

    let base_encoded_len = engine.internal_encode(&orig_data[..], &mut encode_buf[..]);

    let enc_len_with_padding = if padding {
        base_encoded_len + add_padding(base_encoded_len, &mut encode_buf[base_encoded_len..])
    } else {
        base_encoded_len
    };

    assert_eq!(expected_encoded_len, enc_len_with_padding);

    (orig_len, base_encoded_len, enc_len_with_padding)
}

// fill to a random length
fn fill_rand<R: rand::Rng, D: distributions::Distribution<usize>>(
    vec: &mut Vec<u8>,
    rng: &mut R,
    length_distribution: &D,
) -> usize {
    let len = length_distribution.sample(rng);
    for _ in 0..len {
        vec.push(rng.gen());
    }

    len
}

fn fill_rand_len<R: rand::Rng>(vec: &mut Vec<u8>, rng: &mut R, len: usize) {
    for _ in 0..len {
        vec.push(rng.gen());
    }
}

fn prefixed_data<'i>(input_with_prefix: &'i mut String, prefix_len: usize, data: &str) -> &'i str {
    input_with_prefix.truncate(prefix_len);
    input_with_prefix.push_str(data);
    input_with_prefix.as_str()
}

/// A wrapper to make using engines in rstest fixtures easier.
/// The functions don't need to be instance methods, but rstest does seem
/// to want an instance, so instances are passed to test functions and then ignored.
trait EngineWrapper {
    type Engine: Engine;

    /// Return an engine configured for RFC standard base64
    fn standard() -> Self::Engine;

    /// Return an engine configured for RFC standard base64, except with no padding appended on
    /// encode, and required no padding on decode.
    fn standard_unpadded() -> Self::Engine;

    /// Return an engine configured for RFC standard alphabet with the provided encode and decode
    /// pad settings
    fn standard_with_pad_mode(encode_pad: bool, decode_pad_mode: DecodePaddingMode)
        -> Self::Engine;

    /// Return an engine configured for RFC standard base64 that allows invalid trailing bits
    fn standard_allow_trailing_bits() -> Self::Engine;

    /// Return an engine configured with a randomized alphabet and config
    fn random<R: rand::Rng>(rng: &mut R) -> Self::Engine;

    /// Return an engine configured with the specified alphabet and randomized config
    fn random_alphabet<R: rand::Rng>(rng: &mut R, alphabet: &Alphabet) -> Self::Engine;
}

struct GeneralPurposeWrapper {}

impl EngineWrapper for GeneralPurposeWrapper {
    type Engine = general_purpose::GeneralPurpose;

    fn standard() -> Self::Engine {
        general_purpose::GeneralPurpose::new(&STANDARD, general_purpose::PAD)
    }

    fn standard_unpadded() -> Self::Engine {
        general_purpose::GeneralPurpose::new(&STANDARD, general_purpose::NO_PAD)
    }

    fn standard_with_pad_mode(
        encode_pad: bool,
        decode_pad_mode: DecodePaddingMode,
    ) -> Self::Engine {
        general_purpose::GeneralPurpose::new(
            &STANDARD,
            general_purpose::GeneralPurposeConfig::new()
                .with_encode_padding(encode_pad)
                .with_decode_padding_mode(decode_pad_mode),
        )
    }

    fn standard_allow_trailing_bits() -> Self::Engine {
        general_purpose::GeneralPurpose::new(
            &STANDARD,
            general_purpose::GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true),
        )
    }

    fn random<R: rand::Rng>(rng: &mut R) -> Self::Engine {
        let alphabet = random_alphabet(rng);

        Self::random_alphabet(rng, alphabet)
    }

    fn random_alphabet<R: rand::Rng>(rng: &mut R, alphabet: &Alphabet) -> Self::Engine {
        general_purpose::GeneralPurpose::new(alphabet, random_config(rng))
    }
}

struct NaiveWrapper {}

impl EngineWrapper for NaiveWrapper {
    type Engine = naive::Naive;

    fn standard() -> Self::Engine {
        naive::Naive::new(
            &STANDARD,
            naive::NaiveConfig {
                encode_padding: true,
                decode_allow_trailing_bits: false,
                decode_padding_mode: DecodePaddingMode::RequireCanonical,
            },
        )
    }

    fn standard_unpadded() -> Self::Engine {
        naive::Naive::new(
            &STANDARD,
            naive::NaiveConfig {
                encode_padding: false,
                decode_allow_trailing_bits: false,
                decode_padding_mode: DecodePaddingMode::RequireNone,
            },
        )
    }

    fn standard_with_pad_mode(
        encode_pad: bool,
        decode_pad_mode: DecodePaddingMode,
    ) -> Self::Engine {
        naive::Naive::new(
            &STANDARD,
            naive::NaiveConfig {
                encode_padding: false,
                decode_allow_trailing_bits: false,
                decode_padding_mode: decode_pad_mode,
            },
        )
    }

    fn standard_allow_trailing_bits() -> Self::Engine {
        naive::Naive::new(
            &STANDARD,
            naive::NaiveConfig {
                encode_padding: true,
                decode_allow_trailing_bits: true,
                decode_padding_mode: DecodePaddingMode::RequireCanonical,
            },
        )
    }

    fn random<R: rand::Rng>(rng: &mut R) -> Self::Engine {
        let alphabet = random_alphabet(rng);

        Self::random_alphabet(rng, alphabet)
    }

    fn random_alphabet<R: rand::Rng>(rng: &mut R, alphabet: &Alphabet) -> Self::Engine {
        let mode = rng.gen();

        let config = naive::NaiveConfig {
            encode_padding: match mode {
                DecodePaddingMode::Indifferent => rng.gen(),
                DecodePaddingMode::RequireCanonical => true,
                DecodePaddingMode::RequireNone => false,
            },
            decode_allow_trailing_bits: rng.gen(),
            decode_padding_mode: mode,
        };

        naive::Naive::new(alphabet, config)
    }
}

/// A pseudo-Engine that routes all decoding through [DecoderReader]
struct DecoderReaderEngine<E: Engine> {
    engine: E,
}

impl<E: Engine> From<E> for DecoderReaderEngine<E> {
    fn from(value: E) -> Self {
        Self { engine: value }
    }
}

impl<E: Engine> Engine for DecoderReaderEngine<E> {
    type Config = E::Config;
    type DecodeEstimate = E::DecodeEstimate;

    fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
        self.engine.internal_encode(input, output)
    }

    fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
        self.engine.internal_decoded_len_estimate(input_len)
    }

    fn internal_decode(
        &self,
        input: &[u8],
        output: &mut [u8],
        decode_estimate: Self::DecodeEstimate,
    ) -> Result<DecodeMetadata, DecodeError> {
        let mut reader = DecoderReader::new(input, &self.engine);
        let mut buf = vec![0; input.len()];
        // to avoid effects like not detecting invalid length due to progressively growing
        // the output buffer in read_to_end etc, read into a big enough buffer in one go
        // to make behavior more consistent with normal engines
        let _ = reader
            .read(&mut buf)
            .and_then(|len| {
                buf.truncate(len);
                // make sure we got everything
                reader.read_to_end(&mut buf)
            })
            .map_err(|io_error| {
                *io_error
                    .into_inner()
                    .and_then(|inner| inner.downcast::<DecodeError>().ok())
                    .unwrap()
            })?;
        output[..buf.len()].copy_from_slice(&buf);
        Ok(DecodeMetadata::new(
            buf.len(),
            input
                .iter()
                .enumerate()
                .filter(|(_offset, byte)| **byte == PAD_BYTE)
                .map(|(offset, _byte)| offset)
                .next(),
        ))
    }

    fn config(&self) -> &Self::Config {
        self.engine.config()
    }
}

struct DecoderReaderEngineWrapper {}

impl EngineWrapper for DecoderReaderEngineWrapper {
    type Engine = DecoderReaderEngine<general_purpose::GeneralPurpose>;

    fn standard() -> Self::Engine {
        GeneralPurposeWrapper::standard().into()
    }

    fn standard_unpadded() -> Self::Engine {
        GeneralPurposeWrapper::standard_unpadded().into()
    }

    fn standard_with_pad_mode(
        encode_pad: bool,
        decode_pad_mode: DecodePaddingMode,
    ) -> Self::Engine {
        GeneralPurposeWrapper::standard_with_pad_mode(encode_pad, decode_pad_mode).into()
    }

    fn standard_allow_trailing_bits() -> Self::Engine {
        GeneralPurposeWrapper::standard_allow_trailing_bits().into()
    }

    fn random<R: rand::Rng>(rng: &mut R) -> Self::Engine {
        GeneralPurposeWrapper::random(rng).into()
    }

    fn random_alphabet<R: rand::Rng>(rng: &mut R, alphabet: &Alphabet) -> Self::Engine {
        GeneralPurposeWrapper::random_alphabet(rng, alphabet).into()
    }
}

fn seeded_rng() -> impl rand::Rng {
    rngs::SmallRng::from_entropy()
}

fn all_pad_modes() -> Vec<DecodePaddingMode> {
    vec![
        DecodePaddingMode::Indifferent,
        DecodePaddingMode::RequireCanonical,
        DecodePaddingMode::RequireNone,
    ]
}

fn pad_modes_allowing_padding() -> Vec<DecodePaddingMode> {
    vec![
        DecodePaddingMode::Indifferent,
        DecodePaddingMode::RequireCanonical,
    ]
}

fn assert_all_suffixes_ok<E: Engine>(engine: E, suffixes: Vec<&str>) {
    for num_prefix_quads in 0..256 {
        for &suffix in suffixes.iter() {
            let mut encoded = "AAAA".repeat(num_prefix_quads);
            encoded.push_str(suffix);

            let res = &engine.decode(&encoded);
            assert!(res.is_ok());
        }
    }
}
