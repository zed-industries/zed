// Copyright (c) 2020, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_EC_EXTRA_INTERNAL_H
#define OPENSSL_HEADER_EC_EXTRA_INTERNAL_H

#include <openssl/ec.h>

#include "../fipsmodule/ec/internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


// Hash-to-curve.
//
// Internal |EC_JACOBIAN| versions of the corresponding public APIs.

// ec_hash_to_curve_p256_xmd_sha256_sswu hashes |msg| to a point on |group| and
// writes the result to |out|, implementing the P256_XMD:SHA-256_SSWU_RO_ suite
// from RFC 9380. It returns one on success and zero on error.
OPENSSL_EXPORT int ec_hash_to_curve_p256_xmd_sha256_sswu(
    const EC_GROUP *group, EC_JACOBIAN *out, const uint8_t *dst, size_t dst_len,
    const uint8_t *msg, size_t msg_len);

// ec_hash_to_curve_p384_xmd_sha384_sswu hashes |msg| to a point on |group| and
// writes the result to |out|, implementing the P384_XMD:SHA-384_SSWU_RO_ suite
// from RFC 9380. It returns one on success and zero on error.
OPENSSL_EXPORT int ec_hash_to_curve_p384_xmd_sha384_sswu(
    const EC_GROUP *group, EC_JACOBIAN *out, const uint8_t *dst, size_t dst_len,
    const uint8_t *msg, size_t msg_len);

// ec_hash_to_scalar_p384_xmd_sha384 hashes |msg| to a scalar on |group|
// and writes the result to |out|, using the hash_to_field operation from the
// P384_XMD:SHA-384_SSWU_RO_ suite from RFC 9380, but generating a value modulo
// the group order rather than a field element.
OPENSSL_EXPORT int ec_hash_to_scalar_p384_xmd_sha384(
    const EC_GROUP *group, EC_SCALAR *out, const uint8_t *dst, size_t dst_len,
    const uint8_t *msg, size_t msg_len);

// ec_hash_to_curve_p384_xmd_sha512_sswu_draft07 hashes |msg| to a point on
// |group| and writes the result to |out|, implementing the
// P384_XMD:SHA-512_SSWU_RO_ suite from draft-irtf-cfrg-hash-to-curve-07. It
// returns one on success and zero on error.
//
// TODO(https://crbug.com/1414562): Migrate this to the final version.
OPENSSL_EXPORT int ec_hash_to_curve_p384_xmd_sha512_sswu_draft07(
    const EC_GROUP *group, EC_JACOBIAN *out, const uint8_t *dst, size_t dst_len,
    const uint8_t *msg, size_t msg_len);

// ec_hash_to_scalar_p384_xmd_sha512_draft07 hashes |msg| to a scalar on |group|
// and writes the result to |out|, using the hash_to_field operation from the
// P384_XMD:SHA-512_SSWU_RO_ suite from draft-irtf-cfrg-hash-to-curve-07, but
// generating a value modulo the group order rather than a field element.
//
// TODO(https://crbug.com/1414562): Migrate this to the final version.
OPENSSL_EXPORT int ec_hash_to_scalar_p384_xmd_sha512_draft07(
    const EC_GROUP *group, EC_SCALAR *out, const uint8_t *dst, size_t dst_len,
    const uint8_t *msg, size_t msg_len);

enum ECParametersType {
  UNKNOWN_EC_PARAMETERS = 0,
  NAMED_CURVE_EC_PARAMETERS = 1,
  SPECIFIED_CURVE_EC_PARAMETERS = 2,
};

// EC_KEY_parse_parameters_and_type parses the elliptic curve key parameters from |cbs| and
// sets the type of parameters found on |paramType|.
EC_GROUP *EC_KEY_parse_parameters_and_type(CBS *cbs, enum ECParametersType *paramType);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_EC_EXTRA_INTERNAL_H
