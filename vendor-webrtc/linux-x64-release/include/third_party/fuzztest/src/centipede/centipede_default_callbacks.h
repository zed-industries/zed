// Copyright 2022 The Centipede Authors.
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

// Example fuzzer built on top of Centipede. It is capable of fuzzing any binary
// target built with sancov (see build_defs.bzl). CentipedeCallbacks::Mutate
// uses ByteArrayMutator.

#ifndef THIRD_PARTY_CENTIPEDE_CENTIPEDE_DEFAULT_CALLBACKS_H_
#define THIRD_PARTY_CENTIPEDE_CENTIPEDE_DEFAULT_CALLBACKS_H_

#include <cstddef>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include "absl/status/statusor.h"
#include "./centipede/centipede_callbacks.h"
#include "./centipede/environment.h"
#include "./centipede/mutation_input.h"
#include "./centipede/runner_result.h"
#include "./common/defs.h"

namespace fuzztest::internal {

// Example of customized CentipedeCallbacks.
class CentipedeDefaultCallbacks : public CentipedeCallbacks {
 public:
  explicit CentipedeDefaultCallbacks(const Environment &env);
  size_t GetSeeds(size_t num_seeds, std::vector<ByteArray> &seeds) override;
  absl::StatusOr<std::string> GetSerializedTargetConfig() override;
  bool Execute(std::string_view binary, const std::vector<ByteArray> &inputs,
               BatchResult &batch_result) override;
  std::vector<ByteArray> Mutate(const std::vector<MutationInputRef> &inputs,
                                size_t num_mutants) override;

 private:
  std::optional<bool> custom_mutator_is_usable_ = std::nullopt;
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_CENTIPEDE_DEFAULT_CALLBACKS_H_
