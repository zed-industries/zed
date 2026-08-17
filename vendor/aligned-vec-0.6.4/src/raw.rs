use crate::{Alignment, TryReserveError};
use alloc::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};
use core::{
    marker::PhantomData,
    mem::{align_of, size_of},
    ptr::{null_mut, NonNull},
};

pub struct ARawVec<T, A: Alignment> {
    pub ptr: NonNull<T>,
    pub capacity: usize,
    pub align: A,
    _marker: PhantomData<T>,
}

impl<T, A: Alignment> Drop for ARawVec<T, A> {
    #[inline]
    fn drop(&mut self) {
        // this can't overflow since we already have this much stored in a slice
        let size_bytes = self.capacity * size_of::<T>();
        if size_bytes > 0 {
            // SAFETY: memory was allocated with alloc::alloc::alloc
            unsafe {
                dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::from_size_align_unchecked(
                        size_bytes,
                        self.align.alignment(align_of::<T>()),
                    ),
                )
            }
        }
    }
}

pub fn capacity_overflow() -> ! {
    panic!("capacity overflow")
}

impl<T, A: Alignment> ARawVec<T, A> {
    /// # Safety
    ///
    /// `align` must be a power of two.  
    /// `align` must be greater than or equal to `core::mem::align_of::<T>()`.
    #[inline]
    pub unsafe fn new_unchecked(align: usize) -> Self {
        let cap = if size_of::<T>() == 0 { usize::MAX } else { 0 };
        Self::from_raw_parts(null_mut::<u8>().wrapping_add(align) as *mut T, cap, align)
    }

    /// # Safety
    ///
    /// `align` must be a power of two.  
    /// `align` must be greater than or equal to `core::mem::align_of::<T>()`.
    #[inline]
    pub unsafe fn with_capacity_unchecked(capacity: usize, align: usize) -> Self {
        if capacity == 0 || size_of::<T>() == 0 {
            Self::new_unchecked(align)
        } else {
            Self {
                ptr: NonNull::<T>::new_unchecked(with_capacity_unchecked(
                    capacity,
                    align,
                    size_of::<T>(),
                ) as *mut T),
                capacity,
                align: A::new(align, align_of::<T>()),
                _marker: PhantomData,
            }
        }
    }

    /// # Safety
    ///
    /// `align` must be a power of two.  
    /// `align` must be greater than or equal to `core::mem::align_of::<T>()`.
    #[inline]
    pub unsafe fn try_with_capacity_unchecked(
        capacity: usize,
        align: usize,
    ) -> Result<Self, TryReserveError> {
        if capacity == 0 || size_of::<T>() == 0 {
            Ok(Self::new_unchecked(align))
        } else {
            Ok(Self {
                ptr: NonNull::<T>::new_unchecked(try_with_capacity_unchecked(
                    capacity,
                    align,
                    size_of::<T>(),
                )? as *mut T),
                capacity,
                align: A::new(align, align_of::<T>()),
                _marker: PhantomData,
            })
        }
    }

    const MIN_NON_ZERO_CAP: usize = if size_of::<T>() == 1 {
        8
    } else if size_of::<T>() <= 1024 {
        4
    } else {
        1
    };

    pub unsafe fn grow_amortized(&mut self, len: usize, additional: usize) {
        debug_assert!(additional > 0);
        if self.capacity == 0 {
            *self = Self::with_capacity_unchecked(
                additional.max(Self::MIN_NON_ZERO_CAP),
                self.align.alignment(align_of::<T>()),
            );
            return;
        }

        if size_of::<T>() == 0 {
            debug_assert_eq!(self.capacity, usize::MAX);
            capacity_overflow();
        }

        let new_cap = match len.checked_add(additional) {
            Some(cap) => cap,
            None => capacity_overflow(),
        };

        // self.cap * 2 can't overflow because it's less than isize::MAX
        let new_cap = new_cap.max(self.capacity * 2);
        let new_cap = new_cap.max(Self::MIN_NON_ZERO_CAP);

        let ptr = {
            grow_unchecked(
                self.as_mut_ptr() as *mut u8,
                self.capacity,
                new_cap,
                self.align.alignment(align_of::<T>()),
                size_of::<T>(),
            ) as *mut T
        };

        self.capacity = new_cap;
        self.ptr = NonNull::<T>::new_unchecked(ptr);
    }

