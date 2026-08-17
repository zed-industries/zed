// Copyright (c) 2019-2022, The rav1e contributors. All rights reserved
//
// This source code is subject to the terms of the BSD 2 Clause License and
// the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
// was not distributed with this source code in the LICENSE file, you can
// obtain it at www.aomedia.org/license/software. If the Alliance for Open
// Media Patent License 1.0 was not distributed with this source code in the
// PATENTS file, you can obtain it at www.aomedia.org/license/patent.

use crate::cpu_features::CpuFeatureLevel;
use crate::frame::PlaneSlice;
use crate::lrf::*;
use crate::util::Pixel;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::mem;

// computes an intermediate (ab) row for stripe_w + 2 columns at row y
#[inline]
pub fn sgrproj_box_ab_r1<const BD: usize>(
  af: &mut [u32], bf: &mut [u32], iimg: &[u32], iimg_sq: &[u32],
  iimg_stride: usize, y: usize, stripe_w: usize, s: u32, cpu: CpuFeatureLevel,
) {
  // only use 8-bit AVX2 assembly when bitdepth equals 8
  if cpu >= CpuFeatureLevel::AVX2 && BD == 8 {
    // SAFETY: Calls Assembly code.
    return unsafe {
      sgrproj_box_ab_r1_avx2::<BD>(
        af,
        bf,
        iimg,
        iimg_sq,
        iimg_stride,
        y,
        stripe_w,
        s,
      );
    };
  }

  rust::sgrproj_box_ab_r1::<BD>(
    af,
    bf,
    iimg,
    iimg_sq,
    iimg_stride,
    y,
    stripe_w,
    s,
    cpu,
  );
}

// computes an intermediate (ab) row for stripe_w + 2 columns at row y
#[inline]
pub fn sgrproj_box_ab_r2<const BD: usize>(
  af: &mut [u32], bf: &mut [u32], iimg: &[u32], iimg_sq: &[u32],
  iimg_stride: usize, y: usize, stripe_w: usize, s: u32, cpu: CpuFeatureLevel,
) {
  // only use 8-bit AVX2 assembly when bitdepth equals 8
  if cpu >= CpuFeatureLevel::AVX2 && BD == 8 {
    // SAFETY: Calls Assembly code.
    return unsafe {
      sgrproj_box_ab_r2_avx2::<BD>(
        af,
        bf,
        iimg,
        iimg_sq,
        iimg_stride,
        y,
        stripe_w,
        s,
      );
    };
  }

  rust::sgrproj_box_ab_r2::<BD>(
    af,
    bf,
    iimg,
    iimg_sq,
    iimg_stride,
    y,
    stripe_w,
    s,
    cpu,
  );
}

#[inline]
pub fn sgrproj_box_f_r0<T: Pixel>(
  f: &mut [u32], y: usize, w: usize, cdeffed: &PlaneSlice<T>,
  cpu: CpuFeatureLevel,
) {
  if cpu >= CpuFeatureLevel::AVX2 {
    // SAFETY: Calls Assembly code.
    return unsafe {
      sgrproj_box_f_r0_avx2(f, y, w, cdeffed);
    };
  }

  rust::sgrproj_box_f_r0(f, y, w, cdeffed, cpu);
}

#[inline]
pub fn sgrproj_box_f_r1<T: Pixel>(
  af: &[&[u32]; 3], bf: &[&[u32]; 3], f: &mut [u32], y: usize, w: usize,
  cdeffed: &PlaneSlice<T>, cpu: CpuFeatureLevel,
) {
  if cpu >= CpuFeatureLevel::AVX2 {
    // SAFETY: Calls Assembly code.
    return unsafe {
      sgrproj_box_f_r1_avx2(af, bf, f, y, w, cdeffed);
    };
  }

  rust::sgrproj_box_f_r1(af, bf, f, y, w, cdeffed, cpu);
}

