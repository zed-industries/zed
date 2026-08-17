#[cxx::bridge]
mod ffi {
    struct Shared {
        fd: i32,
    }
}

impl Drop for ffi::Shared {
    fn drop(&mut self) {
        println!("close({})", self.fd);
    }
}

fn main() {}
