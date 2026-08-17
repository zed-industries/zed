/*
 * Copyright (C) 2025 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_CURSOR_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_CURSOR_H_

#include <cstddef>
#include <cstdint>
#include <limits>
#include <utility>

#include "perfetto/base/logging.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/containers/null_term_string_view.h"
#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/dataframe/impl/bytecode_interpreter.h"
#include "src/trace_processor/dataframe/impl/query_plan.h"
#include "src/trace_processor/dataframe/impl/types.h"
#include "src/trace_processor/dataframe/specs.h"
#include "src/trace_processor/dataframe/value_fetcher.h"

namespace perfetto::trace_processor::dataframe {

// Callback for receiving cell values
struct CellCallback {
  void OnCell(int64_t);
  void OnCell(double);
  void OnCell(NullTermStringView);
  void OnCell(std::nullptr_t);
  void OnCell(uint32_t);
  void OnCell(int32_t);
};

// Cursor provides a mechanism to iterate through dataframe query results
// and access column values.
template <typename FilterValueFetcherImpl>
class Cursor {
 public:
  static_assert(std::is_base_of_v<ValueFetcher, FilterValueFetcherImpl>,
                "FilterValueFetcherImpl must be a subclass of ValueFetcher");

  // Constructs a cursor from a query plan and dataframe columns.
  Cursor(impl::QueryPlan plan,
         const impl::Column* columns,
         const StringPool* pool)
      : interpreter_(std::move(plan.bytecode), columns, pool),
        params_(plan.params),
        columns_(columns),
        pool_(pool) {}

  // Executes the query and prepares the cursor for iteration.
  // This initializes the cursor's position to the first row of results.
  //
  // Parameters:
  //   fvf: A subclass of `ValueFetcher` that defines the logic for fetching
  //        filter values for each filter spec.
  PERFETTO_ALWAYS_INLINE void Execute(
      FilterValueFetcherImpl& filter_value_fetcher) {
    using S = impl::Span<uint32_t>;
    interpreter_.Execute(filter_value_fetcher);

    const auto& span =
        *interpreter_.template GetRegisterValue<S>(params_.output_register);
    pos_ = span.b;
    end_ = span.e;
  }

  // Advances the cursor to the next row of results.
  PERFETTO_ALWAYS_INLINE void Next() {
    PERFETTO_DCHECK(pos_ < end_);
    pos_ += params_.output_per_row;
  }

  // Returns true if the cursor has reached the end of the result set.
  PERFETTO_ALWAYS_INLINE bool Eof() const { return pos_ == end_; }

  // Returns the value of the column at the current cursor position.
  // The visitor pattern allows type-safe access to heterogeneous column types.
  //
  // Parameters:
  //   col:    The index of the column to access.
  //   callback: A subclass of `CellCallback` that defines the logic for
  //             processing the value of the column at the current cursor
  //             position.
  template <typename CellCallbackImpl>
  PERFETTO_ALWAYS_INLINE void Cell(uint32_t col,
                                   CellCallbackImpl& cell_callback_impl) {
    static_assert(std::is_base_of_v<CellCallback, CellCallbackImpl>,
                  "CellCallbackImpl must be a subclass of CellCallback");
    const impl::Column& c = columns_[col];
    uint32_t idx = pos_[params_.col_to_output_offset[col]];
    if (idx == std::numeric_limits<uint32_t>::max()) {
      cell_callback_impl.OnCell(nullptr);
      return;
    }
    switch (c.storage.type().index()) {
      case StorageType::GetTypeIndex<Id>():
        cell_callback_impl.OnCell(idx);
        break;
      case StorageType::GetTypeIndex<Uint32>():
        cell_callback_impl.OnCell(c.storage.unchecked_data<Uint32>()[idx]);
        break;
      case StorageType::GetTypeIndex<Int32>():
        cell_callback_impl.OnCell(c.storage.unchecked_data<Int32>()[idx]);
        break;
      case StorageType::GetTypeIndex<Int64>():
        cell_callback_impl.OnCell(c.storage.unchecked_data<Int64>()[idx]);
        break;
      case StorageType::GetTypeIndex<Double>():
        cell_callback_impl.OnCell(c.storage.unchecked_data<Double>()[idx]);
        break;
      case StorageType::GetTypeIndex<String>():
        cell_callback_impl.OnCell(
            pool_->Get(c.storage.unchecked_data<String>()[idx]));
        break;
      default:
        PERFETTO_FATAL("Invalid storage spec");
    }
  }

 private:
  // Bytecode interpreter that executes the query.
  impl::bytecode::Interpreter<FilterValueFetcherImpl> interpreter_;
  // Parameters for query execution.
  impl::QueryPlan::ExecutionParams params_;
  // Pointer to the dataframe columns.
  const impl::Column* columns_;
  // String pool for string values.
  const StringPool* pool_;

  // Current position in the result set.
  const uint32_t* pos_;
  // End position in the result set.
  const uint32_t* end_;
};

}  // namespace perfetto::trace_processor::dataframe

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_CURSOR_H_
