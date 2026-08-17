/* md5.h

   The MD5 hash function, described in RFC 1321.

   Copyright (C) 2001 Niels Möller

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
 
#ifndef NETTLE_MD5_H_INCLUDED
#define NETTLE_MD5_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define md5_init nettle_md5_init
#define md5_update nettle_md5_update
#define md5_digest nettle_md5_digest

#define MD5_DIGEST_SIZE 16
#define MD5_BLOCK_SIZE 64
/* For backwards compatibility */
#define MD5_DATA_SIZE MD5_BLOCK_SIZE

/* Digest is kept internally as 4 32-bit words. */
#define _MD5_DIGEST_LENGTH 4

struct md5_ctx
{
  uint32_t state[_MD5_DIGEST_LENGTH];
  uint64_t count;               /* Block count */
  unsigned index;               /* Into buffer */
  uint8_t block[MD5_BLOCK_SIZE]; /* Block buffer */
};

void
md5_init(struct md5_ctx *ctx);

void
md5_update(struct md5_ctx *ctx,
	   size_t length,
	   const uint8_t *data);

void
md5_digest(struct md5_ctx *ctx,
	   size_t length,
	   uint8_t *digest);

/* Internal compression function. STATE points to 4 uint32_t words,
   and DATA points to 64 bytes of input data, possibly unaligned. */
void
nettle_md5_compress(uint32_t *state, const uint8_t *data);

#define _nettle_md5_compress nettle_md5_compress

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_MD5_H_INCLUDED */
