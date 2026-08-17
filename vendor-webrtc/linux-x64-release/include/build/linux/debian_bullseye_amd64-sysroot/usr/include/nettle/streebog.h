/* streebog.h

   The Streebog family of hash functions.

   Copyright (C) 2020 Dmitry Baryshkov

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
 
#ifndef NETTLE_STREEBOG_H_INCLUDED
#define NETTLE_STREEBOG_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define streebog256_init nettle_streebog256_init
#define streebog256_digest nettle_streebog256_digest
#define streebog512_init nettle_streebog512_init
#define streebog512_update nettle_streebog512_update
#define streebog512_digest nettle_streebog512_digest

/* STREEBOG512 */

#define STREEBOG512_DIGEST_SIZE 64
#define STREEBOG512_BLOCK_SIZE 64

/* Digest is kept internally as 8 64-bit words. */
#define _STREEBOG512_DIGEST_LENGTH 8

struct streebog512_ctx
{
  uint64_t state[_STREEBOG512_DIGEST_LENGTH];    /* State variables */
  uint64_t count[_STREEBOG512_DIGEST_LENGTH];
  uint64_t sigma[_STREEBOG512_DIGEST_LENGTH];
  unsigned int index;                       /* index into buffer */
  uint8_t block[STREEBOG512_BLOCK_SIZE];          /* STREEBOG512 data buffer */
};

void
streebog512_init(struct streebog512_ctx *ctx);

void
streebog512_update(struct streebog512_ctx *ctx,
	      size_t length,
	      const uint8_t *data);

void
streebog512_digest(struct streebog512_ctx *ctx,
	      size_t length,
	      uint8_t *digest);


#define STREEBOG256_DIGEST_SIZE 32
#define STREEBOG256_BLOCK_SIZE STREEBOG512_BLOCK_SIZE
#define streebog256_ctx streebog512_ctx

void
streebog256_init(struct streebog256_ctx *ctx);

#define streebog256_update nettle_streebog512_update

void
streebog256_digest(struct streebog256_ctx *ctx,
		  size_t length,
		  uint8_t *digest);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_STREEBOG_H_INCLUDED */
