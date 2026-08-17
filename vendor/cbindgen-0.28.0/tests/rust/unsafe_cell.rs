#[repr(C)]
pub struct MyStruct {
    number: std::cell::UnsafeCell<i32>,
}

pub struct NotReprC<T> { inner: T }

pub type Foo = NotReprC<std::cell::SyncUnsafeCell<i32>>;

#[no_mangle]
pub extern "C" fn root(a: &Foo, with_cell: &MyStruct) {}
