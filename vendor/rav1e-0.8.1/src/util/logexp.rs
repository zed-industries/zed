// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

/// Convert an integer into a Q57 fixed-point fraction.
pub const fn q57(v: i32) -> i64 {
  debug_assert!(v >= -64 && v <= 63);
  (v as i64) << 57
}

#[rustfmt::skip]
const ATANH_LOG2: &[i64; 32] = &[
  0x32B803473F7AD0F4, 0x2F2A71BD4E25E916, 0x2E68B244BB93BA06,
  0x2E39FB9198CE62E4, 0x2E2E683F68565C8F, 0x2E2B850BE2077FC1,
  0x2E2ACC58FE7B78DB, 0x2E2A9E2DE52FD5F2, 0x2E2A92A338D53EEC,
  0x2E2A8FC08F5E19B6, 0x2E2A8F07E51A485E, 0x2E2A8ED9BA8AF388,
  0x2E2A8ECE2FE7384A, 0x2E2A8ECB4D3E4B1A, 0x2E2A8ECA94940FE8,
  0x2E2A8ECA6669811D, 0x2E2A8ECA5ADEDD6A, 0x2E2A8ECA57FC347E,
  0x2E2A8ECA57438A43, 0x2E2A8ECA57155FB4, 0x2E2A8ECA5709D510,
  0x2E2A8ECA5706F267, 0x2E2A8ECA570639BD, 0x2E2A8ECA57060B92,
  0x2E2A8ECA57060008, 0x2E2A8ECA5705FD25, 0x2E2A8ECA5705FC6C,
  0x2E2A8ECA5705FC3E, 0x2E2A8ECA5705FC33, 0x2E2A8ECA5705FC30,
  0x2E2A8ECA5705FC2F, 0x2E2A8ECA5705FC2F
];

/// Computes the binary exponential of `logq57`.
/// `logq57`: a log base 2 in Q57 format.
/// Returns a 64 bit integer in Q0 (no fraction).
pub const fn bexp64(logq57: i64) -> i64 {
  let ipart = (logq57 >> 57) as i32;
  if ipart < 0 {
    return 0;
  }
  if ipart >= 63 {
    return 0x7FFFFFFFFFFFFFFF;
  }
  // z is the fractional part of the log in Q62 format.
  // We need 1 bit of headroom since the magnitude can get larger than 1
  //  during the iteration, and a sign bit.
  let mut z = logq57 - q57(ipart);
  let mut w: i64;
  if z != 0 {
    // Rust has 128 bit multiplies, so it should be possible to do this
    //  faster without losing accuracy.
    z <<= 5;
    // w is the exponential in Q61 format (since it also needs headroom and can
    //  get as large as 2.0); we could get another bit if we dropped the sign,
    //  but we'll recover that bit later anyway.
    // Ideally this should start out as
    //   \lim_{n->\infty} 2^{61}/\product_{i=1}^n \sqrt{1-2^{-2i}}
    //  but in order to guarantee convergence we have to repeat iterations 4,
    //  13 (=3*4+1), and 40 (=3*13+1, etc.), so it winds up somewhat larger.
    w = 0x26A3D0E401DD846D;
    let mut i: i64 = 0;
    loop {
      let mask = -((z < 0) as i64);
      w += ((w >> (i + 1)) + mask) ^ mask;
      z -= (ATANH_LOG2[i as usize] + mask) ^ mask;
      // Repeat iteration 4.
      if i >= 3 {
        break;
      }
      z *= 2;
      i += 1;
    }
    loop {
      let mask = -((z < 0) as i64);
      w += ((w >> (i + 1)) + mask) ^ mask;
      z -= (ATANH_LOG2[i as usize] + mask) ^ mask;
      // Repeat iteration 13.
      if i >= 12 {
        break;
      }
      z *= 2;
      i += 1;
    }
    while i < 32 {
      let mask = -((z < 0) as i64);
      w += ((w >> (i + 1)) + mask) ^ mask;
      z = (z - ((ATANH_LOG2[i as usize] + mask) ^ mask)) * 2;
      i += 1;
    }
    // Skip the remaining iterations unless we really require that much
    //  precision.
    // We could have bailed out earlier for smaller iparts, but that would
    //  require initializing w from a table, as the limit doesn't converge to
    //  61-bit precision until n=30.
    let mut wlo: i32 = 0;
    if ipart > 30 {
      // For these iterations, we just update the low bits, as the high bits
      //  can't possibly be affected.
      // OD_ATANH_LOG2 has also converged (it actually did so one iteration
      //  earlier, but that's no reason for an extra special case).
      loop {
        let mask = -((z < 0) as i64);
        wlo += (((w >> i) + mask) ^ mask) as i32;
        z -= (ATANH_LOG2[31] + mask) ^ mask;
        // Repeat iteration 40.
        if i >= 39 {
          break;
        }
        z *= 2;
        i += 1;
      }
      while i < 61 {
        let mask = -((z < 0) as i64);
        wlo += (((w >> i) + mask) ^ mask) as i32;
        z = (z - ((ATANH_LOG2[31] + mask) ^ mask)) * 2;
        i += 1;
      }
    }
    w = (w << 1) + (wlo as i64);
  } else {
    w = 1i64 << 62;
  }
  if ipart < 62 {
    w = ((w >> (61 - ipart)) + 1) >> 1;
  }
  w
}

