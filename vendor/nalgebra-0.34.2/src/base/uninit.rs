use std::mem::MaybeUninit;

/// This trait is used to write code that may work on matrices that may or may not
/// be initialized.
///
/// This trait is used to describe how a value must be accessed to initialize it or
/// to retrieve a reference or mutable reference. Typically, a function accepting
/// both initialized and uninitialized inputs should have a `Status: InitStatus<T>`
/// type parameter. Then the methods of the `Status` can be used to access the element.
///
/// # Safety
/// This trait must not be implemented outside of this crate.
pub unsafe trait InitStatus<T>: Copy {
    /// The type of the values with the initialization status described by `Self`.
    type Value;

    /// Initialize the given element.
    fn init(out: &mut Self::Value, t: T);

    /// Retrieve a reference to the element, assuming that it is initialized.
    ///
    /// # Safety
    /// This is unsound if the referenced value isn’t initialized.
    unsafe fn assume_init_ref(t: &Self::Value) -> &T;

    /// Retrieve a mutable reference to the element, assuming that it is initialized.
    ///
    /// # Safety
    /// This is unsound if the referenced value isn’t initialized.
    unsafe fn assume_init_mut(t: &mut Self::Value) -> &mut T;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// A type implementing `InitStatus` indicating that the value is completely initialized.
pub struct Init;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// A type implementing `InitStatus` indicating that the value is completely uninitialized.
pub struct Uninit;

unsafe impl<T> InitStatus<T> for Init {
    type Value = T;

    #[inline(always)]
    fn init(out: &mut T, t: T) {
        *out = t;
    }

    #[inline(always)]
    unsafe fn assume_init_ref(t: &T) -> &T {
        t
    }

    #[inline(always)]
    unsafe fn assume_init_mut(t: &mut T) -> &mut T {
        t
    }
}

unsafe impl<T> InitStatus<T> for Uninit {
    type Value = MaybeUninit<T>;

    #[inline(always)]
    fn init(out: &mut MaybeUninit<T>, t: T) {
        *out = MaybeUninit::new(t);
    }

    #[inline(always)]
    unsafe fn assume_init_ref(t: &MaybeUninit<T>) -> &T {
        unsafe {
            &*t.as_ptr() // TODO: use t.assume_init_ref()
        }
    }

    #[inline(always)]
    unsafe fn assume_init_mut(t: &mut MaybeUninit<T>) -> &mut T {
        unsafe {
            &mut *t.as_mut_ptr() // TODO: use t.assume_init_mut()
        }
    }
}
