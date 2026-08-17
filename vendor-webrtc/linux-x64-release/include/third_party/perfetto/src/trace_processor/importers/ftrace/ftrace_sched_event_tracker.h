/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_FTRACE_SCHED_EVENT_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_FTRACE_SCHED_EVENT_TRACKER_H_

#include <cstdint>

#include <array>

#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/importers/common/sched_event_state.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

class EventTracker;

// Tracks sched events and stores them into the storage as sched slices.
class FtraceSchedEventTracker : public Destructible {
 public:
  explicit FtraceSchedEventTracker(TraceProcessorContext*);
  ~FtraceSchedEventTracker() override;

  FtraceSchedEventTracker(const FtraceSchedEventTracker&) = delete;
  FtraceSchedEventTracker& operator=(const FtraceSchedEventTracker&) = delete;

  static FtraceSchedEventTracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->ftrace_sched_tracker) {
      context->ftrace_sched_tracker.reset(new FtraceSchedEventTracker(context));
    }
    return static_cast<FtraceSchedEventTracker*>(
        context->ftrace_sched_tracker.get());
  }

  // This method is called when a sched_switch event is seen in the trace.
  // Virtual for testing.
  virtual void PushSchedSwitch(uint32_t cpu,
                               int64_t timestamp,
                               uint32_t prev_pid,
                               base::StringView prev_comm,
                               int32_t prev_prio,
                               int64_t prev_state,
                               uint32_t next_pid,
                               base::StringView next_comm,
                               int32_t next_prio);

  void AddRawSchedSwitchEvent(uint32_t cpu,
                              int64_t ts,
                              UniqueTid prev_utid,
                              uint32_t prev_pid,
                              StringId prev_comm_id,
                              int32_t prev_prio,
                              int64_t prev_state,
                              uint32_t next_pid,
                              StringId next_comm_id,
                              int32_t next_prio);

  // This method is called when parsing a sched_switch encoded in the compact
  // format.
  void PushSchedSwitchCompact(uint32_t cpu,
                              int64_t ts,
                              int64_t prev_state,
                              uint32_t next_pid,
                              int32_t next_prio,
                              StringId next_comm_id,
                              bool parse_only_into_raw);

  // This method is called when parsing a sched_waking encoded in the compact
  // format. Note that the default encoding is handled by
  // |EventTracker::PushInstant|.
  void PushSchedWakingCompact(uint32_t cpu,
                              int64_t ts,
                              uint32_t wakee_pid,
                              uint16_t target_cpu,
                              uint16_t prio,
                              StringId comm_id,
                              uint16_t common_flags,
                              bool parse_only_into_raw);

 private:
  StringId TaskStateToStringId(int64_t task_state_int);

  static constexpr uint8_t kSchedSwitchMaxFieldId = 7;
  std::array<StringId, kSchedSwitchMaxFieldId + 1> sched_switch_field_ids_;
  StringId sched_switch_id_;

  static constexpr uint8_t kSchedWakingMaxFieldId = 5;
  std::array<StringId, kSchedWakingMaxFieldId + 1> sched_waking_field_ids_;
  StringId sched_waking_id_;

  TraceProcessorContext* const context_;

  SchedEventState sched_event_state_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_FTRACE_SCHED_EVENT_TRACKER_H_
