// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/md5.h>

#include <gtest/gtest.h>

#include "internal.h"
#include "../../test/abi_test.h"


#if defined(MD5_ASM) && defined(SUPPORTS_ABI_TEST)
TEST(MD5Test, ABI) {
  MD5_CTX ctx;
  MD5_Init(&ctx);

  static const uint8_t kBuf[MD5_CBLOCK * 8] = {0};
  CHECK_ABI(md5_block_asm_data_order, ctx.h, kBuf, 1);
  CHECK_ABI(md5_block_asm_data_order, ctx.h, kBuf, 2);
  CHECK_ABI(md5_block_asm_data_order, ctx.h, kBuf, 4);
  CHECK_ABI(md5_block_asm_data_order, ctx.h, kBuf, 8);
}
#endif  // MD5_ASM && SUPPORTS_ABI_TEST
