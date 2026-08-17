use core::fmt;
use core::mem::ManuallyDrop;
use core::ops::Deref;
use core::ptr::NonNull;

use objc2::encode::{EncodeArguments, EncodeReturn};

use crate::abi::BlockHeader;
use crate::debug::debug_block_header;
use crate::{ffi, Block, IntoBlock, StackBlock};

/// A reference-counted Objective-C block that is stored on the heap.
///
/// This is a smart pointer that [`Deref`]s to [`Block`].
///
/// The generic type `F` must be a [`dyn`] [`Fn`] that implements the
/// [`BlockFn`] trait, just like described in [`Block`]'s documentation.
///
/// [`dyn`]: https://doc.rust-lang.org/std/keyword.dyn.html
/// [`BlockFn`]: crate::BlockFn
///
///
/// # Memory-layout
///
/// This is guaranteed to have the same size and alignment as a pointer to a
/// block (i.e. same size as `*const Block<A, R>`).
///
/// Additionally, it participates in the null-pointer optimization, that is,
/// `Option<RcBlock<A, R>>` is guaranteed to have the same size as
/// `RcBlock<A, R>`.
#[doc(alias = "MallocBlock")]
pub struct RcBlock<F: ?Sized> {
    // Covariant
    ptr: NonNull<Block<F>>,
}

impl<F: ?Sized> RcBlock<F> {
    /// Construct an `RcBlock` from the given block pointer by taking
    /// ownership.
    ///
    /// This will return `None` if the pointer is NULL.
    ///
    ///
    /// # Safety
    ///
    /// The given pointer must point to a valid block, the parameter and
    /// return types must be correct, and the block must have a +1 reference /
    /// retain count from somewhere else.
    ///
    /// Additionally, the block must be safe to call (or, if it is not, then
    /// you must treat every call to the block as `unsafe`).
    #[inline]
    pub unsafe fn from_raw(ptr: *mut Block<F>) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }

    /// Construct an `RcBlock` from the given block pointer.
    ///
    /// The block will be copied, and have its reference-count increased by
    /// one.
    ///
    /// This will return `None` if the pointer is NULL, or if an allocation
    /// failure occurred.
    ///
    /// See [`Block::copy`] for a safe alternative when you already know the
    /// block pointer is valid.
    ///
    ///
    /// # Safety
    ///
    /// The given pointer must point to a valid block, and the parameter and
    /// return types must be correct.
    ///
    /// Additionally, the block must be safe to call (or, if it is not, then
    /// you must treat every call to the block as `unsafe`).
    #[doc(alias = "Block_copy")]
    #[doc(alias = "_Block_copy")]
    #[inline]
    pub unsafe fn copy(ptr: *mut Block<F>) -> Option<Self> {
        let ptr: *mut Block<F> = unsafe { ffi::_Block_copy(ptr.cast()) }.cast();
        // SAFETY: We just copied the block, so the reference count is +1
        unsafe { Self::from_raw(ptr) }
    }
}

// TODO: Move so this appears first in the docs.
impl<F: ?Sized> RcBlock<F> {
    /// Construct a `RcBlock` with the given closure.
    ///
    /// The closure will be coped to the heap on construction.
    ///
    /// When the block is called, it will return the value that results from
    /// calling the closure.
    //
    // Note: Unsure if this should be #[inline], but I think it may be able to
    // benefit from not being so.
    pub fn new<'f, A, R, Closure>(closure: Closure) -> Self
    where
        A: EncodeArguments,
        R: EncodeReturn,
        Closure: IntoBlock<'f, A, R, Dyn = F>,
    {
        // SAFETY: The stack block is copied once below.
        //
        // Note: We could theoretically use `_NSConcreteMallocBlock`, and use
        // `malloc` ourselves to put the block on the heap, but that symbol is
        // not part of the public ABI, and may break in the future.
        //
        // Clang doesn't do this optimization either.
        // <https://github.com/llvm/llvm-project/blob/llvmorg-17.0.6/clang/lib/CodeGen/CGBlocks.cpp#L281-L284>
        let block = unsafe { StackBlock::new_no_clone(closure) };

        // Transfer ownership from the stack to the heap.
        let mut block = ManuallyDrop::new(block);
        let ptr: *mut StackBlock<'f, A, R, Closure> = &mut *block;
        let ptr: *mut Block<F> = ptr.cast();
        // SAFETY: The block will be moved to the heap, and we forget the
        // original block because the heap block will drop in our dispose
        // helper.
        unsafe { Self::copy(ptr) }.unwrap_or_else(|| rc_new_fail())
    }
}

