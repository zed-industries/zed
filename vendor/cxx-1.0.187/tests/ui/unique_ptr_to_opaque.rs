mod outside {
    #[repr(C)]
    pub struct C {
        pub a: u8,
    }
    unsafe impl cxx::ExternType for C {
        type Id = cxx::type_id!("C");
        type Kind = cxx::kind::Opaque;
    }
}

#[cxx::bridge]
mod ffi {
    extern "C++" {
        type C = crate::outside::C;
    }

    impl UniquePtr<C> {}
}

fn main() {
    cxx::UniquePtr::new(outside::C { a: 4 });
}
