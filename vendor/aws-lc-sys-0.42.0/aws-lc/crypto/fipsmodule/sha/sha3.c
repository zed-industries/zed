 // Copyright 2017-2020 The OpenSSL Project Authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#include "internal.h"
#include <string.h>

uint8_t *SHA3_224(const uint8_t *data, size_t len,
                  uint8_t out[SHA3_224_DIGEST_LENGTH]) {
  FIPS_service_indicator_lock_state();
  KECCAK1600_CTX ctx;
  int ok = (SHA3_Init(&ctx, SHA3_224_DIGEST_BITLENGTH) &&
            SHA3_Update(&ctx, data, len) &&
            SHA3_Final(out, &ctx));

  OPENSSL_cleanse(&ctx, sizeof(ctx));
  FIPS_service_indicator_unlock_state();
  if (ok == 0) {
    return NULL;
  }
  FIPS_service_indicator_update_state();
  return out;
}

uint8_t *SHA3_256(const uint8_t *data, size_t len,
                  uint8_t out[SHA3_256_DIGEST_LENGTH]) {
  FIPS_service_indicator_lock_state();
  KECCAK1600_CTX ctx;
  int ok = (SHA3_Init(&ctx, SHA3_256_DIGEST_BITLENGTH) &&
            SHA3_Update(&ctx, data, len) &&
            SHA3_Final(out, &ctx));

  OPENSSL_cleanse(&ctx, sizeof(ctx));
  FIPS_service_indicator_unlock_state();
  if (ok == 0) {
    return NULL;
  }
  FIPS_service_indicator_update_state();
  return out;
}

uint8_t *SHA3_384(const uint8_t *data, size_t len,
                  uint8_t out[SHA3_384_DIGEST_LENGTH]) {
  FIPS_service_indicator_lock_state();
  KECCAK1600_CTX ctx;
  int ok = (SHA3_Init(&ctx, SHA3_384_DIGEST_BITLENGTH) &&
            SHA3_Update(&ctx, data, len) &&
            SHA3_Final(out, &ctx));

  OPENSSL_cleanse(&ctx, sizeof(ctx));
  FIPS_service_indicator_unlock_state();
  if (ok == 0) {
    return NULL;
  }
  FIPS_service_indicator_update_state();
  return out;
}

uint8_t *SHA3_512(const uint8_t *data, size_t len,
                  uint8_t out[SHA3_512_DIGEST_LENGTH]) {
  FIPS_service_indicator_lock_state();
  KECCAK1600_CTX ctx;
  int ok = (SHA3_Init(&ctx, SHA3_512_DIGEST_BITLENGTH) &&
            SHA3_Update(&ctx, data, len) &&
            SHA3_Final(out, &ctx));

  OPENSSL_cleanse(&ctx, sizeof(ctx));
  FIPS_service_indicator_unlock_state();
  if (ok == 0) {
    return NULL;
  }
  FIPS_service_indicator_update_state();
  return out;
}

uint8_t *SHAKE128(const uint8_t *data, const size_t in_len, uint8_t *out, size_t out_len) {
  FIPS_service_indicator_lock_state();
  KECCAK1600_CTX ctx;
  int ok = (SHAKE_Init(&ctx, SHAKE128_BLOCKSIZE) &&
            SHAKE_Absorb(&ctx, data, in_len) &&
            SHAKE_Final(out, &ctx, out_len));

  OPENSSL_cleanse(&ctx, sizeof(ctx));
  FIPS_service_indicator_unlock_state();
  if (ok == 0) {
    return NULL;
  }
  FIPS_service_indicator_update_state();
  return out;
}

uint8_t *SHAKE256(const uint8_t *data, const size_t in_len, uint8_t *out, size_t out_len) {
  FIPS_service_indicator_lock_state();
  KECCAK1600_CTX ctx;
  int ok = (SHAKE_Init(&ctx, SHAKE256_BLOCKSIZE) &&
            SHAKE_Absorb(&ctx, data, in_len) &&
            SHAKE_Final(out, &ctx, out_len));
  OPENSSL_cleanse(&ctx, sizeof(ctx));
  FIPS_service_indicator_unlock_state();
  if (ok == 0) {
    return NULL;
  }
  FIPS_service_indicator_update_state();
  return out;
}

