// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include "internal.h"

#include <gtest/gtest.h>

#if defined(OPENSSL_THREADS)
#include <thread>
#endif


TEST(RefCountTest, Basic) {
  CRYPTO_refcount_t count = 0;

  CRYPTO_refcount_inc(&count);
  EXPECT_EQ(1u, count);

  EXPECT_TRUE(CRYPTO_refcount_dec_and_test_zero(&count));
  EXPECT_EQ(0u, count);

  count = CRYPTO_REFCOUNT_MAX;
  CRYPTO_refcount_inc(&count);
  EXPECT_EQ(CRYPTO_REFCOUNT_MAX, count)
      << "Count did not saturate correctly when incrementing.";
  EXPECT_FALSE(CRYPTO_refcount_dec_and_test_zero(&count));
  EXPECT_EQ(CRYPTO_REFCOUNT_MAX, count)
      << "Count did not saturate correctly when decrementing.";

  count = 2;
  EXPECT_FALSE(CRYPTO_refcount_dec_and_test_zero(&count));
  EXPECT_EQ(1u, count);
}

#if defined(OPENSSL_THREADS)
// This test is primarily intended to run under ThreadSanitizer.
TEST(RefCountTest, Threads) {
  CRYPTO_refcount_t count = 0;

  // Race two increments.
  {
    std::thread thread([&] { CRYPTO_refcount_inc(&count); });
    CRYPTO_refcount_inc(&count);
    thread.join();
    EXPECT_EQ(2u, count);
  }

  // Race an increment with a decrement.
  {
    std::thread thread([&] { CRYPTO_refcount_inc(&count); });
    EXPECT_FALSE(CRYPTO_refcount_dec_and_test_zero(&count));
    thread.join();
    EXPECT_EQ(2u, count);
  }

  // Race two decrements.
  {
    bool thread_saw_zero;
    std::thread thread(
        [&] { thread_saw_zero = CRYPTO_refcount_dec_and_test_zero(&count); });
    bool saw_zero = CRYPTO_refcount_dec_and_test_zero(&count);
    thread.join();
    EXPECT_EQ(0u, count);
    // Exactly one thread should see zero.
    EXPECT_NE(saw_zero, thread_saw_zero);
  }
}
#endif
