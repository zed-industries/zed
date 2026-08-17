use crate::prelude::*;
use core::cell::RefCell;

/// Small data structure to help extend the lifetime of a slice to a higher
/// scope.
///
/// This is currently used during component translation where translation in
/// general works on a borrowed slice which contains all input modules, but
/// generated adapter modules for components don't live within the original
/// slice but the data structures are much easier if the dynamically generated
/// adapter modules live for the same lifetime as the original input slice. To
/// solve this problem this `ScopeVec` helper is used to move ownership of a
/// `Vec<T>` to a higher scope in the program, then borrowing the slice from
/// that scope.
pub struct ScopeVec<T> {
    data: RefCell<Vec<Box<[T]>>>,
}

impl<T> ScopeVec<T> {
    /// Creates a new blank scope.
    pub fn new() -> ScopeVec<T> {
        ScopeVec {
            data: Default::default(),
        }
    }

    /// Transfers ownership of `data` into this scope and then yields the slice
    /// back to the caller.
    ///
    /// The original data will be deallocated when `self` is dropped.
    pub fn push(&self, data: Vec<T>) -> &mut [T] {
        let data: Box<[T]> = data.into();
        let len = data.len();

        let mut storage = self.data.borrow_mut();
        storage.push(data);
        let ptr = storage.last_mut().unwrap().as_mut_ptr();

        // This should be safe for a few reasons:
        //
        // * The returned pointer on the heap that `data` owns. Despite moving
        //   `data` around it doesn't actually move the slice itself around, so
        //   the pointer returned should be valid (and length).
        //
        // * The lifetime of the returned pointer is connected to the lifetime
        //   of `self`. This reflects how when `self` is destroyed the `data` is
        //   destroyed as well, or otherwise the returned slice will be valid
        //   for as long as `self` is valid since `self` owns the original data
        //   at that point.
        //
        // * This function was given ownership of `data` so it should be safe to
        //   hand back a mutable reference. Once placed within a `ScopeVec` the
        //   data is never mutated so the caller will enjoy exclusive access to
        //   the slice of the original vec.
        //
        // This all means that it should be safe to return a mutable slice of
        // all of `data` after the data has been pushed onto our internal list.
        unsafe { core::slice::from_raw_parts_mut(ptr, len) }
    }

    /// Iterate over items in this `ScopeVec`, consuming ownership.
    pub fn into_iter(self) -> impl ExactSizeIterator<Item = Box<[T]>> {
        self.data.into_inner().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::ScopeVec;
    use crate::prelude::*;

    #[test]
    fn smoke() {
        let scope = ScopeVec::new();
        let a = scope.push(Vec::new());
        let b = scope.push(vec![1, 2, 3]);
        let c = scope.push(vec![4, 5, 6]);
        assert_eq!(a.len(), 0);
        b[0] = 4;
        c[2] = 5;
        assert_eq!(a, []);
        assert_eq!(b, [4, 2, 3]);
        assert_eq!(c, [4, 5, 5]);
    }
}
