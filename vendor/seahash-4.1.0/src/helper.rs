//! Helper functions.

/// Read a buffer smaller than 8 bytes into an integer in little-endian.
///
/// This assumes that `buf.len() < 8`. If this is not satisfied, the behavior is unspecified.
#[inline(always)]
pub fn read_int(buf: &[u8]) -> u64 {
    // Because we want to make sure that it is register allocated, we fetch this into a variable.
    // It will likely make no difference anyway, though.
    let ptr = buf.as_ptr();

    unsafe {
        // Break it down to reads of integers with widths in total spanning the buffer. This minimizes
        // the number of reads
        match buf.len() {
            // u8.
            1 => *ptr as u64,
            // u16.
            2 => (ptr as *const u16).read_unaligned().to_le() as u64,
            // u16 + u8.
            3 => {
                let a = (ptr as *const u16).read_unaligned().to_le() as u64;
                let b = *ptr.offset(2) as u64;

                a | (b << 16)
            }
            // u32.
            4 => (ptr as *const u32).read_unaligned().to_le() as u64,
            // u32 + u8.
            5 => {
                let a = (ptr as *const u32).read_unaligned().to_le() as u64;
                let b = *ptr.offset(4) as u64;

                a | (b << 32)
            }
            // u32 + u16.
            6 => {
                let a = (ptr as *const u32).read_unaligned().to_le() as u64;
                let b = (ptr.offset(4) as *const u16).read_unaligned().to_le() as u64;

                a | (b << 32)
            }
            // u32 + u16 + u8.
            7 => {
                let a = (ptr as *const u32).read_unaligned().to_le() as u64;
                let b = (ptr.offset(4) as *const u16).read_unaligned().to_le() as u64;
                let c = *ptr.offset(6) as u64;

                a | (b << 32) | (c << 48)
            }
            _ => 0,
        }
    }
}

/// Read a little-endian 64-bit integer from some buffer.
#[inline(always)]
pub unsafe fn read_u64(ptr: *const u8) -> u64 {
    #[cfg(target_pointer_width = "32")]
    {
        // We cannot be sure about the memory layout of a potentially emulated 64-bit integer, so
        // we read it manually. If possible, the compiler should emit proper instructions.
        let a = (ptr as *const u32).read_unaligned().to_le();
        let b = (ptr.offset(4) as *const u32).read_unaligned().to_le();

        a as u64 | ((b as u64) << 32)
    }

    #[cfg(target_pointer_width = "64")]
    {
        (ptr as *const u64).read_unaligned().to_le()
    }
}

/// The diffusion function.
///
/// This is a bijective function emitting chaotic behavior. Such functions are used as building
/// blocks for hash functions.
pub const fn diffuse(mut x: u64) -> u64 {
    // These are derived from the PCG RNG's round. Thanks to @Veedrac for proposing this. The basic
    // idea is that we use dynamic shifts, which are determined by the input itself. The shift is
    // chosen by the higher bits, which means that changing those flips the lower bits, which
    // scatters upwards because of the multiplication.

    x = x.wrapping_mul(0x6eed0e9da4d94a4f);
    let a = x >> 32;
    let b = x >> 60;
    x ^= a >> b;
    x = x.wrapping_mul(0x6eed0e9da4d94a4f);

    x
}

/// Reverse the `diffuse` function.
pub const fn undiffuse(mut x: u64) -> u64 {
    // 0x2f72b4215a3d8caf is the modular multiplicative inverse of the constant used in `diffuse`.

    x = x.wrapping_mul(0x2f72b4215a3d8caf);
    let a = x >> 32;
    let b = x >> 60;
    x ^= a >> b;
    x = x.wrapping_mul(0x2f72b4215a3d8caf);

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    fn diffuse_test(x: u64, y: u64) {
        assert_eq!(diffuse(x), y);
        assert_eq!(x, undiffuse(y));
        assert_eq!(undiffuse(diffuse(x)), x);
    }

    #[test]
    fn read_int_() {
        assert_eq!(read_int(&[2, 3]), 770);
        assert_eq!(read_int(&[3, 2]), 515);
        assert_eq!(read_int(&[3, 2, 5]), 328195);
    }

    #[test]
    fn read_u64_() {
        unsafe {
            assert_eq!(read_u64([1, 0, 0, 0, 0, 0, 0, 0].as_ptr()), 1);
            assert_eq!(read_u64([2, 1, 0, 0, 0, 0, 0, 0].as_ptr()), 258);
        }
    }

    #[test]
    fn diffuse_test_vectors() {
        diffuse_test(94203824938, 17289265692384716055);
        diffuse_test(0xDEADBEEF, 12110756357096144265);
        diffuse_test(0, 0);
        diffuse_test(1, 15197155197312260123);
        diffuse_test(2, 1571904453004118546);
        diffuse_test(3, 16467633989910088880);
    }
}
