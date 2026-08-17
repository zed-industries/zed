use crate::stream::read::{Decoder, Encoder};
use std::io::Read;

#[test]
fn test_error_handling() {
    let invalid_input = b"Abcdefghabcdefgh";

    let mut decoder = Decoder::new(&invalid_input[..]).unwrap();
    let output = decoder.read_to_end(&mut Vec::new());

    assert_eq!(output.is_err(), true);
}

#[test]
fn test_cycle() {
    let input = b"Abcdefghabcdefgh";

    let mut encoder = Encoder::new(&input[..], 1).unwrap();
    let mut buffer = Vec::new();
    encoder.read_to_end(&mut buffer).unwrap();

    let mut decoder = Decoder::new(&buffer[..]).unwrap();
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer).unwrap();

    assert_eq!(input, &buffer[..]);
}
