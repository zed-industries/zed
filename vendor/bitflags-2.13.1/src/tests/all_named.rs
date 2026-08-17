use super::*;

use crate::Flags;

#[test]
fn cases() {
    case(1 | 1 << 1 | 1 << 2, TestFlags::all_named);

    case(1 | 1 << 1 | 1 << 2, TestExternal::all_named);

    case(0, TestExternalFull::all_named);
}

#[track_caller]
fn case<T: Flags>(expected: T::Bits, inherent: impl FnOnce() -> T)
where
    <T as Flags>::Bits: std::fmt::Debug + PartialEq,
{
    assert_eq!(expected, inherent().bits(), "T::all_named()");
    assert_eq!(expected, T::all_named().bits(), "Flags::all_named()");
}
