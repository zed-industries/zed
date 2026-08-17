// Copyright 2000-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_EC_H
#define OPENSSL_HEADER_EC_H

#include <openssl/base.h>   // IWYU pragma: export

#if defined(__cplusplus)
extern "C" {
#endif


// Low-level operations on elliptic curves.


// point_conversion_form_t enumerates forms, as defined in X9.62 (ECDSA), for
// the encoding of a elliptic curve point (x,y)
typedef enum {
  // POINT_CONVERSION_COMPRESSED indicates that the point is encoded as z||x,
  // where the octet z specifies which solution of the quadratic equation y
  // is.
  POINT_CONVERSION_COMPRESSED = 2,

  // POINT_CONVERSION_UNCOMPRESSED indicates that the point is encoded as
  // z||x||y, where z is the octet 0x04.
  POINT_CONVERSION_UNCOMPRESSED = 4,

  // POINT_CONVERSION_HYBRID indicates that the point is encoded as z||x||y,
  // where z specifies which solution of the quadratic equation y is. This is
  // not supported by the code and has never been observed in use.
  //
  // TODO(agl): remove once node.js no longer references this.
  POINT_CONVERSION_HYBRID = 6,
} point_conversion_form_t;


// Elliptic curve groups.
//
// Elliptic curve groups are represented by |EC_GROUP| objects. Unlike OpenSSL,
// if limited to the APIs in this section, callers may treat |EC_GROUP|s as
// static, immutable objects which do not need to be copied or released. In
// BoringSSL, only custom |EC_GROUP|s created by |EC_GROUP_new_curve_GFp|
// (deprecated) are dynamic.
//
// Callers may cast away |const| and use |EC_GROUP_dup| and |EC_GROUP_free| with
// static groups, for compatibility with OpenSSL or dynamic groups, but it is
// otherwise unnecessary.

// EC_group_p224 returns an |EC_GROUP| for P-224, also known as secp224r1.
OPENSSL_EXPORT const EC_GROUP *EC_group_p224(void);

// EC_group_p256 returns an |EC_GROUP| for P-256, also known as secp256r1 or
// prime256v1.
OPENSSL_EXPORT const EC_GROUP *EC_group_p256(void);

// EC_group_p384 returns an |EC_GROUP| for P-384, also known as secp384r1.
OPENSSL_EXPORT const EC_GROUP *EC_group_p384(void);

// EC_group_p521 returns an |EC_GROUP| for P-521, also known as secp521r1.
OPENSSL_EXPORT const EC_GROUP *EC_group_p521(void);

// EC_GROUP_new_by_curve_name returns the |EC_GROUP| object for the elliptic
// curve specified by |nid|, or NULL on unsupported NID.  For OpenSSL
// compatibility, this function returns a non-const pointer which may be passed
// to |EC_GROUP_free|. However, the resulting object is actually static and
// calling |EC_GROUP_free| is optional.
//
// The supported NIDs are:
// - |NID_secp224r1| (P-224)
// - |NID_X9_62_prime256v1| (P-256)
// - |NID_secp384r1| (P-384)
// - |NID_secp521r1| (P-521)
//
// Calling this function causes all four curves to be linked into the binary.
// Prefer calling |EC_group_*| to allow the static linker to drop unused curves.
//
// If in doubt, use |NID_X9_62_prime256v1|, or see the curve25519.h header for
// more modern primitives.
OPENSSL_EXPORT EC_GROUP *EC_GROUP_new_by_curve_name(int nid);

// EC_GROUP_cmp returns zero if |a| and |b| are the same group and non-zero
// otherwise.
OPENSSL_EXPORT int EC_GROUP_cmp(const EC_GROUP *a, const EC_GROUP *b,
                                BN_CTX *ignored);

// EC_GROUP_get0_generator returns a pointer to the internal |EC_POINT| object
// in |group| that specifies the generator for the group.
OPENSSL_EXPORT const EC_POINT *EC_GROUP_get0_generator(const EC_GROUP *group);

// EC_GROUP_get0_order returns a pointer to the internal |BIGNUM| object in
// |group| that specifies the order of the group.
OPENSSL_EXPORT const BIGNUM *EC_GROUP_get0_order(const EC_GROUP *group);

// EC_GROUP_order_bits returns the number of bits of the order of |group|.
OPENSSL_EXPORT int EC_GROUP_order_bits(const EC_GROUP *group);

// EC_GROUP_get_cofactor sets |*cofactor| to the cofactor of |group| using
// |ctx|, if it's not NULL. It returns one on success and zero otherwise.
OPENSSL_EXPORT int EC_GROUP_get_cofactor(const EC_GROUP *group,
                                         BIGNUM *cofactor, BN_CTX *ctx);

// EC_GROUP_get_curve_GFp gets various parameters about a group. It sets
// |*out_p| to the order of the coordinate field and |*out_a| and |*out_b| to
// the parameters of the curve when expressed as y² = x³ + ax + b. Any of the
// output parameters can be NULL. It returns one on success and zero on
// error.
OPENSSL_EXPORT int EC_GROUP_get_curve_GFp(const EC_GROUP *group, BIGNUM *out_p,
                                          BIGNUM *out_a, BIGNUM *out_b,
                                          BN_CTX *ctx);

// EC_GROUP_get_curve_name returns a NID that identifies |group|.
OPENSSL_EXPORT int EC_GROUP_get_curve_name(const EC_GROUP *group);

// EC_GROUP_get_degree returns the number of bits needed to represent an
// element of the field underlying |group|.
OPENSSL_EXPORT unsigned EC_GROUP_get_degree(const EC_GROUP *group);

// EC_curve_nid2nist returns the NIST name of the elliptic curve specified by
// |nid|, or NULL if |nid| is not a NIST curve. For example, it returns "P-256"
// for |NID_X9_62_prime256v1|.
OPENSSL_EXPORT const char *EC_curve_nid2nist(int nid);

// EC_curve_nist2nid returns the NID of the elliptic curve specified by the NIST
// name |name|, or |NID_undef| if |name| is not a recognized name. For example,
// it returns |NID_X9_62_prime256v1| for "P-256".
OPENSSL_EXPORT int EC_curve_nist2nid(const char *name);


// Points on elliptic curves.

// EC_POINT_new returns a fresh |EC_POINT| object in the given group, or NULL
// on error.
OPENSSL_EXPORT EC_POINT *EC_POINT_new(const EC_GROUP *group);

// EC_POINT_free frees |point| and the data that it points to.
OPENSSL_EXPORT void EC_POINT_free(EC_POINT *point);

// EC_POINT_copy sets |*dest| equal to |*src|. It returns one on success and
// zero otherwise.
OPENSSL_EXPORT int EC_POINT_copy(EC_POINT *dest, const EC_POINT *src);

// EC_POINT_dup returns a fresh |EC_POINT| that contains the same values as
// |src|, or NULL on error.
OPENSSL_EXPORT EC_POINT *EC_POINT_dup(const EC_POINT *src,
                                      const EC_GROUP *group);

// EC_POINT_set_to_infinity sets |point| to be the "point at infinity" for the
// given group.
OPENSSL_EXPORT int EC_POINT_set_to_infinity(const EC_GROUP *group,
                                            EC_POINT *point);

// EC_POINT_is_at_infinity returns one iff |point| is the point at infinity and
// zero otherwise.
OPENSSL_EXPORT int EC_POINT_is_at_infinity(const EC_GROUP *group,
                                           const EC_POINT *point);

// EC_POINT_is_on_curve returns one if |point| is an element of |group| and
// and zero otherwise or when an error occurs. This is different from OpenSSL,
// which returns -1 on error. If |ctx| is non-NULL, it may be used.
OPENSSL_EXPORT int EC_POINT_is_on_curve(const EC_GROUP *group,
                                        const EC_POINT *point, BN_CTX *ctx);

// EC_POINT_cmp returns zero if |a| is equal to |b|, greater than zero if
// not equal and -1 on error. If |ctx| is not NULL, it may be used.
OPENSSL_EXPORT int EC_POINT_cmp(const EC_GROUP *group, const EC_POINT *a,
                                const EC_POINT *b, BN_CTX *ctx);


// Point conversion.

// EC_POINT_get_affine_coordinates_GFp sets |x| and |y| to the affine value of
// |point| using |ctx|, if it's not NULL. It returns one on success and zero
// otherwise.
//
// Either |x| or |y| may be NULL to skip computing that coordinate. This is
// slightly faster in the common case where only the x-coordinate is needed.
OPENSSL_EXPORT int EC_POINT_get_affine_coordinates_GFp(const EC_GROUP *group,
                                                       const EC_POINT *point,
                                                       BIGNUM *x, BIGNUM *y,
                                                       BN_CTX *ctx);

// EC_POINT_get_affine_coordinates is an alias of
// |EC_POINT_get_affine_coordinates_GFp|.
OPENSSL_EXPORT int EC_POINT_get_affine_coordinates(const EC_GROUP *group,
                                                   const EC_POINT *point,
                                                   BIGNUM *x, BIGNUM *y,
                                                   BN_CTX *ctx);

// EC_POINT_set_affine_coordinates_GFp sets the value of |point| to be
// (|x|, |y|). The |ctx| argument may be used if not NULL. It returns one
// on success or zero on error. It's considered an error if the point is not on
// the curve.
//
// Note that the corresponding function in OpenSSL versions prior to 1.0.2s does
// not check if the point is on the curve. This is a security-critical check, so
// code additionally supporting OpenSSL should repeat the check with
// |EC_POINT_is_on_curve| or check for older OpenSSL versions with
// |OPENSSL_VERSION_NUMBER|.
OPENSSL_EXPORT int EC_POINT_set_affine_coordinates_GFp(const EC_GROUP *group,
                                                       EC_POINT *point,
                                                       const BIGNUM *x,
                                                       const BIGNUM *y,
                                                       BN_CTX *ctx);

// EC_POINT_set_affine_coordinates is an alias of
// |EC_POINT_set_affine_coordinates_GFp|.
OPENSSL_EXPORT int EC_POINT_set_affine_coordinates(const EC_GROUP *group,
                                                   EC_POINT *point,
                                                   const BIGNUM *x,
                                                   const BIGNUM *y,
                                                   BN_CTX *ctx);

// EC_POINT_point2oct serialises |point| into the X9.62 form given by |form|
// into, at most, |max_out| bytes at |buf|. It returns the number of bytes
// written or zero on error if |buf| is non-NULL, else the number of bytes
// needed. The |ctx| argument may be used if not NULL.
OPENSSL_EXPORT size_t EC_POINT_point2oct(const EC_GROUP *group,
                                         const EC_POINT *point,
                                         point_conversion_form_t form,
                                         uint8_t *buf, size_t max_out,
                                         BN_CTX *ctx);

// EC_POINT_point2buf serialises |point| into the X9.62 form given by |form| to
// a newly-allocated buffer and sets |*out_buf| to point to it. It returns the
// length of the result on success or zero on error. The caller must release
// |*out_buf| with |OPENSSL_free| when done.
OPENSSL_EXPORT size_t EC_POINT_point2buf(const EC_GROUP *group,
                                         const EC_POINT *point,
                                         point_conversion_form_t form,
                                         uint8_t **out_buf, BN_CTX *ctx);

// EC_POINT_point2cbb behaves like |EC_POINT_point2oct| but appends the
// serialised point to |cbb|. It returns one on success and zero on error.
OPENSSL_EXPORT int EC_POINT_point2cbb(CBB *out, const EC_GROUP *group,
                                      const EC_POINT *point,
                                      point_conversion_form_t form,
                                      BN_CTX *ctx);

// EC_POINT_oct2point sets |point| from |len| bytes of X9.62 format
// serialisation in |buf|. It returns one on success and zero on error. The
// |ctx| argument may be used if not NULL. It's considered an error if |buf|
// does not represent a point on the curve.
OPENSSL_EXPORT int EC_POINT_oct2point(const EC_GROUP *group, EC_POINT *point,
                                      const uint8_t *buf, size_t len,
                                      BN_CTX *ctx);

// EC_POINT_set_compressed_coordinates_GFp sets |point| to equal the point with
// the given |x| coordinate and the y coordinate specified by |y_bit| (see
// X9.62). It returns one on success and zero otherwise.
OPENSSL_EXPORT int EC_POINT_set_compressed_coordinates_GFp(
    const EC_GROUP *group, EC_POINT *point, const BIGNUM *x, int y_bit,
    BN_CTX *ctx);


// Group operations.

// EC_POINT_add sets |r| equal to |a| plus |b|. It returns one on success and
// zero otherwise. If |ctx| is not NULL, it may be used.
OPENSSL_EXPORT int EC_POINT_add(const EC_GROUP *group, EC_POINT *r,
                                const EC_POINT *a, const EC_POINT *b,
                                BN_CTX *ctx);

// EC_POINT_dbl sets |r| equal to |a| plus |a|. It returns one on success and
// zero otherwise. If |ctx| is not NULL, it may be used.
OPENSSL_EXPORT int EC_POINT_dbl(const EC_GROUP *group, EC_POINT *r,
                                const EC_POINT *a, BN_CTX *ctx);

// EC_POINT_invert sets |a| equal to minus |a|. It returns one on success and
// zero otherwise. If |ctx| is not NULL, it may be used.
OPENSSL_EXPORT int EC_POINT_invert(const EC_GROUP *group, EC_POINT *a,
                                   BN_CTX *ctx);

// EC_POINT_mul sets r = generator*n + q*m. It returns one on success and zero
// otherwise. If |ctx| is not NULL, it may be used.
OPENSSL_EXPORT int EC_POINT_mul(const EC_GROUP *group, EC_POINT *r,
                                const BIGNUM *n, const EC_POINT *q,
                                const BIGNUM *m, BN_CTX *ctx);


// Hash-to-curve.
//
// The following functions implement primitives from RFC 9380. The |dst|
// parameter in each function is the domain separation tag and must be unique
// for each protocol and between the |hash_to_curve| and |hash_to_scalar|
// variants. See section 3.1 of the spec for additional guidance on this
// parameter.

// EC_hash_to_curve_p256_xmd_sha256_sswu hashes |msg| to a point on |group| and
// writes the result to |out|, implementing the P256_XMD:SHA-256_SSWU_RO_ suite
// from RFC 9380. It returns one on success and zero on error.
OPENSSL_EXPORT int EC_hash_to_curve_p256_xmd_sha256_sswu(
    const EC_GROUP *group, EC_POINT *out, const uint8_t *dst, size_t dst_len,
    const uint8_t *msg, size_t msg_len);

// EC_hash_to_curve_p384_xmd_sha384_sswu hashes |msg| to a point on |group| and
// writes the result to |out|, implementing the P384_XMD:SHA-384_SSWU_RO_ suite
// from RFC 9380. It returns one on success and zero on error.
OPENSSL_EXPORT int EC_hash_to_curve_p384_xmd_sha384_sswu(
    const EC_GROUP *group, EC_POINT *out, const uint8_t *dst, size_t dst_len,
    const uint8_t *msg, size_t msg_len);


// Deprecated functions.

// EC_GROUP_free releases a reference to |group|, if |group| was created by
// |EC_GROUP_new_curve_GFp|. If |group| is static, it does nothing.
//
// This function exists for OpenSSL compatibilty, and to manage dynamic
// |EC_GROUP|s constructed by |EC_GROUP_new_curve_GFp|. Callers that do not need
// either may ignore this function.
OPENSSL_EXPORT void EC_GROUP_free(EC_GROUP *group);

// EC_GROUP_dup increments |group|'s reference count and returns it, if |group|
// was created by |EC_GROUP_new_curve_GFp|. If |group| is static, it simply
// returns |group|.
//
// This function exists for OpenSSL compatibilty, and to manage dynamic
// |EC_GROUP|s constructed by |EC_GROUP_new_curve_GFp|. Callers that do not need
// either may ignore this function.
OPENSSL_EXPORT EC_GROUP *EC_GROUP_dup(const EC_GROUP *group);

// EC_GROUP_new_curve_GFp creates a new, arbitrary elliptic curve group based
// on the equation y² = x³ + a·x + b. It returns the new group or NULL on
// error. The lifetime of the resulting object must be managed with
// |EC_GROUP_dup| and |EC_GROUP_free|.
//
// This new group has no generator. It is an error to use a generator-less group
// with any functions except for |EC_GROUP_free|, |EC_POINT_new|,
// |EC_POINT_set_affine_coordinates_GFp|, and |EC_GROUP_set_generator|.
//
// |EC_GROUP|s returned by this function will always compare as unequal via
// |EC_GROUP_cmp| (even to themselves). |EC_GROUP_get_curve_name| will always
// return |NID_undef|.
//
// This function is provided for compatibility with some legacy applications
// only. Avoid using arbitrary curves and use |EC_GROUP_new_by_curve_name|
// instead. This ensures the result meets preconditions necessary for
// elliptic curve algorithms to function correctly and securely.
//
// Given invalid parameters, this function may fail or it may return an
// |EC_GROUP| which breaks these preconditions. Subsequent operations may then
// return arbitrary, incorrect values. Callers should not pass
// attacker-controlled values to this function.
OPENSSL_EXPORT EC_GROUP *EC_GROUP_new_curve_GFp(const BIGNUM *p,
                                                const BIGNUM *a,
                                                const BIGNUM *b, BN_CTX *ctx);

// EC_GROUP_set_generator sets the generator for |group| to |generator|, which
// must have the given order and cofactor. It may only be used with |EC_GROUP|
// objects returned by |EC_GROUP_new_curve_GFp| and may only be used once on
// each group. |generator| must have been created using |group|.
OPENSSL_EXPORT int EC_GROUP_set_generator(EC_GROUP *group,
                                          const EC_POINT *generator,
                                          const BIGNUM *order,
                                          const BIGNUM *cofactor);

// EC_GROUP_get_order sets |*order| to the order of |group|, if it's not
// NULL. It returns one on success and zero otherwise. |ctx| is ignored. Use
// |EC_GROUP_get0_order| instead.
OPENSSL_EXPORT int EC_GROUP_get_order(const EC_GROUP *group, BIGNUM *order,
                                      BN_CTX *ctx);

#define OPENSSL_EC_EXPLICIT_CURVE 0
#define OPENSSL_EC_NAMED_CURVE 1

// EC_GROUP_set_asn1_flag does nothing.
OPENSSL_EXPORT void EC_GROUP_set_asn1_flag(EC_GROUP *group, int flag);

// EC_GROUP_get_asn1_flag returns |OPENSSL_EC_NAMED_CURVE|.
OPENSSL_EXPORT int EC_GROUP_get_asn1_flag(const EC_GROUP *group);

typedef struct ec_method_st EC_METHOD;

// EC_GROUP_method_of returns a dummy non-NULL pointer.
OPENSSL_EXPORT const EC_METHOD *EC_GROUP_method_of(const EC_GROUP *group);

// EC_METHOD_get_field_type returns NID_X9_62_prime_field.
OPENSSL_EXPORT int EC_METHOD_get_field_type(const EC_METHOD *meth);

// EC_GROUP_set_point_conversion_form aborts the process if |form| is not
// |POINT_CONVERSION_UNCOMPRESSED| and otherwise does nothing.
OPENSSL_EXPORT void EC_GROUP_set_point_conversion_form(
    EC_GROUP *group, point_conversion_form_t form);

// EC_builtin_curve describes a supported elliptic curve.
typedef struct {
  int nid;
  const char *comment;
} EC_builtin_curve;

// EC_get_builtin_curves writes at most |max_num_curves| elements to
// |out_curves| and returns the total number that it would have written, had
// |max_num_curves| been large enough.
//
// The |EC_builtin_curve| items describe the supported elliptic curves.
OPENSSL_EXPORT size_t EC_get_builtin_curves(EC_builtin_curve *out_curves,
                                            size_t max_num_curves);

// EC_POINT_clear_free calls |EC_POINT_free|.
OPENSSL_EXPORT void EC_POINT_clear_free(EC_POINT *point);


#if defined(__cplusplus)
}  // extern C
#endif

