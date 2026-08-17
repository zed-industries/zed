/// Helper for specifying the method family for a given selector.
///
/// Note that we can't actually check if a method is in a method family; only
/// whether the _selector_ is in a _selector_ family.
///
/// The slight difference here is:
/// - The method may be annotated with the `objc_method_family` attribute,
///   which would cause it to be in a different family. That this is not the
///   case is part of the `unsafe` contract of `msg_send!`.
/// - The method may not obey the added restrictions of the method family.
///   The added restrictions are:
///   - `new`, `alloc`, `copy` and `mutableCopy`: The method must return a
///     retainable object pointer type - we ensure this by making
///     `message_send` return `Retained`.
///   - `init`: The method must be an instance method and must return an
///     Objective-C pointer type - We ensure this by taking `Allocated<T>`,
///     which means it can't be a class method!
///
/// <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#retainable-object-pointers-as-operands-and-arguments>
// TODO: Use an enum instead of u8 here when possible in `const`.
#[derive(Debug)]
pub struct MethodFamily<const INNER: u8> {}

/// The `new` family.
pub type NewFamily = MethodFamily<1>;
/// The `alloc` family.
pub type AllocFamily = MethodFamily<2>;
/// The `init` family.
pub type InitFamily = MethodFamily<3>;
/// The `copy` family.
pub type CopyFamily = MethodFamily<4>;
/// The `mutableCopy` family.
pub type MutableCopyFamily = MethodFamily<5>;
/// No family.
pub type NoneFamily = MethodFamily<6>;

// These are used to avoid trying to do retain-semantics for these special
// selectors that would otherwise fall under `NoneFamily`.

/// The `retain` selector itself.
pub type RetainSelector = MethodFamily<8>;
/// The `release` selector itself.
pub type ReleaseSelector = MethodFamily<9>;
/// The `autorelease` selector itself.
pub type AutoreleaseSelector = MethodFamily<10>;
/// The `dealloc` selector itself.
pub type DeallocSelector = MethodFamily<11>;

/// Helper module where `#[unsafe(method_family = $family:ident)]` will import
/// its value from.
#[allow(non_camel_case_types)]
pub mod method_family_import {
    // Rename to match Clang's `__attribute__((objc_method_family(family)))`.
    pub use super::{
        AllocFamily as alloc, CopyFamily as copy, InitFamily as init,
        MutableCopyFamily as mutableCopy, NewFamily as new, NoneFamily as none,
    };
}

/// Determine the constant to specify in `MethodFamily` to get the method
/// family type.
///
/// This is only called with the first part of the selector, as that's enough
/// to determine the family, and that way we can emit less code for rustc to
/// parse.
///
/// Examples:
/// - `init` in `init`, returns `3`.
/// - `allocWithZone` in `allocWithZone:`, returns `2`.
/// - `copyItemAtURL` in `copyItemAtURL:toURL:error:`, returns `4`.
/// - `convertRect` in `convertRect:fromView:`, returns `6`.
pub const fn method_family(first_selector_part: &str) -> u8 {
    let first_selector_part = first_selector_part.as_bytes();
    match (
        in_selector_family(first_selector_part, b"new"),
        in_selector_family(first_selector_part, b"alloc"),
        in_selector_family(first_selector_part, b"init"),
        in_selector_family(first_selector_part, b"copy"),
        in_selector_family(first_selector_part, b"mutableCopy"),
    ) {
        (true, false, false, false, false) => 1,
        (false, true, false, false, false) => 2,
        (false, false, true, false, false) => 3,
        (false, false, false, true, false) => 4,
        (false, false, false, false, true) => 5,
        (false, false, false, false, false) => 6,
        _ => unreachable!(),
    }
}

/// Get the method family from an explicit family name, if specified,
/// otherwise infer it from the given selector.
///
/// No validation of the selector is done here, that must be done elsewhere.
#[doc(hidden)]
#[macro_export]
macro_rules! __method_family {
    // Explicit method family provided.
    //
    // This branch is placed first for compile-time performance.
    (
        ($($method_family:tt)+)
        ($($sel:tt)*)
    ) => {
        $crate::__macro_helpers::method_family_import::$($method_family)+
    };

    // Often called, avoid generating logic for figuring it out from selector.
    (
        ()
        (alloc)
    ) => {
        $crate::__macro_helpers::AllocFamily
    };
    (
        ()
        (new)
    ) => {
        $crate::__macro_helpers::NewFamily
    };
    (
        ()
        (init)
    ) => {
        $crate::__macro_helpers::InitFamily
    };

    // To prevent automatic memory management when using these.
    (
        ()
        (dealloc)
    ) => {
        $crate::__macro_helpers::DeallocSelector
    };
    (
        ()
        (retain)
    ) => {
        $crate::__macro_helpers::RetainSelector
    };
    (
        ()
        (release)
    ) => {
        $crate::__macro_helpers::ReleaseSelector
    };
    (
        ()
        (autorelease)
    ) => {
        $crate::__macro_helpers::AutoreleaseSelector
    };

    // Figure out from selector.
    (
        ()
        ($sel_first:tt $($sel_rest:tt)*)
    ) => {
        $crate::__macro_helpers::MethodFamily<{
            // Method families can be determined from just the first part of
            // the selector, so for compile-time performance we only stringify
            // and pass that part.
            $crate::__macro_helpers::method_family($crate::__macro_helpers::stringify!($sel_first))
        }>
    };

    // Missing selector, allow for better UI.
    (
        ()
        ()
    ) => {
        $crate::__macro_helpers::MethodFamily<{
            $crate::__macro_helpers::compile_error!("missing selector");
            $crate::__macro_helpers::method_family("")
        }>
    };
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
            // Selector can't be part of family if smaller than it
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

    #[test]
    fn test_method_family() {
        #[track_caller]
        fn assert_types_eq<T: 'static, U: 'static>() {
            assert_eq!(std::any::TypeId::of::<T>(), std::any::TypeId::of::<U>());
        }

        assert_types_eq::<AllocFamily, __method_family!(()(alloc))>();
        assert_types_eq::<AllocFamily, __method_family!(()(allocWithZone:))>();
        assert_types_eq::<CopyFamily, __method_family!(()(copyItemAtURL:toURL:error:))>();
        assert_types_eq::<NewFamily, __method_family!(()(new))>();
        assert_types_eq::<InitFamily, __method_family!(()(initWithArray:))>();
        assert_types_eq::<NoneFamily, __method_family!(()(somethingElse:))>();

        assert_types_eq::<CopyFamily, __method_family!((copy)(initWithArray:))>();
    }
}
