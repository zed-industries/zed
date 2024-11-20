use parking_lot::Mutex;
use std::{
    alloc,
    cell::Cell,
    ops::{Deref, DerefMut},
    ptr,
    rc::Rc,
    sync::{
        atomic::{self, AtomicPtr},
        Arc,
    },
    thread,
    time::Duration,
};

/// An arena allocator with the following properties:
///
/// * Support for mixed-type values.
/// * Can only allocate from one thread.
/// * Can invalidate + trash all current values in constant time.
/// * Support for concurrently dropping trash from a thread other than the allocating thread.
///
/// The internal layout of is a circular buffer (ring buffer) that looks like this:
///
/// ```
/// <       tttttttttttttvvvvvvvvvvv            >
///  ^      ^            ^          ^           ^
///  start  trash_start  trash_end  next_write  end
/// ```
///
/// Illustrating the circular nature of the buffer where the values wrap around:
///
/// ```
/// <vvvvvvv             tttttttttttvvvvvvvvvvvv>
///  ^      ^            ^          ^           ^
///  start  next_write trash_start trash_end end
/// ```
pub struct Arena {
    shared: Arc<SharedState>,
    next_write: *mut u8,
    valid: Rc<Cell<bool>>,
}

/// Mutable state shared by `Arena` and `TrashHandle`.
struct SharedState {
    start: AtomicPtr<u8>,
    end: AtomicPtr<u8>,
    trash_start: AtomicPtr<u8>,
    trash_end: AtomicPtr<u8>,
    drop_trash_is_running: Mutex<()>,
}

/// `TrashHandle` is used to drop trashed values from the arena. It can be used from threads other
/// than the allocating thread.
pub struct TrashHandle(Arc<SharedState>);

impl TrashHandle {
    pub fn drop_trash(&self) {
        self.0.drop_trash_if_not_already_running();
    }
}

impl Arena {
    pub fn new(size_in_bytes: usize) -> Self {
        unsafe {
            let layout = alloc::Layout::from_size_align(size_in_bytes, 1).unwrap();
            let start = alloc::alloc(layout);
            let end = start.add(size_in_bytes);
            let shared = Arc::new(SharedState {
                start: AtomicPtr::new(start),
                end: AtomicPtr::new(end),
                trash_start: AtomicPtr::new(start),
                trash_end: AtomicPtr::new(start),
                drop_trash_is_running: Mutex::new(()),
            });
            Self {
                shared,
                next_write: start,
                valid: Rc::new(Cell::new(true)),
            }
        }
    }

    pub fn len(&self) -> usize {
        self.next_write as usize - self.shared.start() as usize
    }

    pub fn capacity(&self) -> usize {
        self.shared.end() as usize - self.shared.start() as usize
    }

    pub fn trash_everything(&mut self) -> TrashHandle {
        self.valid.set(false);
        self.valid = Rc::new(Cell::new(true));
        self.shared
            .trash_end
            .store(self.next_write, atomic::Ordering::Relaxed);
        TrashHandle(self.shared.clone())
    }

    #[inline(always)]
    pub fn alloc<T>(&mut self, f: impl FnOnce() -> T) -> ArenaBox<T> {
        unsafe fn drop<T>(ptr: *mut u8) {
            ptr::drop_in_place(ptr.cast::<T>());
        }

        unsafe {
            let (header_ptr, offset) =
                self.get_wrapped_allocation_ptrs::<ArenaElementHeader>(self.next_write);
            let (value_ptr, offset) = self.get_wrapped_allocation_ptrs::<T>(offset);
            assert!(offset <= self.shared.end(), "not enough space in Arena");
            let trash_start = self.shared.trash_start();
            let trash_end = self.shared.trash_end();
            assert!(
                // FIXME: this is broken.
                trash_start == trash_end
                    || ptr_is_within_circular_interval(offset, self.next_write, trash_start),
                "not enough space in Arena"
            );
            self.shared.ensure_no_trash_before(offset);

            ptr::write(
                header_ptr,
                ArenaElementHeader {
                    next_element: offset,
                    value: AtomicPtr::new(value_ptr.cast()),
                    drop: drop::<T>,
                },
            );
            ptr::write(value_ptr, f());

            self.next_write = offset;

            ArenaBox {
                ptr: value_ptr,
                valid: self.valid.clone(),
            }
        }
    }

    #[inline(always)]
    unsafe fn get_wrapped_allocation_ptrs<T>(&self, offset: *mut u8) -> (*mut T, *mut u8) {
        let (ptr, next_offset) = get_allocation_ptrs::<T>(offset);
        if next_offset < self.shared.end() {
            (ptr, next_offset)
        } else if next_offset == self.shared.end() {
            (ptr, self.shared.start())
        } else {
            get_allocation_ptrs::<T>(self.shared.start())
        }
    }
}

