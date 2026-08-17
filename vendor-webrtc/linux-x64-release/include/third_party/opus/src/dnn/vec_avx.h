/* Copyright (c) 2018 Mozilla
                 2012-2017 Jean-Marc Valin */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR
   CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/*
  AVX implementation of vector operations, compile with -mavx
  AVX2/FMA implementation of vector operations, compile with -mavx2 -mfma
*/

#ifndef VEC_AVX_H
#define VEC_AVX_H

#include <immintrin.h>
#include <math.h>
#include "celt/x86/x86cpu.h"

#define MAX_INPUTS (2048)

#define USE_SU_BIAS

#ifndef __SSE_4_1__
static inline __m128 mm_floor_ps(__m128 x) {
  __m128 half = _mm_set1_ps(0.5);
  return _mm_cvtepi32_ps(_mm_cvtps_epi32(_mm_sub_ps(x, half)));
}
#undef _mm_floor_ps
#define _mm_floor_ps(x) mm_floor_ps(x)
#endif


/* If we don't have AVX available, emulate what we need with SSE up to 4.1. */
#ifndef __AVX__

typedef struct {
  __m128 lo;
  __m128 hi;
} mm256_emu;
#define __m256 mm256_emu

static inline mm256_emu mm256_loadu_ps(const float *src) {
  mm256_emu ret;
  ret.lo = _mm_loadu_ps(&src[0]);
  ret.hi = _mm_loadu_ps(&src[4]);
  return ret;
}
#define _mm256_loadu_ps(src) mm256_loadu_ps(src)


static inline void mm256_storeu_ps(float *dst, mm256_emu src) {
  _mm_storeu_ps(dst, src.lo);
  _mm_storeu_ps(&dst[4], src.hi);
}
#define _mm256_storeu_ps(dst, src) mm256_storeu_ps(dst, src)


static inline mm256_emu mm256_setzero_ps(void) {
  mm256_emu ret;
  ret.lo = _mm_setzero_ps();
  ret.hi = ret.lo;
  return ret;
}
#define _mm256_setzero_ps mm256_setzero_ps

static inline mm256_emu mm256_broadcast_ss(const float *x) {
  mm256_emu ret;
  ret.lo = _mm_set1_ps(*x);
  ret.hi = ret.lo;
  return ret;
}
#define _mm256_broadcast_ss(x) mm256_broadcast_ss(x)

static inline mm256_emu mm256_set1_ps(float x) {
  mm256_emu ret;
  ret.lo = _mm_set1_ps(x);
  ret.hi = ret.lo;
  return ret;
}
#define _mm256_set1_ps(x) mm256_set1_ps(x)



static inline mm256_emu mm256_mul_ps(mm256_emu a, mm256_emu b) {
  mm256_emu ret;
  ret.lo = _mm_mul_ps(a.lo, b.lo);
  ret.hi = _mm_mul_ps(a.hi, b.hi);
  return ret;
}
#define _mm256_mul_ps(a,b) mm256_mul_ps(a,b)

static inline mm256_emu mm256_add_ps(mm256_emu a, mm256_emu b) {
  mm256_emu ret;
  ret.lo = _mm_add_ps(a.lo, b.lo);
  ret.hi = _mm_add_ps(a.hi, b.hi);
  return ret;
}
#define _mm256_add_ps(a,b) mm256_add_ps(a,b)


static inline mm256_emu mm256_max_ps(mm256_emu a, mm256_emu b) {
  mm256_emu ret;
  ret.lo = _mm_max_ps(a.lo, b.lo);
  ret.hi = _mm_max_ps(a.hi, b.hi);
  return ret;
}
#define _mm256_max_ps(a,b) mm256_max_ps(a,b)

static inline mm256_emu mm256_min_ps(mm256_emu a, mm256_emu b) {
  mm256_emu ret;
  ret.lo = _mm_min_ps(a.lo, b.lo);
  ret.hi = _mm_min_ps(a.hi, b.hi);
  return ret;
}
#define _mm256_min_ps(a,b) mm256_min_ps(a,b)

static inline mm256_emu mm256_rcp_ps(mm256_emu a) {
  mm256_emu ret;
  ret.lo = _mm_rcp_ps(a.lo);
  ret.hi = _mm_rcp_ps(a.hi);
  return ret;
}
#define _mm256_rcp_ps(a) mm256_rcp_ps(a)


