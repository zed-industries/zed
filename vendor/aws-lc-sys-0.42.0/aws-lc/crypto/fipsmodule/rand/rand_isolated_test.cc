// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <gtest/gtest.h>

#include <openssl/ctrdrbg.h>
#include <openssl/mem.h>
#include <openssl/rand.h>

#include "entropy/internal.h"
#include "internal.h"
#include "../../ube/internal.h"

#include "../../test/ube_test.h"
#include "../../test/test_util.h"

#include <array>

// Use `#if GTEST_HAS_DEATH_TEST` (not `defined()`); some toolchains define it to 0.
#if GTEST_HAS_DEATH_TEST

static const size_t request_len = 64;
static const size_t number_of_threads = 8;

static thread_local size_t initialize_count = 0;
static thread_local size_t zeroize_thread_count = 0;
static thread_local size_t free_thread_count = 0;
static thread_local size_t get_seed_count = 0;
static thread_local size_t get_extra_entropy_count = 0;
static thread_local size_t get_prediction_resistance_count = 0;

static thread_local size_t cached_get_seed_count = 0;
static thread_local size_t cached_get_extra_entropy_count = 0;

static entropy_source_methods entropy_methods{
  nullptr,  // initialize
  nullptr,  // zeroize_thread
  nullptr,  // free_thread
  nullptr,  // get_seed
  nullptr,  // get_extra_entropy
  nullptr,  // get_prediction_resistance
  OVERRIDDEN_ENTROPY_SOURCE
};

static int overrideInitialize(struct entropy_source_t *entropy_source) {
  initialize_count++;
  return 1;
}

static void overrideZeroizeThread(struct entropy_source_t *entropy_source) {
  zeroize_thread_count++;
}

static void overrideFreeThread(struct entropy_source_t *entropy_source) {
  free_thread_count++;
}

static int overrideGetSeed(const struct entropy_source_t *entropy_source,
  uint8_t seed[CTR_DRBG_ENTROPY_LEN]) {
  get_seed_count++;
  return 1;
}

static int overrideGetExtraEntropy(const struct entropy_source_t *entropy_source,
  uint8_t seed[CTR_DRBG_ENTROPY_LEN]) {
  get_extra_entropy_count++;
  return 1;
}

static int overrideGetPredictionResistance(
  const struct entropy_source_t *entropy_source,
  uint8_t seed[RAND_PRED_RESISTANCE_LEN]) {
  get_prediction_resistance_count++;
  return 1;
}

static bool assertSeedOrReseed(size_t expected_count, std::function<bool()> func,
  const char *error_text = "") {

  cached_get_seed_count = get_seed_count;
  cached_get_extra_entropy_count = get_extra_entropy_count;

  if (!func()) {
    return false;
  }

  if (cached_get_seed_count + expected_count != get_seed_count ||
      cached_get_extra_entropy_count + expected_count != get_extra_entropy_count) {
    std::cerr << "Entropy source method expected count mismatch " << error_text << '\n'
              << "  Get seed count: expected=" << (cached_get_seed_count + expected_count)
              << ", actual=" << get_seed_count << '\n'
              << "  Get extra entropy count: expected=" << (cached_get_extra_entropy_count + expected_count)
              << ", actual=" << get_extra_entropy_count << '\n';
    return false;
  }

  return true;
}

static void overrideEntropySourceMethodsCount() {
  entropy_methods = {
    &overrideInitialize,
    &overrideZeroizeThread,
    &overrideFreeThread,
    &overrideGetSeed,
    &overrideGetExtraEntropy,
    &overrideGetPredictionResistance,
    OVERRIDDEN_ENTROPY_SOURCE
  };
  override_entropy_source_method_FOR_TESTING(&entropy_methods);
}

class randIsolatedTest : public::testing::Test {
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
      ube_base_.allowMockedUbe();
    }
};

static bool generateRandomness(size_t req_Len, const char *error_text = "") {
  std::vector<uint8_t> output_rand(req_Len);
  if (!RAND_bytes(output_rand.data(), output_rand.size())) {
    std::cerr << "Generating randomness failed " << error_text << '\n';
    return false;
  }
  return true;
}

TEST_F(randIsolatedTest, BasicThread) {

  // Test reseeds are observed when spawning new threads.
  auto testFunc = [this]() {

    // Setup entropy source method override.
    overrideEntropySourceMethodsCount();
    generateRandomness(request_len);

    std::function<void(bool*)> threadFunc = [this](bool *result) {
      // In a fresh thread, we expect a seed.
      bool test1 = assertSeedOrReseed(1, []() {
        return generateRandomness(request_len);
      });

      // If UBE detection is not supported, we expect a reseed again. Otherwise,
      // no reseed is expected.
      size_t expect_reseed = 1;
      if (UbeIsSupported()) {
        expect_reseed = 0;
      }

      bool test2 = assertSeedOrReseed(expect_reseed, []() {
        return generateRandomness(request_len);
      });

      *result = test1 && test2;
    };

    bool exit_code = threadTest(number_of_threads, threadFunc);
    exit(exit_code ? 0 : 1);
  };

  EXPECT_EXIT(testFunc(), ::testing::ExitedWithCode(0), "");
}

#if !defined(OPENSSL_WINDOWS)

