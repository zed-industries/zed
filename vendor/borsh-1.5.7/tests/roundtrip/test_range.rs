#[test]
fn test_ranges() {
    let want = (1..2, 3..=4, 5.., ..6, ..=7, ..);

    let encoded = borsh::to_vec(&want).unwrap();
    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded);

    let got = borsh::from_slice(&encoded).unwrap();
    assert_eq!(want, got);
}
