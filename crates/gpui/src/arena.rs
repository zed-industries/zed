use parking_lot::Mutex;
use std::{
    alloc,
    cell::Cell,
    ops::{Deref, DerefMut},
    ptr,
    rc::Rc,
    sync::atomic,
    sync::atomic::AtomicPtr,
};

struct ArenaElementHeader {
    next_element: *mut u8,
    value: *mut u8,
    drop_mutex: Mutex<unsafe fn(*mut u8)>,
}

pub struct Arena {
    start: *mut u8,
    end: *mut u8,
    next_write: AtomicPtr<u8>,
    trash_start: AtomicPtr<u8>,
    trash_end: AtomicPtr<u8>,
    valid: Rc<Cell<bool>>,
}

pub struct MustDropTrash();

impl Arena {
    pub fn new(size_in_bytes: usize) -> Self {
        unsafe {
            let layout = alloc::Layout::from_size_align(size_in_bytes, 1).unwrap();
            let start = alloc::alloc(layout);
            let end = start.add(size_in_bytes);
            Self {
                start,
                end,
                next_write: AtomicPtr::new(start),
                trash_start: AtomicPtr::new(start),
                trash_end: AtomicPtr::new(start),
                valid: Rc::new(Cell::new(true)),
            }
        }
    }

    pub fn len(&self) -> usize {
        self.next_write.load(atomic::Ordering::Relaxed) as usize - self.start as usize
    }

    pub fn capacity(&self) -> usize {
        self.end as usize - self.start as usize
    }

    pub fn clear(&mut self) {
        let must_drop_trash = self.trash_everything();
        self.drop_trash(must_drop_trash);
    }

    pub fn trash_everything(&mut self) -> MustDropTrash {
        self.valid.set(false);
        self.valid = Rc::new(Cell::new(true));
        let trash_end = self.trash_end.load(atomic::Ordering::Relaxed);
        unsafe {
            // Help finish last `drop_trash` if still ongoing. This is so that there is only one
            // concurrent `drop_trash` happening at a time.
            self.ensure_no_trash_before(trash_end);
        }
        let next_write = self.next_write.load(atomic::Ordering::Relaxed);
        let trash_start = self.trash_start.load(atomic::Ordering::Relaxed);
        assert!(trash_start == trash_end);
        self.trash_end.store(next_write, atomic::Ordering::Relaxed);
        MustDropTrash()
    }

    pub fn drop_trash(&self, _: MustDropTrash) {
        let trash_start = self.trash_start.load(atomic::Ordering::Relaxed);
        let trash_end = self.trash_end.load(atomic::Ordering::Relaxed);
        let mut offset = trash_start;
        while ptr_is_within_circular_interval(offset, trash_start, trash_end) {
            unsafe {
                offset = drop_arena_element(offset);
            }
            self.trash_start.store(offset, atomic::Ordering::Relaxed);
        }
    }

    unsafe fn ensure_no_trash_before(&self, target: *mut u8) {
        let mut offset = self.trash_start.load(atomic::Ordering::Relaxed);
        let mut trash_end = self.trash_end.load(atomic::Ordering::Relaxed);
        if ptr_is_within_circular_interval(target, offset, trash_end) {
            log::warn!("Unexpectedly needed to clear element arena trash on main thread.");
            while ptr_is_within_circular_interval(target, offset, trash_end) {
                offset = drop_arena_element(offset);
            }
        }
    }

    #[inline(always)]
    pub fn alloc<T>(&mut self, f: impl FnOnce() -> T) -> ArenaBox<T> {
        unsafe fn drop<T>(ptr: *mut u8) {
            ptr::drop_in_place(ptr.cast::<T>());
        }

        unsafe {
            let next_write = self.next_write.load(atomic::Ordering::Relaxed);
            let trash_start = self.trash_start.load(atomic::Ordering::Relaxed);
            let trash_end = self.trash_end.load(atomic::Ordering::Relaxed);

            let (header_ptr, offset) =
                self.get_wrapped_allocation_ptrs::<ArenaElementHeader>(next_write);
            let (value_ptr, offset) = self.get_wrapped_allocation_ptrs::<T>(offset);
            assert!(offset <= self.end, "not enough space in Arena");
            assert!(
                !ptr_is_within_circular_interval(offset, trash_start, trash_end),
                "not enough space in Arena"
            );
            self.ensure_no_trash_before(offset);

            ptr::write(
                header_ptr,
                ArenaElementHeader {
                    next_element: offset,
                    value: value_ptr.cast(),
                    drop_mutex: Mutex::new(drop::<T>),
                },
            );
            ptr::write(value_ptr, f());

            self.next_write.store(offset, atomic::Ordering::Relaxed);

            ArenaBox {
                ptr: value_ptr,
                valid: self.valid.clone(),
            }
        }
    }

    #[inline(always)]
    unsafe fn get_wrapped_allocation_ptrs<T>(&self, offset: *mut u8) -> (*mut T, *mut u8) {
        let (ptr, next_offset) = get_allocation_ptrs::<T>(offset);
        if next_offset < self.end {
            (ptr, next_offset)
        } else if next_offset == self.end {
            (ptr, self.start)
        } else {
            get_allocation_ptrs::<T>(self.start)
        }
    }
}

#[inline(always)]
unsafe fn get_allocation_ptrs<T>(offset: *mut u8) -> (*mut T, *mut u8) {
    let data_layout = alloc::Layout::new::<T>();
    let offset = offset.add(offset.align_offset(data_layout.align()));
    let data_ptr = offset.cast::<T>();
    (data_ptr, offset.add(data_layout.size()))
}

// TODO: many uses of this only actively use one of the bounds.
#[inline(always)]
fn ptr_is_within_circular_interval<T>(ptr: *const T, start: *const T, end: *const T) -> bool {
    if end < start {
        ptr >= start || ptr < end
    } else {
        ptr >= start && ptr < end
    }
}

unsafe fn drop_arena_element(offset: *mut u8) -> *mut u8 {
    let (header, _) = get_allocation_ptrs::<ArenaElementHeader>(offset);
    if (*header).value.is_null() {
        return (*header).next_element;
    }
    let drop_fn = (*header).drop_mutex.lock();
    if (*header).value.is_null() {
        return (*header).next_element;
    }
    (drop_fn)((*header).value);
    (*header).value = ptr::null_mut();
    (*header).next_element
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
        let mut arena = Arena::new(1024);
        let a = arena.alloc(|| 1u64);
        let b = arena.alloc(|| 2u32);
        let c = arena.alloc(|| 3u16);
        let d = arena.alloc(|| 4u8);
        assert_eq!(*a, 1);
        assert_eq!(*b, 2);
        assert_eq!(*c, 3);
        assert_eq!(*d, 4);

        arena.clear();
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
    #[should_panic(expected = "not enough space in Arena")]
    fn test_arena_overflow() {
        let mut arena = Arena::new(16);
        arena.alloc(|| 1u64);
        arena.alloc(|| 2u64);
        // This should panic.
        arena.alloc(|| 3u64);
    }

    #[test]
    fn test_arena_alignment() {
        let mut arena = Arena::new(256);
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
    }

    #[test]
    #[should_panic(expected = "attempted to dereference an ArenaRef after its Arena was cleared")]
    fn test_arena_use_after_clear() {
        let mut arena = Arena::new(256);
        let value = arena.alloc(|| 1u64);

        arena.clear();
        let _read_value = *value;
    }
}
