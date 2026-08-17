/* umac.h

   UMAC message authentication code (RFC-4418).

   Copyright (C) 2013 Niels Möller

   This file is part of GNU Nettle.

   GNU Nettle is free software: you can redistribute it and/or
   modify it under the terms of either:

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at your
       option) any later version.

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at your
       option) any later version.

   or both in parallel, as here.

   GNU Nettle is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see http://www.gnu.org/licenses/.
*/

#ifndef NETTLE_UMAC_H_INCLUDED
#define NETTLE_UMAC_H_INCLUDED

#ifdef __cplusplus
extern "C" {
#endif

/* Namespace mangling */
#define umac32_set_key  nettle_umac32_set_key
#define umac64_set_key  nettle_umac64_set_key
#define umac96_set_key  nettle_umac96_set_key
#define umac128_set_key nettle_umac128_set_key
#define umac32_set_nonce  nettle_umac32_set_nonce
#define umac64_set_nonce  nettle_umac64_set_nonce
#define umac96_set_nonce  nettle_umac96_set_nonce
#define umac128_set_nonce nettle_umac128_set_nonce
#define umac32_update  nettle_umac32_update
#define umac64_update  nettle_umac64_update
#define umac96_update  nettle_umac96_update
#define umac128_update nettle_umac128_update
#define umac32_digest  nettle_umac32_digest
#define umac64_digest  nettle_umac64_digest
#define umac96_digest  nettle_umac96_digest
#define umac128_digest nettle_umac128_digest

#include "nettle-types.h"
#include "aes.h"

#define UMAC_KEY_SIZE AES128_KEY_SIZE
#define UMAC32_DIGEST_SIZE 4
#define UMAC64_DIGEST_SIZE 8
#define UMAC96_DIGEST_SIZE 12
#define UMAC128_DIGEST_SIZE 16
#define UMAC_BLOCK_SIZE 1024
#define UMAC_MIN_NONCE_SIZE 1
#define UMAC_MAX_NONCE_SIZE AES_BLOCK_SIZE
/* For backwards compatibility */
#define UMAC_DATA_SIZE UMAC_BLOCK_SIZE

/* Subkeys and state for UMAC with tag size 32*n bits. */
#define _UMAC_STATE(n)					\
  uint32_t l1_key[UMAC_BLOCK_SIZE/4 + 4*((n)-1)];	\
  /* Keys in 32-bit pieces, high first */		\
  uint32_t l2_key[6*(n)];				\
  uint64_t l3_key1[8*(n)];				\
  uint32_t l3_key2[(n)];				\
  /* AES cipher for encrypting the nonce */		\
  struct aes128_ctx pdf_key;				\
  /* The l2_state consists of 2*n uint64_t, for poly64	\
     and poly128 hashing, followed by n additional	\
     uint64_t used as an input buffer. */		\
  uint64_t l2_state[3*(n)];				\
  /* Input to the pdf_key, zero-padded and low bits	\
     cleared if appropriate. */				\
  uint8_t nonce[AES_BLOCK_SIZE];			\
  unsigned short nonce_length /* For incrementing */

  /* Buffering */ 
#define _UMAC_BUFFER					\
  unsigned index;					\
  /* Complete blocks processed */			\
  uint64_t count;					\
  uint8_t block[UMAC_BLOCK_SIZE]
  
#define _UMAC_NONCE_CACHED 0x80

struct umac32_ctx
{
  _UMAC_STATE(1);
  /* Low bits and cache flag. */
  unsigned short nonce_low;
  /* Previous padding block */
  uint32_t pad_cache[AES_BLOCK_SIZE / 4];
  _UMAC_BUFFER;
};

struct umac64_ctx
{
  _UMAC_STATE(2);
  /* Low bit and cache flag. */
  unsigned short nonce_low;
  /* Previous padding block */
  uint32_t pad_cache[AES_BLOCK_SIZE/4];
  _UMAC_BUFFER;
};

struct umac96_ctx
{
  _UMAC_STATE(3);
  _UMAC_BUFFER;
};

struct umac128_ctx
{
  _UMAC_STATE(4);
  _UMAC_BUFFER;
};

/* The _set_key function initialize the nonce to zero. */
void
umac32_set_key (struct umac32_ctx *ctx, const uint8_t *key);
void
umac64_set_key (struct umac64_ctx *ctx, const uint8_t *key);
void
umac96_set_key (struct umac96_ctx *ctx, const uint8_t *key);
void
umac128_set_key (struct umac128_ctx *ctx, const uint8_t *key);

/* Optional, if not used, messages get incrementing nonces starting from zero. */
void
umac32_set_nonce (struct umac32_ctx *ctx,
		  size_t nonce_length, const uint8_t *nonce);
void
umac64_set_nonce (struct umac64_ctx *ctx,
		  size_t nonce_length, const uint8_t *nonce);
void
umac96_set_nonce (struct umac96_ctx *ctx,
		  size_t nonce_length, const uint8_t *nonce);
void
umac128_set_nonce (struct umac128_ctx *ctx,
		   size_t nonce_length, const uint8_t *nonce);

void
umac32_update (struct umac32_ctx *ctx,
	       size_t length, const uint8_t *data);
void
umac64_update (struct umac64_ctx *ctx,
	       size_t length, const uint8_t *data);
void
umac96_update (struct umac96_ctx *ctx,
	       size_t length, const uint8_t *data);
void
umac128_update (struct umac128_ctx *ctx,
		size_t length, const uint8_t *data);

/* The _digest functions increment the nonce */
void
umac32_digest (struct umac32_ctx *ctx,
	       size_t length, uint8_t *digest);
void
umac64_digest (struct umac64_ctx *ctx,
	       size_t length, uint8_t *digest);
void
umac96_digest (struct umac96_ctx *ctx,
	       size_t length, uint8_t *digest);
void
umac128_digest (struct umac128_ctx *ctx,
		size_t length, uint8_t *digest);


/* Internal functions */
#define UMAC_POLY64_BLOCKS 16384

#define UMAC_P64_OFFSET 59
#define UMAC_P64 (- (uint64_t) UMAC_P64_OFFSET)

#define UMAC_P128_OFFSET 159
#define UMAC_P128_HI (~(uint64_t) 0)
#define UMAC_P128_LO (-(uint64_t) UMAC_P128_OFFSET)

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_UMAC_H_INCLUDED */