static inline __m128 mm256_extractf128_ps(mm256_emu x, int i) {
    return (i==0) ? x.lo : x.hi;
}
#undef _mm256_extractf128_ps
#define _mm256_extractf128_ps(x,i) mm256_extractf128_ps(x,i)

static inline mm256_emu mm256_insertf128_ps(mm256_emu dst, __m128 src, int i) {
    if (i==0) dst.lo = src;
    else dst.hi = src;
    return dst;
}
#undef _mm256_insertf128_ps
#define _mm256_insertf128_ps(dst,src,i) mm256_insertf128_ps(dst,src,i)

#endif /* __AVX__ */



/* If we don't have AVX2 available, emulate what we need with SSE up to 4.1. */
#ifndef __AVX2__

typedef struct {
  __m128i lo;
  __m128i hi;
} mm256i_emu;
typedef __m256i real_m256i;
#define __m256i mm256i_emu

static inline mm256i_emu mm256_setzero_si256(void) {
  mm256i_emu ret;
  ret.lo = _mm_setzero_si128();
  ret.hi = ret.lo;
  return ret;
}
#define _mm256_setzero_si256 mm256_setzero_si256


static inline mm256i_emu mm256_loadu_si256(const mm256i_emu *src) {
  mm256i_emu ret;
  ret.lo = _mm_loadu_si128((const __m128i*)src);
  ret.hi = _mm_loadu_si128(&((const __m128i*)src)[1]);
  return ret;
}
#define _mm256_loadu_si256(src) mm256_loadu_si256(src)


static inline void mm256_storeu_si256(mm256i_emu *dst, mm256i_emu src) {
  _mm_storeu_si128((__m128i*)dst, src.lo);
  _mm_storeu_si128(&((__m128i*)dst)[1], src.hi);
}
#define _mm256_storeu_si256(dst, src) mm256_storeu_si256(dst, src)


static inline mm256i_emu mm256_broadcastd_epi32(__m128i x) {
  mm256i_emu ret;
  ret.hi = ret.lo = _mm_shuffle_epi32(x, 0);
  return ret;
}
#define _mm256_broadcastd_epi32(x) mm256_broadcastd_epi32(x)


static inline mm256i_emu mm256_set1_epi32(int x) {
  mm256i_emu ret;
  ret.lo = _mm_set1_epi32(x);
  ret.hi = ret.lo;
  return ret;
}
#define _mm256_set1_epi32(x) mm256_set1_epi32(x)

static inline mm256i_emu mm256_set1_epi16(int x) {
  mm256i_emu ret;
  ret.lo = _mm_set1_epi16(x);
  ret.hi = ret.lo;
  return ret;
}
#define _mm256_set1_epi16(x) mm256_set1_epi16(x)


static inline mm256i_emu mm256_add_epi32(mm256i_emu a, mm256i_emu b) {
  mm256i_emu ret;
  ret.lo = _mm_add_epi32(a.lo, b.lo);
  ret.hi = _mm_add_epi32(a.hi, b.hi);
  return ret;
}
#define _mm256_add_epi32(a,b) mm256_add_epi32(a,b)

static inline mm256i_emu mm256_madd_epi16(mm256i_emu a, mm256i_emu b) {
  mm256i_emu ret;
  ret.lo = _mm_madd_epi16(a.lo, b.lo);
  ret.hi = _mm_madd_epi16(a.hi, b.hi);
  return ret;
}
#define _mm256_madd_epi16(a,b) mm256_madd_epi16(a,b)

static inline mm256i_emu mm256_maddubs_epi16(mm256i_emu a, mm256i_emu b) {
  mm256i_emu ret;
  ret.lo = _mm_maddubs_epi16(a.lo, b.lo);
  ret.hi = _mm_maddubs_epi16(a.hi, b.hi);
  return ret;
}
#define _mm256_maddubs_epi16(a,b) mm256_maddubs_epi16(a,b)



/* Emulating the conversion functions is tricky because they use __m256i but are defined in AVX.
   So we need to make a special when only AVX is available. */
#ifdef __AVX__

typedef union {
  mm256i_emu fake;
  real_m256i real;
} mm256_union;

