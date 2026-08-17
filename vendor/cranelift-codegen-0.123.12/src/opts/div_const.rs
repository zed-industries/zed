//! Compute "magic numbers" for division-by-constants transformations.
//!
//! Math helpers for division by (non-power-of-2) constants. This is based
//! on the presentation in "Hacker's Delight" by Henry Warren, 2003. There
//! are four cases: {unsigned, signed} x {32 bit, 64 bit}. The word size
//! makes little difference, but the signed-vs-unsigned aspect has a large
//! effect. Therefore everything is presented in the order U32 U64 S32 S64
//! so as to emphasise the similarity of the U32 and U64 cases and the S32
//! and S64 cases.

// Structures to hold the "magic numbers" computed.

#[derive(PartialEq, Debug)]
pub struct MU32 {
    pub mul_by: u32,
    pub do_add: bool,
    pub shift_by: i32,
}

#[derive(PartialEq, Debug)]
pub struct MU64 {
    pub mul_by: u64,
    pub do_add: bool,
    pub shift_by: i32,
}

#[derive(PartialEq, Debug)]
pub struct MS32 {
    pub mul_by: i32,
    pub shift_by: i32,
}

#[derive(PartialEq, Debug)]
pub struct MS64 {
    pub mul_by: i64,
    pub shift_by: i32,
}

// The actual "magic number" generators follow.

pub fn magic_u32(d: u32) -> MU32 {
    debug_assert_ne!(d, 0);
    debug_assert_ne!(d, 1); // d==1 generates out of range shifts.

    let mut do_add: bool = false;
    let mut p: i32 = 31;
    let nc: u32 = 0xFFFFFFFFu32 - u32::wrapping_neg(d) % d;
    let mut q1: u32 = 0x80000000u32 / nc;
    let mut r1: u32 = 0x80000000u32 - q1 * nc;
    let mut q2: u32 = 0x7FFFFFFFu32 / d;
    let mut r2: u32 = 0x7FFFFFFFu32 - q2 * d;
    loop {
        p = p + 1;
        if r1 >= nc - r1 {
            q1 = u32::wrapping_add(u32::wrapping_mul(2, q1), 1);
            r1 = u32::wrapping_sub(u32::wrapping_mul(2, r1), nc);
        } else {
            q1 = u32::wrapping_mul(2, q1);
            r1 = 2 * r1;
        }
        if r2 + 1 >= d - r2 {
            if q2 >= 0x7FFFFFFFu32 {
                do_add = true;
            }
            q2 = 2 * q2 + 1;
            r2 = u32::wrapping_sub(u32::wrapping_add(u32::wrapping_mul(2, r2), 1), d);
        } else {
            if q2 >= 0x80000000u32 {
                do_add = true;
            }
            q2 = u32::wrapping_mul(2, q2);
            r2 = 2 * r2 + 1;
        }
        let delta: u32 = d - 1 - r2;
        if !(p < 64 && (q1 < delta || (q1 == delta && r1 == 0))) {
            break;
        }
    }

    MU32 {
        mul_by: q2 + 1,
        do_add,
        shift_by: p - 32,
    }
}

pub fn magic_u64(d: u64) -> MU64 {
    debug_assert_ne!(d, 0);
    debug_assert_ne!(d, 1); // d==1 generates out of range shifts.

    let mut do_add: bool = false;
    let mut p: i32 = 63;
    let nc: u64 = 0xFFFFFFFFFFFFFFFFu64 - u64::wrapping_neg(d) % d;
    let mut q1: u64 = 0x8000000000000000u64 / nc;
    let mut r1: u64 = 0x8000000000000000u64 - q1 * nc;
    let mut q2: u64 = 0x7FFFFFFFFFFFFFFFu64 / d;
    let mut r2: u64 = 0x7FFFFFFFFFFFFFFFu64 - q2 * d;
    loop {
        p = p + 1;
        if r1 >= nc - r1 {
            q1 = u64::wrapping_add(u64::wrapping_mul(2, q1), 1);
            r1 = u64::wrapping_sub(u64::wrapping_mul(2, r1), nc);
        } else {
            q1 = u64::wrapping_mul(2, q1);
            r1 = 2 * r1;
        }
        if r2 + 1 >= d - r2 {
            if q2 >= 0x7FFFFFFFFFFFFFFFu64 {
                do_add = true;
            }
            q2 = 2 * q2 + 1;
            r2 = u64::wrapping_sub(u64::wrapping_add(u64::wrapping_mul(2, r2), 1), d);
        } else {
            if q2 >= 0x8000000000000000u64 {
                do_add = true;
            }
            q2 = u64::wrapping_mul(2, q2);
            r2 = 2 * r2 + 1;
        }
        let delta: u64 = d - 1 - r2;
        if !(p < 128 && (q1 < delta || (q1 == delta && r1 == 0))) {
            break;
        }
    }

    MU64 {
        mul_by: q2 + 1,
        do_add,
        shift_by: p - 64,
    }
}

pub fn magic_s32(d: i32) -> MS32 {
    debug_assert_ne!(d, -1);
    debug_assert_ne!(d, 0);
    debug_assert_ne!(d, 1);
    let two31: u32 = 0x80000000u32;
    let mut p: i32 = 31;
    let ad: u32 = i32::wrapping_abs(d) as u32;
    let t: u32 = two31 + ((d as u32) >> 31);
    let anc: u32 = u32::wrapping_sub(t - 1, t % ad);
    let mut q1: u32 = two31 / anc;
    let mut r1: u32 = two31 - q1 * anc;
    let mut q2: u32 = two31 / ad;
    let mut r2: u32 = two31 - q2 * ad;
    loop {
        p = p + 1;
        q1 = 2 * q1;
        r1 = 2 * r1;
        if r1 >= anc {
            q1 = q1 + 1;
            r1 = r1 - anc;
        }
        q2 = 2 * q2;
        r2 = 2 * r2;
        if r2 >= ad {
            q2 = q2 + 1;
            r2 = r2 - ad;
        }
        let delta: u32 = ad - r2;
        if !(q1 < delta || (q1 == delta && r1 == 0)) {
            break;
        }
    }

    MS32 {
        mul_by: (if d < 0 {
            u32::wrapping_neg(q2 + 1)
        } else {
            q2 + 1
        }) as i32,
        shift_by: p - 32,
    }
}

