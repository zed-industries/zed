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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_BATTERY_STATS_READER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_BATTERY_STATS_READER_H_

#include <chrono>
#include <cstdint>
#include <optional>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/importers/android_bugreport/android_dumpstate_event.h"
#include "src/trace_processor/importers/android_bugreport/chunked_line_reader.h"

namespace perfetto ::trace_processor {

class TraceProcessorContext;

// Parses the battery stats checkin produded by (dumpsys batterystats -c),
class AndroidBatteryStatsReader : public ChunkedLineReader {
 public:
  explicit AndroidBatteryStatsReader(TraceProcessorContext* context);

  ~AndroidBatteryStatsReader() override;

  base::Status ParseLine(base::StringView line) override;
  void EndOfStream(base::StringView leftovers) override;

 private:
  // Called for each event parsed from the stream.
  // `event_ts` is the ts of the event as read from the log.
  // Default implementation just calls `SendToSorter`.
  base::Status ProcessBatteryStatsHistoryEvent(
      const base::StringView raw_event);

  // Sends the given event to the sorting stage.
  // `event_ts` is the ts of the event as read from the log and will be
  // converted to a trace_ts (with necessary clock conversions applied)
  base::Status SendToSorter(std::chrono::nanoseconds event_ts,
                            AndroidDumpstateEvent event);

  base::Status ProcessItemStr(base::StringView item);

  TraceProcessorContext* const context_;

  int64_t current_timestamp_ms_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ANDROID_BUGREPORT_ANDROID_BATTERY_STATS_READER_H_