static inline __m256 mm256_cvtepi32_ps(mm256i_emu a) {
  mm256_union src;
  src.fake = a;
  return _mm256_cvtepi32_ps(src.real);
}
#define _mm256_cvtepi32_ps(a) mm256_cvtepi32_ps(a)

static inline mm256i_emu mm256_cvtps_epi32(__m256 a) {
  mm256_union ret;
  ret.real =   _mm256_cvtps_epi32(a);
  return ret.fake;
}
#define _mm256_cvtps_epi32(a) mm256_cvtps_epi32(a)


#else

static inline mm256_emu mm256_cvtepi32_ps(mm256i_emu a) {
  mm256_emu ret;
  ret.lo = _mm_cvtepi32_ps(a.lo);
  ret.hi = _mm_cvtepi32_ps(a.hi);
  return ret;
}
#define _mm256_cvtepi32_ps(a) mm256_cvtepi32_ps(a)

static inline mm256i_emu mm256_cvtps_epi32(mm256_emu a) {
  mm256i_emu ret;
  ret.lo = _mm_cvtps_epi32(a.lo);
  ret.hi = _mm_cvtps_epi32(a.hi);
  return ret;
}
#define _mm256_cvtps_epi32(a) mm256_cvtps_epi32(a)

#endif /* __AVX__ */


#endif /* __AVX2__ */

/* In case we don't have FMA, make it a mul and an add. */
#if !(defined(__FMA__) && defined(__AVX__))
#define _mm256_fmadd_ps(a,b,c) _mm256_add_ps(_mm256_mul_ps(a, b), c)
#define _mm_fmadd_ps(a,b,c) _mm_add_ps(_mm_mul_ps(a, b), c)
#endif

#ifdef __AVX2__
static inline __m256 exp8_approx(__m256 X)
{
   const __m256 K0 = _mm256_set1_ps(0.99992522f);
   const __m256 K1 = _mm256_set1_ps(0.69583354f);
   const __m256 K2 = _mm256_set1_ps(0.22606716f);
   const __m256 K3 = _mm256_set1_ps(0.078024523f);
   const __m256 log2_E = _mm256_set1_ps(1.44269504f);
   const __m256 max_in = _mm256_set1_ps(50.f);
   const __m256 min_in = _mm256_set1_ps(-50.f);
   __m256 XF, Y;
   __m256i I;
   X = _mm256_mul_ps(X, log2_E);
   X = _mm256_max_ps(min_in, _mm256_min_ps(max_in, X));
   XF = _mm256_floor_ps(X);
   I = _mm256_cvtps_epi32(XF);
   X = _mm256_sub_ps(X, XF);
   Y = _mm256_fmadd_ps(_mm256_fmadd_ps(_mm256_fmadd_ps(K3, X, K2), X, K1), X, K0);
   I = _mm256_slli_epi32(I, 23);
   Y = _mm256_castsi256_ps(_mm256_add_epi32(I, _mm256_castps_si256(Y)));
   return Y;
}

static inline void vector_ps_to_epi8(unsigned char *x, const float *_x, int len) {
    int i;
   __m256 const127 = _mm256_set1_ps(127.f);
    for (i=0;i<len;i+=8) {
       __m256 xf;
       __m256i xi;
       xf = _mm256_loadu_ps(&_x[i]);
       xf = _mm256_fmadd_ps(xf, const127, const127);
       xi = _mm256_cvtps_epi32(xf);
       xi = _mm256_packus_epi32(xi,  _mm256_setzero_si256());
       xi = _mm256_permute4x64_epi64(xi, 0xD8);
       xi = _mm256_packus_epi16(xi, _mm256_setzero_si256());
       xi = _mm256_permutevar8x32_epi32(xi, _mm256_setr_epi32(0,1, 0,0, 0,0, 0,0));
       _mm256_storeu_si256 ((__m256i *)(void*)&x[i], xi);
   }
}