    pub unsafe fn grow_exact(&mut self, len: usize, additional: usize) {
        debug_assert!(additional > 0);
        if size_of::<T>() == 0 {
            debug_assert_eq!(self.capacity, usize::MAX);
            capacity_overflow();
        }

        if self.capacity == 0 {
            *self =
                Self::with_capacity_unchecked(additional, self.align.alignment(align_of::<T>()));
            return;
        }

        let new_cap = match len.checked_add(additional) {
            Some(cap) => cap,
            None => capacity_overflow(),
        };

        let ptr = grow_unchecked(
            self.as_mut_ptr() as *mut u8,
            self.capacity,
            new_cap,
            self.align.alignment(align_of::<T>()),
            size_of::<T>(),
        ) as *mut T;

        self.capacity = new_cap;
        self.ptr = NonNull::<T>::new_unchecked(ptr);
    }

    pub unsafe fn try_grow_amortized(
        &mut self,
        len: usize,
        additional: usize,
    ) -> Result<(), TryReserveError> {
        debug_assert!(additional > 0);
        if self.capacity == 0 {
            *self = Self::try_with_capacity_unchecked(
                additional.max(Self::MIN_NON_ZERO_CAP),
                self.align.alignment(align_of::<T>()),
            )?;
            return Ok(());
        }

        if size_of::<T>() == 0 {
            debug_assert_eq!(self.capacity, usize::MAX);
            return Err(TryReserveError::CapacityOverflow);
        }

        let new_cap = match len.checked_add(additional) {
            Some(cap) => cap,
            None => return Err(TryReserveError::CapacityOverflow),
        };

        // self.cap * 2 can't overflow because it's less than isize::MAX
        let new_cap = new_cap.max(self.capacity * 2);
        let new_cap = new_cap.max(Self::MIN_NON_ZERO_CAP);

        let ptr = {
            try_grow_unchecked(
                self.as_mut_ptr() as *mut u8,
                self.capacity,
                new_cap,
                self.align.alignment(align_of::<T>()),
                size_of::<T>(),
            )? as *mut T
        };

        self.capacity = new_cap;
        self.ptr = NonNull::<T>::new_unchecked(ptr);
        Ok(())
    }

    pub unsafe fn try_grow_exact(
        &mut self,
        len: usize,
        additional: usize,
    ) -> Result<(), TryReserveError> {
        debug_assert!(additional > 0);
        if size_of::<T>() == 0 {
            debug_assert_eq!(self.capacity, usize::MAX);
            return Err(TryReserveError::CapacityOverflow);
        }

        if self.capacity == 0 {
            *self = Self::try_with_capacity_unchecked(
                additional,
                self.align.alignment(align_of::<T>()),
            )?;
            return Ok(());
        }

        let new_cap = match len.checked_add(additional) {
            Some(cap) => cap,
            None => return Err(TryReserveError::CapacityOverflow),
        };

        let ptr = try_grow_unchecked(
            self.as_mut_ptr() as *mut u8,
            self.capacity,
            new_cap,
            self.align.alignment(align_of::<T>()),
            size_of::<T>(),
        )? as *mut T;

        self.capacity = new_cap;
        self.ptr = NonNull::<T>::new_unchecked(ptr);
        Ok(())
    }

    pub unsafe fn shrink_to(&mut self, len: usize) {
        if size_of::<T>() == 0 {
            return;
        }

        debug_assert!(len < self.capacity());
        let size_of = size_of::<T>();
        let old_capacity = self.capacity;
        let align = self.align;
        let old_ptr = self.ptr.as_ptr() as *mut u8;

        // this cannot overflow or exceed isize::MAX bytes since len < cap and the same was true
        // for cap
        let new_size_bytes = len * size_of;
        let old_size_bytes = old_capacity * size_of;
        let old_layout =
            Layout::from_size_align_unchecked(old_size_bytes, align.alignment(align_of::<T>()));

        let ptr = realloc(old_ptr, old_layout, new_size_bytes);
        let ptr = ptr as *mut T;
        self.capacity = len;
        self.ptr = NonNull::<T>::new_unchecked(ptr);
    }

