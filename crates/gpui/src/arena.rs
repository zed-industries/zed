use std::{
    alloc,
    cell::Cell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

struct ArenaElement {
    value: *mut u8,
    drop: unsafe fn(*mut u8),
}

impl Drop for ArenaElement {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            (self.drop)(self.value);
        }
    }
}

struct OutsideElement {
    value: *mut u8,
    layout: alloc::Layout,
    drop: unsafe fn(*mut u8),
}

impl Drop for OutsideElement {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            (self.drop)(self.value);
            alloc::dealloc(self.value, self.layout);
        }
    }
}

pub struct Arena {
    start: *mut u8,
    end: *mut u8,
    offset: *mut u8,
    elements: Vec<ArenaElement>,
    outside_elements: Vec<OutsideElement>,
    outside_byte_count: usize,
    valid: Rc<Cell<bool>>,
}

impl Arena {
    pub fn new(size_in_bytes: usize) -> Self {
        unsafe {
            let layout = alloc::Layout::from_size_align(size_in_bytes, 1).unwrap();
            let start = alloc::alloc(layout);
            let end = start.add(size_in_bytes);
            Self {
                start,
                end,
                offset: start,
                elements: Vec::new(),
                outside_elements: Vec::new(),
                outside_byte_count: 0,
                valid: Rc::new(Cell::new(true)),
            }
        }
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.offset as usize - self.start as usize
    }

    pub fn capacity(&self) -> usize {
        self.end as usize - self.start as usize
    }

    /// Clears the arena contents, dropping all elements. All `ArenaBox` provided by this arena are
    /// invalidated and will panic if dereferenced.
    pub fn clear(&mut self) {
        self.valid.set(false);
        self.valid = Rc::new(Cell::new(true));
        self.elements.clear();
        self.outside_elements.clear();
        self.outside_byte_count = 0;
        self.offset = self.start;
    }

    /// Like `clear`, but also grows the arena if it needed to make allocations outside the arena.
    /// The new capacity will have at least enough space for those allocations, and at least
    /// `size_increment_bytes` beyond that - at most `2 * size_increment_bytes`.
    pub fn clear_and_grow_if_needed(&mut self, size_increment_in_bytes: usize) {
        if self.outside_byte_count == 0 {
            self.clear();
        } else {
            let additional_bytes =
                (self.outside_byte_count / size_increment_in_bytes + 2) * size_increment_in_bytes;
            let old_size = self.capacity();
            let new_size = old_size + additional_bytes;

            // free instead of keeping its capacity, often it won't be needed again
            self.outside_elements = Vec::new();

            self.clear();

            let old_layout = alloc::Layout::from_size_align(old_size, 1).unwrap();
            unsafe {
                alloc::dealloc(self.start, old_layout);
            }

            let new_layout = alloc::Layout::from_size_align(new_size, 1).unwrap();
            unsafe {
                self.start = alloc::alloc(new_layout);
                self.end = self.start.add(new_size);
                self.offset = self.start;
            }
        }
    }

    #[inline(always)]
    pub fn alloc<T>(&mut self, f: impl FnOnce() -> T) -> ArenaBox<T> {
        #[inline(always)]
        unsafe fn inner_writer<T, F>(ptr: *mut T, f: F)
        where
            F: FnOnce() -> T,
        {
            unsafe {
                std::ptr::write(ptr, f());
            }
        }

        unsafe fn drop<T>(ptr: *mut u8) {
            unsafe {
                std::ptr::drop_in_place(ptr.cast::<T>());
            }
        }

        unsafe {
            let layout = alloc::Layout::new::<T>();
            let offset = self.offset.add(self.offset.align_offset(layout.align()));
            let next_offset = offset.add(layout.size());

            if next_offset <= self.end {
                let ptr = offset.cast();
                inner_writer(ptr, f);

                self.elements.push(ArenaElement {
                    value: offset,
                    drop: drop::<T>,
                });
                self.offset = next_offset;

                ArenaBox {
                    ptr,
                    valid: self.valid.clone(),
                }
            } else {
                let value = alloc::alloc(layout);
                let ptr = value.cast();
                inner_writer(ptr, f);

                self.outside_byte_count += layout.size();
                self.outside_elements.push(OutsideElement {
                    value,
                    layout,
                    drop: drop::<T>,
                });

                ArenaBox {
                    ptr,
                    valid: self.valid.clone(),
                }
            }
        }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        self.clear();
    }
}

pub struct ArenaBox<T: ?Sized> {
    ptr: *mut T,
    valid: Rc<Cell<bool>>,
}

impl<T: ?Sized> ArenaBox<T> {
    #[inline(always)]
    pub fn map<U: ?Sized>(mut self, f: impl FnOnce(&mut T) -> &mut U) -> ArenaBox<U> {
        ArenaBox {
            ptr: f(&mut self),
            valid: self.valid,
        }
    }

    #[track_caller]
    fn validate(&self) {
        assert!(
            self.valid.get(),
            "attempted to dereference an ArenaRef after its Arena was cleared"
        );
    }
}

impl<T: ?Sized> Deref for ArenaBox<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.validate();
        unsafe { &*self.ptr }
    }
}

