use std::{
    alloc::{self, handle_alloc_error},
    cell::Cell,
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    rc::Rc,
};

struct ArenaElement {
    value: *mut u8,
    drop: unsafe fn(*mut u8),
}

impl Drop for ArenaElement {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { (self.drop)(self.value) };
    }
}

struct Chunk {
    start: *mut u8,
    end: *mut u8,
    offset: *mut u8,
}

impl Drop for Chunk {
    fn drop(&mut self) {
        unsafe {
            let chunk_size = self.end.offset_from_unsigned(self.start);
            // SAFETY: This succeeded during allocation.
            let layout = alloc::Layout::from_size_align_unchecked(chunk_size, 1);
            alloc::dealloc(self.start, layout);
        }
    }
}

impl Chunk {
    fn new(chunk_size: NonZeroUsize) -> Self {
        // this only fails if chunk_size is unreasonably huge
        let layout = alloc::Layout::from_size_align(chunk_size.get(), 1).unwrap();
        let start = unsafe { alloc::alloc(layout) };
        if start.is_null() {
            handle_alloc_error(layout);
        }
        let end = unsafe { start.add(chunk_size.get()) };
        Self {
            start,
            end,
            offset: start,
        }
    }

    fn allocate(&mut self, layout: alloc::Layout) -> Option<NonNull<u8>> {
        // Compute the allocation bounds in integer address space so that the
        // bounds check happens before any pointer offsetting. Offsetting a
        // pointer past the end of its allocation is undefined behavior even if
        // the result is never dereferenced (as happens on the chunk-spill
        // path), so `ptr::add` cannot be used until we know the result stays
        // in bounds. `checked_add` also handles the documented case where
        // `align_offset` returns `usize::MAX`.
        let base = self.offset.addr();
        let aligned_addr = base.checked_add(self.offset.align_offset(layout.align()))?;
        let next_addr = aligned_addr.checked_add(layout.size())?;

        if next_addr <= self.end.addr() {
            let aligned = self.offset.with_addr(aligned_addr);
            self.offset = self.offset.with_addr(next_addr);
            NonNull::new(aligned)
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.offset = self.start;
    }
}

pub struct Arena {
    chunks: Vec<Chunk>,
    elements: Vec<ArenaElement>,
    valid: Rc<Cell<bool>>,
    current_chunk_index: usize,
    chunk_size: NonZeroUsize,
    scope_depth: usize,
}

impl Drop for Arena {
    fn drop(&mut self) {
        self.force_clear();
    }
}

impl Arena {
    pub fn new(chunk_size: usize) -> Self {
        let chunk_size = NonZeroUsize::try_from(chunk_size).unwrap();
        Self {
            chunks: vec![Chunk::new(chunk_size)],
            elements: Vec::new(),
            valid: Rc::new(Cell::new(true)),
            current_chunk_index: 0,
            chunk_size,
            scope_depth: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.chunks.len() * self.chunk_size.get()
    }

    /// Marks the start of a scope (e.g. a window draw) whose allocations must stay
    /// live until the scope ends, even if `clear` is called by a nested scope in
    /// the meantime.
    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    /// Ends the innermost scope started with `begin_scope`.
    ///
    /// Panics if no scope is active: an unbalanced `end_scope` would let `clear`
    /// run while an enclosing scope still references arena memory, which is
    /// exactly the use-after-free this bookkeeping exists to prevent, so failing
    /// loudly here is preferable.
    pub fn end_scope(&mut self) {
        self.scope_depth = self
            .scope_depth
            .checked_sub(1)
            .expect("Arena::end_scope called without a matching begin_scope");
    }

    /// Drops all allocations and resets the arena, unless a scope is still active.
    ///
    /// When a draw triggers a nested draw (e.g. re-entrant window procedure
    /// invocations on Windows, or opening a window from within a draw), the nested
    /// draw's clear must not free memory the outer draw still references, so it is
    /// deferred: the outer draw's own clear will drop both draws' allocations.
    pub fn clear(&mut self) {
        if self.scope_depth == 0 {
            self.force_clear();
        } else {
            log::debug!(
                "deferring arena clear; {} enclosing scope(s) still active",
                self.scope_depth
            );
        }
    }

    fn force_clear(&mut self) {
        self.valid.set(false);
        self.valid = Rc::new(Cell::new(true));
        self.elements.clear();
        for chunk_index in 0..=self.current_chunk_index {
            self.chunks[chunk_index].reset();
        }
        self.current_chunk_index = 0;
    }

    #[inline(always)]
    pub fn alloc<T>(&mut self, f: impl FnOnce() -> T) -> ArenaBox<T> {
        self.alloc_mapped(f, |value| value)
    }

    #[inline(always)]
    pub fn alloc_mapped<T, U: ?Sized>(
        &mut self,
        f: impl FnOnce() -> T,
        map: impl FnOnce(&mut T) -> &mut U,
    ) -> ArenaBox<U> {
        #[inline(always)]
        unsafe fn inner_writer<T, F>(ptr: *mut T, f: F)
        where
            F: FnOnce() -> T,
        {
            unsafe { ptr::write(ptr, f()) };
        }

        unsafe fn drop<T>(ptr: *mut u8) {
            unsafe { std::ptr::drop_in_place(ptr.cast::<T>()) };
        }

        let layout = alloc::Layout::new::<T>();
        let current_chunk = &mut self.chunks[self.current_chunk_index];
        let ptr = if let Some(ptr) = current_chunk.allocate(layout) {
            ptr.as_ptr()
        } else {
            self.allocate_next_chunk(layout)
        };

        let typed_ptr = ptr.cast::<T>();
        unsafe { inner_writer(typed_ptr, f) };
        self.elements.push(ArenaElement {
            value: ptr,
            drop: drop::<T>,
        });

        let mapped = NonNull::from(map(unsafe { &mut *typed_ptr }));
        ArenaBox {
            ptr: mapped,
            valid: Some(self.valid.clone()),
        }
    }

    #[inline(never)]
    fn allocate_next_chunk(&mut self, layout: alloc::Layout) -> *mut u8 {
        self.current_chunk_index += 1;
        if self.current_chunk_index >= self.chunks.len() {
            self.chunks.push(Chunk::new(self.chunk_size));
            assert_eq!(self.current_chunk_index, self.chunks.len() - 1);
            log::trace!(
                "increased element arena capacity to {}kb",
                self.capacity() / 1024,
            );
        }
        let current_chunk = &mut self.chunks[self.current_chunk_index];
        if let Some(pointer) = current_chunk.allocate(layout) {
            pointer.as_ptr()
        } else {
            panic!(
                "Arena chunk_size of {} is too small to allocate {} bytes",
                self.chunk_size,
                layout.size()
            );
        }
    }
}

pub struct ArenaBox<T: ?Sized> {
    ptr: NonNull<T>,
    /// `Some` when `ptr` lives in an [`Arena`], whose `force_clear` drops the value;
    /// `None` when `ptr` owns a heap `Box<T>` directly.
    valid: Option<Rc<Cell<bool>>>,
}

const _: () = {
    assert!(size_of::<ArenaBox<u8>>() == 2 * size_of::<usize>());
    assert!(size_of::<ArenaBox<dyn std::any::Any>>() == 3 * size_of::<usize>());
};

impl<T: ?Sized> ArenaBox<T> {
    #[inline]
    pub fn from_box(value: Box<T>) -> Self {
        Self {
            ptr: Box::into_non_null(value),
            valid: None,
        }
    }

