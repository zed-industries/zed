//! Utilities for loading SPIR-V module data.

use alloc::borrow::Cow;
use core::mem;

#[cfg_attr(not(any(feature = "spirv", doc)), expect(unused_imports))]
use crate::ShaderSource;

#[cfg(doc)]
use crate::Device;

const SPIRV_MAGIC_NUMBER: u32 = 0x0723_0203;

/// Treat the given byte slice as a SPIR-V module.
///
/// # Panics
///
/// This function panics if:
///
/// - `data.len()` is not a multiple of 4
/// - `data` does not begin with the SPIR-V magic number
///
/// It does not check that the data is a valid SPIR-V module in any other way.
#[cfg(feature = "spirv")] // ShaderSource::SpirV only exists in this case
pub fn make_spirv(data: &[u8]) -> ShaderSource<'_> {
    ShaderSource::SpirV(make_spirv_raw(data))
}

/// Check whether the byte slice has the SPIR-V magic number (in either byte order) and of an
/// appropriate size, and panic with a suitable message when it is not.
///
/// Returns whether the endianness is opposite of native endianness (i.e. whether
/// [`u32::swap_bytes()`] should be called.)
///
/// Note: this function’s checks are relied upon for the soundness of [`make_spirv_const()`].
/// Undefined behavior will result if it does not panic when `bytes.len()` is not a multiple of 4.
#[track_caller]
const fn assert_has_spirv_magic_number_and_length(bytes: &[u8]) -> bool {
    // First, check the magic number.
    // This way we give the best error for wrong formats.
    // (Plus a special case for the empty slice.)
    let found_magic_number: Option<bool> = match *bytes {
        [] => panic!("byte slice is empty, not SPIR-V"),
        // This would be simpler as slice::starts_with(), but that isn't a const fn yet.
        [b1, b2, b3, b4, ..] => {
            let prefix = u32::from_ne_bytes([b1, b2, b3, b4]);
            if prefix == SPIRV_MAGIC_NUMBER {
                Some(false)
            } else if prefix == const { SPIRV_MAGIC_NUMBER.swap_bytes() } {
                // needs swapping
                Some(true)
            } else {
                None
            }
        }
        _ => None, // fallthrough case = between 1 and 3 bytes
    };

    match found_magic_number {
        Some(needs_byte_swap) => {
            // Note: this assertion is relied upon for the soundness of `make_spirv_const()`.
            assert!(
                bytes.len().is_multiple_of(mem::size_of::<u32>()),
                "SPIR-V data must be a multiple of 4 bytes long"
            );

            needs_byte_swap
        }
        None => {
            panic!(
                "byte slice does not start with SPIR-V magic number. \
            Make sure you are using a binary SPIR-V file."
            );
        }
    }
}

#[cfg_attr(not(feature = "spirv"), expect(rustdoc::broken_intra_doc_links))]
/// Version of [`make_spirv()`] intended for use with
/// [`Device::create_shader_module_passthrough()`].
///
/// Returns a raw slice instead of [`ShaderSource`].
///
/// # Panics
///
/// This function panics if:
///
/// - `data.len()` is not a multiple of 4
/// - `data` does not begin with the SPIR-V magic number
///
/// It does not check that the data is a valid SPIR-V module in any other way.
pub fn make_spirv_raw(bytes: &[u8]) -> Cow<'_, [u32]> {
    let needs_byte_swap = assert_has_spirv_magic_number_and_length(bytes);

    // If the data happens to be aligned, directly use the byte array,
    // otherwise copy the byte array in an owned vector and use that instead.
    let mut words: Cow<'_, [u32]> = match bytemuck::try_cast_slice(bytes) {
        Ok(words) => Cow::Borrowed(words),
        // We already checked the length, so if this fails, it fails due to lack of alignment only.
        Err(_) => Cow::Owned(bytemuck::pod_collect_to_vec(bytes)),
    };

    // If necessary, swap bytes to native endianness.
    if needs_byte_swap {
        for word in Cow::to_mut(&mut words) {
            *word = word.swap_bytes();
        }
    }

    assert!(
        words[0] == SPIRV_MAGIC_NUMBER,
        "can't happen: wrong magic number after swap_bytes"
    );
    words
}

