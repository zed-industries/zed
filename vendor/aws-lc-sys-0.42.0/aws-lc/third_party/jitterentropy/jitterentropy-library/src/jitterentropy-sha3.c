/* Jitter RNG: SHA-3 Implementation
 *
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

#include "jitterentropy-sha3.h"
#include "jitterentropy-internal.h"

/***************************************************************************
 * Message Digest Implementation
 ***************************************************************************/

/*
 * Conversion of Little-Endian representations in byte streams - the data
 * representation in the integer values is the host representation.
 */
static inline uint32_t ptr_to_le32(const uint8_t *p)
{
	return (uint32_t)p[0]       | (uint32_t)p[1] << 8 |
	       (uint32_t)p[2] << 16 | (uint32_t)p[3] << 24;
}

static inline uint64_t ptr_to_le64(const uint8_t *p)
{
	return (uint64_t)ptr_to_le32(p) | (uint64_t)ptr_to_le32(p + 4) << 32;
}

static inline void le32_to_ptr(uint8_t *p, const uint32_t value)
{
	p[0] = (uint8_t)(value);
	p[1] = (uint8_t)(value >> 8);
	p[2] = (uint8_t)(value >> 16);
	p[3] = (uint8_t)(value >> 24);
}

static inline void le64_to_ptr(uint8_t *p, const uint64_t value)
{
	le32_to_ptr(p + 4, (uint32_t)(value >> 32));
	le32_to_ptr(p,     (uint32_t)(value));
}

/*********************************** Keccak ***********************************/
/* state[x + y*5] */
#define A(x, y) (x + 5 * y)

static inline void jent_keccakp_theta(uint64_t s[25])
{
	uint64_t C[5], D[5];

	/* Step 1 */
	C[0] = s[A(0, 0)] ^ s[A(0, 1)] ^ s[A(0, 2)] ^ s[A(0, 3)] ^ s[A(0, 4)];
	C[1] = s[A(1, 0)] ^ s[A(1, 1)] ^ s[A(1, 2)] ^ s[A(1, 3)] ^ s[A(1, 4)];
	C[2] = s[A(2, 0)] ^ s[A(2, 1)] ^ s[A(2, 2)] ^ s[A(2, 3)] ^ s[A(2, 4)];
	C[3] = s[A(3, 0)] ^ s[A(3, 1)] ^ s[A(3, 2)] ^ s[A(3, 3)] ^ s[A(3, 4)];
	C[4] = s[A(4, 0)] ^ s[A(4, 1)] ^ s[A(4, 2)] ^ s[A(4, 3)] ^ s[A(4, 4)];

	/* Step 2 */
	D[0] = C[4] ^ rol64(C[1], 1);
	D[1] = C[0] ^ rol64(C[2], 1);
	D[2] = C[1] ^ rol64(C[3], 1);
	D[3] = C[2] ^ rol64(C[4], 1);
	D[4] = C[3] ^ rol64(C[0], 1);

	/* Step 3 */
	s[A(0, 0)] ^= D[0];
	s[A(1, 0)] ^= D[1];
	s[A(2, 0)] ^= D[2];
	s[A(3, 0)] ^= D[3];
	s[A(4, 0)] ^= D[4];

	s[A(0, 1)] ^= D[0];
	s[A(1, 1)] ^= D[1];
	s[A(2, 1)] ^= D[2];
	s[A(3, 1)] ^= D[3];
	s[A(4, 1)] ^= D[4];

	s[A(0, 2)] ^= D[0];
	s[A(1, 2)] ^= D[1];
	s[A(2, 2)] ^= D[2];
	s[A(3, 2)] ^= D[3];
	s[A(4, 2)] ^= D[4];

	s[A(0, 3)] ^= D[0];
	s[A(1, 3)] ^= D[1];
	s[A(2, 3)] ^= D[2];
	s[A(3, 3)] ^= D[3];
	s[A(4, 3)] ^= D[4];

	s[A(0, 4)] ^= D[0];
	s[A(1, 4)] ^= D[1];
	s[A(2, 4)] ^= D[2];
	s[A(3, 4)] ^= D[3];
	s[A(4, 4)] ^= D[4];
}

