use core::ffi::CStr;
use core::marker::PhantomData;
use core::mem;
use core::ptr;

use objc2::encode::EncodeArguments;
use objc2::encode::{EncodeArgument, EncodeReturn};

use crate::{Block, StackBlock};

mod private {
    pub trait Sealed<A, R> {}
}

/// Types that represent closure parameters/arguments and return types in a
/// block.
///
/// This is implemented for [`dyn`] [`Fn`] closures with up to 12 parameters,
/// where each parameter implements [`EncodeArgument`] and the return type
/// implements [`EncodeReturn`].
///
/// [`dyn`]: https://doc.rust-lang.org/std/keyword.dyn.html
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait BlockFn: private::Sealed<Self::Args, Self::Output> {
    /// The parameters/arguments to the block.
    type Args: EncodeArguments;

    /// The return type of the block.
    type Output: EncodeReturn;

    /// Calls the given invoke function with the block and arguments.
    #[doc(hidden)]
    unsafe fn __call_block(
        invoke: unsafe extern "C-unwind" fn(),
        block: *mut Block<Self>,
        args: Self::Args,
    ) -> Self::Output;
}

/// Types that may be converted into a block.
///
/// This is implemented for [`Fn`] closures of up to 12 parameters, where each
/// parameter implements [`EncodeArgument`] and the return type implements
/// [`EncodeReturn`].
///
///
/// # Safety
///
/// This is a sealed trait, and should not need to be implemented. Open an
/// issue if you know a use-case where this restrition should be lifted!
pub unsafe trait IntoBlock<'f, A, R>: private::Sealed<A, R>
where
    A: EncodeArguments,
    R: EncodeReturn,
{
    /// The type-erased `dyn Fn(...Args) -> R + 'f`.
    type Dyn: ?Sized + BlockFn<Args = A, Output = R>;

    #[doc(hidden)]
    fn __get_invoke_stack_block() -> unsafe extern "C-unwind" fn();
}

macro_rules! impl_traits {
    ($($a:ident: $t:ident),*) => (
        impl<$($t: EncodeArgument,)* R: EncodeReturn, Closure> private::Sealed<($($t,)*), R> for Closure
        where
            Closure: ?Sized + Fn($($t),*) -> R,
        {}

        // TODO: Add `+ Send`, `+ Sync` and `+ Send + Sync` versions.
        unsafe impl<$($t: EncodeArgument,)* R: EncodeReturn> BlockFn for dyn Fn($($t),*) -> R + '_ {
            type Args = ($($t,)*);
            type Output = R;

            #[inline]
            unsafe fn __call_block(
                invoke: unsafe extern "C-unwind" fn(),
                block: *mut Block<Self>,
                ($($a,)*): Self::Args,
            ) -> Self::Output {
                // Very similar to `MessageArguments::__invoke`
                let invoke: unsafe extern "C-unwind" fn(*mut Block<Self> $(, $t)*) -> R = unsafe {
                    mem::transmute(invoke)
                };

                unsafe { invoke(block $(, $a)*) }
            }
        }

        unsafe impl<'f, $($t,)* R, Closure> IntoBlock<'f, ($($t,)*), R> for Closure
        where
            $($t: EncodeArgument,)*
            R: EncodeReturn,
            Closure: Fn($($t),*) -> R + 'f,
        {
            type Dyn = dyn Fn($($t),*) -> R + 'f;

            #[inline]
            fn __get_invoke_stack_block() -> unsafe extern "C-unwind" fn() {
                unsafe extern "C-unwind" fn invoke<'f, $($t,)* R, Closure>(
                    block: *mut StackBlock<'f, ($($t,)*), R, Closure>,
                    $($a: $t,)*
                ) -> R
                where
                    Closure: Fn($($t),*) -> R + 'f
                {
                    let closure = unsafe { &*ptr::addr_of!((*block).closure) };
                    (closure)($($a),*)
                }

                unsafe {
                    mem::transmute::<
                        unsafe extern "C-unwind" fn(*mut StackBlock<'f, ($($t,)*), R, Closure>, $($t,)*) -> R,
                        unsafe extern "C-unwind" fn(),
                    >(invoke)
                }
            }
        }
    );
}

impl_traits!();
impl_traits!(t0: T0);
impl_traits!(t0: T0, t1: T1);
impl_traits!(t0: T0, t1: T1, t2: T2);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10);
impl_traits!(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11);

