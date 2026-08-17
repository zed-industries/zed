/* ctr.h

   Counter mode, using an network byte order incremented counter,
   matching the testcases of NIST 800-38A.

   Copyright (C) 2005 Niels Möller

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

#ifndef NETTLE_CTR_H_INCLUDED
#define NETTLE_CTR_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define ctr_crypt nettle_ctr_crypt

void
ctr_crypt(const void *ctx, nettle_cipher_func *f,
	  size_t block_size, uint8_t *ctr,
	  size_t length, uint8_t *dst,
	  const uint8_t *src);

#define CTR_CTX(type, size) \
{ type ctx; uint8_t ctr[size]; }

#define CTR_SET_COUNTER(ctx, data) \
memcpy((ctx)->ctr, (data), sizeof((ctx)->ctr))

#define CTR_CRYPT(self, f, length, dst, src)		\
  (0 ? ((f)(&(self)->ctx, ~(size_t) 0,			\
	  (uint8_t *) 0, (const uint8_t *) 0))		\
   : ctr_crypt((void *) &(self)->ctx,			\
	       (nettle_cipher_func *) (f),		\
	       sizeof((self)->ctr), (self)->ctr,	\
	       (length), (dst), (src)))

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_CTR_H_INCLUDED */
