#[repr(C)]
pub struct MyStruct {
    number: std::cell::Cell<i32>,
}

pub struct NotReprC<T> { inner: T }

pub type Foo = NotReprC<std::cell::RefCell<i32>>;

#[no_mangle]
pub extern "C" fn root(a: &Foo, with_cell: &MyStruct) {}