/// Interim abstraction to manually provide block encodings for use at compile
/// time with [`StackBlock::with_encoding`] and [`RcBlock::with_encoding`].
///
/// See these functions for examples of how to implement and use this trait,
/// since its sole purpose is passing values at compile time to them.
///
/// As a side note, one might be surprised by the additional [`Self::Arguments`]
/// and [`Self::Return`] associated types requiring concrete implementations to
/// specify them while they are not actually used. This is intentional:
///
///  * the types are checked at compile-time to be equal to the ones used with
///    [`RcBlock::with_encoding`] and [`StackBlock::with_encoding`], usually
///    inferred by the compiler when giving a closure: this should help avoid
///    some easy oversights;
///  * the user is forced to write both the standard Rust types and the
///    encoding string at the same time, so particular attention to the types
///    is put to the forefront for them;
///  * reading a block encoding string is tough when not initiated, so these
///    also serve as self-documentation;
///  * the safety validation can be moved to the trait implementation, so that
///    the use can be marked safe.
///
/// [`RcBlock::with_encoding`]: crate::RcBlock::with_encoding
///
/// # Safety
///
/// [`Self::ENCODING_CSTR`] must correspond to the actual signature string a
/// recent-enough Objective-C compiler would generate for a block taking in
/// [`Self::Arguments`] as input and returning [`Self::Return`] as output.
/// This information is actually used by the Objective-C runtime in order to
/// correctly invoke the block, so specifying a wrong encoding is definitely a
/// soundness issue: see [this issue comment][i442-sign-check] for more details
/// about what exactly goes on behind the scenes in order to justify all the
/// following precautions.
///
/// The easiest way to do this is probably to ask Clang; the following program
/// will print the signature of the block (if you're having trouble linking,
/// you should be able to find the signature in the assembly output).
///
/// ```objective-c
/// #import <Foundation/Foundation.h>
///
/// // Unstable API, but fine for test usage like this.
/// const char * _Block_signature(void *);
///
/// int main() {
///     // Declare the signature of your block.
///     // This one takes `id` and `int`, and returns `NSString*`.
///     id block = ^NSString* (id a, int b) {
///         return nil;
///     };
///
///     printf("%s\n", _Block_signature((void*)block));
///     return 0;
/// }
/// ```
///
/// A more thorough but manual approach is to only follow the rules described
/// below.
///
/// In this process, you may be able to use [`Encoding::to_string`][enc2s] in
/// order to get the various components of the signature string and then
/// concatenate them manually with the required numbers (described below)
/// inserted at their correct place.
///
/// [enc2s]: objc2::encode::Encoding#impl-Display-for-Encoding
/// [i442-sign-check]: https://github.com/madsmtm/objc2/issues/442#issuecomment-2284932726
///
/// # Encoding string generation
///
/// This is the result of the `@encode` Objective-C compiler directive. The
/// [Apple documentation] and [GCC documentation] explain how each base type is
/// encoded into a string representation. See there for a somewhat-formal
/// specification and a few basic examples. See also [`Encoding`].
///
/// See also the [GCC method signatures] section. It is mostly valid for blocks
/// as well, since they are basically functions with captured environment --
/// i.e. closures, except that no selector is implicitly sent, only the block
/// object is. In short, the "signature" is a null-terminated string, composed
/// of the following, in order:
///
///  * The return type, including type qualifiers. For example, a block
///    returning `int` ([`i32`]) would have `i` here.
///  * The total size (in bytes) required to pass all the parameters: the call
///    frame size. This includes the hidden block object parameter that is
///    passed as a pointer, so at least 4 bytes when under a 32-bit system or
///    most probably 8 bytes when under a 64-bit one.
///  * Each argument, with the type encoding, followed by the offset (in bytes)
///    of the argument in the list of parameters. The first is the always the
///    hidden block object pointer and is therefore `@?0`.
///
/// Examples:
///
/// | Objective-C signature    | Runtime encoding                           |
/// | ------------------------ | ------------------------------------------ |
/// | `void (^)(void)`         | `v8@?0`                                    |
/// | `int (^)(void)`          | `i8@?0`                                    |
/// | `int (^)(float)`         | `i12@?0f8`                                 |
/// | `int (^)(float, _Bool)`  | `i16@?0f8B12`                              |
/// | `void (^)(int*)`         | `v16@?0^i8`                                |
/// | `void (^)(NSError*)`     | `v16@?0@8` or `v16@?0@"NSError"8`          |
/// | `NSError* (^)(NSError*)` | `@16@?0@8` or `@"NSError"16@?0@"NSError"8` |
///
/// [Apple documentation]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html
/// [`Encoding`]: objc2::encode::Encoding
/// [GCC documentation]: https://gcc.gnu.org/onlinedocs/gcc/Type-encoding.html
/// [GCC method signatures]: https://gcc.gnu.org/onlinedocs/gcc/Method-signatures.html
pub unsafe trait ManualBlockEncoding {
    /// The function's input argument types.
    type Arguments: EncodeArguments;
    /// The function's return type.
    type Return: EncodeReturn;
    /// The raw encoding information string.
    const ENCODING_CSTR: &'static CStr;
}

/// Particular [`ManualBlockEncoding`] that indicates no actual encoding should
/// be set in the block's descriptor.
///
/// This is used in a bit of a hackish way in order to share more code between
/// the encoded and non-encoded paths.
pub(crate) struct NoBlockEncoding<A, R>
where
    A: EncodeArguments,
    R: EncodeReturn,
{
    _a: PhantomData<A>,
    _r: PhantomData<R>,
}

