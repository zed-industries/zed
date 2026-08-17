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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_OPERATORS_COUNTER_MIPMAP_OPERATOR_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_OPERATORS_COUNTER_MIPMAP_OPERATOR_H_

#include <sqlite3.h>
#include <cstdint>
#include <vector>

#include "src/trace_processor/containers/implicit_segment_forest.h"
#include "src/trace_processor/perfetto_sql/engine/perfetto_sql_engine.h"
#include "src/trace_processor/sqlite/bindings/sqlite_module.h"
#include "src/trace_processor/sqlite/module_state_manager.h"

namespace perfetto::trace_processor {

// Operator for building "mipmaps" [1] over the counter-like tracks.
//
// In the context of trace data, mipmap really means aggregating the counter
// values in a given time period into the {min, max, last} value for that
// period, allowing UIs to efficiently display the contents of a counter track
// when very zoomed out.
//
// Specifically, we are computing the query:
// ```
//   select
//     last_value(ts),
//     min(value),
//     max(value),
//     last_value(value)
//   from $input in
//   where in.ts_end >= $window_start and in.ts <= $window_end
//   group by ts / $window_resolution
//   order by ts
// ```
// but in O(logn) time by using a segment-tree like data structure (see
// ImplicitSegmentForest).
//
// [1] https://en.wikipedia.org/wiki/Mipmap
struct CounterMipmapOperator : sqlite::Module<CounterMipmapOperator> {
  struct Counter {
    double min;
    double max;
  };
  struct Agg {
    Counter operator()(const Counter& a, const Counter& b) {
      Counter res;
      res.min = b.min < a.min ? b.min : a.min;
      res.max = b.max > a.max ? b.max : a.max;
      return res;
    }
  };
  struct State {
    ImplicitSegmentForest<Counter, Agg> forest;
    std::vector<int64_t> timestamps;
  };
  struct Context : sqlite::ModuleStateManager<CounterMipmapOperator> {
    explicit Context(PerfettoSqlEngine* _engine) : engine(_engine) {}
    PerfettoSqlEngine* engine;
  };
  struct Vtab : sqlite::Module<CounterMipmapOperator>::Vtab {
    sqlite::ModuleStateManager<CounterMipmapOperator>::PerVtabState* state;
  };
  struct Cursor : sqlite::Module<CounterMipmapOperator>::Cursor {
    struct Result {
      Counter min_max_counter;
      Counter last_counter;
      int64_t last_ts;
    };
    std::vector<Result> counters;
    uint32_t index;
  };

  static constexpr auto kType = kCreateOnly;
  static constexpr bool kSupportsWrites = false;
  static constexpr bool kDoesOverloadFunctions = false;

  static int Create(sqlite3*,
                    void*,
                    int,
                    const char* const*,
                    sqlite3_vtab**,
                    char**);
  static int Destroy(sqlite3_vtab*);

  static int Connect(sqlite3*,
                     void*,
                     int,
                     const char* const*,
                     sqlite3_vtab**,
                     char**);
  static int Disconnect(sqlite3_vtab*);

  static int BestIndex(sqlite3_vtab*, sqlite3_index_info*);

  static int Open(sqlite3_vtab*, sqlite3_vtab_cursor**);
  static int Close(sqlite3_vtab_cursor*);

  static int Filter(sqlite3_vtab_cursor*,
                    int,
                    const char*,
                    int,
                    sqlite3_value**);
  static int Next(sqlite3_vtab_cursor*);
  static int Eof(sqlite3_vtab_cursor*);
  static int Column(sqlite3_vtab_cursor*, sqlite3_context*, int);
  static int Rowid(sqlite3_vtab_cursor*, sqlite_int64*);

  // This needs to happen at the end as it depends on the functions
  // defined above.
  static constexpr sqlite3_module kModule = CreateModule();
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_OPERATORS_COUNTER_MIPMAP_OPERATOR_H_
