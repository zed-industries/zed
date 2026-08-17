/*
 * Copyright (C) 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_PROTOZERO_FILTERING_STRING_FILTER_H_
#define SRC_PROTOZERO_FILTERING_STRING_FILTER_H_

#include <regex>
#include <string>
#include <string_view>

namespace protozero {

// Performs filtering of strings in an "iptables" style. See the comments in
// |TraceConfig.TraceFilter| for information on how this class works.
class StringFilter {
 public:
  enum class Policy {
    kMatchRedactGroups = 1,
    kAtraceMatchRedactGroups = 2,
    kMatchBreak = 3,
    kAtraceMatchBreak = 4,
    kAtraceRepeatedSearchRedactGroups = 5,
  };

  // Adds a new rule for filtering strings.
  void AddRule(Policy policy,
               std::string_view pattern,
               std::string atrace_payload_starts_with);

  // Tries to filter the given string. Returns true if the string was modified
  // in any way, false otherwise.
  bool MaybeFilter(char* ptr, size_t len) const {
    if (len == 0 || rules_.empty()) {
      return false;
    }
    return MaybeFilterInternal(ptr, len);
  }

 private:
  struct Rule {
    Policy policy;
    std::regex pattern;
    std::string atrace_payload_starts_with;
  };

  bool MaybeFilterInternal(char* ptr, size_t len) const;

  std::vector<Rule> rules_;
};

}  // namespace protozero

#endif  // SRC_PROTOZERO_FILTERING_STRING_FILTER_H_
