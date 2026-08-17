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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_WINDOW_FUNCTIONS_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_WINDOW_FUNCTIONS_H_

#include <sqlite3.h>
#include <cstdint>
#include <type_traits>

#include "perfetto/base/logging.h"
#include "src/trace_processor/perfetto_sql/engine/perfetto_sql_engine.h"
#include "src/trace_processor/sqlite/bindings/sqlite_result.h"
#include "src/trace_processor/sqlite/bindings/sqlite_window_function.h"

namespace perfetto::trace_processor {

// Keeps track of the latest non null value and its position withing the
// window. Every time the window shrinks (`xInverse` is called) the window size
// is reduced by one and the position of the value moves one back, if it gets
// out of the window the value is discarded.
class LastNonNullAggregateContext {
 public:
  static LastNonNullAggregateContext* Get(sqlite3_context* ctx) {
    return reinterpret_cast<LastNonNullAggregateContext*>(
        sqlite3_aggregate_context(ctx, 0));
  }

  static LastNonNullAggregateContext* GetOrCreate(sqlite3_context* ctx) {
    return reinterpret_cast<LastNonNullAggregateContext*>(
        sqlite3_aggregate_context(ctx, sizeof(LastNonNullAggregateContext)));
  }

  inline void PopFront() {
    PERFETTO_CHECK(window_size_ > 0);
    --window_size_;
    if (!last_non_null_value_) {
      return;
    }
    if (value_index_ == 0) {
      sqlite3_value_free(last_non_null_value_);
      last_non_null_value_ = nullptr;
      return;
    }
    PERFETTO_CHECK(value_index_ > 0);
    --value_index_;
  }

  inline void PushBack(sqlite3_value* value) {
    ++window_size_;
    if (sqlite3_value_type(value) == SQLITE_NULL) {
      return;
    }

    Destroy();
    last_non_null_value_ = sqlite3_value_dup(value);
    value_index_ = window_size_ - 1;
  }

  inline void Destroy() {
    if (last_non_null_value_) {
      sqlite3_value_free(last_non_null_value_);
    }
  }

  sqlite3_value* last_non_null_value() const { return last_non_null_value_; }

 private:
  int64_t window_size_;
  // Index within the window of the last non null value. Only valid if `value`
  // is set.
  int64_t value_index_;
  // Actual value
  sqlite3_value* last_non_null_value_;
};

static_assert(std::is_standard_layout_v<LastNonNullAggregateContext>,
              "Must be able to be initialized by sqlite3_aggregate_context "
              "(similar to calloc, i.e. no constructor called)");
static_assert(std::is_trivial_v<LastNonNullAggregateContext>,
              "Must be able to be destroyed by just calling free (i.e. no "
              "destructor called)");

class LastNonNull : public SqliteWindowFunction {
 public:
  static void Step(sqlite3_context* ctx, int argc, sqlite3_value** argv) {
    if (argc != 1) {
      return sqlite::result::Error(
          ctx, "Unsupported number of args passed to LAST_NON_NULL");
    }

    auto* ptr = LastNonNullAggregateContext::GetOrCreate(ctx);
    if (!ptr) {
      return sqlite::result::Error(ctx,
                                   "LAST_NON_NULL: Failed to allocate context");
    }

    ptr->PushBack(argv[0]);
  }

  static void Inverse(sqlite3_context* ctx, int, sqlite3_value**) {
    auto* ptr = LastNonNullAggregateContext::GetOrCreate(ctx);
    PERFETTO_CHECK(ptr != nullptr);
    ptr->PopFront();
  }

  static void Value(sqlite3_context* ctx) {
    auto* ptr = LastNonNullAggregateContext::GetOrCreate(ctx);
    if (!ptr || !ptr->last_non_null_value()) {
      return sqlite::result::Null(ctx);
    }
    sqlite3_result_value(ctx, ptr->last_non_null_value());
  }

  static void Final(sqlite3_context* ctx) {
    auto* ptr = LastNonNullAggregateContext::Get(ctx);
    if (!ptr || !ptr->last_non_null_value()) {
      return sqlite::result::Null(ctx);
    }
    sqlite::result::Value(ctx, ptr->last_non_null_value());
    ptr->Destroy();
  }
};

inline base::Status RegisterLastNonNullFunction(PerfettoSqlEngine& engine) {
  return engine.RegisterSqliteWindowFunction<LastNonNull>("LAST_NON_NULL", 1,
                                                          nullptr);
}

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_WINDOW_FUNCTIONS_H_