#else
static inline __m128 exp4_approx(__m128 X)
{
   const __m128 K0 = _mm_set1_ps(0.99992522f);
   const __m128 K1 = _mm_set1_ps(0.69583354f);
   const __m128 K2 = _mm_set1_ps(0.22606716f);
   const __m128 K3 = _mm_set1_ps(0.078024523f);
   const __m128 log2_E = _mm_set1_ps(1.44269504);
   const __m128 max_in = _mm_set1_ps(50.f);
   const __m128 min_in = _mm_set1_ps(-50.f);
   const __m128i mask = _mm_set1_epi32(0x7fffffff);
   __m128 XF, Y;
   __m128i I;
   X = _mm_mul_ps(X, log2_E);
   X = _mm_max_ps(min_in, _mm_min_ps(max_in, X));
   XF = _mm_floor_ps(X);
   I = _mm_cvtps_epi32(XF);
   X = _mm_sub_ps(X, XF);
   Y = _mm_fmadd_ps(_mm_fmadd_ps(_mm_fmadd_ps(K3, X, K2), X, K1), X, K0);
   I = _mm_slli_epi32(I, 23);
   Y = _mm_castsi128_ps(_mm_and_si128(mask, _mm_add_epi32(I, _mm_castps_si128(Y))));
   return Y;
}
static inline __m256 exp8_approx(__m256 X)
{
   __m256 Y;
   __m128 Xhi, Xlo, Yhi, Ylo;
   Xhi = _mm256_extractf128_ps(X, 1);
   Xlo = _mm256_extractf128_ps(X, 0);
   Yhi = exp4_approx(Xhi);
   Ylo = exp4_approx(Xlo);
   Y = _mm256_insertf128_ps(_mm256_setzero_ps(), Yhi, 1);
   Y = _mm256_insertf128_ps(Y, Ylo, 0);
   return Y;
}

static inline void vector_ps_to_epi8(unsigned char *x, const float *_x, int len) {
    int i;
    for (i=0;i<len;i++) x[i] = 127+(int)floor(.5+127*_x[i]);
}

#endif


#ifdef __AVX__

/* Approximating tanh() using a Padé-like rational function:
   tanh(x) ~= x * (N0 + N1*x^2 + N2*x^4)/(D0 + D1*x^2 + D2*x^4)
   subject to the +/- 1 bounds.
   The coefficients were determined by gradient descent trying to minimize
   the maximum deviation over the whole range (this is only possible because
   of the bounds). The max error is around 3e-4 and is dominated by the
   reciprocal approximation (the max error of the rational function is
   around 6e-5).
   */
static inline __m256 tanh8_approx(__m256 X)
{
   const __m256 N0 = _mm256_set1_ps(952.52801514f);
   const __m256 N1 = _mm256_set1_ps(96.39235687f);
   const __m256 N2 = _mm256_set1_ps(0.60863042f);
   const __m256 D0 = _mm256_set1_ps(952.72399902f);
   const __m256 D1 = _mm256_set1_ps(413.36801147f);
   const __m256 D2 = _mm256_set1_ps(11.88600922f);
   const __m256 max_out = _mm256_set1_ps(1.f);
   const __m256 min_out = _mm256_set1_ps(-1.f);
   __m256 X2, num, den;
   X2 = _mm256_mul_ps(X, X);
   num = _mm256_fmadd_ps(_mm256_fmadd_ps(N2, X2, N1), X2, N0);
   den = _mm256_fmadd_ps(_mm256_fmadd_ps(D2, X2, D1), X2, D0);
   num = _mm256_mul_ps(num, X);
   den = _mm256_rcp_ps(den);
   num = _mm256_mul_ps(num, den);
   return _mm256_max_ps(min_out, _mm256_min_ps(max_out, num));
}

/* Sigmoid approximation using a Padé-like rational function:
   1/(1+exp(-x)) ~= 0.5 + x * (N0 + N1*x^2 + N2*x^4)/(D0 + D1*x^2 + D2*x^4)
   subject to the [0, 1] bounds.
   The coefficients are directly derived by dividing the tanh() coefficients
   by powers of two to get the correct scaling. The max error is around 1.5e-4
   and is dominated by the reciprocal approximation (the max error of the
   rational function is around 3e-5).
   */
