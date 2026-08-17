use core::ffi::c_ulong;
use core::ffi::c_void;
use core::fmt;
use core::marker::PhantomData;
use core::mem::{self, MaybeUninit};
use core::ops::Deref;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::{self, NonNull};

use objc2::encode::{EncodeArguments, EncodeReturn, Encoding, RefEncode};

use crate::abi::{
    BlockDescriptor, BlockDescriptorCopyDispose, BlockDescriptorCopyDisposeSignature,
    BlockDescriptorPtr, BlockDescriptorSignature, BlockFlags, BlockHeader,
};
use crate::debug::debug_block_header;
use crate::traits::{ManualBlockEncoding, ManualBlockEncodingExt, NoBlockEncoding, UserSpecified};
use crate::{ffi, Block, IntoBlock};

/// An Objective-C block constructed on the stack.
///
/// This can be a micro-optimization if you know that the function you're
/// passing the block to won't [copy] the block at all (e.g. if it guarantees
/// that it'll run it synchronously). That's very rare though, most of the
/// time you'll want to use [`RcBlock`].
///
/// This is a smart pointer that [`Deref`]s to [`Block`].
///
/// [copy]: Block::copy
/// [`RcBlock`]: crate::RcBlock
///
///
/// # Type parameters
///
/// The type parameters for this is a bit complex due to trait system
/// limitations. Usually, you will not need to specify them, as the compiler
/// should be able to infer them.
///
/// - The lifetime `'f`, which is the lifetime of the block.
/// - The parameter `A`, which is a tuple representing the parameters to the
///   block.
/// - The parameter `R`, which is the return type of the block.
/// - The parameter `Closure`, which is the contained closure type. This is
///   usually not nameable, and you will have to use `_`, `impl Fn()` or
///   similar.
///
///
/// # Memory layout
///
/// The memory layout of this type is _not_ guaranteed.
///
/// That said, it will always be safe to reinterpret pointers to this as a
/// pointer to a [`Block`] with the corresponding `dyn Fn` type.
#[repr(C)]
pub struct StackBlock<'f, A, R, Closure> {
    /// For correct variance of the other type parameters.
    ///
    /// We add extra auto traits such that they depend on the closure instead.
    p: PhantomData<dyn Fn(A) -> R + Send + Sync + RefUnwindSafe + UnwindSafe + Unpin + 'f>,
    header: BlockHeader,
    /// The block's closure.
    ///
    /// The ABI requires this field to come after the header.
    ///
    /// Note that this is not wrapped in a `ManuallyDrop`; once the
    /// `StackBlock` is dropped, the closure will also be dropped.
    pub(crate) closure: Closure,
}

// SAFETY: Pointers to the stack block is always safe to reinterpret as an
// ordinary block pointer.
unsafe impl<'f, A, R, Closure> RefEncode for StackBlock<'f, A, R, Closure>
where
    A: EncodeArguments,
    R: EncodeReturn,
    Closure: IntoBlock<'f, A, R>,
{
    const ENCODING_REF: Encoding = Encoding::Block;
}

// Basic constants and helpers.
impl<A, R, Closure> StackBlock<'_, A, R, Closure> {
    /// The size of the block header and the trailing closure.
    ///
    /// This ensures that the closure that the block contains is also moved to
    /// the heap in `_Block_copy` operations.
    const SIZE: c_ulong = mem::size_of::<Self>() as _;

    // Drop the closure that this block contains.
    unsafe extern "C-unwind" fn drop_closure(block: *mut c_void) {
        let block: *mut Self = block.cast();
        // When this function is called, the block no longer lives on the
        // stack, it has been moved to the heap as part of some `_Block_copy`
        // operation, including ownership over the block.
        //
        // It is our task to ensure that the closure's data is properly
        // disposed, which we do by calling `Drop`.

        // We use `addr_of_mut` here to not assume anything about the block's
        // contents. This is probably overly cautious, `BlockHeader` already
        // makes very few assumptions about the validity of the data it
        // contains.
        //
        // SAFETY: The block pointer is valid, and contains the closure.
        let closure = unsafe { ptr::addr_of_mut!((*block).closure) };
        // SAFETY: The ownership of the closure was moved into the block as
        // part of some `_Block_copy` operation, and as such it is valid to
        // drop here.
        unsafe { ptr::drop_in_place(closure) };
    }

    const DESCRIPTOR_BASIC: BlockDescriptor = BlockDescriptor {
        reserved: 0,
        size: Self::SIZE,
    };
}