TEST_F(randIsolatedTest, BasicFork) {

  if (runtimeEmulationIsIntelSde() && addressSanitizerIsEnabled()) {
    GTEST_SKIP() << "Test not supported under Intel SDE + ASAN";
  }

  // Test reseed is observed when forking.
  auto testFuncSingleFork = [this]() {

    // Setup entropy source method override
    overrideEntropySourceMethodsCount();
    generateRandomness(request_len);

    bool exit_code = forkAndRunTest(
      []() {
        // In child. If fork detection is supported, we expect a reseed.
        // If fork detection is not enabled, we also expect a reseed.
        return assertSeedOrReseed(1, []() {
          return generateRandomness(request_len, "child");
        }, "child");
      },
      [this]() {
        // In parent. If UBE detection is not supported, we expect a reseed
        // again. Otherwise, no reseed is expected.
        size_t expect_reseed = 1;
        if (UbeIsSupported()) {
          expect_reseed = 0;
        }

        return assertSeedOrReseed(expect_reseed, []() {
          return generateRandomness(request_len, "parent");
        }, "parent");
      }
    );

    exit(exit_code ? 0 : 1);
  };

  EXPECT_EXIT(testFuncSingleFork(), ::testing::ExitedWithCode(0), "");

  // Test reseed is observed when forking and spawning new threads before
  // generating randomness.
  auto testFuncSingleForkThenThread = [this]() {

    // Setup entropy source method override
    overrideEntropySourceMethodsCount();
    generateRandomness(request_len);

    bool exit_code = forkAndRunTest(
      []() {
        // In child. Spawn a number of threads before generating randomness.
        // If fork detection is supported, we expect a seed in each thread.
        // If fork detection is not enabled, we also expect a seed in each
        // thread.
        std::function<void(bool*)> threadFunc = [](bool *result) {
          *result = assertSeedOrReseed(1, []() {
            return generateRandomness(request_len, "child");
          }, "child");
        };
        return threadTest(number_of_threads, threadFunc);
      },
      [this]() {
        // In parent. If UBE detection is not supported, we expect a reseed
        // again. Otherwise, no reseed is expected.
        size_t expect_reseed = 1;
        if (UbeIsSupported()) {
          expect_reseed = 0;
        }

        return assertSeedOrReseed(expect_reseed, []() {
          return generateRandomness(request_len, "parent");
        }, "parent");
      }
    );

    exit(exit_code ? 0 : 1);
  };

  EXPECT_EXIT(testFuncSingleForkThenThread(), ::testing::ExitedWithCode(0), "");

  // Test reseed is observed when forking and then forking again before
  // generating any randomness.
  auto testFuncDoubleFork = [this]() {

    // Setup entropy source method override
    overrideEntropySourceMethodsCount();
    generateRandomness(request_len);

    bool exit_code = forkAndRunTest(
      []() {

        // In child. Fork again before generating randomness.
        bool exit_code_child = forkAndRunTest(
          []() {
            // In child-child. If fork detection is supported, we expect a
            // reseed. If fork detection is not enabled, we also expect a reseed.
            return assertSeedOrReseed(1, []() {
              return generateRandomness(request_len, "child-child");
            }, "child-child");
          },
          []() {
            // In a forked process, should expect a reseed no matter what.
            return assertSeedOrReseed(1, []() {
              return generateRandomness(request_len, "child-parent");
            }, "child-parent");
          }
        );

        return exit_code_child;
      },
      [this]() {
        // In parent. If UBE detection is not supported, we expect a reseed
        // again. Otherwise, no reseed is expected.
        size_t expect_reseed = 1;
        if (UbeIsSupported()) {
          expect_reseed = 0;
        }

        return assertSeedOrReseed(expect_reseed, []() {
          return generateRandomness(request_len, "parent");
        }, "parent");
      }
    );

    exit(exit_code ? 0 : 1);
  };

  EXPECT_EXIT(testFuncDoubleFork(), ::testing::ExitedWithCode(0), "");

  // Test reseed is observed when forking, generate randomness, and then fork
  // again.
  auto testFuncForkGenerateFork = [this]() {

    // Setup entropy source method override
    overrideEntropySourceMethodsCount();
    generateRandomness(request_len);

    bool exit_code = forkAndRunTest(
      [this]() {

        // In a forked process, should expect a reseed no matter what.
        return assertSeedOrReseed(1, []() {
          return generateRandomness(request_len, "child-parent");
        }, "child-parent");

        bool exit_code_child = forkAndRunTest(
          []() {
            // In a forked process, should expect a reseed no matter what.
            return assertSeedOrReseed(1, []() {
              return generateRandomness(request_len, "child-parent");
            }, "child-parent");
          },
          [this]() {
            // In parent. If UBE detection is not supported, we expect a reseed
            // again. Otherwise, no reseed is expected.
            size_t expect_reseed = 1;
            if (UbeIsSupported()) {
              expect_reseed = 0;
            }

            return assertSeedOrReseed(expect_reseed, []() {
              return generateRandomness(request_len, "parent");
            }, "parent");
          }
        );

        return exit_code_child;
      },
      [this]() {
        // In parent. If UBE detection is not supported, we expect a reseed
        // again. Otherwise, no reseed is expected.
        size_t expect_reseed = 1;
        if (UbeIsSupported()) {
          expect_reseed = 0;
        }

        return assertSeedOrReseed(expect_reseed, []() {
          return generateRandomness(request_len, "parent");
        }, "parent");
      }
    );

    exit(exit_code ? 0 : 1);
  };

  EXPECT_EXIT(testFuncForkGenerateFork(), ::testing::ExitedWithCode(0), "");
}
#endif

