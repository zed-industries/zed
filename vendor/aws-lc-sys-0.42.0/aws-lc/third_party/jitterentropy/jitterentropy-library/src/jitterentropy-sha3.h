/*
 * Copyright (C) 2021 - 2025, Stephan Mueller <smueller@chronox.de>
 *
 * License: see LICENSE file in root directory
 *
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE, ALL OF
 * WHICH ARE HEREBY DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
 * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR
 * BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
 * USE OF THIS SOFTWARE, EVEN IF NOT ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef JITTERENTROPY_SHA3_H
#define JITTERENTROPY_SHA3_H

#include "jitterentropy-internal.h"

#ifdef __cplusplus
extern "C"
{
#endif

#define JENT_SHA3_SIZE_BLOCK(bits)	((1600 - 2 * bits) >> 3)
#define JENT_SHA3_256_SIZE_BLOCK                                               \
	JENT_SHA3_SIZE_BLOCK(JENT_SHA3_256_SIZE_DIGEST_BITS)
#define JENT_SHA3_MAX_SIZE_BLOCK	JENT_SHA3_256_SIZE_BLOCK

struct jent_sha_ctx {
	uint64_t state[25];
	size_t msg_len;
	unsigned int r;
	unsigned int rword;
	unsigned int digestsize;
	uint8_t partial[JENT_SHA3_MAX_SIZE_BLOCK];
};

#define JENT_SHA_MAX_CTX_SIZE	(sizeof(struct jent_sha_ctx))
#define HASH_CTX_ON_STACK(name)						       \
	struct jent_sha_ctx name

void jent_sha3_256_init(struct jent_sha_ctx *ctx);
void jent_sha3_update(struct jent_sha_ctx *ctx, const uint8_t *in,
		      size_t inlen);
void jent_sha3_final(struct jent_sha_ctx *ctx, uint8_t *digest);
int jent_sha3_alloc(void **hash_state);
void jent_sha3_dealloc(void *hash_state);
int jent_sha3_tester(void);

#ifdef __cplusplus
}
#endif

#endif /* JITTERENTROPY_SHA3_H */
