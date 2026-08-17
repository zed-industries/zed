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

#ifndef THIRD_PARTY_CENTIPEDE_GOOGLE_CONFIG_INIT_H_
#define THIRD_PARTY_CENTIPEDE_GOOGLE_CONFIG_INIT_H_

#include <memory>
#include <string>
#include <vector>

namespace fuzztest::internal {

// The runtime state returned by `InitRuntime()`. The caller should take over
// the ownership of this and keep it alive for the duration of the process.
class [[nodiscard]] RuntimeState {
 public:
  explicit RuntimeState(std::vector<std::string> leftover_argv);
  virtual ~RuntimeState() = default;

  // Not copyable nor movable for simplicity and maximum extensibility.
  RuntimeState(const RuntimeState&) = delete;
  RuntimeState& operator=(const RuntimeState&) = delete;
  RuntimeState(RuntimeState&&) = delete;
  RuntimeState& operator=(RuntimeState&&) = delete;

  auto leftover_argv() const { return leftover_argv_; }
  auto& leftover_argv() { return leftover_argv_; }

 private:
  std::vector<std::string> leftover_argv_;
};

// * Initializes the relevant runtime subsystems in the correct order.
// * Directs all `LOG(INFO)`s to also to stderr (by default, only `LOG(ERROR)`s
//   and higher go to stderr).
// * Tweaks --help behavior to print any flags defined by any Centipede source
//   (by default, --help only prints flags defined in the source named
//   <program>.cc or <program_main>.cc).
// * Returns the runtime state, which the client should keep alive for the
//   duration of the process.
[[nodiscard]] std::unique_ptr<RuntimeState> InitRuntime(int argc, char* argv[]);

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_GOOGLE_CONFIG_INIT_H_