static inline __m256 sigmoid8_approx(__m256 X)
{
   const __m256 N0 = _mm256_set1_ps(238.13200378f);
   const __m256 N1 = _mm256_set1_ps(6.02452230f);
   const __m256 N2 = _mm256_set1_ps(0.00950985f);
   const __m256 D0 = _mm256_set1_ps(952.72399902f);
   const __m256 D1 = _mm256_set1_ps(103.34200287f);
   const __m256 D2 = _mm256_set1_ps(0.74287558f);
   const __m256 half = _mm256_set1_ps(0.5);
   const __m256 max_out = _mm256_set1_ps(1.f);
   const __m256 min_out = _mm256_set1_ps(0.f);
   __m256 X2, num, den;
   X2 = _mm256_mul_ps(X, X);
   num = _mm256_fmadd_ps(_mm256_fmadd_ps(N2, X2, N1), X2, N0);
   den = _mm256_fmadd_ps(_mm256_fmadd_ps(D2, X2, D1), X2, D0);
   num = _mm256_mul_ps(num, X);
   den = _mm256_rcp_ps(den);
   num = _mm256_fmadd_ps(num, den, half);
   return _mm256_max_ps(min_out, _mm256_min_ps(max_out, num));
}

static inline float tanh_approx(float x)
{
   float out[8];
   __m256 X, Y;
   X = _mm256_set1_ps(x);
   Y = tanh8_approx(X);
   _mm256_storeu_ps(out, Y);
   return out[0];
}

static inline float sigmoid_approx(float x)
{
   float out[8];
   __m256 X, Y;
   X = _mm256_set1_ps(x);
   Y = sigmoid8_approx(X);
   _mm256_storeu_ps(out, Y);
   return out[0];
}

#else

static inline __m128 tanh4_approx(__m128 X)
{
   const __m128 N0 = _mm_set1_ps(952.52801514f);
   const __m128 N1 = _mm_set1_ps(96.39235687f);
   const __m128 N2 = _mm_set1_ps(0.60863042f);
   const __m128 D0 = _mm_set1_ps(952.72399902f);
   const __m128 D1 = _mm_set1_ps(413.36801147f);
   const __m128 D2 = _mm_set1_ps(11.88600922f);
   const __m128 max_out = _mm_set1_ps(1.f);
   const __m128 min_out = _mm_set1_ps(-1.f);
   __m128 X2, num, den;
   X2 = _mm_mul_ps(X, X);
   num = _mm_fmadd_ps(_mm_fmadd_ps(N2, X2, N1), X2, N0);
   den = _mm_fmadd_ps(_mm_fmadd_ps(D2, X2, D1), X2, D0);
   num = _mm_mul_ps(num, X);
   den = _mm_rcp_ps(den);
   num = _mm_mul_ps(num, den);
   return _mm_max_ps(min_out, _mm_min_ps(max_out, num));
}

static inline __m128 sigmoid4_approx(__m128 X)
{
   const __m128 N0 = _mm_set1_ps(238.13200378f);
   const __m128 N1 = _mm_set1_ps(6.02452230f);
   const __m128 N2 = _mm_set1_ps(0.00950985f);
   const __m128 D0 = _mm_set1_ps(952.72399902f);
   const __m128 D1 = _mm_set1_ps(103.34200287f);
   const __m128 D2 = _mm_set1_ps(0.74287558f);
   const __m128 half = _mm_set1_ps(0.5);
   const __m128 max_out = _mm_set1_ps(1.f);
   const __m128 min_out = _mm_set1_ps(0.f);
   __m128 X2, num, den;
   X2 = _mm_mul_ps(X, X);
   num = _mm_fmadd_ps(_mm_fmadd_ps(N2, X2, N1), X2, N0);
   den = _mm_fmadd_ps(_mm_fmadd_ps(D2, X2, D1), X2, D0);
   num = _mm_mul_ps(num, X);
   den = _mm_rcp_ps(den);
   num = _mm_fmadd_ps(num, den, half);
   return _mm_max_ps(min_out, _mm_min_ps(max_out, num));
}

static inline float tanh_approx(float x)
{
   float out[4];
   __m128 X, Y;
   X = _mm_set1_ps(x);
   Y = tanh4_approx(X);
   _mm_storeu_ps(out, Y);
   return out[0];
}

static inline float sigmoid_approx(float x)
{
   float out[4];
   __m128 X, Y;
   X = _mm_set1_ps(x);
   Y = sigmoid4_approx(X);
   _mm_storeu_ps(out, Y);
   return out[0];
}

#endif

static inline float lpcnet_exp(float x)
{
   float out[8];
   __m256 X, Y;
   X = _mm256_set1_ps(x);
   Y = exp8_approx(X);
   _mm256_storeu_ps(out, Y);
   return out[0];
}