// Old code expects to get EC_KEY from ec.h.
#include <openssl/ec_key.h>

#if defined(__cplusplus)
extern "C++" {

BSSL_NAMESPACE_BEGIN

BORINGSSL_MAKE_DELETER(EC_POINT, EC_POINT_free)
BORINGSSL_MAKE_DELETER(EC_GROUP, EC_GROUP_free)

BSSL_NAMESPACE_END

}  // extern C++

#endif

#define EC_R_BUFFER_TOO_SMALL 100
#define EC_R_COORDINATES_OUT_OF_RANGE 101
#define EC_R_D2I_ECPKPARAMETERS_FAILURE 102
#define EC_R_EC_GROUP_NEW_BY_NAME_FAILURE 103
#define EC_R_GROUP2PKPARAMETERS_FAILURE 104
#define EC_R_I2D_ECPKPARAMETERS_FAILURE 105
#define EC_R_INCOMPATIBLE_OBJECTS 106
#define EC_R_INVALID_COMPRESSED_POINT 107
#define EC_R_INVALID_COMPRESSION_BIT 108
#define EC_R_INVALID_ENCODING 109
#define EC_R_INVALID_FIELD 110
#define EC_R_INVALID_FORM 111
#define EC_R_INVALID_GROUP_ORDER 112
#define EC_R_INVALID_PRIVATE_KEY 113
#define EC_R_MISSING_PARAMETERS 114
#define EC_R_MISSING_PRIVATE_KEY 115
#define EC_R_NON_NAMED_CURVE 116
#define EC_R_NOT_INITIALIZED 117
#define EC_R_PKPARAMETERS2GROUP_FAILURE 118
#define EC_R_POINT_AT_INFINITY 119
#define EC_R_POINT_IS_NOT_ON_CURVE 120
#define EC_R_SLOT_FULL 121
#define EC_R_UNDEFINED_GENERATOR 122
#define EC_R_UNKNOWN_GROUP 123
#define EC_R_UNKNOWN_ORDER 124
#define EC_R_WRONG_ORDER 125
#define EC_R_BIGNUM_OUT_OF_RANGE 126
#define EC_R_WRONG_CURVE_PARAMETERS 127
#define EC_R_DECODE_ERROR 128
#define EC_R_ENCODE_ERROR 129
#define EC_R_GROUP_MISMATCH 130
#define EC_R_INVALID_COFACTOR 131
#define EC_R_PUBLIC_KEY_VALIDATION_FAILED 132
#define EC_R_INVALID_SCALAR 133

#endif  // OPENSSL_HEADER_EC_H
