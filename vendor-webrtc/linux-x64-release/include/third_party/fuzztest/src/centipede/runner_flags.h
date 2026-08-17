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

#ifndef THIRD_PARTY_CENTIPEDE_RUNNER_FLAGS_H_
#define THIRD_PARTY_CENTIPEDE_RUNNER_FLAGS_H_

#include <cstddef>
#include <string>
#include <utility>  // std::pair
#include <vector>

namespace fuzztest::internal {

class RunnerFlags {
 public:
  // Takes a strings of colon-separated flags. Flags in the string are either
  // 1. <flag name> or
  // 2. <flag name>=<flag value>
  explicit RunnerFlags(const std::string& runner_flags_string);

  ~RunnerFlags() = default;
  // By default copyable and movable.
  RunnerFlags(const RunnerFlags&) = default;
  RunnerFlags& operator=(const RunnerFlags&) = default;
  RunnerFlags(RunnerFlags&&) = default;
  RunnerFlags& operator=(RunnerFlags&&) = default;

  // Tells if 'flag' is in this.
  bool HasFlag(const std::string& flag) const {
    return IndexOfFlag(flag) != flags_.size();
  }

  // Gets value of 'flag'. Returns an empty string if 'flag' is not in this
  // or 'flag' has no value. If a flag is repeated, this returns
  // value of the last occurrence.
  std::string GetFlagValue(const std::string& flag) const {
    const size_t index = IndexOfFlag(flag);
    return index == flags_.size() ? "" : flags_[index].second;
  }

  // Set 'flag' to 'value'.  If 'value' is empty, 'flag' is considered
  // present but without a value. If a flag is not in this, it is appended
  // at the end of flags. If a flag is repeated, only the occurrence is updated.
  void SetFlagValue(const std::string& flag, const std::string& value) {
    const size_t index = IndexOfFlag(flag);
    if (index < flags_.size()) {
      flags_[index].second = value;
    } else {
      flags_.push_back(std::make_pair(flag, value));
    }
  }

  // Returns a strings with all flags separated by colons in lexicographical
  // order. The result also begins and ends with colons.
  std::string ToString() const;

 private:
  // Returns index of 'flag' in flags_ or flags_.size() if not found.
  size_t IndexOfFlag(const std::string& flag) const {
    const size_t flags_size = flags_.size();
    size_t i = 0;
    // Search backward so that we pick the last occurrence of a repeated flag.
    while (i < flags_size) {
      size_t pos = flags_size - 1 - i;
      if (flags_[pos].first == flag) {
        return pos;
      }
      ++i;
    }
    return flags_size;
  }

  // Individual flags with their optional values as a vector of string pairs.
  // To make this suitable for the runner, we use std::vector<>, which is not
  // very efficient to look up. This should not be a problem as the number of
  // flags are quite small.
  std::vector<std::pair<std::string, std::string>> flags_;
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_RUNNER_FLAGS_H_