pub fn magic_s64(d: i64) -> MS64 {
    debug_assert_ne!(d, -1);
    debug_assert_ne!(d, 0);
    debug_assert_ne!(d, 1);
    let two63: u64 = 0x8000000000000000u64;
    let mut p: i32 = 63;
    let ad: u64 = i64::wrapping_abs(d) as u64;
    let t: u64 = two63 + ((d as u64) >> 63);
    let anc: u64 = u64::wrapping_sub(t - 1, t % ad);
    let mut q1: u64 = two63 / anc;
    let mut r1: u64 = two63 - q1 * anc;
    let mut q2: u64 = two63 / ad;
    let mut r2: u64 = two63 - q2 * ad;
    loop {
        p = p + 1;
        q1 = 2 * q1;
        r1 = 2 * r1;
        if r1 >= anc {
            q1 = q1 + 1;
            r1 = r1 - anc;
        }
        q2 = 2 * q2;
        r2 = 2 * r2;
        if r2 >= ad {
            q2 = q2 + 1;
            r2 = r2 - ad;
        }
        let delta: u64 = ad - r2;
        if !(q1 < delta || (q1 == delta && r1 == 0)) {
            break;
        }
    }

    MS64 {
        mul_by: (if d < 0 {
            u64::wrapping_neg(q2 + 1)
        } else {
            q2 + 1
        }) as i64,
        shift_by: p - 64,
    }
}

#[cfg(test)]
mod tests {
    use super::{MS32, MS64, MU32, MU64};
    use super::{magic_s32, magic_s64, magic_u32, magic_u64};
    use proptest::strategy::Strategy;

    fn make_mu32(mul_by: u32, do_add: bool, shift_by: i32) -> MU32 {
        MU32 {
            mul_by,
            do_add,
            shift_by,
        }
    }

    fn make_mu64(mul_by: u64, do_add: bool, shift_by: i32) -> MU64 {
        MU64 {
            mul_by,
            do_add,
            shift_by,
        }
    }

    fn make_ms32(mul_by: i32, shift_by: i32) -> MS32 {
        MS32 { mul_by, shift_by }
    }

    fn make_ms64(mul_by: i64, shift_by: i32) -> MS64 {
        MS64 { mul_by, shift_by }
    }

    #[test]
    fn test_magic_u32() {
        assert_eq!(magic_u32(2u32), make_mu32(0x80000000u32, false, 0));
        assert_eq!(magic_u32(3u32), make_mu32(0xaaaaaaabu32, false, 1));
        assert_eq!(magic_u32(4u32), make_mu32(0x40000000u32, false, 0));
        assert_eq!(magic_u32(5u32), make_mu32(0xcccccccdu32, false, 2));
        assert_eq!(magic_u32(6u32), make_mu32(0xaaaaaaabu32, false, 2));
        assert_eq!(magic_u32(7u32), make_mu32(0x24924925u32, true, 3));
        assert_eq!(magic_u32(9u32), make_mu32(0x38e38e39u32, false, 1));
        assert_eq!(magic_u32(10u32), make_mu32(0xcccccccdu32, false, 3));
        assert_eq!(magic_u32(11u32), make_mu32(0xba2e8ba3u32, false, 3));
        assert_eq!(magic_u32(12u32), make_mu32(0xaaaaaaabu32, false, 3));
        assert_eq!(magic_u32(25u32), make_mu32(0x51eb851fu32, false, 3));
        assert_eq!(magic_u32(125u32), make_mu32(0x10624dd3u32, false, 3));
        assert_eq!(magic_u32(625u32), make_mu32(0xd1b71759u32, false, 9));
        assert_eq!(magic_u32(1337u32), make_mu32(0x88233b2bu32, true, 11));
        assert_eq!(magic_u32(65535u32), make_mu32(0x80008001u32, false, 15));
        assert_eq!(magic_u32(65536u32), make_mu32(0x00010000u32, false, 0));
        assert_eq!(magic_u32(65537u32), make_mu32(0xffff0001u32, false, 16));
        assert_eq!(magic_u32(31415927u32), make_mu32(0x445b4553u32, false, 23));
        assert_eq!(
            magic_u32(0xdeadbeefu32),
            make_mu32(0x93275ab3u32, false, 31)
        );
        assert_eq!(
            magic_u32(0xfffffffdu32),
            make_mu32(0x40000001u32, false, 30)
        );
        assert_eq!(magic_u32(0xfffffffeu32), make_mu32(0x00000003u32, true, 32));
        assert_eq!(
            magic_u32(0xffffffffu32),
            make_mu32(0x80000001u32, false, 31)
        );
    }

