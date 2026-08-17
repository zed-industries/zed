use crate::{Array, Type};

pub struct ArrayProxy<T: Type<T>> {
    data: *mut *mut T::Default,
    len: *mut u32,
    temp: core::mem::ManuallyDrop<Array<T>>,
}

pub unsafe fn array_proxy<T: Type<T>>(data: *mut *mut T::Default, len: *mut u32) -> ArrayProxy<T> {
    ArrayProxy {
        data,
        len,
        temp: core::mem::ManuallyDrop::new(Array::new()),
    }
}

impl<T: Type<T>> Drop for ArrayProxy<T> {
    fn drop(&mut self) {
        unsafe {
            *self.data = self.temp.data;
            *self.len = self.temp.len;
        }
    }
}

impl<T: Type<T>> core::ops::Deref for ArrayProxy<T> {
    type Target = Array<T>;

    fn deref(&self) -> &Array<T> {
        &self.temp
    }
}

impl<T: Type<T>> core::ops::DerefMut for ArrayProxy<T> {
    fn deref_mut(&mut self) -> &mut Array<T> {
        &mut self.temp
    }
}
