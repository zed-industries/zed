/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_VALUE_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_VALUE_H_

#include <cstdint>
#include <string>
#include <variant>

namespace perfetto::trace_processor::perfetto_sql {

using Value = std::variant<std::monostate, int64_t, double, std::string>;

template <typename T>
inline constexpr uint32_t ValueIndex() {
  if constexpr (std::is_same_v<T, std::monostate>) {
    return 0;
  } else if constexpr (std::is_same_v<T, int64_t>) {
    return 1;
  } else if constexpr (std::is_same_v<T, double>) {
    return 2;
  } else if constexpr (std::is_same_v<T, std::string>) {
    return 3;
  } else {
    static_assert(!sizeof(T*), "T is not supported");
  }
}

}  // namespace perfetto::trace_processor::perfetto_sql

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_VALUE_H_
