// Copyright 2024 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_SLHDSA_H
#define OPENSSL_HEADER_SLHDSA_H

#include <openssl/base.h>   // IWYU pragma: export

#if defined(__cplusplus)
extern "C" {
#endif


// SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES is the number of bytes in an
// SLH-DSA-SHA2-128s public key.
#define SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES 32

// SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES is the number of bytes in an
// SLH-DSA-SHA2-128s private key.
#define SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES 64

// SLHDSA_SHA2_128S_SIGNATURE_BYTES is the number of bytes in an
// SLH-DSA-SHA2-128s signature.
#define SLHDSA_SHA2_128S_SIGNATURE_BYTES 7856

// SLHDSA_SHA2_128S_generate_key generates a SLH-DSA-SHA2-128s key pair and
// writes the result to |out_public_key| and |out_private_key|.
OPENSSL_EXPORT void SLHDSA_SHA2_128S_generate_key(
    uint8_t out_public_key[SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    uint8_t out_private_key[SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES]);

// SLHDSA_SHA2_128S_public_from_private writes the public key corresponding to
// |private_key| to |out_public_key|.
OPENSSL_EXPORT void SLHDSA_SHA2_128S_public_from_private(
    uint8_t out_public_key[SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    const uint8_t private_key[SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES]);

// SLHDSA_SHA2_128S_sign slowly generates a SLH-DSA-SHA2-128s signature of |msg|
// using |private_key| and writes it to |out_signature|. The |context| argument
// is also signed over and can be used to include implicit contextual
// information that isn't included in |msg|. The same value of |context| must be
// presented to |SLHDSA_SHA2_128S_verify| in order for the generated signature
// to be considered valid. |context| and |context_len| may be |NULL| and 0 to
// use an empty context (this is common). It returns 1 on success and 0 if
// |context_len| is larger than 255.
OPENSSL_EXPORT int SLHDSA_SHA2_128S_sign(
    uint8_t out_signature[SLHDSA_SHA2_128S_SIGNATURE_BYTES],
    const uint8_t private_key[SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES],
    const uint8_t *msg, size_t msg_len, const uint8_t *context,
    size_t context_len);

// SLHDSA_SHA2_128S_verify verifies that |signature| is a valid
// SLH-DSA-SHA2-128s signature of |msg| by |public_key|. The value of |context|
// must equal the value that was passed to |SLHDSA_SHA2_128S_sign| when the
// signature was generated. It returns 1 if the signature is valid and 0
// otherwise.
OPENSSL_EXPORT int SLHDSA_SHA2_128S_verify(
    const uint8_t *signature, size_t signature_len,
    const uint8_t public_key[SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    const uint8_t *msg, size_t msg_len, const uint8_t *context,
    size_t context_len);


// Prehashed SLH-DSA-SHA2-128s.
//
// These functions sign the hash of a message. They should generally not be
// used. The general functions are perfectly capable of signing a hash if you
// wish. These functions should only be used when:
//
//   a) Compatibility with an external system that uses prehashed messages is
//   required. (The general signature of a hash is not compatible with a
//   "prehash" signature of the same hash.)
//   b) A single private key is used to sign both prehashed and raw messages,
//   and there's no other way to prevent ambiguity.

// SLHDSA_SHA2_128S_prehash_sign slowly generates a SLH-DSA-SHA2-128s signature
// of the prehashed |hashed_msg| using |private_key| and writes it to
// |out_signature|. The |context| argument is also signed over and can be used
// to include implicit contextual information that isn't included in
// |hashed_msg|. The same value of |context| must be presented to
// |SLHDSA_SHA2_128S_prehash_verify| in order for the generated signature to be
// considered valid. |context| and |context_len| may be |NULL| and 0 to use an
// empty context (this is common).
//
// The |hash_nid| argument must specify the hash function that was used to
// generate |hashed_msg|. This function only accepts hash functions listed in
// FIPS 205.
//
// This function returns 1 on success and 0 if |context_len| is larger than 255,
// if the hash function is not supported, or if |hashed_msg| is the wrong
// length.
OPENSSL_EXPORT int SLHDSA_SHA2_128S_prehash_sign(
    uint8_t out_signature[SLHDSA_SHA2_128S_SIGNATURE_BYTES],
    const uint8_t private_key[SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES],
    const uint8_t *hashed_msg, size_t hashed_msg_len, int hash_nid,
    const uint8_t *context, size_t context_len);

// SLHDSA_SHA2_128S_prehash_verify verifies that |signature| is a valid
// SLH-DSA-SHA2-128s signature of the prehashed |hashed_msg| by |public_key|,
// using the hash algorithm identified by |hash_nid|. The value of |context|
// must equal the value that was passed to |SLHDSA_SHA2_128S_prehash_sign| when
// the signature was generated.
//
// The |hash_nid| argument must specify the hash function that was used to
// generate |hashed_msg|. This function only accepts hash functions that are
// listed in FIPS 205.
//
// This function returns 1 if the signature is valid and 0 if the signature is
// invalid, the hash function is not supported, or if |hashed_msg| is the wrong
// length.
OPENSSL_EXPORT int SLHDSA_SHA2_128S_prehash_verify(
    const uint8_t *signature, size_t signature_len,
    const uint8_t public_key[SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    const uint8_t *hashed_msg, size_t hashed_msg_len, int hash_nid,
    const uint8_t *context, size_t context_len);

// SLHDSA_SHA2_128S_prehash_warning_nonstandard_sign slowly generates a
// SLH-DSA-SHA2-128s signature of the prehashed |hashed_msg| using |private_key|
// and writes it to |out_signature|. The |context| argument is also signed over
// and can be used to include implicit contextual information that isn't
// included in |hashed_msg|. The same value of |context| must be presented to
// |SLHDSA_SHA2_128S_prehash_warning_nonstandard_verify| in order for the
// generated signature to be considered valid. |context| and |context_len| may
// be |NULL| and 0 to use an empty context (this is common).
//
// The |hash_nid| argument must specify the hash function that was used to
// generate |hashed_msg|. This function only accepts non-standard hash functions
// that are not compliant with FIPS 205.
//
// This function returns 1 on success and 0 if |context_len| is larger than 255,
// if the hash function is not supported, or if |hashed_msg| is the wrong
// length.
OPENSSL_EXPORT int SLHDSA_SHA2_128S_prehash_warning_nonstandard_sign(
    uint8_t out_signature[SLHDSA_SHA2_128S_SIGNATURE_BYTES],
    const uint8_t private_key[SLHDSA_SHA2_128S_PRIVATE_KEY_BYTES],
    const uint8_t *hashed_msg, size_t hashed_msg_len, int hash_nid,
    const uint8_t *context, size_t context_len);

// SLHDSA_SHA2_128S_prehash_warning_nonstandard_verify verifies that |signature|
// is a valid SLH-DSA-SHA2-128s signature of the prehashed |hashed_msg| by
// |public_key|, using the hash algorithm identified by |hash_nid|. The value of
// |context| must equal the value that was passed to
// |SLHDSA_SHA2_128S_prehash_sign| when the signature was generated.
//
// The |hash_nid| argument must specify the hash function that was used to
// generate |hashed_msg|. This function only accepts non-standard hash functions
// that are not compliant with FIPS 205.
//
// This function returns 1 if the signature is valid and 0 if the signature is
// invalid, the hash function is not supported, or if |hashed_msg| is the wrong
// length.
OPENSSL_EXPORT int SLHDSA_SHA2_128S_prehash_warning_nonstandard_verify(
    const uint8_t *signature, size_t signature_len,
    const uint8_t public_key[SLHDSA_SHA2_128S_PUBLIC_KEY_BYTES],
    const uint8_t *hashed_msg, size_t hashed_msg_len, int hash_nid,
    const uint8_t *context, size_t context_len);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_SLHDSA_H
