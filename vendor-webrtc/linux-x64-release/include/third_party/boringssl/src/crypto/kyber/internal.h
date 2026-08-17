// Copyright 2023 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_KYBER_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_KYBER_INTERNAL_H

#include <openssl/base.h>
#include <openssl/experimental/kyber.h>

#if defined(__cplusplus)
extern "C" {
#endif


// KYBER_ENCAP_ENTROPY is the number of bytes of uniformly random entropy
// necessary to encapsulate a secret. The entropy will be leaked to the
// decapsulating party.
#define KYBER_ENCAP_ENTROPY 32

// KYBER_GENERATE_KEY_ENTROPY is the number of bytes of uniformly random entropy
// necessary to generate a key.
#define KYBER_GENERATE_KEY_ENTROPY 64

// KYBER_generate_key_external_entropy is a deterministic function to create a
// pair of Kyber768 keys, using the supplied entropy. The entropy needs to be
// uniformly random generated. This function is should only be used for tests,
// regular callers should use the non-deterministic |KYBER_generate_key|
// directly.
OPENSSL_EXPORT void KYBER_generate_key_external_entropy(
    uint8_t out_encoded_public_key[KYBER_PUBLIC_KEY_BYTES],
    struct KYBER_private_key *out_private_key,
    const uint8_t entropy[KYBER_GENERATE_KEY_ENTROPY]);

// KYBER_encap_external_entropy behaves like |KYBER_encap|, but uses
// |KYBER_ENCAP_ENTROPY| bytes of |entropy| for randomization. The decapsulating
// side will be able to recover |entropy| in full. This function should only be
// used for tests, regular callers should use the non-deterministic
// |KYBER_encap| directly.
OPENSSL_EXPORT void KYBER_encap_external_entropy(
    uint8_t out_ciphertext[KYBER_CIPHERTEXT_BYTES],
    uint8_t out_shared_secret[KYBER_SHARED_SECRET_BYTES],
    const struct KYBER_public_key *public_key,
    const uint8_t entropy[KYBER_ENCAP_ENTROPY]);

#if defined(__cplusplus)
}
#endif

#endif  // OPENSSL_HEADER_CRYPTO_KYBER_INTERNAL_H
