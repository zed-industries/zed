// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Utilities for fuzz and/or property testing using `arbitrary`.

use core::f64;

use arbitrary::Unstructured;

use crate::{Cubic, Poly, Quadratic};

fn check_finite(f: f64) -> Result<f64, arbitrary::Error> {
    if f.is_finite() {
        Ok(f)
    } else {
        Err(arbitrary::Error::IncorrectFormat)
    }
}

pub fn finite_float(u: &mut Unstructured<'_>) -> Result<f64, arbitrary::Error> {
    check_finite(u.arbitrary()?)
}

/// Generate a float, but give it a chance to be close to another float.
fn another_finite_float(orig: f64, u: &mut Unstructured<'_>) -> Result<f64, arbitrary::Error> {
    let close: bool = u.arbitrary()?;
    if close {
        let ulps: i32 = u.int_in_range(-32..=32)?;
        let scale = 1.0f64 + ulps as f64 * f64::EPSILON;
        check_finite(orig * scale)
    } else {
        finite_float(u)
    }
}

/// An arbitrary float in (-1.0, 1.0).
pub fn float_in_unit_interval(u: &mut Unstructured<'_>) -> Result<f64, arbitrary::Error> {
    let mantissa: u64 = u.arbitrary()?;
    let mantissa = mantissa & ((1u64 << 52) - 1);
    let negative: bool = u.arbitrary()?;
    let sign: u64 = if negative { 1u64 << 63 } else { 0 };

    // 1023 is an exponent of zero, which would lead to numbers of the form 1.something.
    // `% 1023` means we get a maximum exponent of 1022, so our biggest number is 0.11111...
    //
    // `large` here gives us a decent chance of producing a number of magnitude between 0.5 and 1.0.
    // Without it, we only ever generate tiny numbers.
    let large: bool = u.arbitrary()?;
    let exponent: u64 = if large {
        1022 << 52
    } else {
        (u.arbitrary::<u64>()? % 1023u64) << 52
    };

    Ok(f64::from_bits(sign | exponent | mantissa))
}

/// Generate an arbitrary quadratic polynomial.
pub fn quadratic(u: &mut Unstructured<'_>) -> Result<Quadratic, arbitrary::Error> {
    poly(u)
}

/// Generate an arbitrary cubic polynomial.
pub fn cubic(u: &mut Unstructured<'_>) -> Result<Cubic, arbitrary::Error> {
    poly(u)
}

/// Generate an arbitrary polynomial (with finite coefficients).
pub fn poly<const N: usize>(u: &mut Unstructured<'_>) -> Result<Poly<N>, arbitrary::Error> {
    assert!(N >= 2);

    let use_coeffs: bool = u.arbitrary()?;
    if use_coeffs {
        let mut coeffs = [0.0; N];
        coeffs[0] = finite_float(u)?;
        for i in 1..coeffs.len() {
            coeffs[i] = another_finite_float(coeffs[i - 1], u)?;
        }

        Ok(Poly::new(coeffs))
    } else {
        // Generate the roots, with a bias towards roots being almost-repeated.
        let mut r = finite_float(u)?;
        let mut coeffs = [0.0; N];
        coeffs[1] = 1.0;
        coeffs[0] = -r;

        for _ in 0..(N - 2) {
            r = another_finite_float(r, u)?;
            mul(&mut coeffs, r);
        }

        let scale = finite_float(u)?;
        for c in &mut coeffs {
            *c *= scale;
            check_finite(*c)?;
        }

        Ok(Poly::new(coeffs))
    }
}

/// Generate a polynomial with a root at `root` and no other roots within
/// `buffer` of `root`.
pub fn poly_with_planted_root<const N: usize>(
    u: &mut Unstructured<'_>,
    root: f64,
    buffer: f64,
) -> Result<Poly<N>, arbitrary::Error> {
    assert!(N >= 2);

    // Generate the roots, with a bias towards roots being almost-repeated.
    let mut coeffs = [0.0; N];
    coeffs[1] = 1.0;
    coeffs[0] = -root;

    let mut r = finite_float(u)?;
    if (r - root).abs() < buffer {
        return Err(arbitrary::Error::IncorrectFormat);
    }
    mul(&mut coeffs, r);
    for _ in 0..(N - 3) {
        r = another_finite_float(r, u)?;
        if (r - root).abs() < buffer {
            return Err(arbitrary::Error::IncorrectFormat);
        }
        mul(&mut coeffs, r);
    }

    let scale = finite_float(u)?.max(1.0);
    for c in &mut coeffs {
        *c *= scale;
        check_finite(*c)?;
    }

    Ok(Poly::new(coeffs))
}

// Takes the polynomial in `coeffs` and multiplies it by (x - root).
// (Only correct if the last coefficient is zero.)
fn mul<const N: usize>(coeffs: &mut [f64; N], root: f64) {
    for i in (1..coeffs.len()).rev() {
        coeffs[i] = coeffs[i - 1] - root * coeffs[i];
    }
    coeffs[0] *= -root;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_interval() {
        arbtest::arbtest(|u| {
            let x = float_in_unit_interval(u)?;
            assert!(x.abs() < 1.0);
            Ok(())
        });
    }

    #[test]
    fn planted_root() {
        arbtest::arbtest(|u| {
            let r = float_in_unit_interval(u)?;
            let p: Poly<6> = poly_with_planted_root(u, r, 1e-6)?;
            assert!(p.eval(r).abs() <= 1e-12 * p.max_abs_coefficient().max(1.0));
            Ok(())
        });
    }
}
