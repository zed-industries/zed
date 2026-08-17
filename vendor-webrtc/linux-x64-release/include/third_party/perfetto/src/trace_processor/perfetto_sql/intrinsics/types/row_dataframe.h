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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_ROW_DATAFRAME_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_ROW_DATAFRAME_H_

#include <algorithm>
#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "src/trace_processor/perfetto_sql/intrinsics/types/array.h"
#include "src/trace_processor/perfetto_sql/intrinsics/types/value.h"

namespace perfetto::trace_processor::perfetto_sql {

// Data structure to allow easy exchange of "table-like" data between SQL and
// C++ code. Allows fast lookup of rows by id (if an id column exists).
struct RowDataframe {
  perfetto_sql::StringArray column_names;
  std::vector<uint32_t> id_to_cell_index;
  // Cell = a value at a row + column index.
  std::vector<perfetto_sql::Value> cells;
  std::optional<uint32_t> id_column_index;

  const perfetto_sql::Value* RowForId(uint32_t id) const {
    return cells.data() + id_to_cell_index[id];
  }

  std::optional<uint32_t> FindColumnWithName(const std::string& name) {
    auto it = std::find(column_names.begin(), column_names.end(), name);
    return it == column_names.end() ? std::nullopt
                                    : std::make_optional(static_cast<uint32_t>(
                                          it - column_names.begin()));
  }

  uint32_t size() const {
    return static_cast<uint32_t>(cells.size() / column_names.size());
  }
};

}  // namespace perfetto::trace_processor::perfetto_sql

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TYPES_ROW_DATAFRAME_H_
