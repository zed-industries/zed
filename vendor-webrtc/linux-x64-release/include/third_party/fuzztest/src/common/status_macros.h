// Copyright 2024 The Centipede Authors.
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

// Convenience macros for dealing with absl::Status and friends.

#ifndef FUZZTEST_COMMON_STATUS_MACROS_H_
#define FUZZTEST_COMMON_STATUS_MACROS_H_

#include <cstdint>

#include "absl/base/attributes.h"
#include "absl/base/optimization.h"
#include "absl/log/log.h"

// If `status_expr` (an expression of type `absl::Status`) is not OK then return
// it from the current function. Otherwise, do nothing.
#define RETURN_IF_NOT_OK(status_expr)                \
  do {                                               \
    const absl::Status status_value = (status_expr); \
    if (ABSL_PREDICT_FALSE(!status_value.ok())) {    \
      return status_value;                           \
    }                                                \
  } while (false)

// Assigns `dest` to the value contained within the `absl::StatusOr<T> src` if
// `src.ok()`, otherwise, returns `src.status()` from the current function.
#define ASSIGN_OR_RETURN_IF_NOT_OK(dest, src) \
  ASSIGN_OR_RETURN_IF_NOT_OK_IMPL_(           \
      CHECKS_INTERNAL_CONCAT_(value_or_, __LINE__), dest, src)
#define ASSIGN_OR_RETURN_IF_NOT_OK_IMPL_(value_or, dest, src) \
  auto value_or = (src);                                      \
  if (ABSL_PREDICT_FALSE(!value_or.ok())) {                   \
    return std::move(value_or).status();                      \
  }                                                           \
  dest = std::move(value_or).value()

// Internal helper for concatenating macro values.
#define CHECKS_INTERNAL_CONCAT_IMPL_(x, y) x##y
#define CHECKS_INTERNAL_CONCAT_(x, y) CHECKS_INTERNAL_CONCAT_IMPL_(x, y)

namespace fuzztest::internal {
template <typename T>
decltype(auto) ValueOrDie(T&& value ABSL_ATTRIBUTE_LIFETIME_BOUND,
                          std::uint_least32_t line = __builtin_LINE(),
                          const char* file_name = __builtin_FILE()) {
  if (ABSL_PREDICT_FALSE(!value.ok())) {
    LOG(FATAL) << file_name << ":" << line
               << ": ValueOrDie on non-OK status: " << value.status();
  }
  return *std::forward<T>(value);
}
}  // namespace fuzztest::internal

#endif  // FUZZTEST_COMMON_STATUS_MACROS_H_
