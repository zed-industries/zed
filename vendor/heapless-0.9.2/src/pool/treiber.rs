use core::mem::ManuallyDrop;

#[cfg_attr(not(arm_llsc), path = "treiber/cas.rs")]
#[cfg_attr(arm_llsc, path = "treiber/llsc.rs")]
mod impl_;

pub use impl_::{AtomicPtr, NonNullPtr};

pub struct Stack<N>
where
    N: Node,
{
    top: AtomicPtr<N>,
}

impl<N> Stack<N>
where
    N: Node,
{
    pub const fn new() -> Self {
        Self {
            top: AtomicPtr::null(),
        }
    }

    /// # Safety
    /// - `node` must be a valid pointer
    /// - aliasing rules must be enforced by the caller. e.g, the same `node` may not be pushed more
    ///   than once
    /// - It must be valid to call `next()` on the `node`, meaning it must be properly initialized
    ///   for insertion into a stack/linked list
    pub unsafe fn push(&self, node: NonNullPtr<N>) {
        impl_::push(self, node);
    }

    pub fn try_pop(&self) -> Option<NonNullPtr<N>> {
        impl_::try_pop(self)
    }
}

pub trait Node: Sized {
    type Data;

    /// Returns a reference to the atomic pointer that stores the link to the next `Node`
    ///
    /// # Safety
    ///
    /// It must be valid to obtain a reference to the next link pointer, e.g. in the case of
    /// `UnionNode`, the `next` field must be properly initialized when calling this function
    unsafe fn next(&self) -> &AtomicPtr<Self>;

    /// Returns a mutable reference to the atomic pointer that stores the link to the next `Node`
    ///
    /// # Safety
    ///
    /// It must be valid to obtain a reference to the next link pointer, e.g. in the case of
    /// `UnionNode`, the `next` field must be properly initialized when calling this function
    #[allow(dead_code)] // used conditionally
    unsafe fn next_mut(&mut self) -> &mut AtomicPtr<Self>;
}

#[repr(C)]
pub union UnionNode<T> {
    next: ManuallyDrop<AtomicPtr<UnionNode<T>>>,
    pub data: ManuallyDrop<T>,
}

impl<T> UnionNode<T> {
    /// Returns a new `UnionNode` that does not contain data and is not linked to any other nodes.
    /// The return value of this function is guaranteed to have the `next` field properly
    /// initialized. Use this function if you want to insert a new `UnionNode` into a linked
    /// list
    pub const fn unlinked() -> Self {
        Self {
            next: ManuallyDrop::new(AtomicPtr::null()),
        }
    }
}

impl<T> Node for UnionNode<T> {
    type Data = T;

    unsafe fn next(&self) -> &AtomicPtr<Self> {
        // SAFETY: Caller ensures that `self.next` is properly initialized
        unsafe { &self.next }
    }

    unsafe fn next_mut(&mut self) -> &mut AtomicPtr<Self> {
        // SAFETY: Caller ensures that `self.next` is properly initialized
        unsafe { &mut self.next }
    }
}

pub struct StructNode<T> {
    pub next: ManuallyDrop<AtomicPtr<StructNode<T>>>,
    pub data: ManuallyDrop<T>,
}

impl<T> Node for StructNode<T> {
    type Data = T;

    unsafe fn next(&self) -> &AtomicPtr<Self> {
        &self.next
    }

    unsafe fn next_mut(&mut self) -> &mut AtomicPtr<Self> {
        &mut self.next
    }
}

#[cfg(test)]
mod tests {
    use core::mem;

    use super::*;

    #[test]
    fn node_is_never_zero_sized() {
        struct Zst;

        assert_ne!(mem::size_of::<UnionNode<Zst>>(), 0);
    }
}
