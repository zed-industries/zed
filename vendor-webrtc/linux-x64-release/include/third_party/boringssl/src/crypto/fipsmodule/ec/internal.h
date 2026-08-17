// Copyright 2001-2016 The OpenSSL Project Authors. All Rights Reserved.
// Copyright (c) 2002, Oracle and/or its affiliates. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_EC_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_EC_INTERNAL_H

#include <openssl/base.h>

#include <assert.h>

#include <openssl/bn.h>
#include <openssl/ec.h>
#include <openssl/ex_data.h>

#include "../bn/internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


// EC internals.


// Cap the size of all field elements and scalars, including custom curves, to
// 66 bytes, large enough to fit secp521r1 and brainpoolP512r1, which appear to
// be the largest fields anyone plausibly uses.
#define EC_MAX_BYTES 66
#define EC_MAX_WORDS ((EC_MAX_BYTES + BN_BYTES - 1) / BN_BYTES)
#define EC_MAX_COMPRESSED (EC_MAX_BYTES + 1)
#define EC_MAX_UNCOMPRESSED (2 * EC_MAX_BYTES + 1)

static_assert(EC_MAX_WORDS <= BN_SMALL_MAX_WORDS,
              "bn_*_small functions not usable");


// Scalars.

// An EC_SCALAR is an integer fully reduced modulo the order. Only the first
// |order->width| words are used. An |EC_SCALAR| is specific to an |EC_GROUP|
// and must not be mixed between groups.
typedef struct {
  BN_ULONG words[EC_MAX_WORDS];
} EC_SCALAR;

// ec_bignum_to_scalar converts |in| to an |EC_SCALAR| and writes it to
// |*out|. It returns one on success and zero if |in| is out of range.
OPENSSL_EXPORT int ec_bignum_to_scalar(const EC_GROUP *group, EC_SCALAR *out,
                                       const BIGNUM *in);

// ec_scalar_to_bytes serializes |in| as a big-endian bytestring to |out| and
// sets |*out_len| to the number of bytes written. The number of bytes written
// is |BN_num_bytes(&group->order)|, which is at most |EC_MAX_BYTES|.
OPENSSL_EXPORT void ec_scalar_to_bytes(const EC_GROUP *group, uint8_t *out,
                                       size_t *out_len, const EC_SCALAR *in);

// ec_scalar_from_bytes deserializes |in| and stores the resulting scalar over
// group |group| to |out|. It returns one on success and zero if |in| is
// invalid.
OPENSSL_EXPORT int ec_scalar_from_bytes(const EC_GROUP *group, EC_SCALAR *out,
                                        const uint8_t *in, size_t len);

// ec_scalar_reduce sets |out| to |words|, reduced modulo the group order.
// |words| must be less than order^2. |num| must be at most twice the width of
// group order. This function treats |words| as secret.
void ec_scalar_reduce(const EC_GROUP *group, EC_SCALAR *out,
                      const BN_ULONG *words, size_t num);

// ec_random_nonzero_scalar sets |out| to a uniformly selected random value from
// zero to |group->order| - 1. It returns one on success and zero on error.
int ec_random_scalar(const EC_GROUP *group, EC_SCALAR *out,
                     const uint8_t additional_data[32]);

// ec_random_nonzero_scalar sets |out| to a uniformly selected random value from
// 1 to |group->order| - 1. It returns one on success and zero on error.
int ec_random_nonzero_scalar(const EC_GROUP *group, EC_SCALAR *out,
                             const uint8_t additional_data[32]);

// ec_scalar_equal_vartime returns one if |a| and |b| are equal and zero
// otherwise. Both values are treated as public.
int ec_scalar_equal_vartime(const EC_GROUP *group, const EC_SCALAR *a,
                            const EC_SCALAR *b);

// ec_scalar_is_zero returns one if |a| is zero and zero otherwise.
int ec_scalar_is_zero(const EC_GROUP *group, const EC_SCALAR *a);

// ec_scalar_add sets |r| to |a| + |b|.
void ec_scalar_add(const EC_GROUP *group, EC_SCALAR *r, const EC_SCALAR *a,
                   const EC_SCALAR *b);

// ec_scalar_sub sets |r| to |a| - |b|.
void ec_scalar_sub(const EC_GROUP *group, EC_SCALAR *r, const EC_SCALAR *a,
                   const EC_SCALAR *b);

// ec_scalar_neg sets |r| to -|a|.
void ec_scalar_neg(const EC_GROUP *group, EC_SCALAR *r, const EC_SCALAR *a);

