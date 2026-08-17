/* arctwo.h

   The arctwo/rfc2268 block cipher.

   Copyright (C) 2004 Simon Josefsson
   Copyright (C) 2002, 2004, 2014 Niels Möller

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

#ifndef NETTLE_ARCTWO_H_INCLUDED
#define NETTLE_ARCTWO_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define arctwo_set_key nettle_arctwo_set_key
#define arctwo_set_key_ekb nettle_arctwo_set_key_ekb
#define arctwo_set_key_gutmann nettle_arctwo_set_key_gutmann
#define arctwo40_set_key nettle_arctwo40_set_key
#define arctwo64_set_key nettle_arctwo64_set_key
#define arctwo128_set_key nettle_arctwo128_set_key
#define arctwo128_set_key_gutmann nettle_arctwo128_set_key_gutmann
#define arctwo_encrypt nettle_arctwo_encrypt
#define arctwo_decrypt nettle_arctwo_decrypt

#define ARCTWO_BLOCK_SIZE 8

/* Variable key size from 1 byte to 128 bytes. */
#define ARCTWO_MIN_KEY_SIZE 1
#define ARCTWO_MAX_KEY_SIZE 128

#define ARCTWO_KEY_SIZE 8

struct arctwo_ctx
{
  uint16_t S[64];
};

/* Key expansion function that takes the "effective key bits", 1-1024,
   as an explicit argument. 0 means maximum key bits. */
void
arctwo_set_key_ekb (struct arctwo_ctx *ctx,
		    size_t length, const uint8_t * key, unsigned ekb);

/* Equvivalent to arctwo_set_key_ekb, with ekb = 8 * length */
void
arctwo_set_key (struct arctwo_ctx *ctx, size_t length, const uint8_t *key);
void
arctwo40_set_key (struct arctwo_ctx *ctx, const uint8_t *key);
void
arctwo64_set_key (struct arctwo_ctx *ctx, const uint8_t *key);
void
arctwo128_set_key (struct arctwo_ctx *ctx, const uint8_t *key);

/* Equvivalent to arctwo_set_key_ekb, with ekb = 1024 */
void
arctwo_set_key_gutmann (struct arctwo_ctx *ctx,
			size_t length, const uint8_t *key);
void
arctwo128_set_key_gutmann (struct arctwo_ctx *ctx,
			   const uint8_t *key);

void
arctwo_encrypt (struct arctwo_ctx *ctx,
		size_t length, uint8_t *dst, const uint8_t *src);
void
arctwo_decrypt (struct arctwo_ctx *ctx,
		size_t length, uint8_t *dst, const uint8_t *src);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_ARCTWO_H_INCLUDED */
