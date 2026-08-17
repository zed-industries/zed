/* md2.h

   The MD2 hash function, described in RFC 1319.

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
 
#ifndef NETTLE_MD2_H_INCLUDED
#define NETTLE_MD2_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define md2_init nettle_md2_init
#define md2_update nettle_md2_update
#define md2_digest nettle_md2_digest

#define MD2_DIGEST_SIZE 16
#define MD2_BLOCK_SIZE 16
/* For backwards compatibility */
#define MD2_DATA_SIZE MD2_BLOCK_SIZE

struct md2_ctx
{
  uint8_t C[MD2_BLOCK_SIZE];
  uint8_t X[3 * MD2_BLOCK_SIZE];
  unsigned index;               /* Into buffer */
  uint8_t block[MD2_BLOCK_SIZE]; /* Block buffer */
};

void
md2_init(struct md2_ctx *ctx);

void
md2_update(struct md2_ctx *ctx,
	   size_t length,
	   const uint8_t *data);

void
md2_digest(struct md2_ctx *ctx,
	   size_t length,
	   uint8_t *digest);


#ifdef __cplusplus
}
#endif

#endif /* NETTLE_MD2_H_INCLUDED */
