use crate::rt;

use std::borrow::Borrow;
use std::pin::Pin;
use std::{mem, ops, ptr};

/// Mock implementation of `std::sync::Arc`.
#[derive(Debug)]
pub struct Arc<T: ?Sized> {
    obj: std::sync::Arc<rt::Arc>,
    value: std::sync::Arc<T>,
}

impl<T> Arc<T> {
    /// Constructs a new `Arc<T>`.
    #[track_caller]
    pub fn new(value: T) -> Arc<T> {
        let std = std::sync::Arc::new(value);

        Arc::from_std(std)
    }

    /// Constructs a new `Pin<Arc<T>>`.
    pub fn pin(data: T) -> Pin<Arc<T>> {
        unsafe { Pin::new_unchecked(Arc::new(data)) }
    }

    /// Returns the inner value, if the `Arc` has exactly one strong reference.
    #[track_caller]
    pub fn try_unwrap(this: Arc<T>) -> Result<T, Arc<T>> {
        if !this.obj.get_mut(location!()) {
            return Err(this);
        }

        assert_eq!(1, std::sync::Arc::strong_count(&this.value));
        // work around our inability to destruct the object normally,
        // because of the `Drop` presense.
        this.obj.ref_dec(location!());
        this.unregister();

        // Use the same pattern of unwrapping as `std` does.
        // We can't normally move the field out of the object
        // because it implements `drop`.
        let arc_value = unsafe {
            let _arc_obj = ptr::read(&this.obj);
            let arc_value = ptr::read(&this.value);

            mem::forget(this);

            arc_value
        };
        match std::sync::Arc::try_unwrap(arc_value) {
            Ok(value) => Ok(value),
            Err(_) => unreachable!(),
        }
    }
}

impl<T: ?Sized> Arc<T> {
    /// Converts `std::sync::Arc` to `loom::sync::Arc`.
    ///
    /// This is needed to create a `loom::sync::Arc<T>` where `T: !Sized`.
    ///
    /// ## Panics
    ///
    /// If the provided `Arc` has copies (i.e., if it is not unique).
    ///
    /// ## Examples
    ///
    /// While `std::sync::Arc` with `T: !Sized` can be created by coercing an
    /// `std::sync::Arc` with a sized value:
    ///
    /// ```rust
    /// let sized: std::sync::Arc<[u8; 3]> = std::sync::Arc::new([1, 2, 3]);
    /// let _unsized: std::sync::Arc<[u8]> = sized; // coercion
    /// ```
    ///
    /// `loom::sync::Arc` can't be created in the same way:
    ///
    /// ```compile_fail,E0308
    /// use loom::sync::Arc;
    ///
    /// let sized: Arc<[u8; 3]> = Arc::new([1, 2, 3]);
    /// let _unsized: Arc<[u8]> = sized; // error: mismatched types
    /// ```
    ///
    /// This is because `std::sync::Arc` uses an unstable trait called `CoerceUnsized`
    /// that loom can't use. To create `loom::sync::Arc` with an unsized inner value
    /// first create a `std::sync::Arc` of an appropriate type and then use this method:
    ///
    /// ```rust
    /// use loom::sync::Arc;
    ///
    /// # loom::model::model(|| {
    /// let std: std::sync::Arc<[u8]> = std::sync::Arc::new([1, 2, 3]);
    /// let loom: Arc<[u8]> = Arc::from_std(std);
    ///
    /// let std: std::sync::Arc<dyn Send + Sync> = std::sync::Arc::new([1, 2, 3]);
    /// let loom: Arc<dyn Send + Sync> = Arc::from_std(std);
    /// # });
    /// ```
    #[track_caller]
    pub fn from_std(mut std: std::sync::Arc<T>) -> Self {
        assert!(
            std::sync::Arc::get_mut(&mut std).is_some(),
            "Arc provided to `from_std` is not unique"
        );

        let obj = std::sync::Arc::new(rt::Arc::new(location!()));
        let objc = std::sync::Arc::clone(&obj);

        rt::execution(|e| {
            e.arc_objs
                .insert(std::sync::Arc::as_ptr(&std) as *const (), objc);
        });

        Arc { obj, value: std }
    }

    /// Gets the number of strong (`Arc`) pointers to this value.
    #[track_caller]
    pub fn strong_count(this: &Self) -> usize {
        this.obj.strong_count()
    }

