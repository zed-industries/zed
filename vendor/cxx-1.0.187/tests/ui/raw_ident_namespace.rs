use cxx::{type_id, ExternType};

#[repr(transparent)]
pub struct QuotedRaw(usize);

unsafe impl ExternType for QuotedRaw {
    type Id = type_id!("org::r#box::implementation::QuotedRaw");
    type Kind = cxx::kind::Trivial;
}

#[repr(transparent)]
pub struct QuotedKeyword(usize);

unsafe impl ExternType for QuotedKeyword {
    type Id = type_id!("org::box::implementation::QuotedKeyword");
    type Kind = cxx::kind::Trivial;
}

#[repr(transparent)]
pub struct UnquotedRaw(usize);

unsafe impl ExternType for UnquotedRaw {
    type Id = type_id!(org::r#box::implementation::UnquotedRaw);
    type Kind = cxx::kind::Trivial;
}

#[repr(transparent)]
pub struct UnquotedKeyword(usize);

unsafe impl ExternType for UnquotedKeyword {
    type Id = type_id!(org::box::implementation::UnquotedKeyword);
    type Kind = cxx::kind::Trivial;
}

#[cxx::bridge]
pub mod ffi {
    extern "C++" {
        #[namespace = "org::r#box::implementation"]
        type QuotedRaw = crate::QuotedRaw;

        #[namespace = "org::box::implementation"]
        type QuotedKeyword = crate::QuotedKeyword;

        #[namespace = org::r#box::implementation]
        type UnquotedRaw = crate::UnquotedRaw;

        // Not allowed by rustc (independent of cxx):
        // #[namespace = org::box::implementation]
        // type UnquotedKeyword = crate::UnquotedKeyword;
    }
}

fn main() {}
