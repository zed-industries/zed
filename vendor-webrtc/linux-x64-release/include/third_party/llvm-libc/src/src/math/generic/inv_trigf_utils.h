//===-- Single-precision general inverse trigonometric functions ----------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_MATH_GENERIC_INV_TRIGF_UTILS_H
#define LLVM_LIBC_SRC_MATH_GENERIC_INV_TRIGF_UTILS_H

#include "src/__support/FPUtil/PolyEval.h"
#include "src/__support/FPUtil/multiply_add.h"
#include "src/__support/common.h"
#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

// PI and PI / 2
static constexpr double M_MATH_PI = 0x1.921fb54442d18p+1;
static constexpr double M_MATH_PI_2 = 0x1.921fb54442d18p+0;

extern double ATAN_COEFFS[17][9];

// Look-up table for atan(k/16) with k = 0..16.
static constexpr double ATAN_K_OVER_16[17] = {
    0.0,
    0x1.ff55bb72cfdeap-5,
    0x1.fd5ba9aac2f6ep-4,
    0x1.7b97b4bce5b02p-3,
    0x1.f5b75f92c80ddp-3,
    0x1.362773707ebccp-2,
    0x1.6f61941e4def1p-2,
    0x1.a64eec3cc23fdp-2,
    0x1.dac670561bb4fp-2,
    0x1.0657e94db30dp-1,
    0x1.1e00babdefeb4p-1,
    0x1.345f01cce37bbp-1,
    0x1.4978fa3269ee1p-1,
    0x1.5d58987169b18p-1,
    0x1.700a7c5784634p-1,
    0x1.819d0b7158a4dp-1,
    0x1.921fb54442d18p-1,
};

// For |x| <= 1/32 and 0 <= i <= 16, return Q(x) such that:
//   Q(x) ~ (atan(x + i/16) - atan(i/16)) / x.
LIBC_INLINE static double atan_eval(double x, unsigned i) {
  double x2 = x * x;

  double c0 = fputil::multiply_add(x, ATAN_COEFFS[i][2], ATAN_COEFFS[i][1]);
  double c1 = fputil::multiply_add(x, ATAN_COEFFS[i][4], ATAN_COEFFS[i][3]);
  double c2 = fputil::multiply_add(x, ATAN_COEFFS[i][6], ATAN_COEFFS[i][5]);
  double c3 = fputil::multiply_add(x, ATAN_COEFFS[i][8], ATAN_COEFFS[i][7]);

  double x4 = x2 * x2;
  double d1 = fputil::multiply_add(x2, c1, c0);
  double d2 = fputil::multiply_add(x2, c3, c2);
  double p = fputil::multiply_add(x4, d2, d1);
  return p;
}

// Evaluate atan without big lookup table.
//   atan(n/d) - atan(k/16) = atan((n/d - k/16) / (1 + (n/d) * (k/16)))
//                          = atan((n - d * k/16)) / (d + n * k/16))
// So we let q = (n - d * k/16) / (d + n * k/16),
// and approximate with Taylor polynomial:
//   atan(q) ~ q - q^3/3 + q^5/5 - q^7/7 + q^9/9
LIBC_INLINE static double atan_eval_no_table(double num, double den,
                                             double k_over_16) {
  double num_r = fputil::multiply_add(den, -k_over_16, num);
  double den_r = fputil::multiply_add(num, k_over_16, den);
  double q = num_r / den_r;

  constexpr double ATAN_TAYLOR[] = {
      -0x1.5555555555555p-2,
      0x1.999999999999ap-3,
      -0x1.2492492492492p-3,
      0x1.c71c71c71c71cp-4,
  };
  double q2 = q * q;
  double q3 = q2 * q;
  double q4 = q2 * q2;
  double c0 = fputil::multiply_add(q2, ATAN_TAYLOR[1], ATAN_TAYLOR[0]);
  double c1 = fputil::multiply_add(q2, ATAN_TAYLOR[3], ATAN_TAYLOR[2]);
  double d = fputil::multiply_add(q4, c1, c0);
  return fputil::multiply_add(q3, d, q);
}

// > Q = fpminimax(asin(x)/x, [|0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20|],
//                 [|1, D...|], [0, 0.5]);
static constexpr double ASIN_COEFFS[10] = {
    0x1.5555555540fa1p-3, 0x1.333333512edc2p-4, 0x1.6db6cc1541b31p-5,
    0x1.f1caff324770ep-6, 0x1.6e43899f5f4f4p-6, 0x1.1f847cf652577p-6,
    0x1.9b60f47f87146p-7, 0x1.259e2634c494fp-6, -0x1.df946fa875ddp-8,
    0x1.02311ecf99c28p-5};

// Evaluate P(x^2) - 1, where P(x^2) ~ asin(x)/x
LIBC_INLINE static double asin_eval(double xsq) {
  double x4 = xsq * xsq;
  double r1 = fputil::polyeval(x4, ASIN_COEFFS[0], ASIN_COEFFS[2],
                               ASIN_COEFFS[4], ASIN_COEFFS[6], ASIN_COEFFS[8]);
  double r2 = fputil::polyeval(x4, ASIN_COEFFS[1], ASIN_COEFFS[3],
                               ASIN_COEFFS[5], ASIN_COEFFS[7], ASIN_COEFFS[9]);
  return fputil::multiply_add(xsq, r2, r1);
}

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_MATH_GENERIC_INV_TRIGF_UTILS_H
