/* base16.h
   
   Hex encoding and decoding, following spki conventions (i.e.
   allowing whitespace between digits).

   Copyright (C) 2002 Niels Möller

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
 
#ifndef NETTLE_BASE16_H_INCLUDED
#define NETTLE_BASE16_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define base16_encode_single nettle_base16_encode_single
#define base16_encode_update nettle_base16_encode_update
#define base16_decode_init nettle_base16_decode_init
#define base16_decode_single nettle_base16_decode_single
#define base16_decode_update nettle_base16_decode_update
#define base16_decode_final nettle_base16_decode_final

/* Base16 encoding */

/* Maximum length of output for base16_encode_update. */
#define BASE16_ENCODE_LENGTH(length) ((length) * 2)

/* Encodes a single byte. Always stores two digits in dst[0] and dst[1]. */
void
base16_encode_single(char *dst,
		     uint8_t src);

/* Always stores BASE16_ENCODE_LENGTH(length) digits in dst. */
void
base16_encode_update(char *dst,
		     size_t length,
		     const uint8_t *src);


/* Base16 decoding */

/* Maximum length of output for base16_decode_update. */
/* We have at most 4 buffered bits, and a total of (length + 1) * 4 bits. */
#define BASE16_DECODE_LENGTH(length) (((length) + 1) / 2)

struct base16_decode_ctx
{
  unsigned char word; /* Leftover bits */
  unsigned char bits; /* Number buffered bits */
};

void
base16_decode_init(struct base16_decode_ctx *ctx);

/* Decodes a single byte. Returns amount of output (0 or 1), or -1 on
 * errors. */
int
base16_decode_single(struct base16_decode_ctx *ctx,
		     uint8_t *dst,
		     char src);

/* Returns 1 on success, 0 on error. DST should point to an area of
 * size at least BASE16_DECODE_LENGTH(length). The amount of data
 * generated is returned in *DST_LENGTH. */

int
base16_decode_update(struct base16_decode_ctx *ctx,
		     size_t *dst_length,
		     uint8_t *dst,
		     size_t src_length,
		     const char *src);

/* Returns 1 on success. */
int
base16_decode_final(struct base16_decode_ctx *ctx);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_BASE16_H_INCLUDED */