// ec_scalar_to_montgomery sets |r| to |a| in Montgomery form.
void ec_scalar_to_montgomery(const EC_GROUP *group, EC_SCALAR *r,
                             const EC_SCALAR *a);

// ec_scalar_to_montgomery sets |r| to |a| converted from Montgomery form.
void ec_scalar_from_montgomery(const EC_GROUP *group, EC_SCALAR *r,
                               const EC_SCALAR *a);

// ec_scalar_mul_montgomery sets |r| to |a| * |b| where inputs and outputs are
// in Montgomery form.
void ec_scalar_mul_montgomery(const EC_GROUP *group, EC_SCALAR *r,
                              const EC_SCALAR *a, const EC_SCALAR *b);

// ec_scalar_inv0_montgomery sets |r| to |a|^-1 where inputs and outputs are in
// Montgomery form. If |a| is zero, |r| is set to zero.
void ec_scalar_inv0_montgomery(const EC_GROUP *group, EC_SCALAR *r,
                               const EC_SCALAR *a);

// ec_scalar_to_montgomery_inv_vartime sets |r| to |a|^-1 R. That is, it takes
// in |a| not in Montgomery form and computes the inverse in Montgomery form. It
// returns one on success and zero if |a| has no inverse. This function assumes
// |a| is public and may leak information about it via timing.
//
// Note this is not the same operation as |ec_scalar_inv0_montgomery|.
int ec_scalar_to_montgomery_inv_vartime(const EC_GROUP *group, EC_SCALAR *r,
                                        const EC_SCALAR *a);

// ec_scalar_select, in constant time, sets |out| to |a| if |mask| is all ones
// and |b| if |mask| is all zeros.
void ec_scalar_select(const EC_GROUP *group, EC_SCALAR *out, BN_ULONG mask,
                      const EC_SCALAR *a, const EC_SCALAR *b);


// Field elements.

// An EC_FELEM represents a field element. Only the first |field->width| words
// are used. An |EC_FELEM| is specific to an |EC_GROUP| and must not be mixed
// between groups. Additionally, the representation (whether or not elements are
// represented in Montgomery-form) may vary between |EC_METHOD|s.
typedef struct {
  BN_ULONG words[EC_MAX_WORDS];
} EC_FELEM;

// ec_felem_one returns one in |group|'s field.
const EC_FELEM *ec_felem_one(const EC_GROUP *group);

// ec_bignum_to_felem converts |in| to an |EC_FELEM|. It returns one on success
// and zero if |in| is out of range.
int ec_bignum_to_felem(const EC_GROUP *group, EC_FELEM *out, const BIGNUM *in);

// ec_felem_to_bignum converts |in| to a |BIGNUM|. It returns one on success and
// zero on allocation failure.
int ec_felem_to_bignum(const EC_GROUP *group, BIGNUM *out, const EC_FELEM *in);

// ec_felem_to_bytes serializes |in| as a big-endian bytestring to |out| and
// sets |*out_len| to the number of bytes written. The number of bytes written
// is |BN_num_bytes(&group->order)|, which is at most |EC_MAX_BYTES|.
void ec_felem_to_bytes(const EC_GROUP *group, uint8_t *out, size_t *out_len,
                       const EC_FELEM *in);

// ec_felem_from_bytes deserializes |in| and stores the resulting field element
// to |out|. It returns one on success and zero if |in| is invalid.
int ec_felem_from_bytes(const EC_GROUP *group, EC_FELEM *out, const uint8_t *in,
                        size_t len);

// ec_felem_neg sets |out| to -|a|.
void ec_felem_neg(const EC_GROUP *group, EC_FELEM *out, const EC_FELEM *a);

// ec_felem_add sets |out| to |a| + |b|.
void ec_felem_add(const EC_GROUP *group, EC_FELEM *out, const EC_FELEM *a,
                  const EC_FELEM *b);

// ec_felem_add sets |out| to |a| - |b|.
void ec_felem_sub(const EC_GROUP *group, EC_FELEM *out, const EC_FELEM *a,
                  const EC_FELEM *b);

// ec_felem_non_zero_mask returns all ones if |a| is non-zero and all zeros
// otherwise.
BN_ULONG ec_felem_non_zero_mask(const EC_GROUP *group, const EC_FELEM *a);

// ec_felem_select, in constant time, sets |out| to |a| if |mask| is all ones
// and |b| if |mask| is all zeros.
void ec_felem_select(const EC_GROUP *group, EC_FELEM *out, BN_ULONG mask,
                     const EC_FELEM *a, const EC_FELEM *b);

