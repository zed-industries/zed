/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TABLE_FUNCTIONS_CONNECTED_FLOW_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TABLE_FUNCTIONS_CONNECTED_FLOW_H_

#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "perfetto/ext/base/status_or.h"
#include "perfetto/trace_processor/basic_types.h"
#include "src/trace_processor/db/table.h"
#include "src/trace_processor/perfetto_sql/intrinsics/table_functions/static_table_function.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

// Implementation of tables:
// - DIRECTLY_CONNECTED_FLOW
// - PRECEDING_FLOW
// - FOLLOWING_FLOW
class ConnectedFlow : public StaticTableFunction {
 public:
  enum class Mode {
    // Directly connected slices through the same flow ID given by the trace
    // writer.
    kDirectlyConnectedFlow,
    // Flow events which can be reached from the given slice by going over
    // incoming flow events or to parent slices.
    kPrecedingFlow,
    // Flow events which can be reached from the given slice by going over
    // outgoing flow events or to child slices.
    kFollowingFlow,
  };

  ConnectedFlow(Mode mode, const TraceStorage*);
  ~ConnectedFlow() override;

  Table::Schema CreateSchema() override;
  std::string TableName() override;
  uint32_t EstimateRowCount() override;
  base::StatusOr<std::unique_ptr<Table>> ComputeTable(
      const std::vector<SqlValue>& arguments) override;

 private:
  Mode mode_;
  const TraceStorage* storage_ = nullptr;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_TABLE_FUNCTIONS_CONNECTED_FLOW_H_
