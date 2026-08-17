/* blowfish.h

   Blowfish block cipher.

   Copyright (C) 2014 Niels Möller
   Copyright (C) 1998, 2001 FSF, Ray Dassen, Niels Möller

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
 
#ifndef NETTLE_BLOWFISH_H_INCLUDED
#define NETTLE_BLOWFISH_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define blowfish_set_key nettle_blowfish_set_key
#define blowfish128_set_key nettle_blowfish128_set_key
#define blowfish_encrypt nettle_blowfish_encrypt
#define blowfish_decrypt nettle_blowfish_decrypt
#define blowfish_bcrypt_hash nettle_blowfish_bcrypt_hash
#define blowfish_bcrypt_verify nettle_blowfish_bcrypt_verify

#define BLOWFISH_BLOCK_SIZE 8

/* Variable key size between 64 and 448 bits. */
#define BLOWFISH_MIN_KEY_SIZE 8
#define BLOWFISH_MAX_KEY_SIZE 56

/* Default to 128 bits */
#define BLOWFISH_KEY_SIZE 16

#define BLOWFISH128_KEY_SIZE 16

#define _BLOWFISH_ROUNDS 16

#define BLOWFISH_BCRYPT_HASH_SIZE (60 + 1) /* Including null-terminator */
#define BLOWFISH_BCRYPT_BINSALT_SIZE 16    /* Binary string size */

struct blowfish_ctx
{
  uint32_t s[4][256];
  uint32_t p[_BLOWFISH_ROUNDS+2];
};

/* Returns 0 for weak keys, otherwise 1. */
int
blowfish_set_key(struct blowfish_ctx *ctx,
                 size_t length, const uint8_t *key);
int
blowfish128_set_key(struct blowfish_ctx *ctx, const uint8_t *key);

void
blowfish_encrypt(const struct blowfish_ctx *ctx,
                 size_t length, uint8_t *dst,
                 const uint8_t *src);
void
blowfish_decrypt(const struct blowfish_ctx *ctx,
                 size_t length, uint8_t *dst,
                 const uint8_t *src);

/* dst parameter must point to a buffer of minimally
 * BLOWFISH_BCRYPT_HASH_SIZE bytes */
int
blowfish_bcrypt_hash(uint8_t *dst,
                     size_t lenkey, const uint8_t *key,
                     size_t lenscheme, const uint8_t *scheme,
		     int log2rounds,
		     const uint8_t *salt);
int
blowfish_bcrypt_verify(size_t lenkey, const uint8_t *key,
                       size_t lenhashed, const uint8_t *hashed);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_BLOWFISH_H_INCLUDED */
