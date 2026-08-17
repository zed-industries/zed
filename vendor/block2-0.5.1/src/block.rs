use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;

use objc2::encode::{Encoding, RefEncode};

use crate::abi::BlockHeader;
use crate::debug::debug_block_header;
use crate::rc_block::block_copy_fail;
use crate::{BlockFn, RcBlock};

/// An opaque type that holds an Objective-C block.
///
/// The generic type `F` must be a [`dyn`] [`Fn`] that implements
/// the [`BlockFn`] trait (which means parameter and return types must be
/// "encodable"), and describes the parameter and return types of the block.
///
/// For example, you may have the type `Block<dyn Fn(u8, u8) -> i32>`, and
/// that would be a `'static` block that takes two `u8`s, and returns an
/// `i32`.
///
/// If you want the block to carry a lifetime, use `Block<dyn Fn() + 'a>`,
/// just like you'd usually do with `dyn Fn`.
///
/// [`dyn`]: https://doc.rust-lang.org/std/keyword.dyn.html
///
///
/// # Memory layout
///
/// This is intented to be an `extern type`, and as such the memory layout of
/// this type is _not_ guaranteed. That said, **pointers** to this type are
/// always thin, and match that of Objective-C blocks. So the layout of e.g.
/// `&Block<dyn Fn(...) -> ... + '_>` is defined, and guaranteed to be
/// pointer-sized and ABI-compatible with a block pointer.
///
///
/// # Safety invariant
///
/// Calling this potentially invokes foreign code, so you must verify, when
/// creating a reference to this, or returning it from an external API, that
/// it doesn't violate any of Rust's safety rules.
///
/// In particular, blocks are sharable with multiple references (see e.g.
/// [`Block::copy`]), so the caller must ensure that calling it can never
/// cause a data race. This usually means you'll have to use some form of
/// interior mutability, if you need to mutate something from inside a block.
//
// TODO: Potentially restrict to `F: BlockFn`, for better error messages?
#[repr(C)]
pub struct Block<F: ?Sized> {
    _inner: [u8; 0],
    /// We store `BlockHeader` + the closure captures, but `Block` has to
    /// remain an empty type because we don't know the size of the closure,
    /// and otherwise the compiler would think we only have provenance over
    /// `BlockHeader`.
    ///
    /// This is possible to improve once we have extern types.
    _header: PhantomData<BlockHeader>,
    _p: PhantomData<F>,
}

// SAFETY: Pointers to `Block` is an Objective-C block.
// This is only valid when `F: BlockFn`, as that bounds the parameters and
// return type to be encodable too.
unsafe impl<F: ?Sized + BlockFn> RefEncode for Block<F> {
    const ENCODING_REF: Encoding = Encoding::Block;
}

impl<F: ?Sized> Block<F> {
    fn header(&self) -> &BlockHeader {
        let ptr: NonNull<Self> = NonNull::from(self);
        let ptr: NonNull<BlockHeader> = ptr.cast();
        // SAFETY: `Block` is `BlockHeader` + closure
        unsafe { ptr.as_ref() }
    }

    /// Copy the block onto the heap as an [`RcBlock`].
    ///
    /// The behaviour of this function depends on whether the block is from a
    /// [`RcBlock`] or a [`StackBlock`]. In the former case, it will bump the
    /// reference-count (just as-if you'd `Clone`'d the `RcBlock`), in the
    /// latter case it will construct a new `RcBlock` from the `StackBlock`.
    ///
    /// This distiction should not matter, except for micro-optimizations.
    ///
    /// [`StackBlock`]: crate::StackBlock
    #[doc(alias = "Block_copy")]
    #[doc(alias = "_Block_copy")]
    #[inline]
    pub fn copy(&self) -> RcBlock<F> {
        let ptr: *const Self = self;
        let ptr: *mut Block<F> = ptr as *mut _;
        // SAFETY: The lifetime of the block is extended from `&self` to that
        // of the `RcBlock`, which is fine, because the lifetime of the
        // contained closure `F` is still carried along to the `RcBlock`.
        unsafe { RcBlock::copy(ptr) }.unwrap_or_else(|| block_copy_fail())
    }

    /// Call the block.
    ///
    /// The arguments must be passed as a tuple. The return is the output of
    /// the block.
    #[doc(alias = "invoke")]
    pub fn call(&self, args: F::Args) -> F::Output
    where
        F: BlockFn,
    {
        // TODO: Is `invoke` actually ever null?
        let invoke = self.header().invoke.unwrap_or_else(|| unreachable!());

        let ptr: NonNull<Self> = NonNull::from(self);
        let ptr: *mut Self = ptr.as_ptr();

        // SAFETY: The closure is an `Fn`, and as such is safe to call from an
        // immutable reference.
        unsafe { F::__call_block(invoke, ptr, args) }
    }
}

impl<F: ?Sized> fmt::Debug for Block<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("Block");
        debug_block_header(self.header(), &mut f);
        f.finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use core::cell::Cell;
    use core::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    /// Test that the way you specify lifetimes are as documented in the
    /// reference.
    /// <https://doc.rust-lang.org/nightly/reference/lifetime-elision.html#default-trait-object-lifetimes>
    #[test]
    fn test_rust_dyn_lifetime_semantics() {
        fn takes_static(block: &Block<dyn Fn() + 'static>) {
            block.call(());
        }

        fn takes_elided(block: &Block<dyn Fn() + '_>) {
            block.call(());
        }

        fn takes_unspecified(block: &Block<dyn Fn()>) {
            block.call(());
        }

        // Static lifetime
        static MY_STATIC: AtomicUsize = AtomicUsize::new(0);
        MY_STATIC.store(0, Ordering::Relaxed);
        let static_lifetime: RcBlock<dyn Fn() + 'static> = RcBlock::new(|| {
            MY_STATIC.fetch_add(1, Ordering::Relaxed);
        });
        takes_static(&static_lifetime);
        takes_elided(&static_lifetime);
        takes_unspecified(&static_lifetime);
        assert_eq!(MY_STATIC.load(Ordering::Relaxed), 3);

        // Lifetime declared with `'_`
        let captured = Cell::new(0);
        let elided_lifetime: RcBlock<dyn Fn() + '_> = RcBlock::new(|| {
            captured.set(captured.get() + 1);
        });
        // takes_static(&elided_lifetime); // Compile error
        takes_elided(&elided_lifetime);
        // takes_unspecified(&elided_lifetime); // Compile error
        assert_eq!(captured.get(), 1);

        // Lifetime kept unspecified
        let captured = Cell::new(0);
        let unspecified_lifetime: RcBlock<dyn Fn()> = RcBlock::new(|| {
            captured.set(captured.get() + 1);
        });
        // takes_static(&unspecified_lifetime); // Compile error
        takes_elided(&unspecified_lifetime);
        // takes_unspecified(&unspecified_lifetime); // Compile error
        assert_eq!(captured.get(), 1);
    }

    #[allow(dead_code)]
    fn unspecified_in_fn_is_static(block: &Block<dyn Fn()>) -> &Block<dyn Fn() + 'static> {
        block
    }

    #[allow(dead_code)]
    fn lending_block<'b>(block: &Block<dyn Fn() -> &'b i32 + 'b>) {
        let _ = *block.call(());
    }

    #[allow(dead_code)]
    fn takes_lifetime(_: &Block<dyn Fn(&i32) -> &i32>) {
        // Not actually callable yet
    }

    #[allow(dead_code)]
    fn covariant<'b, 'f>(b: &'b Block<dyn Fn() + 'static>) -> &'b Block<dyn Fn() + 'f> {
        b
    }
}
