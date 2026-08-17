/* nettle-types.h

   Copyright (C) 2005, 2014 Niels Möller

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

#ifndef NETTLE_TYPES_H
#define NETTLE_TYPES_H

/* For size_t */
#include <stddef.h>
#include <stdint.h>

/* Attributes we want to use in installed header files, and hence
   can't rely on config.h. */
#ifdef __GNUC__

#define _NETTLE_ATTRIBUTE_PURE __attribute__((pure))
#ifndef _NETTLE_ATTRIBUTE_DEPRECATED
/* Variant without message is supported since gcc-3.1 or so. */
#define _NETTLE_ATTRIBUTE_DEPRECATED __attribute__((deprecated))
#endif

#else /* !__GNUC__ */

#define _NETTLE_ATTRIBUTE_PURE
#define _NETTLE_ATTRIBUTE_DEPRECATED

#endif /* !__GNUC__ */

#ifdef __cplusplus
extern "C" {
#endif

/* An aligned 16-byte block. */
union nettle_block16
{
  uint8_t b[16];
  unsigned long w[16 / sizeof(unsigned long)] _NETTLE_ATTRIBUTE_DEPRECATED;
  uint64_t u64[2];
};

union nettle_block8
{
  uint8_t b[8];
  uint64_t u64;
};

/* Randomness. Used by key generation and dsa signature creation. */
typedef void nettle_random_func(void *ctx,
				size_t length, uint8_t *dst);

/* Progress report function, mainly for key generation. */
typedef void nettle_progress_func(void *ctx, int c);

/* Realloc function, used by struct nettle_buffer. */
typedef void *nettle_realloc_func(void *ctx, void *p, size_t length);

/* Ciphers */
typedef void nettle_set_key_func(void *ctx, const uint8_t *key);

/* For block ciphers, const context. */
typedef void nettle_cipher_func(const void *ctx,
				size_t length, uint8_t *dst,
				const uint8_t *src);


/* Uses a void * for cipher contexts. Used for crypt operations where
   the internal state changes during the encryption. */
typedef void nettle_crypt_func(void *ctx,
			       size_t length, uint8_t *dst,
			       const uint8_t *src);

/* Hash algorithms */
typedef void nettle_hash_init_func(void *ctx);
typedef void nettle_hash_update_func(void *ctx,
				     size_t length,
				     const uint8_t *src);
typedef void nettle_hash_digest_func(void *ctx,
				     size_t length, uint8_t *dst);

/* ASCII armor codecs. NOTE: Experimental and subject to change. */

typedef size_t nettle_armor_length_func(size_t length);
typedef void nettle_armor_init_func(void *ctx);

typedef size_t nettle_armor_encode_update_func(void *ctx,
					       char *dst,
					       size_t src_length,
					       const uint8_t *src);

typedef size_t nettle_armor_encode_final_func(void *ctx, char *dst);

typedef int nettle_armor_decode_update_func(void *ctx,
					    size_t *dst_length,
					    uint8_t *dst,
					    size_t src_length,
					    const char *src);

typedef int nettle_armor_decode_final_func(void *ctx);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_TYPES_H */
