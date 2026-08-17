/* salsa20.h

   The Salsa20 stream cipher.

   Copyright (C) 2012 Simon Josefsson
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

#ifndef NETTLE_SALSA20_H_INCLUDED
#define NETTLE_SALSA20_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define salsa20_set_key nettle_salsa20_set_key
#define salsa20_128_set_key nettle_salsa20_128_set_key
#define salsa20_256_set_key nettle_salsa20_256_set_key
#define salsa20_set_nonce nettle_salsa20_set_nonce
#define salsa20_crypt nettle_salsa20_crypt

#define salsa20r12_crypt nettle_salsa20r12_crypt

/* Alias for backwards compatibility */
#define salsa20_set_iv nettle_salsa20_set_nonce

/* In octets.*/
#define SALSA20_128_KEY_SIZE 16
#define SALSA20_256_KEY_SIZE 32
#define SALSA20_BLOCK_SIZE 64
#define SALSA20_NONCE_SIZE 8
#define SALSA20_IV_SIZE SALSA20_NONCE_SIZE

/* Aliases */
#define SALSA20_MIN_KEY_SIZE 16
#define SALSA20_MAX_KEY_SIZE 32
#define SALSA20_KEY_SIZE 32

#define _SALSA20_INPUT_LENGTH 16

struct salsa20_ctx
{
  /* Indices 1-4 and 11-14 holds the key (two identical copies for the
     shorter key size), indices 0, 5, 10, 15 are constant, indices 6, 7
     are the IV, and indices 8, 9 are the block counter:

     C K K K
     K C I I
     B B C K
     K K K C
  */
  uint32_t input[_SALSA20_INPUT_LENGTH];
};

void
salsa20_128_set_key(struct salsa20_ctx *ctx, const uint8_t *key);
void
salsa20_256_set_key(struct salsa20_ctx *ctx, const uint8_t *key);

void
salsa20_set_key(struct salsa20_ctx *ctx,
		size_t length, const uint8_t *key);

void
salsa20_set_nonce(struct salsa20_ctx *ctx, const uint8_t *nonce);
  
void
salsa20_crypt(struct salsa20_ctx *ctx,
	      size_t length, uint8_t *dst,
	      const uint8_t *src);

void
salsa20r12_crypt(struct salsa20_ctx *ctx,
		 size_t length, uint8_t *dst,
		 const uint8_t *src);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_SALSA20_H_INCLUDED */