    #[track_caller]
    #[inline(always)]
    fn validate(&self) {
        if let Some(valid) = &self.valid
            && !valid.get()
        {
            cold_invalid_arena_ref();
        }
    }
}

#[cold]
#[track_caller]
fn cold_invalid_arena_ref() -> ! {
    panic!("attempted to dereference an ArenaRef after its Arena was cleared");
}

impl<T: ?Sized> Drop for ArenaBox<T> {
    #[inline]
    fn drop(&mut self) {
        if self.valid.is_none() {
            #[cold]
            unsafe fn drop_boxed<T: ?Sized>(ptr: NonNull<T>) {
                // SAFETY: Caller guarantees `ptr` came from `Box::into_raw(Box<T>)`.
                unsafe {
                    drop(Box::from_raw(ptr.as_ptr()));
                }
            }

            // SAFETY: `self.valid` is `None` only when constructed by `ArenaBox::from_box`,
            // with `self.ptr` produced by `Box::into_non_null` for the exact type `T`.
            unsafe { drop_boxed(self.ptr) };
        }
    }
}

impl<T: ?Sized> Deref for ArenaBox<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.validate();
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for ArenaBox<T> {
    #[inline(always)]
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
    fn test_arena_grow() {
        let mut arena = Arena::new(8);
        arena.alloc(|| 1u64);
        arena.alloc(|| 2u64);

        assert_eq!(arena.capacity(), 16);

        arena.alloc(|| 3u32);
        arena.alloc(|| 4u32);

        assert_eq!(arena.capacity(), 24);
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

        assert_eq!(x1.ptr.as_ptr().align_offset(std::mem::align_of_val(&*x1)), 0);
        assert_eq!(x2.ptr.as_ptr().align_offset(std::mem::align_of_val(&*x2)), 0);
    }

    #[test]
    #[should_panic(expected = "attempted to dereference an ArenaRef after its Arena was cleared")]
    fn test_arena_use_after_clear() {
        let mut arena = Arena::new(16);
        let value = arena.alloc(|| 1u64);

        arena.clear();
        let _read_value = *value;
    }

    #[test]
    fn test_clear_deferred_while_scope_active() {
        struct DropCounter(Rc<Cell<usize>>);
        impl Drop for DropCounter {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        let drops = Rc::new(Cell::new(0));
        let mut arena = Arena::new(1024);

        // Outer draw starts and allocates.
        arena.begin_scope();
        let outer = arena.alloc(|| 42u64);
        arena.alloc({
            let drops = drops.clone();
            || DropCounter(drops)
        });

        // Nested draw runs to completion and requests a clear.
        arena.begin_scope();
        let inner = arena.alloc(|| 7u64);
        arena.alloc({
            let drops = drops.clone();
            || DropCounter(drops)
        });
        arena.end_scope();
        arena.clear();

        // The clear must be deferred: the outer draw's allocations are still live.
        assert_eq!(*outer, 42);
        assert_eq!(*inner, 7);
        assert_eq!(drops.get(), 0);

        // Once the outer draw finishes, its clear drops both draws' allocations.
        arena.end_scope();
        arena.clear();
        assert_eq!(drops.get(), 2);
    }

    #[test]
    fn test_clear_without_scope_is_immediate() {
        let mut arena = Arena::new(1024);
        let value = arena.alloc(|| 1u64);
        assert_eq!(*value, 1);
        arena.clear();
        assert!(!value.valid.as_ref().unwrap().get());
    }

    #[test]
    fn test_heap_arena_box_drops_on_drop() {
        struct DropCounter(Rc<Cell<usize>>);
        impl Drop for DropCounter {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        let drops = Rc::new(Cell::new(0));
        let mut boxed = ArenaBox::from_box(Box::new((41u64, DropCounter(drops.clone()))));
        boxed.0 += 1;
        assert_eq!(boxed.0, 42);
        assert_eq!(drops.get(), 0);
        drop(boxed);
        assert_eq!(drops.get(), 1);

        let unsized_box: ArenaBox<dyn std::any::Any> =
            ArenaBox::from_box(Box::new(DropCounter(drops.clone())));
        assert!(unsized_box.downcast_ref::<DropCounter>().is_some());
        drop(unsized_box);
        assert_eq!(drops.get(), 2);
    }

    #[test]
    #[should_panic(expected = "Arena::end_scope called without a matching begin_scope")]
    fn test_unbalanced_end_scope_panics() {
        let mut arena = Arena::new(1024);
        arena.begin_scope();
        arena.end_scope();
        arena.end_scope();
    }
}