impl<F: ?Sized> Clone for RcBlock<F> {
    /// Increase the reference-count of the block.
    #[doc(alias = "Block_copy")]
    #[doc(alias = "_Block_copy")]
    #[inline]
    fn clone(&self) -> Self {
        // SAFETY: The block pointer is valid, and its safety invariant is
        // upheld, since the only way to get an `RcBlock` in the first place
        // is through unsafe functions that requires these preconditions to be
        // upheld.
        unsafe { Self::copy(self.ptr.as_ptr()) }.unwrap_or_else(|| rc_clone_fail())
    }
}

// Intentionally not `#[track_caller]`, to keep the code-size smaller (as this
// error is very unlikely).
fn rc_new_fail() -> ! {
    // This likely means the system is out of memory.
    panic!("failed creating RcBlock")
}

// Intentionally not `#[track_caller]`, see above.
pub(crate) fn block_copy_fail() -> ! {
    // This likely means the system is out of memory.
    panic!("failed copying Block")
}

// Intentionally not `#[track_caller]`, see above.
fn rc_clone_fail() -> ! {
    unreachable!("cloning a RcBlock bumps the reference count, which should be infallible")
}

impl<F: ?Sized> Deref for RcBlock<F> {
    type Target = Block<F>;

    #[inline]
    fn deref(&self) -> &Block<F> {
        // SAFETY: The pointer is valid, as ensured by creation methods, and
        // will be so for as long as the `RcBlock` is, since that holds +1
        // reference count.
        unsafe { self.ptr.as_ref() }
    }
}

impl<F: ?Sized> Drop for RcBlock<F> {
    /// Release the block, decreasing the reference-count by 1.
    ///
    /// The `Drop` method of the underlying closure will be called once the
    /// reference-count reaches zero.
    #[doc(alias = "Block_release")]
    #[doc(alias = "_Block_release")]
    #[inline]
    fn drop(&mut self) {
        // SAFETY: The pointer has +1 reference count, as ensured by creation
        // methods.
        unsafe { ffi::_Block_release(self.ptr.as_ptr().cast()) };
    }
}

impl<F: ?Sized> fmt::Debug for RcBlock<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("RcBlock");
        let header = unsafe { self.ptr.cast::<BlockHeader>().as_ref() };
        debug_block_header(header, &mut f);
        f.finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use alloc::rc::Rc;
    use core::cell::OnceCell;

    use super::*;

    #[test]
    fn return_rc_block() {
        fn get_adder(x: i32) -> RcBlock<dyn Fn(i32) -> i32> {
            RcBlock::new(move |y| y + x)
        }

        let add2 = get_adder(2);
        assert_eq!(add2.call((5,)), 7);
        assert_eq!(add2.call((-1,)), 1);
    }

    #[test]
    fn rc_block_with_precisely_described_lifetimes() {
        fn args<'a, 'b>(
            f: impl Fn(&'a i32, &'b i32) + 'static,
        ) -> RcBlock<dyn Fn(&'a i32, &'b i32) + 'static> {
            RcBlock::new(f)
        }

        fn args_return<'a, 'b>(
            f: impl Fn(&'a i32) -> &'b i32 + 'static,
        ) -> RcBlock<dyn Fn(&'a i32) -> &'b i32 + 'static> {
            RcBlock::new(f)
        }

        fn args_entire<'a, 'b>(f: impl Fn(&'a i32) + 'b) -> RcBlock<dyn Fn(&'a i32) + 'b> {
            RcBlock::new(f)
        }

        fn return_entire<'a, 'b>(
            f: impl Fn() -> &'a i32 + 'b,
        ) -> RcBlock<dyn Fn() -> &'a i32 + 'b> {
            RcBlock::new(f)
        }

        let _ = args(|_, _| {});
        let _ = args_return(|x| x);
        let _ = args_entire(|_| {});
        let _ = return_entire(|| &5);
    }

    #[allow(dead_code)]
    fn covariant<'f>(b: RcBlock<dyn Fn() + 'static>) -> RcBlock<dyn Fn() + 'f> {
        b
    }

    #[test]
    fn allow_re_entrancy() {
        #[allow(clippy::type_complexity)]
        let block: Rc<OnceCell<RcBlock<dyn Fn(u32) -> u32>>> = Rc::new(OnceCell::new());

        let captured_block = block.clone();
        let fibonacci = move |n| {
            let captured_fibonacci = captured_block.get().unwrap();
            match n {
                0 => 0,
                1 => 1,
                n => captured_fibonacci.call((n - 1,)) + captured_fibonacci.call((n - 2,)),
            }
        };

        let block = block.get_or_init(|| RcBlock::new(fibonacci));

        assert_eq!(block.call((0,)), 0);
        assert_eq!(block.call((1,)), 1);
        assert_eq!(block.call((6,)), 8);
        assert_eq!(block.call((10,)), 55);
        assert_eq!(block.call((19,)), 4181);
    }
}
