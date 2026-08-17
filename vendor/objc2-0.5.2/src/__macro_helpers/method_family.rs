/// Helper for specifying the retain semantics for a given selector family.
///
/// Note that we can't actually check if a method is in a method family; only
/// whether the _selector_ is in a _selector_ family.
///
/// The slight difference here is:
/// - The method may be annotated with the `objc_method_family` attribute,
///   which would cause it to be in a different family. That this is not the
///   case is part of the `unsafe` contract of `msg_send_id!`.
/// - The method may not obey the added restrictions of the method family.
///   The added restrictions are:
///   - `new`, `alloc`, `copy` and `mutableCopy`: The method must return a
///     retainable object pointer type - we ensure this by making
///     `message_send_id` return `Retained`.
///   - `init`: The method must be an instance method and must return an
///     Objective-C pointer type - We ensure this by taking `Allocated<T>`,
///     which means it can't be a class method!
///
/// <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#retainable-object-pointers-as-operands-and-arguments>
// TODO: Use an enum instead of u8 here when stable
#[derive(Debug)]
pub struct RetainSemantics<const INNER: u8> {}

pub type New = RetainSemantics<1>;
pub type Alloc = RetainSemantics<2>;
pub type Init = RetainSemantics<3>;
pub type CopyOrMutCopy = RetainSemantics<4>;
pub type Other = RetainSemantics<5>;

pub const fn retain_semantics(selector: &str) -> u8 {
    let selector = selector.as_bytes();
    match (
        in_selector_family(selector, b"new"),
        in_selector_family(selector, b"alloc"),
        in_selector_family(selector, b"init"),
        in_selector_family(selector, b"copy"),
        in_selector_family(selector, b"mutableCopy"),
    ) {
        (true, false, false, false, false) => 1,
        (false, true, false, false, false) => 2,
        (false, false, true, false, false) => 3,
        (false, false, false, true, false) => 4,
        (false, false, false, false, true) => 4,
        (false, false, false, false, false) => 5,
        _ => unreachable!(),
    }
}

/// Checks whether a given selector is said to be in a given selector family.
///
/// <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#arc-method-families>
const fn in_selector_family(mut selector: &[u8], mut family: &[u8]) -> bool {
    // Skip leading underscores from selector
    loop {
        selector = match selector {
            [b'_', rest @ ..] => rest,
            _ => break,
        }
    }

    // Compare each character
    loop {
        (selector, family) = match (selector, family) {
            // Remaining items
            ([s, selector @ ..], [f, family @ ..]) => {
                if *s == *f {
                    // Next iteration
                    (selector, family)
                } else {
                    // Family does not begin with selector
                    return false;
                }
            }
            // Equal
            ([], []) => {
                return true;
            }
            // Selector can't be part of familiy if smaller than it
            ([], _) => {
                return false;
            }
            // Remaining items in selector
            // -> ensure next character is not lowercase
            ([s, ..], []) => {
                return !s.is_ascii_lowercase();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_in_selector_family() {
        #[track_caller]
        fn assert_in_family(selector: &str, family: &str) {
            assert!(in_selector_family(selector.as_bytes(), family.as_bytes()));
            let selector = selector.to_string() + "\0";
            assert!(in_selector_family(selector.as_bytes(), family.as_bytes()));
        }

        #[track_caller]
        fn assert_not_in_family(selector: &str, family: &str) {
            assert!(!in_selector_family(selector.as_bytes(), family.as_bytes()));
            let selector = selector.to_string() + "\0";
            assert!(!in_selector_family(selector.as_bytes(), family.as_bytes()));
        }

        // Common cases

        assert_in_family("alloc", "alloc");
        assert_in_family("allocWithZone:", "alloc");
        assert_not_in_family("dealloc", "alloc");
        assert_not_in_family("initialize", "init");
        assert_not_in_family("decimalNumberWithDecimal:", "init");
        assert_in_family("initWithCapacity:", "init");
        assert_in_family("_initButPrivate:withParam:", "init");
        assert_not_in_family("description", "init");
        assert_not_in_family("inIT", "init");

        assert_not_in_family("init", "copy");
        assert_not_in_family("copyingStuff:", "copy");
        assert_in_family("copyWithZone:", "copy");
        assert_not_in_family("initWithArray:copyItems:", "copy");
        assert_in_family("copyItemAtURL:toURL:error:", "copy");

        assert_not_in_family("mutableCopying", "mutableCopy");
        assert_in_family("mutableCopyWithZone:", "mutableCopy");
        assert_in_family("mutableCopyWithZone:", "mutableCopy");

        assert_in_family(
            "newScriptingObjectOfClass:forValueForKey:withContentsValue:properties:",
            "new",
        );
        assert_in_family(
            "newScriptingObjectOfClass:forValueForKey:withContentsValue:properties:",
            "new",
        );
        assert_not_in_family("newsstandAssetDownload", "new");

        // Trying to weed out edge-cases:

        assert_in_family("__abcDef", "abc");
        assert_in_family("_abcDef", "abc");
        assert_in_family("abcDef", "abc");
        assert_in_family("___a", "a");
        assert_in_family("__a", "a");
        assert_in_family("_a", "a");
        assert_in_family("a", "a");

        assert_not_in_family("_abcdef", "abc");
        assert_not_in_family("_abcdef", "def");
        assert_not_in_family("_bcdef", "abc");
        assert_not_in_family("a_bc", "abc");
        assert_not_in_family("abcdef", "abc");
        assert_not_in_family("abcdef", "def");
        assert_not_in_family("abcdef", "abb");
        assert_not_in_family("___", "a");
        assert_not_in_family("_", "a");
        assert_not_in_family("", "a");

        assert_in_family("copy", "copy");
        assert_in_family("copy:", "copy");
        assert_in_family("copyMe", "copy");
        assert_in_family("_copy", "copy");
        assert_in_family("_copy:", "copy");
        assert_in_family("_copyMe", "copy");
        assert_not_in_family("copying", "copy");
        assert_not_in_family("copying:", "copy");
        assert_not_in_family("_copying", "copy");
        assert_not_in_family("Copy", "copy");
        assert_not_in_family("COPY", "copy");

        // Empty family (not supported)
        assert_in_family("___", "");
        assert_in_family("__", "");
        assert_in_family("_", "");
        assert_in_family("", "");
        assert_not_in_family("_a", "");
        assert_not_in_family("a", "");
        assert_in_family("_A", "");
        assert_in_family("A", "");

        // Double-colon selectors
        assert_in_family("abc::abc::", "abc");
        assert_in_family("abc:::", "abc");
        assert_in_family("abcDef::xyz:", "abc");
        // Invalid selector (probably)
        assert_not_in_family("::abc:", "abc");
    }
}
