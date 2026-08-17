// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#if !defined(DISABLE_CPU_JITTER_ENTROPY)

#include <gtest/gtest.h>

#include "internal.h"
#include "../internal.h"
#include "../../../ube/internal.h"

#include "../../../test/ube_test.h"
#include "../../../test/test_util.h"

#include <cstdio>

// Use `#if GTEST_HAS_DEATH_TEST` (not `defined()`); some toolchains define it to 0.
#if GTEST_HAS_DEATH_TEST

#define TEST_IN_FORK_ASSERT_TRUE(condition) if (!condition) { std::cerr << __FILE__ << ":" << __LINE__ << ": Assertion failed: " << #condition << std::endl;; exit(1);}
#define TEST_IN_FORK_ASSERT_FALSE(condition) if (condition) { std::cerr << __FILE__ << ":" << __LINE__ << ": Assertion failed: " << #condition << std::endl;; exit(1);}

static const size_t number_of_threads = 8;

class treeDrbgJitterentropyTest : public::testing::Test {
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

static bool get_tree_drbg_call(const struct entropy_source_t *entropy_source,
  struct test_tree_drbg_t *test_tree_drbg) {
  if (get_thread_and_global_tree_drbg_calls_FOR_TESTING(
    entropy_source, test_tree_drbg)) {
    return true;
  }
  return false;
}

TEST_F(treeDrbgJitterentropyTest, BasicInitialization) {

  if (runtimeEmulationIsIntelSde() && addressSanitizerIsEnabled()) {
    GTEST_SKIP() << "Test not supported under Intel SDE + ASAN";
  }

  // Test only one seed occurs on initialization.
  auto testFunc = []() {

    struct entropy_source_t entropy_source = {0, 0};
    struct test_tree_drbg_t new_test_tree_drbg = {0, 0, 0, 0};

    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_initialize(&entropy_source))
    TEST_IN_FORK_ASSERT_TRUE(get_tree_drbg_call(&entropy_source, &new_test_tree_drbg))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_reseed_calls_since_initialization == 1))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_reseed_calls_since_initialization == 1))
    // Calling tree_jitter_initialize before thread test would set
    // |thread_generate_calls_since_seed| equal to 1 and
    // |global_generate_calls_since_seed| equal to 2. The latter because the
    // initial value 1 and we perform a generate call on the global tree-DRBG
    // to seed the thread-local tree-DRBG.
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_generate_calls_since_seed == 1))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_generate_calls_since_seed == 2))

    tree_jitter_zeroize_thread_drbg(&entropy_source);
    tree_jitter_free_thread_drbg(&entropy_source);

    exit(0);
  };

  EXPECT_EXIT(testFunc(), ::testing::ExitedWithCode(0), "");
}

TEST_F(treeDrbgJitterentropyTest, BasicThread) {

  // Test seeds are observed when spawning new threads.
  auto testFunc = []() {

    struct entropy_source_t entropy_source = {0, 0};
    struct test_tree_drbg_t new_test_tree_drbg = {0, 0, 0, 0};
    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_initialize(&entropy_source))

    std::function<void(bool*)> threadFunc = [](bool *result) {
      struct entropy_source_t entropy_source_thread = {0, 0};
      struct test_tree_drbg_t new_test_tree_drbg_thread = {0, 0, 0, 0};

      bool test = tree_jitter_initialize(&entropy_source_thread);
      test = test && get_tree_drbg_call(&entropy_source_thread, &new_test_tree_drbg_thread);
      test = test && new_test_tree_drbg_thread.thread_reseed_calls_since_initialization == 1;
      test = test && new_test_tree_drbg_thread.global_reseed_calls_since_initialization == 1;
      *result = test;

      tree_jitter_free_thread_drbg(&entropy_source_thread);
    };

    bool exit_code = threadTest(number_of_threads, threadFunc);
    TEST_IN_FORK_ASSERT_TRUE(exit_code)
    TEST_IN_FORK_ASSERT_TRUE(get_tree_drbg_call(&entropy_source, &new_test_tree_drbg))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_reseed_calls_since_initialization == 1))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_reseed_calls_since_initialization == 1))
    // Calling tree_jitter_initialize before thread test would set
    // |global_generate_calls_since_seed| equal to 2. We then expect an
    // additional |number_of_threads| thread-local tree-DRBGs to seed using the
    // global tree-DRBG.
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_generate_calls_since_seed == (number_of_threads+2)))

    tree_jitter_zeroize_thread_drbg(&entropy_source);
    tree_jitter_free_thread_drbg(&entropy_source);
    exit(0);
  };

  EXPECT_EXIT(testFunc(), ::testing::ExitedWithCode(0), "");
}