static inline void jent_keccakp_rho(uint64_t s[25])
{
	/* Step 1 */
	/* s[A(0, 0)] = s[A(0, 0)]; */

#define RHO_ROL(t)	(((t + 1) * (t + 2) / 2) % 64)
	/* Step 3 */
	s[A(1, 0)] = rol64(s[A(1, 0)], RHO_ROL(0));
	s[A(0, 2)] = rol64(s[A(0, 2)], RHO_ROL(1));
	s[A(2, 1)] = rol64(s[A(2, 1)], RHO_ROL(2));
	s[A(1, 2)] = rol64(s[A(1, 2)], RHO_ROL(3));
	s[A(2, 3)] = rol64(s[A(2, 3)], RHO_ROL(4));
	s[A(3, 3)] = rol64(s[A(3, 3)], RHO_ROL(5));
	s[A(3, 0)] = rol64(s[A(3, 0)], RHO_ROL(6));
	s[A(0, 1)] = rol64(s[A(0, 1)], RHO_ROL(7));
	s[A(1, 3)] = rol64(s[A(1, 3)], RHO_ROL(8));
	s[A(3, 1)] = rol64(s[A(3, 1)], RHO_ROL(9));
	s[A(1, 4)] = rol64(s[A(1, 4)], RHO_ROL(10));
	s[A(4, 4)] = rol64(s[A(4, 4)], RHO_ROL(11));
	s[A(4, 0)] = rol64(s[A(4, 0)], RHO_ROL(12));
	s[A(0, 3)] = rol64(s[A(0, 3)], RHO_ROL(13));
	s[A(3, 4)] = rol64(s[A(3, 4)], RHO_ROL(14));
	s[A(4, 3)] = rol64(s[A(4, 3)], RHO_ROL(15));
	s[A(3, 2)] = rol64(s[A(3, 2)], RHO_ROL(16));
	s[A(2, 2)] = rol64(s[A(2, 2)], RHO_ROL(17));
	s[A(2, 0)] = rol64(s[A(2, 0)], RHO_ROL(18));
	s[A(0, 4)] = rol64(s[A(0, 4)], RHO_ROL(19));
	s[A(4, 2)] = rol64(s[A(4, 2)], RHO_ROL(20));
	s[A(2, 4)] = rol64(s[A(2, 4)], RHO_ROL(21));
	s[A(4, 1)] = rol64(s[A(4, 1)], RHO_ROL(22));
	s[A(1, 1)] = rol64(s[A(1, 1)], RHO_ROL(23));
}

static inline void jent_keccakp_pi(uint64_t s[25])
{
	uint64_t t = s[A(4, 4)];

	/* Step 1 */
	/* s[A(0, 0)] = s[A(0, 0)]; */
	s[A(4, 4)] = s[A(1, 4)];
	s[A(1, 4)] = s[A(3, 1)];
	s[A(3, 1)] = s[A(1, 3)];
	s[A(1, 3)] = s[A(0, 1)];
	s[A(0, 1)] = s[A(3, 0)];
	s[A(3, 0)] = s[A(3, 3)];
	s[A(3, 3)] = s[A(2, 3)];
	s[A(2, 3)] = s[A(1, 2)];
	s[A(1, 2)] = s[A(2, 1)];
	s[A(2, 1)] = s[A(0, 2)];
	s[A(0, 2)] = s[A(1, 0)];
	s[A(1, 0)] = s[A(1, 1)];
	s[A(1, 1)] = s[A(4, 1)];
	s[A(4, 1)] = s[A(2, 4)];
	s[A(2, 4)] = s[A(4, 2)];
	s[A(4, 2)] = s[A(0, 4)];
	s[A(0, 4)] = s[A(2, 0)];
	s[A(2, 0)] = s[A(2, 2)];
	s[A(2, 2)] = s[A(3, 2)];
	s[A(3, 2)] = s[A(4, 3)];
	s[A(4, 3)] = s[A(3, 4)];
	s[A(3, 4)] = s[A(0, 3)];
	s[A(0, 3)] = s[A(4, 0)];
	s[A(4, 0)] = t;
}

