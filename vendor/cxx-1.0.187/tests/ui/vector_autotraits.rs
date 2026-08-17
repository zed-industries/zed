use cxx::CxxVector;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        type ThreadSafe;
        type NotThreadSafe;
    }

    impl CxxVector<ThreadSafe> {}
    impl CxxVector<NotThreadSafe> {}
}

unsafe impl Send for ffi::ThreadSafe {}

fn assert_send<T: Send>() {}

fn main() {
    assert_send::<CxxVector<ffi::ThreadSafe>>();
    assert_send::<CxxVector<ffi::NotThreadSafe>>();
}
