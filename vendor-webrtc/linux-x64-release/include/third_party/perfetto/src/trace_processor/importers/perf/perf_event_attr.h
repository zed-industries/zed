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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_PERF_EVENT_ATTR_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_PERF_EVENT_ATTR_H_

#include <sys/types.h>
#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>
#include <unordered_map>
#include <utility>

#include "perfetto/trace_processor/ref_counted.h"
#include "src/trace_processor/importers/common/clock_tracker.h"
#include "src/trace_processor/importers/perf/perf_counter.h"
#include "src/trace_processor/importers/perf/perf_event.h"
#include "src/trace_processor/tables/profiler_tables_py.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

namespace perf_importer {

// Wrapper around a `perf_event_attr` object that add some helper methods.
class PerfEventAttr : public RefCounted {
 public:
  PerfEventAttr(TraceProcessorContext* context,
                tables::PerfSessionTable::Id perf_session_id_,
                perf_event_attr attr);
  ~PerfEventAttr();
  uint32_t type() const { return attr_.type; }
  uint64_t config() const { return attr_.config; }
  uint64_t sample_type() const { return attr_.sample_type; }
  uint64_t read_format() const { return attr_.read_format; }
  bool sample_id_all() const { return !!attr_.sample_id_all; }

  // Returns period if set.
  std::optional<uint64_t> sample_period() const {
    // attr_.freq tells whether attr_.sample_period or attr_.sample_freq is set.
    return attr_.freq ? std::nullopt : std::make_optional(attr_.sample_period);
  }

  // Returns frequency if set.
  std::optional<uint64_t> sample_freq() const {
    // attr_.freq tells whether attr_.sample_period or attr_.sample_freq is set.
    return attr_.freq ? std::make_optional(attr_.sample_freq) : std::nullopt;
  }

  // Offset from the end of a record's payload to the time filed (if present).
  // To be used with non `PERF_RECORD_SAMPLE` records
  std::optional<size_t> time_offset_from_end() const {
    return time_offset_from_end_;
  }

  // Offset from the start of a record's payload to the time filed (if present).
  // To be used with `PERF_RECORD SAMPLE` records
  std::optional<size_t> time_offset_from_start() const {
    return time_offset_from_start_;
  }

  // Offsets from start and end of record payload to the id field. These offsets
  // are used to determine the event_id and thus the perf_event_attr value of a
  // record. During tokenization we need to determine the `sample_type` to be
  // able to later parse the record. The `sample_type` is stored in the
  // `perf_event_attr` structure.

  // To be used with PERF_SAMPLE_RECORD records
  std::optional<size_t> id_offset_from_start() const {
    return id_offset_from_start_;
  }
  // To be used with non PERF_SAMPLE_RECORD records if `sample_id_all` is set.
  std::optional<size_t> id_offset_from_end() const {
    return id_offset_from_end_;
  }

  void set_event_name(std::string event_name) {
    event_name_ = std::move(event_name);
  }

  size_t sample_id_size() const { return sample_id_size_; }

  PerfCounter& GetOrCreateCounter(uint32_t cpu);

  ClockTracker::ClockId clock_id() const { return clock_id_; }

 private:
  bool is_timebase() const {
    // This is what simpleperf uses for events that are not supposed to sample
    // TODO(b/334978369): Determine if there is a better way to figure this out.
    return attr_.sample_period < (1ull << 62);
  }

  PerfCounter CreateCounter(uint32_t cpu) const;

  TraceProcessorContext* const context_;
  const ClockTracker::ClockId clock_id_;
  tables::PerfSessionTable::Id perf_session_id_;
  perf_event_attr attr_;
  std::optional<size_t> time_offset_from_start_;
  std::optional<size_t> time_offset_from_end_;
  std::optional<size_t> id_offset_from_start_;
  std::optional<size_t> id_offset_from_end_;
  size_t sample_id_size_;
  std::unordered_map<uint32_t, PerfCounter> counters_;
  std::string event_name_;
};

}  // namespace perf_importer
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_PERF_EVENT_ATTR_H_
