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
#ifndef SRC_TRACE_PROCESSOR_DB_QUERY_EXECUTOR_H_
#define SRC_TRACE_PROCESSOR_DB_QUERY_EXECUTOR_H_

#include <cstdint>
#include <utility>
#include <vector>

#include "src/trace_processor/containers/row_map.h"
#include "src/trace_processor/db/column.h"
#include "src/trace_processor/db/column/data_layer.h"
#include "src/trace_processor/db/column/types.h"

namespace perfetto::trace_processor {

// Responsible for executing filtering/sorting operations on a single Table.
// TODO(b/283763282): Introduce sorting.
class QueryExecutor {
 public:
  static constexpr uint32_t kMaxOverlayCount = 8;

  // |row_count| is the size of the last overlay.
  QueryExecutor(const std::vector<column::DataLayerChain*>& columns,
                uint32_t row_count)
      : columns_(columns), row_count_(row_count) {}

  // Apply all the constraints on the data and return the filtered RowMap.
  RowMap Filter(const std::vector<Constraint>& cs) {
    RowMap rm(0, row_count_);
    for (const auto& c : cs) {
      ApplyConstraint(c, *columns_[c.col_idx], &rm);
    }
    return rm;
  }

  // Enables QueryExecutor::Sort on Table columns.
  static void SortLegacy(const Table*,
                         const std::vector<Order>&,
                         std::vector<uint32_t>&);

  // Used only in unittests. Exposes private function.
  static void BoundedColumnFilterForTesting(const Constraint&,
                                            const column::DataLayerChain&,
                                            RowMap*);

  // Used only in unittests. Exposes private function.
  static void IndexedColumnFilterForTesting(const Constraint&,
                                            const column::DataLayerChain&,
                                            RowMap*);

  // Updates RowMap with result of filtering single column using the Constraint.
  static void ApplyConstraint(const Constraint&,
                              const column::DataLayerChain&,
                              RowMap*);

 private:
  // Filters the column using Range algorithm - tries to find the smallest Range
  // to filter the storage with.
  static void LinearSearch(const Constraint&,
                           const column::DataLayerChain&,
                           RowMap*);

  // Filters the column using Index algorithm - finds the indices to filter the
  // storage with.
  static void IndexSearch(const Constraint&,
                          const column::DataLayerChain&,
                          RowMap*);

  std::vector<column::DataLayerChain*> columns_;

  // Number of rows in the outmost overlay.
  uint32_t row_count_ = 0;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_DB_QUERY_EXECUTOR_H_
