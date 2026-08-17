// Copyright 2021 gRPC authors.
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

#ifndef GRPC_SRC_CORE_UTIL_MATCHERS_H
#define GRPC_SRC_CORE_UTIL_MATCHERS_H

#include <grpc/support/port_platform.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <string>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "re2/re2.h"

namespace grpc_core {

class StringMatcher {
 public:
  enum class Type {
    kExact,      // value stored in string_matcher_ field
    kPrefix,     // value stored in string_matcher_ field
    kSuffix,     // value stored in string_matcher_ field
    kSafeRegex,  // pattern stored in regex_matcher_ field
    kContains,   // value stored in string_matcher_ field
  };

  // Creates StringMatcher instance. Returns error status on failure.
  // Note: case_sensitive is ignored for type kSafeRegex.
  static absl::StatusOr<StringMatcher> Create(Type type,
                                              absl::string_view matcher,
                                              bool case_sensitive = true);

  StringMatcher() = default;
  StringMatcher(const StringMatcher& other);
  StringMatcher& operator=(const StringMatcher& other);
  StringMatcher(StringMatcher&& other) noexcept;
  StringMatcher& operator=(StringMatcher&& other) noexcept;
  bool operator==(const StringMatcher& other) const;

  bool Match(absl::string_view value) const;

  std::string ToString() const;

  Type type() const { return type_; }

  // Valid for kExact, kPrefix, kSuffix and kContains.
  const std::string& string_matcher() const { return string_matcher_; }

  // Valid for kSafeRegex.
  RE2* regex_matcher() const { return regex_matcher_.get(); }

  bool case_sensitive() const { return case_sensitive_; }

 private:
  StringMatcher(Type type, absl::string_view matcher, bool case_sensitive);
  explicit StringMatcher(std::unique_ptr<RE2> regex_matcher);

  Type type_ = Type::kExact;
  std::string string_matcher_;
  std::unique_ptr<RE2> regex_matcher_;
  bool case_sensitive_ = true;
};

class HeaderMatcher {
 public:
  enum class Type {
    kExact,      // value stored in StringMatcher field
    kPrefix,     // value stored in StringMatcher field
    kSuffix,     // value stored in StringMatcher field
    kSafeRegex,  // value stored in StringMatcher field
    kContains,   // value stored in StringMatcher field
    kRange,      // uses range_start and range_end fields
    kPresent,    // uses present_match field
  };

  // Make sure that the first five HeaderMatcher::Type enum values match up to
  // the corresponding StringMatcher::Type enum values, so that it's safe to
  // convert by casting when delegating to StringMatcher.
  static_assert(static_cast<StringMatcher::Type>(Type::kExact) ==
                StringMatcher::Type::kExact);
  static_assert(static_cast<StringMatcher::Type>(Type::kPrefix) ==
                StringMatcher::Type::kPrefix);
  static_assert(static_cast<StringMatcher::Type>(Type::kSuffix) ==
                StringMatcher::Type::kSuffix);
  static_assert(static_cast<StringMatcher::Type>(Type::kSafeRegex) ==
                StringMatcher::Type::kSafeRegex);
  static_assert(static_cast<StringMatcher::Type>(Type::kContains) ==
                StringMatcher::Type::kContains);

  // Creates HeaderMatcher instance. Returns error status on failure.
  static absl::StatusOr<HeaderMatcher> Create(absl::string_view name, Type type,
                                              absl::string_view matcher,
                                              int64_t range_start = 0,
                                              int64_t range_end = 0,
                                              bool present_match = false,
                                              bool invert_match = false,
                                              bool case_sensitive = true);

  // Creates a HeaderMatcher from an existing StringMatcher instance.
  static HeaderMatcher CreateFromStringMatcher(absl::string_view name,
                                               StringMatcher matcher,
                                               bool invert_match);

  HeaderMatcher() = default;
  HeaderMatcher(const HeaderMatcher& other);
  HeaderMatcher& operator=(const HeaderMatcher& other);
  HeaderMatcher(HeaderMatcher&& other) noexcept;
  HeaderMatcher& operator=(HeaderMatcher&& other) noexcept;
  bool operator==(const HeaderMatcher& other) const;

  const std::string& name() const { return name_; }

  Type type() const { return type_; }

  // Valid for kExact, kPrefix, kSuffix and kContains.
  const std::string& string_matcher() const {
    return matcher_.string_matcher();
  }

  // Valid for kSafeRegex.
  RE2* regex_matcher() const { return matcher_.regex_matcher(); }

  bool Match(const std::optional<absl::string_view>& value) const;

  std::string ToString() const;

 private:
  // For StringMatcher.
  HeaderMatcher(absl::string_view name, Type type, StringMatcher matcher,
                bool invert_match);
  // For RangeMatcher.
  HeaderMatcher(absl::string_view name, int64_t range_start, int64_t range_end,
                bool invert_match);
  // For PresentMatcher.
  HeaderMatcher(absl::string_view name, bool present_match, bool invert_match);

  std::string name_;
  Type type_ = Type::kExact;
  StringMatcher matcher_;
  int64_t range_start_;
  int64_t range_end_;
  bool present_match_;
  bool invert_match_ = false;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_UTIL_MATCHERS_H
