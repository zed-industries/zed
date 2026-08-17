#[test]
fn string_matches_ints() {
    use crate::version::*;
    assert_eq!(STRING, format!("{}.{}.{}", MAJOR, MINOR, PATCH));
}
