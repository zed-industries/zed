//! Non-WASM, non-Windows, non-Apple: orphan section start/end symbols.

/// On LLVM/GCC platforms we can use orphan sections with _start and _end
/// symbols.
#[doc(hidden)]
#[macro_export]
macro_rules! __get_section_standard {
    (movable, name=$name:tt, type=$generic_ty:ty) => {
        {
            $crate::__weak_section_symbols!(item data $name);
            $crate::__weak_section_symbols!(backref data $name);
            $crate::__support::MovableBounds::new(
                $crate::__support::PtrBounds::new(
                    $crate::__address_of_symbol!(item data start $name),
                    $crate::__address_of_symbol!(item data end $name),
                ),
                $crate::__support::PtrBounds::new(
                    $crate::__address_of_symbol!(backref data start $name),
                    $crate::__address_of_symbol!(backref data end $name),
                ),
            )
        }
    };
    ($section_type:ident, name=$name:tt, type=$generic_ty:ty) => {
        {
            $crate::__weak_section_symbols!(item data $name);
            $crate::__support::PtrBounds::new(
                $crate::__address_of_symbol!(item data start $name),
                $crate::__address_of_symbol!(item data end $name),
            )
        }
    }
}

/// Declare a section's `__start_`/`__stop_` encapsulation symbols as weak.
///
/// This ensures we can reference them even if the section is empty
#[doc(hidden)]
#[macro_export]
#[cfg(all(not(miri), not(target_os = "aix"), not(target_family = "wasm")))]
macro_rules! __weak_section_symbols {
    // `$ref_or_item` is `item` or `backref`; it both selects the symbol names
    // (via `section_name!`) and names the wrapper module, so the value- and
    // backref-symbol invocations in the `movable` arm don't collide.
    ($ref_or_item:ident $section:ident $name:tt) => {
        mod $ref_or_item {
            ::core::arch::global_asm!(::core::concat!(
                ".weak ",
                $crate::__support::section_name!(string $ref_or_item $section start $name),
                "\n",
                ".weak ",
                $crate::__support::section_name!(string $ref_or_item $section end $name),
                "\n",
            ));
        }
    };
}

/// Declare a section's `__start_`/`__stop_` encapsulation symbols as weak.
///
/// This ensures we can reference them even if the section is empty
#[doc(hidden)]
#[macro_export]
#[cfg(not(all(not(miri), not(target_os = "aix"), not(target_family = "wasm"))))]
macro_rules! __weak_section_symbols {
    ($($args:tt)*) => {};
}

pub use crate::__get_section_standard as get_section;

crate::__def_section_name! {
    __section_name_standard,
    {
        data bare =>    (           "_data" "_link_section_") __ ();
        data section => (           "_data" "_link_section_") __ ();
        data start =>   ("__start_" "_data" "_link_section_") __ ();
        data end =>     ("__stop_"  "_data" "_link_section_") __ ();
        code bare =>    (           "_text" "_link_section_") __ ();
        code section => (           "_text" "_link_section_") __ ();
        code start =>   ("__start_" "_text" "_link_section_") __ ();
        code end =>     ("__stop_"  "_text" "_link_section_") __ ();
    }
    AUXILIARY = "_";
    REFS = "_r_";
    MAX_LENGTH = 64;
    HASH_LENGTH = 10;
    VALID_SECTION_CHARS = "_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
}
