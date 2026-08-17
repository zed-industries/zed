use cxx::{type_id, ExternType};
use std::marker::PhantomPinned;

#[repr(C)]
struct Pinned(usize, PhantomPinned);

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type Pinned = crate::Pinned;
        fn f(_: &[Pinned], _: &mut [Pinned]);
    }
}

unsafe impl ExternType for Pinned {
    type Id = type_id!("Pinned");
    type Kind = cxx::kind::Trivial;
}

fn main() {}