TEST_F(randIsolatedTest, BasicInitialization) {

  if (runtimeEmulationIsIntelSde() && addressSanitizerIsEnabled()) {
    GTEST_SKIP() << "Test not supported under Intel SDE + ASAN";
  }

  // Test only one seed occurs on initialization.
  auto testFunc = [this]() {

    // Setup entropy source method override.
    overrideEntropySourceMethodsCount();

    bool test1 = assertSeedOrReseed(1, []() {
      return generateRandomness(request_len);
    });

    // In parent. If UBE detection is not supported, we expect a reseed on
    // second invocation. Otherwise, no reseed is expected.
    size_t expect_reseed = 1;
    if (UbeIsSupported()) {
      expect_reseed = 0;
    }

    bool test2 = assertSeedOrReseed(expect_reseed, []() {
      return generateRandomness(request_len);
    });

    exit((test1 && test2) ? 0 : 1);
  };

  EXPECT_EXIT(testFunc(), ::testing::ExitedWithCode(0), "");
}

// Test vectors for randomness generation KATs.
// The tests below proves that we can predict the input to the underlying
// CTR-DRBG implementation and that the result is what we expect. Test vectors
// and expected results are computed from an independent implementation. See
// original PR for source code.

namespace RngKatTestUtils {

enum class TestType {
  NoReseedNoPr,
  NoReseedAndPr,
  NoReseedAndPrAndUserPr,
  NoReseedNoPrAndUserPr,
  WithReseedNoPr,
  WithReseedAndPr,
  WithReseedAndPrAndUserPr,
  WithReseedNoPrAndUserPr
};

enum class GenerationState {
  First,
  Second
};

// Global state for test generation
thread_local GenerationState current_generation_state = GenerationState::First;

class RngKatTestData {
public:
  static const size_t KAT_GENERATE_REQ_LEN = 64;

  struct TestVectors {
    std::array<uint8_t, CTR_DRBG_ENTROPY_LEN> seed;
    std::array<uint8_t, CTR_DRBG_ENTROPY_LEN> seedPersonalization;
    std::array<uint8_t, CTR_DRBG_ENTROPY_LEN> reseed;
    std::array<uint8_t, CTR_DRBG_ENTROPY_LEN> reseedPersonalization;
    std::array<uint8_t, RAND_PRED_RESISTANCE_LEN> predictionResistance1;
    std::array<uint8_t, RAND_PRED_RESISTANCE_LEN> predictionResistance2;
    std::array<uint8_t, RAND_PRED_RESISTANCE_LEN> userPredictionResistance1;
    std::array<uint8_t, RAND_PRED_RESISTANCE_LEN> userPredictionResistance2;
  };

  struct ExpectedOutput {
    std::vector<uint8_t> output1;
    std::vector<uint8_t> output2;
  };

