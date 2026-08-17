#[cxx::bridge]
mod ffi {
    extern "C++" {
        type Opaque;
    }
}

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}
fn assert_unpin<T: Unpin>() {}

fn main() {
    assert_send::<ffi::Opaque>();
    assert_sync::<ffi::Opaque>();
    assert_unpin::<ffi::Opaque>();
}