    #[test]
    fn test_magic_u64() {
        assert_eq!(magic_u64(2u64), make_mu64(0x8000000000000000u64, false, 0));
        assert_eq!(magic_u64(3u64), make_mu64(0xaaaaaaaaaaaaaaabu64, false, 1));
        assert_eq!(magic_u64(4u64), make_mu64(0x4000000000000000u64, false, 0));
        assert_eq!(magic_u64(5u64), make_mu64(0xcccccccccccccccdu64, false, 2));
        assert_eq!(magic_u64(6u64), make_mu64(0xaaaaaaaaaaaaaaabu64, false, 2));
        assert_eq!(magic_u64(7u64), make_mu64(0x2492492492492493u64, true, 3));
        assert_eq!(magic_u64(9u64), make_mu64(0xe38e38e38e38e38fu64, false, 3));
        assert_eq!(magic_u64(10u64), make_mu64(0xcccccccccccccccdu64, false, 3));
        assert_eq!(magic_u64(11u64), make_mu64(0x2e8ba2e8ba2e8ba3u64, false, 1));
        assert_eq!(magic_u64(12u64), make_mu64(0xaaaaaaaaaaaaaaabu64, false, 3));
        assert_eq!(magic_u64(25u64), make_mu64(0x47ae147ae147ae15u64, true, 5));
        assert_eq!(magic_u64(125u64), make_mu64(0x0624dd2f1a9fbe77u64, true, 7));
        assert_eq!(
            magic_u64(625u64),
            make_mu64(0x346dc5d63886594bu64, false, 7)
        );
        assert_eq!(
            magic_u64(1337u64),
            make_mu64(0xc4119d952866a139u64, false, 10)
        );
        assert_eq!(
            magic_u64(31415927u64),
            make_mu64(0x116d154b9c3d2f85u64, true, 25)
        );
        assert_eq!(
            magic_u64(0x00000000deadbeefu64),
            make_mu64(0x93275ab2dfc9094bu64, false, 31)
        );
        assert_eq!(
            magic_u64(0x00000000fffffffdu64),
            make_mu64(0x8000000180000005u64, false, 31)
        );
        assert_eq!(
            magic_u64(0x00000000fffffffeu64),
            make_mu64(0x0000000200000005u64, true, 32)
        );
        assert_eq!(
            magic_u64(0x00000000ffffffffu64),
            make_mu64(0x8000000080000001u64, false, 31)
        );
        assert_eq!(
            magic_u64(0x0000000100000000u64),
            make_mu64(0x0000000100000000u64, false, 0)
        );
        assert_eq!(
            magic_u64(0x0000000100000001u64),
            make_mu64(0xffffffff00000001u64, false, 32)
        );
        assert_eq!(
            magic_u64(0x0ddc0ffeebadf00du64),
            make_mu64(0x2788e9d394b77da1u64, true, 60)
        );
        assert_eq!(
            magic_u64(0xfffffffffffffffdu64),
            make_mu64(0x4000000000000001u64, false, 62)
        );
        assert_eq!(
            magic_u64(0xfffffffffffffffeu64),
            make_mu64(0x0000000000000003u64, true, 64)
        );
        assert_eq!(
            magic_u64(0xffffffffffffffffu64),
            make_mu64(0x8000000000000001u64, false, 63)
        );
    }

    #[test]
    fn test_magic_s32() {
        assert_eq!(
            magic_s32(-0x80000000i32),
            make_ms32(0x7fffffffu32 as i32, 30)
        );
        assert_eq!(
            magic_s32(-0x7FFFFFFFi32),
            make_ms32(0xbfffffffu32 as i32, 29)
        );
        assert_eq!(
            magic_s32(-0x7FFFFFFEi32),
            make_ms32(0x7ffffffdu32 as i32, 30)
        );
        assert_eq!(magic_s32(-31415927i32), make_ms32(0xbba4baadu32 as i32, 23));
        assert_eq!(magic_s32(-1337i32), make_ms32(0x9df73135u32 as i32, 9));
        assert_eq!(magic_s32(-256i32), make_ms32(0x7fffffffu32 as i32, 7));
        assert_eq!(magic_s32(-5i32), make_ms32(0x99999999u32 as i32, 1));
        assert_eq!(magic_s32(-3i32), make_ms32(0x55555555u32 as i32, 1));
        assert_eq!(magic_s32(-2i32), make_ms32(0x7fffffffu32 as i32, 0));
        assert_eq!(magic_s32(2i32), make_ms32(0x80000001u32 as i32, 0));
        assert_eq!(magic_s32(3i32), make_ms32(0x55555556u32 as i32, 0));
        assert_eq!(magic_s32(4i32), make_ms32(0x80000001u32 as i32, 1));
        assert_eq!(magic_s32(5i32), make_ms32(0x66666667u32 as i32, 1));
        assert_eq!(magic_s32(6i32), make_ms32(0x2aaaaaabu32 as i32, 0));
        assert_eq!(magic_s32(7i32), make_ms32(0x92492493u32 as i32, 2));
        assert_eq!(magic_s32(9i32), make_ms32(0x38e38e39u32 as i32, 1));
        assert_eq!(magic_s32(10i32), make_ms32(0x66666667u32 as i32, 2));
        assert_eq!(magic_s32(11i32), make_ms32(0x2e8ba2e9u32 as i32, 1));
        assert_eq!(magic_s32(12i32), make_ms32(0x2aaaaaabu32 as i32, 1));
        assert_eq!(magic_s32(25i32), make_ms32(0x51eb851fu32 as i32, 3));
        assert_eq!(magic_s32(125i32), make_ms32(0x10624dd3u32 as i32, 3));
        assert_eq!(magic_s32(625i32), make_ms32(0x68db8badu32 as i32, 8));
        assert_eq!(magic_s32(1337i32), make_ms32(0x6208cecbu32 as i32, 9));
        assert_eq!(magic_s32(31415927i32), make_ms32(0x445b4553u32 as i32, 23));
        assert_eq!(
            magic_s32(0x7ffffffei32),
            make_ms32(0x80000003u32 as i32, 30)
        );
        assert_eq!(
            magic_s32(0x7fffffffi32),
            make_ms32(0x40000001u32 as i32, 29)
        );
    }

