// Copyright (c) 2016, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_POOL_INTERNAL_H
#define OPENSSL_HEADER_POOL_INTERNAL_H

#include <openssl/lhash.h>
#include <openssl/thread.h>

#include "../lhash/internal.h"


#if defined(__cplusplus)
extern "C" {
#endif


DEFINE_LHASH_OF(CRYPTO_BUFFER)

struct crypto_buffer_st {
  CRYPTO_BUFFER_POOL *pool;
  uint8_t *data;
  size_t len;
  CRYPTO_refcount_t references;
  int data_is_static;
};

struct crypto_buffer_pool_st {
  LHASH_OF(CRYPTO_BUFFER) *bufs;
  CRYPTO_MUTEX lock;
  const uint64_t hash_key[2];
};


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_POOL_INTERNAL_H
