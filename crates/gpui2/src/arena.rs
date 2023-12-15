use std::{
    alloc,
    cell::Cell,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    rc::Rc,
};

struct ArenaElement {
    value: NonNull<u8>,
    drop: unsafe fn(NonNull<u8>),
}

impl Drop for ArenaElement {
    fn drop(&mut self) {
        unsafe {
            (self.drop)(self.value);
        }
    }
}

pub struct Arena {
    start: NonNull<u8>,
    offset: usize,
    elements: Vec<ArenaElement>,
    valid: Rc<Cell<bool>>,
}

impl Arena {
    pub fn new(size_in_bytes: usize) -> Self {
        unsafe {
            let layout = alloc::Layout::from_size_align(size_in_bytes, 1).unwrap();
            let ptr = alloc::alloc(layout);
            Self {
                start: NonNull::new_unchecked(ptr),
                offset: 0,
                elements: Vec::new(),
                valid: Rc::new(Cell::new(true)),
            }
        }
    }

    pub fn clear(&mut self) {
        self.valid.set(false);
        self.valid = Rc::new(Cell::new(true));
        self.elements.clear();
        self.offset = 0;
    }

    #[inline(always)]
    pub fn alloc<T>(&mut self, value: T) -> ArenaRef<T> {
        unsafe fn drop<T>(ptr: NonNull<u8>) {
            std::ptr::drop_in_place(ptr.cast::<T>().as_ptr());
        }

        unsafe {
            let layout = alloc::Layout::for_value(&value).pad_to_align();
            let ptr = NonNull::new_unchecked(self.start.as_ptr().add(self.offset).cast::<T>());
            ptr::write(ptr.as_ptr(), value);

            self.elements.push(ArenaElement {
                value: ptr.cast(),
                drop: drop::<T>,
            });
            self.offset += layout.size();
            ArenaRef {
                ptr,
                valid: self.valid.clone(),
            }
        }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        self.clear();
    }
}

pub struct ArenaRef<T: ?Sized> {
    ptr: NonNull<T>,
    valid: Rc<Cell<bool>>,
}

impl<T: ?Sized> Clone for ArenaRef<T> {
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr,
            valid: self.valid.clone(),
        }
    }
}

impl<T: ?Sized> ArenaRef<T> {
    pub fn map<U: ?Sized>(mut self, f: impl FnOnce(&mut T) -> &mut U) -> ArenaRef<U> {
        ArenaRef {
            ptr: unsafe { NonNull::new_unchecked(f(&mut *self)) },
            valid: self.valid,
        }
    }

    fn validate(&self) {
        assert!(
            self.valid.get(),
            "attempted to dereference an ArenaRef after its Arena was cleared"
        );
    }
}

impl<T: ?Sized> Deref for ArenaRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.validate();
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for ArenaRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.validate();
        unsafe { self.ptr.as_mut() }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    use super::*;

    #[test]
    fn test_arena() {
        let mut arena = Arena::new(1024);
        let a = arena.alloc(1u64);
        let b = arena.alloc(2u32);
        let c = arena.alloc(3u16);
        let d = arena.alloc(4u8);
        assert_eq!(*a, 1);
        assert_eq!(*b, 2);
        assert_eq!(*c, 3);
        assert_eq!(*d, 4);

        arena.clear();
        let a = arena.alloc(5u64);
        let b = arena.alloc(6u32);
        let c = arena.alloc(7u16);
        let d = arena.alloc(8u8);
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
        arena.alloc(DropGuard(dropped.clone()));
        arena.clear();
        assert!(dropped.get());
    }
}
