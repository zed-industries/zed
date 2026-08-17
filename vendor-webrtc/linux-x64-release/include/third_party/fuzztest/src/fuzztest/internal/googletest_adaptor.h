// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef FUZZTEST_FUZZTEST_GOOGLETEST_ADAPTOR_H_
#define FUZZTEST_FUZZTEST_GOOGLETEST_ADAPTOR_H_

#include <cstdlib>
#include <string>
#include <utility>
#include <vector>

#include "gtest/gtest.h"
#include "absl/strings/string_view.h"
#include "./fuzztest/internal/configuration.h"
#include "./fuzztest/internal/registry.h"
#include "./fuzztest/internal/runtime.h"

namespace fuzztest::internal {

class GTest_TestAdaptor : public ::testing::Test {
 public:
  explicit GTest_TestAdaptor(FuzzTest& test, int* argc, char*** argv,
                             Configuration configuration)
      : test_(test),
        argc_(argc),
        argv_(argv),
        configuration_(std::move(configuration)) {}

  void TestBody() override {
    auto test = test_.make();
    configuration_.fuzz_tests_in_current_shard = GetFuzzTestsInCurrentShard();
    configuration_.replay_in_single_process =
        configuration_.crashing_input_to_reproduce.has_value() &&
        testing::UnitTest::GetInstance()->test_to_run_count() == 1;
    if (Runtime::instance().run_mode() == RunMode::kUnitTest) {
      // In "bug reproduction" mode, sometimes we need to reproduce multiple
      // bugs, i.e., run multiple tests that lead to a crash.
      bool needs_subprocess = false;
#if defined(GTEST_HAS_DEATH_TEST) && !defined(FUZZTEST_USE_CENTIPEDE)
      needs_subprocess =
          configuration_.crashing_input_to_reproduce.has_value() &&
          (!configuration_.replay_in_single_process ||
           // EXPECT_EXIT is required in the death-test subprocess, but in
           // the subprocess there's only one test to run.
           testing::internal::InDeathTestChild());
#endif
      if (needs_subprocess) {
        configuration_.preprocess_crash_reproducing = [] {
          // EXPECT_EXIT disables event forwarding in gtest and as a result,
          // EXPECT/ASSERT-s are disabled. Here, we overwrite this option.
          testing::UnitTest::GetInstance()->listeners().SuppressEventForwarding(
              false);
        };
        // `RunInUnitTestMode` is supposed to fail and we wish to show the
        // failure to the user. Directly running the test would terminate the
        // process and using `EXPECT_DEATH` causes the test to pass. We use
        // `EXPECT_EXIT` so that the test exit unsuccessfully, meaning that the
        // test below fails without terminating the process.
#ifdef GTEST_HAS_DEATH_TEST
        EXPECT_EXIT(
            (test->RunInUnitTestMode(configuration_),
             void(
                 R"( FuzzTest failure! Please see 'actual message' below for the crash report. )"),
             std::exit(0)),
            ::testing::ExitedWithCode(0), "");
#else
        EXPECT_TRUE(false) << "Death test is not supported.";
#endif
      } else {
        EXPECT_TRUE(test->RunInUnitTestMode(configuration_))
            << "Failure(s) found in the unit-test mode.";
      }
    } else {
      // TODO(b/245753736): Consider using `tolerate_failure` when FuzzTest can
      // tolerate crashes in fuzzing mode.
      EXPECT_TRUE(test->RunInFuzzingMode(argc_, argv_, configuration_))
          << "Failure(s) found in the fuzzing mode.";
    }
  }

  static void SetUpTestSuite() {
    SetUpTearDownTestSuiteFunction set_up_test_suite = GetSetUpTestSuite(
        testing::UnitTest::GetInstance()->current_test_suite()->name());
    if (set_up_test_suite != nullptr) set_up_test_suite();
  }

  static void TearDownTestSuite() {
    SetUpTearDownTestSuiteFunction tear_down_test_suite = GetTearDownTestSuite(
        testing::UnitTest::GetInstance()->current_test_suite()->name());
    if (tear_down_test_suite != nullptr) tear_down_test_suite();
  }

 private:
  std::vector<std::string> GetFuzzTestsInCurrentShard() const;

  FuzzTest& test_;
  int* argc_;
  char*** argv_;
  Configuration configuration_;
};

template <typename Base, typename TestPartResult>
class GTest_EventListener : public Base {
 public:
  void OnTestPartResult(const TestPartResult& test_part_result) override {
    if (!test_part_result.failed()) return;
    Runtime& runtime = Runtime::instance();
    runtime.SetCrashTypeIfUnset("GoogleTest assertion failure");
    if (runtime.run_mode() == RunMode::kFuzz) {
      if (runtime.should_terminate_on_non_fatal_failure()) {
        // The SIGABRT will trigger a report.
        std::abort();
      }
    } else {
      // Otherwise, we report it manually.
      runtime.PrintReportOnDefaultSink();
    }
    runtime.SetExternalFailureDetected(true);
  }
};

// Registers FUZZ_TEST as GoogleTest TEST-s.
void RegisterFuzzTestsAsGoogleTests(int* argc, char*** argv,
                                    const Configuration& configuration);

// Set listing mode validator for GoogleTest to check that fuzz test listing was
// properly handled.
void SetFuzzTestListingModeValidatorForGoogleTest(bool listing_mode);

// Returns the list of registered tests.
std::vector<const testing::TestInfo*> GetRegisteredTests();

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_GOOGLETEST_ADAPTOR_H_
