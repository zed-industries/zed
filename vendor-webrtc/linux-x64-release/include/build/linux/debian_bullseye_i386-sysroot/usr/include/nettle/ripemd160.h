/* ripemd160.h

   RIPEMD-160 hash function.

   Copyright (C) 2011 Andres Mejia

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

#ifndef NETTLE_RIPEMD160_H_INCLUDED
#define NETTLE_RIPEMD160_H_INCLUDED

#ifdef __cplusplus
extern "C" {
#endif

#include "nettle-types.h"

/* Name mangling */
#define ripemd160_init nettle_ripemd160_init
#define ripemd160_update nettle_ripemd160_update
#define ripemd160_digest nettle_ripemd160_digest

/* RIPEMD160 */

#define RIPEMD160_DIGEST_SIZE 20
#define RIPEMD160_BLOCK_SIZE 64
/* For backwards compatibility */
#define RIPEMD160_DATA_SIZE RIPEMD160_BLOCK_SIZE

/* Digest is kept internally as 5 32-bit words. */
#define _RIPEMD160_DIGEST_LENGTH 5

struct ripemd160_ctx
{
  uint32_t state[_RIPEMD160_DIGEST_LENGTH];
  uint64_t count;         /* 64-bit block count */
  unsigned int index;
  uint8_t block[RIPEMD160_BLOCK_SIZE];
};

void
ripemd160_init(struct ripemd160_ctx *ctx);

void
ripemd160_update(struct ripemd160_ctx *ctx,
		 size_t length,
		 const uint8_t *data);

void
ripemd160_digest(struct ripemd160_ctx *ctx,
		 size_t length,
		 uint8_t *digest);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_RIPEMD160_H_INCLUDED */
