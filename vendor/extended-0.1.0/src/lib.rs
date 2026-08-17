//! Extended-precision 80-bit floating-point numbers (f80).

#[warn(missing_docs)]

use std::convert::From;

/// An 80-bit extended floating-point number.
/// 
/// See Apple Numerics Manual, 2nd edition (1988), p. 18 "SANE Data Types".
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Extended {
    // The sign is stored as the high bit. The low 15 bits contain the exponent,
	// with a bias of 16383.
    pub sign_exponent: u16,

    // The fraction includes a ones place as the high bit. The value in the ones
	// place may be zero.
    pub fraction: u64,
}

const MAX_EXPONENT_64: u32 = (1 << 11) - 1;

impl Extended {
    /// Create an extended 80-bit floating-point number from its big endian
    /// representation.
    pub fn from_be_bytes(b: [u8; 10]) -> Self {
        Extended {
            sign_exponent: u16::from_be_bytes(b[0..2].try_into().unwrap()),
            fraction: u64::from_be_bytes(b[2..10].try_into().unwrap()),
        }
    }

    /// Create an extended 80-bit floating-point number from its little endian
    /// representation.
    pub fn from_le_bytes(b: [u8; 10]) -> Self {
        Extended {
            sign_exponent: u16::from_le_bytes(b[8..10].try_into().unwrap()),
            fraction: u64::from_le_bytes(b[0..8].try_into().unwrap()),
        }
    }

    /// Convert an 80-bit floating-point number to its big endian
    /// representation.
    pub fn to_be_bytes(&self) -> [u8; 10] {
        let mut b = [0u8; 10];
        b[0..2].copy_from_slice(&self.sign_exponent.to_be_bytes());
        b[2..10].copy_from_slice(&self.fraction.to_be_bytes());
        b
    }

    /// Convert an 80-bit floating-point number to its big endian
    /// representation.
    pub fn to_le_bytes(&self) -> [u8; 10] {
        let mut b = [0u8; 10];
        b[8..10].copy_from_slice(&self.sign_exponent.to_le_bytes());
        b[0..8].copy_from_slice(&self.fraction.to_le_bytes());
        b
    }

    /// Convert to a 64-bit floating-point number. Values which are out of range
    /// are flushed to infinity or zero.
    pub fn to_f64(&self) -> f64 {
        const INFINITY: u64 = (MAX_EXPONENT_64 as u64) << 52;
        const NAN: u64 = u64::MAX >> 1;
        let exponent = i32::from(self.sign_exponent) & 0x7fff;
        let bits = if exponent == 0x7fff {
            if self.fraction == 0 {
                INFINITY
            } else {
                NAN
            }
        } else if self.fraction == 0 {
            0
        } else {
            // 2^(e64 - 1023) * 1.fraction
            // = 2^(e80 - 16383) * 1.fraction / 2^nzero
            // e63 - 1023 = e80 - 16383
            // e63 = e80 - 16383 + 1023 - nzero
            let nzero = self.fraction.leading_zeros();
            let exponent = exponent - 16383 + 1023 - (nzero as i32);
            let fraction = self.fraction << nzero;
            // Fraction is of the form 1.xxxxx.
            if exponent <= 0 {
                // Subnormal numbers.
                let shift = 12 - exponent;
                let (fraction, rem) = if shift > 64 {
                    (0, 0)
                } else if shift == 64 {
                    (0, fraction)
                } else {
                    (fraction >> shift, fraction << (64 - shift))
                };
                // The (fraction & 1) makes this round to even.
                if (rem | (fraction & 1)) <= (1 << 63) {
                    fraction
                } else {
                    fraction + 1
                }
            } else {
                // Round it to 52 bits. The addition of ((fraction >> 11) & 1)
                // makes this round to even.
                let rem = (fraction & ((1 << 11) - 1)) | ((fraction >> 11) & 1);
                let fraction = (fraction >> 11) & ((1 << 52) - 1);
                let (exponent, fraction) = if rem <= (1 << 10) {
                    (exponent, fraction)
                } else if fraction < (1 << 52) - 1 {
                    (exponent, fraction + 1)
                } else {
                    (exponent + 1, 0)
                };
                if exponent >= (MAX_EXPONENT_64 as i32) {
                    // Out of range.
                    INFINITY
                } else {
                    fraction | ((exponent as u64) << 52)
                }
            }
        };
        let sign = (u64::from(self.sign_exponent) & 0x8000) << 48;
        f64::from_bits(bits | sign)
    }
}

impl From<f64> for Extended {
    fn from(x: f64) -> Self {
        let bits = x.to_bits();
        let sign = ((bits >> (63 - 15)) as u32) & 0x8000;
        let exponent = ((bits >> 52) as u32) & MAX_EXPONENT_64;
        let mantissa = bits & ((1 << 52) - 1);
        if exponent == 0 {
            // Zero or subnormal.
            // Number is (-1)^sign * 2^-1022 * 0.mantissa.
            if mantissa == 0 {
                Extended {
                    sign_exponent: sign as u16,
                    fraction: 0,
                }
            } else {
                // 2^-1022 * 0.mantissa = 2^(e-16383) * 2^lzero * 0.mantissa
                // -1022 = e - 16383 + lzero
                // e = -1022 + 16383 - lzero
                let nzero = mantissa.leading_zeros();
                let exponent = 16383 - 1022 + 11 - nzero;
                Extended {
                    sign_exponent: (sign | exponent) as u16,
                    fraction: mantissa << nzero,
                }
            }
        } else if exponent == MAX_EXPONENT_64 {
            // Infinity or NaN.
            Extended {
                sign_exponent: (sign | 0x7fff) as u16,
                fraction: if mantissa == 0 { 0 } else { u64::MAX },
            }
        } else {
            // 2^(e64 - 1023) * 1.fraction = 2^(e80 - 16383) * 1.fraction
            // e63 - 1023 = e80 - 16383
            // e80 = e63 + 16383 - 1023
            let exponent = exponent + 16383 - 1023;
            Extended {
                sign_exponent: (sign | exponent) as u16,
                fraction: (1 << 63) | (mantissa << 11),
            }
        }
    }
}

