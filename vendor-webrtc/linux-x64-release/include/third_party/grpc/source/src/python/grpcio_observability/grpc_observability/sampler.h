// Copyright 2023 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef SAMPLER_MAIN_H
#define SAMPLER_MAIN_H

#include <cstdint>
#include <string>

namespace grpc_observability {

// Returns true or false for sampling based on the given probability. Objects of
// this class should be cached between uses because there is a cost to
// constructing them.
class ProbabilitySampler final {
 public:
  static ProbabilitySampler& Get();

  bool ShouldSample(const std::string& trace_id);

  void SetThreshold(double probability);

 private:
  ProbabilitySampler() = default;

  // Probability is converted to a value between [0, UINT64_MAX].
  uint64_t threshold_;
};

}  // namespace grpc_observability

#endif  // SAMPLER_MAIN_H
