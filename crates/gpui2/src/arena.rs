use std::{
    alloc,
    ptr::{self, NonNull},
};

pub struct Arena {
    start: NonNull<u8>,
    offset: usize,
    elements: Vec<ArenaElement>,
}

impl Default for Arena {
    fn default() -> Self {
        unsafe {
            let layout = alloc::Layout::from_size_align(16 * 1024 * 1024, 1).unwrap();
            let ptr = alloc::alloc(layout);
            Self {
                start: NonNull::new_unchecked(ptr),
                offset: 0,
                elements: Vec::new(),
            }
        }
    }
}

struct ArenaElement {
    value: NonNull<u8>,
    drop: unsafe fn(NonNull<u8>),
}

impl Arena {
    pub fn clear(&mut self) {
        for element in self.elements.drain(..) {
            unsafe {
                (element.drop)(element.value);
            }
        }
        self.offset = 0;
    }

    #[inline(always)]
    pub fn alloc<T>(&mut self, value: T) -> ArenaRef<T> {
        unsafe fn drop<T>(ptr: NonNull<u8>) {
            std::ptr::drop_in_place(ptr.cast::<T>().as_ptr());
        }

        unsafe {
            let layout = alloc::Layout::for_value(&value).pad_to_align();
            let value_ptr = self.start.as_ptr().add(self.offset).cast::<T>();
            ptr::write(value_ptr, value);

            let value = NonNull::new_unchecked(value_ptr);
            self.elements.push(ArenaElement {
                value: value.cast(),
                drop: drop::<T>,
            });
            self.offset += layout.size();
            ArenaRef(value)
        }
    }
}

pub struct ArenaRef<T: ?Sized>(NonNull<T>);

impl<T: ?Sized> Copy for ArenaRef<T> {}

impl<T: ?Sized> Clone for ArenaRef<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T: ?Sized> ArenaRef<T> {
    pub unsafe fn map<U: ?Sized>(mut self, f: impl FnOnce(&mut T) -> &mut U) -> ArenaRef<U> {
        let u = f(self.get_mut());
        ArenaRef(NonNull::new_unchecked(u))
    }

    pub unsafe fn get(&self) -> &T {
        self.0.as_ref()
    }

    pub unsafe fn get_mut(&mut self) -> &mut T {
        self.0.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    use super::*;

    #[test]
    fn test_arena() {
        let mut arena = Arena::default();
        let mut a = arena.alloc(1u64);
        let mut b = arena.alloc(2u32);
        let mut c = arena.alloc(3u16);
        let mut d = arena.alloc(4u8);
        assert_eq!(unsafe { *a.get_mut() }, 1);
        assert_eq!(unsafe { *b.get_mut() }, 2);
        assert_eq!(unsafe { *c.get_mut() }, 3);
        assert_eq!(unsafe { *d.get_mut() }, 4);

        arena.clear();
        let mut a = arena.alloc(5u64);
        let mut b = arena.alloc(6u32);
        let mut c = arena.alloc(7u16);
        let mut d = arena.alloc(8u8);
        assert_eq!(unsafe { *a.get_mut() }, 5);
        assert_eq!(unsafe { *b.get_mut() }, 6);
        assert_eq!(unsafe { *c.get_mut() }, 7);
        assert_eq!(unsafe { *d.get_mut() }, 8);

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
