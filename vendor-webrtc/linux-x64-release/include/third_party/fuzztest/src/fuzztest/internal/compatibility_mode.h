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

// Experimental compatibility mode with external fuzzing engines implementing
// the LLVMFuzzerRunDriver interface. See:
// https://llvm.org/docs/LibFuzzer.html#using-libfuzzer-as-a-library
//
// This is only for benchmarking purposes of evaluating fuzzing effectiveness.
//
// Do NOT use in production.
#ifndef FUZZTEST_FUZZTEST_INTERNAL_RUNTIME_COMPATIBILITY_H_
#define FUZZTEST_FUZZTEST_INTERNAL_RUNTIME_COMPATIBILITY_H_

#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <cstring>
#include <memory>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/random/distributions.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "absl/time/clock.h"
#include "./fuzztest/internal/fixture_driver.h"
#include "./fuzztest/internal/logging.h"
#include "./fuzztest/internal/runtime.h"

namespace fuzztest::internal {

#ifndef FUZZTEST_COMPATIBILITY_MODE

class FuzzTestExternalEngineAdaptor {};

#else

// Callback functions for the custom fuzzing logic when using external fuzzing
// engine.
class ExternalEngineCallback {
 public:
  virtual ~ExternalEngineCallback() = default;
  virtual void RunOneInputData(absl::string_view data) = 0;
  virtual std::string MutateData(absl::string_view data, size_t max_size,
                                 unsigned int seed) = 0;
};

// Sets and gets the global instance of libFuzzer callback object.
void SetExternalEngineCallback(ExternalEngineCallback* callback);
ExternalEngineCallback* GetExternalEngineCallback();

// Library API exposed from LibFuzzer.
extern "C" int LLVMFuzzerRunDriver(int* argc, char*** argv,
                                   int (*user_callback)(const uint8_t* data,
                                                        size_t size));

class FuzzTestExternalEngineAdaptor : public FuzzTestFuzzer,
                                      public ExternalEngineCallback {
 public:
  using Driver = UntypedFixtureDriver;

  FuzzTestExternalEngineAdaptor(const FuzzTest& test,
                                std::unique_ptr<Driver> fixture_driver);
  bool RunInUnitTestMode(const Configuration& configuration) override;
  bool RunInFuzzingMode(int* argc, char*** argv,
                        const Configuration& configuration) override;

  // External engine callbacks.

  void RunOneInputData(absl::string_view data) override;
  std::string MutateData(absl::string_view data, size_t max_size,
                         unsigned int seed) override;

 private:
  using FuzzerImpl = FuzzTestFuzzerImpl;

  FuzzerImpl& GetFuzzerImpl();

  const FuzzTest& test_;
  // Stores the fixture driver before the fuzzer gets instantiated. Once
  // `fuzzer_impl_` is no longer nullptr, `fixture_driver_staging_` becomes
  // nullptr.
  std::unique_ptr<Driver> fixture_driver_staging_;
  std::unique_ptr<FuzzerImpl> fuzzer_impl_;

  Runtime& runtime_ = Runtime::instance();
};

#endif  // FUZZTEST_COMPATIBILITY_MODE

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_RUNTIME_COMPATIBILITY_H_