    /// Increments the strong reference count on the `Arc<T>` associated with the
    /// provided pointer by one.
    ///
    /// # Safety
    ///
    /// The pointer must have been obtained through `Arc::into_raw`, and the
    /// associated `Arc` instance must be valid (i.e. the strong count must be at
    /// least 1) for the duration of this method.
    #[track_caller]
    pub unsafe fn increment_strong_count(ptr: *const T) {
        // Retain Arc, but don't touch refcount by wrapping in ManuallyDrop
        let arc = mem::ManuallyDrop::new(Arc::<T>::from_raw(ptr));
        // Now increase refcount, but don't drop new refcount either
        let _arc_clone: mem::ManuallyDrop<_> = arc.clone();
    }

    /// Decrements the strong reference count on the `Arc<T>` associated with the
    /// provided pointer by one.
    ///
    /// # Safety
    ///
    /// The pointer must have been obtained through `Arc::into_raw`, and the
    /// associated `Arc` instance must be valid (i.e. the strong count must be at
    /// least 1) when invoking this method. This method can be used to release the final
    /// `Arc` and backing storage, but **should not** be called after the final `Arc` has been
    /// released.
    #[track_caller]
    pub unsafe fn decrement_strong_count(ptr: *const T) {
        mem::drop(Arc::from_raw(ptr));
    }

    /// Returns a mutable reference to the inner value, if there are
    /// no other `Arc` pointers to the same value.
    #[track_caller]
    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        if this.obj.get_mut(location!()) {
            assert_eq!(1, std::sync::Arc::strong_count(&this.value));
            Some(std::sync::Arc::get_mut(&mut this.value).unwrap())
        } else {
            None
        }
    }

    /// Returns `true` if the two `Arc`s point to the same value (not
    /// just values that compare as equal).
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&this.value, &other.value)
    }

    /// Consumes the `Arc`, returning the wrapped pointer.
    pub fn into_raw(this: Self) -> *const T {
        let ptr = Self::as_ptr(&this);
        mem::forget(this);
        ptr
    }

    /// Provides a raw pointer to the data.
    pub fn as_ptr(this: &Self) -> *const T {
        std::sync::Arc::as_ptr(&this.value)
    }

    /// Constructs an `Arc` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The raw pointer must have been previously returned by a call to
    /// [`Arc<U>::into_raw`][into_raw] where `U` must have the same size and
    /// alignment as `T`. This is trivially true if `U` is `T`.
    /// Note that if `U` is not `T` but has the same size and alignment, this is
    /// basically like transmuting references of different types. See
    /// [`mem::transmute`][transmute] for more information on what
    /// restrictions apply in this case.
    ///
    /// The user of `from_raw` has to make sure a specific value of `T` is only
    /// dropped once.
    ///
    /// This function is unsafe because improper use may lead to memory unsafety,
    /// even if the returned `Arc<T>` is never accessed.
    ///
    /// [into_raw]: Arc::into_raw
    /// [transmute]: core::mem::transmute
    #[track_caller]
    pub unsafe fn from_raw(ptr: *const T) -> Self {
        let inner = std::sync::Arc::from_raw(ptr);
        let obj = rt::execution(|e| std::sync::Arc::clone(&e.arc_objs[&ptr.cast()]));
        Arc { value: inner, obj }
    }

    /// Unregister this object before it's gone.
    fn unregister(&self) {
        rt::execution(|e| {
            e.arc_objs
                .remove(&std::sync::Arc::as_ptr(&self.value).cast())
                .expect("Arc object was removed before dropping last Arc");
        });
    }
}

impl<T: ?Sized> ops::Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: ?Sized> Clone for Arc<T> {
    #[track_caller]
    fn clone(&self) -> Arc<T> {
        self.obj.ref_inc(location!());

        Arc {
            value: self.value.clone(),
            obj: self.obj.clone(),
        }
    }
}

impl<T: ?Sized> Drop for Arc<T> {
    #[track_caller]
    fn drop(&mut self) {
        if self.obj.ref_dec(location!()) {
            assert_eq!(
                1,
                std::sync::Arc::strong_count(&self.value),
                "something odd is going on"
            );
            self.unregister();
        }
    }
}

impl<T: Default> Default for Arc<T> {
    #[track_caller]
    fn default() -> Arc<T> {
        Arc::new(Default::default())
    }
}

impl<T> From<T> for Arc<T> {
    #[track_caller]
    fn from(t: T) -> Self {
        Arc::new(t)
    }
}

impl<T: ?Sized> AsRef<T> for Arc<T> {
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T: ?Sized> Borrow<T> for Arc<T> {
    fn borrow(&self) -> &T {
        self
    }
}