    #[test]
    fn test_magic_s64() {
        assert_eq!(
            magic_s64(-0x8000000000000000i64),
            make_ms64(0x7fffffffffffffffu64 as i64, 62)
        );
        assert_eq!(
            magic_s64(-0x7FFFFFFFFFFFFFFFi64),
            make_ms64(0xbfffffffffffffffu64 as i64, 61)
        );
        assert_eq!(
            magic_s64(-0x7FFFFFFFFFFFFFFEi64),
            make_ms64(0x7ffffffffffffffdu64 as i64, 62)
        );
        assert_eq!(
            magic_s64(-0x0ddC0ffeeBadF00di64),
            make_ms64(0x6c3b8b1635a4412fu64 as i64, 59)
        );
        assert_eq!(
            magic_s64(-0x100000001i64),
            make_ms64(0x800000007fffffffu64 as i64, 31)
        );
        assert_eq!(
            magic_s64(-0x100000000i64),
            make_ms64(0x7fffffffffffffffu64 as i64, 31)
        );
        assert_eq!(
            magic_s64(-0xFFFFFFFFi64),
            make_ms64(0x7fffffff7fffffffu64 as i64, 31)
        );
        assert_eq!(
            magic_s64(-0xFFFFFFFEi64),
            make_ms64(0x7ffffffefffffffdu64 as i64, 31)
        );
        assert_eq!(
            magic_s64(-0xFFFFFFFDi64),
            make_ms64(0x7ffffffe7ffffffbu64 as i64, 31)
        );
        assert_eq!(
            magic_s64(-0xDeadBeefi64),
            make_ms64(0x6cd8a54d2036f6b5u64 as i64, 31)
        );
        assert_eq!(
            magic_s64(-31415927i64),
            make_ms64(0x7749755a31e1683du64 as i64, 24)
        );
        assert_eq!(
            magic_s64(-1337i64),
            make_ms64(0x9df731356bccaf63u64 as i64, 9)
        );
        assert_eq!(
            magic_s64(-256i64),
            make_ms64(0x7fffffffffffffffu64 as i64, 7)
        );
        assert_eq!(magic_s64(-5i64), make_ms64(0x9999999999999999u64 as i64, 1));
        assert_eq!(magic_s64(-3i64), make_ms64(0x5555555555555555u64 as i64, 1));
        assert_eq!(magic_s64(-2i64), make_ms64(0x7fffffffffffffffu64 as i64, 0));
        assert_eq!(magic_s64(2i64), make_ms64(0x8000000000000001u64 as i64, 0));
        assert_eq!(magic_s64(3i64), make_ms64(0x5555555555555556u64 as i64, 0));
        assert_eq!(magic_s64(4i64), make_ms64(0x8000000000000001u64 as i64, 1));
        assert_eq!(magic_s64(5i64), make_ms64(0x6666666666666667u64 as i64, 1));
        assert_eq!(magic_s64(6i64), make_ms64(0x2aaaaaaaaaaaaaabu64 as i64, 0));
        assert_eq!(magic_s64(7i64), make_ms64(0x4924924924924925u64 as i64, 1));
        assert_eq!(magic_s64(9i64), make_ms64(0x1c71c71c71c71c72u64 as i64, 0));
        assert_eq!(magic_s64(10i64), make_ms64(0x6666666666666667u64 as i64, 2));
        assert_eq!(magic_s64(11i64), make_ms64(0x2e8ba2e8ba2e8ba3u64 as i64, 1));
        assert_eq!(magic_s64(12i64), make_ms64(0x2aaaaaaaaaaaaaabu64 as i64, 1));
        assert_eq!(magic_s64(25i64), make_ms64(0xa3d70a3d70a3d70bu64 as i64, 4));
        assert_eq!(
            magic_s64(125i64),
            make_ms64(0x20c49ba5e353f7cfu64 as i64, 4)
        );
        assert_eq!(
            magic_s64(625i64),
            make_ms64(0x346dc5d63886594bu64 as i64, 7)
        );
        assert_eq!(
            magic_s64(1337i64),
            make_ms64(0x6208ceca9433509du64 as i64, 9)
        );
        assert_eq!(
            magic_s64(31415927i64),
            make_ms64(0x88b68aa5ce1e97c3u64 as i64, 24)
        );
        assert_eq!(
            magic_s64(0x00000000deadbeefi64),
            make_ms64(0x93275ab2dfc9094bu64 as i64, 31)
        );
        assert_eq!(
            magic_s64(0x00000000fffffffdi64),
            make_ms64(0x8000000180000005u64 as i64, 31)
        );
        assert_eq!(
            magic_s64(0x00000000fffffffei64),
            make_ms64(0x8000000100000003u64 as i64, 31)
        );
        assert_eq!(
            magic_s64(0x00000000ffffffffi64),
            make_ms64(0x8000000080000001u64 as i64, 31)
        );
        assert_eq!(
            magic_s64(0x0000000100000000i64),
            make_ms64(0x8000000000000001u64 as i64, 31)
        );
        assert_eq!(
            magic_s64(0x0000000100000001i64),
            make_ms64(0x7fffffff80000001u64 as i64, 31)
        );
        assert_eq!(
            magic_s64(0x0ddc0ffeebadf00di64),
            make_ms64(0x93c474e9ca5bbed1u64 as i64, 59)
        );
        assert_eq!(
            magic_s64(0x7ffffffffffffffdi64),
            make_ms64(0x2000000000000001u64 as i64, 60)
        );
        assert_eq!(
            magic_s64(0x7ffffffffffffffei64),
            make_ms64(0x8000000000000003u64 as i64, 62)
        );
        assert_eq!(
            magic_s64(0x7fffffffffffffffi64),
            make_ms64(0x4000000000000001u64 as i64, 61)
        );
    }