impl Drop for SharedState {
    fn drop(&mut self) {
        self.drop_trash_ensure_finishes();
    }
}

impl SharedState {
    fn drop_trash_if_not_already_running(&self) {
        self.drop_trash_impl(false);
    }

    fn drop_trash_ensure_finishes(&self) {
        self.drop_trash_impl(true);
    }

    fn drop_trash_impl(&self, ensure_finishes: bool) {
        if let Some(_guard) = self.drop_trash_is_running.try_lock() {
            let trash_start = self.trash_start();
            let mut offset = self.trash_end();
            // trash_end is re-queried as this might run concurrently with multiple trash_everything calls.
            //
            // FIXME: what if there is nothing to trash / nothing was allocated?
            while ptr_is_within_circular_interval(offset, trash_start, self.trash_end()) {
                unsafe {
                    offset = drop_arena_element(offset);
                }
                self.trash_start.store(offset, atomic::Ordering::Relaxed);
            }
        } else if ensure_finishes {
            unsafe {
                // If we can't get the lock, help out with trashing.
                self.ensure_no_trash_before(self.trash_end());
            }
        }
    }

    unsafe fn ensure_no_trash_before(&self, target: *mut u8) {
        let mut offset = self.trash_start();
        let mut trash_end = self.trash_end();
        if ptr_is_within_circular_interval(target, offset, trash_end) {
            log::warn!("Unexpectedly needed to clear element arena trash on main thread.");
            while ptr_is_within_circular_interval(target, offset, trash_end) {
                // Note that it is possible for the background thread to increment trash_start
                // beyond this offset while the drop is still doing work. This is fine because this
                // code is run on the same thread as allocations, so allocations will only see a valid
                // trash_start.
                offset = drop_arena_element(offset);
            }
        }

        // FIXME: won't make progress if background thread is starved. If ArenaElementHeader stores
        // a state indicating that the drop has happened this can possibly be fixed.
        if ptr_is_within_circular_interval(target, self.trash_start(), trash_end) {
            log::error!(
                "Needed to spin waiting for background thread to clear element arena trash."
            );
            while ptr_is_within_circular_interval(target, self.trash_start(), trash_end) {
                // TODO: revisit choice of sleep duration
                thread::sleep(Duration::from_micros(50));
            }
        }
    }

    fn start(&self) -> *mut u8 {
        self.start.load(atomic::Ordering::Relaxed)
    }

    fn end(&self) -> *mut u8 {
        self.end.load(atomic::Ordering::Relaxed)
    }

    fn trash_start(&self) -> *mut u8 {
        self.trash_start.load(atomic::Ordering::Relaxed)
    }

    fn trash_end(&self) -> *mut u8 {
        self.trash_end.load(atomic::Ordering::Relaxed)
    }
}

#[inline(always)]
unsafe fn get_allocation_ptrs<T>(offset: *mut u8) -> (*mut T, *mut u8) {
    let data_layout = alloc::Layout::new::<T>();
    let offset = offset.add(offset.align_offset(data_layout.align()));
    let data_ptr = offset.cast::<T>();
    (data_ptr, offset.add(data_layout.size()))
}

// FIXME: many uses of this only actively use one of the bounds.
//
// FIXME: start == end could either mean everything is included or nothing. Here it is nothing
// included, but are there corner cases where the actual state is everything?
#[inline(always)]
fn ptr_is_within_circular_interval<T>(ptr: *const T, start: *const T, end: *const T) -> bool {
    if end < start {
        ptr >= start || ptr < end
    } else {
        ptr >= start && ptr < end
    }
}

struct ArenaElementHeader {
    next_element: *mut u8,
    value: AtomicPtr<u8>,
    drop: unsafe fn(*mut u8),
}

unsafe fn drop_arena_element(offset: *mut u8) -> *mut u8 {
    let (header, _) = get_allocation_ptrs::<ArenaElementHeader>(offset);
    let value = (*header)
        .value
        .swap(ptr::null_mut(), atomic::Ordering::AcqRel);
    if value.is_null() {
        return (*header).next_element;
    }
    let drop_fn = (*header).drop;
    (drop_fn)(value);
    (*header).next_element
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

        arena.trash_everything().drop_trash();
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
        arena.trash_everything().drop_trash();
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

        arena.trash_everything().drop_trash();
        let _read_value = *value;
    }
}
