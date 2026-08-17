use {
    crate::{size_hint, Arbitrary, Result, Unstructured},
    core::{
        array,
        mem::{self, MaybeUninit},
        ptr,
    },
};

/// Helper to safely create arrays since the standard library doesn't
/// provide one yet. Shouldn't be necessary in the future.
struct ArrayGuard<T, const N: usize> {
    dst: *mut T,
    initialized: usize,
}

impl<T, const N: usize> Drop for ArrayGuard<T, N> {
    fn drop(&mut self) {
        debug_assert!(self.initialized <= N);
        let initialized_part = ptr::slice_from_raw_parts_mut(self.dst, self.initialized);
        unsafe {
            ptr::drop_in_place(initialized_part);
        }
    }
}

fn try_create_array<F, T, const N: usize>(mut cb: F) -> Result<[T; N]>
where
    F: FnMut(usize) -> Result<T>,
{
    let mut array: MaybeUninit<[T; N]> = MaybeUninit::uninit();
    let array_ptr = array.as_mut_ptr();
    let dst = array_ptr as _;
    let mut guard: ArrayGuard<T, N> = ArrayGuard {
        dst,
        initialized: 0,
    };
    unsafe {
        for (idx, value_ptr) in (*array.as_mut_ptr()).iter_mut().enumerate() {
            ptr::write(value_ptr, cb(idx)?);
            guard.initialized += 1;
        }
        mem::forget(guard);
        Ok(array.assume_init())
    }
}

impl<'a, T, const N: usize> Arbitrary<'a> for [T; N]
where
    T: Arbitrary<'a>,
{
    #[inline]
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        try_create_array(|_| <T as Arbitrary<'a>>::arbitrary(u))
    }

    #[inline]
    fn arbitrary_take_rest(mut u: Unstructured<'a>) -> Result<Self> {
        let mut array = Self::arbitrary(&mut u)?;
        if let Some(last) = array.last_mut() {
            *last = Arbitrary::arbitrary_take_rest(u)?;
        }
        Ok(array)
    }

    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        Self::try_size_hint(depth).unwrap_or_default()
    }

    #[inline]
    fn try_size_hint(depth: usize) -> Result<(usize, Option<usize>), crate::MaxRecursionReached> {
        let hint = <T as Arbitrary>::try_size_hint(depth)?;
        Ok(size_hint::and_all(&array::from_fn::<_, N, _>(|_| hint)))
    }
}
