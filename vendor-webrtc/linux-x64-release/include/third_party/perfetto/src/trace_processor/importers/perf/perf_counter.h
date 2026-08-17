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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_PERF_COUNTER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_PERF_COUNTER_H_

#include <cstdint>

#include "src/trace_processor/tables/counter_tables_py.h"
#include "src/trace_processor/tables/track_tables_py.h"

namespace perfetto::trace_processor::perf_importer {

// Helper class to keep track of perf counters and convert delta values found in
// perf files to absolute values needed for the perfetto counter table.
class PerfCounter {
 public:
  PerfCounter(tables::CounterTable* counter_table,
              tables::TrackTable::Id track_id,
              bool is_timebase)
      : counter_table_(*counter_table),
        track_id_(track_id),
        is_timebase_(is_timebase) {}

  bool is_timebase() const { return is_timebase_; }

  void AddDelta(int64_t ts, double delta);
  void AddCount(int64_t ts, double count);

 private:
  tables::CounterTable& counter_table_;
  tables::TrackTable::Id track_id_;
  const bool is_timebase_;
  double last_count_{0};
};

}  // namespace perfetto::trace_processor::perf_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_PERF_COUNTER_H_
