/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_EVENT_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_EVENT_TRACKER_H_

#include <algorithm>
#include <cstdint>
#include <functional>
#include <optional>
#include <variant>
#include <vector>

#include "src/trace_processor/importers/common/args_tracker.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

// Tracks sched events, instants, and counters.
class EventTracker {
 public:
  struct OomScoreAdj {};
  struct MmEvent {
    const char* type;
    const char* metric;
  };
  struct RssStat {
    const char* process_memory_key;
  };
  struct JsonCounter {
    StringId counter_name_id;
  };
  using ProcessCounterForThread =
      std::variant<OomScoreAdj, MmEvent, RssStat, JsonCounter>;

  using SetArgsCallback = std::function<void(ArgsTracker::BoundInserter*)>;

  explicit EventTracker(TraceProcessorContext*);
  EventTracker(const EventTracker&) = delete;
  EventTracker& operator=(const EventTracker&) = delete;
  virtual ~EventTracker();

  // Adds a counter event to the counters table returning the index of the
  // newly added row.
  virtual std::optional<CounterId> PushCounter(int64_t timestamp,
                                               double value,
                                               TrackId track_id);

  // Adds a counter event with args to the counters table returning the index of
  // the newly added row.
  std::optional<CounterId> PushCounter(int64_t timestamp,
                                       double value,
                                       TrackId track_id,
                                       const SetArgsCallback& args_callback);

  // Adds a counter event to the counters table for counter events which
  // should be associated with a process but only have a thread context
  // (e.g. rss_stat events).
  //
  // This function will resolve the utid to a upid when the events are
  // flushed (see |FlushPendingEvents()|).
  void PushProcessCounterForThread(ProcessCounterForThread,
                                   int64_t timestamp,
                                   double value,
                                   UniqueTid utid);

  // Called at the end of trace to flush any events which are pending to the
  // storage.
  void FlushPendingEvents();

 private:
  // Represents a counter event which is currently pending upid resolution.
  struct PendingUpidResolutionCounter {
    ProcessCounterForThread counter;
    uint32_t row = 0;
    UniqueTid utid = 0;
  };

  // Store the rows in the counters table which need upids resolved.
  std::vector<PendingUpidResolutionCounter> pending_upid_resolution_counter_;

  TraceProcessorContext* const context_;
};
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_EVENT_TRACKER_H_
