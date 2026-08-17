#[repr(C)]
pub struct MyStruct {
    number: Box<i32>,
}

pub struct NotReprC<T> {
    inner: T,
}

pub type Foo = NotReprC<Box<i32>>;

#[no_mangle]
pub extern "C" fn root(a: &Foo, with_box: &MyStruct) {}

#[no_mangle]
pub extern "C" fn drop_box(x: Box<i32>) {}

#[no_mangle]
pub extern "C" fn drop_box_opt(x: Option<Box<i32>>) {}
