// Copyright 2023 Google LLC
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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_CENTIPEDE_ADAPTOR_H_
#define FUZZTEST_FUZZTEST_INTERNAL_CENTIPEDE_ADAPTOR_H_

#include <memory>

#include "./fuzztest/internal/configuration.h"
#include "./fuzztest/internal/fixture_driver.h"
#include "./fuzztest/internal/runtime.h"

namespace fuzztest::internal {

class CentipedeFixtureDriver;

// Adaptor for running FuzzTest fuzzers with the Centipede engine.
class CentipedeFuzzerAdaptor : public FuzzTestFuzzer {
 public:
  CentipedeFuzzerAdaptor(const FuzzTest& test,
                         std::unique_ptr<UntypedFixtureDriver> fixture_driver);
  bool RunInUnitTestMode(const Configuration& configuration) override;
  bool RunInFuzzingMode(int* argc, char*** argv,
                        const Configuration& configuration) override;

 private:
  bool Run(int* argc, char*** argv, RunMode mode,
           const Configuration& configuration);
  bool ReplayCrashInSingleProcess(const Configuration& configuration);

  Runtime& runtime_ = Runtime::instance();
  const FuzzTest& test_;
  CentipedeFixtureDriver* centipede_fixture_driver_;
  FuzzTestFuzzerImpl fuzzer_impl_;
};

// Lists crash IDs for `test_name` with `configuration` using Centipede.
std::vector<std::string> ListCrashIdsUsingCentipede(
    const Configuration& configuration, absl::string_view test_name);

// Returns whether the current process is a Centipede runner process.
bool IsCentipedeRunner();

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_CENTIPEDE_ADAPTOR_H_