static inline void jent_keccakp_chi(uint64_t s[25])
{
	uint64_t t0[5], t1[5];

	t0[0] = s[A(0, 0)];
	t0[1] = s[A(0, 1)];
	t0[2] = s[A(0, 2)];
	t0[3] = s[A(0, 3)];
	t0[4] = s[A(0, 4)];

	t1[0] = s[A(1, 0)];
	t1[1] = s[A(1, 1)];
	t1[2] = s[A(1, 2)];
	t1[3] = s[A(1, 3)];
	t1[4] = s[A(1, 4)];

	s[A(0, 0)] ^= ~s[A(1, 0)] & s[A(2, 0)];
	s[A(0, 1)] ^= ~s[A(1, 1)] & s[A(2, 1)];
	s[A(0, 2)] ^= ~s[A(1, 2)] & s[A(2, 2)];
	s[A(0, 3)] ^= ~s[A(1, 3)] & s[A(2, 3)];
	s[A(0, 4)] ^= ~s[A(1, 4)] & s[A(2, 4)];

	s[A(1, 0)] ^= ~s[A(2, 0)] & s[A(3, 0)];
	s[A(1, 1)] ^= ~s[A(2, 1)] & s[A(3, 1)];
	s[A(1, 2)] ^= ~s[A(2, 2)] & s[A(3, 2)];
	s[A(1, 3)] ^= ~s[A(2, 3)] & s[A(3, 3)];
	s[A(1, 4)] ^= ~s[A(2, 4)] & s[A(3, 4)];

	s[A(2, 0)] ^= ~s[A(3, 0)] & s[A(4, 0)];
	s[A(2, 1)] ^= ~s[A(3, 1)] & s[A(4, 1)];
	s[A(2, 2)] ^= ~s[A(3, 2)] & s[A(4, 2)];
	s[A(2, 3)] ^= ~s[A(3, 3)] & s[A(4, 3)];
	s[A(2, 4)] ^= ~s[A(3, 4)] & s[A(4, 4)];

	s[A(3, 0)] ^= ~s[A(4, 0)] & t0[0];
	s[A(3, 1)] ^= ~s[A(4, 1)] & t0[1];
	s[A(3, 2)] ^= ~s[A(4, 2)] & t0[2];
	s[A(3, 3)] ^= ~s[A(4, 3)] & t0[3];
	s[A(3, 4)] ^= ~s[A(4, 4)] & t0[4];

	s[A(4, 0)] ^= ~t0[0] & t1[0];
	s[A(4, 1)] ^= ~t0[1] & t1[1];
	s[A(4, 2)] ^= ~t0[2] & t1[2];
	s[A(4, 3)] ^= ~t0[3] & t1[3];
	s[A(4, 4)] ^= ~t0[4] & t1[4];
}

static const uint64_t jent_keccakp_iota_vals[] = {
	0x0000000000000001ULL, 0x0000000000008082ULL, 0x800000000000808aULL,
	0x8000000080008000ULL, 0x000000000000808bULL, 0x0000000080000001ULL,
	0x8000000080008081ULL, 0x8000000000008009ULL, 0x000000000000008aULL,
	0x0000000000000088ULL, 0x0000000080008009ULL, 0x000000008000000aULL,
	0x000000008000808bULL, 0x800000000000008bULL, 0x8000000000008089ULL,
	0x8000000000008003ULL, 0x8000000000008002ULL, 0x8000000000000080ULL,
	0x000000000000800aULL, 0x800000008000000aULL, 0x8000000080008081ULL,
	0x8000000000008080ULL, 0x0000000080000001ULL, 0x8000000080008008ULL
};

static inline void jent_keccakp_iota(uint64_t s[25], unsigned int round)
{
	s[0] ^= jent_keccakp_iota_vals[round];
}

static inline void jent_keccakp_1600(uint64_t s[25])
{
	unsigned int round;

	for (round = 0; round < 24; round++) {
		jent_keccakp_theta(s);
		jent_keccakp_rho(s);
		jent_keccakp_pi(s);
		jent_keccakp_chi(s);
		jent_keccakp_iota(s, round);
	}
}

/*********************************** SHA-3 ************************************/

static inline void jent_sha3_init(struct jent_sha_ctx *ctx)
{
	unsigned int i;

	for (i = 0; i < 25; i++)
		ctx->state[i] = 0;
	ctx->msg_len = 0;
}

void jent_sha3_256_init(struct jent_sha_ctx *ctx)
{
	jent_sha3_init(ctx);
	ctx->r = JENT_SHA3_256_SIZE_BLOCK;
	ctx->rword = JENT_SHA3_256_SIZE_BLOCK / sizeof(uint64_t);
	ctx->digestsize = JENT_SHA3_256_SIZE_DIGEST;
}

