#[cxx::bridge]
mod ffi {
    struct Shared {
        arraystr: [String; "13"],
        arraysub: [String; 15 - 1],
        arrayzero: [String; 0],
    }
}

fn main() {}