static inline void softmax(float *y, const float *x, int N)
{
    int i;
    for (i=0;i<N-7;i+=8)
    {
        __m256 X, Y;
        X = _mm256_loadu_ps(&x[i]);
        Y = exp8_approx(X);
        _mm256_storeu_ps(&y[i], Y);
    }
    for (;i<N;i++)
        y[i] = lpcnet_exp(x[i]);
}

#ifdef __AVX__
static inline void vec_tanh(float *y, const float *x, int N)
{
    int i;
    for (i=0;i<N-7;i+=8)
    {
        __m256 X, Y;
        X = _mm256_loadu_ps(&x[i]);
        Y = tanh8_approx(X);
        _mm256_storeu_ps(&y[i], Y);
    }
    for (;i<N;i++)
    {
        y[i] = tanh_approx(x[i]);
    }
}

static inline void vec_sigmoid(float *y, const float *x, int N)
{
    int i;
    for (i=0;i<N-7;i+=8)
    {
        __m256 X, Y;
        X = _mm256_loadu_ps(&x[i]);
        Y = sigmoid8_approx(X);
        _mm256_storeu_ps(&y[i], Y);
    }
    for (;i<N;i++)
    {
        y[i] = sigmoid_approx(x[i]);
    }
}
#else
static inline void vec_tanh(float *y, const float *x, int N)
{
    int i;
    for (i=0;i<N-3;i+=4)
    {
        __m128 X, Y;
        X = _mm_loadu_ps(&x[i]);
        Y = tanh4_approx(X);
        _mm_storeu_ps(&y[i], Y);
    }
    for (;i<N;i++)
    {
        y[i] = tanh_approx(x[i]);
    }
}

static inline void vec_sigmoid(float *y, const float *x, int N)
{
    int i;
    for (i=0;i<N-3;i+=4)
    {
        __m128 X, Y;
        X = _mm_loadu_ps(&x[i]);
        Y = sigmoid4_approx(X);
        _mm_storeu_ps(&y[i], Y);
    }
    for (;i<N;i++)
    {
        y[i] = sigmoid_approx(x[i]);
    }
}

#endif

#if defined(__AVXVNNI__) || defined(__AVX512VNNI__)

#define opus_mm256_dpbusds_epi32(src, a, b) _mm256_dpbusds_epi32(src, a, b)

#elif defined(__AVX2__)

static inline __m256i opus_mm256_dpbusds_epi32(__m256i src, __m256i a, __m256i b) {
  __m256i ones, tmp;
  ones = _mm256_set1_epi16(1);
  tmp = _mm256_maddubs_epi16(a, b);
  tmp = _mm256_madd_epi16(tmp, ones);
  return _mm256_add_epi32(src, tmp);
}

#elif defined(__SSSE3__)

static inline mm256i_emu opus_mm256_dpbusds_epi32(mm256i_emu src, mm256i_emu a, mm256i_emu b) {
  mm256i_emu ones, tmp;
  ones = _mm256_set1_epi16(1);
  tmp = _mm256_maddubs_epi16(a, b);
  tmp = _mm256_madd_epi16(tmp, ones);
  return _mm256_add_epi32(src, tmp);
}

#elif defined(__SSE2__)

static inline __m128i mm_dpbusds_epi32(__m128i src, __m128i a, __m128i b) {
  __m128i ah, al, bh, bl, tmp;
  ah = _mm_srli_epi16(a, 8);
  bh = _mm_srai_epi16(b, 8);
  al = _mm_srli_epi16(_mm_slli_epi16(a, 8), 8);
  bl = _mm_srai_epi16(_mm_slli_epi16(b, 8), 8);
  tmp = _mm_add_epi32(_mm_madd_epi16(ah, bh), _mm_madd_epi16(al, bl));
  return _mm_add_epi32(src, tmp);
}

static inline mm256i_emu opus_mm256_dpbusds_epi32(mm256i_emu src, mm256i_emu a, mm256i_emu b) {
  mm256i_emu res;
  res.hi = mm_dpbusds_epi32(src.hi, a.hi, b.hi);
  res.lo = mm_dpbusds_epi32(src.lo, a.lo, b.lo);
  return res;
}


#else

#error "No optimizations in vec_avx.h. This should never happen. "
#endif