TEST_F(treeDrbgJitterentropyTest, BasicReseed) {

  if (runtimeEmulationIsIntelSde() && addressSanitizerIsEnabled()) {
    GTEST_SKIP() << "Test not supported under Intel SDE + ASAN";
  }

  // Test reseeding happens as expected
  auto testFunc = []() {

    struct entropy_source_t entropy_source = {0, 0};
    struct test_tree_drbg_t new_test_tree_drbg = {0, 0, 0, 0};
    uint8_t seed_out[CTR_DRBG_ENTROPY_LEN];
    const uint64_t tree_drbg_thread_reseed_limit = TREE_JITTER_THREAD_DRBG_MAX_GENERATE;
    const uint64_t tree_drbg_global_reseed_limit = TREE_JITTER_GLOBAL_DRBG_MAX_GENERATE;

    // Similar to initialization test above.
    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_initialize(&entropy_source))
    TEST_IN_FORK_ASSERT_TRUE(get_tree_drbg_call(&entropy_source, &new_test_tree_drbg))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_reseed_calls_since_initialization == 1))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_generate_calls_since_seed == 1))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_reseed_calls_since_initialization == 1))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_generate_calls_since_seed == 2))

    // Set reseed counter for thread-local tree-DRBG to max value + 1 (because
    // the reseed interval condition uses strict inequality and
    // drbg.reseed_counter is initialized to 1).
    TEST_IN_FORK_ASSERT_TRUE(set_thread_and_global_tree_drbg_reseed_counter_FOR_TESTING(
      &entropy_source, tree_drbg_thread_reseed_limit + 1, 0))
    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_get_seed(&entropy_source, seed_out))
    TEST_IN_FORK_ASSERT_TRUE(get_tree_drbg_call(&entropy_source, &new_test_tree_drbg))
    // Thread-local tree-DRBG should generate a seed from global tree-DRBG
    // causing its generate call counter to increment by 1. Thread-local
    // tree-DRBG reseed counter should also go increment by 1.
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_reseed_calls_since_initialization == 2)) // changed
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_generate_calls_since_seed == 2)) // changed
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_reseed_calls_since_initialization == 1)) // unchanged
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_generate_calls_since_seed == 3)) // changed

    // Set reseed counter for global tree-DRBG to max value + 1. Thread-local
    // tree-DRBG is unchanged
    TEST_IN_FORK_ASSERT_TRUE(set_thread_and_global_tree_drbg_reseed_counter_FOR_TESTING(
      &entropy_source, 0, tree_drbg_global_reseed_limit + 1))
    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_get_seed(&entropy_source, seed_out))
    TEST_IN_FORK_ASSERT_TRUE(get_tree_drbg_call(&entropy_source, &new_test_tree_drbg))
    // We generated a seed from the tread-local tree-DRBG which should not
    // reseed. Hence, we do not expect a generate call made to the global
    // tree-DRBG. The value of the latter will change though because the reseed
    // counter is equal to the number of generate calls. Since we are generating
    // a seed from the thread-local tree-DRBG its generate counter should increment by 1
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_reseed_calls_since_initialization == 2)) // unchanged
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_generate_calls_since_seed == 3)) // changed
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_reseed_calls_since_initialization == 1)) // unchanged
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_generate_calls_since_seed == (tree_drbg_global_reseed_limit + 1))) // changed

    // Set reseed counter for both thread-local and global tree-DRBG to
    // max value + 1.
    TEST_IN_FORK_ASSERT_TRUE(set_thread_and_global_tree_drbg_reseed_counter_FOR_TESTING(
      &entropy_source, tree_drbg_thread_reseed_limit + 1, tree_drbg_global_reseed_limit + 1))
    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_get_seed(&entropy_source, seed_out))
    TEST_IN_FORK_ASSERT_TRUE(get_tree_drbg_call(&entropy_source, &new_test_tree_drbg))
    // When generating a seed from from the thread-local tree-DRBG it should
    // reseed by getting a seed from the global tree-DRBG. The global tree-DRBG
    // should itself reseed. In both cases, their generate calls (since last
    // seed/reseed) should be reset.
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_reseed_calls_since_initialization == 3)) // changed
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_generate_calls_since_seed == 2)) // changed
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_reseed_calls_since_initialization == 2)) // changed
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_generate_calls_since_seed == 2)) // changed

    // Try without calling zeroize thread-local tree-DRBG first.
    tree_jitter_free_thread_drbg(&entropy_source);
    exit(0);
  };

  EXPECT_EXIT(testFunc(), ::testing::ExitedWithCode(0), "");
}

