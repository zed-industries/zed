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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_STRUCT_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_STRUCT_H_

#include <array>
#include <cstdint>
#include <string>
#include <utility>

#include "src/trace_processor/perfetto_sql/intrinsics/types/value.h"

namespace perfetto::trace_processor::perfetto_sql {

struct Struct {
  static constexpr uint32_t kMaxFields = 8;
  std::array<std::pair<std::string, Value>, kMaxFields> fields;
  uint32_t field_count = 0;
};

}  // namespace perfetto::trace_processor::perfetto_sql

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_STRUCT_H_
