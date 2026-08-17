// Copyright (c) 2014 The OpenSSL Project.  All rights reserved.
// Author: Emilia Kasper (emilia@openssl.org)
// Based on previous work by Bodo Moeller, Emilia Kasper, Adam Langley (Google).
// SPDX-License-Identifier: Apache-2.0
//
// Utilities for constant-time cryptography.
 
#include "internal.h"
#include "test/test_util.h"

#include <limits.h>
#include <stdio.h>
#include <stdlib.h>

#include <limits>

#include <gtest/gtest.h>

#include <openssl/mem.h>
#include <openssl/rand.h>


static uint8_t FromBool8(bool b) {
  return b ? CONSTTIME_TRUE_8 : CONSTTIME_FALSE_8;
}

static crypto_word_t FromBoolW(bool b) {
  return b ? CONSTTIME_TRUE_W : CONSTTIME_FALSE_W;
}

static const uint8_t test_values_8[] = {0, 1, 2, 20, 32, 127, 128, 129, 255};

static crypto_word_t test_values_w[] = {
    0,
    1,
    1024,
    12345,
    32000,
#if defined(OPENSSL_64_BIT)
    0xffffffff / 2 - 1,
    0xffffffff / 2,
    0xffffffff / 2 + 1,
    0xffffffff - 1,
    0xffffffff,
#endif
    std::numeric_limits<crypto_word_t>::max() / 2 - 1,
    std::numeric_limits<crypto_word_t>::max() / 2,
    std::numeric_limits<crypto_word_t>::max() / 2 + 1,
    std::numeric_limits<crypto_word_t>::max() - 1,
    std::numeric_limits<crypto_word_t>::max(),
};

static int signed_test_values[] = {
    0,     1,      -1,      1024,    -1024,       12345,      -12345,
    32000, -32000, INT_MAX, INT_MIN, INT_MAX - 1, INT_MIN + 1};

TEST(ConstantTimeTest, Test) {
  for (crypto_word_t a : test_values_w) {
    SCOPED_TRACE(a);

    EXPECT_EQ(FromBoolW(a == 0), constant_time_is_zero_w(a));
    EXPECT_EQ(FromBool8(a == 0), constant_time_is_zero_8(a));

    for (crypto_word_t b : test_values_w) {
      SCOPED_TRACE(b);

      EXPECT_EQ(FromBoolW(a < b), constant_time_lt_w(a, b));
      EXPECT_EQ(FromBool8(a < b), constant_time_lt_8(a, b));

      EXPECT_EQ(FromBoolW(a >= b), constant_time_ge_w(a, b));
      EXPECT_EQ(FromBool8(a >= b), constant_time_ge_8(a, b));

      EXPECT_EQ(FromBoolW(a == b), constant_time_eq_w(a, b));
      EXPECT_EQ(FromBool8(a == b), constant_time_eq_8(a, b));

      EXPECT_EQ(a, constant_time_select_w(CONSTTIME_TRUE_W, a, b));
      EXPECT_EQ(b, constant_time_select_w(CONSTTIME_FALSE_W, a, b));
    }
  }

  for (int a : signed_test_values) {
    SCOPED_TRACE(a);
    for (int b : signed_test_values) {
      SCOPED_TRACE(b);

      EXPECT_EQ(a, constant_time_select_int(CONSTTIME_TRUE_W, a, b));
      EXPECT_EQ(b, constant_time_select_int(CONSTTIME_FALSE_W, a, b));

      EXPECT_EQ(FromBoolW(a == b), constant_time_eq_int(a, b));
      EXPECT_EQ(FromBool8(a == b), constant_time_eq_int_8(a, b));
    }
  }

  for (uint8_t a : test_values_8) {
    SCOPED_TRACE(static_cast<int>(a));
    for (uint8_t b : test_values_8) {
      SCOPED_TRACE(static_cast<int>(b));
      EXPECT_EQ(a, constant_time_select_8(CONSTTIME_TRUE_8, a, b));
      EXPECT_EQ(b, constant_time_select_8(CONSTTIME_FALSE_8, a, b));
    }
  }

  // Test constant_time_select_array_w.
  crypto_word_t a[256], b[256], c[256];
  RAND_bytes(reinterpret_cast<uint8_t *>(a), sizeof(a));
  RAND_bytes(reinterpret_cast<uint8_t *>(b), sizeof(b));
  RAND_bytes(reinterpret_cast<uint8_t *>(c), sizeof(c));

  constant_time_select_array_w(c, a, b, FromBoolW(true), 256);
  EXPECT_EQ(0, OPENSSL_memcmp(c, a, sizeof(c)));
  constant_time_select_array_w(c, a, b, FromBoolW(false), 256);
  EXPECT_EQ(0, OPENSSL_memcmp(c, b, sizeof(c)));

  // Test constant_time_select_entry_from_table_w.
  // The table consists of 100 entries, each of size 10 crypto_word_t's.
  crypto_word_t table[100 * 10];
  crypto_word_t entry[10];
  RAND_bytes(reinterpret_cast<uint8_t *>(table), sizeof(table));
  RAND_bytes(reinterpret_cast<uint8_t *>(entry), sizeof(entry));

  for (size_t i = 0; i < 100; i++) {
    constant_time_select_entry_from_table_w(entry, table, i, 100, 10);
    EXPECT_EQ(0, OPENSSL_memcmp(entry, &table[i * 10], sizeof(entry)));
  }

  // Test that the output remains unchanged when index is out of bounds.
  crypto_word_t entry_copy[10];
  OPENSSL_memcpy(entry_copy, entry, sizeof(entry));
  constant_time_select_entry_from_table_w(entry, table, 101, 100, 10);
  EXPECT_EQ(0, OPENSSL_memcmp(entry_copy, entry, sizeof(entry)));
}

TEST(ConstantTimeTest, MemCmp) {
  uint8_t buf[256], copy[256];
  RAND_bytes(buf, sizeof(buf));

  OPENSSL_memcpy(copy, buf, sizeof(buf));
  EXPECT_EQ(0, CRYPTO_memcmp(buf, copy, sizeof(buf)));

  for (size_t i = 0; i < sizeof(buf); i++) {
    for (uint8_t bit = 1; bit != 0; bit <<= 1) {
      OPENSSL_memcpy(copy, buf, sizeof(buf));
      copy[i] ^= bit;
      EXPECT_NE(0, CRYPTO_memcmp(buf, copy, sizeof(buf)));
    }
  }
}

TEST(ConstantTimeTest, ValueBarrier) {
  for (int i = 0; i < 10; i++) {
    crypto_word_t word;
    RAND_bytes(reinterpret_cast<uint8_t *>(&word), sizeof(word));
    EXPECT_EQ(word, value_barrier_w(word));

    uint32_t u32;
    RAND_bytes(reinterpret_cast<uint8_t *>(&u32), sizeof(u32));
    EXPECT_EQ(u32, value_barrier_u32(u32));

    uint64_t u64;
    RAND_bytes(reinterpret_cast<uint8_t *>(&u64), sizeof(u64));
    EXPECT_EQ(u64, value_barrier_u64(u64));
  }
}
