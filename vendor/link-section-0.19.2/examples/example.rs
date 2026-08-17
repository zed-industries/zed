//! Example usage of the `link-section` crate.
#![warn(missing_docs)]

use link_section::{in_section, section};

/// An untyped link section with `code` linkage.
#[section(untyped)]
pub static LINK_SECTION: link_section::Section;

/// A function in the `LINK_SECTION` section.
#[in_section(LINK_SECTION)]
pub fn link_section_function() {
    println!("link_section_function");
}

/// A typed link section with `data` linkage.
#[section(typed)]
pub static TYPED_LINK_SECTION: link_section::TypedSection<u32>;

/// A `u32` in the `TYPED_LINK_SECTION` section.
#[in_section(TYPED_LINK_SECTION)]
pub static LINKED_U32: u32 = 1;

/// Another `u32` in the `TYPED_LINK_SECTION` section.
#[in_section(TYPED_LINK_SECTION)]
pub static LINKED_U32_2: u32 = 2;

mod aux_section {
    use link_section::{in_section, section};

    /// Create an aux link section for `TYPED_LINK_SECTION`.
    #[section(typed, aux(main = super::TYPED_LINK_SECTION))]
    pub static AUX_LINK_SECTION: link_section::TypedSection<u32>;

    /// An auxiliary section item.
    #[in_section(AUX_LINK_SECTION)]
    pub static AUX_LINKED_U32: u32 = 3;
}

/// A function pointerarray in the `data` section.
#[section(typed)]
pub static FN_ARRAY: link_section::TypedSection<fn()>;

/// A function in the `FN_ARRAY` section.
#[in_section(FN_ARRAY)]
pub fn linked_function() {
    eprintln!("linked_function");
}

/// Another function in the `FN_ARRAY` section.
#[in_section(FN_ARRAY)]
pub fn linked_function_2() {
    eprintln!("linked_function_2");
}

/// Yet another function in the `FN_ARRAY` section.
#[in_section(FN_ARRAY)]
pub static OTHER_FN: fn() = link_section_function;

/// A debuggable section in the `data` section.
#[section(typed)]
pub static DEBUGGABLES: link_section::TypedSection<&'static (dyn ::core::fmt::Debug + Sync)>;

/// A debuggable in the `DEBUGGABLES` section.
#[in_section(DEBUGGABLES)]
pub static DEBUGGABLE: &'static (dyn ::core::fmt::Debug + Sync) = &1;

/// Another debuggable in the `DEBUGGABLES` section.
#[in_section(DEBUGGABLES)]
pub static DEBUGGABLE_2: &'static (dyn ::core::fmt::Debug + Sync) = &2;

/// A function pointer in the `DEBUGGABLES` section.
#[in_section(DEBUGGABLES)]
pub const DEBUGGABLE_FUNCTION: &'static (dyn ::core::fmt::Debug + Sync) = {
    struct Debuggable;
    impl ::core::fmt::Debug for Debuggable {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.write_str("debuggable_function")
        }
    }
    &Debuggable
};

/// Dumps the various sections.
pub fn main() {
    eprintln!("LINK_SECTION: {:?}", LINK_SECTION);
    link_section_function();
    eprintln!("TYPED_LINK_SECTION: {:?}", TYPED_LINK_SECTION);
    assert!(TYPED_LINK_SECTION.offset_of(&LINKED_U32).is_some());
    assert!(TYPED_LINK_SECTION.offset_of(&LINKED_U32_2).is_some());
    eprintln!("AUX_LINK_SECTION: {:?}", aux_section::AUX_LINK_SECTION);
    assert!(aux_section::AUX_LINK_SECTION.len() == 1);
    let random_u32 = 1234567890;
    assert!(TYPED_LINK_SECTION.offset_of(&random_u32).is_none());
    eprintln!("CODE_SECTION: {:?}", FN_ARRAY);
    eprintln!("{:?}", FN_ARRAY.as_slice());
    for f in FN_ARRAY {
        eprintln!("f: {:?}", f);
        f();
        assert!(FN_ARRAY.offset_of(f).is_some());
    }
    eprintln!("DEBUGGABLES: {:?}", DEBUGGABLES.as_slice());
}