impl From<f32> for Extended {
    fn from(x: f32) -> Self {
        f64::from(x).into()
    }
}

impl From<i32> for Extended {
    fn from(x: i32) -> Self {
        f64::from(x).into()
    }
}

impl From<u32> for Extended {
    fn from(x: u32) -> Self {
        f64::from(x).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn equal_f64(x: f64, y: f64) -> bool {
        if x.is_nan() {
            y.is_nan()
        } else {
            x == y
        }
    }

    #[test]
    fn test_to_f64() {
        const CASES: &[(u16, u64, f64)] = &[
            // Easy.
            (16383, 1 << 63, 1.0),
            (16384, 1 << 63, 2.0),
            (16382, 1 << 63, 0.5),
            // Next after 1.0.
            (16383, (1 << 63) + (1 << 11), 1.0000000000000002),
            // Rounds to even.
            (16383, (1 << 63) + (1 << 10), 1.0),
            (16383, (1 << 63) + (1 << 10) + 1, 1.0000000000000002),
            (16383, (1 << 63) + (1 << 11), 1.0000000000000002),
            (16383, (1 << 63) + (3 << 10) - 1, 1.0000000000000002),
            (16383, (1 << 63) + (3 << 10), 1.0000000000000004),
            // Rounds to next exponent.
            (16381, u64::MAX, 0.5),
            // Is infinity.
            (32767, 0, f64::INFINITY),
            // Out of range.
            (32000, 1 << 63, f64::INFINITY),
            (32000, u64::MAX, f64::INFINITY),
            (17406, 0xfffffffffffff800, 1.7976931348623157e+308),
            (17406, 0xfffffffffffffbff, 1.7976931348623157e+308),
            (17406, 0xfffffffffffffc00, f64::INFINITY),
            // Zero.
            (0, 0, 0.0),
            // NaN.
            (32767, 1, f64::NAN),
            (32767, 1 << 63, f64::NAN),
            // Smallest normal.
            (15361, 1 << 63, 2.2250738585072014e-308),
            // Subnormal.
            (15360, 1 << 63, 1.1125369292536007e-308),
            // Smallest subnormal.
            (15309, 1 << 63, 5e-324),
            // Rounds up to smallest subnormal.
            (15308, (1 << 63) + 1, 5e-324),
            (15308, 1 << 63, 0.0),
            // Very small.
            (10000, 1 << 63, 0.0),
        ];
        let mut failed = false;
        for (n, &(exponent, fraction, expect)) in CASES.iter().enumerate() {
            for sign in 0..2 {
                let exponent = exponent | ((sign as u16) << 15);
                let fin = Extended { sign_exponent: exponent, fraction };
                let fout = fin.to_f64();
                let expect = if sign == 0 { expect } else { -expect };
                if !equal_f64(fout, expect) {
                    failed = true;
                    eprintln!(
                        "Case {}: Input = {:04x}:{:016x}, Output = {:?}, Expected = {:?}",
                        n, exponent, fraction, fout, expect
                    );
                }
            }
        }
        if failed {
            panic!("test failed");
        }
    }

    #[test]
    fn test_from_f64() {
        const CASES: &[(u16, u64, f64)] = &[
            // Easy.
            (16383, 1 << 63, 1.0),
            (16384, 1 << 63, 2.0),
            (16382, 1 << 63, 0.5),
            (16383 - 10, 1 << 63, 0.0009765625),
            (16383 - 100, 1 << 63, 7.888609052210118e-31),
            // Next after 1.0.
            (16383, (1 << 63) + (1 << 11), 1.0000000000000002),
            // Is infinity.
            (32767, 0, f64::INFINITY),
            // Zero.
            (0, 0, 0.0),
            // NaN.
            (32767, u64::MAX, f64::NAN),
            // Smallest normal.
            (15361, 1 << 63, 2.2250738585072014e-308),
            // Subnormal.
            (15360, 1 << 63, 1.1125369292536007e-308),
            // // Smallest subnormal.
            (15309, 1 << 63, 5e-324),
        ];
        let mut failed = false;
        for (n, &(exponent, fraction, fin)) in CASES.iter().enumerate() {
            for sign in 0..2 {
                let exponent = exponent | ((sign as u16) << 15);
                let fin = if sign == 0 { fin } else { -fin };
                let fout = Extended::from(fin);
                let expect = Extended { sign_exponent: exponent, fraction };
                if fout != expect {
                    failed = true;
                    eprintln!(
                        "Case {}: Input = {:?}, Output = {:04x}:{:016x}, Expected = {:04x}:{:016x}",
                        n, fin, fout.sign_exponent, fout.fraction, expect.sign_exponent, expect.fraction
                    );
                    continue;
                }
                // Round-trip sanity check.
                let rev = fout.to_f64();
                if !equal_f64(fin, rev) {
                    failed = true;
                    eprintln!(
                        "Case {}: Round trip faied: {:?} -> {:04x}:{:016x} -> {:?}",
                        n, fin, fout.sign_exponent, fout.fraction, rev
                    );
                }
            }
        }
        if failed {
            panic!("test failed");
        }
    }
}