// ec_felem_equal returns one if |a| and |b| are equal and zero otherwise.
int ec_felem_equal(const EC_GROUP *group, const EC_FELEM *a, const EC_FELEM *b);


// Points.
//
// Points may represented in affine coordinates as |EC_AFFINE| or Jacobian
// coordinates as |EC_JACOBIAN|. Affine coordinates directly represent a
// point on the curve, but point addition over affine coordinates requires
// costly field inversions, so arithmetic is done in Jacobian coordinates.
// Converting from affine to Jacobian is cheap, while converting from Jacobian
// to affine costs a field inversion. (Jacobian coordinates amortize the field
// inversions needed in a sequence of point operations.)

// An EC_JACOBIAN represents an elliptic curve point in Jacobian coordinates.
// Unlike |EC_POINT|, it is a plain struct which can be stack-allocated and
// needs no cleanup. It is specific to an |EC_GROUP| and must not be mixed
// between groups.
typedef struct {
  // X, Y, and Z are Jacobian projective coordinates. They represent
  // (X/Z^2, Y/Z^3) if Z != 0 and the point at infinity otherwise.
  EC_FELEM X, Y, Z;
} EC_JACOBIAN;

// An EC_AFFINE represents an elliptic curve point in affine coordinates.
// coordinates. Note the point at infinity cannot be represented in affine
// coordinates.
typedef struct {
  EC_FELEM X, Y;
} EC_AFFINE;

// ec_affine_to_jacobian converts |p| to Jacobian form and writes the result to
// |*out|. This operation is very cheap and only costs a few copies.
void ec_affine_to_jacobian(const EC_GROUP *group, EC_JACOBIAN *out,
                           const EC_AFFINE *p);

// ec_jacobian_to_affine converts |p| to affine form and writes the result to
// |*out|. It returns one on success and zero if |p| was the point at infinity.
// This operation performs a field inversion and should only be done once per
// point.
//
// If only extracting the x-coordinate, use |ec_get_x_coordinate_*| which is
// slightly faster.
OPENSSL_EXPORT int ec_jacobian_to_affine(const EC_GROUP *group, EC_AFFINE *out,
                                         const EC_JACOBIAN *p);

// ec_jacobian_to_affine_batch converts |num| points in |in| from Jacobian
// coordinates to affine coordinates and writes the results to |out|. It returns
// one on success and zero if any of the input points were infinity.
//
// This function is not implemented for all curves. Add implementations as
// needed.
int ec_jacobian_to_affine_batch(const EC_GROUP *group, EC_AFFINE *out,
                                const EC_JACOBIAN *in, size_t num);

// ec_point_set_affine_coordinates sets |out|'s to a point with affine
// coordinates |x| and |y|. It returns one if the point is on the curve and
// zero otherwise. If the point is not on the curve, the value of |out| is
// undefined.
int ec_point_set_affine_coordinates(const EC_GROUP *group, EC_AFFINE *out,
                                    const EC_FELEM *x, const EC_FELEM *y);

// ec_point_mul_no_self_test does the same as |EC_POINT_mul|, but doesn't try to
// run the self-test first. This is for use in the self tests themselves, to
// prevent an infinite loop.
int ec_point_mul_no_self_test(const EC_GROUP *group, EC_POINT *r,
                              const BIGNUM *g_scalar, const EC_POINT *p,
                              const BIGNUM *p_scalar, BN_CTX *ctx);

// ec_point_mul_scalar sets |r| to |p| * |scalar|. Both inputs are considered
// secret.
int ec_point_mul_scalar(const EC_GROUP *group, EC_JACOBIAN *r,
                        const EC_JACOBIAN *p, const EC_SCALAR *scalar);

// ec_point_mul_scalar_base sets |r| to generator * |scalar|. |scalar| is
// treated as secret.
int ec_point_mul_scalar_base(const EC_GROUP *group, EC_JACOBIAN *r,
                             const EC_SCALAR *scalar);

