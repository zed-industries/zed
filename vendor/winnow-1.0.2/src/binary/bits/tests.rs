use super::*;
use crate::error::ErrMode;
use crate::error::InputError;
use crate::prelude::*;
use crate::stream::{Offset, Stream};
use crate::Partial;
#[cfg(feature = "std")]
use proptest::prelude::*;

#[test]
/// Take the `bits` function and assert that remaining bytes are correctly returned, if the
/// previous bytes are fully consumed
fn test_complete_byte_consumption_bits() {
    let input = &[0x12, 0x34, 0x56, 0x78][..];

    // Take 3 bit slices with sizes [4, 8, 4].
    #[allow(clippy::type_complexity)]
    let result: ModalResult<(&[u8], (u8, u8, u8)), InputError<_>> =
        bits::<_, _, ErrMode<InputError<Bits<&[u8]>>>, _, _>((
            take(4usize),
            take(8usize),
            take(4usize),
        ))
        .parse_peek(input);

    let output = result.expect("We take 2 bytes and the input is longer than 2 bytes");

    let remaining = output.0;
    assert_eq!(remaining, [0x56, 0x78]);

    let parsed = output.1;
    assert_eq!(parsed.0, 0x01);
    assert_eq!(parsed.1, 0x23);
    assert_eq!(parsed.2, 0x04);
}

#[test]
/// Take the `bits` function and assert that remaining bytes are correctly returned, if the
/// previous bytes are NOT fully consumed. Partially consumed bytes are supposed to be dropped.
/// I.e. if we consume 1.5 bytes of 4 bytes, 2 bytes will be returned, bits 13-16 will be
/// dropped.
fn test_partial_byte_consumption_bits() {
    let input = &[0x12, 0x34, 0x56, 0x78][..];

    // Take bit slices with sizes [4, 8].
    let result: ModalResult<(&[u8], (u8, u8)), InputError<_>> =
        bits::<_, _, ErrMode<InputError<Bits<&[u8]>>>, _, _>((take(4usize), take(8usize)))
            .parse_peek(input);

    let output = result.expect("We take 1.5 bytes and the input is longer than 2 bytes");

    let remaining = output.0;
    assert_eq!(remaining, [0x56, 0x78]);

    let parsed = output.1;
    assert_eq!(parsed.0, 0x01);
    assert_eq!(parsed.1, 0x23);
}

#[test]
#[cfg(feature = "std")]
/// Ensure that in Incomplete error is thrown, if too few bytes are passed for a given parser.
fn test_incomplete_bits() {
    let input = Partial::new(&[0x12][..]);

    // Take bit slices with sizes [4, 8].
    let result: ModalResult<(_, (u8, u8)), InputError<_>> =
        bits::<_, _, ErrMode<InputError<Bits<_>>>, _, _>((take(4usize), take(8usize)))
            .parse_peek(input);

    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!("Parsing requires 2 more data", error.to_string());
}

#[test]
fn test_take_complete_0() {
    let input = &[0b00010010][..];
    let count = 0usize;
    assert_eq!(count, 0usize);
    let offset = 0usize;

    let result: ModalResult<(Bits<&[u8]>, usize), InputError<_>> =
        take(count).parse_peek(Bits(input, offset));

    assert_eq!(result, Ok((Bits(input, offset), 0)));
}

#[test]
fn test_take_complete_eof() {
    let input = &[0b00010010][..];

    let result: ModalResult<(Bits<&[u8]>, usize), InputError<_>> =
        take(1usize).parse_peek(Bits(input, 8));

    assert_eq!(
        result,
        Err(crate::error::ErrMode::Backtrack(InputError::at(Bits(
            input, 8
        ),)))
    );
}

#[test]
fn test_take_complete_span_over_multiple_bytes() {
    let input = &[0b00010010, 0b00110100, 0b11111111, 0b11111111][..];

    let result: ModalResult<(Bits<&[u8]>, usize), InputError<_>> =
        take(24usize).parse_peek(Bits(input, 4));

    assert_eq!(
        result,
        Ok((Bits([0b11111111].as_ref(), 4), 0b1000110100111111111111))
    );
}

#[test]
fn test_take_partial_0() {
    let input = Partial::new(&[][..]);
    let count = 0usize;
    assert_eq!(count, 0usize);
    let offset = 0usize;

    let result: ModalResult<(Bits<_>, usize), InputError<_>> =
        take(count).parse_peek(Bits(input, offset));

    assert_eq!(result, Ok((Bits(input, offset), 0)));
}

