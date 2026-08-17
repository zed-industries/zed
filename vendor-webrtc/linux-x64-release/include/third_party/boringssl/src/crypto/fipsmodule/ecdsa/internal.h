// Copyright 2021 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_ECDSA_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_ECDSA_INTERNAL_H

#include <openssl/base.h>

#include "../ec/internal.h"

#if defined(__cplusplus)
extern "C" {
#endif


// ECDSA_MAX_FIXED_LEN is the maximum length of an ECDSA signature in the
// fixed-width, big-endian format from IEEE P1363.
#define ECDSA_MAX_FIXED_LEN (2 * EC_MAX_BYTES)

// ecdsa_sign_fixed behaves like |ECDSA_sign| but uses the fixed-width,
// big-endian format from IEEE P1363.
int ecdsa_sign_fixed(const uint8_t *digest, size_t digest_len, uint8_t *sig,
                     size_t *out_sig_len, size_t max_sig_len,
                     const EC_KEY *key);

// ecdsa_sign_fixed_with_nonce_for_known_answer_test behaves like
// |ecdsa_sign_fixed| but takes a caller-supplied nonce. This function is used
// as part of known-answer tests in the FIPS module.
int ecdsa_sign_fixed_with_nonce_for_known_answer_test(
    const uint8_t *digest, size_t digest_len, uint8_t *sig, size_t *out_sig_len,
    size_t max_sig_len, const EC_KEY *key, const uint8_t *nonce,
    size_t nonce_len);

// ecdsa_verify_fixed behaves like |ECDSA_verify| but uses the fixed-width,
// big-endian format from IEEE P1363.
int ecdsa_verify_fixed(const uint8_t *digest, size_t digest_len,
                       const uint8_t *sig, size_t sig_len, const EC_KEY *key);

// ecdsa_verify_fixed_no_self_test behaves like ecdsa_verify_fixed, but doesn't
// try to run the self-test first. This is for use in the self tests themselves,
// to prevent an infinite loop.
int ecdsa_verify_fixed_no_self_test(const uint8_t *digest, size_t digest_len,
                                    const uint8_t *sig, size_t sig_len,
                                    const EC_KEY *key);


#if defined(__cplusplus)
}
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_ECDSA_INTERNAL_H