/// Version of `make_spirv_raw` used for implementing [`include_spirv!`] and [`include_spirv_raw!`] macros.
///
/// Not public API. Also, don't even try calling at runtime; you'll get a stack overflow.
///
/// [`include_spirv!`]: crate::include_spirv
#[doc(hidden)]
pub const fn make_spirv_const<const IN: usize, const OUT: usize>(bytes: [u8; IN]) -> [u32; OUT] {
    let needs_byte_swap = assert_has_spirv_magic_number_and_length(&bytes);

    // NOTE: to get around lack of generic const expressions, the input and output lengths must
    // be specified separately.
    // Check that they are consistent with each other.
    assert!(mem::size_of_val(&bytes) == mem::size_of::<[u32; OUT]>());

    // Can't use `bytemuck` in `const fn` (yet), so do it unsafely.
    // SAFETY:
    // * The previous assertion checked that the byte sizes of `bytes` and `words` are equal.
    // * `transmute_copy` doesn't care that the alignment might be wrong.
    let mut words: [u32; OUT] = unsafe { mem::transmute_copy(&bytes) };

    // If necessary, swap bytes to native endianness.
    if needs_byte_swap {
        let mut idx = 0;
        while idx < words.len() {
            words[idx] = words[idx].swap_bytes();
            idx += 1;
        }
    }

    assert!(
        words[0] == SPIRV_MAGIC_NUMBER,
        "can't happen: wrong magic number after swap_bytes"
    );

    words
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    fn test_success_with_misalignments<const IN: usize, const OUT: usize>(
        input: &[u8; IN],
        expected: [u32; OUT],
    ) {
        // We don't know which 3 out of 4 offsets will produce an actually misaligned slice,
        // but they always will. (Note that it is necessary to reuse the same allocation for all 4
        // tests, or we could (in theory) get unlucky and not test any misalignments.)
        let mut buffer = vec![0; input.len() + 4];
        for offset in 0..4 {
            let misaligned_slice: &mut [u8; IN] =
                (&mut buffer[offset..][..input.len()]).try_into().unwrap();

            misaligned_slice.copy_from_slice(input);
            assert_eq!(*make_spirv_raw(misaligned_slice), expected);
            assert_eq!(make_spirv_const(*misaligned_slice), expected);
        }
    }

    #[test]
    fn success_be() {
        // magic number followed by dummy data to see the endianness handling
        let input = b"\x07\x23\x02\x03\xF1\xF2\xF3\xF4";
        let expected: [u32; 2] = [SPIRV_MAGIC_NUMBER, 0xF1F2F3F4];
        test_success_with_misalignments(input, expected);
    }

    #[test]
    fn success_le() {
        let input = b"\x03\x02\x23\x07\xF1\xF2\xF3\xF4";
        let expected: [u32; 2] = [SPIRV_MAGIC_NUMBER, 0xF4F3F2F1];
        test_success_with_misalignments(input, expected);
    }

    #[should_panic = "multiple of 4"]
    #[test]
    fn nonconst_le_fail() {
        let _: Cow<'_, [u32]> = make_spirv_raw(&[0x03, 0x02, 0x23, 0x07, 0x44, 0x33]);
    }

    #[should_panic = "multiple of 4"]
    #[test]
    fn nonconst_be_fail() {
        let _: Cow<'_, [u32]> = make_spirv_raw(&[0x07, 0x23, 0x02, 0x03, 0x11, 0x22]);
    }

    #[should_panic = "multiple of 4"]
    #[test]
    fn const_le_fail() {
        let _: [u32; 1] = make_spirv_const([0x03, 0x02, 0x23, 0x07, 0x44, 0x33]);
    }

    #[should_panic = "multiple of 4"]
    #[test]
    fn const_be_fail() {
        let _: [u32; 1] = make_spirv_const([0x07, 0x23, 0x02, 0x03, 0x11, 0x22]);
    }

    #[should_panic = "byte slice is empty, not SPIR-V"]
    #[test]
    fn make_spirv_empty() {
        let _: [u32; 0] = make_spirv_const([]);
    }
}
