/* camellia.h

   Copyright (C) 2006,2007 NTT
   (Nippon Telegraph and Telephone Corporation).

   Copyright (C) 2010, 2013 Niels Möller

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

#ifndef NETTLE_CAMELLIA_H_INCLUDED
#define NETTLE_CAMELLIA_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define camellia128_set_encrypt_key nettle_camellia128_set_encrypt_key
#define camellia128_set_decrypt_key nettle_camellia_set_decrypt_key
#define camellia128_invert_key nettle_camellia128_invert_key
#define camellia128_crypt nettle_camellia128_crypt

#define camellia192_set_encrypt_key nettle_camellia192_set_encrypt_key
#define camellia192_set_decrypt_key nettle_camellia192_set_decrypt_key

#define camellia256_set_encrypt_key nettle_camellia256_set_encrypt_key
#define camellia256_set_decrypt_key nettle_camellia256_set_decrypt_key
#define camellia256_invert_key nettle_camellia256_invert_key
#define camellia256_crypt nettle_camellia256_crypt


#define CAMELLIA_BLOCK_SIZE 16
/* Valid key sizes are 128, 192 or 256 bits (16, 24 or 32 bytes) */
#define CAMELLIA128_KEY_SIZE 16
#define CAMELLIA192_KEY_SIZE 24
#define CAMELLIA256_KEY_SIZE 32

/* For 128-bit keys, there are 18 regular rounds, pre- and
   post-whitening, and two FL and FLINV rounds, using a total of 26
   subkeys, each of 64 bit. For 192- and 256-bit keys, there are 6
   additional regular rounds and one additional FL and FLINV, using a
   total of 34 subkeys. */
/* The clever combination of subkeys imply one of the pre- and
   post-whitening keys is folded with the round keys, so that subkey
   #1 and the last one (#25 or #33) is not used. The result is that we
   have only 24 or 32 subkeys at the end of key setup. */

#define _CAMELLIA128_NKEYS 24
#define _CAMELLIA256_NKEYS 32

struct camellia128_ctx
{
  uint64_t keys[_CAMELLIA128_NKEYS];
};

void
camellia128_set_encrypt_key(struct camellia128_ctx *ctx,
			    const uint8_t *key);

void
camellia128_set_decrypt_key(struct camellia128_ctx *ctx,
			    const uint8_t *key);

void
camellia128_invert_key(struct camellia128_ctx *dst,
		       const struct camellia128_ctx *src);

void
camellia128_crypt(const struct camellia128_ctx *ctx,
		  size_t length, uint8_t *dst,
		  const uint8_t *src);

struct camellia256_ctx
{
  uint64_t keys[_CAMELLIA256_NKEYS];
};

void
camellia256_set_encrypt_key(struct camellia256_ctx *ctx,
			    const uint8_t *key);

void
camellia256_set_decrypt_key(struct camellia256_ctx *ctx,
			    const uint8_t *key);

void
camellia256_invert_key(struct camellia256_ctx *dst,
		       const struct camellia256_ctx *src);

void
camellia256_crypt(const struct camellia256_ctx *ctx,
		  size_t length, uint8_t *dst,
		  const uint8_t *src);

/* camellia192 is the same as camellia256, except for the key
   schedule. */
/* Slightly ugly with a #define on a struct tag, since it might cause
   surprises if also used as a name of a variable. */
#define camellia192_ctx camellia256_ctx

void
camellia192_set_encrypt_key(struct camellia256_ctx *ctx,
			    const uint8_t *key);

void
camellia192_set_decrypt_key(struct camellia256_ctx *ctx,
			    const uint8_t *key);

#define camellia192_invert_key camellia256_invert_key
#define camellia192_crypt camellia256_crypt

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_CAMELLIA_H_INCLUDED */
