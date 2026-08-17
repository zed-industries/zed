/* knuth-lfib.h

   The "lagged fibonacci" pseudorandomness generator, described in
   Knuth, TAoCP, 3.6

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

/* NOTE: This generator is totally inappropriate for cryptographic
 * applications. It is useful for generating deterministic but
 * random-looking test data, and is used by the Nettle testsuite. */
#ifndef NETTLE_KNUTH_LFIB_H_INCLUDED
#define NETTLE_KNUTH_LFIB_H_INCLUDED

#include "nettle-types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Namespace mangling */
#define knuth_lfib_init nettle_knuth_lfib_init
#define knuth_lfib_get nettle_knuth_lfib_get
#define knuth_lfib_get_array nettle_knuth_lfib_get_array
#define knuth_lfib_random nettle_knuth_lfib_random

#define _KNUTH_LFIB_KK 100

struct knuth_lfib_ctx
{
  uint32_t x[_KNUTH_LFIB_KK];
  unsigned index;
};

void
knuth_lfib_init(struct knuth_lfib_ctx *ctx, uint32_t seed);

/* Get's a single number in the range 0 ... 2^30-1 */
uint32_t
knuth_lfib_get(struct knuth_lfib_ctx *ctx);

/* Get an array of numbers */
void
knuth_lfib_get_array(struct knuth_lfib_ctx *ctx,
		     size_t n, uint32_t *a);

/* Get an array of octets. */
void
knuth_lfib_random(struct knuth_lfib_ctx *ctx,
		  size_t n, uint8_t *dst);

#ifdef __cplusplus
}
#endif

#endif /* NETTLE_KNUTH_LFIB_H_INCLUDED */
