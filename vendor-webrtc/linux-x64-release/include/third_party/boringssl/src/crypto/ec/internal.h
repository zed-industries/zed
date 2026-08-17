// Copyright 2020 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_EC_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_EC_INTERNAL_H

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


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_EC_INTERNAL_H
