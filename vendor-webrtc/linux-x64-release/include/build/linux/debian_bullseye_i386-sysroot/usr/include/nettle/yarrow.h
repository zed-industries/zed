/* yarrow.h

   The yarrow pseudo-randomness generator.

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
 
#ifndef NETTLE_YARROW_H_INCLUDED
#define NETTLE_YARROW_H_INCLUDED

#include "aes.h"
#include "sha2.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Name mangling */
#define yarrow256_init nettle_yarrow256_init
#define yarrow256_seed nettle_yarrow256_seed
#define yarrow256_update nettle_yarrow256_update
#define yarrow256_random nettle_yarrow256_random
#define yarrow256_is_seeded nettle_yarrow256_is_seeded
#define yarrow256_needed_sources nettle_yarrow256_needed_sources
#define yarrow256_fast_reseed nettle_yarrow256_fast_reseed
#define yarrow256_slow_reseed nettle_yarrow256_slow_reseed
#define yarrow_key_event_init nettle_yarrow_key_event_init
#define yarrow_key_event_estimate nettle_yarrow_key_event_estimate

/* Obsolete alias for backwards compatibility. Will be deleted in some
   later version. */
#define yarrow256_force_reseed yarrow256_slow_reseed
  
enum yarrow_pool_id { YARROW_FAST = 0, YARROW_SLOW = 1 };

struct yarrow_source
{
  /* Indexed by yarrow_pool_id */
  uint32_t estimate[2];
  
  /* The pool next sample should go to. */
  enum yarrow_pool_id next;
};


#define YARROW256_SEED_FILE_SIZE (2 * AES_BLOCK_SIZE)

/* Yarrow-256, based on SHA-256 and AES-256 */
struct yarrow256_ctx
{
  /* Indexed by yarrow_pool_id */
  struct sha256_ctx pools[2];

  int seeded;

  /* The current key and counter block */
  struct aes256_ctx key;
  uint8_t counter[AES_BLOCK_SIZE];

  /* The entropy sources */
  unsigned nsources;
  struct yarrow_source *sources;
};

void
yarrow256_init(struct yarrow256_ctx *ctx,
	       unsigned nsources,
	       struct yarrow_source *sources);

void
yarrow256_seed(struct yarrow256_ctx *ctx,
	       size_t length,
	       const uint8_t *seed_file);

/* Returns 1 on reseed */
int
yarrow256_update(struct yarrow256_ctx *ctx,
		 unsigned source, unsigned entropy,
		 size_t length, const uint8_t *data);

void
yarrow256_random(struct yarrow256_ctx *ctx, size_t length, uint8_t *dst);

int
yarrow256_is_seeded(struct yarrow256_ctx *ctx);

unsigned
yarrow256_needed_sources(struct yarrow256_ctx *ctx);

void
yarrow256_fast_reseed(struct yarrow256_ctx *ctx);

void
yarrow256_slow_reseed(struct yarrow256_ctx *ctx);


/* Key event estimator */
#define YARROW_KEY_EVENT_BUFFER 16

struct yarrow_key_event_ctx
{
  /* Counter for initial priming of the state */
  unsigned index;
  unsigned chars[YARROW_KEY_EVENT_BUFFER];
  unsigned previous;
};

void
yarrow_key_event_init(struct yarrow_key_event_ctx *ctx);

unsigned
yarrow_key_event_estimate(struct yarrow_key_event_ctx *ctx,
			  unsigned key, unsigned time);
  
#ifdef __cplusplus
}
#endif

#endif /* NETTLE_YARROW_H_INCLUDED */