#[inline]
pub fn sgrproj_box_f_r2<T: Pixel>(
  af: &[&[u32]; 2], bf: &[&[u32]; 2], f0: &mut [u32], f1: &mut [u32],
  y: usize, w: usize, cdeffed: &PlaneSlice<T>, cpu: CpuFeatureLevel,
) {
  if cpu >= CpuFeatureLevel::AVX2 {
    // SAFETY: Calls Assembly code.
    return unsafe {
      sgrproj_box_f_r2_avx2(af, bf, f0, f1, y, w, cdeffed);
    };
  }

  rust::sgrproj_box_f_r2(af, bf, f0, f1, y, w, cdeffed, cpu);
}

static X_BY_XPLUS1: [u32; 256] = [
  // Special case: Map 0 -> 1 (corresponding to a value of 1/256)
  // instead of 0. See comments in selfguided_restoration_internal() for why
  1, 128, 171, 192, 205, 213, 219, 224, 228, 230, 233, 235, 236, 238, 239, 240,
  241, 242, 243, 243, 244, 244, 245, 245, 246, 246, 247, 247, 247, 247, 248,
  248, 248, 248, 249, 249, 249, 249, 249, 250, 250, 250, 250, 250, 250, 250,
  251, 251, 251, 251, 251, 251, 251, 251, 251, 251, 252, 252, 252, 252, 252,
  252, 252, 252, 252, 252, 252, 252, 252, 252, 252, 252, 252, 253, 253, 253,
  253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253,
  253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 253, 254, 254, 254, 254,
  254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254,
  254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254,
  254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254,
  254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254, 254,
  254, 254, 254, 254, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 256,
];

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn sgrproj_box_ab_8_avx2<const BD: usize>(
  r: usize, af: &mut [u32], bf: &mut [u32], iimg: &[u32], iimg_sq: &[u32],
  iimg_stride: usize, x: usize, y: usize, s: u32,
) {
  let bdm8 = BD - 8;
  let d: usize = r * 2 + 1;
  let n: i32 = (d * d) as i32;
  let one_over_n = if r == 1 { 455 } else { 164 };

  // Using an integral image, compute the sum of a square region
  #[inline]
  #[target_feature(enable = "avx2")]
  unsafe fn get_integral_square_avx2(
    iimg: &[u32], stride: usize, x: usize, y: usize, size: usize,
  ) -> __m256i {
    let iimg = iimg.as_ptr().add(y * stride + x);
    // Cancel out overflow in iimg by using wrapping arithmetic
    _mm256_sub_epi32(
      _mm256_add_epi32(
        _mm256_loadu_si256(iimg as *const _),
        _mm256_loadu_si256(iimg.add(size * stride + size) as *const _),
      ),
      _mm256_add_epi32(
        _mm256_loadu_si256(iimg.add(size * stride) as *const _),
        _mm256_loadu_si256(iimg.add(size) as *const _),
      ),
    )
  }

  let sum = get_integral_square_avx2(iimg, iimg_stride, x, y, d);
  let ssq = get_integral_square_avx2(iimg_sq, iimg_stride, x, y, d);
  let scaled_sum = _mm256_srlv_epi32(
    _mm256_add_epi32(sum, _mm256_set1_epi32(1 << bdm8 as i32 >> 1)),
    _mm256_set1_epi32(bdm8 as i32),
  );
  let scaled_ssq = _mm256_srlv_epi32(
    _mm256_add_epi32(ssq, _mm256_set1_epi32(1 << (2 * bdm8) as i32 >> 1)),
    _mm256_set1_epi32(2 * bdm8 as i32),
  );
  let p = _mm256_max_epi32(
    _mm256_setzero_si256(),
    _mm256_sub_epi32(
      _mm256_mullo_epi32(scaled_ssq, _mm256_set1_epi32(n)),
      _mm256_madd_epi16(scaled_sum, scaled_sum),
    ),
  );
  let z = _mm256_srli_epi32(
    _mm256_add_epi32(
      _mm256_mullo_epi32(p, _mm256_set1_epi32(s as i32)),
      _mm256_set1_epi32(1 << SGRPROJ_MTABLE_BITS as i32 >> 1),
    ),
    SGRPROJ_MTABLE_BITS as i32,
  );
  let a = _mm256_i32gather_epi32(
    X_BY_XPLUS1.as_ptr() as *const _,
    _mm256_min_epi32(z, _mm256_set1_epi32(255)),
    4,
  );
  let b = _mm256_mullo_epi32(
    _mm256_madd_epi16(
      _mm256_sub_epi32(_mm256_set1_epi32(1 << SGRPROJ_SGR_BITS as i32), a),
      sum,
    ),
    _mm256_set1_epi32(one_over_n),
  );
  let b = _mm256_srlv_epi32(
    _mm256_add_epi32(
      b,
      _mm256_set1_epi32(1 << SGRPROJ_RECIP_BITS as i32 >> 1),
    ),
    _mm256_set1_epi32(SGRPROJ_RECIP_BITS as i32),
  );
  _mm256_storeu_si256(af.as_mut_ptr().add(x) as *mut _, a);
  _mm256_storeu_si256(bf.as_mut_ptr().add(x) as *mut _, b);
}

