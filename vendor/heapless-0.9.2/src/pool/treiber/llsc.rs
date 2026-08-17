use core::{
    cell::UnsafeCell,
    ptr::{self, NonNull},
};

use super::{Node, Stack};

pub struct AtomicPtr<N>
where
    N: Node,
{
    inner: UnsafeCell<Option<NonNull<N>>>,
}

impl<N> AtomicPtr<N>
where
    N: Node,
{
    #[inline]
    pub const fn null() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }
}

pub struct NonNullPtr<N>
where
    N: Node,
{
    inner: NonNull<N>,
}

impl<N> NonNullPtr<N>
where
    N: Node,
{
    #[inline]
    pub fn as_ptr(&self) -> *mut N {
        self.inner.as_ptr().cast()
    }

    #[inline]
    pub fn from_static_mut_ref(ref_: &'static mut N) -> Self {
        Self {
            inner: NonNull::from(ref_),
        }
    }

    #[inline]
    pub unsafe fn from_ptr_unchecked(ptr: *mut N) -> Self {
        debug_assert!(!ptr.is_null(), "Pointer must be non-null");
        Self {
            inner: NonNull::new_unchecked(ptr),
        }
    }
}

impl<N> Clone for NonNullPtr<N>
where
    N: Node,
{
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<N> Copy for NonNullPtr<N> where N: Node {}

/// Pushes the given node on top of the stack
///
/// # Safety
///
/// - `node` must point to a node that is properly initialized for linking, i.e.
///   `node.as_mut().next_mut()` must be valid to call (see [`Node::next_mut`])
/// - `node` must be convertible to a reference (see [`NonNull::as_mut`])
pub unsafe fn push<N>(stack: &Stack<N>, mut node: NonNullPtr<N>)
where
    N: Node,
{
    let top_addr = ptr::addr_of!(stack.top) as *mut usize;

    loop {
        let top = arch::load_link(top_addr);

        node.inner
            .as_mut()
            // SAFETY: Caller guarantees that it is valid to call `next_mut`
            .next_mut()
            .inner
            .get()
            // SAFETY: The pointer comes from `AtomicPtr::inner`, which is valid for writes
            .write(NonNull::new(top as *mut _));

        if arch::store_conditional(node.inner.as_ptr() as usize, top_addr).is_ok() {
            break;
        }
    }
}

pub fn try_pop<N>(stack: &Stack<N>) -> Option<NonNullPtr<N>>
where
    N: Node,
{
    unsafe {
        let top_addr = ptr::addr_of!(stack.top) as *mut usize;

        loop {
            let top = arch::load_link(top_addr);

            if let Some(top) = NonNull::new(top as *mut N) {
                let next = &top.as_ref().next();

                if arch::store_conditional(
                    next.inner
                        .get()
                        .read()
                        .map(|non_null| non_null.as_ptr() as usize)
                        .unwrap_or_default(),
                    top_addr,
                )
                .is_ok()
                {
                    break Some(NonNullPtr { inner: top });
                }
            } else {
                arch::clear_load_link();

                break None;
            }
        }
    }
}

#[cfg(arm_llsc)]
mod arch {
    use core::arch::asm;

    #[inline(always)]
    pub fn clear_load_link() {
        unsafe { asm!("clrex", options(nomem, nostack)) }
    }

    /// # Safety
    ///
    /// - `addr` must be a valid pointer.
    #[inline(always)]
    pub unsafe fn load_link(addr: *const usize) -> usize {
        let value;
        asm!("ldrex {}, [{}]", out(reg) value, in(reg) addr, options(nostack));
        value
    }

    /// # Safety
    ///
    /// - `addr` must be a valid pointer.
    #[inline(always)]
    pub unsafe fn store_conditional(value: usize, addr: *mut usize) -> Result<(), ()> {
        let outcome: usize;
        asm!("strex {}, {}, [{}]", out(reg) outcome, in(reg) value, in(reg) addr, options(nostack));
        if outcome == 0 {
            Ok(())
        } else {
            Err(())
        }
    }
}
