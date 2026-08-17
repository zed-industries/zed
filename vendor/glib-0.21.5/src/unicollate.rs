// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ffi;

// rustdoc-stripper-ignore-next
/// A `CollationKey` allows ordering strings using the linguistically correct rules for the current locale.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CollationKey(crate::Slice<u8>);

impl<T: AsRef<str>> From<T> for CollationKey {
    // rustdoc-stripper-ignore-next
    /// Converts a string into a `CollationKey` that can be compared with other
    /// collation keys produced by the same function using `std::cmp::Ordering::cmp()`.
    #[doc(alias = "g_utf8_collate_key")]
    fn from(s: T) -> Self {
        let s = s.as_ref();
        let key = unsafe {
            let ptr = ffi::g_utf8_collate_key(s.as_ptr() as *const _, s.len() as isize);
            let len = libc::strlen(ptr);

            crate::Slice::from_glib_full_num(ptr as *mut u8, len)
        };
        Self(key)
    }
}

// rustdoc-stripper-ignore-next
/// A `FilenameCollationKey` allows ordering file names using the linguistically correct rules for the current locale.
/// Compared to `CollationKey`, filename collation keys take into consideration dots and other characters
/// commonly found in file names.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FilenameCollationKey(crate::Slice<u8>);

impl<T: AsRef<str>> From<T> for FilenameCollationKey {
    // rustdoc-stripper-ignore-next
    /// Converts a string into a `FilenameCollationKey` that can be compared with other
    /// collation keys produced by the same function using `std::cmp::Ordering::cmp()`.
    #[doc(alias = "g_utf8_collate_key_for_filename")]
    fn from(s: T) -> Self {
        let s = s.as_ref();
        let key = unsafe {
            let ptr =
                ffi::g_utf8_collate_key_for_filename(s.as_ptr() as *const _, s.len() as isize);
            let len = libc::strlen(ptr);

            crate::Slice::from_glib_full_num(ptr as *mut u8, len)
        };
        Self(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collate() {
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

        unsorted.sort_by(|s1, s2| {
            FilenameCollationKey::from(&s1).cmp(&FilenameCollationKey::from(&s2))
        });

        assert_eq!(unsorted, sorted);
    }

    #[test]
    fn collate_filenames_non_ascii() {
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

        unsorted.sort_by(|s1, s2| {
            FilenameCollationKey::from(&s1).cmp(&FilenameCollationKey::from(&s2))
        });

        assert_eq!(unsorted, sorted);
    }

    #[test]
    fn collate_filenames_from_path() {
        use std::path::PathBuf;

        let mut unsorted = vec![
            PathBuf::from("猫の手も借りたい.foo"),
            PathBuf::from("日本語は難しい.bar"),
            PathBuf::from("ありがとう.baz"),
        ];

        let sorted = vec![
            PathBuf::from("ありがとう.baz"),
            PathBuf::from("日本語は難しい.bar"),
            PathBuf::from("猫の手も借りたい.foo"),
        ];

        unsorted.sort_by(|s1, s2| {
            FilenameCollationKey::from(&s1.to_string_lossy())
                .cmp(&FilenameCollationKey::from(&s2.to_string_lossy()))
        });

        assert_eq!(unsorted, sorted);
    }
}
