// Copyright 2014-2016 The OpenSSL Project Authors. All Rights Reserved.
// Copyright (c) 2014, Intel Corporation. All Rights Reserved.
//
// Originally written by Shay Gueron (1, 2), and Vlad Krasnov (1)
// (1) Intel Corporation, Israel Development Center, Haifa, Israel
// (2) University of Haifa, Israel
//
// SPDX-License-Identifier: Apache-2.0
//
// Reference:
// S.Gueron and V.Krasnov, "Fast Prime Field Elliptic Curve Cryptography with
//                          256 Bit Primes"

#ifndef OPENSSL_HEADER_EC_P256_X86_64_H
#define OPENSSL_HEADER_EC_P256_X86_64_H

#include <openssl/base.h>

#include <openssl/bn.h>

#include "../bn/internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


#if !defined(OPENSSL_NO_ASM) && \
    (defined(OPENSSL_X86_64) || defined(OPENSSL_AARCH64)) && \
    !defined(OPENSSL_SMALL)

// P-256 field operations.
//
// An element mod P in P-256 is represented as a little-endian array of
// |P256_LIMBS| |BN_ULONG|s, spanning the full range of values.
//
// The following functions take fully-reduced inputs mod P and give
// fully-reduced outputs. They may be used in-place.

#define P256_LIMBS (256 / BN_BITS2)

// ecp_nistz256_neg sets |res| to -|a| mod P.
void ecp_nistz256_neg(BN_ULONG res[P256_LIMBS], const BN_ULONG a[P256_LIMBS]);

// ecp_nistz256_mul_mont sets |res| to |a| * |b| * 2^-256 mod P.
void ecp_nistz256_mul_mont(BN_ULONG res[P256_LIMBS],
                           const BN_ULONG a[P256_LIMBS],
                           const BN_ULONG b[P256_LIMBS]);

// ecp_nistz256_sqr_mont sets |res| to |a| * |a| * 2^-256 mod P.
void ecp_nistz256_sqr_mont(BN_ULONG res[P256_LIMBS],
                           const BN_ULONG a[P256_LIMBS]);

// ecp_nistz256_from_mont sets |res| to |in|, converted from Montgomery domain
// by multiplying with 1.
static inline void ecp_nistz256_from_mont(BN_ULONG res[P256_LIMBS],
                                          const BN_ULONG in[P256_LIMBS]) {
  static const BN_ULONG ONE[P256_LIMBS] = { 1 };
  ecp_nistz256_mul_mont(res, in, ONE);
}


// P-256 scalar operations.
//
// The following functions compute modulo N, where N is the order of P-256. They
// take fully-reduced inputs and give fully-reduced outputs.

// ecp_nistz256_ord_mul_mont sets |res| to |a| * |b| where inputs and outputs
// are in Montgomery form. That is, |res| is |a| * |b| * 2^-256 mod N.
void ecp_nistz256_ord_mul_mont(BN_ULONG res[P256_LIMBS],
                               const BN_ULONG a[P256_LIMBS],
                               const BN_ULONG b[P256_LIMBS]);

// ecp_nistz256_ord_sqr_mont sets |res| to |a|^(2*|rep|) where inputs and
// outputs are in Montgomery form. That is, |res| is
// (|a| * 2^-256)^(2*|rep|) * 2^256 mod N.
void ecp_nistz256_ord_sqr_mont(BN_ULONG res[P256_LIMBS],
                               const BN_ULONG a[P256_LIMBS], BN_ULONG rep);

// beeu_mod_inverse_vartime sets out = a^-1 mod p using a Euclidean algorithm.
// Assumption: 0 < a < p < 2^(256) and p is odd.
int beeu_mod_inverse_vartime(BN_ULONG out[P256_LIMBS],
                             const BN_ULONG a[P256_LIMBS],
                             const BN_ULONG p[P256_LIMBS]);


// P-256 point operations.
//
// The following functions may be used in-place. All coordinates are in the
// Montgomery domain.

// A P256_POINT represents a P-256 point in Jacobian coordinates.
typedef struct {
  BN_ULONG X[P256_LIMBS];
  BN_ULONG Y[P256_LIMBS];
  BN_ULONG Z[P256_LIMBS];
} P256_POINT;

// A P256_POINT_AFFINE represents a P-256 point in affine coordinates. Infinity
// is encoded as (0, 0).
typedef struct {
  BN_ULONG X[P256_LIMBS];
  BN_ULONG Y[P256_LIMBS];
} P256_POINT_AFFINE;

// ecp_nistz256_select_w5 sets |*val| to |in_t[index-1]| if 1 <= |index| <= 16
// and all zeros (the point at infinity) if |index| is 0. This is done in
// constant time.
void ecp_nistz256_select_w5(P256_POINT *val, const P256_POINT in_t[16],
                            int index);

// ecp_nistz256_select_w7 sets |*val| to |in_t[index-1]| if 1 <= |index| <= 64
// and all zeros (the point at infinity) if |index| is 0. This is done in
// constant time.
void ecp_nistz256_select_w7(P256_POINT_AFFINE *val,
                            const P256_POINT_AFFINE in_t[64], int index);

// ecp_nistz256_point_double sets |r| to |a| doubled.
void ecp_nistz256_point_double(P256_POINT *r, const P256_POINT *a);

// ecp_nistz256_point_add adds |a| to |b| and places the result in |r|.
void ecp_nistz256_point_add(P256_POINT *r, const P256_POINT *a,
                            const P256_POINT *b);

// ecp_nistz256_point_add_affine adds |a| to |b| and places the result in
// |r|. |a| and |b| must not represent the same point unless they are both
// infinity.
void ecp_nistz256_point_add_affine(P256_POINT *r, const P256_POINT *a,
                                   const P256_POINT_AFFINE *b);

#endif /* !defined(OPENSSL_NO_ASM) && \
          (defined(OPENSSL_X86_64) || defined(OPENSSL_AARCH64)) &&   \
          !defined(OPENSSL_SMALL) */


#if defined(__cplusplus)
}  // extern C++
#endif

#endif  // OPENSSL_HEADER_EC_P256_X86_64_H
