use super::{copy_encode, decode_all, encode_all};
use super::{Decoder, Encoder};

use partial_io::{PartialOp, PartialWrite};

use std::io;
use std::iter;

#[test]
fn test_end_of_frame() {
    use std::io::{Read, Write};

    let mut enc = Encoder::new(Vec::new(), 1).unwrap();
    enc.write_all(b"foo").unwrap();
    let mut compressed = enc.finish().unwrap();

    // Add footer/whatever to underlying storage.
    compressed.push(0);

    // Drain zstd stream until end-of-frame.
    let mut dec = Decoder::new(&compressed[..]).unwrap().single_frame();
    let mut buf = Vec::new();
    dec.read_to_end(&mut buf).unwrap();
    assert_eq!(&buf, b"foo", "Error decoding a single frame.");
}

#[test]
fn test_concatenated_frames() {
    let mut buffer = Vec::new();
    copy_encode(&b"foo"[..], &mut buffer, 1).unwrap();
    copy_encode(&b"bar"[..], &mut buffer, 2).unwrap();
    copy_encode(&b"baz"[..], &mut buffer, 3).unwrap();

    assert_eq!(
        &decode_all(&buffer[..]).unwrap(),
        b"foobarbaz",
        "Error decoding concatenated frames."
    );
}

#[test]
fn test_flush() {
    use std::io::Write;

    let buf = Vec::new();
    let mut z = Encoder::new(buf, 19).unwrap();

    z.write_all(b"hello").unwrap();

    z.flush().unwrap(); // Might corrupt stream
    let buf = z.finish().unwrap();

    let s = decode_all(&buf[..]).unwrap();
    assert_eq!(s, b"hello", "Error decoding after flush.");
}

#[test]
fn test_try_finish() {
    use std::io::Write;
    let mut z = setup_try_finish();

    z.get_mut().set_ops(iter::repeat(PartialOp::Unlimited));

    // flush() should continue to work even though write() doesn't.
    z.flush().unwrap();

    let buf = match z.try_finish() {
        Ok(buf) => buf.into_inner(),
        Err((_z, e)) => panic!("try_finish failed with {:?}", e),
    };

    // Make sure the multiple try_finish calls didn't screw up the internal
    // buffer and continued to produce valid compressed data.
    assert_eq!(&decode_all(&buf[..]).unwrap(), b"hello", "Error decoding");
}

#[test]
#[should_panic]
fn test_write_after_try_finish() {
    use std::io::Write;
    let mut z = setup_try_finish();
    z.write_all(b"hello world").unwrap();
}

fn setup_try_finish() -> Encoder<'static, PartialWrite<Vec<u8>>> {
    use std::io::Write;

    let buf =
        PartialWrite::new(Vec::new(), iter::repeat(PartialOp::Unlimited));
    let mut z = Encoder::new(buf, 19).unwrap();

    z.write_all(b"hello").unwrap();

    z.get_mut()
        .set_ops(iter::repeat(PartialOp::Err(io::ErrorKind::WouldBlock)));

    let (z, err) = z.try_finish().unwrap_err();
    assert_eq!(
        err.kind(),
        io::ErrorKind::WouldBlock,
        "expected WouldBlock error"
    );

    z
}

#[test]
fn test_failing_write() {
    use std::io::Write;

    let buf = PartialWrite::new(
        Vec::new(),
        iter::repeat(PartialOp::Err(io::ErrorKind::WouldBlock)),
    );
    let mut z = Encoder::new(buf, 1).unwrap();

    // Fill in enough data to make sure the buffer gets written out.
    let input = vec![b'b'; 128 * 1024];
    // This should work even though the inner writer rejects writes.
    assert_eq!(
        z.write(&input).unwrap(),
        128 * 1024,
        "did not write all input buffer"
    );

    // The next write would fail (the buffer still has some data in it).
    assert_eq!(
        z.write(b"abc").unwrap_err().kind(),
        io::ErrorKind::WouldBlock,
        "expected WouldBlock error"
    );

    z.get_mut().set_ops(iter::repeat(PartialOp::Unlimited));

    // This shouldn't have led to any corruption.
    let buf = z.finish().unwrap().into_inner();
    assert_eq!(
        &decode_all(&buf[..]).unwrap(),
        &input,
        "WouldBlock errors should not corrupt stream"
    );
}