/// Computes the binary log of `n`.
/// `n`: a 64-bit integer in Q0 (no fraction).
/// Returns a 64-bit log in Q57.
pub const fn blog64(n: i64) -> i64 {
  if n <= 0 {
    return -1;
  }
  let ipart = 63 - n.leading_zeros() as i32;
  let w = if ipart > 61 { n >> (ipart - 61) } else { n << (61 - ipart) };
  if (w & (w - 1)) == 0 {
    return q57(ipart);
  }
  // z is the fractional part of the log in Q61 format.
  let mut z: i64 = 0;
  // Rust has 128 bit multiplies, so it should be possible to do this
  //  faster without losing accuracy.
  // x and y are the cosh() and sinh(), respectively, in Q61 format.
  // We are computing z = 2*atanh(y/x) = 2*atanh((w - 1)/(w + 1)).
  let mut x = w + (1i64 << 61);
  let mut y = w - (1i64 << 61);
  // Repeat iteration 4.
  // Repeat iteration 13.
  // Repeat iteration 40.
  let bounds = [3, 12, 39, 61];
  let mut i = 0;
  let mut j = 0;
  loop {
    let end = bounds[j];
    loop {
      let mask = -((y < 0) as i64);
      // ATANH_LOG2 has converged at iteration 32.
      z += ((ATANH_LOG2[if i < 31 { i } else { 31 }] >> i) + mask) ^ mask;
      let u = x >> (i + 1);
      x -= ((y >> (i + 1)) + mask) ^ mask;
      y -= (u + mask) ^ mask;
      if i == end {
        break;
      }
      i += 1;
    }
    j += 1;
    if j == bounds.len() {
      break;
    }
  }
  z = (z + 8) >> 4;
  q57(ipart) + z
}

/// Computes the binary log of `n`.
/// `n`: an unsigned 32-bit integer in Q0 (no fraction).
/// Returns a signed 32-bit log in Q24.
#[allow(unused)]
pub const fn blog32(n: u32) -> i32 {
  if n == 0 {
    return -1;
  }
  let ipart = 31 - n.leading_zeros() as i32;
  let n = n as i64;
  let w = if ipart > 61 { n >> (ipart - 61) } else { n << (61 - ipart) };
  if (w & (w - 1)) == 0 {
    return ipart << 24;
  }
  // z is the fractional part of the log in Q61 format.
  let mut z: i64 = 0;
  // Rust has 128 bit multiplies, so it should be possible to do this
  //  faster without losing accuracy.
  // x and y are the cosh() and sinh(), respectively, in Q61 format.
  // We are computing z = 2*atanh(y/x) = 2*atanh((w - 1)/(w + 1)).
  let mut x = w + (1i64 << 61);
  let mut y = w - (1i64 << 61);
  // Repeat iteration 4.
  // Repeat iteration 13.
  let bounds = [3, 12, 29];
  let mut i = 0;
  let mut j = 0;
  loop {
    let end = bounds[j];
    loop {
      let mask = -((y < 0) as i64);
      z += ((ATANH_LOG2[i] >> i) + mask) ^ mask;
      let u = x >> (i + 1);
      x -= ((y >> (i + 1)) + mask) ^ mask;
      y -= (u + mask) ^ mask;
      if i == end {
        break;
      }
      i += 1;
    }
    j += 1;
    if j == bounds.len() {
      break;
    }
  }
  const SHIFT: usize = 61 - 24;
  z = (z + (1 << SHIFT >> 1)) >> SHIFT;
  (ipart << 24) + z as i32
}

/// Converts a Q57 fixed-point fraction to Q24 by rounding.
pub const fn q57_to_q24(v: i64) -> i32 {
  (((v >> 32) + 1) >> 1) as i32
}

/// Converts a Q24 fixed-point fraction to Q57.
pub const fn q24_to_q57(v: i32) -> i64 {
  (v as i64) << 33
}

