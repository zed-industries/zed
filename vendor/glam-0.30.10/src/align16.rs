#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
#[repr(C, align(16))]
pub(crate) struct Align16<T>(pub T);

impl<T> Align16<T> {
    #[allow(dead_code)]
    pub fn as_ptr(&self) -> *const T {
        &self.0
    }

    #[allow(dead_code)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        &mut self.0
    }
}

#[test]
fn test_align16() {
    use core::{mem, ptr};
    let mut a = Align16::<f32>(1.0);
    assert_eq!(mem::align_of_val(&a), 16);
    unsafe {
        assert_eq!(ptr::read(a.as_ptr()).to_bits(), f32::to_bits(1.0));
        ptr::write(a.as_mut_ptr(), -1.0);
    }
    assert_eq!(a.0.to_bits(), f32::to_bits(-1.0));
}