// `StackBlock::new`
impl<A, R, Closure: Clone> StackBlock<'_, A, R, Closure> {
    // Clone the closure from one block to another.
    unsafe extern "C-unwind" fn clone_closure(dst: *mut c_void, src: *const c_void) {
        let dst: *mut Self = dst.cast();
        let src: *const Self = src.cast();
        // When this function is called as part of some `_Block_copy`
        // operation, the `dst` block has been constructed on the heap, and
        // the `src` block has been `memmove`d to that.
        //
        // It is our task to ensure that the rest of the closure's data is
        // properly copied, which we do by calling `Clone`. This newly cloned
        // closure will be dropped in `drop_closure`.

        // We use `addr_of[_mut]` to not assume anything about the block's
        // contents, see `drop_closure` for details.
        //
        // SAFETY: The block pointers are valid, and each contain the closure.
        let dst_closure = unsafe { ptr::addr_of_mut!((*dst).closure) };
        let src_closure = unsafe { &*ptr::addr_of!((*src).closure) };
        // SAFETY: `dst` is valid for writes.
        // TODO: How do we ensure proper alignment?
        //
        // Note: This is slightly inefficient, as we're overwriting the
        // already `memmove`d data once more, which is unnecessary for closure
        // captures that implement `Copy`.
        unsafe { ptr::write(dst_closure, src_closure.clone()) };
    }

    const DESCRIPTOR_WITH_CLONE: BlockDescriptorCopyDispose = BlockDescriptorCopyDispose {
        reserved: 0,
        size: Self::SIZE,
        copy: Some(Self::clone_closure),
        dispose: Some(Self::drop_closure),
    };
}

