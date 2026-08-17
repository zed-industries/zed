// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <gtest/gtest.h>

#include <openssl/crypto.h>

#include "internal.h"
#include "../../../ube/vm_ube_detect.h"

#define MAX_MULTIPLE_FROM_RNG (16)

// In the future this test can be improved by being able to predict whether the
// test is running on hardware that we expect to support RNDR. This will require
// amending the CI with such information.
// For now, simply ensure we exercise all code-paths in the hw rng
// implementations.

TEST(EntropySourceHw, Aarch64) {
  uint8_t buf[MAX_MULTIPLE_FROM_RNG*8] = { 0 } ;

#if !defined(OPENSSL_AARCH64) || defined(OPENSSL_NO_ASM)
  ASSERT_FALSE(have_hw_rng_aarch64_for_testing());
  ASSERT_FALSE(rndr_multiple8(buf, 0));
  ASSERT_FALSE(rndr_multiple8(buf, 8));
#else
  if (have_hw_rng_aarch64_for_testing() != 1) {
    GTEST_SKIP() << "Compiled for Arm64, but Aarch64 hw rng is not available in run-time";
  }

  // Extracting 0 bytes is never supported.
  ASSERT_FALSE(rndr_multiple8(buf, 0));

  // Multiples of 8 allowed.
  for (size_t i = 8; i < MAX_MULTIPLE_FROM_RNG; i += 8) {
    ASSERT_TRUE(rndr_multiple8(buf, i));
  }

  // Must be multiples of 8.
  for (size_t i : {1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15}) {
    ASSERT_FALSE(rndr_multiple8(buf, i));
  }
#endif
}

TEST(EntropySourceHw, x86_64) {
  uint8_t buf[MAX_MULTIPLE_FROM_RNG*8] = { 0 } ;

#if !defined(OPENSSL_X86_64) || defined(OPENSSL_NO_ASM)
  ASSERT_FALSE(have_hw_rng_x86_64_for_testing());
  ASSERT_FALSE(rdrand_multiple8(buf, 0));
  ASSERT_FALSE(rdrand_multiple8(buf, 8));
#else
  if (have_hw_rng_x86_64_for_testing() != 1) {
    GTEST_SKIP() << "Compiled for x86_64, but x86_64 hw rng is not available in run-time";
  }

  // Extracting 0 bytes is never supported.
  ASSERT_FALSE(rdrand_multiple8(buf, 0));

  // Multiples of 8 allowed.
  for (size_t i = 8; i < MAX_MULTIPLE_FROM_RNG; i += 8) {
    ASSERT_TRUE(rdrand_multiple8(buf, i));
  }

  // Must be multiples of 8.
  for (size_t i : {1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15}) {
    ASSERT_FALSE(rdrand_multiple8(buf, i));
  }
#endif
}

TEST(EntropySources, Configuration) {
  uint8_t buf[1];
  ASSERT_TRUE(RAND_bytes(buf, sizeof(buf)));

// VM UBE detection is only defined for Linux. So, only strongly assert on
// that kernel.
#if defined(AWSLC_VM_UBE_TESTING) && defined(OPENSSL_LINUX)
  EXPECT_EQ(OPT_OUT_CPU_JITTER_ENTROPY_SOURCE, get_entropy_source_method_id_FOR_TESTING());

// If entropy build configuration choose to explicitly opt-out of CPU Jitter
// Entropy
#elif defined(DISABLE_CPU_JITTER_ENTROPY)
  EXPECT_EQ(OPT_OUT_CPU_JITTER_ENTROPY_SOURCE, get_entropy_source_method_id_FOR_TESTING());

#else
  int expected_entropy_source_id = TREE_DRBG_JITTER_ENTROPY_SOURCE;
  if (CRYPTO_get_vm_ube_supported()) {
    expected_entropy_source_id = OPT_OUT_CPU_JITTER_ENTROPY_SOURCE;
  }

  EXPECT_EQ(expected_entropy_source_id, get_entropy_source_method_id_FOR_TESTING());

  // For FIPS build we can strongly assert.
  if (FIPS_mode() == 1 && CRYPTO_get_vm_ube_supported() != 1) {
    EXPECT_NE(OPT_OUT_CPU_JITTER_ENTROPY_SOURCE, get_entropy_source_method_id_FOR_TESTING());
  }
#endif
}
