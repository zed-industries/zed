use core::{cmp, mem, ptr};
use core::alloc::Layout;
use core::any::TypeId;
use core::mem::size_of;
use crate::any_value::{AnyValue, Unknown, AnyValueSizeless};
use crate::assert_types_equal;
use crate::clone_type::CloneFn;
use crate::mem::{Mem, MemBuilder, MemResizable};

pub type DropFn = unsafe fn(ptr: *mut u8, len: usize);

pub struct AnyVecRaw<M: MemBuilder> {
    pub(crate) mem_builder: M,         // usually ZST
    pub(crate) mem: M::Mem,
    pub(crate) len: usize,  // in elements
    pub(crate) type_id: TypeId,        // purely for safety checks
    pub(crate) drop_fn: Option<DropFn>
}

impl<M: MemBuilder> AnyVecRaw<M> {
    #[inline]
    pub fn new<T: 'static>(mem_builder: M, mem: M::Mem) -> Self {
        Self{
            mem_builder,
            mem,
            len: 0,
            type_id: TypeId::of::<T>(),
            drop_fn:
                if !mem::needs_drop::<T>(){
                    None
                } else{
                    Some(|mut ptr: *mut u8, len: usize|{
                        for _ in 0..len{
                            unsafe{
                                ptr::drop_in_place(ptr as *mut T);
                                ptr = ptr.add(mem::size_of::<T>());
                            }
                        }
                    })
                }
        }
    }

    #[inline]
    pub(crate) fn clone_empty(&self) -> Self {
        self.clone_empty_in(self.mem_builder.clone())
    }

    #[inline]
    pub(crate) fn clone_empty_in<NewM: MemBuilder>(&self, mut mem_builder: NewM) -> AnyVecRaw<NewM>{
        let mem = mem_builder.build(self.element_layout());
        AnyVecRaw{
            mem_builder,
            mem,
            len: 0,
            type_id: self.type_id,
            drop_fn: self.drop_fn,
        }
    }

    /// Unsafe, because type cloneability is not checked
    pub(crate) unsafe fn clone(&self, clone_fn: CloneFn) -> Self {
        // 1. construct empty "prototype"
        let mut cloned = self.clone_empty();

        // 2. allocate
        cloned.mem.expand(self.len);

        // 3. copy/clone
        {
            let src = self.mem.as_ptr();
            let dst = cloned.mem.as_mut_ptr();
            (clone_fn)(src, dst, self.len);
        }

        // 4. set len
        cloned.len = self.len;
        cloned
    }

    #[inline]
    pub(crate) fn index_check(&self, index: usize){
        assert!(index < self.len, "Index out of range!");
    }

    #[inline]
    pub(crate) fn type_check<V: AnyValue>(&self, value: &V){
        assert_types_equal(value.value_typeid(), self.type_id);
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> *const u8{
        self.mem.as_ptr().add(self.element_layout().size() * index)
    }

    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> *mut u8{
        self.mem.as_mut_ptr().add(self.element_layout().size() * index)
    }

    #[cold]
    #[inline(never)]
    fn expand_one(&mut self){
        self.mem.expand(1);
    }

    #[inline]
    /*pub(crate)*/ fn reserve_one(&mut self){
        if self.len == self.capacity(){
            self.expand_one();
        }
    }

    /// Leave this, for Mem, because implementation need it.
    /// If M::Mem does not implement MemResizable, then `expand`
    /// will panic, if out of capacity.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        let new_len = self.len + additional;
        if self.capacity() < new_len{
            self.mem.expand(new_len - self.capacity());
        }
    }

    #[inline]
    pub fn reserve_exact(&mut self, additional: usize)
        where M::Mem: MemResizable
    {
        let new_len = self.len + additional;
        if self.capacity() < new_len{
            self.mem.expand_exact(new_len - self.capacity());
        }
    }

    pub fn shrink_to_fit(&mut self)
        where M::Mem: MemResizable
    {
        self.mem.resize(self.len);
    }

    pub fn shrink_to(&mut self, min_capacity: usize)
        where M::Mem: MemResizable
    {
        let new_len = cmp::max(self.len, min_capacity);
        self.mem.resize(new_len);
    }

    #[inline]
    pub unsafe fn set_len(&mut self, new_len: usize) {
        debug_assert!(new_len <= self.capacity());
        self.len = new_len;
    }

    /// # Safety
    ///
    /// Type is not checked.
    pub unsafe fn insert_unchecked<V: AnyValueSizeless>(&mut self, index: usize, value: V) {
        assert!(index <= self.len, "Index out of range!");

        self.reserve_one();

        // Compile time type optimization
        if !Unknown::is::<V::Type>(){
            let element = self.mem.as_mut_ptr().cast::<V::Type>().add(index);

            // 1. shift right
            ptr::copy(
                element,
                element.add(1),
                self.len - index
            );

            // 2. write value
            value.move_into::<V::Type>(element as *mut u8, size_of::<V::Type>());
        } else {
            let element_size = self.element_layout().size();
            let element = self.mem.as_mut_ptr().add(element_size * index);

            // 1. shift right
            crate::copy_bytes(
                element,
                element.add(element_size),
                element_size * (self.len - index)
            );

            // 2. write value
            value.move_into::<V::Type>(element, element_size);
        }

        self.len += 1;
    }

    /// # Safety
    ///
    /// Type is not checked.
    #[inline]
    pub unsafe fn push_unchecked<V: AnyValueSizeless>(&mut self, value: V) {
        self.reserve_one();

        // Compile time type optimization
        if !Unknown::is::<V::Type>(){
            let element = self.mem.as_mut_ptr().cast::<V::Type>().add(self.len) as *mut u8;
            value.move_into::<V::Type>(element, size_of::<V::Type>());
        } else {
            let element_size = self.element_layout().size();
            let element = self.mem.as_mut_ptr().add(element_size * self.len);
            value.move_into::<V::Type>(element, element_size);
        };

        self.len += 1;
    }

    #[inline]
    pub fn clear(&mut self){
        let len = self.len;

        // Prematurely set the length to zero so that even if dropping the values panics users
        // won't be able to access the dropped values.
        self.len = 0;

        if let Some(drop_fn) = self.drop_fn{
            unsafe{
                (drop_fn)(self.mem.as_mut_ptr(), len);
            }
        }
    }

    /// Element Layout
    #[inline]
    pub fn element_layout(&self) -> Layout {
        self.mem.element_layout()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.mem.size()
    }
}

impl<M: MemBuilder> Drop for AnyVecRaw<M> {
    #[inline]
    fn drop(&mut self) {
        self.clear();
    }
}
