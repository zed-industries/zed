// Copyright 2023 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_CENTIPEDE_BATCH_FUZZ_EXAMPLE_CUSTOMIZED_CENTIPEDE_H_
#define THIRD_PARTY_CENTIPEDE_BATCH_FUZZ_EXAMPLE_CUSTOMIZED_CENTIPEDE_H_

#include <sys/types.h>

#include <vector>

#include "absl/strings/string_view.h"
#include "./centipede/centipede_callbacks.h"
#include "./centipede/environment.h"
#include "./centipede/runner_result.h"
#include "./common/defs.h"

namespace fuzztest::internal {

// This class implements the `Execute()` method of the `CentipedeCallbacks`
// class. It saves a collection of inputs into files and passes them to a target
// binary. The binary should exercise them in a batch and store the execution
// result of each input into an output file. Those execution results will be
// loaded from the output file and packed as the given `batch_result`.
class CustomizedCallbacks : public CentipedeCallbacks {
 public:
  explicit CustomizedCallbacks(const Environment& env)
      : CustomizedCallbacks(env, /*feature_only_feedback=*/false) {}

  explicit CustomizedCallbacks(const Environment& env,
                               bool feature_only_feedback);

  bool Execute(std::string_view binary, const std::vector<ByteArray>& inputs,
               BatchResult& batch_result) override;

 private:
  const bool feature_only_feedback_;
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_BATCH_FUZZ_EXAMPLE_CUSTOMIZED_CENTIPEDE_H_
