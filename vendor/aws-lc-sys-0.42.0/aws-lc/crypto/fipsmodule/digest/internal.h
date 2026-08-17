// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_DIGEST_INTERNAL_H
#define OPENSSL_HEADER_DIGEST_INTERNAL_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


// env_md_st is typoed ("evp" -> "env"), but the typo comes from OpenSSL and
// some consumers forward-declare these structures so we're leaving it alone.
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
  // Digest update functions propagate the internal functions return value.
  // update calls after |final| are restricted via |ctx| check (|final|
  // cleanses the |ctx|). Digest XOF update returns 1 on success and 0 on
  // failure. Failures can only occur on a digest XOF update if called after
  // |squeezeXOF| or |finalXOF|.
  int (*update)(EVP_MD_CTX *ctx, const void *data, size_t count);

  // final completes the hash and writes |md_size| bytes of digest to |out|.
  void (*final)(EVP_MD_CTX *ctx, uint8_t *out);

  // block_size contains the hash's native block size.
  unsigned block_size;

  // ctx_size contains the size, in bytes, of the state of the hash function.
  unsigned ctx_size;

  // finalXOF completes the hash and writes |len| bytes of digest extended
  // output to |out|. Digest XOF finalXOF function propagates the return
  // value from |SHAKE_Final|, that is 1 on success and 0 on failure,
  // to restrict single-call finalXOF calls after |squeezeXOF|.
  int (*finalXOF)(EVP_MD_CTX *ctx, uint8_t *out, size_t len);

  // squeezeXOF incrementally generates |len| bytes of digest extended output
  // to |out|.
  void (*squeezeXOF)(EVP_MD_CTX *ctx, uint8_t *out, size_t len);
};

// evp_md_pctx_ops contains function pointers to allow the |pctx| member of
// |EVP_MD_CTX| to be manipulated without breaking layering by calling EVP
// functions.
struct evp_md_pctx_ops {
  // free is called when an |EVP_MD_CTX| is being freed and the |pctx| also
  // needs to be freed.
  void (*free)(EVP_PKEY_CTX *pctx);

  // dup is called when an |EVP_MD_CTX| is copied and so the |pctx| also needs
  // to be copied.
  EVP_PKEY_CTX *(*dup)(EVP_PKEY_CTX *pctx);
};


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_DIGEST_INTERNAL
