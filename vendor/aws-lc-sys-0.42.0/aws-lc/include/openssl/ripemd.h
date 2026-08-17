// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com) All rights reserved.
// SPDX-License-Identifier: Apache-2.0

#ifndef OPENSSL_HEADER_RIPEMD_H
#define OPENSSL_HEADER_RIPEMD_H

#include <openssl/base.h>

#ifdef  __cplusplus
extern "C" {
#endif


# define RIPEMD160_CBLOCK        64
# define RIPEMD160_LBLOCK        (RIPEMD160_CBLOCK/4)
# define RIPEMD160_DIGEST_LENGTH 20

struct RIPEMD160state_st {
  uint32_t h[5];
  uint32_t Nl, Nh;
  uint8_t data[RIPEMD160_CBLOCK];
  unsigned num;
};

// RIPEMD160_Init initialises |ctx| and returns one.
OPENSSL_EXPORT int RIPEMD160_Init(RIPEMD160_CTX *ctx);

// RIPEMD160_Update adds |len| bytes from |data| to |ctx| and returns one.
OPENSSL_EXPORT int RIPEMD160_Update(RIPEMD160_CTX *ctx, const void *data,
                                   size_t len);

// RIPEMD160_Final adds the final padding to |ctx| and writes the resulting
// digest to |out|, which must have at least |RIPEMD160_DIGEST_LENGTH| bytes of
// space. It returns one.
OPENSSL_EXPORT int RIPEMD160_Final(uint8_t out[RIPEMD160_DIGEST_LENGTH],
                                   RIPEMD160_CTX *ctx);

// RIPEMD160 writes the digest of |len| bytes from |data| to |out| and returns
// |out|. There must be at least |RIPEMD160_DIGEST_LENGTH| bytes of space in
// |out|.
OPENSSL_EXPORT uint8_t *RIPEMD160(const uint8_t *data, size_t len,
                                  uint8_t out[RIPEMD160_DIGEST_LENGTH]);

#if defined(__cplusplus)
}  // extern C
#endif

#endif  // OPENSSL_HEADER_RIPEMD_H
