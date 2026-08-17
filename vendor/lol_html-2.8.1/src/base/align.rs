pub(crate) trait Align {
    fn align(&mut self, offset: usize);
}

impl<T: Align> Align for &mut [T] {
    #[inline]
    fn align(&mut self, offset: usize) {
        for item in self.iter_mut() {
            item.align(offset);
        }
    }
}

impl<T: Align> Align for Option<T> {
    #[inline]
    fn align(&mut self, offset: usize) {
        if let Some(val) = self {
            val.align(offset);
        }
    }
}

impl Align for usize {
    #[inline]
    fn align(&mut self, offset: usize) {
        if *self >= offset {
            *self -= offset;
        }
    }
}
