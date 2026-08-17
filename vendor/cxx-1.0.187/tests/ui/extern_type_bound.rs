#[cxx::bridge]
mod ffi {
    extern "C++" {
        type Opaque: PartialEq + PartialOrd;
    }
}

#[cxx::bridge]
mod ffi {
    extern "C++" {
        type Opaque: for<'de> Deserialize<'de>;
    }
}

fn main() {}