/*
 * FIPS202 APIs manage internal input/output buffer on top of Keccak1600 API layer
 */
// FIPS202_Reset zero's |ctx| fields.
static void FIPS202_Reset(KECCAK1600_CTX *ctx) {
  OPENSSL_memset(ctx->A, 0, sizeof(ctx->A));
  ctx->buf_load = 0;
  ctx->state = KECCAK1600_STATE_ABSORB;
}

// FIPS202_Init checks the correctness of the padding character and size of
// the internal buffer. It initialises the |ctx| fields and returns 1 on
// success and 0 on failure.
static int FIPS202_Init(KECCAK1600_CTX *ctx, uint8_t pad, size_t block_size, size_t bit_len) {
  if (pad != SHA3_PAD_CHAR && 
      pad != SHAKE_PAD_CHAR) { 
    return 0;
  }
      
  if (block_size <= sizeof(ctx->buf)) {
      FIPS202_Reset(ctx);
      ctx->block_size = block_size;
      ctx->md_size = bit_len / 8;
      ctx->pad = pad;
      return 1;
    }
    return 0;
}

// FIPS202_Update checks the state of the |ctx| and processes intermediate buffer from
// previous calls. It processes |data| in blocks through |Keccak1600_Absorb| and places
// the rest in the intermediate buffer. FIPS202_Update fails if called from inappropriate
// |ctx->state| or on |Keccak1600_Absorb| error. Otherwise, it returns 1.
static int FIPS202_Update(KECCAK1600_CTX *ctx, const void *data, size_t len) {
  uint8_t *data_ptr_copy = (uint8_t *) data;
  size_t block_size = ctx->block_size;
  size_t num, rem;

  if (ctx->state == KECCAK1600_STATE_SQUEEZE ||
      ctx->state == KECCAK1600_STATE_FINAL ) {
    return 0;
  }

  // Case |len| equals 0 is checked in SHA3/SHAKE higher level APIs
  // Process intermediate buffer.
  num = ctx->buf_load;
  if (num != 0) {
    rem = block_size - num;
    if (len < rem) {
      OPENSSL_memcpy(ctx->buf + num, data_ptr_copy, len);
      ctx->buf_load += len;
      return 1;
    }

    // There is enough data to fill or overflow the intermediate
    // buffer. So we append |rem| bytes and process the block,
    // leaving the rest for later processing.
    OPENSSL_memcpy(ctx->buf + num, data_ptr_copy, rem);
    data_ptr_copy += rem, len -= rem;
    if (Keccak1600_Absorb(ctx->A, ctx->buf, block_size, block_size) != 0 ) {
      return 0;
    }
    ctx->buf_load = 0;
    // ctx->buf is processed, ctx->buf_load is guaranteed to be zero
  }

  if (len >= block_size) {
    rem = Keccak1600_Absorb(ctx->A, data_ptr_copy, len, block_size);
  }
  else {
    rem = len;
  }

  if (rem != 0) {
    OPENSSL_memcpy(ctx->buf, data_ptr_copy + len - rem, rem);
    ctx->buf_load = rem;
  }

  return 1;
}

// FIPS202_Finalize processes padding and absorb of last input block
// This function should be called once to finalize absorb and initiate
// squeeze phase. FIPS202_Finalize fails if called from inappropriate
// |ctx->state| or on |Keccak1600_Absorb| error. Otherwise, it returns 1.
static int FIPS202_Finalize(uint8_t *md, KECCAK1600_CTX *ctx) {
  size_t block_size = ctx->block_size;
  size_t num = ctx->buf_load;

  if (ctx->state == KECCAK1600_STATE_SQUEEZE || 
      ctx->state == KECCAK1600_STATE_FINAL ) {
    return 0;
  }

  // Pad the data with 10*1. Note that |num| can be |block_size - 1|
  // in which case both byte operations below are performed on
  // the same byte.
  OPENSSL_memset(ctx->buf + num, 0, block_size - num);
  ctx->buf[num] = ctx->pad;
  ctx->buf[block_size - 1] |= 0x80;

  if (Keccak1600_Absorb(ctx->A, ctx->buf, block_size, block_size) != 0) {
    return 0;
  }
  
  // ctx->buf is processed, ctx->buf_load is guaranteed to be zero
  ctx->buf_load = 0;

  return 1;
}

