#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("path/to" what);
        include!(<path/to> what);
        include!(<path/to);
        include!(<path[to]>);
        include!(...);
    }
}

fn main() {}
