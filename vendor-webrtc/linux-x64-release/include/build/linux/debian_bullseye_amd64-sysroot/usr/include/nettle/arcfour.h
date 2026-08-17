/* arcfour.h

   The arcfour/rc4 stream cipher.

   Copyright (C) 2001, 2014 Niels Möller

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
 
#ifndef NETTLE_ARCFOUR_H_INCLUDED
#define NETTLE_ARCFOUR_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define arcfour128_set_key nettle_arcfour128_set_key
#define arcfour_set_key nettle_arcfour_set_key
#define arcfour_crypt nettle_arcfour_crypt

/* Minimum and maximum keysizes, and a reasonable default. In
 * octets.*/
#define ARCFOUR_MIN_KEY_SIZE 1
#define ARCFOUR_MAX_KEY_SIZE 256
#define ARCFOUR_KEY_SIZE 16
#define ARCFOUR128_KEY_SIZE 16

struct arcfour_ctx
{
  uint8_t S[256];
  uint8_t i;
  uint8_t j;
};

void
arcfour_set_key(struct arcfour_ctx *ctx,
		size_t length, const uint8_t *key);

void
arcfour128_set_key(struct arcfour_ctx *ctx, const uint8_t *key);

void
arcfour_crypt(struct arcfour_ctx *ctx,
	      size_t length, uint8_t *dst,
	      const uint8_t *src);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_ARCFOUR_H_INCLUDED */

