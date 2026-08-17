// TODO: More work is needed so that the missing_docs lints produced by rustc
// are properly positioned inside of the bridge.

//! ...

#![deny(missing_docs)]

/// ...
#[cxx::bridge]
pub mod ffi {
    pub struct UndocumentedStruct {
        pub undocumented_field: u8,
    }

    /// ...
    pub struct DocumentedStruct {
        /// ...
        pub documented_field: u8,
    }

    pub enum UndocumentedEnum {
        UndocumentedVariant = 0,
    }

    /// ...
    pub enum DocumentedEnum {
        /// ...
        DocumentedVariant = 0,
    }

    extern "Rust" {
        pub type UndocumentedRustType;

        /// ...
        pub type DocumentedRustType;

        pub fn undocumented_rust_fn() -> u8;

        /// ...
        pub fn documented_rust_fn() -> u8;
    }

    unsafe extern "C++" {
        pub type UndocumentedForeignType;

        /// ...
        pub type DocumentedForeignType;

        pub type UndocumentedTypeAlias = crate::bindgen::UndocumentedTypeAlias;

        /// ...
        pub type DocumentedTypeAlias = crate::bindgen::DocumentedTypeAlias;

        pub fn undocumented_foreign_fn() -> u8;

        /// ...
        pub fn documented_foreign_fn() -> u8;
    }

    #[allow(missing_docs)]
    pub struct SuppressUndocumentedStruct {
        pub undocumented_field: u8,
    }
}

struct UndocumentedRustType;
struct DocumentedRustType;

mod bindgen {
    use cxx::{type_id, ExternType};

    pub struct UndocumentedTypeAlias;
    pub struct DocumentedTypeAlias;

    unsafe impl ExternType for UndocumentedTypeAlias {
        type Id = type_id!("UndocumentedTypeAlias");
        type Kind = cxx::kind::Opaque;
    }

    unsafe impl ExternType for DocumentedTypeAlias {
        type Id = type_id!("DocumentedTypeAlias");
        type Kind = cxx::kind::Opaque;
    }
}

fn undocumented_rust_fn() -> u8 {
    0
}

fn documented_rust_fn() -> u8 {
    0
}

fn main() {}
