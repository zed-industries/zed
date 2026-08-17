// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <openssl/crypto.h>
#include <gtest/gtest.h>

#include "internal.h"

#if defined(OPENSSL_THREADS)
#include <chrono>
#include <thread>
#endif

#if defined(AARCH64_DIT_SUPPORTED) && !defined(OPENSSL_STATIC_ARMCAP)

#if defined(ENABLE_AUTO_SET_RESET_DIT)
static void NestedMacroInvocation(uint64_t one) {
  SET_DIT_AUTO_RESET;
  uint64_t current_dit = armv8_get_dit();
  EXPECT_EQ(current_dit, one);
}
#endif // ENABLE_AUTO_SET_RESET_DIT

TEST(DITTest, SetReset) {
  uint64_t one = CRYPTO_is_ARMv8_DIT_capable_for_testing()? (uint64_t)1 : (uint64_t)0;

  uint64_t original_dit = 0, original_dit_2 = 0,
    current_dit = 0;
  original_dit = armv8_set_dit();
  EXPECT_EQ(original_dit, (uint64_t)0);

  // the case of a nested call of setting DIT
  original_dit_2 = armv8_set_dit();
  EXPECT_EQ(original_dit_2, one);

  current_dit = armv8_get_dit();
  EXPECT_EQ(current_dit, one);

  armv8_restore_dit(&original_dit);
  current_dit = armv8_get_dit();
  EXPECT_EQ(current_dit, (uint64_t)0);

#if defined(ENABLE_AUTO_SET_RESET_DIT)
  { // invoke the macro within a scope
    // to test that it restores the CPU DIT flag at the end
    SET_DIT_AUTO_RESET;
    current_dit = armv8_get_dit();
    EXPECT_EQ(current_dit, one);
    // Nested macro invocation will exit the scope leaving DIT = 1
    NestedMacroInvocation(one);
    current_dit = armv8_get_dit();
    EXPECT_EQ(current_dit, one);
  }
  current_dit = armv8_get_dit();
  EXPECT_EQ(current_dit, (uint64_t)0);
#endif // ENABLE_AUTO_SET_RESET_DIT
}

#if defined(OPENSSL_THREADS)

TEST(DITTest, Threads) {
  uint64_t one = CRYPTO_is_ARMv8_DIT_capable_for_testing()? (uint64_t)1 : (uint64_t)0;

  {
    // Test that the CPU DIT flag (bit in PSTATE register) is
    // context-switched at the thread level.
    std::thread thread1([&] {
      uint64_t original_dit = 0, current_dit = 0;
      original_dit = armv8_set_dit();
      EXPECT_EQ(original_dit, (uint64_t)0);

      current_dit = armv8_get_dit();
      EXPECT_EQ(current_dit, one);

      // Sleep until thread2 starts, sets and resets DIT
      std::this_thread::sleep_for(std::chrono::milliseconds(40));

      // This thread should still see DIT=1
      current_dit = armv8_get_dit();
      EXPECT_EQ(current_dit, one);

      armv8_restore_dit(&original_dit);
      current_dit = armv8_get_dit();
      EXPECT_EQ(current_dit, (uint64_t)0);
    });

    std::thread thread2([&] {
      uint64_t original_dit = 0, current_dit = 0;
      original_dit = armv8_set_dit();
      EXPECT_EQ(original_dit, (uint64_t)0);

      current_dit = armv8_get_dit();
      EXPECT_EQ(current_dit, one);

      armv8_restore_dit(&original_dit);
      current_dit = armv8_get_dit();
      EXPECT_EQ(current_dit, (uint64_t)0);
    });

    thread2.join();
    thread1.join();
  }

  {
    // Test that the DIT runtime dis/enabler in OPENSSL_armcap_P is
    // at the process level.
    // (Trying to make the threads concurrent and synchronising them
    //  with sleep time was making the Thread Sanitizer warn about a
    //  a data race.)

    std::thread thread1([&] {
      uint64_t original_dit = 0, current_dit = 0;

      original_dit = armv8_set_dit();
      EXPECT_EQ(original_dit, (uint64_t)0);

      current_dit = armv8_get_dit();
      EXPECT_EQ(current_dit, one);

      armv8_restore_dit(&original_dit);
      current_dit = armv8_get_dit();
      EXPECT_EQ(current_dit, (uint64_t)0);

      armv8_disable_dit(); // disable DIT capability at run-time
    });

    thread1.join();

    std::thread thread2([&] {
      uint64_t original_dit = 0, current_dit = 0;

      // DIT was disabled at runtime, so the DIT bit would be read as 0
      EXPECT_EQ(CRYPTO_is_ARMv8_DIT_capable_for_testing(), 0);

      original_dit = armv8_set_dit();
      EXPECT_EQ(original_dit, (uint64_t)0);

      current_dit = armv8_get_dit();
      EXPECT_EQ(current_dit, (uint64_t)0);

      armv8_restore_dit(&original_dit);
      current_dit = armv8_get_dit();
      EXPECT_EQ(current_dit, (uint64_t)0);

    });

    thread2.join();

    std::thread thread3([&] {
      armv8_enable_dit();  // enable back DIT capability at run-time
    });

    thread3.join();

    std::thread thread4([&] {
      uint64_t original_dit = 0, current_dit = 0;

      EXPECT_EQ(CRYPTO_is_ARMv8_DIT_capable_for_testing(), (int)one);
      original_dit = armv8_set_dit();
      EXPECT_EQ(original_dit, (uint64_t)0);

      current_dit = armv8_get_dit();
      EXPECT_EQ(current_dit, one);

      armv8_restore_dit(&original_dit);
      current_dit = armv8_get_dit();
      EXPECT_EQ(current_dit, (uint64_t)0);
    });

    thread4.join();

  }
}
#endif // OPENSSL_THREADS

#endif  // AARCH64_DIT_SUPPORTED && !OPENSSL_STATIC_ARMCAP
