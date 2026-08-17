/* serpent.h

   The serpent block cipher.

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

/* Serpent is a 128-bit block cipher that accepts a key size of 256
 * bits, designed by Ross Anderson, Eli Biham, and Lars Knudsen. See
 * http://www.cl.cam.ac.uk/~rja14/serpent.html for details.
 */

#ifndef NETTLE_SERPENT_H_INCLUDED
#define NETTLE_SERPENT_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define serpent_set_key nettle_serpent_set_key
#define serpent128_set_key nettle_serpent128_set_key
#define serpent192_set_key nettle_serpent192_set_key
#define serpent256_set_key nettle_serpent256_set_key
#define serpent_encrypt nettle_serpent_encrypt
#define serpent_decrypt nettle_serpent_decrypt

#define SERPENT_BLOCK_SIZE 16

/* Other key lengths are possible, but the design of Serpent makes
 * smaller key lengths quite pointless; they cheated with the AES
 * requirements, using a 256-bit key length exclusively and just
 * padding it out if the desired key length was less, so there really
 * is no advantage to using key lengths less than 256 bits. */
#define SERPENT_KEY_SIZE 32

/* Allow keys of size 128 <= bits <= 256 */

#define SERPENT_MIN_KEY_SIZE 16
#define SERPENT_MAX_KEY_SIZE 32

#define SERPENT128_KEY_SIZE 16
#define SERPENT192_KEY_SIZE 24
#define SERPENT256_KEY_SIZE 32

struct serpent_ctx
{
  uint32_t keys[33][4];  /* key schedule */
};

void
serpent_set_key(struct serpent_ctx *ctx,
                size_t length, const uint8_t *key);
void
serpent128_set_key(struct serpent_ctx *ctx, const uint8_t *key);
void
serpent192_set_key(struct serpent_ctx *ctx, const uint8_t *key);
void
serpent256_set_key(struct serpent_ctx *ctx, const uint8_t *key);

void
serpent_encrypt(const struct serpent_ctx *ctx,
                size_t length, uint8_t *dst,
                const uint8_t *src);
void
serpent_decrypt(const struct serpent_ctx *ctx,
                size_t length, uint8_t *dst,
                const uint8_t *src);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_SERPENT_H_INCLUDED */
