use postcard::accumulator::{CobsAccumulator, FeedResult};
use serde::{Deserialize, Serialize};
use std::io::Read;

// Read a "huge" serialized struct in 32 byte chunks into a 256 byte buffer and deserialize it.
#[test]
fn reader() {
    let mut raw_buf = [0u8; 32];
    let mut input_buf = [0u8; 256];
    let mut cobs_buf: CobsAccumulator<256> = CobsAccumulator::new();

    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct Huge {
        a: u32,
        b: [u32; 32],
        d: u64,
    }

    let expected = Huge {
        a: 0xabcdef00,
        b: [0x01234567; 32],
        d: u64::MAX,
    };

    let input = postcard::to_slice_cobs(&expected, &mut input_buf).unwrap();
    // TODO(https://github.com/rust-lang/rust-clippy/issues/12751): Remove once fixed.
    #[allow(clippy::redundant_slicing)]
    let mut input = &input[..];

    // Magic number from serializing struct and printing length
    assert_eq!(input.len(), 145);

    let mut output = None;

    while let Ok(ct) = input.read(&mut raw_buf) {
        // Finished reading input
        if ct == 0 {
            break;
        }

        let buf = &raw_buf[..ct];
        let mut window = buf;

        'cobs: while !window.is_empty() {
            window = match cobs_buf.feed::<Huge>(window) {
                FeedResult::Consumed => break 'cobs,
                FeedResult::OverFull(new_wind) => new_wind,
                FeedResult::DeserError(new_wind) => new_wind,
                FeedResult::Success { data, remaining } => {
                    output = Some(data);

                    remaining
                }
            };
        }
    }

    assert_eq!(output.unwrap(), expected);
}