#[target_feature(enable = "avx2")]
pub(crate) unsafe fn sgrproj_box_ab_r1_avx2<const BD: usize>(
  af: &mut [u32], bf: &mut [u32], iimg: &[u32], iimg_sq: &[u32],
  iimg_stride: usize, y: usize, stripe_w: usize, s: u32,
) {
  for x in (0..stripe_w + 2).step_by(8) {
    if x + 8 <= stripe_w + 2 {
      sgrproj_box_ab_8_avx2::<BD>(
        1,
        af,
        bf,
        iimg,
        iimg_sq,
        iimg_stride,
        x,
        y,
        s,
      );
    } else {
      // finish using scalar
      rust::sgrproj_box_ab_internal::<BD>(
        1,
        af,
        bf,
        iimg,
        iimg_sq,
        iimg_stride,
        x,
        y,
        stripe_w,
        s,
      );
    }
  }

  #[cfg(feature = "check_asm")]
  {
    let mut af_ref: Vec<u32> = vec![0; stripe_w + 2];
    let mut bf_ref: Vec<u32> = vec![0; stripe_w + 2];
    rust::sgrproj_box_ab_internal::<BD>(
      1,
      &mut af_ref,
      &mut bf_ref,
      iimg,
      iimg_sq,
      iimg_stride,
      0,
      y,
      stripe_w,
      s,
    );
    assert_eq!(&af[..stripe_w + 2], &af_ref[..]);
    assert_eq!(&bf[..stripe_w + 2], &bf_ref[..]);
  }
}