/*
 * SHA3 APIs implement SHA3 functionalities on top of FIPS202 API layer
 */
int SHA3_Init(KECCAK1600_CTX *ctx, size_t bit_len) {
  if (ctx == NULL) {
    return 0;
  }

  if (bit_len != SHA3_224_DIGEST_BITLENGTH && 
      bit_len != SHA3_256_DIGEST_BITLENGTH && 
      bit_len != SHA3_384_DIGEST_BITLENGTH && 
      bit_len != SHA3_512_DIGEST_BITLENGTH) {
        return 0;
  }
  // |block_size| depends on the SHA3 |bit_len| output (digest) length
  return FIPS202_Init(ctx, SHA3_PAD_CHAR, SHA3_BLOCKSIZE(bit_len), bit_len);
}

int SHA3_Update(KECCAK1600_CTX *ctx, const void *data, size_t len) {
  if (ctx == NULL) {
    return 0;
  }

  if (data == NULL && len != 0) {
    return 0;
  }

  if (len == 0) {
    return 1;
  }

  return FIPS202_Update(ctx, data, len);
}

// SHA3_Final should be called once to process final digest value
int SHA3_Final(uint8_t *md, KECCAK1600_CTX *ctx) {
  if (md == NULL || ctx == NULL) {
    return 0;
  }

  if (ctx->md_size == 0) {
    return 1;
  }

  if (FIPS202_Finalize(md, ctx) == 0) {
    return 0;
  }

  Keccak1600_Squeeze(ctx->A, md, ctx->md_size, ctx->block_size, ctx->state);
  ctx->state = KECCAK1600_STATE_FINAL;

  FIPS_service_indicator_update_state();
  return 1;
}

int SHA3_224_Init(KECCAK1600_CTX *ctx) {
    return SHA3_Init(ctx, SHA3_224_DIGEST_BITLENGTH);
}

int SHA3_224_Update(KECCAK1600_CTX *ctx, const void *data,
                                     size_t len) {
    return SHA3_Update(ctx, data, len);
}

int SHA3_224_Final(uint8_t out[SHA3_224_DIGEST_LENGTH],
                                    KECCAK1600_CTX *ctx) {
    return SHA3_Final(&out[0], ctx);
}

int SHA3_256_Init(KECCAK1600_CTX *ctx) {
    return SHA3_Init(ctx, SHA3_256_DIGEST_BITLENGTH);
}

int SHA3_256_Update(KECCAK1600_CTX *ctx, const void *data,
                                     size_t len) {
    return SHA3_Update(ctx, data, len);
}

int SHA3_256_Final(uint8_t out[SHA3_256_DIGEST_LENGTH],
                                    KECCAK1600_CTX *ctx) {
    return SHA3_Final(&out[0], ctx);
}

int SHA3_384_Init(KECCAK1600_CTX *ctx) {
    return SHA3_Init(ctx, SHA3_384_DIGEST_BITLENGTH);
}

int SHA3_384_Update(KECCAK1600_CTX *ctx, const void *data,
                                     size_t len) {
    return SHA3_Update(ctx, data, len);
}

int SHA3_384_Final(uint8_t out[SHA3_384_DIGEST_LENGTH],
                                    KECCAK1600_CTX *ctx) {
    return SHA3_Final(&out[0], ctx);
}

int SHA3_512_Init(KECCAK1600_CTX *ctx) {
    return SHA3_Init(ctx, SHA3_512_DIGEST_BITLENGTH);
}

int SHA3_512_Update(KECCAK1600_CTX *ctx, const void *data,
                                     size_t len) {
    return SHA3_Update(ctx, data, len);
}

int SHA3_512_Final(uint8_t out[SHA3_512_DIGEST_LENGTH],
                                    KECCAK1600_CTX *ctx) {
    return SHA3_Final(&out[0], ctx);
}

