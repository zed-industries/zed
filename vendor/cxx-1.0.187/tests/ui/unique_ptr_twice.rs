#[cxx::bridge]
mod here {
    extern "C++" {
        type C;
    }

    impl UniquePtr<C> {}
}

#[cxx::bridge]
mod there {
    extern "C++" {
        type C = crate::here::C;
    }

    impl UniquePtr<C> {}
}

fn main() {}