    #[test]
    fn test_magic_generators_dont_panic() {
        // The point of this is to check that the magic number generators
        // don't panic with integer wraparounds, especially at boundary cases
        // for their arguments. The actual results are thrown away, although
        // we force `total` to be used, so that rustc can't optimise the
        // entire computation away.

        // Testing UP magic_u32
        let mut total: u64 = 0;
        for x in 2..(200 * 1000u32) {
            let m = magic_u32(x);
            total = total ^ (m.mul_by as u64);
            total = total + (m.shift_by as u64);
            total = total + (if m.do_add { 123 } else { 456 });
        }
        assert_eq!(total, 2481999609);

        total = 0;
        // Testing MIDPOINT magic_u32
        for x in 0x8000_0000u32 - 10 * 1000u32..0x8000_0000u32 + 10 * 1000u32 {
            let m = magic_u32(x);
            total = total ^ (m.mul_by as u64);
            total = total + (m.shift_by as u64);
            total = total + (if m.do_add { 123 } else { 456 });
        }
        assert_eq!(total, 2399809723);

        total = 0;
        // Testing DOWN magic_u32
        for x in 0..(200 * 1000u32) {
            let m = magic_u32(0xFFFF_FFFFu32 - x);
            total = total ^ (m.mul_by as u64);
            total = total + (m.shift_by as u64);
            total = total + (if m.do_add { 123 } else { 456 });
        }
        assert_eq!(total, 271138267);

        // Testing UP magic_u64
        total = 0;
        for x in 2..(200 * 1000u64) {
            let m = magic_u64(x);
            total = total ^ m.mul_by;
            total = total + (m.shift_by as u64);
            total = total + (if m.do_add { 123 } else { 456 });
        }
        assert_eq!(total, 7430004086976261161);

        total = 0;
        // Testing MIDPOINT magic_u64
        for x in 0x8000_0000_0000_0000u64 - 10 * 1000u64..0x8000_0000_0000_0000u64 + 10 * 1000u64 {
            let m = magic_u64(x);
            total = total ^ m.mul_by;
            total = total + (m.shift_by as u64);
            total = total + (if m.do_add { 123 } else { 456 });
        }
        assert_eq!(total, 10312117246769520603);

        // Testing DOWN magic_u64
        total = 0;
        for x in 0..(200 * 1000u64) {
            let m = magic_u64(0xFFFF_FFFF_FFFF_FFFFu64 - x);
            total = total ^ m.mul_by;
            total = total + (m.shift_by as u64);
            total = total + (if m.do_add { 123 } else { 456 });
        }
        assert_eq!(total, 1126603594357269734);

        // Testing UP magic_s32
        total = 0;
        for x in 0..(200 * 1000i32) {
            let m = magic_s32(-0x8000_0000i32 + x);
            total = total ^ (m.mul_by as u64);
            total = total + (m.shift_by as u64);
        }
        assert_eq!(total, 18446744069953376812);

        total = 0;
        // Testing MIDPOINT magic_s32
        for x in 0..(200 * 1000i32) {
            let x2 = -100 * 1000i32 + x;
            if x2 != -1 && x2 != 0 && x2 != 1 {
                let m = magic_s32(x2);
                total = total ^ (m.mul_by as u64);
                total = total + (m.shift_by as u64);
            }
        }
        assert_eq!(total, 351839350);

        // Testing DOWN magic_s32
        total = 0;
        for x in 0..(200 * 1000i32) {
            let m = magic_s32(0x7FFF_FFFFi32 - x);
            total = total ^ (m.mul_by as u64);
            total = total + (m.shift_by as u64);
        }
        assert_eq!(total, 18446744072916880714);

        // Testing UP magic_s64
        total = 0;
        for x in 0..(200 * 1000i64) {
            let m = magic_s64(-0x8000_0000_0000_0000i64 + x);
            total = total ^ (m.mul_by as u64);
            total = total + (m.shift_by as u64);
        }
        assert_eq!(total, 17929885647724831014);

        total = 0;
        // Testing MIDPOINT magic_s64
        for x in 0..(200 * 1000i64) {
            let x2 = -100 * 1000i64 + x;
            if x2 != -1 && x2 != 0 && x2 != 1 {
                let m = magic_s64(x2);
                total = total ^ (m.mul_by as u64);
                total = total + (m.shift_by as u64);
            }
        }
        assert_eq!(total, 18106042338125661964);

        // Testing DOWN magic_s64
        total = 0;
        for x in 0..(200 * 1000i64) {
            let m = magic_s64(0x7FFF_FFFF_FFFF_FFFFi64 - x);
            total = total ^ (m.mul_by as u64);
            total = total + (m.shift_by as u64);
        }
        assert_eq!(total, 563301797155560970);
    }

    // These generate reference results for signed/unsigned 32/64 bit
    // division, rounding towards zero.
    fn div_u32(x: u32, y: u32) -> u32 {
        return x / y;
    }
    fn div_s32(x: i32, y: i32) -> i32 {
        return x / y;
    }
    fn div_u64(x: u64, y: u64) -> u64 {
        return x / y;
    }
    fn div_s64(x: i64, y: i64) -> i64 {
        return x / y;
    }

    // Returns the high half of a 32 bit unsigned widening multiply.
    fn mulhw_u32(x: u32, y: u32) -> u32 {
        let x64: u64 = x as u64;
        let y64: u64 = y as u64;
        let r64: u64 = x64 * y64;
        (r64 >> 32) as u32
    }

    // Returns the high half of a 32 bit signed widening multiply.
    fn mulhw_s32(x: i32, y: i32) -> i32 {
        let x64: i64 = x as i64;
        let y64: i64 = y as i64;
        let r64: i64 = x64 * y64;
        (r64 >> 32) as i32
    }

    // Returns the high half of a 64 bit unsigned widening multiply.
    fn mulhw_u64(x: u64, y: u64) -> u64 {
        let x128 = x as u128;
        let y128 = y as u128;
        let r128 = x128 * y128;
        (r128 >> 64) as u64
    }

    // Returns the high half of a 64 bit signed widening multiply.
    fn mulhw_s64(x: i64, y: i64) -> i64 {
        let x128 = x as i128;
        let y128 = y as i128;
        let r128 = x128 * y128;
        (r128 >> 64) as i64
    }

