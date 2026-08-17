#[allow(unused)]
use futures::{executor::block_on, io::AsyncReadExt};

#[macro_use]
mod utils;

test_cases!(xz);

#[cfg(feature = "xz-parallel")]
test_cases!(xz_parallel);

#[allow(unused)]
use utils::{algos::xz::sync, InputStream};

#[cfg(feature = "futures-io")]
use utils::algos::xz::futures::{bufread, read};

#[test]
#[ntest::timeout(1000)]
#[cfg(feature = "futures-io")]
fn bufread_multiple_members_with_padding() {
    let compressed = [
        sync::compress(&[1, 2, 3, 4, 5, 6]),
        vec![0, 0, 0, 0],
        sync::compress(&[6, 5, 4, 3, 2, 1]),
        vec![0, 0, 0, 0],
    ]
    .join(&[][..]);

    let input = InputStream::from(vec![compressed]);

    let mut decoder = bufread::Decoder::new(bufread::from(&input));
    decoder.multiple_members(true);
    let output = read::to_vec(decoder);

    assert_eq!(output, &[1, 2, 3, 4, 5, 6, 6, 5, 4, 3, 2, 1][..]);
}

#[test]
#[ntest::timeout(1000)]
#[cfg(feature = "futures-io")]
fn bufread_multiple_members_with_invalid_padding() {
    let compressed = [
        sync::compress(&[1, 2, 3, 4, 5, 6]),
        vec![0, 0, 0],
        sync::compress(&[6, 5, 4, 3, 2, 1]),
        vec![0, 0, 0, 0],
    ]
    .join(&[][..]);

    let input = InputStream::from(vec![compressed]);

    let mut decoder = bufread::Decoder::new(bufread::from(&input));
    decoder.multiple_members(true);

    let mut output = Vec::new();
    assert!(block_on(decoder.read_to_end(&mut output)).is_err());
}