  static const TestVectors& getTestVectors() {
    static const TestVectors vectors = {
      // seed
      {{0x22, 0xa8, 0x9e, 0xe0, 0xe3, 0x7b, 0x54, 0xea, 0x63, 0x68, 0x63, 0xd9,
        0xfe, 0xd1, 0x08, 0x21, 0xf1, 0x95, 0x2a, 0x42, 0x84, 0x88, 0xd5, 0x28,
        0xec, 0xeb, 0x9d, 0x2e, 0xc6, 0x9d, 0x57, 0x3e, 0xc6, 0x21, 0x62, 0x16,
        0xfb, 0x3e, 0x8f, 0x72, 0xa1, 0x48, 0xa5, 0xad, 0xa9, 0xd6, 0x20, 0xb1}},
      // seedPersonalization
      {{0x95, 0x3c, 0x10, 0xba, 0xdc, 0xbc, 0xd4, 0x5f, 0xb4, 0xe5, 0x47, 0x58,
        0x26, 0x47, 0x7f, 0xc1, 0x37, 0xac, 0x96, 0xa4, 0x9a, 0xd5, 0x00, 0x5f,
        0xb1, 0x4b, 0xda, 0xf6, 0x46, 0x8a, 0xe7, 0xf4, 0x6c, 0x5d, 0x0d, 0xe2,
        0x2d, 0x30, 0x4a, 0xfc, 0x67, 0x98, 0x96, 0x15, 0xad, 0xc2, 0xe9, 0x83}},
      // reseed
      {{0x8f, 0x84, 0x7a, 0x6f, 0x65, 0x5a, 0x50, 0x45, 0x3b, 0x30, 0x26, 0x1b,
        0x11, 0x06, 0xfc, 0xf1, 0xe7, 0xdc, 0xd2, 0xc7, 0xbd, 0xb2, 0xa8, 0x9d,
        0x93, 0x88, 0x7e, 0x73, 0x69, 0x5e, 0x54, 0x49, 0x3f, 0x34, 0x2a, 0x1f,
        0x15, 0x0a, 0x00, 0xf5, 0xeb, 0xe0, 0xd6, 0xcb, 0xc1, 0xb6, 0xac, 0xa1}},
      // reseedPersonalization
      {{0xc7, 0xc0, 0xb9, 0xb2, 0xab, 0xa4, 0x9d, 0x96, 0x8f, 0x88, 0x81, 0x7a,
        0x73, 0x6c, 0x65, 0x5e, 0x57, 0x50, 0x49, 0x42, 0x3b, 0x34, 0x2d, 0x26,
        0x1f, 0x18, 0x11, 0x0a, 0x03, 0xfc, 0xf5, 0xee, 0xe7, 0xe0, 0xd9, 0xd2,
        0xcb, 0xc4, 0xbd, 0xb6, 0xaf, 0xa8, 0xa1, 0x9a, 0x93, 0x8c, 0x85, 0x7e}},
      // predictionResistance1
      {{0x9d, 0x95, 0x8d, 0x85, 0x7d, 0x75, 0x6d, 0x65, 0x5d, 0x55, 0x4d, 0x45,
        0x3d, 0x35, 0x2d, 0x25, 0x1d, 0x15, 0x0d, 0x05, 0xfd, 0xf5, 0xed, 0xe5,
        0xdd, 0xd5, 0xcd, 0xc5, 0xbd, 0xb5, 0xad, 0xa5}},
      // predictionResistance2
      {{0xb5, 0xad, 0xa5, 0x9d, 0x95, 0x8d, 0x85, 0x7d, 0x75, 0x6d, 0x65, 0x5d,
        0x55, 0x4d, 0x45, 0x3d, 0x35, 0x2d, 0x25, 0x1d, 0x15, 0x0d, 0x05, 0xfd,
        0xf5, 0xed, 0xe5, 0xdd, 0xd5, 0xcd, 0xc5, 0xbd}},
      // userPredictionResistance1
      {{0x7b, 0x93, 0x45, 0xf2, 0x8d, 0x1c, 0xa4, 0xe6, 0x2f, 0xb8, 0x5d, 0x91,
        0x3c, 0x6a, 0xd4, 0x87, 0x15, 0xc9, 0x4e, 0xb2, 0x7f, 0x38, 0x96, 0x5a,
        0xd1, 0x4c, 0x83, 0x2b, 0xe5, 0x9f, 0x67, 0xa0}},
      // userPredictionResistance2
      {{0xe4, 0x2d, 0x9b, 0x56, 0xf8, 0x3a, 0xc1, 0x7d, 0x95, 0x42, 0xb6, 0x8f,
        0x1e, 0xd3, 0x69, 0xa4, 0x5b, 0xf1, 0x87, 0x2c, 0xd5, 0x9e, 0x43, 0xb8,
        0x6f, 0x12, 0xa7, 0x5d, 0xc4, 0x8b, 0x31, 0xe9}},
    };
    return vectors;
  }