/*
 * SHAKE APIs implement SHAKE functionalities on top of FIPS202 API layer
 */
int SHAKE_Init(KECCAK1600_CTX *ctx, size_t block_size) {
  if (ctx == NULL) {
    return 0;
  }

  if (block_size != SHAKE128_BLOCKSIZE &&
      block_size != SHAKE256_BLOCKSIZE) {
        return 0;
  }
  // |block_size| depends on the SHAKE security level
  // The output length |bit_len| is initialized to 0
  return FIPS202_Init(ctx, SHAKE_PAD_CHAR, block_size, 0);
}

int SHAKE_Absorb(KECCAK1600_CTX *ctx, const void *data, size_t len) {
  if (ctx == NULL) {
    return 0;
  }

  if (data == NULL && len != 0) {
    return 0;
  }

  if (len == 0) {
    return 1;
  }

  return FIPS202_Update(ctx, data, len);
}

// SHAKE_Final is to be called once to finalize absorb and squeeze phases
// |ctx->state| restricts consecutive calls to |FIPS202_Finalize|.
// Function |SHAKE_Squeeze| should be used for incremental XOF output.
int SHAKE_Final(uint8_t *md, KECCAK1600_CTX *ctx, size_t len) {
  if (ctx == NULL || md == NULL) {
    return 0;
  }

  ctx->md_size = len;
  if (ctx->md_size == 0) {
    return 1;
  }

  if (FIPS202_Finalize(md, ctx) == 0) {
    return 0;
  }

  Keccak1600_Squeeze(ctx->A, md, ctx->md_size, ctx->block_size, ctx->state);
  ctx->state = KECCAK1600_STATE_FINAL;

  FIPS_service_indicator_update_state();
  return 1;
}

// SHAKE_Squeeze can be called multiple time for incremental XOF output
int SHAKE_Squeeze(uint8_t *md, KECCAK1600_CTX *ctx, size_t len) {
  size_t block_bytes;

  if (ctx == NULL || md == NULL) {
    return 0;
  }

  ctx->md_size = len;

  if (ctx->md_size == 0) {
    return 1;
  }

  if (ctx->state == KECCAK1600_STATE_FINAL) {
    return 0;
  }

  // Skip FIPS202_Finalize if the input has been padded and
  // the last block has been processed
  if (ctx->state == KECCAK1600_STATE_ABSORB) {
    if (FIPS202_Finalize(md, ctx) == 0) {
      return 0;
    }
  }
  // Process previous data from output buffer if any
  if (ctx->buf_load != 0) {
    if (len <= ctx->buf_load) {
      OPENSSL_memcpy(md, ctx->buf + ctx->block_size - ctx->buf_load, len);
      ctx->buf_load -= len;
      return 1;
    } else {
      OPENSSL_memcpy(md, ctx->buf + ctx->block_size - ctx->buf_load, ctx->buf_load);
      md += ctx->buf_load;
      len -= ctx->buf_load;
      ctx->buf_load = 0;
    }
  }

  // Process all full size output requested blocks
  if (len > ctx->block_size) {
    block_bytes = ctx->block_size * (len / ctx->block_size);
    Keccak1600_Squeeze(ctx->A, md, block_bytes, ctx->block_size, ctx->state);
    md += block_bytes;
    len -= block_bytes;
    ctx->state = KECCAK1600_STATE_SQUEEZE;
  }

  if (len > 0) {
    // Process an additional block if output length is not a multiple of block size.
    // Generated output is store in |ctx->buf|. Only requested bytes are transfered
    // to the output. The 'unused' output data is kept for processing in a sequenctual
    // call to SHAKE_Squeeze (incremental byte-wise SHAKE_Squeeze)
    Keccak1600_Squeeze(ctx->A, ctx->buf, ctx->block_size, ctx->block_size, ctx->state);
    OPENSSL_memcpy(md, ctx->buf, len);
    ctx->buf_load = ctx->block_size - len; // how much there is still in buffer to be consumed
    ctx->state = KECCAK1600_STATE_SQUEEZE;
  }

  //FIPS_service_indicator_update_state();
  return 1;
}

