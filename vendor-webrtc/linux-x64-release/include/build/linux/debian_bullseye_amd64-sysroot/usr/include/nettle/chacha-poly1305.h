/* chacha-poly1305.h

   AEAD mechanism based on chacha and poly1305.
   See draft-agl-tls-chacha20poly1305-04.

   Copyright (C) 2014 Niels Möller

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

#ifndef NETTLE_CHACHA_POLY1305_H_INCLUDED
#define NETTLE_CHACHA_POLY1305_H_INCLUDED

#include "chacha.h"
#include "poly1305.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define chacha_poly1305_set_key nettle_chacha_poly1305_set_key
#define chacha_poly1305_set_nonce nettle_chacha_poly1305_set_nonce
#define chacha_poly1305_update nettle_chacha_poly1305_update
#define chacha_poly1305_decrypt nettle_chacha_poly1305_decrypt
#define chacha_poly1305_encrypt nettle_chacha_poly1305_encrypt
#define chacha_poly1305_digest nettle_chacha_poly1305_digest

#define CHACHA_POLY1305_BLOCK_SIZE 64
/* FIXME: Any need for 128-bit variant? */
#define CHACHA_POLY1305_KEY_SIZE 32
#define CHACHA_POLY1305_NONCE_SIZE CHACHA_NONCE96_SIZE
#define CHACHA_POLY1305_DIGEST_SIZE 16

struct chacha_poly1305_ctx
{
  struct chacha_ctx chacha;
  struct poly1305_ctx poly1305;
  union nettle_block16 s;
  uint64_t auth_size;
  uint64_t data_size;
  /* poly1305 block */
  uint8_t block[POLY1305_BLOCK_SIZE];
  unsigned index;
};

void
chacha_poly1305_set_key (struct chacha_poly1305_ctx *ctx,
			 const uint8_t *key);
void
chacha_poly1305_set_nonce (struct chacha_poly1305_ctx *ctx,
			   const uint8_t *nonce);

void
chacha_poly1305_update (struct chacha_poly1305_ctx *ctx,
			size_t length, const uint8_t *data);

void
chacha_poly1305_encrypt (struct chacha_poly1305_ctx *ctx,
			 size_t length, uint8_t *dst, const uint8_t *src);
			 
void
chacha_poly1305_decrypt (struct chacha_poly1305_ctx *ctx,
			 size_t length, uint8_t *dst, const uint8_t *src);
			 
void
chacha_poly1305_digest (struct chacha_poly1305_ctx *ctx,
			size_t length, uint8_t *digest);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_CHACHA_POLY1305_H_INCLUDED */