    #[inline]
    pub unsafe fn from_raw_parts(ptr: *mut T, capacity: usize, align: usize) -> Self {
        Self {
            ptr: NonNull::<T>::new_unchecked(ptr),
            capacity,
            align: A::new(align, align_of::<T>()),
            _marker: PhantomData,
        }
    }

    /// Returns the capacity of the vector.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline]
    pub fn align(&self) -> usize {
        self.align.alignment(align_of::<T>())
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr()
    }
}

pub unsafe fn with_capacity_unchecked(capacity: usize, align: usize, size_of: usize) -> *mut u8 {
    let size_bytes = match capacity.checked_mul(size_of) {
        Some(size_bytes) => size_bytes,
        None => capacity_overflow(),
    };
    debug_assert!(size_bytes > 0);
    let will_overflow = size_bytes > usize::MAX - (align - 1);
    if will_overflow || !is_valid_alloc(size_bytes) {
        capacity_overflow();
    }

    let layout = Layout::from_size_align_unchecked(size_bytes, align);
    let ptr = alloc(layout);
    if ptr.is_null() {
        handle_alloc_error(layout);
    }
    ptr
}

unsafe fn grow_unchecked(
    old_ptr: *mut u8,
    old_capacity: usize,
    new_capacity: usize,
    align: usize,
    size_of: usize,
) -> *mut u8 {
    let new_size_bytes = match new_capacity.checked_mul(size_of) {
        Some(size_bytes) => size_bytes,
        None => capacity_overflow(),
    };
    let will_overflow = new_size_bytes > usize::MAX - (align - 1);
    if will_overflow || !is_valid_alloc(new_size_bytes) {
        capacity_overflow();
    }

    // can't overflow because we already allocated this much
    let old_size_bytes = old_capacity * size_of;
    let old_layout = Layout::from_size_align_unchecked(old_size_bytes, align);

    let ptr = realloc(old_ptr, old_layout, new_size_bytes);

    if ptr.is_null() {
        let new_layout = Layout::from_size_align_unchecked(new_size_bytes, align);
        handle_alloc_error(new_layout);
    }

    ptr
}

pub unsafe fn try_with_capacity_unchecked(
    capacity: usize,
    align: usize,
    size_of: usize,
) -> Result<*mut u8, TryReserveError> {
    let size_bytes = match capacity.checked_mul(size_of) {
        Some(size_bytes) => size_bytes,
        None => return Err(TryReserveError::CapacityOverflow),
    };
    debug_assert!(size_bytes > 0);
    let will_overflow = size_bytes > usize::MAX - (align - 1);
    if will_overflow || !is_valid_alloc(size_bytes) {
        return Err(TryReserveError::CapacityOverflow);
    }

    let layout = Layout::from_size_align_unchecked(size_bytes, align);
    let ptr = alloc(layout);
    if ptr.is_null() {
        return Err(TryReserveError::AllocError { layout });
    }
    Ok(ptr)
}

unsafe fn try_grow_unchecked(
    old_ptr: *mut u8,
    old_capacity: usize,
    new_capacity: usize,
    align: usize,
    size_of: usize,
) -> Result<*mut u8, TryReserveError> {
    let new_size_bytes = match new_capacity.checked_mul(size_of) {
        Some(size_bytes) => size_bytes,
        None => return Err(TryReserveError::CapacityOverflow),
    };
    let will_overflow = new_size_bytes > usize::MAX - (align - 1);
    if will_overflow || !is_valid_alloc(new_size_bytes) {
        return Err(TryReserveError::CapacityOverflow);
    }

    // can't overflow because we already allocated this much
    let old_size_bytes = old_capacity * size_of;
    let old_layout = Layout::from_size_align_unchecked(old_size_bytes, align);

    let ptr = realloc(old_ptr, old_layout, new_size_bytes);

    if ptr.is_null() {
        let layout = Layout::from_size_align_unchecked(new_size_bytes, align);
        return Err(TryReserveError::AllocError { layout });
    }

    Ok(ptr)
}

#[inline]
fn is_valid_alloc(alloc_size: usize) -> bool {
    !(usize::BITS < 64 && alloc_size > isize::MAX as usize)
}
