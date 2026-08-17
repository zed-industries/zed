#[cxx::bridge]
mod ffi {
    #[derive(Copy)]
    struct TryCopy {
        other: Other,
    }

    struct Other {
        x: usize,
    }
}

fn main() {}
