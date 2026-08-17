// Common routines for timestamp normalization.
//
// This is split out into its own file so that the tests in `tests` can include
// it without including the normal endpoints and interface definitions.

/// 64-bit unsigned integer type.
///
/// We cannot rely on native 64-bit integers, so we define our own 64-bit
/// integer type as two 32-bit integers.
struct Uint64 {
    /// Least significant word.
    low: u32,
    /// Most significant word.
    high: u32,
}

/// 96-bit unsigned integer type.
struct Uint96 {
    /// Least significant word.
    low: u32,
    /// Middle word.
    mid: u32,
    /// Most significant word.
    high: u32,
}

/// Truncates a 96-bit number to a 64-bit number by discarding the upper 32 bits.
fn truncate_u96_to_u64(x: Uint96) -> Uint64 {
    return Uint64(
        x.low,
        x.mid,
    );
}

/// Returns the lower 16 bits of a 32-bit integer.
fn low(a: u32) -> u32 {
    return a & 0xFFFF;
}

/// Returns the upper 16 bits of a 32-bit integer.
fn high(a: u32) -> u32 {
    return a >> 16;
}

/// Combines two 16bit words into a single 32bit word. 
/// `w1` is the upper 16 bits and `w0` is the lower 16 bits.
///
/// The high 16 bits of each argument are discarded.
fn u32_from_u16s(w1: u32, w0: u32) -> u32 {
    return low(w1) << 16 | low(w0);
}

// Multiplies a 64-bit number by a 32-bit number and outputs a 96-bit result.
//
// The number of digits (bits) needed to represent the result of a multiplication
// is the sum of the number of input digits (bits). Since we are multiplying a
// 64-bit number by a 32-bit number, we need 96 bits to represent the result.
fn u64_mul_u32(a: Uint64, b: u32) -> Uint96 {
    // Does not use any 64-bit operations and we don't have access to `mul(u32, u32) -> u64`
    // operations, so we operate entirely on `mul(u16, u16) -> u32`.

    // This implements standard "long multiplication" algorithm using 16-bit words.
    // Each element in this diagram is a 16-bit word.
    //
    //                  a3  a2  a1  a0
    //               *          b1  b0
    //     ----------------------------
    // i0 =                    p00 p00
    // i1 =                p10 p10
    // i2 =            p20 p20
    // i3 =        p30 p30
    // i4 =                p01 p01
    // i5 =            p11 p11
    // i6 =        p21 p21
    // i7 =    p31 p31
    //     ----------------------------
    //      r6  r5  r4  r3  r2  r1  r0

    // Decompose the 64-bit number into four 16-bit words.
    let a0 = low(a.low);
    let a1 = high(a.low);
    let a2 = low(a.high);
    let a3 = high(a.high);

    // Decompose the 32-bit number into two 16-bit words.
    let b0 = low(b);
    let b1 = high(b);

    // Each line represents one row in the diagram above.
    let i0 = a0 * b0;
    let i1 = a1 * b0;
    let i2 = a2 * b0;
    let i3 = a3 * b0;
    let i4 = a0 * b1;
    let i5 = a1 * b1;
    let i6 = a2 * b1;
    let i7 = a3 * b1;

    // Each line represents one column in the diagram above.
    //
    // The high 16 bits of each column are the carry to the next column.
    let r0 = low(i0);
    let r1 = high(i0) + low(i1) + low(i4) + high(r0);
    let r2 = high(i1) + low(i2) + high(i4) + low(i5) + high(r1);
    let r3 = high(i2) + low(i3) + high(i5) + low(i6) + high(r2);
    let r4 = high(i3) + high(i6) + low(i7) + high(r3);
    let r5 = high(i7) + high(r4);
    // The r5 carry will always be zero.

    let out0 = u32_from_u16s(r1, r0);
    let out1 = u32_from_u16s(r3, r2);
    let out2 = u32_from_u16s(r5, r4);

    return Uint96(out0, out1, out2);
}

// Shifts a 96-bit number right by a given number of bits.
//
// The shift is in the range [0, 32].
fn shift_right_96(x: Uint96, shift: u32) -> Uint96 {
    // Shift wraps around at 32, which breaks the algorithm when
    // either shift is 32 or inv_shift is 32.
    if (shift == 0) {
        return x;
    }
    if (shift == 32) {
        return Uint96(x.mid, x.high, 0);
    }

    let inv_shift = 32 - shift;

    let carry2 = x.high << inv_shift;
    let carry1 = x.mid << inv_shift;

    var out: Uint96;

    out.high = x.high >> shift;
    out.mid = (x.mid >> shift) | carry2;
    out.low = (x.low >> shift) | carry1;

    return out;
}
