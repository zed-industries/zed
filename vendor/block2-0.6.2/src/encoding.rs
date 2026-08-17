use alloc::ffi::CString;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::mem;

use objc2::encode::{EncodeArguments, EncodeReturn, Encoding};

/// Computes the raw signature string of the object corresponding to the block
/// taking `A` as inputs and returning `R`.
///
/// Although this is currently implemented on a best-effort basis, this should
/// still serve as a good way to obtain what to fill in the encoding string
/// when implementing [`crate::ManualBlockEncoding`].
///
/// # Example
///
/// ```ignore
/// assert_eq!(block_signature_string::<(i32, f32), u8>(), "C16@?0i8f12");
/// ```
#[allow(unused)]
pub(crate) fn block_signature_string<A, R>() -> CString
where
    A: EncodeArguments,
    R: EncodeReturn,
{
    block_signature_string_inner(A::ENCODINGS, &R::ENCODING_RETURN)
}

#[allow(unused)]
fn block_signature_string_inner(args: &[Encoding], ret: &Encoding) -> CString {
    // TODO: alignment?
    let arg_sizes = args
        .iter()
        .map(Encoding::size)
        .map(Option::unwrap_or_default)
        .collect::<Vec<_>>();
    let args_size = arg_sizes.iter().sum::<usize>();

    // Take the hidden block parameter into account.
    let mut off = mem::size_of::<*const ()>();
    let mut res = ret.to_string();
    res.push_str(&(off + args_size).to_string());
    res.push_str("@?0");

    for (arg_enc, arg_size) in args.iter().zip(arg_sizes) {
        res.push_str(&arg_enc.to_string());
        res.push_str(&off.to_string());
        off += arg_size;
    }

    // UNWRAP: The above construction only uses controlled `ToString`
    // implementations that do not include nul bytes.
    CString::new(res).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::borrow::ToOwned;

    #[test]
    fn test_block_signature_string() {
        for (args, ret, val) in [
            (
                &[][..],
                &Encoding::Void,
                #[cfg(target_pointer_width = "64")]
                "v8@?0",
                #[cfg(target_pointer_width = "32")]
                "v4@?0",
            ),
            (
                &[],
                &Encoding::Int,
                #[cfg(target_pointer_width = "64")]
                "i8@?0",
                #[cfg(target_pointer_width = "32")]
                "i4@?0",
            ),
            (
                &[],
                &Encoding::Float,
                #[cfg(target_pointer_width = "64")]
                "f8@?0",
                #[cfg(target_pointer_width = "32")]
                "f4@?0",
            ),
            (
                &[],
                &Encoding::Bool,
                #[cfg(target_pointer_width = "64")]
                "B8@?0",
                #[cfg(target_pointer_width = "32")]
                "B4@?0",
            ),
            (
                &[Encoding::Int],
                &Encoding::Void,
                #[cfg(target_pointer_width = "64")]
                "v12@?0i8",
                #[cfg(target_pointer_width = "32")]
                "v8@?0i4",
            ),
            (
                &[Encoding::Int],
                &Encoding::Int,
                #[cfg(target_pointer_width = "64")]
                "i12@?0i8",
                #[cfg(target_pointer_width = "32")]
                "i8@?0i4",
            ),
            (
                &[Encoding::Long, Encoding::Double, Encoding::FloatComplex],
                &Encoding::Atomic(&Encoding::UChar),
                #[cfg(target_pointer_width = "64")]
                "AC32@?0l8d16jf24",
                #[cfg(target_pointer_width = "32")]
                "AC24@?0l4d8jf16",
            ),
            (
                &[
                    Encoding::Union("ThisOrThat", &[Encoding::UShort, Encoding::Int]),
                    Encoding::Struct(
                        "ThisAndThat",
                        &[
                            Encoding::ULongLong,
                            Encoding::LongDoubleComplex,
                            Encoding::Atomic(&Encoding::Bool),
                        ],
                    ),
                ],
                &Encoding::String,
                // Probably unaligned.
                #[cfg(any(
                    target_arch = "x86_64",
                    all(target_arch = "aarch64", not(target_vendor = "apple"))
                ))]
                "*53@?0(ThisOrThat=Si)8{ThisAndThat=QjDAB}12",
                #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
                "*37@?0(ThisOrThat=Si)8{ThisAndThat=QjDAB}12",
                #[cfg(all(target_arch = "x86", target_vendor = "apple"))]
                "*49@?0(ThisOrThat=Si)4{ThisAndThat=QjDAB}8",
                #[cfg(all(target_arch = "x86", not(target_vendor = "apple")))]
                "*41@?0(ThisOrThat=Si)4{ThisAndThat=QjDAB}8",
                #[cfg(target_arch = "arm")]
                "*37@?0(ThisOrThat=Si)4{ThisAndThat=QjDAB}8",
            ),
            (
                &[
                    Encoding::Block,
                    Encoding::Class,
                    Encoding::Object,
                    Encoding::Pointer(&Encoding::Char),
                    Encoding::Sel,
                    Encoding::String,
                    Encoding::Unknown,
                    Encoding::Unknown,
                    Encoding::Unknown,
                ],
                &Encoding::Pointer(&Encoding::Atomic(&Encoding::UChar)),
                #[cfg(target_pointer_width = "64")]
                "^AC56@?0@?8#16@24^c32:40*48?56?56?56",
                #[cfg(target_pointer_width = "32")]
                "^AC28@?0@?4#8@12^c16:20*24?28?28?28",
            ),
            (
                &[Encoding::Array(123, &Encoding::Object)],
                &Encoding::Pointer(&Encoding::Class),
                #[cfg(target_pointer_width = "64")]
                "^#992@?0[123@]8",
                #[cfg(target_pointer_width = "32")]
                "^#496@?0[123@]4",
            ),
            // Bitfields can probably not be passed around through functions,
            // so this may be a bit nonsensical, but let's test it anyway.
            (
                &[
                    Encoding::BitField(1, None),
                    Encoding::BitField(2, None),
                    Encoding::BitField(3, None),
                    Encoding::BitField(6, None),
                    Encoding::BitField(8, None),
                    Encoding::BitField(42, None),
                    Encoding::BitField(28, Some(&(2, Encoding::UInt))),
                ],
                &Encoding::Sel,
                #[cfg(target_pointer_width = "64")]
                ":25@?0b18b29b310b611b812b4213b2I2821",
                #[cfg(target_pointer_width = "32")]
                ":21@?0b14b25b36b67b88b429b2I2817",
            ),
        ] {
            assert_eq!(
                block_signature_string_inner(args, ret),
                CString::new(val.to_owned().into_bytes()).unwrap()
            );
        }
    }
}