/// Binary exponentiation of a `log_scale` with 24-bit fractional precision and
///  saturation.
/// `log_scale`: A binary logarithm in Q24 format.
/// Returns the binary exponential in Q24 format, saturated to 2**47 - 1 if
///  `log_scale` was too large.
pub const fn bexp_q24(log_scale: i32) -> i64 {
  if log_scale < 23 << 24 {
    let ret = bexp64(((log_scale as i64) << 33) + q57(24));
    if ret < (1i64 << 47) - 1 {
      return ret;
    }
  }
  (1i64 << 47) - 1
}

/// Polynomial approximation of a binary exponential.
/// Q10 input, Q0 output.
#[allow(unused)]
pub const fn bexp32_q10(z: i32) -> u32 {
  let ipart = z >> 10;
  let mut n = ((z & ((1 << 10) - 1)) << 4) as u32;
  n = ({
    n * (((n * (((n * (((n * 3548) >> 15) + 6817)) >> 15) + 15823)) >> 15)
      + 22708)
  } >> 15)
    + 16384;
  if 14 - ipart > 0 {
    (n + (1 << (13 - ipart))) >> (14 - ipart)
  } else {
    n << (ipart - 14)
  }
}

/// Polynomial approximation of a binary logarithm.
/// Q0 input, Q11 output.
pub const fn blog32_q11(w: u32) -> i32 {
  if w == 0 {
    return -1;
  }
  let ipart = 32 - w.leading_zeros() as i32;
  let n = if ipart - 16 > 0 { w >> (ipart - 16) } else { w << (16 - ipart) }
    as i32
    - 32768
    - 16384;
  let fpart = ({
    n * (((n * (((n * (((n * -1402) >> 15) + 2546)) >> 15) - 5216)) >> 15)
      + 15745)
  } >> 15)
    - 6797;
  (ipart << 11) + (fpart >> 3)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn blog64_vectors() {
    assert!(blog64(1793) == 0x159dc71e24d32daf);
    assert!(blog64(0x678dde6e5fd29f05) == 0x7d6373ad151ca685);
  }

  #[test]
  fn bexp64_vectors() {
    assert!(bexp64(0x159dc71e24d32daf) == 1793);
    assert!((bexp64(0x7d6373ad151ca685) - 0x678dde6e5fd29f05).abs() < 29);
  }

  #[test]
  fn blog64_bexp64_round_trip() {
    for a in 1..=std::u16::MAX as i64 {
      let b = std::i64::MAX / a;
      let (log_a, log_b, log_ab) = (blog64(a), blog64(b), blog64(a * b));
      assert!((log_a + log_b - log_ab).abs() < 4);
      assert!(bexp64(log_a) == a);
      assert!((bexp64(log_b) - b).abs() < 128);
      assert!((bexp64(log_ab) - a * b).abs() < 128);
    }
  }

  #[test]
  fn blog32_vectors() {
    assert_eq!(blog32(0), -1);
    assert_eq!(blog32(1793), q57_to_q24(0x159dc71e24d32daf));
  }

  #[test]
  fn bexp_q24_vectors() {
    assert_eq!(bexp_q24(i32::MAX), (1i64 << 47) - 1);
    assert_eq!(
      (bexp_q24(q57_to_q24(0x159dc71e24d32daf)) + (1 << 24 >> 1)) >> 24,
      1793
    );
  }

  #[test]
  fn blog32_bexp_q24_round_trip() {
    for a in 1..=std::u16::MAX as u32 {
      let b = (std::u32::MAX >> 9) / a;
      let (log_a, log_b, log_ab) = (blog32(a), blog32(b), blog32(a * b));
      assert!((log_a + log_b - log_ab).abs() < 4);
      assert!((bexp_q24(log_a) - (i64::from(a) << 24)).abs() < (1 << 24 >> 1));
      assert!(((bexp_q24(log_b) >> 24) - i64::from(b)).abs() < 128);
      assert!(
        ((bexp_q24(log_ab) >> 24) - i64::from(a) * i64::from(b)).abs() < 128
      );
    }
  }

  #[test]
  fn blog32_q11_bexp32_q10_round_trip() {
    for a in 1..=std::i16::MAX as i32 {
      let b = std::i16::MAX as i32 / a;
      let (log_a, log_b, log_ab) = (
        blog32_q11(a as u32),
        blog32_q11(b as u32),
        blog32_q11(a as u32 * b as u32),
      );
      assert!((log_a + log_b - log_ab).abs() < 4);
      assert!((bexp32_q10((log_a + 1) >> 1) as i32 - a).abs() < 18);
      assert!((bexp32_q10((log_b + 1) >> 1) as i32 - b).abs() < 2);
      assert!((bexp32_q10((log_ab + 1) >> 1) as i32 - a * b).abs() < 18);
    }
  }
}
