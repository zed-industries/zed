#![deny(elided_lifetimes_in_paths, mismatched_lifetime_syntaxes)]

use cxx::ExternType;
use std::marker::PhantomData;

#[repr(C)]
struct Alias<'a> {
    ptr: *const std::ffi::c_void,
    lifetime: PhantomData<&'a str>,
}

unsafe impl<'a> ExternType for Alias<'a> {
    type Id = cxx::type_id!("Alias");
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
mod ffi {
    #[derive(PartialEq, PartialOrd, Hash)]
    struct Struct<'a> {
        reference: &'a i32,
    }

    extern "Rust" {
        type Rust<'a>;
    }

    unsafe extern "C++" {
        type Cpp<'a>;
        type Alias<'a> = crate::Alias<'a>;

        fn lifetime_named<'a>(s: &'a i32) -> UniquePtr<Cpp<'a>>;

        fn lifetime_underscore(s: &i32) -> UniquePtr<Cpp<'_>>;

        fn lifetime_elided(s: &i32) -> UniquePtr<Cpp>;
    }
}

pub struct Rust<'a>(&'a i32);

fn main() {}
