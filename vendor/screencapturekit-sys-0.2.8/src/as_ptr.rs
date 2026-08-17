pub trait AsPtr {
    fn as_ptr(&self) -> *const Self {
        self as *const Self
    }
}
pub trait AsMutPtr {
    fn as_mut_ptr(&self) -> *mut Self {
        self as *const _ as *mut Self
    }
}

impl<T> AsPtr for T {}
impl<T> AsMutPtr for T {}