#[test]
fn test_pattern_partial_ok() {
    let input = Partial::new(&[0b00011111][..]);
    let offset = 0usize;
    let bits_to_take = 4usize;
    let value_to_pattern = 0b0001;

    let result: ModalResult<(Bits<_>, usize), InputError<_>> =
        pattern(value_to_pattern, bits_to_take).parse_peek(Bits(input, offset));

    assert_eq!(result, Ok((Bits(input, bits_to_take), value_to_pattern)));
}

#[test]
fn test_pattern_partial_err() {
    let input = Partial::new(&[0b00011111][..]);
    let offset = 0usize;
    let bits_to_take = 4usize;
    let value_to_pattern = 0b1111;

    let result: ModalResult<(Bits<_>, usize), InputError<_>> =
        pattern(value_to_pattern, bits_to_take).parse_peek(Bits(input, offset));

    assert_eq!(
        result,
        Err(crate::error::ErrMode::Backtrack(InputError::at(Bits(
            input, offset
        ),)))
    );
}

#[test]
fn test_bool_0_complete() {
    let input = [0b10000000].as_ref();

    let result: ModalResult<(Bits<&[u8]>, bool), InputError<_>> = bool.parse_peek(Bits(input, 0));

    assert_eq!(result, Ok((Bits(input, 1), true)));
}

#[test]
fn test_bool_eof_complete() {
    let input = [0b10000000].as_ref();

    let result: ModalResult<(Bits<&[u8]>, bool), InputError<_>> = bool.parse_peek(Bits(input, 8));

    assert_eq!(
        result,
        Err(crate::error::ErrMode::Backtrack(InputError::at(Bits(
            input, 8
        ),)))
    );
}

#[test]
fn test_bool_0_partial() {
    let input = Partial::new([0b10000000].as_ref());

    #[allow(clippy::type_complexity)]
    let result: ModalResult<(Bits<Partial<&[u8]>>, bool), InputError<_>> =
        bool.parse_peek(Bits(input, 0));

    assert_eq!(result, Ok((Bits(input, 1), true)));
}

#[test]
fn test_bool_eof_partial() {
    let input = Partial::new([0b10000000].as_ref());

    #[allow(clippy::type_complexity)]
    let result: ModalResult<(Bits<Partial<&[u8]>>, bool), InputError<_>> =
        bool.parse_peek(Bits(input, 8));

    assert_eq!(
        result,
        Err(crate::error::ErrMode::Incomplete(Needed::new(1)))
    );
}

#[test]
#[cfg(feature = "alloc")]
fn test_bit_stream_empty() {
    let i = Bits(&b""[..], 0);

    let actual = i.iter_offsets().collect::<alloc::vec::Vec<_>>();
    assert_eq!(actual, vec![]);

    let actual = i.eof_offset();
    assert_eq!(actual, 0);

    let actual = i.peek_token();
    assert_eq!(actual, None);

    let actual = i.offset_for(|b| b);
    assert_eq!(actual, None);

    let actual = i.offset_at(1);
    assert_eq!(actual, Err(Needed::new(1)));

    let actual_slice = i.peek_slice(0);
    assert_eq!(actual_slice, (&b""[..], 0, 0));
}

#[test]
#[cfg(feature = "alloc")]
fn test_bit_offset_empty() {
    let i = Bits(&b""[..], 0);

    let actual = i.offset_from(&i);
    assert_eq!(actual, 0);
}

#[cfg(feature = "std")]
proptest! {
  #[test]
  #[cfg_attr(miri, ignore)]  // See https://github.com/AltSysrq/proptest/issues/253
  fn bit_stream(byte_len in 0..20usize, start in 0..160usize) {
        bit_stream_inner(byte_len, start);
  }
}

#[cfg(feature = "std")]
fn bit_stream_inner(byte_len: usize, start: usize) {
    let start = start.min(byte_len * 8);
    let start_byte = start / 8;
    let start_bit = start % 8;

    let bytes = vec![0b1010_1010; byte_len];
    let i = Bits(&bytes[start_byte..], start_bit);

    let mut curr_i = i;
    let mut curr_offset = 0;
    while let Some(_token) = curr_i.peek_token() {
        let to_offset = curr_i.offset_from(&i);
        assert_eq!(curr_offset, to_offset);

        let actual_slice = i.peek_slice(curr_offset);
        let expected_slice = i.peek_slice(curr_offset);
        assert_eq!(actual_slice, expected_slice);

        let at_offset = i.offset_at(curr_offset).unwrap();
        assert_eq!(curr_offset, at_offset);

        let eof_offset = curr_i.eof_offset();
        let eof_slice = curr_i.peek_slice(eof_offset);
        let eof_slice_i = Bits(eof_slice.0, eof_slice.1);
        assert_eq!(eof_slice_i, curr_i);

        curr_offset += 1;
        let _ = curr_i.next_token();
    }
    assert_eq!(i.eof_offset(), curr_offset);
}