// SAFETY: The encoding here is incorrect, but it will never be used because
// we specify `IS_NONE = true` in `ManualBlockEncodingExt`.
unsafe impl<A, R> ManualBlockEncoding for NoBlockEncoding<A, R>
where
    A: EncodeArguments,
    R: EncodeReturn,
{
    type Arguments = A;
    type Return = R;
    // TODO: change this to `c""` when the MSRV is at least 1.77.
    // SAFETY: the byte string is written here and contains exactly one nul byte.
    const ENCODING_CSTR: &'static CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };
}

/// Crate-private extension to [`ManualBlockEncoding`].
pub(crate) trait ManualBlockEncodingExt: ManualBlockEncoding {
    /// Only `true` for [`NoBlockEncoding`].
    const IS_NONE: bool;
}

impl<E: ManualBlockEncoding> ManualBlockEncodingExt for UserSpecified<E> {
    const IS_NONE: bool = false;
}

impl<A, R> ManualBlockEncodingExt for NoBlockEncoding<A, R>
where
    A: EncodeArguments,
    R: EncodeReturn,
{
    const IS_NONE: bool = true;
}

/// Dummy newtype used to conditionally implement [`ManualBlockEncodingExt`]
/// and therefore circumvent the need for specialization.
#[repr(transparent)]
pub(crate) struct UserSpecified<E: ManualBlockEncoding>(E);

unsafe impl<E: ManualBlockEncoding> ManualBlockEncoding for UserSpecified<E> {
    type Arguments = E::Arguments;
    type Return = E::Return;
    const ENCODING_CSTR: &'static CStr = E::ENCODING_CSTR;
}

/// Checks for encoding compatibility between the given generic parameters,
/// panicking if it is not, but only on `cfg(debug_assertions)` and if `E` is
/// not none.
#[cfg_attr(not(debug_assertions), inline(always))]
#[allow(unused)]
pub(crate) fn debug_assert_block_encoding<A, R, E>()
where
    A: EncodeArguments,
    R: EncodeReturn,
    E: ManualBlockEncodingExt<Arguments = A, Return = R>,
{
    #[cfg(debug_assertions)]
    {
        if !E::IS_NONE {
            // TODO: relax to check for equivalence instead of strict equality.
            assert_eq!(
                E::ENCODING_CSTR,
                &*crate::encoding::block_signature_string::<A, R>()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use core::ffi::c_char;

    use super::*;

    #[test]
    fn test_manual_block_encoding_is_none() {
        // Normal case.
        struct Enc1;
        unsafe impl ManualBlockEncoding for Enc1 {
            type Arguments = (i32, f32);
            type Return = u8;
            #[cfg(target_pointer_width = "64")]
            const ENCODING_CSTR: &'static CStr =
                // Somehow, using a C string literal seems to fail the MSRV
                // check here, so use the old way instead here.
                unsafe { CStr::from_bytes_with_nul_unchecked(b"C16@?0i8f12\0") };
            #[cfg(not(target_pointer_width = "64"))]
            const ENCODING_CSTR: &'static CStr =
                unsafe { CStr::from_bytes_with_nul_unchecked(b"C12@?0i4f8\0") };
        }
        // HACK: use `identity` in order to circumvent a Clippy warning.
        assert!(!core::convert::identity(UserSpecified::<Enc1>::IS_NONE));

        // No input + no output case.
        struct Enc2;
        unsafe impl ManualBlockEncoding for Enc2 {
            type Arguments = ();
            type Return = ();
            #[cfg(target_pointer_width = "64")]
            const ENCODING_CSTR: &'static CStr =
                unsafe { CStr::from_bytes_with_nul_unchecked(b"v8@?0\0") };
            #[cfg(not(target_pointer_width = "64"))]
            const ENCODING_CSTR: &'static CStr =
                unsafe { CStr::from_bytes_with_nul_unchecked(b"v4@?0\0") };
        }
        assert!(!core::convert::identity(UserSpecified::<Enc2>::IS_NONE));

        // Ensure we don't rely on the encoding string's emptiness.
        struct Enc3;
        unsafe impl ManualBlockEncoding for Enc3 {
            type Arguments = ();
            type Return = ();
            const ENCODING_CSTR: &'static CStr =
                unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") };
        }
        assert!(!core::convert::identity(UserSpecified::<Enc3>::IS_NONE));

        // Only `NoBlockEncoding` should be `IS_NONE`.
        assert!(core::convert::identity(NoBlockEncoding::<(), ()>::IS_NONE));
        assert!(core::convert::identity(
            NoBlockEncoding::<(i32, f32), u8>::IS_NONE
        ));
        assert!(core::convert::identity(
            NoBlockEncoding::<(*const u8,), *const c_char>::IS_NONE
        ));
    }
}
