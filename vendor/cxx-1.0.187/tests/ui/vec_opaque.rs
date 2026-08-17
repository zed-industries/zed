#[cxx::bridge]
mod handle {
    extern "C++" {
        type Job;
    }
}

#[cxx::bridge]
mod ffi1 {
    extern "C++" {
        type Job;
    }

    extern "Rust" {
        fn f() -> Vec<Job>;
    }
}

#[cxx::bridge]
mod ffi2 {
    extern "C++" {
        type Job = crate::handle::Job;
    }

    extern "Rust" {
        fn f() -> Vec<Job>;
    }
}

fn f() -> Vec<handle::Job> {
    unimplemented!()
}

fn main() {}