  static const ExpectedOutput& getExpectedOutput(TestType type) {
    static const std::map<TestType, ExpectedOutput> outputs = {
      {TestType::NoReseedNoPr, {
        // output1
        {0x8e, 0xe6, 0x11, 0xf4, 0x76, 0x67, 0xa6, 0xab, 0xb5, 0x52, 0x55, 0xda, 
         0x07, 0x77, 0x66, 0xd5, 0x8f, 0xb9, 0x5d, 0x9c, 0x83, 0xdb, 0x46, 0x90, 
         0x74, 0x65, 0xce, 0x99, 0x8f, 0x54, 0xfb, 0x3b, 0x41, 0x8c, 0x21, 0xd0, 
         0x2a, 0x74, 0x32, 0xbb, 0x05, 0x6e, 0x99, 0xcf, 0x00, 0xa1, 0x78, 0x22, 
         0xc6, 0x72, 0x1f, 0x48, 0xeb, 0x9a, 0x1d, 0x9f, 0xf2, 0xa1, 0x1c, 0xa0, 
         0x4c, 0x2c, 0x37, 0x0d},
        // output2
        {0xf7, 0xfa, 0xb6, 0xa6, 0xfc, 0xf4, 0x45, 0xf0, 0xa0, 0x43, 0x4b, 0x2a, 
         0xa0, 0xc6, 0x10, 0xbd, 0xef, 0x54, 0x89, 0xec, 0xd9, 0x54, 0x14, 0x63, 
         0x46, 0x23, 0xad, 0xd1, 0x8a, 0x9f, 0x88, 0x8b, 0xca, 0x6b, 0xe1, 0x51, 
         0x31, 0x2d, 0x1b, 0x9e, 0x8f, 0x83, 0xbd, 0x0a, 0xca, 0xd6, 0x23, 0x4d, 
         0x3b, 0xcc, 0xc1, 0x1b, 0x63, 0xa4, 0x0d, 0x6f, 0xbf, 0xf4, 0x48, 0xf6, 
         0x7d, 0xb0, 0xb9, 0x1f},
      }},
      {TestType::NoReseedAndPr, {
        // output1
        {0x08, 0xe0, 0xa7, 0x5e, 0xb3, 0x83, 0x2e, 0xfe, 0xbf, 0xa9, 0x79, 0x98, 
         0x27, 0x12, 0xad, 0x1f, 0x31, 0x39, 0x86, 0x10, 0xf6, 0xaf, 0x6e, 0xfc, 
         0x0f, 0x9c, 0x18, 0x21, 0x50, 0x16, 0x4f, 0xf1, 0x96, 0x6e, 0x8b, 0xd3, 
         0x6f, 0x15, 0x39, 0x6b, 0x38, 0x29, 0x9c, 0x75, 0xf7, 0x34, 0x43, 0xc1, 
         0xa7, 0x69, 0x5e, 0x61, 0xce, 0xa2, 0x92, 0x05, 0x86, 0x7b, 0x95, 0x42, 
         0x24, 0x6e, 0x24, 0x5a},
        // output2
        {0x89, 0x80, 0xf5, 0x04, 0xbc, 0x05, 0xed, 0x77, 0xa7, 0x45, 0x7a, 0x5e, 
         0x41, 0x91, 0xd7, 0x70, 0xad, 0xcf, 0x61, 0xd8, 0x49, 0x74, 0xde, 0x76, 
         0x27, 0xd7, 0x21, 0x06, 0x4a, 0x0c, 0x63, 0xb5, 0xc4, 0x4e, 0xfb, 0x1d, 
         0x2b, 0xd7, 0x0c, 0x4e, 0x9b, 0x08, 0xbc, 0x02, 0xcb, 0x7e, 0xee, 0x3c, 
         0x03, 0xab, 0x59, 0x77, 0xc7, 0xbb, 0x8e, 0x4c, 0x90, 0xee, 0x83, 0x1e, 
         0x63, 0xd5, 0xf2, 0x4c},
      }},
      {TestType::NoReseedAndPrAndUserPr, {
        // output1
        {0x03, 0xa1, 0xe5, 0x62, 0x23, 0xd9, 0x9d, 0xef, 0x40, 0x41, 0x16, 0xab, 
         0x86, 0xbe, 0x6d, 0x5e, 0x10, 0x29, 0xc8, 0xb2, 0xc8, 0xfa, 0x37, 0xd0, 
         0x8d, 0xa5, 0xd5, 0xf4, 0x7d, 0x9c, 0x87, 0x15, 0x6f, 0x7c, 0x82, 0xe5, 
         0xab, 0xd8, 0x27, 0xb9, 0x82, 0xce, 0x9c, 0x83, 0xf2, 0x4d, 0x08, 0x02, 
         0x92, 0xfa, 0x7d, 0x4f, 0x0f, 0x93, 0x0d, 0x1b, 0xd1, 0x41, 0x28, 0xac, 
         0x2d, 0x06, 0x81, 0xf0},
        // output2
        {0x68, 0x30, 0x61, 0xfb, 0xde, 0x35, 0x3b, 0x5b, 0x6f, 0xc4, 0x75, 0x3d, 
         0x08, 0x07, 0xa5, 0xb9, 0x73, 0xce, 0x08, 0x0b, 0x27, 0x78, 0x96, 0xe5, 
         0x9d, 0xb2, 0x25, 0xcc, 0x38, 0x3d, 0x03, 0xf4, 0xc1, 0xae, 0x70, 0x1f, 
         0xbc, 0xbf, 0xbd, 0x28, 0x2d, 0x3c, 0x2b, 0x95, 0xd0, 0x96, 0xcb, 0xd5, 
         0xc0, 0xba, 0xb8, 0x7c, 0x71, 0x39, 0x44, 0x96, 0xb5, 0x3b, 0xe1, 0xf9, 
         0x59, 0xcd, 0xb2, 0xb8},
      }},
      {TestType::NoReseedNoPrAndUserPr, {
        // output1
        {0xcd, 0x7c, 0xba, 0x56, 0x74, 0x37, 0x7d, 0x9f, 0xb1, 0x43, 0x6f, 0xdf, 
         0xaa, 0x63, 0xa5, 0x12, 0x24, 0xec, 0x8e, 0xe8, 0x3a, 0xf0, 0x4a, 0xc7, 
         0xab, 0x3f, 0x57, 0x8d, 0xe8, 0xb4, 0x50, 0x41, 0x60, 0xfd, 0xd2, 0x5f, 
         0x0c, 0x04, 0x45, 0xca, 0x75, 0xf7, 0x71, 0x06, 0x2b, 0x78, 0xd3, 0xef, 
         0xcd, 0x4b, 0x4b, 0x24, 0xc1, 0xda, 0x9e, 0x24, 0x46, 0x5a, 0x4f, 0x2b, 
         0x08, 0x77, 0x35, 0x5f},
        // output2
        {0xac, 0x96, 0x12, 0x4f, 0x2c, 0x07, 0x93, 0x9c, 0x45, 0x67, 0x4b, 0x54, 
         0x69, 0x5a, 0x8a, 0x2c, 0x79, 0x3c, 0x7e, 0xef, 0xe5, 0x5c, 0xcb, 0x98, 
         0xbd, 0x0d, 0xef, 0xce, 0x53, 0x66, 0x6f, 0x26, 0xb6, 0xf7, 0x26, 0x23, 
         0xfc, 0x8b, 0x71, 0x1d, 0xd7, 0xcf, 0xb1, 0xce, 0xce, 0xb0, 0x87, 0x95, 
         0xd6, 0x8b, 0xa3, 0xf8, 0x52, 0xc7, 0xb7, 0xd1, 0x91, 0x1d, 0x4a, 0xc2, 
         0x4c, 0x33, 0x79, 0x33},
      }},
      {TestType::WithReseedNoPr, {
        // output1
        {0x8e, 0xe6, 0x11, 0xf4, 0x76, 0x67, 0xa6, 0xab, 0xb5, 0x52, 0x55, 0xda, 
         0x07, 0x77, 0x66, 0xd5, 0x8f, 0xb9, 0x5d, 0x9c, 0x83, 0xdb, 0x46, 0x90, 
         0x74, 0x65, 0xce, 0x99, 0x8f, 0x54, 0xfb, 0x3b, 0x41, 0x8c, 0x21, 0xd0, 
         0x2a, 0x74, 0x32, 0xbb, 0x05, 0x6e, 0x99, 0xcf, 0x00, 0xa1, 0x78, 0x22, 
         0xc6, 0x72, 0x1f, 0x48, 0xeb, 0x9a, 0x1d, 0x9f, 0xf2, 0xa1, 0x1c, 0xa0, 
         0x4c, 0x2c, 0x37, 0x0d},
        // output2
        {0x62, 0xb9, 0xd5, 0x94, 0x68, 0x6a, 0xce, 0x23, 0x33, 0xf1, 0xcb, 0xe9, 
         0x73, 0x2e, 0x15, 0xfd, 0x9f, 0x6d, 0xe5, 0xf5, 0x58, 0x9d, 0x2f, 0xc1, 
         0x41, 0xf6, 0x13, 0x2e, 0x2d, 0x64, 0x5c, 0x09, 0x3d, 0x9f, 0xa9, 0xf2, 
         0x2b, 0x91, 0xe1, 0x55, 0x07, 0x29, 0x1d, 0x97, 0xac, 0xb8, 0x6b, 0x4e, 
         0x85, 0xb4, 0x72, 0xdc, 0x32, 0x1b, 0x82, 0x11, 0xbb, 0x30, 0x20, 0xa7, 
         0xe7, 0xde, 0x71, 0x67},
      }},
      {TestType::WithReseedAndPr, {
        // output1
        {0x08, 0xe0, 0xa7, 0x5e, 0xb3, 0x83, 0x2e, 0xfe, 0xbf, 0xa9, 0x79, 0x98, 
         0x27, 0x12, 0xad, 0x1f, 0x31, 0x39, 0x86, 0x10, 0xf6, 0xaf, 0x6e, 0xfc, 
         0x0f, 0x9c, 0x18, 0x21, 0x50, 0x16, 0x4f, 0xf1, 0x96, 0x6e, 0x8b, 0xd3, 
         0x6f, 0x15, 0x39, 0x6b, 0x38, 0x29, 0x9c, 0x75, 0xf7, 0x34, 0x43, 0xc1, 
         0xa7, 0x69, 0x5e, 0x61, 0xce, 0xa2, 0x92, 0x05, 0x86, 0x7b, 0x95, 0x42, 
         0x24, 0x6e, 0x24, 0x5a},
        // output2
        {0x26, 0xaf, 0x50, 0x62, 0xec, 0xf2, 0xf9, 0x7b, 0x21, 0x31, 0xbf, 0x74, 
         0x68, 0xa1, 0x1a, 0xd9, 0x9a, 0x0d, 0xf6, 0x4f, 0x51, 0xa4, 0xaa, 0x61, 
         0x14, 0x7f, 0x5f, 0x68, 0x27, 0x80, 0xe4, 0x61, 0x34, 0xad, 0xa3, 0x8e, 
         0x7d, 0x03, 0x59, 0xab, 0x24, 0xbe, 0x96, 0x65, 0xd3, 0x64, 0x5d, 0x34, 
         0xdf, 0x77, 0xb2, 0x65, 0x57, 0xd2, 0xc3, 0xff, 0x40, 0xf8, 0xde, 0x8d, 
         0x0a, 0xb5, 0x98, 0x1f},
      }},
      {TestType::WithReseedAndPrAndUserPr, {
        // output1
        {0x03, 0xa1, 0xe5, 0x62, 0x23, 0xd9, 0x9d, 0xef, 0x40, 0x41, 0x16, 0xab, 
         0x86, 0xbe, 0x6d, 0x5e, 0x10, 0x29, 0xc8, 0xb2, 0xc8, 0xfa, 0x37, 0xd0, 
         0x8d, 0xa5, 0xd5, 0xf4, 0x7d, 0x9c, 0x87, 0x15, 0x6f, 0x7c, 0x82, 0xe5, 
         0xab, 0xd8, 0x27, 0xb9, 0x82, 0xce, 0x9c, 0x83, 0xf2, 0x4d, 0x08, 0x02, 
         0x92, 0xfa, 0x7d, 0x4f, 0x0f, 0x93, 0x0d, 0x1b, 0xd1, 0x41, 0x28, 0xac, 
         0x2d, 0x06, 0x81, 0xf0},
        // output2
        {0x9d, 0x72, 0x6a, 0xa2, 0xde, 0xbb, 0xfd, 0xab, 0x01, 0x05, 0xdc, 0xb3, 
         0xf6, 0x7d, 0x50, 0x18, 0xa3, 0x3a, 0xd5, 0xfb, 0x7c, 0xf9, 0xd4, 0x50, 
         0xad, 0x20, 0xee, 0x09, 0xcb, 0xc2, 0x71, 0x88, 0x1d, 0x5e, 0xbc, 0x56, 
         0xc0, 0x64, 0x5f, 0xe8, 0xe6, 0xe7, 0x76, 0xca, 0x7f, 0x8b, 0xfd, 0xd2, 
         0xf8, 0x16, 0x83, 0xfa, 0xe0, 0xf2, 0xa8, 0xed, 0x58, 0xdf, 0x73, 0xca, 
         0x38, 0x6a, 0xfa, 0xb6},
      }},
      {TestType::WithReseedNoPrAndUserPr, {
        // output1
        {0xcd, 0x7c, 0xba, 0x56, 0x74, 0x37, 0x7d, 0x9f, 0xb1, 0x43, 0x6f, 0xdf, 
         0xaa, 0x63, 0xa5, 0x12, 0x24, 0xec, 0x8e, 0xe8, 0x3a, 0xf0, 0x4a, 0xc7, 
         0xab, 0x3f, 0x57, 0x8d, 0xe8, 0xb4, 0x50, 0x41, 0x60, 0xfd, 0xd2, 0x5f, 
         0x0c, 0x04, 0x45, 0xca, 0x75, 0xf7, 0x71, 0x06, 0x2b, 0x78, 0xd3, 0xef, 
         0xcd, 0x4b, 0x4b, 0x24, 0xc1, 0xda, 0x9e, 0x24, 0x46, 0x5a, 0x4f, 0x2b, 
         0x08, 0x77, 0x35, 0x5f},
        // output2
        {0x58, 0x7f, 0xbe, 0x6a, 0x63, 0x4a, 0x2f, 0x2f, 0x14, 0xd1, 0x40, 0xb1, 
         0x9d, 0xa1, 0x22, 0xd4, 0x41, 0x42, 0x62, 0xc8, 0x8f, 0xb6, 0xd5, 0x91, 
         0xa6, 0x7c, 0x89, 0x00, 0x87, 0xa1, 0xd5, 0xeb, 0x8f, 0x4f, 0x2d, 0xc8, 
         0x0a, 0x95, 0x62, 0x2b, 0xfa, 0x1e, 0x51, 0x00, 0xb5, 0x26, 0x05, 0xdd, 
         0xbe, 0xff, 0x40, 0xe8, 0x73, 0x1d, 0xde, 0xa4, 0x86, 0xfd, 0x58, 0x31, 
         0x5c, 0x78, 0xcd, 0x16},
      }},
    };
    return outputs.at(type);
  }