#if !defined(OPENSSL_WINDOWS)

static bool veryifySeedOrReseed(const struct test_tree_drbg_t *test_tree_drbg,
  const struct test_tree_drbg_t *cached_test_tree_drbg,
  size_t expect_reseed_thread, size_t expect_reseed_global,
  const char *error_text) {

  if (cached_test_tree_drbg->thread_reseed_calls_since_initialization + expect_reseed_thread != test_tree_drbg->thread_reseed_calls_since_initialization ||
      cached_test_tree_drbg->global_reseed_calls_since_initialization + expect_reseed_global != test_tree_drbg->global_reseed_calls_since_initialization) {
    std::cerr << "Tree-DRBG expected count mismatch " << error_text << '\n'
              << "  Thread DRBG: expected=" << (cached_test_tree_drbg->thread_reseed_calls_since_initialization + expect_reseed_thread)
              << ", actual=" << test_tree_drbg->thread_reseed_calls_since_initialization << '\n'
              << "  Global DRBG: expected=" << (cached_test_tree_drbg->global_reseed_calls_since_initialization + expect_reseed_global)
              << ", actual=" << test_tree_drbg->global_reseed_calls_since_initialization << '\n';
    return false;
  }

  return true;
}

static bool assertSeedOrReseed(const struct entropy_source_t *entropy_source,
  size_t expect_reseed_thread, size_t expect_reseed_global,
  std::function<bool()> func, const char *error_text = "") {

  struct test_tree_drbg_t cached_test_tree_drbg = {0, 0, 0, 0};
  TEST_IN_FORK_ASSERT_TRUE(get_tree_drbg_call(entropy_source, &cached_test_tree_drbg))

  TEST_IN_FORK_ASSERT_TRUE(func())

  struct test_tree_drbg_t test_tree_drbg = {0, 0, 0, 0};
  TEST_IN_FORK_ASSERT_TRUE(get_tree_drbg_call(entropy_source, &test_tree_drbg))

  return veryifySeedOrReseed(&test_tree_drbg, &cached_test_tree_drbg,
    expect_reseed_thread, expect_reseed_global, error_text);
}