// ec_point_mul_scalar_batch sets |r| to |p0| * |scalar0| + |p1| * |scalar1| +
// |p2| * |scalar2|. |p2| may be NULL to skip that term.
//
// The inputs are treated as secret, however, this function leaks information
// about whether intermediate computations add a point to itself. Callers must
// ensure that discrete logs between |p0|, |p1|, and |p2| are uniformly
// distributed and independent of the scalars, which should be uniformly
// selected and not under the attackers control. This ensures the doubling case
// will occur with negligible probability.
//
// This function is not implemented for all curves. Add implementations as
// needed.
//
// TODO(davidben): This function does not use base point tables. For now, it is
// only used with the generic |EC_GFp_mont_method| implementation which has
// none. If generalizing to tuned curves, this may be useful. However, we still
// must double up to the least efficient input, so precomputed tables can only
// save table setup and allow a wider window size.
int ec_point_mul_scalar_batch(const EC_GROUP *group, EC_JACOBIAN *r,
                              const EC_JACOBIAN *p0, const EC_SCALAR *scalar0,
                              const EC_JACOBIAN *p1, const EC_SCALAR *scalar1,
                              const EC_JACOBIAN *p2, const EC_SCALAR *scalar2);

#define EC_MONT_PRECOMP_COMB_SIZE 5

// An |EC_PRECOMP| stores precomputed information about a point, to optimize
// repeated multiplications involving it. It is a union so different
// |EC_METHOD|s can store different information in it.
typedef union {
  EC_AFFINE comb[(1 << EC_MONT_PRECOMP_COMB_SIZE) - 1];
} EC_PRECOMP;

// ec_init_precomp precomputes multiples of |p| and writes the result to |out|.
// It returns one on success and zero on error. The resulting table may be used
// with |ec_point_mul_scalar_precomp|. This function will fail if |p| is the
// point at infinity.
//
// This function is not implemented for all curves. Add implementations as
// needed.
int ec_init_precomp(const EC_GROUP *group, EC_PRECOMP *out,
                    const EC_JACOBIAN *p);

// ec_point_mul_scalar_precomp sets |r| to |p0| * |scalar0| + |p1| * |scalar1| +
// |p2| * |scalar2|. |p1| or |p2| may be NULL to skip the corresponding term.
// The points are represented as |EC_PRECOMP| and must be initialized with
// |ec_init_precomp|. This function runs faster than |ec_point_mul_scalar_batch|
// but requires setup work per input point, so it is only appropriate for points
// which are used frequently.
//
// The inputs are treated as secret, however, this function leaks information
// about whether intermediate computations add a point to itself. Callers must
// ensure that discrete logs between |p0|, |p1|, and |p2| are uniformly
// distributed and independent of the scalars, which should be uniformly
// selected and not under the attackers control. This ensures the doubling case
// will occur with negligible probability.
//
// This function is not implemented for all curves. Add implementations as
// needed.
//
// TODO(davidben): This function does not use base point tables. For now, it is
// only used with the generic |EC_GFp_mont_method| implementation which has
// none. If generalizing to tuned curves, we should add a parameter for the base
// point and arrange for the generic implementation to have base point tables
// available.
int ec_point_mul_scalar_precomp(const EC_GROUP *group, EC_JACOBIAN *r,
                                const EC_PRECOMP *p0, const EC_SCALAR *scalar0,
                                const EC_PRECOMP *p1, const EC_SCALAR *scalar1,
                                const EC_PRECOMP *p2, const EC_SCALAR *scalar2);

// ec_point_mul_scalar_public sets |r| to
// generator * |g_scalar| + |p| * |p_scalar|. It assumes that the inputs are
// public so there is no concern about leaking their values through timing.
OPENSSL_EXPORT int ec_point_mul_scalar_public(const EC_GROUP *group,
                                              EC_JACOBIAN *r,
                                              const EC_SCALAR *g_scalar,
                                              const EC_JACOBIAN *p,
                                              const EC_SCALAR *p_scalar);

// ec_point_mul_scalar_public_batch sets |r| to the sum of generator *
// |g_scalar| and |points[i]| * |scalars[i]| where |points| and |scalars| have
// |num| elements. It assumes that the inputs are public so there is no concern
// about leaking their values through timing. |g_scalar| may be NULL to skip
// that term.
//
// This function is not implemented for all curves. Add implementations as
// needed.
int ec_point_mul_scalar_public_batch(const EC_GROUP *group, EC_JACOBIAN *r,
                                     const EC_SCALAR *g_scalar,
                                     const EC_JACOBIAN *points,
                                     const EC_SCALAR *scalars, size_t num);

// ec_point_select, in constant time, sets |out| to |a| if |mask| is all ones
// and |b| if |mask| is all zeros.
void ec_point_select(const EC_GROUP *group, EC_JACOBIAN *out, BN_ULONG mask,
                     const EC_JACOBIAN *a, const EC_JACOBIAN *b);

