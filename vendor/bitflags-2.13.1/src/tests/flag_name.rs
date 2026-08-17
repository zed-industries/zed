use super::*;

use crate::{parser::*, Flags};

#[test]
fn cases() {
    let flags = TestRenamed::FLAGS
        .iter()
        .map(|flag| (flag.name(), flag.value().bits()))
        .collect::<Vec<_>>();

    assert_eq!(
        vec![
            ("custom", 1),
            ("custom", 1 << 1),
            ("c", 1 << 2),
            ("custom | e", 1 << 3),
        ],
        flags,
    );

    // Original names are not recognized
    assert!(TestRenamed::from_name("A").is_none());

    // Regular renames are recognized
    assert_eq!(TestRenamed::C, TestRenamed::from_name("c").unwrap());

    // The first duplicate name is recognized
    assert_eq!(TestRenamed::A, TestRenamed::from_name("custom").unwrap());

    // Exotic names are recognized
    assert_eq!(
        TestRenamed::D,
        TestRenamed::from_name("custom | e").unwrap()
    );

    // The parser doesn't have defined behavior on exotic names, but we
    // make sure it does _something_
    assert!(from_str_truncate::<TestRenamed>("custom | e").is_err());
}
