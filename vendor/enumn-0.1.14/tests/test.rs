use enumn::N;

#[derive(Debug, N, PartialEq)]
enum EmptyEnum {}

#[test]
fn test_empty() {
    assert_eq!(EmptyEnum::n(0), None);
    assert_eq!(EmptyEnum::n(1), None);
    assert_eq!(EmptyEnum::n(-1), None);
}

#[derive(Debug, N, PartialEq)]
enum SimpleEnum {
    Case0,
    Case1,
}

#[test]
fn test_simple() {
    assert_eq!(SimpleEnum::n(0), Some(SimpleEnum::Case0));
    assert_eq!(SimpleEnum::n(1), Some(SimpleEnum::Case1));
    assert_eq!(SimpleEnum::n(4), None);
    assert_eq!(SimpleEnum::n(-1), None);
}

#[derive(Debug, N, PartialEq)]
#[repr(u8)]
enum EnumWithRepr {
    Case0,
}

#[test]
fn test_repr() {
    assert_eq!(EnumWithRepr::n(0), Some(EnumWithRepr::Case0));
    assert_eq!(EnumWithRepr::n(255), None);
}

#[derive(Debug, N, PartialEq)]
enum EnumWithDiscriminant {
    A = 10,
    B, // implicitly 11
    C = -80,
}

#[test]
fn test_discriminant() {
    assert_eq!(EnumWithDiscriminant::n(10), Some(EnumWithDiscriminant::A));
    assert_eq!(EnumWithDiscriminant::n(11), Some(EnumWithDiscriminant::B));
    assert_eq!(EnumWithDiscriminant::n(-80), Some(EnumWithDiscriminant::C));
    assert_eq!(EnumWithDiscriminant::n(12), None);
}