// ec_affine_select behaves like |ec_point_select| but acts on affine points.
void ec_affine_select(const EC_GROUP *group, EC_AFFINE *out, BN_ULONG mask,
                      const EC_AFFINE *a, const EC_AFFINE *b);

// ec_precomp_select behaves like |ec_point_select| but acts on |EC_PRECOMP|.
void ec_precomp_select(const EC_GROUP *group, EC_PRECOMP *out, BN_ULONG mask,
                       const EC_PRECOMP *a, const EC_PRECOMP *b);

// ec_cmp_x_coordinate compares the x (affine) coordinate of |p|, mod the group
// order, with |r|. It returns one if the values match and zero if |p| is the
// point at infinity of the values do not match. |p| is treated as public.
int ec_cmp_x_coordinate(const EC_GROUP *group, const EC_JACOBIAN *p,
                        const EC_SCALAR *r);

// ec_get_x_coordinate_as_scalar sets |*out| to |p|'s x-coordinate, modulo
// |group->order|. It returns one on success and zero if |p| is the point at
// infinity.
int ec_get_x_coordinate_as_scalar(const EC_GROUP *group, EC_SCALAR *out,
                                  const EC_JACOBIAN *p);

// ec_get_x_coordinate_as_bytes writes |p|'s affine x-coordinate to |out|, which
// must have at must |max_out| bytes. It sets |*out_len| to the number of bytes
// written. The value is written big-endian and zero-padded to the size of the
// field. This function returns one on success and zero on failure.
int ec_get_x_coordinate_as_bytes(const EC_GROUP *group, uint8_t *out,
                                 size_t *out_len, size_t max_out,
                                 const EC_JACOBIAN *p);

// ec_point_byte_len returns the number of bytes in the byte representation of
// a non-infinity point in |group|, encoded according to |form|, or zero if
// |form| is invalid.
size_t ec_point_byte_len(const EC_GROUP *group, point_conversion_form_t form);

// ec_point_to_bytes encodes |point| according to |form| and writes the result
// |buf|. It returns the size of the output on success or zero on error. At most
// |max_out| bytes will be written. The buffer should be at least
// |ec_point_byte_len| long to guarantee success.
size_t ec_point_to_bytes(const EC_GROUP *group, const EC_AFFINE *point,
                         point_conversion_form_t form, uint8_t *buf,
                         size_t max_out);

// ec_point_from_uncompressed parses |in| as a point in uncompressed form and
// sets the result to |out|. It returns one on success and zero if the input was
// invalid.
int ec_point_from_uncompressed(const EC_GROUP *group, EC_AFFINE *out,
                               const uint8_t *in, size_t len);

// ec_set_to_safe_point sets |out| to an arbitrary point on |group|, either the
// generator or the point at infinity. This is used to guard against callers of
// external APIs not checking the return value.
void ec_set_to_safe_point(const EC_GROUP *group, EC_JACOBIAN *out);

// ec_affine_jacobian_equal returns one if |a| and |b| represent the same point
// and zero otherwise. It treats both inputs as secret.
int ec_affine_jacobian_equal(const EC_GROUP *group, const EC_AFFINE *a,
                             const EC_JACOBIAN *b);


// Implementation details.

struct ec_method_st {
  // point_get_affine_coordinates sets |*x| and |*y| to the affine coordinates
  // of |p|. Either |x| or |y| may be NULL to omit it. It returns one on success
  // and zero if |p| is the point at infinity. It leaks whether |p| was the
  // point at infinity, but otherwise treats |p| as secret.
  int (*point_get_affine_coordinates)(const EC_GROUP *, const EC_JACOBIAN *p,
                                      EC_FELEM *x, EC_FELEM *y);

  // jacobian_to_affine_batch implements |ec_jacobian_to_affine_batch|.
  int (*jacobian_to_affine_batch)(const EC_GROUP *group, EC_AFFINE *out,
                                  const EC_JACOBIAN *in, size_t num);

  // add sets |r| to |a| + |b|.
  void (*add)(const EC_GROUP *group, EC_JACOBIAN *r, const EC_JACOBIAN *a,
              const EC_JACOBIAN *b);
  // dbl sets |r| to |a| + |a|.
  void (*dbl)(const EC_GROUP *group, EC_JACOBIAN *r, const EC_JACOBIAN *a);

