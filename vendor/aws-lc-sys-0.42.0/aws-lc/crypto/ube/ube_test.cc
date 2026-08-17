// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include  <gtest/gtest.h>

#include <openssl/rand.h>

#include "internal.h"
#include "../test/ube_test.h"
#include "../test/test_util.h"

class ubeGenerationNumberTest : public::testing::Test {
  private:
    UbeBase ube_base_;

  protected:
    void SetUp() override {
      ube_base_.SetUp();
    }

    void TearDown() override {
      ube_base_.TearDown();
    }

    bool UbeIsSupported() const {
      return ube_base_.UbeIsSupported();
    }

    void allowMockedUbe() const {
      return ube_base_.allowMockedUbe();
    }
};

TEST_F(ubeGenerationNumberTest, BasicTests) {
  uint64_t generation_number = 0;
  if (CRYPTO_get_ube_generation_number(&generation_number) == 0) {
    // In this case, UBE detection is disabled, so just return
    // successfully. This should be a persistent state; check that.
    ASSERT_FALSE(CRYPTO_get_ube_generation_number(&generation_number));
    return;
  }

  ASSERT_TRUE(CRYPTO_get_ube_generation_number(&generation_number));

  // Check stability.
  uint64_t current_generation_number = generation_number + 1;
  ASSERT_TRUE(CRYPTO_get_ube_generation_number(&current_generation_number));
  ASSERT_EQ(current_generation_number, generation_number);

  // Check stability again.
  current_generation_number = generation_number + 2;
  ASSERT_TRUE(CRYPTO_get_ube_generation_number(&current_generation_number));
  ASSERT_EQ(current_generation_number, generation_number);
}

static void MockedDetectionMethodTest(
  std::function<void(uint32_t)> set_method_generation_number) {

  uint64_t generation_number = 0;
  uint64_t cached_generation_number = 0;
  uint32_t mocked_generation_number = 0;

  uint8_t initial_mocked_generation_number[4] = {0};
  ASSERT_TRUE(RAND_bytes(initial_mocked_generation_number, 4));
  mocked_generation_number =
        ((uint32_t)initial_mocked_generation_number[0] << 24) |
        ((uint32_t)initial_mocked_generation_number[1] << 16) |
        ((uint32_t)initial_mocked_generation_number[2] << 8)  |
        ((uint32_t)initial_mocked_generation_number[3]);

  // Testing that UBE generation number is incremented when:
  //   mocked_generation_number + 1
  //   mocked_generation_number + 3
  //   mocked_generation_number - 1
  // Set our starting point and get initial UBE generation number
  set_method_generation_number(mocked_generation_number);
  ASSERT_TRUE(CRYPTO_get_ube_generation_number(&generation_number));

  // Should be stable.
  cached_generation_number = generation_number;
  generation_number = 0;
  ASSERT_TRUE(CRYPTO_get_ube_generation_number(&generation_number));
  ASSERT_EQ(generation_number, cached_generation_number);

  // Mock a UBE.
  set_method_generation_number(mocked_generation_number + 1);

  // UBE generation number should have incremented once.
  cached_generation_number = generation_number;
  generation_number = 0;
  ASSERT_TRUE(CRYPTO_get_ube_generation_number(&generation_number));
  ASSERT_EQ(generation_number, cached_generation_number + 1);

  // Should be stable again.
  cached_generation_number = generation_number;
  generation_number = 0;
  ASSERT_TRUE(CRYPTO_get_ube_generation_number(&generation_number));
  ASSERT_EQ(generation_number, cached_generation_number);

  // Mock another UBE with higher increment.
  set_method_generation_number(mocked_generation_number + 3);

  // Generation number should have incremented once.
  cached_generation_number = generation_number;
  generation_number = 0;
  ASSERT_TRUE(CRYPTO_get_ube_generation_number(&generation_number));
  ASSERT_EQ(generation_number, cached_generation_number + 1);

  // Should be stable again.
  cached_generation_number = generation_number;
  generation_number = 0;
  ASSERT_TRUE(CRYPTO_get_ube_generation_number(&generation_number));
  ASSERT_EQ(generation_number, cached_generation_number);

  // Mock another UBE but with a strictly smaller value.
  set_method_generation_number(mocked_generation_number - 1);

  // Generation number should have incremented once.
  cached_generation_number = generation_number;
  generation_number = 0;
  ASSERT_TRUE(CRYPTO_get_ube_generation_number(&generation_number));
  ASSERT_EQ(generation_number, cached_generation_number + 1);

  // Should be stable again.
  cached_generation_number = generation_number;
  generation_number = 0;
  ASSERT_TRUE(CRYPTO_get_ube_generation_number(&generation_number));
  ASSERT_EQ(generation_number, cached_generation_number);
}

TEST_F(ubeGenerationNumberTest, MockedDetectionMethodTests) {

  allowMockedUbe();

  MockedDetectionMethodTest(
    [](uint32_t gn) {
      set_fork_ube_generation_number_FOR_TESTING(static_cast<uint64_t>(gn));
    }
  );

  MockedDetectionMethodTest(
    [](uint32_t gn) {
      set_vm_ube_generation_number_FOR_TESTING(gn);
    }
  );

  MockedDetectionMethodTest(
    [](uint32_t gn) {
      set_fork_ube_generation_number_FOR_TESTING(static_cast<uint64_t>(gn));
      set_vm_ube_generation_number_FOR_TESTING(gn);
    }
  );

  MockedDetectionMethodTest(
    [](uint32_t gn) {
      set_fork_ube_generation_number_FOR_TESTING(static_cast<uint64_t>(gn));
      set_vm_ube_generation_number_FOR_TESTING(gn + 1);
    }
  );
}

TEST_F(ubeGenerationNumberTest, ExpectedSupportTests) {
  uint64_t generation_number = 0;
  // Operating systems where we expect UBE detection to be enabled.
  if (osIsAmazonLinux()) {
    ASSERT_TRUE(CRYPTO_get_ube_generation_number(&generation_number));
  }
}
