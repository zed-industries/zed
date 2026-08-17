/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TABLE_FUNCTIONS_STATIC_TABLE_FUNCTION_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TABLE_FUNCTIONS_STATIC_TABLE_FUNCTION_H_

#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "perfetto/ext/base/status_or.h"
#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/db/table.h"

namespace perfetto::trace_processor {

// Interface which can be subclassed to allow generation of tables dynamically
// at filter time.
// This class is used to implement table-valued functions and other similar
// tables.
class StaticTableFunction {
 public:
  virtual ~StaticTableFunction();

  // Returns the schema of the table that will be returned by ComputeTable.
  virtual Table::Schema CreateSchema() = 0;

  // Returns the name of the dynamic table.
  // This will be used to register the table with SQLite.
  virtual std::string TableName() = 0;

  // Returns the estimated number of rows the table would generate.
  virtual uint32_t EstimateRowCount() = 0;

  // Dynamically computes the table given the provided arguments.
  virtual base::StatusOr<std::unique_ptr<Table>> ComputeTable(
      const std::vector<SqlValue>& arguments) = 0;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TABLE_FUNCTIONS_STATIC_TABLE_FUNCTION_H_
