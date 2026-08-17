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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_CLOCK_CONVERTER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_CLOCK_CONVERTER_H_

#include <stdint.h>

#include <array>
#include <cinttypes>
#include <map>
#include <random>
#include <set>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/ext/base/string_utils.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

#include "protos/perfetto/common/builtin_clock.pbzero.h"
#include "protos/perfetto/trace/clock_snapshot.pbzero.h"

namespace perfetto {
namespace trace_processor {

// Used for conversion to REAL and MONO clocks for provided timestamps. Can only
// be used after trace parsing. Only works if there has been at least one
// snapshot with a target clock. Data is based on clock snapshots table.
class ClockConverter {
 public:
  using ClockId = int64_t;
  using Timestamp = int64_t;

  explicit ClockConverter(TraceProcessorContext*);

  // Converts trace time to REAL clock as string.
  base::StatusOr<std::string> ToAbsTime(Timestamp ts) {
    base::StatusOr<Timestamp> real_ts = FromTraceTime(kRealClock, ts);
    if (!real_ts.ok())
      return real_ts.status();
    return TimeToStr(*real_ts);
  }

  // Converts trace time to REAL clock time.
  base::StatusOr<Timestamp> ToRealtime(Timestamp ts) {
    return FromTraceTime(kRealClock, ts);
  }

  // Converts trace time to MONO clock time.
  base::StatusOr<Timestamp> ToMonotonic(Timestamp ts) {
    return FromTraceTime(kMonoClock, ts);
  }

 private:
  static constexpr int64_t kRealClock =
      protos::pbzero::ClockSnapshot::Clock::REALTIME;
  static constexpr int64_t kMonoClock = protos::pbzero::BUILTIN_CLOCK_MONOTONIC;

  // Timeline uses Trace Time clock as keys and other clocks time as values.
  using Timeline = std::map<Timestamp, Timestamp>;

  // Reads the clocks snapshots table and fetches the data required for
  // conversion. We initialize timelines of only selected clocks to minimize
  // memory usage. Currently those are MONO and REAL clocks.
  void MaybeInitialize();

  // Converts trace time to provided clock.
  base::StatusOr<Timestamp> FromTraceTime(ClockId, Timestamp);

  // Converts timestamp to string.
  std::string TimeToStr(Timestamp);

  TraceProcessorContext* context_;
  bool is_initialized = false;
  base::FlatHashMap<ClockId, Timeline> timelines_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_CLOCK_CONVERTER_H_