  // mul sets |r| to |scalar|*|p|.
  void (*mul)(const EC_GROUP *group, EC_JACOBIAN *r, const EC_JACOBIAN *p,
              const EC_SCALAR *scalar);
  // mul_base sets |r| to |scalar|*generator.
  void (*mul_base)(const EC_GROUP *group, EC_JACOBIAN *r,
                   const EC_SCALAR *scalar);
  // mul_batch implements |ec_mul_scalar_batch|.
  void (*mul_batch)(const EC_GROUP *group, EC_JACOBIAN *r,
                    const EC_JACOBIAN *p0, const EC_SCALAR *scalar0,
                    const EC_JACOBIAN *p1, const EC_SCALAR *scalar1,
                    const EC_JACOBIAN *p2, const EC_SCALAR *scalar2);
  // mul_public sets |r| to |g_scalar|*generator + |p_scalar|*|p|. It assumes
  // that the inputs are public so there is no concern about leaking their
  // values through timing.
  //
  // This function may be omitted if |mul_public_batch| is provided.
  void (*mul_public)(const EC_GROUP *group, EC_JACOBIAN *r,
                     const EC_SCALAR *g_scalar, const EC_JACOBIAN *p,
                     const EC_SCALAR *p_scalar);
  // mul_public_batch implements |ec_point_mul_scalar_public_batch|.
  int (*mul_public_batch)(const EC_GROUP *group, EC_JACOBIAN *r,
                          const EC_SCALAR *g_scalar, const EC_JACOBIAN *points,
                          const EC_SCALAR *scalars, size_t num);

  // init_precomp implements |ec_init_precomp|.
  int (*init_precomp)(const EC_GROUP *group, EC_PRECOMP *out,
                      const EC_JACOBIAN *p);
  // mul_precomp implements |ec_point_mul_scalar_precomp|.
  void (*mul_precomp)(const EC_GROUP *group, EC_JACOBIAN *r,
                      const EC_PRECOMP *p0, const EC_SCALAR *scalar0,
                      const EC_PRECOMP *p1, const EC_SCALAR *scalar1,
                      const EC_PRECOMP *p2, const EC_SCALAR *scalar2);

  // felem_mul and felem_sqr implement multiplication and squaring,
  // respectively, so that the generic |EC_POINT_add| and |EC_POINT_dbl|
  // implementations can work both with |EC_GFp_mont_method| and the tuned
  // operations.
  //
  // TODO(davidben): This constrains |EC_FELEM|'s internal representation, adds
  // many indirect calls in the middle of the generic code, and a bunch of
  // conversions. If p224-64.c were easily convertable to Montgomery form, we
  // could say |EC_FELEM| is always in Montgomery form. If we routed the rest of
  // simple.c to |EC_METHOD|, we could give |EC_POINT| an |EC_METHOD|-specific
  // representation and say |EC_FELEM| is purely a |EC_GFp_mont_method| type.
  void (*felem_mul)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a,
                    const EC_FELEM *b);
  void (*felem_sqr)(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a);

  void (*felem_to_bytes)(const EC_GROUP *group, uint8_t *out, size_t *out_len,
                         const EC_FELEM *in);
  int (*felem_from_bytes)(const EC_GROUP *group, EC_FELEM *out,
                          const uint8_t *in, size_t len);

  // felem_reduce sets |out| to |words|, reduced modulo the field size, p.
  // |words| must be less than p^2. |num| must be at most twice the width of p.
  // This function treats |words| as secret.
  //
  // This function is only used in hash-to-curve and may be omitted in curves
  // that do not support it.
  void (*felem_reduce)(const EC_GROUP *group, EC_FELEM *out,
                       const BN_ULONG *words, size_t num);

  // felem_exp sets |out| to |a|^|exp|. It treats |a| is secret but |exp| as
  // public.
  //
  // This function is used in hash-to-curve and may be NULL in curves not used
  // with hash-to-curve.
  //
  // TODO(https://crbug.com/boringssl/567): hash-to-curve uses this as part of
  // computing a square root, which is what compressed coordinates ultimately
  // needs to avoid |BIGNUM|. Can we unify this a bit? By generalizing to
  // arbitrary exponentiation, we also miss an opportunity to use a specialized
  // addition chain.
  void (*felem_exp)(const EC_GROUP *group, EC_FELEM *out, const EC_FELEM *a,
                    const BN_ULONG *exp, size_t num_exp);

  // scalar_inv0_montgomery implements |ec_scalar_inv0_montgomery|.
  void (*scalar_inv0_montgomery)(const EC_GROUP *group, EC_SCALAR *out,
                                 const EC_SCALAR *in);

  // scalar_to_montgomery_inv_vartime implements
  // |ec_scalar_to_montgomery_inv_vartime|.
  int (*scalar_to_montgomery_inv_vartime)(const EC_GROUP *group, EC_SCALAR *out,
                                          const EC_SCALAR *in);

  // cmp_x_coordinate compares the x (affine) coordinate of |p|, mod the group
  // order, with |r|. It returns one if the values match and zero if |p| is the
  // point at infinity of the values do not match.
  int (*cmp_x_coordinate)(const EC_GROUP *group, const EC_JACOBIAN *p,
                          const EC_SCALAR *r);
} /* EC_METHOD */;