#[target_feature(enable = "avx2")]
pub(crate) unsafe fn sgrproj_box_ab_r2_avx2<const BD: usize>(
  af: &mut [u32], bf: &mut [u32], iimg: &[u32], iimg_sq: &[u32],
  iimg_stride: usize, y: usize, stripe_w: usize, s: u32,
) {
  for x in (0..stripe_w + 2).step_by(8) {
    if x + 8 <= stripe_w + 2 {
      sgrproj_box_ab_8_avx2::<BD>(
        2,
        af,
        bf,
        iimg,
        iimg_sq,
        iimg_stride,
        x,
        y,
        s,
      );
    } else {
      // finish using scalar
      rust::sgrproj_box_ab_internal::<BD>(
        2,
        af,
        bf,
        iimg,
        iimg_sq,
        iimg_stride,
        x,
        y,
        stripe_w,
        s,
      );
    }
  }

  #[cfg(feature = "check_asm")]
  {
    let mut af_ref: Vec<u32> = vec![0; stripe_w + 2];
    let mut bf_ref: Vec<u32> = vec![0; stripe_w + 2];
    rust::sgrproj_box_ab_internal::<BD>(
      2,
      &mut af_ref,
      &mut bf_ref,
      iimg,
      iimg_sq,
      iimg_stride,
      0,
      y,
      stripe_w,
      s,
    );
    assert_eq!(&af[..stripe_w + 2], &af_ref[..]);
    assert_eq!(&bf[..stripe_w + 2], &bf_ref[..]);
  }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn sgrproj_box_f_r0_8_avx2<T: Pixel>(
  f: &mut [u32], x: usize, y: usize, cdeffed: &PlaneSlice<T>,
) {
  _mm256_storeu_si256(
    f.as_mut_ptr().add(x) as *mut _,
    _mm256_slli_epi32(
      if mem::size_of::<T>() == 1 {
        _mm256_cvtepu8_epi32(_mm_loadl_epi64(
          cdeffed.subslice(x, y).as_ptr() as *const _
        ))
      } else {
        _mm256_cvtepu16_epi32(_mm_loadu_si128(
          cdeffed.subslice(x, y).as_ptr() as *const _
        ))
      },
      SGRPROJ_RST_BITS as i32,
    ),
  );
}

#[target_feature(enable = "avx2")]
pub(crate) unsafe fn sgrproj_box_f_r0_avx2<T: Pixel>(
  f: &mut [u32], y: usize, w: usize, cdeffed: &PlaneSlice<T>,
) {
  for x in (0..w).step_by(8) {
    if x + 8 <= w {
      sgrproj_box_f_r0_8_avx2(f, x, y, cdeffed);
    } else {
      // finish using scalar
      rust::sgrproj_box_f_r0_internal(f, x, y, w, cdeffed);
    }
  }

  #[cfg(feature = "check_asm")]
  {
    let mut f_ref: Vec<u32> = vec![0; w];
    rust::sgrproj_box_f_r0_internal(&mut f_ref, 0, y, w, cdeffed);
    assert_eq!(&f[..w], &f_ref[..]);
  }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn sgrproj_box_f_r1_8_avx2<T: Pixel>(
  af: &[&[u32]; 3], bf: &[&[u32]; 3], f: &mut [u32], x: usize, y: usize,
  cdeffed: &PlaneSlice<T>,
) {
  let three = _mm256_set1_epi32(3);
  let four = _mm256_set1_epi32(4);
  let a0 = af[0].as_ptr();
  let a1 = af[1].as_ptr();
  let a2 = af[2].as_ptr();
  let b0 = bf[0].as_ptr();
  let b1 = bf[1].as_ptr();
  let b2 = bf[2].as_ptr();
  let a = _mm256_add_epi32(
    _mm256_madd_epi16(
      _mm256_add_epi32(
        _mm256_add_epi32(
          _mm256_loadu_si256(a0.add(x) as *const _),
          _mm256_loadu_si256(a0.add(x + 2) as *const _),
        ),
        _mm256_add_epi32(
          _mm256_loadu_si256(a2.add(x) as *const _),
          _mm256_loadu_si256(a2.add(x + 2) as *const _),
        ),
      ),
      three,
    ),
    _mm256_madd_epi16(
      _mm256_add_epi32(
        _mm256_add_epi32(
          _mm256_loadu_si256(a1.add(x) as *const _),
          _mm256_loadu_si256(a0.add(x + 1) as *const _),
        ),
        _mm256_add_epi32(
          _mm256_add_epi32(
            _mm256_loadu_si256(a1.add(x + 1) as *const _),
            _mm256_loadu_si256(a2.add(x + 1) as *const _),
          ),
          _mm256_loadu_si256(a1.add(x + 2) as *const _),
        ),
      ),
      four,
    ),
  );
  let b = _mm256_add_epi32(
    _mm256_mullo_epi32(
      _mm256_add_epi32(
        _mm256_add_epi32(
          _mm256_loadu_si256(b0.add(x) as *const _),
          _mm256_loadu_si256(b0.add(x + 2) as *const _),
        ),
        _mm256_add_epi32(
          _mm256_loadu_si256(b2.add(x) as *const _),
          _mm256_loadu_si256(b2.add(x + 2) as *const _),
        ),
      ),
      three,
    ),
    _mm256_mullo_epi32(
      _mm256_add_epi32(
        _mm256_add_epi32(
          _mm256_loadu_si256(b1.add(x) as *const _),
          _mm256_loadu_si256(b0.add(x + 1) as *const _),
        ),
        _mm256_add_epi32(
          _mm256_add_epi32(
            _mm256_loadu_si256(b1.add(x + 1) as *const _),
            _mm256_loadu_si256(b2.add(x + 1) as *const _),
          ),
          _mm256_loadu_si256(b1.add(x + 2) as *const _),
        ),
      ),
      four,
    ),
  );
  let v = _mm256_add_epi32(
    _mm256_madd_epi16(
      a,
      if mem::size_of::<T>() == 1 {
        _mm256_cvtepu8_epi32(_mm_loadl_epi64(
          cdeffed.subslice(x, y).as_ptr() as *const _
        ))
      } else {
        _mm256_cvtepu16_epi32(_mm_loadu_si128(
          cdeffed.subslice(x, y).as_ptr() as *const _
        ))
      },
    ),
    b,
  );
  const SHIFT: i32 = (5 + SGRPROJ_SGR_BITS - SGRPROJ_RST_BITS) as i32;
  _mm256_storeu_si256(
    f.as_mut_ptr().add(x) as *mut _,
    _mm256_srli_epi32(
      _mm256_add_epi32(v, _mm256_set1_epi32(1 << SHIFT >> 1)),
      SHIFT,
    ),
  );
}

#[target_feature(enable = "avx2")]
pub(crate) unsafe fn sgrproj_box_f_r1_avx2<T: Pixel>(
  af: &[&[u32]; 3], bf: &[&[u32]; 3], f: &mut [u32], y: usize, w: usize,
  cdeffed: &PlaneSlice<T>,
) {
  for x in (0..w).step_by(8) {
    if x + 8 <= w {
      sgrproj_box_f_r1_8_avx2(af, bf, f, x, y, cdeffed);
    } else {
      // finish using scalar
      rust::sgrproj_box_f_r1_internal(af, bf, f, x, y, w, cdeffed);
    }
  }

  #[cfg(feature = "check_asm")]
  {
    let mut f_ref: Vec<u32> = vec![0; w];
    rust::sgrproj_box_f_r1_internal(af, bf, &mut f_ref, 0, y, w, cdeffed);
    assert_eq!(&f[..w], &f_ref[..]);
  }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn sgrproj_box_f_r2_8_avx2<T: Pixel>(
  af: &[&[u32]; 2], bf: &[&[u32]; 2], f0: &mut [u32], f1: &mut [u32],
  x: usize, y: usize, cdeffed: &PlaneSlice<T>,
) {
  let five = _mm256_set1_epi32(5);
  let six = _mm256_set1_epi32(6);
  let a0 = af[0].as_ptr();
  let a2 = af[1].as_ptr();
  let b0 = bf[0].as_ptr();
  let b2 = bf[1].as_ptr();
  let a = _mm256_add_epi32(
    _mm256_madd_epi16(
      _mm256_add_epi32(
        _mm256_loadu_si256(a0.add(x) as *const _),
        _mm256_loadu_si256(a0.add(x + 2) as *const _),
      ),
      five,
    ),
    _mm256_madd_epi16(_mm256_loadu_si256(a0.add(x + 1) as *const _), six),
  );
  let b = _mm256_add_epi32(
    _mm256_mullo_epi32(
      _mm256_add_epi32(
        _mm256_loadu_si256(b0.add(x) as *const _),
        _mm256_loadu_si256(b0.add(x + 2) as *const _),
      ),
      five,
    ),
    _mm256_mullo_epi32(_mm256_loadu_si256(b0.add(x + 1) as *const _), six),
  );
  let ao = _mm256_add_epi32(
    _mm256_madd_epi16(
      _mm256_add_epi32(
        _mm256_loadu_si256(a2.add(x) as *const _),
        _mm256_loadu_si256(a2.add(x + 2) as *const _),
      ),
      five,
    ),
    _mm256_madd_epi16(_mm256_loadu_si256(a2.add(x + 1) as *const _), six),
  );
  let bo = _mm256_add_epi32(
    _mm256_mullo_epi32(
      _mm256_add_epi32(
        _mm256_loadu_si256(b2.add(x) as *const _),
        _mm256_loadu_si256(b2.add(x + 2) as *const _),
      ),
      five,
    ),
    _mm256_mullo_epi32(_mm256_loadu_si256(b2.add(x + 1) as *const _), six),
  );
  let v = _mm256_add_epi32(
    _mm256_madd_epi16(
      _mm256_add_epi32(a, ao),
      if mem::size_of::<T>() == 1 {
        _mm256_cvtepu8_epi32(_mm_loadl_epi64(
          cdeffed.subslice(x, y).as_ptr() as *const _
        ))
      } else {
        _mm256_cvtepu16_epi32(_mm_loadu_si128(
          cdeffed.subslice(x, y).as_ptr() as *const _
        ))
      },
    ),
    _mm256_add_epi32(b, bo),
  );
  let vo = _mm256_add_epi32(
    _mm256_madd_epi16(
      ao,
      if mem::size_of::<T>() == 1 {
        _mm256_cvtepu8_epi32(_mm_loadl_epi64(
          cdeffed.subslice(x, y + 1).as_ptr() as *const _,
        ))
      } else {
        _mm256_cvtepu16_epi32(_mm_loadu_si128(
          cdeffed.subslice(x, y + 1).as_ptr() as *const _,
        ))
      },
    ),
    bo,
  );
  const SHIFT: i32 = (5 + SGRPROJ_SGR_BITS - SGRPROJ_RST_BITS) as i32;
  _mm256_storeu_si256(
    f0.as_mut_ptr().add(x) as *mut _,
    _mm256_srli_epi32(
      _mm256_add_epi32(v, _mm256_set1_epi32(1 << SHIFT >> 1)),
      SHIFT,
    ),
  );
  const SHIFTO: i32 = (4 + SGRPROJ_SGR_BITS - SGRPROJ_RST_BITS) as i32;
  _mm256_storeu_si256(
    f1.as_mut_ptr().add(x) as *mut _,
    _mm256_srli_epi32(
      _mm256_add_epi32(vo, _mm256_set1_epi32(1 << SHIFTO >> 1)),
      SHIFTO,
    ),
  );
}

#[target_feature(enable = "avx2")]
pub(crate) unsafe fn sgrproj_box_f_r2_avx2<T: Pixel>(
  af: &[&[u32]; 2], bf: &[&[u32]; 2], f0: &mut [u32], f1: &mut [u32],
  y: usize, w: usize, cdeffed: &PlaneSlice<T>,
) {
  for x in (0..w).step_by(8) {
    if x + 8 <= w {
      sgrproj_box_f_r2_8_avx2(af, bf, f0, f1, x, y, cdeffed);
    } else {
      // finish using scalar
      rust::sgrproj_box_f_r2_internal(af, bf, f0, f1, x, y, w, cdeffed);
    }
  }

  #[cfg(feature = "check_asm")]
  {
    let mut f0_ref: Vec<u32> = vec![0; w];
    let mut f1_ref: Vec<u32> = vec![0; w];
    rust::sgrproj_box_f_r2_internal(
      af,
      bf,
      &mut f0_ref,
      &mut f1_ref,
      0,
      y,
      w,
      cdeffed,
    );
    assert_eq!(&f0[..w], &f0_ref[..]);
    assert_eq!(&f1[..w], &f1_ref[..]);
  }
}