TEST_F(treeDrbgJitterentropyTest, BasicFork) {

  if (runtimeEmulationIsIntelSde() && addressSanitizerIsEnabled()) {
    GTEST_SKIP() << "Test not supported under Intel SDE + ASAN";
  }

  auto testFuncSingleFork = [this]() {
    struct entropy_source_t entropy_source = {0, 0};
    uint8_t seed_out[CTR_DRBG_ENTROPY_LEN];

    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_initialize(&entropy_source))
    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_get_seed(&entropy_source, seed_out))

    bool exit_code = forkAndRunTest(
      [this, entropy_source]() {
        // In child. If UBE detection is supported, we expect a reseed.
        // No UBE detection is handled via prediction resistance.
        size_t expect_reseed = 0;
        if (UbeIsSupported()) {
          expect_reseed = 1;
        }

        TEST_IN_FORK_ASSERT_TRUE(
          assertSeedOrReseed(&entropy_source, expect_reseed, expect_reseed, [entropy_source]() {
            uint8_t child_out[CTR_DRBG_ENTROPY_LEN];
            return tree_jitter_get_seed(&entropy_source, child_out);
          }, "child")
        )

        return true;
      },
      [entropy_source]() {
        // In Parent we expect no reseed, even if UBE detection is not supported.
        TEST_IN_FORK_ASSERT_TRUE(
          assertSeedOrReseed(&entropy_source, 0, 0, [entropy_source]() {
            uint8_t parent_out[CTR_DRBG_ENTROPY_LEN];
            return tree_jitter_get_seed(&entropy_source, parent_out);
          }, "parent")
        )

        return true;
      }
    );

    tree_jitter_free_thread_drbg(&entropy_source);
    exit(exit_code ? 0 : 1);
  };

  EXPECT_EXIT(testFuncSingleFork(), ::testing::ExitedWithCode(0), "");

  auto testFuncSingleForkThenThread = [this]() {
    struct entropy_source_t entropy_source = {0, 0};
    uint8_t seed_out[CTR_DRBG_ENTROPY_LEN];

    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_initialize(&entropy_source))
    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_get_seed(&entropy_source, seed_out))

    bool exit_code = forkAndRunTest(
      [this, entropy_source]() {
        // In child. Spawn a number of threads before generating randomness.
        // If fork detection is supported, we expect a seed in each thread.
        // If fork detection is not enabled, we also expect a seed in each
        // thread. However, this seed should occur when calling
        // tree_jitter_initialize.
        std::function<void(bool*)> threadFunc = [](bool *result) {

          struct entropy_source_t thread_entropy_source = {0, 0};
          TEST_IN_FORK_ASSERT_TRUE(tree_jitter_initialize(&thread_entropy_source))
          TEST_IN_FORK_ASSERT_TRUE(
            assertSeedOrReseed(&thread_entropy_source, 0, 0, [thread_entropy_source]() {
              uint8_t child_out[CTR_DRBG_ENTROPY_LEN];
              return tree_jitter_get_seed(&thread_entropy_source, child_out);
            }, "child")
          )

          tree_jitter_free_thread_drbg(&thread_entropy_source);
          *result = true;
        };

        TEST_IN_FORK_ASSERT_TRUE(threadTest(number_of_threads, threadFunc))

        // Now back to original thread.
        size_t expect_reseed = 0;
        if (UbeIsSupported()) {
          expect_reseed = 1;
        }

        // Global would have been reseeded above.
        TEST_IN_FORK_ASSERT_TRUE(
          assertSeedOrReseed(&entropy_source, expect_reseed, 0, [entropy_source]() {
            uint8_t child_out[CTR_DRBG_ENTROPY_LEN];
            return tree_jitter_get_seed(&entropy_source, child_out);
          }, "child")
        )

        return true;
      },
      [entropy_source]() {
        // In Parent we expect no reseed, even if UBE detection is not supported.
        TEST_IN_FORK_ASSERT_TRUE(
          assertSeedOrReseed(&entropy_source, 0, 0, [entropy_source]() {
            uint8_t child_out[CTR_DRBG_ENTROPY_LEN];
            return tree_jitter_get_seed(&entropy_source, child_out);
          }, "parent")
        )

        return true;
      }
    );

    tree_jitter_free_thread_drbg(&entropy_source);
    exit(exit_code ? 0 : 1);
  };

  EXPECT_EXIT(testFuncSingleForkThenThread(), ::testing::ExitedWithCode(0), "");

  // Test reseed is observed when forking and then forking again before
  // generating any randomness.
  auto testFuncDoubleFork = [this]() {
    struct entropy_source_t entropy_source = {0, 0};
    uint8_t seed_out[CTR_DRBG_ENTROPY_LEN];

    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_initialize(&entropy_source))
    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_get_seed(&entropy_source, seed_out))

    bool exit_code = forkAndRunTest(
      [this, entropy_source]() {

        // Fork again. In both child and parent we expect a reseed if UBE
        // detection is supported.
        bool exit_code_child = forkAndRunTest(
          [this, entropy_source]() {
            size_t expect_reseed = 0;
            if (UbeIsSupported()) {
              expect_reseed = 1;
            }

            TEST_IN_FORK_ASSERT_TRUE(
              assertSeedOrReseed(&entropy_source, expect_reseed, expect_reseed, [entropy_source]() {
                uint8_t child_out[CTR_DRBG_ENTROPY_LEN];
                return tree_jitter_get_seed(&entropy_source, child_out);
              }, "child-child")
            )

            return true;
          },
          [this, entropy_source]() {
            size_t expect_reseed = 0;
            if (UbeIsSupported()) {
              expect_reseed = 1;
            }

            TEST_IN_FORK_ASSERT_TRUE(
              assertSeedOrReseed(&entropy_source, expect_reseed, expect_reseed, [entropy_source]() {
                uint8_t child_out[CTR_DRBG_ENTROPY_LEN];
                return tree_jitter_get_seed(&entropy_source, child_out);
              }, "child-parent")
            )

            return true;
          }
        );

        return exit_code_child;
      },
      [entropy_source]() {
        // In Parent we expect no reseed, even if UBE detection is not supported.
        TEST_IN_FORK_ASSERT_TRUE(
          assertSeedOrReseed(&entropy_source, 0, 0, [entropy_source]() {
            uint8_t parent_out[CTR_DRBG_ENTROPY_LEN];
            return tree_jitter_get_seed(&entropy_source, parent_out);
          }, "parent")
        )

        return true;
      }
    );

    tree_jitter_free_thread_drbg(&entropy_source);
    exit(exit_code ? 0 : 1);
  };

  EXPECT_EXIT(testFuncDoubleFork(), ::testing::ExitedWithCode(0), "");

  // Test reseed is observed when forking, generate randomness, and then fork
  // again.
  auto testFuncForkGenerateFork = [this]() {

    struct entropy_source_t entropy_source = {0, 0};
    uint8_t seed_out[CTR_DRBG_ENTROPY_LEN];

    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_initialize(&entropy_source))
    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_get_seed(&entropy_source, seed_out))

    bool exit_code = forkAndRunTest(
      [this, entropy_source]() {

        size_t expect_reseed = 0;
        if (UbeIsSupported()) {
          expect_reseed = 1;
        }

        TEST_IN_FORK_ASSERT_TRUE(
          assertSeedOrReseed(&entropy_source, expect_reseed, expect_reseed, [entropy_source]() {
            uint8_t parent_out[CTR_DRBG_ENTROPY_LEN];
            return tree_jitter_get_seed(&entropy_source, parent_out);
          }, "child-parent")
        )

        bool exit_code_child = forkAndRunTest(
          [this, entropy_source]() {
            size_t expect_reseed_child = 0;
            if (UbeIsSupported()) {
              expect_reseed_child = 1;
            }
            TEST_IN_FORK_ASSERT_TRUE(
              assertSeedOrReseed(&entropy_source, expect_reseed_child, expect_reseed_child, [entropy_source]() {
                uint8_t parent_out[CTR_DRBG_ENTROPY_LEN];
                return tree_jitter_get_seed(&entropy_source, parent_out);
              }, "child-child")
            )
            return true;
          },
          [entropy_source]() {
            TEST_IN_FORK_ASSERT_TRUE(
              assertSeedOrReseed(&entropy_source, 0, 0, [entropy_source]() {
                uint8_t parent_out[CTR_DRBG_ENTROPY_LEN];
                return tree_jitter_get_seed(&entropy_source, parent_out);
              }, "child-parent")
            )
            return true;
          }
        );

        return exit_code_child;
      },
      [entropy_source]() {
        TEST_IN_FORK_ASSERT_TRUE(
          assertSeedOrReseed(&entropy_source, 0, 0, [entropy_source]() {
            uint8_t parent_out[CTR_DRBG_ENTROPY_LEN];
            return tree_jitter_get_seed(&entropy_source, parent_out);
          }, "parent")
        )

        return true;
      }
    );

    exit(exit_code ? 0 : 1);
  };

  EXPECT_EXIT(testFuncForkGenerateFork(), ::testing::ExitedWithCode(0), "");
}

