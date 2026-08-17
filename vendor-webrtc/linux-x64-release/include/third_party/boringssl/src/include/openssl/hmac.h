// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
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

#ifndef OPENSSL_HEADER_HMAC_H
#define OPENSSL_HEADER_HMAC_H

#include <openssl/base.h>   // IWYU pragma: export

#include <openssl/digest.h>

#if defined(__cplusplus)
extern "C" {
#endif


// HMAC contains functions for constructing PRFs from Merkle–Damgård hash
// functions using HMAC.


// One-shot operation.

// HMAC calculates the HMAC of |data_len| bytes of |data|, using the given key
// and hash function, and writes the result to |out|. On entry, |out| must
// contain at least |EVP_MD_size| bytes of space. The actual length of the
// result is written to |*out_len|. An output size of |EVP_MAX_MD_SIZE| will
// always be large enough. It returns |out| or NULL on error.
OPENSSL_EXPORT uint8_t *HMAC(const EVP_MD *evp_md, const void *key,
                             size_t key_len, const uint8_t *data,
                             size_t data_len, uint8_t *out,
                             unsigned int *out_len);


// Incremental operation.

// HMAC_CTX_init initialises |ctx| for use in an HMAC operation. It's assumed
// that HMAC_CTX objects will be allocated on the stack thus no allocation
// function is provided.
OPENSSL_EXPORT void HMAC_CTX_init(HMAC_CTX *ctx);

// HMAC_CTX_new allocates and initialises a new |HMAC_CTX| and returns it, or
// NULL on allocation failure. The caller must use |HMAC_CTX_free| to release
// the resulting object.
OPENSSL_EXPORT HMAC_CTX *HMAC_CTX_new(void);

// HMAC_CTX_cleanup frees data owned by |ctx|. It does not free |ctx| itself.
OPENSSL_EXPORT void HMAC_CTX_cleanup(HMAC_CTX *ctx);

// HMAC_CTX_cleanse zeros the digest state from |ctx| and then performs the
// actions of |HMAC_CTX_cleanup|.
OPENSSL_EXPORT void HMAC_CTX_cleanse(HMAC_CTX *ctx);

// HMAC_CTX_free calls |HMAC_CTX_cleanup| and then frees |ctx| itself.
OPENSSL_EXPORT void HMAC_CTX_free(HMAC_CTX *ctx);

// HMAC_Init_ex sets up an initialised |HMAC_CTX| to use |md| as the hash
// function and |key| as the key. For a non-initial call, |md| may be NULL, in
// which case the previous hash function will be used. If the hash function has
// not changed and |key| is NULL, |ctx| reuses the previous key. It returns one
// on success or zero on allocation failure.
//
// WARNING: NULL and empty keys are ambiguous on non-initial calls. Passing NULL
// |key| but repeating the previous |md| reuses the previous key rather than the
// empty key.
OPENSSL_EXPORT int HMAC_Init_ex(HMAC_CTX *ctx, const void *key, size_t key_len,
                                const EVP_MD *md, ENGINE *impl);

// HMAC_Update hashes |data_len| bytes from |data| into the current HMAC
// operation in |ctx|. It returns one.
OPENSSL_EXPORT int HMAC_Update(HMAC_CTX *ctx, const uint8_t *data,
                               size_t data_len);

// HMAC_Final completes the HMAC operation in |ctx| and writes the result to
// |out| and the sets |*out_len| to the length of the result. On entry, |out|
// must contain at least |HMAC_size| bytes of space. An output size of
// |EVP_MAX_MD_SIZE| will always be large enough. It returns one on success or
// zero on allocation failure.
OPENSSL_EXPORT int HMAC_Final(HMAC_CTX *ctx, uint8_t *out,
                              unsigned int *out_len);


// Utility functions.

// HMAC_size returns the size, in bytes, of the HMAC that will be produced by
// |ctx|. On entry, |ctx| must have been setup with |HMAC_Init_ex|.
OPENSSL_EXPORT size_t HMAC_size(const HMAC_CTX *ctx);

// HMAC_CTX_get_md returns |ctx|'s hash function.
OPENSSL_EXPORT const EVP_MD *HMAC_CTX_get_md(const HMAC_CTX *ctx);

// HMAC_CTX_copy_ex sets |dest| equal to |src|. On entry, |dest| must have been
// initialised by calling |HMAC_CTX_init|. It returns one on success and zero
// on error.
OPENSSL_EXPORT int HMAC_CTX_copy_ex(HMAC_CTX *dest, const HMAC_CTX *src);

// HMAC_CTX_reset calls |HMAC_CTX_cleanup| followed by |HMAC_CTX_init|.
OPENSSL_EXPORT void HMAC_CTX_reset(HMAC_CTX *ctx);


// Deprecated functions.

OPENSSL_EXPORT int HMAC_Init(HMAC_CTX *ctx, const void *key, int key_len,
                             const EVP_MD *md);

// HMAC_CTX_copy calls |HMAC_CTX_init| on |dest| and then sets it equal to
// |src|. On entry, |dest| must /not/ be initialised for an operation with
// |HMAC_Init_ex|. It returns one on success and zero on error.
OPENSSL_EXPORT int HMAC_CTX_copy(HMAC_CTX *dest, const HMAC_CTX *src);


// Private functions

struct hmac_ctx_st {
  const EVP_MD *md;
  EVP_MD_CTX md_ctx;
  EVP_MD_CTX i_ctx;
  EVP_MD_CTX o_ctx;
} /* HMAC_CTX */;


#if defined(__cplusplus)
}  // extern C

#if !defined(BORINGSSL_NO_CXX)
extern "C++" {

BSSL_NAMESPACE_BEGIN

BORINGSSL_MAKE_DELETER(HMAC_CTX, HMAC_CTX_free)

using ScopedHMAC_CTX =
    internal::StackAllocated<HMAC_CTX, void, HMAC_CTX_init, HMAC_CTX_cleanup>;

BSSL_NAMESPACE_END

}  // extern C++
#endif

#endif

#endif  // OPENSSL_HEADER_HMAC_H