    /// Compute and check `q = n / d` using `magic`.
    fn eval_magic_u32_div(n: u32, magic: &MU32) -> u32 {
        let mut q = mulhw_u32(n, magic.mul_by);
        if magic.do_add {
            assert!(magic.shift_by >= 1 && magic.shift_by <= 32);
            let mut t = n - q;
            t >>= 1;
            t = t + q;
            q = t >> (magic.shift_by - 1);
        } else {
            assert!(magic.shift_by >= 0 && magic.shift_by <= 31);
            q >>= magic.shift_by;
        }
        q
    }

    /// Compute and check `r = n % d` using `magic`.
    fn eval_magic_u32_rem(n: u32, d: u32, magic: &MU32) -> u32 {
        let mut q = mulhw_u32(n, magic.mul_by);
        if magic.do_add {
            assert!(magic.shift_by >= 1 && magic.shift_by <= 32);
            let mut t = n - q;
            t >>= 1;
            t = t + q;
            q = t >> (magic.shift_by - 1);
        } else {
            assert!(magic.shift_by >= 0 && magic.shift_by <= 31);
            q >>= magic.shift_by;
        }
        let tt = q.wrapping_mul(d);
        n.wrapping_sub(tt)
    }

    /// Compute and check `q = n / d` using `magic`.
    fn eval_magic_u64_div(n: u64, magic: &MU64) -> u64 {
        let mut q = mulhw_u64(n, magic.mul_by);
        if magic.do_add {
            assert!(magic.shift_by >= 1 && magic.shift_by <= 64);
            let mut t = n - q;
            t >>= 1;
            t = t + q;
            q = t >> (magic.shift_by - 1);
        } else {
            assert!(magic.shift_by >= 0 && magic.shift_by <= 63);
            q >>= magic.shift_by;
        }
        q
    }

    /// Compute and check `r = n % d` using `magic`.
    fn eval_magic_u64_rem(n: u64, d: u64, magic: &MU64) -> u64 {
        let mut q = mulhw_u64(n, magic.mul_by);
        if magic.do_add {
            assert!(magic.shift_by >= 1 && magic.shift_by <= 64);
            let mut t = n - q;
            t >>= 1;
            t = t + q;
            q = t >> (magic.shift_by - 1);
        } else {
            assert!(magic.shift_by >= 0 && magic.shift_by <= 63);
            q >>= magic.shift_by;
        }
        let tt = q.wrapping_mul(d);
        n.wrapping_sub(tt)
    }

    /// Compute and check `q = n / d` using `magic`.
    fn eval_magic_s32_div(n: i32, d: i32, magic: &MS32) -> i32 {
        let mut q: i32 = mulhw_s32(n, magic.mul_by);
        if d > 0 && magic.mul_by < 0 {
            q = q + n;
        } else if d < 0 && magic.mul_by > 0 {
            q = q - n;
        }
        assert!(magic.shift_by >= 0 && magic.shift_by <= 31);
        q = q >> magic.shift_by;
        let mut t: u32 = q as u32;
        t = t >> 31;
        q = q + (t as i32);
        q
    }

    /// Compute and check `r = n % d` using `magic`.
    fn eval_magic_s32_rem(n: i32, d: i32, magic: &MS32) -> i32 {
        let mut q: i32 = mulhw_s32(n, magic.mul_by);
        if d > 0 && magic.mul_by < 0 {
            q = q + n;
        } else if d < 0 && magic.mul_by > 0 {
            q = q - n;
        }
        assert!(magic.shift_by >= 0 && magic.shift_by <= 31);
        q = q >> magic.shift_by;
        let mut t: u32 = q as u32;
        t = t >> 31;
        q = q + (t as i32);
        let tt = q.wrapping_mul(d);
        n.wrapping_sub(tt)
    }

    /// Compute and check `q = n / d` using `magic`. */
    fn eval_magic_s64_div(n: i64, d: i64, magic: &MS64) -> i64 {
        let mut q: i64 = mulhw_s64(n, magic.mul_by);
        if d > 0 && magic.mul_by < 0 {
            q = q + n;
        } else if d < 0 && magic.mul_by > 0 {
            q = q - n;
        }
        assert!(magic.shift_by >= 0 && magic.shift_by <= 63);
        q = q >> magic.shift_by;
        let mut t: u64 = q as u64;
        t = t >> 63;
        q = q + (t as i64);
        q
    }

    /// Compute and check `r = n % d` using `magic`.
    fn eval_magic_s64_rem(n: i64, d: i64, magic: &MS64) -> i64 {
        let mut q: i64 = mulhw_s64(n, magic.mul_by);
        if d > 0 && magic.mul_by < 0 {
            q = q + n;
        } else if d < 0 && magic.mul_by > 0 {
            q = q - n;
        }
        assert!(magic.shift_by >= 0 && magic.shift_by <= 63);
        q = q >> magic.shift_by;
        let mut t: u64 = q as u64;
        t = t >> 63;
        q = q + (t as i64);
        let tt = q.wrapping_mul(d);
        n.wrapping_sub(tt)
    }