static inline void sgemv(float *out, const float *weights, int rows, int cols, int col_stride, const float *x)
{
  int i, j;
  i=0;
  for (;i<rows-15;i+=16)
  {
     float *y;
     __m256 vy0, vy8;
     y = &out[i];
     vy0 = _mm256_setzero_ps();
     vy8 = _mm256_setzero_ps();
     for (j=0;j<cols;j++)
     {
        __m256 vxj;
        __m256 vw;
        vxj = _mm256_broadcast_ss(&x[j]);

        vw = _mm256_loadu_ps(&weights[j*col_stride + i]);
        vy0 = _mm256_fmadd_ps(vw, vxj, vy0);

        vw = _mm256_loadu_ps(&weights[j*col_stride + i + 8]);
        vy8 = _mm256_fmadd_ps(vw, vxj, vy8);
     }
     _mm256_storeu_ps (&y[0], vy0);
     _mm256_storeu_ps (&y[8], vy8);
  }
  for (;i<rows-7;i+=8)
  {
     float *y;
     __m256 vy0;
     y = &out[i];
     vy0 = _mm256_setzero_ps();
     for (j=0;j<cols;j++)
     {
        __m256 vxj;
        __m256 vw;
        vxj = _mm256_broadcast_ss(&x[j]);

        vw = _mm256_loadu_ps(&weights[j*col_stride + i]);
        vy0 = _mm256_fmadd_ps(vw, vxj, vy0);
     }
     _mm256_storeu_ps (&y[0], vy0);
  }
  for (;i<rows-3;i+=4)
  {
     float *y;
     __m128 vy0;
     y = &out[i];
     vy0 = _mm_setzero_ps();
     for (j=0;j<cols;j++)
     {
        __m128 vxj;
        __m128 vw;
        vxj = _mm_set1_ps(x[j]);

        vw = _mm_loadu_ps(&weights[j*col_stride + i]);
        vy0 = _mm_fmadd_ps(vw, vxj, vy0);
     }
     _mm_storeu_ps (&y[0], vy0);
  }
  for (;i<rows;i++)
  {
    out[i] = 0;
    for (j=0;j<cols;j++) out[i] += weights[j*col_stride + i]*x[j];
  }
}

static inline void sparse_sgemv8x4(float *out, const float *weights, const int *idx, int rows, const float *x)
{
   int i, j;
   for (i=0;i<rows;i+=8)
   {
      float *y;
      int cols;
      __m256 vy0;
      y = &out[i];
      vy0 = _mm256_setzero_ps();
      cols = *idx++;
      for (j=0;j<cols;j++)
      {
         int id;
         __m256 vxj;
         __m256 vw;
         id = *idx++;
         vxj = _mm256_broadcast_ss(&x[id]);
         vw = _mm256_loadu_ps(&weights[0]);
         vy0 = _mm256_fmadd_ps(vw, vxj, vy0);

         vxj = _mm256_broadcast_ss(&x[id+1]);
         vw = _mm256_loadu_ps(&weights[8]);
         vy0 = _mm256_fmadd_ps(vw, vxj, vy0);

         vxj = _mm256_broadcast_ss(&x[id+2]);
         vw = _mm256_loadu_ps(&weights[16]);
         vy0 = _mm256_fmadd_ps(vw, vxj, vy0);

         vxj = _mm256_broadcast_ss(&x[id+3]);
         vw = _mm256_loadu_ps(&weights[24]);
         vy0 = _mm256_fmadd_ps(vw, vxj, vy0);

         weights += 32;
      }
      _mm256_storeu_ps (&y[0], vy0);
   }
}