#endif // !defined(OPENSSL_WINDOWS)

TEST_F(treeDrbgJitterentropyTest, TreeDRBGThreadReseedInterval) {

  if (runtimeEmulationIsIntelSde() && addressSanitizerIsEnabled()) {
    GTEST_SKIP() << "Test not supported under Intel SDE + ASAN";
  }

  // Test reseeding happens as expected
  auto testFunc = []() {

    struct entropy_source_t entropy_source = {0, 0};
    struct test_tree_drbg_t new_test_tree_drbg = {0, 0, 0, 0};
    uint8_t seed_out[CTR_DRBG_ENTROPY_LEN];
    const uint64_t tree_drbg_thread_reseed_limit = TREE_JITTER_THREAD_DRBG_MAX_GENERATE;

    // Similar to initialization test above.
    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_initialize(&entropy_source))
    TEST_IN_FORK_ASSERT_TRUE(get_tree_drbg_call(&entropy_source, &new_test_tree_drbg))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_reseed_calls_since_initialization == 1))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_generate_calls_since_seed == 1))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_reseed_calls_since_initialization == 1))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_generate_calls_since_seed == 2))

    // Must allow |tree_drbg_thread_reseed_limit| generate calls before
    // reseeding. For the tree-DRBG, not having UBE detection does not trigger
    // a pre-invocation reseed. Instead, prediction resistance is used. Hence,
    // we do not need to cater for UBE in the logic below.
    for (size_t i = 1; i <= tree_drbg_thread_reseed_limit; i++) {
      TEST_IN_FORK_ASSERT_TRUE(tree_jitter_get_seed(&entropy_source, seed_out))
      TEST_IN_FORK_ASSERT_TRUE(get_tree_drbg_call(&entropy_source, &new_test_tree_drbg))
      TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_reseed_calls_since_initialization == 1))
      TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_generate_calls_since_seed == (1 + i)))
      TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_reseed_calls_since_initialization == 1))
      TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_generate_calls_since_seed == 2))
    }

    // Now reseed should happen.
    TEST_IN_FORK_ASSERT_TRUE(tree_jitter_get_seed(&entropy_source, seed_out))
    TEST_IN_FORK_ASSERT_TRUE(get_tree_drbg_call(&entropy_source, &new_test_tree_drbg))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_reseed_calls_since_initialization == 2))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.thread_generate_calls_since_seed == 2)) // Because drbg.reseed_counter is initialized to 1
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_reseed_calls_since_initialization == 1))
    TEST_IN_FORK_ASSERT_TRUE((new_test_tree_drbg.global_generate_calls_since_seed == 3))

    // Try without calling zeroize thread-local tree-DRBG first.
    tree_jitter_free_thread_drbg(&entropy_source);
    exit(0);
  };

  EXPECT_EXIT(testFunc(), ::testing::ExitedWithCode(0), "");
}

#else // GTEST_HAS_DEATH_TEST

TEST(treeDrbgJitterentropyTest, SkippedALL) {
  GTEST_SKIP() << "All treeDrbgJitterentropyTest tests are not supported due to Death Tests not supported on this platform";
}

#endif

#endif // !defined(DISABLE_CPU_JITTER_ENTROPY)
