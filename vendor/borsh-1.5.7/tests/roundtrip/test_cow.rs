use borsh::{from_slice, to_vec};
use core::{matches, ops::Deref};
use alloc::string::ToString;

use alloc::{borrow::Cow, vec};

#[test]
fn test_cow_str() {
    let input: Cow<'_, str> = Cow::Borrowed("static input");

    let encoded = to_vec(&input).unwrap();

    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded);

    let out: Cow<'_, str> = from_slice(&encoded).unwrap();

    assert!(matches!(out, Cow::Owned(..)));

    assert_eq!(input, out);
    assert_eq!(out, "static input");
}

#[test]
fn test_cow_byte_slice() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let input: Cow<'_, [u8]> = Cow::Borrowed(&arr);

    let encoded = to_vec(&input).unwrap();

    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded);

    let out: Cow<'_, [u8]> = from_slice(&encoded).unwrap();

    assert!(matches!(out, Cow::Owned(..)));

    assert_eq!(input, out);
    assert_eq!(out, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_cow_slice_of_cow_str() {
    let arr = [
        Cow::Borrowed("first static input"),
        Cow::Owned("second static input".to_string()),
    ];
    let input: Cow<'_, [Cow<'_, str>]> = Cow::Borrowed(&arr);

    let encoded = to_vec(&input).unwrap();

    #[cfg(feature = "std")]
    insta::assert_debug_snapshot!(encoded);

    let out: Cow<'_, [Cow<'_, str>]> = from_slice(&encoded).unwrap();

    assert!(matches!(out, Cow::Owned(..)));

    for element in out.deref() {
        assert!(matches!(element, Cow::Owned(..)));
    }

    assert_eq!(input, out);
    assert_eq!(
        out,
        vec![
            Cow::Borrowed("first static input"),
            Cow::Borrowed("second static input"),
        ]
    );
}
