//===-- NVPTX specific declarations for math support ----------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_MATH_NVPTX_DECLARATIONS_H
#define LLVM_LIBC_SRC_MATH_NVPTX_DECLARATIONS_H

#include "src/__support/macros/config.h"

namespace LIBC_NAMESPACE_DECL {

extern "C" {
double __nv_acos(double);
float __nv_acosf(float);
double __nv_acosh(double);
float __nv_acoshf(float);
double __nv_asin(double);
float __nv_asinf(float);
double __nv_asinh(double);
float __nv_asinhf(float);
double __nv_atan(double);
float __nv_atanf(float);
double __nv_atan2(double, double);
float __nv_atan2f(float, float);
double __nv_atanh(double);
float __nv_atanhf(float);
double __nv_cos(double);
float __nv_cosf(float);
double __nv_cosh(double);
float __nv_coshf(float);
double __nv_erf(double);
float __nv_erff(float);
double __nv_exp(double);
float __nv_expf(float);
double __nv_exp2(double);
float __nv_exp2f(float);
double __nv_exp10(double);
float __nv_exp10f(float);
double __nv_expm1(double);
float __nv_expm1f(float);
double __nv_fdim(double, double);
float __nv_fdimf(float, float);
double __nv_hypot(double, double);
float __nv_hypotf(float, float);
int __nv_ilogb(double);
int __nv_ilogbf(float);
double __nv_ldexp(double, int);
float __nv_ldexpf(float, int);
long long __nv_llrint(double);
long long __nv_llrintf(float);
long __nv_lrint(double);
long __nv_lrintf(float);
double __nv_log10(double);
float __nv_log10f(float);
double __nv_log1p(double);
float __nv_log1pf(float);
double __nv_log2(double);
float __nv_log2f(float);
double __nv_log(double);
float __nv_logf(float);
double __nv_nextafter(double, double);
float __nv_nextafterf(float, float);
double __nv_pow(double, double);
float __nv_powf(float, float);
double __nv_powi(double, int);
float __nv_powif(float, int);
double __nv_sin(double);
float __nv_sinf(float);
void __nv_sincos(double, double *, double *);
void __nv_sincosf(float, float *, float *);
double __nv_sinh(double);
float __nv_sinhf(float);
double __nv_tan(double);
float __nv_tanf(float);
double __nv_tanh(double);
float __nv_tanhf(float);
double __nv_frexp(double, int *);
float __nv_frexpf(float, int *);
double __nv_scalbn(double, int);
float __nv_scalbnf(float, int);
double __nv_remquo(double, double, int *);
float __nv_remquof(float, float, int *);
double __nv_tgamma(double);
float __nv_tgammaf(float);
float __nv_lgamma(double);
}

} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_MATH_NVPTX_DECLARATIONS_H
