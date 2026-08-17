// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_MD4_H
#define OPENSSL_HEADER_MD4_H

#include <openssl/base.h>

#if defined(__cplusplus)
extern "C" {
#endif


// MD4.

// MD4_CBLOCK is the block size of MD4.
#define MD4_CBLOCK 64

// MD4_DIGEST_LENGTH is the length of an MD4 digest.
#define MD4_DIGEST_LENGTH 16

// MD4_Init initialises |md4| and returns one.
OPENSSL_EXPORT int MD4_Init(MD4_CTX *md4);

// MD4_Update adds |len| bytes from |data| to |md4| and returns one.
OPENSSL_EXPORT int MD4_Update(MD4_CTX *md4, const void *data, size_t len);

// MD4_Final adds the final padding to |md4| and writes the resulting digest to
// |out|, which must have at least |MD4_DIGEST_LENGTH| bytes of space. It
// returns one.
OPENSSL_EXPORT int MD4_Final(uint8_t out[MD4_DIGEST_LENGTH], MD4_CTX *md4);

// MD4 writes the digest of |len| bytes from |data| to |out| and returns |out|.
// There must be at least |MD4_DIGEST_LENGTH| bytes of space in |out|.
OPENSSL_EXPORT uint8_t *MD4(const uint8_t *data, size_t len,
                            uint8_t out[MD4_DIGEST_LENGTH]);

// MD4_Transform is a low-level function that performs a single, MD4 block
// transformation using the state from |md4| and 64 bytes from |block|.
OPENSSL_EXPORT void MD4_Transform(MD4_CTX *md4,
                                  const uint8_t block[MD4_CBLOCK]);

struct md4_state_st {
  uint32_t h[4];
  uint32_t Nl, Nh;
  uint8_t data[MD4_CBLOCK];
  unsigned num;
};


#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_MD4_H
