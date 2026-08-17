use glib::{CollationKey, FilenameCollationKey};

fn init() {
    use std::sync::Once;
    static ONCE: Once = Once::new();

    // Make sure that all tests below are running with the system
    // locale and not the "C" locale.
    ONCE.call_once(|| unsafe {
        libc::setlocale(libc::LC_ALL, c"".as_ptr() as *const _);
    });
}

#[test]
fn collate() {
    init();

    let mut unsorted = vec![
        String::from("bcd"),
        String::from("cde"),
        String::from("abc"),
    ];

    let sorted = vec![
        String::from("abc"),
        String::from("bcd"),
        String::from("cde"),
    ];

    unsorted.sort_by(|s1, s2| CollationKey::from(&s1).cmp(&CollationKey::from(&s2)));

    assert_eq!(unsorted, sorted);
}

#[test]
fn collate_non_ascii() {
    init();

    let mut unsorted = vec![
        String::from("猫の手も借りたい"),
        String::from("日本語は難しい"),
        String::from("ありがとう"),
    ];

    let sorted = vec![
        String::from("ありがとう"),
        String::from("日本語は難しい"),
        String::from("猫の手も借りたい"),
    ];

    unsorted.sort_by(|s1, s2| CollationKey::from(&s1).cmp(&CollationKey::from(&s2)));

    assert_eq!(unsorted, sorted);
}

#[test]
fn collate_filenames() {
    init();

    let mut unsorted = vec![
        String::from("bcd.a"),
        String::from("cde.b"),
        String::from("abc.c"),
    ];

    let sorted = vec![
        String::from("abc.c"),
        String::from("bcd.a"),
        String::from("cde.b"),
    ];

    unsorted
        .sort_by(|s1, s2| FilenameCollationKey::from(&s1).cmp(&FilenameCollationKey::from(&s2)));

    assert_eq!(unsorted, sorted);
}

#[test]
fn collate_filenames_non_ascii() {
    init();

    let mut unsorted = vec![
        String::from("猫の手も借りたい.foo"),
        String::from("日本語は難しい.bar"),
        String::from("ありがとう.baz"),
    ];

    let sorted = vec![
        String::from("ありがとう.baz"),
        String::from("日本語は難しい.bar"),
        String::from("猫の手も借りたい.foo"),
    ];

    unsorted
        .sort_by(|s1, s2| FilenameCollationKey::from(&s1).cmp(&FilenameCollationKey::from(&s2)));

    assert_eq!(unsorted, sorted);
}