    #[test]
    fn test_magic_generators_give_correct_numbers() {
        // For a variety of values for both `n` and `d`, compute the magic
        // numbers for `d`, and in effect interpret them so as to compute
        // `n / d`.  Check that that equals the value of `n / d` computed
        // directly by the hardware.  This serves to check that the magic
        // number generates work properly.  In total, 50,148,000 tests are
        // done.

        // Some constants
        const MIN_U32: u32 = 0;
        const MAX_U32: u32 = 0xFFFF_FFFFu32;
        const MAX_U32_HALF: u32 = 0x8000_0000u32; // more or less

        const MIN_S32: i32 = 0x8000_0000u32 as i32;
        const MAX_S32: i32 = 0x7FFF_FFFFu32 as i32;

        const MIN_U64: u64 = 0;
        const MAX_U64: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;
        const MAX_U64_HALF: u64 = 0x8000_0000_0000_0000u64; // ditto

        const MIN_S64: i64 = 0x8000_0000_0000_0000u64 as i64;
        const MAX_S64: i64 = 0x7FFF_FFFF_FFFF_FFFFu64 as i64;

        // Compute the magic numbers for `d` and then use them to compute and
        // check `n / d` for around 1000 values of `n`, using unsigned 32-bit
        // division.
        fn test_magic_u32_inner(d: u32, n_tests_done: &mut i32) {
            // Advance the numerator (the `n` in `n / d`) so as to test
            // densely near the range ends (and, in the signed variants, near
            // zero) but not so densely away from those regions.
            fn advance_n_u32(x: u32) -> u32 {
                if x < MIN_U32 + 110 {
                    return x + 1;
                }
                if x < MIN_U32 + 1700 {
                    return x + 23;
                }
                if x < MAX_U32 - 1700 {
                    let xd: f64 = (x as f64) * 1.06415927;
                    return if xd >= (MAX_U32 - 1700) as f64 {
                        MAX_U32 - 1700
                    } else {
                        xd as u32
                    };
                }
                if x < MAX_U32 - 110 {
                    return x + 23;
                }
                u32::wrapping_add(x, 1)
            }

            let magic: MU32 = magic_u32(d);
            let mut n: u32 = MIN_U32;
            loop {
                *n_tests_done += 1;
                let q = eval_magic_u32_div(n, &magic);
                assert_eq!(q, div_u32(n, d));

                n = advance_n_u32(n);
                if n == MIN_U32 {
                    break;
                }
            }
        }

        // Compute the magic numbers for `d` and then use them to compute and
        // check `n / d` for around 1000 values of `n`, using signed 32-bit
        // division.
        fn test_magic_s32_inner(d: i32, n_tests_done: &mut i32) {
            // See comment on advance_n_u32 above.
            fn advance_n_s32(x: i32) -> i32 {
                if x >= 0 && x <= 29 {
                    return x + 1;
                }
                if x < MIN_S32 + 110 {
                    return x + 1;
                }
                if x < MIN_S32 + 1700 {
                    return x + 23;
                }
                if x < MAX_S32 - 1700 {
                    let mut xd: f64 = x as f64;
                    xd = if xd < 0.0 {
                        xd / 1.06415927
                    } else {
                        xd * 1.06415927
                    };
                    return if xd >= (MAX_S32 - 1700) as f64 {
                        MAX_S32 - 1700
                    } else {
                        xd as i32
                    };
                }
                if x < MAX_S32 - 110 {
                    return x + 23;
                }
                if x == MAX_S32 {
                    return MIN_S32;
                }
                x + 1
            }

            let magic: MS32 = magic_s32(d);
            let mut n: i32 = MIN_S32;
            loop {
                *n_tests_done += 1;
                let q = eval_magic_s32_div(n, d, &magic);
                assert_eq!(q, div_s32(n, d));

                n = advance_n_s32(n);
                if n == MIN_S32 {
                    break;
                }
            }
        }

        // Compute the magic numbers for `d` and then use them to compute and
        // check `n / d` for around 1000 values of `n`, using unsigned 64-bit
        // division.
        fn test_magic_u64_inner(d: u64, n_tests_done: &mut i32) {
            // See comment on advance_n_u32 above.
            fn advance_n_u64(x: u64) -> u64 {
                if x < MIN_U64 + 110 {
                    return x + 1;
                }
                if x < MIN_U64 + 1700 {
                    return x + 23;
                }
                if x < MAX_U64 - 1700 {
                    let xd: f64 = (x as f64) * 1.06415927;
                    return if xd >= (MAX_U64 - 1700) as f64 {
                        MAX_U64 - 1700
                    } else {
                        xd as u64
                    };
                }
                if x < MAX_U64 - 110 {
                    return x + 23;
                }
                u64::wrapping_add(x, 1)
            }

            let magic: MU64 = magic_u64(d);
            let mut n: u64 = MIN_U64;
            loop {
                *n_tests_done += 1;
                let q = eval_magic_u64_div(n, &magic);
                assert_eq!(q, div_u64(n, d));

                n = advance_n_u64(n);
                if n == MIN_U64 {
                    break;
                }
            }
        }

        // Compute the magic numbers for `d` and then use them to compute and
        // check `n / d` for around 1000 values of `n`, using signed 64-bit
        // division.
        fn test_magic_s64_inner(d: i64, n_tests_done: &mut i32) {
            // See comment on advance_n_u32 above.
            fn advance_n_s64(x: i64) -> i64 {
                if x >= 0 && x <= 29 {
                    return x + 1;
                }
                if x < MIN_S64 + 110 {
                    return x + 1;
                }
                if x < MIN_S64 + 1700 {
                    return x + 23;
                }
                if x < MAX_S64 - 1700 {
                    let mut xd: f64 = x as f64;
                    xd = if xd < 0.0 {
                        xd / 1.06415927
                    } else {
                        xd * 1.06415927
                    };
                    return if xd >= (MAX_S64 - 1700) as f64 {
                        MAX_S64 - 1700
                    } else {
                        xd as i64
                    };
                }
                if x < MAX_S64 - 110 {
                    return x + 23;
                }
                if x == MAX_S64 {
                    return MIN_S64;
                }
                x + 1
            }

            let magic: MS64 = magic_s64(d);
            let mut n: i64 = MIN_S64;
            loop {
                *n_tests_done += 1;
                let q = eval_magic_s64_div(n, d, &magic);
                assert_eq!(q, div_s64(n, d));

                n = advance_n_s64(n);
                if n == MIN_S64 {
                    break;
                }
            }
        }

        // Using all the above support machinery, actually run the tests.

        let mut n_tests_done: i32 = 0;

        // u32 division tests
        {
            // 2 .. 3k
            let mut d: u32 = 2;
            for _ in 0..3 * 1000 {
                test_magic_u32_inner(d, &mut n_tests_done);
                d += 1;
            }

            // across the midpoint: midpoint - 3k .. midpoint + 3k
            d = MAX_U32_HALF - 3 * 1000;
            for _ in 0..2 * 3 * 1000 {
                test_magic_u32_inner(d, &mut n_tests_done);
                d += 1;
            }

            // MAX_U32 - 3k .. MAX_U32 (in reverse order)
            d = MAX_U32;
            for _ in 0..3 * 1000 {
                test_magic_u32_inner(d, &mut n_tests_done);
                d -= 1;
            }
        }

        // s32 division tests
        {
            // MIN_S32 .. MIN_S32 + 3k
            let mut d: i32 = MIN_S32;
            for _ in 0..3 * 1000 {
                test_magic_s32_inner(d, &mut n_tests_done);
                d += 1;
            }

            // -3k .. -2 (in reverse order)
            d = -2;
            for _ in 0..3 * 1000 {
                test_magic_s32_inner(d, &mut n_tests_done);
                d -= 1;
            }

            // 2 .. 3k
            d = 2;
            for _ in 0..3 * 1000 {
                test_magic_s32_inner(d, &mut n_tests_done);
                d += 1;
            }

            // MAX_S32 - 3k .. MAX_S32 (in reverse order)
            d = MAX_S32;
            for _ in 0..3 * 1000 {
                test_magic_s32_inner(d, &mut n_tests_done);
                d -= 1;
            }
        }

        // u64 division tests
        {
            // 2 .. 3k
            let mut d: u64 = 2;
            for _ in 0..3 * 1000 {
                test_magic_u64_inner(d, &mut n_tests_done);
                d += 1;
            }

            // across the midpoint: midpoint - 3k .. midpoint + 3k
            d = MAX_U64_HALF - 3 * 1000;
            for _ in 0..2 * 3 * 1000 {
                test_magic_u64_inner(d, &mut n_tests_done);
                d += 1;
            }

            // mAX_U64 - 3000 .. mAX_U64 (in reverse order)
            d = MAX_U64;
            for _ in 0..3 * 1000 {
                test_magic_u64_inner(d, &mut n_tests_done);
                d -= 1;
            }
        }

        // s64 division tests
        {
            // MIN_S64 .. MIN_S64 + 3k
            let mut d: i64 = MIN_S64;
            for _ in 0..3 * 1000 {
                test_magic_s64_inner(d, &mut n_tests_done);
                d += 1;
            }

            // -3k .. -2 (in reverse order)
            d = -2;
            for _ in 0..3 * 1000 {
                test_magic_s64_inner(d, &mut n_tests_done);
                d -= 1;
            }

            // 2 .. 3k
            d = 2;
            for _ in 0..3 * 1000 {
                test_magic_s64_inner(d, &mut n_tests_done);
                d += 1;
            }

            // MAX_S64 - 3k .. MAX_S64 (in reverse order)
            d = MAX_S64;
            for _ in 0..3 * 1000 {
                test_magic_s64_inner(d, &mut n_tests_done);
                d -= 1;
            }
        }
        assert_eq!(n_tests_done, 50_148_000);
    }