  static void printVector(const std::vector<uint8_t>& v, const char *label) {
    printf("%s: ", label);
    for (uint8_t byte : v) {
      printf("%02x ", byte);
    }
    printf("\n");
  }
};

class RngKatTestEnv {
public:
  explicit RngKatTestEnv(TestType type) : kat_test_type_(type) {
    setupEntropyMethods();
  }

  bool runTest() {
    const auto& expected = RngKatTestData::getExpectedOutput(kat_test_type_);

    current_generation_state = GenerationState::First;
    bool test1 = generateRandomness(expected.output1, "gen 1");

    current_generation_state = GenerationState::Second;
    bool test2 = generateRandomness(expected.output2, "gen 2");

    return test1 && test2;
  }

private:
  TestType kat_test_type_;
  entropy_source_methods entropy_methods_;
  using PredictionResistanceCallback = int (*)(const struct entropy_source_t*,
                                             uint8_t[RAND_PRED_RESISTANCE_LEN]);

  static int overrideInitializeKat(struct entropy_source_t *entropy_source) {
    return 1;
  }

  static void overrideZeroizeThreadKat(struct entropy_source_t *entropy_source) {}
  static void overrideFreeThreadKat(struct entropy_source_t *entropy_source) {}

  static int overrideGetSeedKat(const struct entropy_source_t *entropy_source,
                               uint8_t seed[CTR_DRBG_ENTROPY_LEN]) {
    const auto& vectors = RngKatTestData::getTestVectors();
    switch (current_generation_state) {
      case GenerationState::First:
        std::copy(vectors.seed.begin(), vectors.seed.end(), seed);
        return 1;
      case GenerationState::Second:
        std::copy(vectors.reseed.begin(), vectors.reseed.end(), seed);
        return 1;
      default:
        return 0;
    }
  }

