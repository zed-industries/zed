/* sha1.h

   The sha1 hash function.

   Copyright (C) 2001, 2012 Niels Möller

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
 
#ifndef NETTLE_SHA1_H_INCLUDED
#define NETTLE_SHA1_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define sha1_init nettle_sha1_init
#define sha1_update nettle_sha1_update
#define sha1_digest nettle_sha1_digest

/* SHA1 */

#define SHA1_DIGEST_SIZE 20
#define SHA1_BLOCK_SIZE 64
/* For backwards compatibility */
#define SHA1_DATA_SIZE SHA1_BLOCK_SIZE

/* Digest is kept internally as 5 32-bit words. */
#define _SHA1_DIGEST_LENGTH 5

struct sha1_ctx
{
  uint32_t state[_SHA1_DIGEST_LENGTH];    /* State variables */
  uint64_t count;                         /* 64-bit block count */
  unsigned int index;                     /* index into buffer */
  uint8_t block[SHA1_BLOCK_SIZE];         /* SHA1 data buffer */
};

void
sha1_init(struct sha1_ctx *ctx);

void
sha1_update(struct sha1_ctx *ctx,
	    size_t length,
	    const uint8_t *data);

void
sha1_digest(struct sha1_ctx *ctx,
	    size_t length,
	    uint8_t *digest);

/* Internal compression function. STATE points to 5 uint32_t words,
   and DATA points to 64 bytes of input data, possibly unaligned. */
void
nettle_sha1_compress(uint32_t *state, const uint8_t *data);

#define _nettle_sha1_compress nettle_sha1_compress

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_SHA1_H_INCLUDED */