    proptest::proptest! {
        #[test]
        fn proptest_magic_u32(numerator in 0..u32::MAX, divisor in 1..u32::MAX) {
            let expected_div = numerator.wrapping_div(divisor);
            let expected_rem = numerator.wrapping_rem(divisor);

            let magic = magic_u32(divisor);
            let actual_div = eval_magic_u32_div(numerator, &magic);
            let actual_rem = eval_magic_u32_rem(numerator, divisor, &magic);

            assert_eq!(expected_div, actual_div);
            assert_eq!(expected_rem, actual_rem);
        }

        #[test]
        fn proptest_magic_u64(numerator in 0..u64::MAX, divisor in 1..u64::MAX) {
            let expected_div = numerator.wrapping_div(divisor);
            let expected_rem = numerator.wrapping_rem(divisor);

            let magic = magic_u64(divisor);
            let actual_div = eval_magic_u64_div(numerator, &magic);
            let actual_rem = eval_magic_u64_rem(numerator, divisor, &magic);

            assert_eq!(expected_div, actual_div);
            assert_eq!(expected_rem, actual_rem);
        }

        #[test]
        fn proptest_magic_s32(
            numerator in i32::MIN..i32::MAX,
            divisor in (i32::MIN..i32::MAX).prop_filter("avoid divide-by-zero", |d| *d != 0),
        ) {
            let expected_div = numerator.wrapping_div(divisor);
            let expected_rem = numerator.wrapping_rem(divisor);

            let magic = magic_s32(divisor);
            let actual_div = eval_magic_s32_div(numerator, divisor, &magic);
            let actual_rem = eval_magic_s32_rem(numerator, divisor, &magic);

            assert_eq!(expected_div, actual_div);
            assert_eq!(expected_rem, actual_rem);
        }

        #[test]
        fn proptest_magic_s64(
            numerator in i64::MIN..i64::MAX,
            divisor in (i64::MIN..i64::MAX).prop_filter("avoid divide-by-zero", |d| *d != 0),
        ) {
            let expected_div = numerator.wrapping_div(divisor);
            let expected_rem = numerator.wrapping_rem(divisor);

            let magic = magic_s64(divisor);
            let actual_div = eval_magic_s64_div(numerator, divisor, &magic);
            let actual_rem = eval_magic_s64_rem(numerator, divisor, &magic);

            assert_eq!(expected_div, actual_div);
            assert_eq!(expected_rem, actual_rem);
        }
    }
}