  static int overrideGetExtraEntropyKat(const struct entropy_source_t *entropy_source,
                                       uint8_t seed[CTR_DRBG_ENTROPY_LEN]) {
    const auto& vectors = RngKatTestData::getTestVectors();
    switch (current_generation_state) {
      case GenerationState::First:
        std::copy(vectors.seedPersonalization.begin(),
                  vectors.seedPersonalization.end(), seed);
        return 1;
      case GenerationState::Second:
        std::copy(vectors.reseedPersonalization.begin(),
                  vectors.reseedPersonalization.end(), seed);
        return 1;
      default:
        return 0;
    }
  }

  static int overrideGetPredictionResistanceKat(
    const struct entropy_source_t *entropy_source,
    uint8_t seed[RAND_PRED_RESISTANCE_LEN]) {
    const auto& vectors = RngKatTestData::getTestVectors();
    switch (current_generation_state) {
      case GenerationState::First:
        std::copy(vectors.predictionResistance1.begin(),
                  vectors.predictionResistance1.end(), seed);
        return 1;
      case GenerationState::Second:
        std::copy(vectors.predictionResistance2.begin(),
                  vectors.predictionResistance2.end(), seed);
        return 1;
      default:
        return 0;
    }
  }

  static PredictionResistanceCallback getPredictionResistanceCallback(TestType type) {
    if (type == TestType::NoReseedAndPrAndUserPr ||
        type == TestType::WithReseedAndPrAndUserPr ||
        type == TestType::NoReseedAndPr ||
        type == TestType::WithReseedAndPr) {
      return &overrideGetPredictionResistanceKat;
    }

    return nullptr;
  }