impl<T: ?Sized> DerefMut for ArenaBox<T> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.validate();
        unsafe { &mut *self.ptr }
    }
}

pub struct ArenaRef<T: ?Sized>(ArenaBox<T>);

impl<T: ?Sized> From<ArenaBox<T>> for ArenaRef<T> {
    fn from(value: ArenaBox<T>) -> Self {
        ArenaRef(value)
    }
}

impl<T: ?Sized> Clone for ArenaRef<T> {
    fn clone(&self) -> Self {
        Self(ArenaBox {
            ptr: self.0.ptr,
            valid: self.0.valid.clone(),
        })
    }
}

impl<T: ?Sized> Deref for ArenaRef<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    use super::*;

    #[test]
    fn test_arena() {
        run_test_arena(Arena::new(20))
    }

    #[test]
    fn test_arena_with_outside_allocation() {
        run_test_arena(Arena::new(0));
    }

    fn run_test_arena(mut arena: Arena) {
        let a = arena.alloc(|| 1u64);
        let b = arena.alloc(|| 2u32);
        let c = arena.alloc(|| 3u16);
        let d = arena.alloc(|| 4u8);
        assert_eq!(*a, 1);
        assert_eq!(*b, 2);
        assert_eq!(*c, 3);
        assert_eq!(*d, 4);

        arena.clear();
        assert_eq!(arena.len(), 0);

        let a = arena.alloc(|| 5u64);
        let b = arena.alloc(|| 6u32);
        let c = arena.alloc(|| 7u16);
        let d = arena.alloc(|| 8u8);
        assert_eq!(*a, 5);
        assert_eq!(*b, 6);
        assert_eq!(*c, 7);
        assert_eq!(*d, 8);

        // Ensure drop gets called.
        let dropped = Rc::new(Cell::new(false));
        struct DropGuard(Rc<Cell<bool>>);
        impl Drop for DropGuard {
            fn drop(&mut self) {
                self.0.set(true);
            }
        }
        arena.alloc(|| DropGuard(dropped.clone()));
        arena.clear();
        assert!(dropped.get());
    }

    #[test]
    fn test_arena_alignment() {
        let mut arena = Arena::new(256);
        run_test_arena_alignment(&mut arena);
        assert_eq!(arena.capacity(), 256);
        assert!(arena.outside_elements.is_empty());
    }

    #[test]
    fn test_arena_alignment_with_outside_allocation() {
        let mut arena = Arena::new(0);
        run_test_arena_alignment(&mut arena);
        assert_eq!(arena.capacity(), 0);
        assert!(!arena.outside_elements.is_empty());
    }

    fn run_test_arena_alignment(arena: &mut Arena) {
        let x1 = arena.alloc(|| 1u8);
        let x2 = arena.alloc(|| 2u16);
        let x3 = arena.alloc(|| 3u32);
        let x4 = arena.alloc(|| 4u64);
        let x5 = arena.alloc(|| 5u64);

        assert_eq!(*x1, 1);
        assert_eq!(*x2, 2);
        assert_eq!(*x3, 3);
        assert_eq!(*x4, 4);
        assert_eq!(*x5, 5);

        assert_eq!(x1.ptr.align_offset(std::mem::align_of_val(&*x1)), 0);
        assert_eq!(x2.ptr.align_offset(std::mem::align_of_val(&*x2)), 0);
        assert_eq!(x3.ptr.align_offset(std::mem::align_of_val(&*x3)), 0);
        assert_eq!(x4.ptr.align_offset(std::mem::align_of_val(&*x4)), 0);
        assert_eq!(x5.ptr.align_offset(std::mem::align_of_val(&*x5)), 0);
    }

    #[test]
    #[should_panic(expected = "attempted to dereference an ArenaRef after its Arena was cleared")]
    fn test_arena_use_after_clear() {
        run_test_arena_use_after_clear(Arena::new(16))
    }

    #[test]
    #[should_panic(expected = "attempted to dereference an ArenaRef after its Arena was cleared")]
    fn test_arena_use_after_clear_with_outside_allocation() {
        run_test_arena_use_after_clear(Arena::new(0))
    }

    fn run_test_arena_use_after_clear(mut arena: Arena) {
        let value = arena.alloc(|| 1u64);

        arena.clear();
        let _read_value = *value;
    }

    #[test]
    fn test_arena_clear_and_grow_if_needed() {
        if align_of::<u8>() != 1 {
            return;
        }

        let mut arena = Arena::new(2);
        arena.alloc(|| 1u8);
        arena.alloc(|| 2u8);
        arena.alloc(|| 3u8);
        arena.alloc(|| [4u8; 5]);

        assert_eq!(arena.outside_elements.len(), 2);

        arena.clear_and_grow_if_needed(2);
        assert_eq!(arena.len(), 0);
        assert_eq!(arena.capacity(), 12);

        arena.alloc(|| 1u8);
        arena.alloc(|| 2u8);

        arena.clear_and_grow_if_needed(2);
        assert_eq!(arena.len(), 0);
        assert_eq!(arena.capacity(), 12);
    }
}
