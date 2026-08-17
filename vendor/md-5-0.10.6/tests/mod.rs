use digest::dev::{feed_rand_16mib, fixed_reset_test};
use hex_literal::hex;
use md5::{Digest, Md5};

digest::new_test!(md5_main, "md5", md5::Md5, fixed_reset_test);

#[test]
fn md5_rand() {
    let mut h = Md5::new();
    feed_rand_16mib(&mut h);
    assert_eq!(
        h.finalize()[..],
        hex!("61aec26f1b909578ef638ae02dac0977")[..]
    );
}
