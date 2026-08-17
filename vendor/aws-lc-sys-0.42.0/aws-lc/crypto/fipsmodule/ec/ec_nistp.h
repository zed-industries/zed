// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC
#ifndef EC_NISTP_H
#define EC_NISTP_H

#include <openssl/target.h>

#include <stdint.h>

// We have two implementations of field arithmetic for NIST curves:
//   - Fiat-crypto
//   - s2n-bignum
// Both Fiat-crypto and s2n-bignum implementations are formally verified.
// Fiat-crypto implementation is fully portable C code, while s2n-bignum
// implements the operations in assembly for x86_64 and aarch64 platforms.
// If (1) x86_64 or aarch64, (2) linux or apple, and (3) OPENSSL_NO_ASM is not
// set, s2n-bignum path is capable.
#if !defined(OPENSSL_NO_ASM) &&                                                \
    (defined(OPENSSL_LINUX) || defined(OPENSSL_APPLE)) &&                      \
    ((defined(OPENSSL_X86_64) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_512AVX)) || \
     defined(OPENSSL_AARCH64))
#  define EC_NISTP_USE_S2N_BIGNUM
#  define EC_NISTP_USE_64BIT_LIMB
#else
// Fiat-crypto has both 64-bit and 32-bit implementation.
#  if defined(BORINGSSL_HAS_UINT128)
#    define EC_NISTP_USE_64BIT_LIMB
#  endif
#endif

#if defined(EC_NISTP_USE_64BIT_LIMB)
typedef uint64_t ec_nistp_felem_limb;
#else
typedef uint32_t ec_nistp_felem_limb;
#endif

// ec_nistp_meth is a struct that holds pointers to implementations of field
// and point arithmetic functions for specific curves. It is meant to be used
// in higher level functions like this:
//   void point_double(ec_nistp_meth *ctx, ...) {
//     ctx->felem_add(...);
//     ctx->felem_mul(...);
//
//     ctx->point_dbl(...);
//   }
// This makes the functions reusable between different curves by simply
// providing an appropriate methods object.
typedef struct {
  size_t felem_num_limbs;
  size_t felem_num_bits;
  void (*felem_add)(ec_nistp_felem_limb *c, const ec_nistp_felem_limb *a, const ec_nistp_felem_limb *b);
  void (*felem_sub)(ec_nistp_felem_limb *c, const ec_nistp_felem_limb *a, const ec_nistp_felem_limb *b);
  void (*felem_mul)(ec_nistp_felem_limb *c, const ec_nistp_felem_limb *a, const ec_nistp_felem_limb *b);
  void (*felem_sqr)(ec_nistp_felem_limb *c, const ec_nistp_felem_limb *a);
  void (*felem_neg)(ec_nistp_felem_limb *c, const ec_nistp_felem_limb *a);
  ec_nistp_felem_limb (*felem_nz)(const ec_nistp_felem_limb *a);
  const ec_nistp_felem_limb *felem_one;

  void (*point_dbl)(ec_nistp_felem_limb *x_out,
                    ec_nistp_felem_limb *y_out,
                    ec_nistp_felem_limb *z_out,
                    const ec_nistp_felem_limb *x_in,
                    const ec_nistp_felem_limb *y_in,
                    const ec_nistp_felem_limb *z_in);
  void (*point_add)(ec_nistp_felem_limb *x3,
                    ec_nistp_felem_limb *y3,
                    ec_nistp_felem_limb *z3,
                    const ec_nistp_felem_limb *x1,
                    const ec_nistp_felem_limb *y1,
                    const ec_nistp_felem_limb *z1,
                    const int mixed,
                    const ec_nistp_felem_limb *x2,
                    const ec_nistp_felem_limb *y2,
                    const ec_nistp_felem_limb *z2);

  const ec_nistp_felem_limb *scalar_mul_base_table;
} ec_nistp_meth;

const ec_nistp_meth *p256_methods(void);
const ec_nistp_meth *p384_methods(void);
const ec_nistp_meth *p521_methods(void);

void ec_nistp_point_double(const ec_nistp_meth *ctx,
                           ec_nistp_felem_limb *x_out,
                           ec_nistp_felem_limb *y_out,
                           ec_nistp_felem_limb *z_out,
                           const ec_nistp_felem_limb *x_in,
                           const ec_nistp_felem_limb *y_in,
                           const ec_nistp_felem_limb *z_in);

void ec_nistp_point_add(const ec_nistp_meth *ctx,
                        ec_nistp_felem_limb *x3,
                        ec_nistp_felem_limb *y3,
                        ec_nistp_felem_limb *z3,
                        const ec_nistp_felem_limb *x1,
                        const ec_nistp_felem_limb *y1,
                        const ec_nistp_felem_limb *z1,
                        const int mixed,
                        const ec_nistp_felem_limb *x2,
                        const ec_nistp_felem_limb *y2,
                        const ec_nistp_felem_limb *z2);

void ec_nistp_scalar_mul(const ec_nistp_meth *ctx,
                         ec_nistp_felem_limb *x_out,
                         ec_nistp_felem_limb *y_out,
                         ec_nistp_felem_limb *z_out,
                         const ec_nistp_felem_limb *x_in,
                         const ec_nistp_felem_limb *y_in,
                         const ec_nistp_felem_limb *z_in,
                         const EC_SCALAR *scalar);

void ec_nistp_scalar_mul_base(const ec_nistp_meth *ctx,
                              ec_nistp_felem_limb *x_out,
                              ec_nistp_felem_limb *y_out,
                              ec_nistp_felem_limb *z_out,
                              const EC_SCALAR *scalar);

void ec_nistp_scalar_mul_public(const ec_nistp_meth *ctx,
                                ec_nistp_felem_limb *x_out,
                                ec_nistp_felem_limb *y_out,
                                ec_nistp_felem_limb *z_out,
                                const EC_SCALAR *g_scalar,
                                const ec_nistp_felem_limb *x_p,
                                const ec_nistp_felem_limb *y_p,
                                const ec_nistp_felem_limb *z_p,
                                const EC_SCALAR *p_scalar);

void ec_nistp_point_to_coordinates(ec_nistp_felem_limb *x_out,
                                   ec_nistp_felem_limb *y_out,
                                   ec_nistp_felem_limb *z_out,
                                   const ec_nistp_felem_limb *xyz_in,
                                   size_t num_limbs_per_coord);

void ec_nistp_coordinates_to_point(ec_nistp_felem_limb *xyz_out,
                                   const ec_nistp_felem_limb *x_in,
                                   const ec_nistp_felem_limb *y_in,
                                   const ec_nistp_felem_limb *z_in,
                                   size_t num_limbs_per_coord);
#endif // EC_NISTP_H

