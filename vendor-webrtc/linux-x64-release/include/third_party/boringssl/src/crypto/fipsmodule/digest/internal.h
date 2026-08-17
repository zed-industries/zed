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

#ifndef OPENSSL_HEADER_CRYPTO_FIPSMODULE_DIGEST_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_FIPSMODULE_DIGEST_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


struct env_md_st {
  // type contains a NID identifing the digest function. (For example,
  // NID_md5.)
  int type;

  // md_size contains the size, in bytes, of the resulting digest.
  unsigned md_size;

  // flags contains the OR of |EVP_MD_FLAG_*| values.
  uint32_t flags;

  // init initialises the state in |ctx->md_data|.
  void (*init)(EVP_MD_CTX *ctx);

  // update hashes |len| bytes of |data| into the state in |ctx->md_data|.
  void (*update)(EVP_MD_CTX *ctx, const void *data, size_t count);

  // final completes the hash and writes |md_size| bytes of digest to |out|.
  void (*final)(EVP_MD_CTX *ctx, uint8_t *out);

  // block_size contains the hash's native block size.
  unsigned block_size;

  // ctx_size contains the size, in bytes, of the state of the hash function.
  unsigned ctx_size;
};

// evp_md_pctx_ops contains function pointers to allow the |pctx| member of
// |EVP_MD_CTX| to be manipulated without breaking layering by calling EVP
// functions.
struct evp_md_pctx_ops {
  // free is called when an |EVP_MD_CTX| is being freed and the |pctx| also
  // needs to be freed.
  void (*free) (EVP_PKEY_CTX *pctx);

  // dup is called when an |EVP_MD_CTX| is copied and so the |pctx| also needs
  // to be copied.
  EVP_PKEY_CTX* (*dup) (EVP_PKEY_CTX *pctx);
};


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_CRYPTO_FIPSMODULE_DIGEST_INTERNAL_H
