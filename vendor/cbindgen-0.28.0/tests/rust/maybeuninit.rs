#[repr(C)]
pub struct MyStruct<'a> {
    number: std::mem::MaybeUninit<&'a i32>,
}

pub struct NotReprC<T> {
    inner: T,
}

pub type Foo<'a> = NotReprC<std::mem::MaybeUninit<&'a i32>>;

#[no_mangle]
pub extern "C" fn root<'a, 'b>(a: &'a Foo, with_maybe_uninit: &'b MyStruct) {}
