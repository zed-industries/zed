/* poly1305.h

   Poly1305 message authentication code.

   Copyright (C) 2013 Nikos Mavrogiannopoulos
   Copyright (C) 2013, 2014 Niels Möller

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

#ifndef NETTLE_POLY1305_H_INCLUDED
#define NETTLE_POLY1305_H_INCLUDED

#include "aes.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define poly1305_aes_set_key nettle_poly1305_aes_set_key
#define poly1305_aes_set_nonce nettle_poly1305_aes_set_nonce
#define poly1305_aes_update nettle_poly1305_aes_update
#define poly1305_aes_digest nettle_poly1305_aes_digest

/* Low level functions/macros for the poly1305 construction. */

#define POLY1305_BLOCK_SIZE 16

struct poly1305_ctx {
  /* Key, 128-bit value and some cached multiples. */
  union
  {
    uint32_t r32[6];
    uint64_t r64[3];
  } r;
  uint32_t s32[3];
  /* State, represented as words of 26, 32 or 64 bits, depending on
     implementation. */
  /* High bits first, to maintain alignment. */
  uint32_t hh;
  union
  {
    uint32_t h32[4];
    uint64_t h64[2];
  } h;
};

/* poly1305-aes */

#define POLY1305_AES_KEY_SIZE 32
#define POLY1305_AES_DIGEST_SIZE 16
#define POLY1305_AES_NONCE_SIZE 16

struct poly1305_aes_ctx
{
  /* Keep aes context last, to make it possible to use a general
     poly1305_update if other variants are added. */
  struct poly1305_ctx pctx;
  uint8_t block[POLY1305_BLOCK_SIZE];
  unsigned index;
  uint8_t nonce[POLY1305_BLOCK_SIZE];
  struct aes128_ctx aes;
};

/* Also initialize the nonce to zero. */
void
poly1305_aes_set_key (struct poly1305_aes_ctx *ctx, const uint8_t *key);

/* Optional, if not used, messages get incrementing nonces starting
   from zero. */
void
poly1305_aes_set_nonce (struct poly1305_aes_ctx *ctx,
		        const uint8_t *nonce);

/* Update is not aes-specific, but since this is the only implemented
   variant, we need no more general poly1305_update. */
void
poly1305_aes_update (struct poly1305_aes_ctx *ctx, size_t length, const uint8_t *data);

/* Also increments the nonce */
void
poly1305_aes_digest (struct poly1305_aes_ctx *ctx,
	       	     size_t length, uint8_t *digest);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_POLY1305_H_INCLUDED */
