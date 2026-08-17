// Copyright 2016 The BoringSSL Authors
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

#ifndef OPENSSL_HEADER_CRYPTO_POOL_INTERNAL_H
#define OPENSSL_HEADER_CRYPTO_POOL_INTERNAL_H

#include <openssl/thread.h>

#include "../internal.h"
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

#endif  // OPENSSL_HEADER_CRYPTO_POOL_INTERNAL_H
