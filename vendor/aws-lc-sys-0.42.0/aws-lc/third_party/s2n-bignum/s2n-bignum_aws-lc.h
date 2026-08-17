/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License").
 * You may not use this file except in compliance with the License.
 * A copy of the License is located at
 *
 *  http://aws.amazon.com/apache2.0
 *
 * or in the "LICENSE" file accompanying this file. This file is distributed
 * on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 * express or implied. See the License for the specific language governing
 * permissions and limitations under the License.
 */
#ifndef S2N_BIGNUM_AWS_LC_H
#define S2N_BIGNUM_AWS_LC_H

#include "s2n-bignum-imported/include/s2n-bignum.h"

// ----------------------------------------------------------------------------
// C prototypes for s2n-bignum functions used in AWS-LC
// ----------------------------------------------------------------------------

// For some functions there are additional variants with names ending in
// "_alt". These have the same core mathematical functionality as their
// non-"alt" versions, but can be better suited to some microarchitectures:
//
//      - On x86, the "_alt" forms avoid BMI and ADX instruction set
//        extensions, so will run on any x86_64 machine, even older ones
//
//      - On ARM, the "_alt" forms target machines with higher multiplier
//        throughput, generally offering higher performance there.
// For each of those, we define a _selector function that selects, in runtime,
// the _alt or non-_alt version to run.

#if defined(OPENSSL_X86_64)
// On x86_64 platforms s2n-bignum uses bmi2 and adx instruction sets
// for some of the functions. These instructions are not supported by
// every x86 CPU so we have to check if they are available and in case
// they are not we fallback to slightly slower but generic implementation.
static inline uint8_t use_s2n_bignum_alt(void) {
  return (!CRYPTO_is_BMI2_capable() || !CRYPTO_is_ADX_capable());
}
#else
// On aarch64 platforms s2n-bignum has two implementations of certain
// functions -- the default one and the alternative (suffixed _alt).
// Depending on the architecture one version is faster than the other.
// Generally, the "_alt" functions are faster on architectures with higher
// multiplier throughput, for example, Graviton 3, Apple's M1 and iPhone chips.
static inline uint8_t use_s2n_bignum_alt(void) {
  return CRYPTO_is_ARMv8_wide_multiplier_capable();
}
#endif

#define S2NBIGNUM_KSQR_16_32_TEMP_NWORDS 24
#define S2NBIGNUM_KMUL_16_32_TEMP_NWORDS 32
#define S2NBIGNUM_KSQR_32_64_TEMP_NWORDS 72
#define S2NBIGNUM_KMUL_32_64_TEMP_NWORDS 96

static inline void p256_montjscalarmul_selector(uint64_t res[S2N_BIGNUM_STATIC 12], const uint64_t scalar[S2N_BIGNUM_STATIC 4], uint64_t point[S2N_BIGNUM_STATIC 12]) {
  if (use_s2n_bignum_alt()) { p256_montjscalarmul_alt(res, scalar, point); }
  else { p256_montjscalarmul(res, scalar, point); }
}

static inline void bignum_deamont_p384_selector(uint64_t z[S2N_BIGNUM_STATIC 6], const uint64_t x[S2N_BIGNUM_STATIC 6]) {
  if (use_s2n_bignum_alt()) { bignum_deamont_p384_alt(z, x); }
  else { bignum_deamont_p384(z, x); }
}

static inline void bignum_montmul_p384_selector(uint64_t z[S2N_BIGNUM_STATIC 6], const uint64_t x[S2N_BIGNUM_STATIC 6], const uint64_t y[S2N_BIGNUM_STATIC 6]) {
  if (use_s2n_bignum_alt()) { bignum_montmul_p384_alt(z, x, y); }
  else { bignum_montmul_p384(z, x, y); }
}

static inline void bignum_montsqr_p384_selector(uint64_t z[S2N_BIGNUM_STATIC 6], const uint64_t x[S2N_BIGNUM_STATIC 6]) {
  if (use_s2n_bignum_alt()) { bignum_montsqr_p384_alt(z, x); }
  else { bignum_montsqr_p384(z, x); }
}

static inline void bignum_tomont_p384_selector(uint64_t z[S2N_BIGNUM_STATIC 6], const uint64_t x[S2N_BIGNUM_STATIC 6]) {
  if (use_s2n_bignum_alt()) { bignum_tomont_p384_alt(z, x); }
  else { bignum_tomont_p384(z, x); }
}

static inline void p384_montjdouble_selector(uint64_t p3[S2N_BIGNUM_STATIC 18],uint64_t p1[S2N_BIGNUM_STATIC 18]) {
    if (use_s2n_bignum_alt()) { p384_montjdouble_alt(p3, p1); }
    else { p384_montjdouble(p3, p1); }
}