static inline void jent_sha3_fill_state(struct jent_sha_ctx *ctx,
					const uint8_t *in)
{
	unsigned int i;

	for (i = 0; i < ctx->rword; i++) {
		ctx->state[i]  ^= ptr_to_le64(in);
		in += 8;
	}
}

void jent_sha3_update(struct jent_sha_ctx *ctx, const uint8_t *in, size_t inlen)
{
	size_t partial = ctx->msg_len % ctx->r;

	ctx->msg_len += inlen;

	/* Sponge absorbing phase */

	/* Check if we have a partial block stored */
	if (partial) {
		size_t todo = ctx->r - partial;

		/*
		 * If the provided data is small enough to fit in the partial
		 * buffer, copy it and leave it unprocessed.
		 */
		if (inlen < todo) {
			memcpy(ctx->partial + partial, in, inlen);
			return;
		}

		/*
		 * The input data is large enough to fill the entire partial
		 * block buffer. Thus, we fill it and transform it.
		 */
		memcpy(ctx->partial + partial, in, todo);
		inlen -= todo;
		in += todo;

		jent_sha3_fill_state(ctx, ctx->partial);
		jent_keccakp_1600(ctx->state);
	}

	/* Perform a transformation of full block-size messages */
	for (; inlen >= ctx->r; inlen -= ctx->r, in += ctx->r) {
		jent_sha3_fill_state(ctx, in);
		jent_keccakp_1600(ctx->state);
	}

	/* If we have data left, copy it into the partial block buffer */
	memcpy(ctx->partial, in, inlen);
}

void jent_sha3_final(struct jent_sha_ctx *ctx, uint8_t *digest)
{
	size_t partial = ctx->msg_len % ctx->r;
	unsigned int i;

	/* Final round in sponge absorbing phase */

	/* Fill the unused part of the partial buffer with zeros */
	memset(ctx->partial + partial, 0, ctx->r - partial);

	/*
	 * Add the leading and trailing bit as well as the 01 bits for the
	 * SHA-3 suffix.
	 */
	ctx->partial[partial] = 0x06;
	ctx->partial[ctx->r - 1] |= 0x80;

	/* Final transformation */
	jent_sha3_fill_state(ctx, ctx->partial);
	jent_keccakp_1600(ctx->state);

	/*
	 * Sponge squeeze phase - the digest size is always smaller as the
	 * state size r which implies we only have one squeeze round.
	 */
	for (i = 0; i < ctx->digestsize / 8; i++, digest += 8)
		le64_to_ptr(digest, ctx->state[i]);

	/* Add remaining 4 bytes if we use SHA3-224 */
	if (ctx->digestsize % 8)
		le32_to_ptr(digest, (uint32_t)(ctx->state[i]));

	memset(ctx->partial, 0, ctx->r);
	jent_sha3_init(ctx);
}

int jent_sha3_tester(void)
{
	HASH_CTX_ON_STACK(ctx);
	static const uint8_t msg_256[] = { 0x5E, 0x5E, 0xD6 };
	static const uint8_t exp_256[] = { 0xF1, 0x6E, 0x66, 0xC0, 0x43, 0x72,
					   0xB4, 0xA3, 0xE1, 0xE3, 0x2E, 0x07,
					   0xC4, 0x1C, 0x03, 0x40, 0x8A, 0xD5,
					   0x43, 0x86, 0x8C, 0xC4, 0x0E, 0xC5,
					   0x5E, 0x00, 0xBB, 0xBB, 0xBD, 0xF5,
					   0x91, 0x1E };
	uint8_t act[JENT_SHA3_256_SIZE_DIGEST] = { 0 };
	unsigned int i;

	jent_sha3_256_init(&ctx);
	jent_sha3_update(&ctx, msg_256, 3);
	jent_sha3_final(&ctx, act);

	for (i = 0; i < JENT_SHA3_256_SIZE_DIGEST; i++) {
		if (exp_256[i] != act[i])
			return 1;
	}

	return 0;
}

int jent_sha3_alloc(void **hash_state)
{
	struct sha_ctx *tmp;

	tmp = jent_zalloc(JENT_SHA_MAX_CTX_SIZE);
	if (!tmp)
		return 1;

	*hash_state = tmp;

	return 0;
}

void jent_sha3_dealloc(void *hash_state)
{
	struct sha_ctx *ctx = (struct sha_ctx *)hash_state;

	jent_zfree(ctx, JENT_SHA_MAX_CTX_SIZE);
}
