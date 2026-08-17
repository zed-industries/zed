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

#ifndef OPENSSL_HEADER_BLAKE2_H
#define OPENSSL_HEADER_BLAKE2_H

#include <openssl/base.h>   // IWYU pragma: export

#if defined(__cplusplus)
extern "C" {
#endif


#define BLAKE2B256_DIGEST_LENGTH (256 / 8)
#define BLAKE2B_CBLOCK 128

struct blake2b_state_st {
  uint64_t h[8];
  uint64_t t_low, t_high;
  uint8_t block[BLAKE2B_CBLOCK];
  size_t block_used;
};

// BLAKE2B256_Init initialises |b2b| to perform a BLAKE2b-256 hash. There are no
// pointers inside |b2b| thus release of |b2b| is purely managed by the caller.
OPENSSL_EXPORT void BLAKE2B256_Init(BLAKE2B_CTX *b2b);

// BLAKE2B256_Update appends |len| bytes from |data| to the digest being
// calculated by |b2b|.
OPENSSL_EXPORT void BLAKE2B256_Update(BLAKE2B_CTX *b2b, const void *data,
                                      size_t len);

// BLAKE2B256_Final completes the digest calculated by |b2b| and writes
// |BLAKE2B256_DIGEST_LENGTH| bytes to |out|.
OPENSSL_EXPORT void BLAKE2B256_Final(uint8_t out[BLAKE2B256_DIGEST_LENGTH],
                                     BLAKE2B_CTX *b2b);

// BLAKE2B256 writes the BLAKE2b-256 digset of |len| bytes from |data| to
// |out|.
OPENSSL_EXPORT void BLAKE2B256(const uint8_t *data, size_t len,
                               uint8_t out[BLAKE2B256_DIGEST_LENGTH]);


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_BLAKE2_H