static inline void p384_montjscalarmul_selector(uint64_t res[S2N_BIGNUM_STATIC 18], const uint64_t scalar[S2N_BIGNUM_STATIC 6], uint64_t point[S2N_BIGNUM_STATIC 18]) {
  if (use_s2n_bignum_alt()) { p384_montjscalarmul_alt(res, scalar, point); }
  else { p384_montjscalarmul(res, scalar, point); }
}

static inline void bignum_mul_p521_selector(uint64_t z[S2N_BIGNUM_STATIC 9], const uint64_t x[S2N_BIGNUM_STATIC 9], const uint64_t y[S2N_BIGNUM_STATIC 9]) {
  if (use_s2n_bignum_alt()) { bignum_mul_p521_alt(z, x, y); }
  else { bignum_mul_p521(z, x, y); }
}

static inline void bignum_sqr_p521_selector(uint64_t z[S2N_BIGNUM_STATIC 9], const uint64_t x[S2N_BIGNUM_STATIC 9]) {
  if (use_s2n_bignum_alt()) { bignum_sqr_p521_alt(z, x); }
  else { bignum_sqr_p521(z, x); }
}

static inline void p521_jdouble_selector(uint64_t p3[S2N_BIGNUM_STATIC 27],uint64_t p1[S2N_BIGNUM_STATIC 27]) {
    if (use_s2n_bignum_alt()) { p521_jdouble_alt(p3, p1); }
    else { p521_jdouble(p3, p1); }
}

static inline void p521_jscalarmul_selector(uint64_t res[S2N_BIGNUM_STATIC 27], const uint64_t scalar[S2N_BIGNUM_STATIC 9], const uint64_t point[S2N_BIGNUM_STATIC 27]) {
    if (use_s2n_bignum_alt()) { p521_jscalarmul_alt(res, scalar, point); }
    else { p521_jscalarmul(res, scalar, point); }
}

static inline void curve25519_x25519_byte_selector(uint8_t res[S2N_BIGNUM_STATIC 32], const uint8_t scalar[S2N_BIGNUM_STATIC 32], const uint8_t point[S2N_BIGNUM_STATIC 32]) {
  if (use_s2n_bignum_alt()) { curve25519_x25519_byte_alt(res, scalar, point); }
  else { curve25519_x25519_byte(res, scalar, point); }
}

static inline void curve25519_x25519base_byte_selector(uint8_t res[S2N_BIGNUM_STATIC 32], const uint8_t scalar[S2N_BIGNUM_STATIC 32]) {
  if (use_s2n_bignum_alt()) { curve25519_x25519base_byte_alt(res, scalar); }
  else { curve25519_x25519base_byte(res, scalar); }
}

static inline void bignum_madd_n25519_selector(uint64_t z[S2N_BIGNUM_STATIC 4], uint64_t x[S2N_BIGNUM_STATIC 4], uint64_t y[S2N_BIGNUM_STATIC 4], uint64_t c[S2N_BIGNUM_STATIC 4]) {
  if (use_s2n_bignum_alt()) { bignum_madd_n25519_alt(z, x, y, c); }
  else { bignum_madd_n25519(z, x, y, c); }
}

static inline uint64_t edwards25519_decode_selector(uint64_t z[S2N_BIGNUM_STATIC 8], const uint8_t c[S2N_BIGNUM_STATIC 32]) {
  if (use_s2n_bignum_alt()) { return edwards25519_decode_alt(z, c); }
  else { return edwards25519_decode(z, c); }
}

static inline void edwards25519_scalarmulbase_selector(uint64_t res[S2N_BIGNUM_STATIC 8], uint64_t scalar[S2N_BIGNUM_STATIC 4]) {
  if (use_s2n_bignum_alt()) { edwards25519_scalarmulbase_alt(res, scalar); }
  else { edwards25519_scalarmulbase(res, scalar); }
}

static inline void edwards25519_scalarmuldouble_selector(uint64_t res[S2N_BIGNUM_STATIC 8], uint64_t scalar[S2N_BIGNUM_STATIC 4], uint64_t point[S2N_BIGNUM_STATIC 8], uint64_t bscalar[S2N_BIGNUM_STATIC 4]) {
  if (use_s2n_bignum_alt()) { edwards25519_scalarmuldouble_alt(res, scalar, point, bscalar); }
  else { edwards25519_scalarmuldouble(res, scalar, point, bscalar); }
}

#endif // S2N_BIGNUM_AWS_LC_H
