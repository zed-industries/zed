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

#ifndef SRC_TRACE_REDACTION_REDACT_SCHED_EVENTS_H_
#define SRC_TRACE_REDACTION_REDACT_SCHED_EVENTS_H_

#include "src/trace_redaction/filtering.h"
#include "src/trace_redaction/modify.h"
#include "src/trace_redaction/trace_redaction_framework.h"

#include "protos/perfetto/trace/ftrace/ftrace_event_bundle.pbzero.h"
#include "protos/perfetto/trace/ftrace/sched.pbzero.h"

namespace perfetto::trace_redaction {

class InternTable {
 public:
  int64_t Push(const char* data, size_t size);

  std::string_view Find(size_t index) const;

  const std::vector<std::string_view>& values() const {
    return interned_comms_;
  }

 private:
  constexpr static size_t kExpectedCommLength = 16;
  constexpr static size_t kMaxElements = 4096;

  std::array<char, kMaxElements * kExpectedCommLength> comms_;
  size_t comms_length_ = 0;

  std::vector<std::string_view> interned_comms_;
};

class RedactSchedEvents : public TransformPrimitive {
 public:
  base::Status Transform(const Context& context,
                         std::string* packet) const override;

  template <class Modifier>
  void emplace_modifier() {
    modifier_ = std::make_unique<Modifier>();
  }

  template <class Filter>
  void emplace_waking_filter() {
    waking_filter_ = std::make_unique<Filter>();
  }

 private:
  base::Status OnFtraceEvents(const Context& context,
                              protozero::Field ftrace_events,
                              protos::pbzero::FtraceEventBundle* message) const;

  base::Status OnFtraceEvent(const Context& context,
                             int32_t cpu,
                             protozero::Field ftrace_event,
                             protos::pbzero::FtraceEvent* message) const;

  // scratch_str is a reusable string, allowing comm modifications to be done in
  // a shared buffer, avoiding allocations when processing ftrace events.
  base::Status OnFtraceEventSwitch(
      const Context& context,
      uint64_t ts,
      int32_t cpu,
      protos::pbzero::SchedSwitchFtraceEvent::Decoder& sched_switch,
      std::string* scratch_str,
      protos::pbzero::SchedSwitchFtraceEvent* message) const;

  // Unlike other On* functions, this one takes the parent message, allowing it
  // to optionally add the body. This is what allows the waking event to be
  // removed.
  base::Status OnFtraceEventWaking(
      const Context& context,
      uint64_t ts,
      int32_t cpu,
      protos::pbzero::SchedWakingFtraceEvent::Decoder& sched_waking,
      std::string* scratch_str,
      protos::pbzero::FtraceEvent* parent_message) const;

  base::Status OnCompSched(
      const Context& context,
      int32_t cpu,
      protos::pbzero::FtraceEventBundle::CompactSched::Decoder& comp_sched,
      protos::pbzero::FtraceEventBundle::CompactSched* message) const;

  base::Status OnCompSchedSwitch(
      const Context& context,
      int32_t cpu,
      protos::pbzero::FtraceEventBundle::CompactSched::Decoder& comp_sched,
      InternTable* intern_table,
      protos::pbzero::FtraceEventBundle::CompactSched* message) const;

  base::Status OnCompactSchedWaking(
      const Context& context,
      protos::pbzero::FtraceEventBundle::CompactSched::Decoder& compact_sched,
      InternTable* intern_table,
      protos::pbzero::FtraceEventBundle::CompactSched* compact_sched_message)
      const;

  std::unique_ptr<PidCommModifier> modifier_;
  std::unique_ptr<PidFilter> waking_filter_;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_REDACT_SCHED_EVENTS_H_