  void setupEntropyMethods() {
    entropy_methods_ = {
      &overrideInitializeKat,
      &overrideZeroizeThreadKat,
      &overrideFreeThreadKat,
      &overrideGetSeedKat,
      &overrideGetExtraEntropyKat,
      getPredictionResistanceCallback(kat_test_type_),
      OVERRIDDEN_ENTROPY_SOURCE
    };
    override_entropy_source_method_FOR_TESTING(&entropy_methods_);
  }

  bool generateRandomness(const std::vector<uint8_t>& expected_output,
                        const char* error_text) {
    std::vector<uint8_t> output(RngKatTestData::KAT_GENERATE_REQ_LEN);

    bool success;
    if (kat_test_type_ == TestType::NoReseedAndPrAndUserPr ||
        kat_test_type_ == TestType::NoReseedNoPrAndUserPr ||
        kat_test_type_ == TestType::WithReseedAndPrAndUserPr ||
        kat_test_type_ == TestType::WithReseedNoPrAndUserPr) {
      const auto& vectors = RngKatTestData::getTestVectors();
      const std::array<uint8_t, RAND_PRED_RESISTANCE_LEN>* pr = nullptr;
      switch (current_generation_state) {
        case GenerationState::First:
          pr = &vectors.userPredictionResistance1;
          break;
        case GenerationState::Second:
          pr = &vectors.userPredictionResistance2;
          break;
        default:
          return false;
      }
      success = RAND_bytes_with_user_prediction_resistance(output.data(),
        output.size(), pr->data());
    } else {
      success = RAND_bytes(output.data(), output.size());
    }

    if (!success) {
      std::cerr << "Generating randomness failed " << error_text << '\n';
      return false;
    }

    if (expected_output != output) {
      std::cerr << "Generated randomness not equal to expected value "
                << error_text << '\n';
      RngKatTestData::printVector(output, "output");
      RngKatTestData::printVector(expected_output, "expected_output");
      return false;
    }

    return true;
  }
};

} // namespace RngKatTestUtils

TEST_F(randIsolatedTest, RngKatWithUbe) {
  if (!UbeIsSupported()) {
    GTEST_SKIP() << "Test not supported when UBE is not supported";
  }

  if (runtimeEmulationIsIntelSde() && addressSanitizerIsEnabled()) {
    GTEST_SKIP() << "Test not supported under Intel SDE + ASAN";
  }

  auto runTest = [](RngKatTestUtils::TestType type) {
    RngKatTestUtils::RngKatTestEnv env(type);
    exit(env.runTest() ? 0 : 1);
  };

  EXPECT_EXIT(runTest(RngKatTestUtils::TestType::NoReseedNoPr),
              ::testing::ExitedWithCode(0), "");
  EXPECT_EXIT(runTest(RngKatTestUtils::TestType::NoReseedAndPr),
              ::testing::ExitedWithCode(0), "");
  EXPECT_EXIT(runTest(RngKatTestUtils::TestType::NoReseedAndPrAndUserPr),
              ::testing::ExitedWithCode(0), "");
  EXPECT_EXIT(runTest(RngKatTestUtils::TestType::NoReseedNoPrAndUserPr),
              ::testing::ExitedWithCode(0), "");
}

TEST_F(randIsolatedTest, RngKatNoUbe) {
  if (UbeIsSupported()) {
    GTEST_SKIP() << "Test not supported when UBE is supported";
  }

  if (runtimeEmulationIsIntelSde() && addressSanitizerIsEnabled()) {
    GTEST_SKIP() << "Test not supported under Intel SDE + ASAN";
  }

  auto runTest = [](RngKatTestUtils::TestType type) {
    RngKatTestUtils::RngKatTestEnv env(type);
    exit(env.runTest() ? 0 : 1);
  };

  EXPECT_EXIT(runTest(RngKatTestUtils::TestType::WithReseedNoPr),
              ::testing::ExitedWithCode(0), "");
  EXPECT_EXIT(runTest(RngKatTestUtils::TestType::WithReseedAndPr),
              ::testing::ExitedWithCode(0), "");
  EXPECT_EXIT(runTest(RngKatTestUtils::TestType::WithReseedAndPrAndUserPr),
              ::testing::ExitedWithCode(0), "");
  EXPECT_EXIT(runTest(RngKatTestUtils::TestType::WithReseedNoPrAndUserPr),
              ::testing::ExitedWithCode(0), "");
}

#else // GTEST_HAS_DEATH_TEST

TEST(randIsolatedTest, SkippedALL) {
  GTEST_SKIP() << "All randIsolatedTest tests are not supported due to Death Tests not supported on this platform";
}

#endif