const EC_METHOD *EC_GFp_mont_method(void);

struct ec_point_st {
  // group is an owning reference to |group|, unless this is
  // |group->generator|.
  EC_GROUP *group;
  // raw is the group-specific point data. Functions that take |EC_POINT|
  // typically check consistency with |EC_GROUP| while functions that take
  // |EC_JACOBIAN| do not. Thus accesses to this field should be externally
  // checked for consistency.
  EC_JACOBIAN raw;
} /* EC_POINT */;

struct ec_group_st {
  const EC_METHOD *meth;

  // Unlike all other |EC_POINT|s, |generator| does not own |generator->group|
  // to avoid a reference cycle. Additionally, Z is guaranteed to be one, so X
  // and Y are suitable for use as an |EC_AFFINE|. Before |has_order| is set, Z
  // is one, but X and Y are uninitialized.
  EC_POINT generator;

  BN_MONT_CTX order;
  BN_MONT_CTX field;

  EC_FELEM a, b;  // Curve coefficients.

  // comment is a human-readable string describing the curve.
  const char *comment;

  int curve_name;  // optional NID for named curve
  uint8_t oid[9];
  uint8_t oid_len;

  // a_is_minus3 is one if |a| is -3 mod |field| and zero otherwise. Point
  // arithmetic is optimized for -3.
  int a_is_minus3;

  // has_order is one if |generator| and |order| have been initialized.
  int has_order;

  // field_greater_than_order is one if |field| is greate than |order| and zero
  // otherwise.
  int field_greater_than_order;

  CRYPTO_refcount_t references;
} /* EC_GROUP */;

EC_GROUP *ec_group_new(const EC_METHOD *meth, const BIGNUM *p, const BIGNUM *a,
                       const BIGNUM *b, BN_CTX *ctx);

void ec_GFp_mont_mul(const EC_GROUP *group, EC_JACOBIAN *r,
                     const EC_JACOBIAN *p, const EC_SCALAR *scalar);
void ec_GFp_mont_mul_base(const EC_GROUP *group, EC_JACOBIAN *r,
                          const EC_SCALAR *scalar);
void ec_GFp_mont_mul_batch(const EC_GROUP *group, EC_JACOBIAN *r,
                           const EC_JACOBIAN *p0, const EC_SCALAR *scalar0,
                           const EC_JACOBIAN *p1, const EC_SCALAR *scalar1,
                           const EC_JACOBIAN *p2, const EC_SCALAR *scalar2);
int ec_GFp_mont_init_precomp(const EC_GROUP *group, EC_PRECOMP *out,
                             const EC_JACOBIAN *p);
void ec_GFp_mont_mul_precomp(const EC_GROUP *group, EC_JACOBIAN *r,
                             const EC_PRECOMP *p0, const EC_SCALAR *scalar0,
                             const EC_PRECOMP *p1, const EC_SCALAR *scalar1,
                             const EC_PRECOMP *p2, const EC_SCALAR *scalar2);
void ec_GFp_mont_felem_reduce(const EC_GROUP *group, EC_FELEM *out,
                              const BN_ULONG *words, size_t num);
void ec_GFp_mont_felem_exp(const EC_GROUP *group, EC_FELEM *out,
                           const EC_FELEM *a, const BN_ULONG *exp,
                           size_t num_exp);

// ec_compute_wNAF writes the modified width-(w+1) Non-Adjacent Form (wNAF) of
// |scalar| to |out|. |out| must have room for |bits| + 1 elements, each of
// which will be either zero or odd with an absolute value less than  2^w
// satisfying
//     scalar = \sum_j out[j]*2^j
// where at most one of any  w+1  consecutive digits is non-zero
// with the exception that the most significant digit may be only
// w-1 zeros away from that next non-zero digit.
void ec_compute_wNAF(const EC_GROUP *group, int8_t *out,
                     const EC_SCALAR *scalar, size_t bits, int w);

