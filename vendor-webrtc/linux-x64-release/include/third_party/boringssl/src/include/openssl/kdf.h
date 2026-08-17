// Copyright 2022 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_KDF_H
#define OPENSSL_HEADER_KDF_H

#include <openssl/base.h>   // IWYU pragma: export

#if defined(__cplusplus)
extern "C" {
#endif


// KDF support for EVP.


// HKDF-specific functions.
//
// The following functions are provided for OpenSSL compatibility. Prefer the
// HKDF functions in <openssl/hkdf.h>. In each, |ctx| must be created with
// |EVP_PKEY_CTX_new_id| with |EVP_PKEY_HKDF| and then initialized with
// |EVP_PKEY_derive_init|.

// EVP_PKEY_HKDEF_MODE_* define "modes" for use with |EVP_PKEY_CTX_hkdf_mode|.
// The mispelling of "HKDF" as "HKDEF" is intentional for OpenSSL compatibility.
#define EVP_PKEY_HKDEF_MODE_EXTRACT_AND_EXPAND 0
#define EVP_PKEY_HKDEF_MODE_EXTRACT_ONLY 1
#define EVP_PKEY_HKDEF_MODE_EXPAND_ONLY 2

// EVP_PKEY_CTX_hkdf_mode configures which HKDF operation to run. It returns one
// on success and zero on error. |mode| must be one of |EVP_PKEY_HKDEF_MODE_*|.
// By default, the mode is |EVP_PKEY_HKDEF_MODE_EXTRACT_AND_EXPAND|.
//
// If |mode| is |EVP_PKEY_HKDEF_MODE_EXTRACT_AND_EXPAND| or
// |EVP_PKEY_HKDEF_MODE_EXPAND_ONLY|, the output is variable-length.
// |EVP_PKEY_derive| uses the size of the output buffer as the output length for
// HKDF-Expand.
//
// WARNING: Although this API calls it a "mode", HKDF-Extract and HKDF-Expand
// are distinct operations with distinct inputs and distinct kinds of keys.
// Callers should not pass input secrets for one operation into the other.
OPENSSL_EXPORT int EVP_PKEY_CTX_hkdf_mode(EVP_PKEY_CTX *ctx, int mode);

// EVP_PKEY_CTX_set_hkdf_md sets |md| as the digest to use with HKDF. It returns
// one on success and zero on error.
OPENSSL_EXPORT int EVP_PKEY_CTX_set_hkdf_md(EVP_PKEY_CTX *ctx,
                                            const EVP_MD *md);

// EVP_PKEY_CTX_set1_hkdf_key configures HKDF to use |key_len| bytes from |key|
// as the "key", described below. It returns one on success and zero on error.
//
// Which input is the key depends on the "mode" (see |EVP_PKEY_CTX_hkdf_mode|).
// If |EVP_PKEY_HKDEF_MODE_EXTRACT_AND_EXPAND| or
// |EVP_PKEY_HKDEF_MODE_EXTRACT_ONLY|, this function specifies the input keying
// material (IKM) for HKDF-Extract. If |EVP_PKEY_HKDEF_MODE_EXPAND_ONLY|, it
// instead specifies the pseudorandom key (PRK) for HKDF-Expand.
OPENSSL_EXPORT int EVP_PKEY_CTX_set1_hkdf_key(EVP_PKEY_CTX *ctx,
                                              const uint8_t *key,
                                              size_t key_len);

// EVP_PKEY_CTX_set1_hkdf_salt configures HKDF to use |salt_len| bytes from
// |salt| as the salt parameter to HKDF-Extract. It returns one on success and
// zero on error. If performing HKDF-Expand only, this parameter is ignored.
OPENSSL_EXPORT int EVP_PKEY_CTX_set1_hkdf_salt(EVP_PKEY_CTX *ctx,
                                               const uint8_t *salt,
                                               size_t salt_len);

// EVP_PKEY_CTX_add1_hkdf_info appends |info_len| bytes from |info| to the info
// parameter used with HKDF-Expand. It returns one on success and zero on error.
// If performing HKDF-Extract only, this parameter is ignored.
OPENSSL_EXPORT int EVP_PKEY_CTX_add1_hkdf_info(EVP_PKEY_CTX *ctx,
                                               const uint8_t *info,
                                               size_t info_len);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_KDF_H