static inline void sparse_cgemv8x4(float *_out, const opus_int8 *w, const int *idx, const float *scale, int rows, int cols, const float *_x)
{
   int i, j;
   unsigned char x[MAX_INPUTS];
   /*for (i=0;i<cols;i++) x[i] = 127+floor(.5+127*_x[i]);*/
   vector_ps_to_epi8(x, _x, cols);
   for (i=0;i<rows;i+=8)
   {
      int colblocks;
      __m256i vy0;
      __m256 vout;
      colblocks = *idx++;
      vy0 = _mm256_setzero_si256();
      j=0;
#if 1 /* Unrolling by 4 gives some gain, comment out if it does not. */
      for (;j<colblocks-3;j+=4)
      {
         __m256i vxj;
         __m256i vw;
         vxj = _mm256_broadcastd_epi32(_mm_loadu_si32(&x[*idx++]));
         vw = _mm256_loadu_si256((const __m256i *)(void*)w);
         vy0 = opus_mm256_dpbusds_epi32(vy0, vxj, vw);
         w += 32;
         vxj = _mm256_broadcastd_epi32(_mm_loadu_si32(&x[*idx++]));
         vw = _mm256_loadu_si256((const __m256i *)(void*)w);
         vy0 = opus_mm256_dpbusds_epi32(vy0, vxj, vw);
         w += 32;
         vxj = _mm256_broadcastd_epi32(_mm_loadu_si32(&x[*idx++]));
         vw = _mm256_loadu_si256((const __m256i *)(void*)w);
         vy0 = opus_mm256_dpbusds_epi32(vy0, vxj, vw);
         w += 32;
         vxj = _mm256_broadcastd_epi32(_mm_loadu_si32(&x[*idx++]));
         vw = _mm256_loadu_si256((const __m256i *)(void*)w);
         vy0 = opus_mm256_dpbusds_epi32(vy0, vxj, vw);
         w += 32;
      }
#endif
      for (;j<colblocks;j++)
      {
         __m256i vxj;
         __m256i vw;
         vxj = _mm256_broadcastd_epi32(_mm_loadu_si32(&x[*idx++]));
         vw = _mm256_loadu_si256((const __m256i *)(void*)w);
         vy0 = opus_mm256_dpbusds_epi32(vy0, vxj, vw);
         w += 32;
      }
      vout = _mm256_cvtepi32_ps(vy0);
      vout = _mm256_mul_ps(vout, _mm256_loadu_ps(&scale[i]));
      _mm256_storeu_ps(&_out[i], vout);
   }
}
static inline void cgemv8x4(float *_out, const opus_int8 *w, const float *scale, int rows, int cols, const float *_x)
{
   int i, j;
   unsigned char x[MAX_INPUTS];
   /*for (i=0;i<cols;i++) x[i] = 127+floor(.5+127*_x[i]);*/
   vector_ps_to_epi8(x, _x, cols);
   for (i=0;i<rows;i+=8)
   {
      __m256i vy0;
      __m256 vout;
      vy0 = _mm256_setzero_si256();
      j=0;
#if 1 /* Unrolling by 4 gives some gain, comment out if it does not. */
      for (;j<cols-12;j+=16)
      {
         __m256i vxj;
         __m256i vw;
         vxj = _mm256_broadcastd_epi32(_mm_loadu_si32(&x[j]));
         vw = _mm256_loadu_si256((const __m256i *)(void*)w);
         vy0 = opus_mm256_dpbusds_epi32(vy0, vxj, vw);
         w += 32;
         vxj = _mm256_broadcastd_epi32(_mm_loadu_si32(&x[j+4]));
         vw = _mm256_loadu_si256((const __m256i *)(void*)w);
         vy0 = opus_mm256_dpbusds_epi32(vy0, vxj, vw);
         w += 32;
         vxj = _mm256_broadcastd_epi32(_mm_loadu_si32(&x[j+8]));
         vw = _mm256_loadu_si256((const __m256i *)(void*)w);
         vy0 = opus_mm256_dpbusds_epi32(vy0, vxj, vw);
         w += 32;
         vxj = _mm256_broadcastd_epi32(_mm_loadu_si32(&x[j+12]));
         vw = _mm256_loadu_si256((const __m256i *)(void*)w);
         vy0 = opus_mm256_dpbusds_epi32(vy0, vxj, vw);
         w += 32;
      }
#endif
      for (;j<cols;j+=4)
      {
         __m256i vxj;
         __m256i vw;
         vxj = _mm256_broadcastd_epi32(_mm_loadu_si32(&x[j]));
         vw = _mm256_loadu_si256((const __m256i *)(void*)w);
         vy0 = opus_mm256_dpbusds_epi32(vy0, vxj, vw);
         w += 32;
      }
      vout = _mm256_cvtepi32_ps(vy0);
      vout = _mm256_mul_ps(vout, _mm256_loadu_ps(&scale[i]));
      _mm256_storeu_ps(&_out[i], vout);
   }
}

#define SCALE (128.f*127.f)
#define SCALE_1 (1.f/128.f/127.f)
#define USE_SU_BIAS


#endif /*VEC_AVX_H*/
