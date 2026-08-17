// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/sha.h>

#include <vector>

#include <gtest/gtest.h>

#include "../../test/abi_test.h"
#include "internal.h"

#if defined(SUPPORTS_ABI_TEST) && !defined(SHA1_ALTIVEC)
TEST(SHATest, SHA1ABI) {
  SHA_CTX ctx;
  SHA1_Init(&ctx);

  static const uint8_t kBuf[SHA_CBLOCK * 8] = {0};
  for (size_t blocks : {1, 2, 4, 8}) {
#if defined(SHA1_ASM)
    CHECK_ABI(sha1_block_data_order, ctx.h, kBuf, blocks);
#endif
#if defined(SHA1_ASM_HW)
    if (sha1_hw_capable()) {
      CHECK_ABI(sha1_block_data_order_hw, ctx.h, kBuf, blocks);
    }
#endif
#if defined(SHA1_ASM_AVX2) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
    if (sha1_avx2_capable()) {
      CHECK_ABI(sha1_block_data_order_avx2, ctx.h, kBuf, blocks);
    }
#endif
#if defined(SHA1_ASM_AVX) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
    if (sha1_avx_capable()) {
      CHECK_ABI(sha1_block_data_order_avx, ctx.h, kBuf, blocks);
    }
#endif
#if defined(SHA1_ASM_SSSE3)
    if (sha1_ssse3_capable()) {
      CHECK_ABI(sha1_block_data_order_ssse3, ctx.h, kBuf, blocks);
    }
#endif
#if defined(SHA1_ASM_NEON)
    if (CRYPTO_is_NEON_capable()) {
      CHECK_ABI(sha1_block_data_order_neon, ctx.h, kBuf, blocks);
    }
#endif
#if defined(SHA1_ASM_NOHW)
    CHECK_ABI(sha1_block_data_order_nohw, ctx.h, kBuf, blocks);
#endif
  }
}

TEST(SHATest, SHA256ABI) {
  SHA256_CTX ctx;
  SHA256_Init(&ctx);

  static const uint8_t kBuf[SHA256_CBLOCK * 8] = {0};
  for (size_t blocks : {1, 2, 4, 8}) {
#if defined(SHA256_ASM)
    CHECK_ABI(sha256_block_data_order, ctx.h, kBuf, blocks);
#endif
#if defined(SHA256_ASM_HW)
    if (sha256_hw_capable()) {
      CHECK_ABI(sha256_block_data_order_hw, ctx.h, kBuf, blocks);
    }
#endif
#if defined(SHA256_ASM_AVX) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
    if (sha256_avx_capable()) {
      CHECK_ABI(sha256_block_data_order_avx, ctx.h, kBuf, blocks);
    }
#endif
#if defined(SHA256_ASM_SSSE3)
    if (sha256_ssse3_capable()) {
      CHECK_ABI(sha256_block_data_order_ssse3, ctx.h, kBuf, blocks);
    }
#endif
#if defined(SHA256_ASM_NEON)
    if (CRYPTO_is_NEON_capable()) {
      CHECK_ABI(sha256_block_data_order_neon, ctx.h, kBuf, blocks);
    }
#endif
#if defined(SHA256_ASM_NOHW)
    CHECK_ABI(sha256_block_data_order_nohw, ctx.h, kBuf, blocks);
#endif
  }
}

TEST(SHATest, SHA512ABI) {
  SHA512_CTX ctx;
  SHA512_Init(&ctx);

  static const uint8_t kBuf[SHA512_CBLOCK * 4] = {0};
  for (size_t blocks : {1, 2, 3, 4}) {
#if defined(SHA512_ASM)
    CHECK_ABI(sha512_block_data_order, ctx.h, kBuf, blocks);
#endif
#if defined(SHA512_ASM_HW)
    if (sha512_hw_capable()) {
      CHECK_ABI(sha512_block_data_order_hw, ctx.h, kBuf, blocks);
    }
#endif
#if defined(SHA512_ASM_AVX) && !defined(MY_ASSEMBLER_IS_TOO_OLD_FOR_AVX)
    if (sha512_avx_capable()) {
      CHECK_ABI(sha512_block_data_order_avx, ctx.h, kBuf, blocks);
    }
#endif
#if defined(SHA512_ASM_NEON)
    if (CRYPTO_is_NEON_capable()) {
      CHECK_ABI(sha512_block_data_order_neon, ctx.h, kBuf, blocks);
    }
#endif
#if defined(SHA512_ASM_NOHW)
    CHECK_ABI(sha512_block_data_order_nohw, ctx.h, kBuf, blocks);
#endif
  }
}

#endif // defined(SUPPORTS_ABI_TEST) && !defined(SHA1_ALTIVEC)
