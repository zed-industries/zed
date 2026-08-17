use cxx::{type_id, ExternType};

#[repr(C)]
struct ElementTrivial(usize);

#[repr(C)]
struct ElementOpaque(usize);

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type ElementTrivial = crate::ElementTrivial;
        type ElementOpaque = crate::ElementOpaque;

        fn f(slice: &mut [ElementTrivial]);
        fn g(slice: &[ElementOpaque]);
    }
}

unsafe impl ExternType for ElementTrivial {
    type Id = type_id!("ElementTrivial");
    type Kind = cxx::kind::Trivial;
}

unsafe impl ExternType for ElementOpaque {
    type Id = type_id!("ElementOpaque");
    type Kind = cxx::kind::Opaque;
}

fn main() {}