impl<'f, A, R, Closure> StackBlock<'f, A, R, Closure>
where
    A: EncodeArguments,
    R: EncodeReturn,
    Closure: IntoBlock<'f, A, R> + Clone,
{
    /// Construct a `StackBlock` with the given closure.
    ///
    /// Note that this requires [`Clone`], as a C block is generally assumed
    /// to be copy-able. If you want to avoid that, put the block directly on
    /// the heap using [`RcBlock::new`].
    ///
    /// When the block is called, it will return the value that results from
    /// calling the closure.
    ///
    /// [`RcBlock::new`]: crate::RcBlock::new
    ///
    ///
    /// ## Example
    ///
    /// ```
    /// use block2::StackBlock;
    /// #
    /// # extern "C" fn check_addition(block: &block2::Block<dyn Fn(i32, i32) -> i32>) {
    /// #     assert_eq!(block.call((5, 8)), 13);
    /// # }
    ///
    /// let block = StackBlock::new(|a, b| a + b);
    /// check_addition(&block);
    /// ```
    #[inline]
    pub fn new(closure: Closure) -> Self {
        Self::maybe_encoded::<NoBlockEncoding<A, R>>(closure)
    }

    /// Constructs a new [`StackBlock`] with the given function and encoding
    /// information.
    ///
    /// Some particular newer-ish Apple Objective-C APIs expect the block they
    /// are given to be created with encoding information set in the block
    /// object itself and crash the calling process if they fail to find it,
    /// which renders them pretty much unusable with only [`Self::new`] that
    /// currently does not set such encoding information. This is for example
    /// the case in [`FileProvider`] for [`NSFileProviderManager`]'s
    /// [`reimportItemsBelowItemWithIdentifier:completionHandler:`] and
    /// [`waitForStabilizationWithCompletionHandler:`], but also in
    /// [`NetworkExtension`] for [`NEFilterDataProvider`]'s
    /// [`applySettings:completionHandler`]. A complete list of such APIs may
    /// not be easily obtained, though.
    ///
    /// This encoding information string could technically be generated at
    /// compile time using the generic parameters already available to
    /// [`Self::new`]. However, doing so would require some constant evaluation
    /// features that are yet to be implemented and stabilized in the Rust
    /// compiler. This function is therefore exposed in the meantime so users
    /// may still be able to call the concerned APIs by providing the raw
    /// encoding information string themselves, thus obtaining a block
    /// containing it and working with these APIs.
    ///
    /// You provide the encoding through the `E` type parameter, which should
    /// implement [`ManualBlockEncoding`].
    ///
    /// The same requirements as [`Self::new`] apply here as well.
    ///
    /// [`FileProvider`]: https://developer.apple.com/documentation/fileprovider?language=objc
    /// [`NSFileProviderManager`]: https://developer.apple.com/documentation/fileprovider/nsfileprovidermanager?language=objc
    /// [`reimportItemsBelowItemWithIdentifier:completionHandler:`]: https://developer.apple.com/documentation/fileprovider/nsfileprovidermanager/reimportitems(below:completionhandler:)?language=objc
    /// [`waitForStabilizationWithCompletionHandler:`]: https://developer.apple.com/documentation/fileprovider/nsfileprovidermanager/waitforstabilization(completionhandler:)?language=objc
    /// [`NetworkExtension`]: https://developer.apple.com/documentation/networkextension?language=objc
    /// [`NEFilterDataProvider`]: https://developer.apple.com/documentation/networkextension/nefilterdataprovider?language=objc
    /// [`applySettings:completionHandler`]: https://developer.apple.com/documentation/networkextension/nefilterdataprovider/3181998-applysettings?language=objc
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ffi::CStr;
    /// # use block2::{Block, ManualBlockEncoding, StackBlock};
    /// # use objc2_foundation::NSError;
    /// #
    /// struct MyBlockEncoding;
    /// // SAFETY: The encoding is correct.
    /// unsafe impl ManualBlockEncoding for MyBlockEncoding {
    ///     type Arguments = (*mut NSError,);
    ///     type Return = i32;
    ///     const ENCODING_CSTR: &'static CStr = if cfg!(target_pointer_width = "64") {
    ///         cr#"i16@?0@"NSError"8"#
    ///     } else {
    ///         cr#"i8@?0@"NSError"4"#
    ///     };
    /// }
    ///
    /// let my_block = StackBlock::with_encoding::<MyBlockEncoding>(|_err: *mut NSError| {
    ///     42i32
    /// });
    /// assert_eq!(my_block.call((core::ptr::null_mut(),)), 42);
    /// ```
    #[inline]
    pub fn with_encoding<E>(closure: Closure) -> Self
    where
        E: ManualBlockEncoding<Arguments = A, Return = R>,
    {
        Self::maybe_encoded::<UserSpecified<E>>(closure)
    }

    #[inline]
    fn maybe_encoded<E>(closure: Closure) -> Self
    where
        E: ManualBlockEncodingExt<Arguments = A, Return = R>,
    {
        // TODO: Re-consider calling `crate::traits::debug_assert_block_encoding`.
        let header = BlockHeader {
            #[allow(unused_unsafe)]
            isa: unsafe { ptr::addr_of!(ffi::_NSConcreteStackBlock) },
            flags: BlockFlags::BLOCK_HAS_COPY_DISPOSE
                | if !E::IS_NONE {
                    BlockFlags::BLOCK_HAS_SIGNATURE
                } else {
                    BlockFlags::EMPTY
                },
            reserved: MaybeUninit::new(0),
            invoke: Some(Closure::__get_invoke_stack_block()),
            // TODO: Use `Self::DESCRIPTOR_BASIC` when `F: Copy`
            // (probably only possible with specialization).
            descriptor: if E::IS_NONE {
                // SAFETY: The descriptor must (probably) point to `static`
                // memory, as Objective-C code may assume the block's
                // descriptor to be alive past the lifetime of the block
                // itself.
                //
                // Ideally, we'd have declared the descriptor as a `static`
                // but since Rust doesn't have generic statics, we have to
                // rely on [promotion] here to convert the constant into a
                // static.
                //
                // For this to work, it requires that the descriptor type does
                // not implement `Drop` (it does not), and that the descriptor
                // does not contain `UnsafeCell` (it does not).
                //
                // [promotion]: https://doc.rust-lang.org/reference/destructors.html#constant-promotion
                BlockDescriptorPtr {
                    with_copy_dispose: &Self::DESCRIPTOR_WITH_CLONE,
                }
            } else {
                // SAFETY: see above; the value is already a similar constant,
                // so promotion can be guaranteed as well here.
                BlockDescriptorPtr {
                    // TODO: move to a `const fn` defined next to the partially-
                    // copied constant and called here in an inline `const` when
                    // the MSRV is at least 1.79.
                    with_copy_dispose_signature:
                        &<Self as EncodedCloneDescriptors<E>>::DESCRIPTOR_WITH_CLONE_AND_ENCODING,
                }
            },
        };
        Self {
            p: PhantomData,
            header,
            closure,
        }
    }
}