#[test]
fn test_invalid_frame() {
    use std::io::Read;

    // I really hope this data is invalid...
    let data = &[1u8, 2u8, 3u8, 4u8, 5u8];
    let mut dec = Decoder::new(&data[..]).unwrap();
    assert_eq!(
        dec.read_to_end(&mut Vec::new()).err().map(|e| e.kind()),
        Some(io::ErrorKind::Other),
        "did not encounter expected 'invalid frame' error"
    );
}

#[test]
fn test_incomplete_frame() {
    use std::io::{Read, Write};

    let mut enc = Encoder::new(Vec::new(), 1).unwrap();
    enc.write_all(b"This is a regular string").unwrap();
    let mut compressed = enc.finish().unwrap();

    let half_size = compressed.len() - 2;
    compressed.truncate(half_size);

    let mut dec = Decoder::new(&compressed[..]).unwrap();
    assert_eq!(
        dec.read_to_end(&mut Vec::new()).err().map(|e| e.kind()),
        Some(io::ErrorKind::UnexpectedEof),
        "did not encounter expected EOF error"
    );
}

#[test]
fn test_cli_compatibility() {
    let input = include_bytes!("../../assets/example.txt.zst");

    let output = decode_all(&input[..]).unwrap();

    let expected = include_bytes!("../../assets/example.txt");

    assert_eq!(
        &output[..],
        &expected[..],
        "error decoding cli-compressed data"
    );
}

#[cfg(feature = "legacy")]
#[test]
fn test_legacy() {
    use std::fs;
    use std::io::Read;

    // Read the content from that file
    let expected = include_bytes!("../../assets/example.txt");

    for version in &[5, 6, 7, 8] {
        let filename = format!("assets/example.txt.v{}.zst", version);
        let file = fs::File::open(filename).unwrap();
        let mut decoder = Decoder::new(file).unwrap();

        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer).unwrap();

        assert_eq!(
            &expected[..],
            &buffer[..],
            "error decompressing legacy version {}",
            version
        );
    }
}

// Check that compressing+decompressing some data gives back the original
fn test_full_cycle(input: &[u8], level: i32) {
    crate::test_cycle_unwrap(
        input,
        |data| encode_all(data, level),
        |data| decode_all(data),
    );
}

#[test]
fn test_empty() {
    // Test compressing empty data
    for level in 1..19 {
        test_full_cycle(b"", level);
    }
}

#[test]
fn test_ll_source() {
    // Where could I find some long text?...
    let data = include_bytes!("../../zstd-safe/zstd-sys/src/bindings_zstd.rs");
    // Test a few compression levels.
    // TODO: check them all?
    for level in 1..5 {
        // Test compressing actual data
        test_full_cycle(data, level);
    }
}

#[test]
fn reader_to_writer() {
    use std::io::{Read, Write};

    let clear = include_bytes!("../../assets/example.txt");
    // Compress using reader
    let mut encoder = super::read::Encoder::new(&clear[..], 1).unwrap();

    let mut compressed_buffer = Vec::new();
    encoder.read_to_end(&mut compressed_buffer).unwrap();

    // eprintln!("Compressed Buffer: {:?}", compressed_buffer);

    // Decompress using writer
    let mut decompressed_buffer = Vec::new();
    let mut decoder =
        super::write::Decoder::new(&mut decompressed_buffer).unwrap();
    decoder.write_all(&compressed_buffer[..]).unwrap();
    decoder.flush().unwrap();
    // eprintln!("{:?}", decompressed_buffer);

    assert_eq!(clear, &decompressed_buffer[..]);
}
