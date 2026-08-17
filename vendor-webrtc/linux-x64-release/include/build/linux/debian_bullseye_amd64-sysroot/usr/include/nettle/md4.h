/* md4.h

   The MD4 hash function, described in RFC 1320.

   Copyright (C) 2003 Niels Möller

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

#ifndef NETTLE_MD4_H_INCLUDED
#define NETTLE_MD4_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define md4_init nettle_md4_init
#define md4_update nettle_md4_update
#define md4_digest nettle_md4_digest

#define MD4_DIGEST_SIZE 16
#define MD4_BLOCK_SIZE 64
/* For backwards compatibility */
#define MD4_DATA_SIZE MD4_BLOCK_SIZE

/* Digest is kept internally as 4 32-bit words. */
#define _MD4_DIGEST_LENGTH 4

/* FIXME: Identical to md5_ctx */
struct md4_ctx
{
  uint32_t state[_MD4_DIGEST_LENGTH];
  uint64_t count;			/* Block count */
  unsigned index;			/* Into buffer */
  uint8_t block[MD4_BLOCK_SIZE];	/* Block buffer */
};

void
md4_init(struct md4_ctx *ctx);

void
md4_update(struct md4_ctx *ctx,
	   size_t length,
	   const uint8_t *data);

void
md4_digest(struct md4_ctx *ctx,
	   size_t length,
	   uint8_t *digest);


#ifdef __cplusplus
}
#endif

#endif /* NETTLE_MD4_H_INCLUDED */
