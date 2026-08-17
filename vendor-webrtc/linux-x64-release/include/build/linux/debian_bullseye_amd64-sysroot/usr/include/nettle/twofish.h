/* twofish.h

   The twofish block cipher.

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

/*
 * Twofish is a 128-bit block cipher that accepts a variable-length
 * key up to 256 bits, designed by Bruce Schneier and others.  See
 * http://www.counterpane.com/twofish.html for details.
 */

#ifndef NETTLE_TWOFISH_H_INCLUDED
#define NETTLE_TWOFISH_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define twofish_set_key nettle_twofish_set_key
#define twofish128_set_key nettle_twofish128_set_key
#define twofish192_set_key nettle_twofish192_set_key
#define twofish256_set_key nettle_twofish256_set_key
#define twofish_encrypt nettle_twofish_encrypt
#define twofish_decrypt nettle_twofish_decrypt

#define TWOFISH_BLOCK_SIZE 16

/* Variable key size between 128 and 256 bits. But the only valid
 * values are 16 (128 bits), 24 (192 bits) and 32 (256 bits). */
#define TWOFISH_MIN_KEY_SIZE 16
#define TWOFISH_MAX_KEY_SIZE 32

#define TWOFISH_KEY_SIZE 32
#define TWOFISH128_KEY_SIZE 16
#define TWOFISH192_KEY_SIZE 24
#define TWOFISH256_KEY_SIZE 32

struct twofish_ctx
{
  uint32_t keys[40];
  uint32_t s_box[4][256];
};

void
twofish_set_key(struct twofish_ctx *ctx,
		size_t length, const uint8_t *key);
void
twofish128_set_key(struct twofish_ctx *context, const uint8_t *key);
void
twofish192_set_key(struct twofish_ctx *context, const uint8_t *key);
void
twofish256_set_key(struct twofish_ctx *context, const uint8_t *key);

void
twofish_encrypt(const struct twofish_ctx *ctx,
		size_t length, uint8_t *dst,
		const uint8_t *src);
void
twofish_decrypt(const struct twofish_ctx *ctx,
		size_t length, uint8_t *dst,
		const uint8_t *src);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_TWOFISH_H_INCLUDED */
