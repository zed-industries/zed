// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <gtest/gtest.h>
#include <openssl/rand.h>
#include <openssl/span.h>

#include "internal.h"
#include "../test/test_util.h"

// This test is, strictly speaking, flaky, but we use large enough buffers
// (48 bytes) that the probability of failing when we should pass is negligible.

TEST(VmUbeFallbackTest, NotObviouslyBroken) {
  static const uint8_t kZeros[CTR_DRBG_ENTROPY_LEN] = {0};

  uint8_t seed1[CTR_DRBG_ENTROPY_LEN];
  uint8_t seed2[CTR_DRBG_ENTROPY_LEN];

  ASSERT_TRUE(vm_ube_fallback_get_seed(seed1));
  ASSERT_TRUE(vm_ube_fallback_get_seed(seed2));

  EXPECT_NE(Bytes(seed1), Bytes(seed2));
  EXPECT_NE(Bytes(seed1), Bytes(kZeros));
  EXPECT_NE(Bytes(seed2), Bytes(kZeros));

  uint8_t seed3[CTR_DRBG_ENTROPY_LEN];
  // Ensure that the implementation is not simply returning the memory unchanged.
  memcpy(seed3, seed1, CTR_DRBG_ENTROPY_LEN);
  ASSERT_TRUE(vm_ube_fallback_get_seed(seed1));
  EXPECT_NE(Bytes(seed1), Bytes(seed3));
}
