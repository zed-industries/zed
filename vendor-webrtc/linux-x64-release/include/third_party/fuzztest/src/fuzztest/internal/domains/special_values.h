// Copyright 2024 Google LLC
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

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_SPECIAL_VALUES_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_SPECIAL_VALUES_H_

#include <array>
#include <cmath>
#include <limits>
#include <type_traits>

namespace fuzztest::internal {

template <typename T, typename = void>
class SpecialValues {
 public:
  static constexpr std::array<T, 0> Get() { return {}; }
};

template <typename T>
class SpecialValues<T, std::enable_if_t<std::numeric_limits<T>::is_integer>> {
 public:
  static constexpr auto Get() {
    // NB: std::to_array() is not available until C++20.
    return std::array{
        T{0}, T{1},
        // For some types, ~T{} is promoted to int. Convert back to T.
        static_cast<T>(~T{}),
        std::numeric_limits<T>::is_signed
            ? std::numeric_limits<T>::max()
            : static_cast<T>(std::numeric_limits<T>::max() >> 1)};
  }
};

template <typename T>
class SpecialValues<T, std::enable_if_t<std::is_floating_point_v<T>>> {
 public:
  static auto Get() {
    return std::array{
        T{0.0}, T{-0.0}, T{1.0}, T{-1.0}, std::numeric_limits<T>::max(),
        std::numeric_limits<T>::infinity(), -std::numeric_limits<T>::infinity(),
        // std::nan is double. Cast to T explicitly.
        static_cast<T>(std::nan(""))};
  }
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_SPECIAL_VALUES_H_
