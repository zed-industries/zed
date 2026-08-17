use crate::{mark_initialized, uninit_buf};

/// Build an array with a function that creates elements based on their index
///
/// ```
/// # use unarray::*;
/// let array: [usize; 5] = build_array(|i| i * 2);
/// assert_eq!(array, [0, 2, 4, 6, 8]);
/// ```
/// If `f` panics, any already-initialized elements will be dropped **without** running their
/// `Drop` implmentations, potentially creating resource leaks. Note that this is still "safe",
/// since Rust's notion of "safety" doesn't guarantee destructors are run.
///
/// For builder functions which might fail, consider using [`build_array_result`] or
/// [`build_array_option`]
pub fn build_array<T, F: FnMut(usize) -> T, const N: usize>(mut f: F) -> [T; N] {
    let mut result = uninit_buf();

    for (index, slot) in result.iter_mut().enumerate() {
        let value = f(index);
        slot.write(value);
    }

    // SAFETY:
    // We have iterated over every element in result and called `.write()` on it, so every element
    // is initialized
    unsafe { mark_initialized(result) }
}

/// Build an array with a function that creates elements based on their value, short-circuiting if
/// any index returns an `Err`
///
/// ```
/// # use unarray::*;
///
/// let success: Result<_, ()> = build_array_result(|i| Ok(i * 2));
/// assert_eq!(success, Ok([0, 2, 4]));
/// ```
///
/// If `f` panics, any already-initialized elements will be dropped **without** running their
/// `Drop` implmentations, potentially creating resource leaks. Note that this is still "safe",
/// since Rust's notion of "safety" doesn't guarantee destructors are run.
///
/// This is similar to the nightly-only [`core::array::try_from_fn`]
pub fn build_array_result<T, E, F: FnMut(usize) -> Result<T, E>, const N: usize>(
    mut f: F,
) -> Result<[T; N], E> {
    let mut result = uninit_buf();

    for (index, slot) in result.iter_mut().enumerate() {
        match f(index) {
            Ok(value) => slot.write(value),
            Err(e) => {
                // SAFETY:
                // We have failed at `index` which is the `index + 1`th element, so the first
                // `index` elements are safe to drop
                result
                    .iter_mut()
                    .take(index)
                    .for_each(|slot| unsafe { slot.assume_init_drop() });
                return Err(e);
            }
        };
    }

    // SAFETY:
    // We have iterated over every element in result and called `.write()` on it, so every element
    // is initialized
    Ok(unsafe { mark_initialized(result) })
}

/// Build an array with a function that creates elements based on their value, short-circuiting if
/// any index returns a `None`
///
/// ```
/// # use unarray::*;
/// let success = build_array_option(|i| Some(i * 2));
/// assert_eq!(success, Some([0, 2, 4]));
/// ```
///
/// If `f` panics, any already-initialized elements will be dropped **without** running their
/// `Drop` implmentations, potentially creating resource leaks. Note that this is still "safe",
/// since Rust's notion of "safety" doesn't guarantee destructors are run.
///
/// This is similar to the nightly-only [`core::array::try_from_fn`]
pub fn build_array_option<T, F: FnMut(usize) -> Option<T>, const N: usize>(
    mut f: F,
) -> Option<[T; N]> {
    let actual_f = |i: usize| -> Result<T, ()> { f(i).ok_or(()) };

    match build_array_result(actual_f) {
        Ok(array) => Some(array),
        Err(()) => None,
    }
}

#[cfg(test)]
mod tests {
    use core::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[test]
    fn test_build_array() {
        let array = build_array(|i| i * 2);
        assert_eq!(array, [0, 2, 4]);
    }

    #[test]
    fn test_build_array_option() {
        let array = build_array_option(|i| Some(i * 2));
        assert_eq!(array, Some([0, 2, 4]));

        let none: Option<[_; 10]> = build_array_option(|i| if i == 5 { None } else { Some(()) });
        assert_eq!(none, None);
    }

    #[test]
    fn test_build_array_result() {
        let array = build_array_result(|i| Ok::<usize, ()>(i * 2));
        assert_eq!(array, Ok([0, 2, 4]));

        let err: Result<[_; 10], _> = build_array_result(|i| if i == 5 { Err(()) } else { Ok(()) });
        assert_eq!(err, Err(()));
    }

    struct IncrementOnDrop<'a>(&'a AtomicUsize);
    impl Drop for IncrementOnDrop<'_> {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn result_doesnt_leak_on_err() {
        let drop_counter = 0.into();

        // this will successfully create 3 structs, fail on the 4th, we expect 3 drops to be
        // called, since the 4th may be in an inconsistent state
        let _: Result<[_; 5], _> = build_array_result(|i| {
            if i == 3 {
                Err(())
            } else {
                Ok(IncrementOnDrop(&drop_counter))
            }
        });

        assert_eq!(drop_counter.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn option_doesnt_leak_on_err() {
        let drop_counter = 0.into();

        // this will successfully create 3 structs, fail on the 4th, we expect 3 drops to be
        // called, since the 4th may be in an inconsistent state
        let _: Option<[_; 5]> = build_array_option(|i| {
            if i == 3 {
                None
            } else {
                Some(IncrementOnDrop(&drop_counter))
            }
        });

        assert_eq!(drop_counter.load(Ordering::Relaxed), 3);
    }
}
