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

#ifndef FUZZTEST_COMMON_DEFS_H_
#define FUZZTEST_COMMON_DEFS_H_

// Only simple definitions here. No code, no dependencies.
// span.h is an exception as it's header-only and very simple.

#include <cstdint>
#include <random>
#include <string>
#include <string_view>
#include <vector>

#include "absl/types/span.h"

namespace fuzztest::internal {

// Just a good random number generator.
using Rng = std::mt19937_64;

using ByteArray = std::vector<uint8_t>;
using ByteSpan = absl::Span<const uint8_t>;

// Wraps a container's data into a `ByteSpan`. The lifetime of `blob` should be
// >= that of the returned object.
template <typename Container>
ByteSpan AsByteSpan(const Container &blob) {
  return ByteSpan(reinterpret_cast<const uint8_t *>(blob.data()),
                  blob.size() * sizeof(typename Container::value_type));
}

// Reinterprets a `ByteSpan` as a string_view pointing at the same data. The
// lifetime of `str` should be >= that of the returned object.
inline std::string_view AsStringView(ByteSpan str) {
  return std::string_view(reinterpret_cast<const char *>(str.data()),
                          str.size());
}

inline std::string AsString(ByteSpan str) {
  return std::string(AsStringView(str));
}

// Macro used to allow tests to access protected or private members of a class.
#define FRIEND_TEST(test_case_name, test_name) \
  friend class test_case_name##_##test_name##_Test

}  // namespace fuzztest::internal

#endif  // FUZZTEST_COMMON_DEFS_H_