int ec_GFp_mont_mul_public_batch(const EC_GROUP *group, EC_JACOBIAN *r,
                                 const EC_SCALAR *g_scalar,
                                 const EC_JACOBIAN *points,
                                 const EC_SCALAR *scalars, size_t num);

// method functions in simple.c
int ec_GFp_simple_group_set_curve(EC_GROUP *, const BIGNUM *p, const BIGNUM *a,
                                  const BIGNUM *b, BN_CTX *);
int ec_GFp_simple_group_get_curve(const EC_GROUP *, BIGNUM *p, BIGNUM *a,
                                  BIGNUM *b);
void ec_GFp_simple_point_init(EC_JACOBIAN *);
void ec_GFp_simple_point_copy(EC_JACOBIAN *, const EC_JACOBIAN *);
void ec_GFp_simple_point_set_to_infinity(const EC_GROUP *, EC_JACOBIAN *);
void ec_GFp_mont_add(const EC_GROUP *, EC_JACOBIAN *r, const EC_JACOBIAN *a,
                     const EC_JACOBIAN *b);
void ec_GFp_mont_dbl(const EC_GROUP *, EC_JACOBIAN *r, const EC_JACOBIAN *a);
void ec_GFp_simple_invert(const EC_GROUP *, EC_JACOBIAN *);
int ec_GFp_simple_is_at_infinity(const EC_GROUP *, const EC_JACOBIAN *);
int ec_GFp_simple_is_on_curve(const EC_GROUP *, const EC_JACOBIAN *);
int ec_GFp_simple_points_equal(const EC_GROUP *, const EC_JACOBIAN *a,
                               const EC_JACOBIAN *b);
void ec_simple_scalar_inv0_montgomery(const EC_GROUP *group, EC_SCALAR *r,
                                      const EC_SCALAR *a);

int ec_simple_scalar_to_montgomery_inv_vartime(const EC_GROUP *group,
                                               EC_SCALAR *r,
                                               const EC_SCALAR *a);

int ec_GFp_simple_cmp_x_coordinate(const EC_GROUP *group, const EC_JACOBIAN *p,
                                   const EC_SCALAR *r);

void ec_GFp_simple_felem_to_bytes(const EC_GROUP *group, uint8_t *out,
                                  size_t *out_len, const EC_FELEM *in);
int ec_GFp_simple_felem_from_bytes(const EC_GROUP *group, EC_FELEM *out,
                                   const uint8_t *in, size_t len);

// method functions in montgomery.c
void ec_GFp_mont_felem_mul(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a,
                           const EC_FELEM *b);
void ec_GFp_mont_felem_sqr(const EC_GROUP *, EC_FELEM *r, const EC_FELEM *a);

void ec_GFp_mont_felem_to_bytes(const EC_GROUP *group, uint8_t *out,
                                size_t *out_len, const EC_FELEM *in);
int ec_GFp_mont_felem_from_bytes(const EC_GROUP *group, EC_FELEM *out,
                                 const uint8_t *in, size_t len);

void ec_GFp_nistp_recode_scalar_bits(crypto_word_t *sign, crypto_word_t *digit,
                                     crypto_word_t in);

const EC_METHOD *EC_GFp_nistp224_method(void);
const EC_METHOD *EC_GFp_nistp256_method(void);

// EC_GFp_nistz256_method is a GFp method using montgomery multiplication, with
// x86-64 optimized P256. See http://eprint.iacr.org/2013/816.
const EC_METHOD *EC_GFp_nistz256_method(void);

// An EC_WRAPPED_SCALAR is an |EC_SCALAR| with a parallel |BIGNUM|
// representation. It exists to support the |EC_KEY_get0_private_key| API.
typedef struct {
  BIGNUM bignum;
  EC_SCALAR scalar;
} EC_WRAPPED_SCALAR;

struct ec_key_st {
  EC_GROUP *group;

  // Ideally |pub_key| would be an |EC_AFFINE| so serializing it does not pay an
  // inversion each time, but the |EC_KEY_get0_public_key| API implies public
  // keys are stored in an |EC_POINT|-compatible form.
  EC_POINT *pub_key;
  EC_WRAPPED_SCALAR *priv_key;

  unsigned int enc_flag;
  point_conversion_form_t conv_form;

  CRYPTO_refcount_t references;

  ECDSA_METHOD *ecdsa_meth;

  CRYPTO_EX_DATA ex_data;
} /* EC_KEY */;


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_EC_INTERNAL_H
