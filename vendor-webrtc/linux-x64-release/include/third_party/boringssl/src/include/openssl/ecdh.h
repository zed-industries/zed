// Copyright 2002-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_ECDH_H
#define OPENSSL_HEADER_ECDH_H

#include <openssl/base.h>   // IWYU pragma: export

#include <openssl/ec_key.h>

#if defined(__cplusplus)
extern "C" {
#endif


// Elliptic curve Diffie-Hellman.


// ECDH_compute_key calculates the shared key between |pub_key| and |priv_key|.
// If |kdf| is not NULL, then it is called with the bytes of the shared key and
// the parameter |out|. When |kdf| returns, the value of |*outlen| becomes the
// return value. Otherwise, as many bytes of the shared key as will fit are
// copied directly to, at most, |outlen| bytes at |out|. It returns the number
// of bytes written to |out|, or -1 on error.
OPENSSL_EXPORT int ECDH_compute_key(
    void *out, size_t outlen, const EC_POINT *pub_key, const EC_KEY *priv_key,
    void *(*kdf)(const void *in, size_t inlen, void *out, size_t *outlen));

// ECDH_compute_key_fips calculates the shared key between |pub_key| and
// |priv_key| and hashes it with the appropriate SHA function for |out_len|. The
// only value values for |out_len| are thus 24 (SHA-224), 32 (SHA-256), 48
// (SHA-384), and 64 (SHA-512). It returns one on success and zero on error.
//
// Note that the return value is different to |ECDH_compute_key|: it returns an
// error flag (as is common for BoringSSL) rather than the number of bytes
// written.
//
// This function allows the FIPS module to compute an ECDH and KDF within the
// module boundary without taking an arbitrary function pointer for the KDF,
// which isn't very FIPSy.
OPENSSL_EXPORT int ECDH_compute_key_fips(uint8_t *out, size_t out_len,
                                         const EC_POINT *pub_key,
                                         const EC_KEY *priv_key);


#if defined(__cplusplus)
}  // extern C
#endif

#define ECDH_R_KDF_FAILED 100
#define ECDH_R_NO_PRIVATE_VALUE 101
#define ECDH_R_POINT_ARITHMETIC_FAILURE 102
#define ECDH_R_UNKNOWN_DIGEST_LENGTH 103

#endif  // OPENSSL_HEADER_ECDH_H
