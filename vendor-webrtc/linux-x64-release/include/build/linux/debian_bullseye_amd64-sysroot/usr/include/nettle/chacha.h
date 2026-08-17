/* chacha.h

   The ChaCha stream cipher.

   Copyright (C) 2013 Joachim Strömbergson
   Copyright (C) 2012 Simon Josefsson
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

#ifndef NETTLE_CHACHA_H_INCLUDED
#define NETTLE_CHACHA_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define chacha_set_key nettle_chacha_set_key
#define chacha_set_nonce nettle_chacha_set_nonce
#define chacha_set_nonce96 nettle_chacha_set_nonce96
#define chacha_set_counter nettle_chacha_set_counter
#define chacha_set_counter32 nettle_chacha_set_counter32
#define chacha_crypt nettle_chacha_crypt
#define chacha_crypt32 nettle_chacha_crypt32

/* Currently, only 256-bit keys are supported. */
#define CHACHA_KEY_SIZE 32
#define CHACHA_BLOCK_SIZE 64
#define CHACHA_NONCE_SIZE 8
#define CHACHA_NONCE96_SIZE 12
#define CHACHA_COUNTER_SIZE 8
#define CHACHA_COUNTER32_SIZE 4

#define _CHACHA_STATE_LENGTH 16

struct chacha_ctx
{
  /* Indices 0-3 holds a constant (SIGMA or TAU).
     Indices 4-11 holds the key.
     Indices 12-13 holds the block counter.
     Indices 14-15 holds the IV:

     This creates the state matrix:
     C C C C
     K K K K
     K K K K
     B B I I
  */
  uint32_t state[_CHACHA_STATE_LENGTH];
};

void
chacha_set_key(struct chacha_ctx *ctx, const uint8_t *key);

void
chacha_set_nonce(struct chacha_ctx *ctx, const uint8_t *nonce);

void
chacha_set_nonce96(struct chacha_ctx *ctx, const uint8_t *nonce);

void
chacha_set_counter(struct chacha_ctx *ctx, const uint8_t *counter);

void
chacha_set_counter32(struct chacha_ctx *ctx, const uint8_t *counter);

void
chacha_crypt(struct chacha_ctx *ctx, size_t length, 
             uint8_t *dst, const uint8_t *src);

void
chacha_crypt32(struct chacha_ctx *ctx, size_t length,
	       uint8_t *dst, const uint8_t *src);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_CHACHA_H_INCLUDED */