// `RcBlock::with_encoding`
impl<'f, A, R, Closure> StackBlock<'f, A, R, Closure> {
    unsafe extern "C-unwind" fn empty_clone_closure(_dst: *mut c_void, _src: *const c_void) {
        // We do nothing, the closure has been `memmove`'d already, and
        // ownership will be passed in `RcBlock::with_encoding`.
    }

    const DESCRIPTOR_WITH_DROP: BlockDescriptorCopyDispose = BlockDescriptorCopyDispose {
        reserved: 0,
        size: Self::SIZE,
        copy: Some(Self::empty_clone_closure),
        dispose: Some(Self::drop_closure),
    };

    /// # Safety
    ///
    ///  `_Block_copy` must be called on the resulting stack block only once.
    #[inline]
    pub(crate) unsafe fn new_no_clone<E>(closure: Closure) -> Self
    where
        A: EncodeArguments,
        R: EncodeReturn,
        Closure: IntoBlock<'f, A, R>,
        E: ManualBlockEncodingExt<Arguments = A, Return = R>,
    {
        // TODO: Re-consider calling `crate::traits::debug_assert_block_encoding`.
        // Don't need to emit copy and dispose helpers if the closure
        // doesn't need it.
        let flags = if mem::needs_drop::<Self>() {
            BlockFlags::BLOCK_HAS_COPY_DISPOSE
        } else {
            BlockFlags::EMPTY
        } | if !E::IS_NONE {
            BlockFlags::BLOCK_HAS_SIGNATURE
        } else {
            BlockFlags::EMPTY
        };
        // See discussion in `new` above with regards to the safety of the
        // pointer to the descriptor.
        let descriptor = match (mem::needs_drop::<Self>(), E::IS_NONE) {
            (true, true) => {
                // SAFETY: see above.
                BlockDescriptorPtr {
                    with_copy_dispose: &Self::DESCRIPTOR_WITH_DROP,
                }
            }
            (false, true) => {
                // SAFETY: see above.
                BlockDescriptorPtr {
                    basic: &Self::DESCRIPTOR_BASIC,
                }
            }
            (true, false) => {
                // SAFETY: see above; the value is already a similar constant,
                // so promotion can be guaranteed as well here.
                BlockDescriptorPtr {
                    // TODO: move to a `const fn` defined next to the partially-
                    // copied constant and called here in an inline `const` when
                    // the MSRV is at least 1.79.
                    with_copy_dispose_signature:
                        &<Self as EncodedDescriptors<E>>::DESCRIPTOR_WITH_DROP_AND_ENCODING,
                }
            }
            (false, false) => {
                // SAFETY: see above; the value is already a similar constant,
                // so promotion can be guaranteed as well here.
                BlockDescriptorPtr {
                    // TODO: move to a `const fn` defined next to the partially-
                    // copied constant and called here in an inline `const` when
                    // the MSRV is at least 1.79.
                    with_signature:
                        &<Self as EncodedDescriptors<E>>::DESCRIPTOR_BASIC_WITH_ENCODING,
                }
            }
        };

        let header = BlockHeader {
            #[allow(unused_unsafe)]
            isa: unsafe { ptr::addr_of!(ffi::_NSConcreteStackBlock) },
            flags,
            reserved: MaybeUninit::new(0),
            invoke: Some(Closure::__get_invoke_stack_block()),
            descriptor,
        };
        Self {
            p: PhantomData,
            header,
            closure,
        }
    }
}

/// Dummy trait used in order to link [`StackBlock`]'s descriptor constants and
/// a [`ManualBlockEncoding`] into new derived constants at compile time.
///
/// This is definitely a hack that should be replaced with `const fn`s defined
/// next to [`StackBlock`]'s descriptor constants and used in the below
/// constants' stead with inline `const`s to guarantee proper promotion.
///
/// See also the below [`EncodedCloneDescriptors`].
trait EncodedDescriptors<E: ManualBlockEncoding> {
    const DESCRIPTOR_BASIC_WITH_ENCODING: BlockDescriptorSignature;
    const DESCRIPTOR_WITH_DROP_AND_ENCODING: BlockDescriptorCopyDisposeSignature;
}