/*
 * SHAKE batched (x4) APIs implement SHAKE functionalities in batches of four on top of SHAKE API layer
 */
int SHAKE128_Init_x4(KECCAK1600_CTX_x4 *ctx) {
  OPENSSL_memset(ctx, 0, sizeof(KECCAK1600_CTX_x4));
  return 1;
}

int SHAKE128_Absorb_once_x4(KECCAK1600_CTX_x4 *ctx, const void *data0, const void *data1,
                                  const void *data2, const void *data3, size_t len) {
  Keccak1600_Absorb_once_x4(ctx->A, data0, data1, data2, data3, len,
                            SHAKE128_BLOCKSIZE, SHAKE_PAD_CHAR);
  return 1;
}

int SHAKE128_Squeezeblocks_x4(uint8_t *md0, uint8_t *md1, uint8_t *md2, uint8_t *md3,
                                  KECCAK1600_CTX_x4 *ctx, size_t blks) {
  Keccak1600_Squeezeblocks_x4(ctx->A, md0, md1, md2, md3, blks, SHAKE128_BLOCKSIZE);
  return 1;
}

static int SHAKE256_Absorb_once_x4(KECCAK1600_CTX_x4 *ctx, const void *data0, const void *data1,
                                  const void *data2, const void *data3, size_t len) {
  Keccak1600_Absorb_once_x4(ctx->A, data0, data1, data2, data3,
                            len, SHAKE256_BLOCKSIZE, SHAKE_PAD_CHAR);
  return 1;
}

static int SHAKE256_Squeezeblocks_x4(uint8_t *md0, uint8_t *md1, uint8_t *md2, uint8_t *md3,
                                  KECCAK1600_CTX_x4 *ctx, size_t blks) {
  Keccak1600_Squeezeblocks_x4(ctx->A, md0, md1, md2, md3, blks, SHAKE256_BLOCKSIZE);
  return 1;
}

int SHAKE256_x4(const uint8_t *data0, const uint8_t *data1, const uint8_t *data2,
                                  const uint8_t *data3, const size_t in_len,
                                  uint8_t *out0, uint8_t *out1, uint8_t *out2,
                                  uint8_t *out3, size_t out_len) {
  KECCAK1600_CTX_x4 ctx;
  OPENSSL_memset(&ctx, 0, sizeof(ctx));
  size_t nblocks = out_len / SHAKE256_BLOCKSIZE;
  uint8_t tmp0[SHAKE256_BLOCKSIZE];
  uint8_t tmp1[SHAKE256_BLOCKSIZE];
  uint8_t tmp2[SHAKE256_BLOCKSIZE];
  uint8_t tmp3[SHAKE256_BLOCKSIZE];

  SHAKE256_Absorb_once_x4(&ctx, data0, data1, data2, data3, in_len);
  SHAKE256_Squeezeblocks_x4(out0, out1, out2, out3, &ctx, nblocks);

  out0 += nblocks * SHAKE256_BLOCKSIZE;
  out1 += nblocks * SHAKE256_BLOCKSIZE;
  out2 += nblocks * SHAKE256_BLOCKSIZE;
  out3 += nblocks * SHAKE256_BLOCKSIZE;

  out_len -= nblocks * SHAKE256_BLOCKSIZE;

  if (out_len > 0)
  {
      SHAKE256_Squeezeblocks_x4(tmp0, tmp1, tmp2, tmp3, &ctx, 1);
      OPENSSL_memcpy(out0, tmp0, out_len);
      OPENSSL_memcpy(out1, tmp1, out_len);
      OPENSSL_memcpy(out2, tmp2, out_len);
      OPENSSL_memcpy(out3, tmp3, out_len);
  }

  OPENSSL_cleanse(tmp0, sizeof(tmp0));
  OPENSSL_cleanse(tmp1, sizeof(tmp1));
  OPENSSL_cleanse(tmp2, sizeof(tmp2));
  OPENSSL_cleanse(tmp3, sizeof(tmp3));
  OPENSSL_cleanse(&ctx, sizeof(ctx));

  return 1;
}
