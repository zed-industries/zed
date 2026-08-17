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

#ifndef SRC_TRACE_REDACTION_REDACT_FTRACE_EVENTS_H_
#define SRC_TRACE_REDACTION_REDACT_FTRACE_EVENTS_H_

#include <string>

#include "perfetto/protozero/field.h"
#include "protos/perfetto/trace/ftrace/ftrace_event_bundle.pbzero.h"
#include "src/trace_redaction/redact_sched_events.h"
#include "src/trace_redaction/trace_redaction_framework.h"

namespace perfetto::trace_redaction {

class FilterFtraceUsingSuspendResume : public FtraceEventFilter {
 public:
  bool Includes(const Context& context, protozero::Field event) const override;
};

// Discard all rss events not belonging to the target package.
class FilterRss : public FtraceEventFilter {
 public:
  bool Includes(const Context& context, protozero::Field event) const override;
};

// Filters ftrace events and modifies remaining events before writing them to
// the packet. Only one filter and/or writer can be assigned to provide finer
// grain control.
class RedactFtraceEvents : public TransformPrimitive {
 public:
  base::Status Transform(const Context& context,
                         std::string* packet) const override;

  // Selects which ftrace events should be redacted. All non-ftrace events are
  // appended to the new packet.
  template <typename Filter>
  void emplace_ftrace_filter() {
    filter_ = std::make_unique<Filter>();
  }

  // For ftrace events that pass the filter, they go through this modifier which
  // will optionally modify the event before adding it to the event bundle (or
  // even drop it).
  template <typename Modifier>
  void emplace_post_filter_modifier() {
    modifier_ = std::make_unique<Modifier>();
  }

 private:
  base::Status OnFtraceEvents(const Context& context,
                              protozero::Field ftrace_events,
                              protos::pbzero::FtraceEventBundle* message) const;

  void OnFtraceEvent(const Context& context,
                     const protos::pbzero::FtraceEventBundle::Decoder& bundle,
                     protozero::Field event,
                     protos::pbzero::FtraceEventBundle* parent_message) const;

  std::unique_ptr<FtraceEventFilter> filter_;
  std::unique_ptr<PidCommModifier> modifier_;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_REDACT_FTRACE_EVENTS_H_