impl<'f, A, R, Closure, E> EncodedDescriptors<E> for StackBlock<'f, A, R, Closure>
where
    A: EncodeArguments,
    R: EncodeReturn,
    Closure: IntoBlock<'f, A, R>,
    E: ManualBlockEncoding<Arguments = A, Return = R>,
{
    /// [`Self::DESCRIPTOR_BASIC`] with the signature added from `E`.
    const DESCRIPTOR_BASIC_WITH_ENCODING: BlockDescriptorSignature = BlockDescriptorSignature {
        reserved: Self::DESCRIPTOR_BASIC.reserved,
        size: Self::DESCRIPTOR_BASIC.size,
        encoding: E::ENCODING_CSTR.as_ptr(),
    };
    /// [`Self::DESCRIPTOR_WITH_DROP`] with the signature added from `E`.
    const DESCRIPTOR_WITH_DROP_AND_ENCODING: BlockDescriptorCopyDisposeSignature =
        BlockDescriptorCopyDisposeSignature {
            reserved: Self::DESCRIPTOR_WITH_DROP.reserved,
            size: Self::DESCRIPTOR_WITH_DROP.size,
            copy: Self::DESCRIPTOR_WITH_DROP.copy,
            dispose: Self::DESCRIPTOR_WITH_DROP.dispose,
            encoding: E::ENCODING_CSTR.as_ptr(),
        };
}

/// Identical role as [`EncodedDescriptors`], with the additional requirement
/// that the block closure be [`Clone`] when implemented on [`StackBlock`]
/// since [`StackBlock::DESCRIPTOR_WITH_CLONE`] is defined in such a context.
trait EncodedCloneDescriptors<E: ManualBlockEncoding> {
    const DESCRIPTOR_WITH_CLONE_AND_ENCODING: BlockDescriptorCopyDisposeSignature;
}

impl<'f, A, R, Closure, E> EncodedCloneDescriptors<E> for StackBlock<'f, A, R, Closure>
where
    A: EncodeArguments,
    R: EncodeReturn,
    Closure: IntoBlock<'f, A, R> + Clone,
    E: ManualBlockEncoding<Arguments = A, Return = R>,
{
    /// [`Self::DESCRIPTOR_WITH_CLONE`] with the signature added from `E`.
    const DESCRIPTOR_WITH_CLONE_AND_ENCODING: BlockDescriptorCopyDisposeSignature =
        BlockDescriptorCopyDisposeSignature {
            reserved: Self::DESCRIPTOR_WITH_CLONE.reserved,
            size: Self::DESCRIPTOR_WITH_CLONE.size,
            copy: Self::DESCRIPTOR_WITH_CLONE.copy,
            dispose: Self::DESCRIPTOR_WITH_CLONE.dispose,
            encoding: E::ENCODING_CSTR.as_ptr(),
        };
}

impl<A, R, Closure: Clone> Clone for StackBlock<'_, A, R, Closure> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            p: PhantomData,
            header: self.header,
            closure: self.closure.clone(),
        }
    }
}

impl<A, R, Closure: Copy> Copy for StackBlock<'_, A, R, Closure> {}

impl<'f, A, R, Closure> Deref for StackBlock<'f, A, R, Closure>
where
    A: EncodeArguments,
    R: EncodeReturn,
    Closure: IntoBlock<'f, A, R>,
{
    type Target = Block<Closure::Dyn>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let ptr: NonNull<Self> = NonNull::from(self);
        let ptr: NonNull<Block<Closure::Dyn>> = ptr.cast();
        // SAFETY: A pointer to `StackBlock` is always safe to convert to a
        // pointer to `Block`, and the reference will be valid as long the
        // stack block exists.
        //
        // Finally, the stack block is implemented correctly, such that it is
        // safe to call `_Block_copy` on the returned value.
        unsafe { ptr.as_ref() }
    }
}

impl<A, R, Closure> fmt::Debug for StackBlock<'_, A, R, Closure> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("StackBlock");
        debug_block_header(&self.header, &mut f);
        f.finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(
            mem::size_of::<BlockHeader>(),
            <StackBlock<'_, (), (), ()>>::SIZE as _,
        );
        assert_eq!(
            mem::size_of::<BlockHeader>() + mem::size_of::<fn()>(),
            <StackBlock<'_, (), (), fn()>>::SIZE as _,
        );
    }

    #[allow(dead_code)]
    fn covariant<'b, 'f>(
        b: StackBlock<'static, (), (), impl Fn() + 'static>,
    ) -> StackBlock<'b, (), (), impl Fn() + 'f> {
        b
    }
}
