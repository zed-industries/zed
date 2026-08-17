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

#ifndef SRC_TRACE_REDACTION_REDACT_PROCESS_EVENTS_H_
#define SRC_TRACE_REDACTION_REDACT_PROCESS_EVENTS_H_

#include <memory>

#include "src/trace_redaction/redact_sched_events.h"
#include "src/trace_redaction/trace_redaction_framework.h"

#include "protos/perfetto/trace/ftrace/ftrace_event_bundle.pbzero.h"

namespace perfetto::trace_redaction {

// Goes through a trace packet and filters:
//
//    - task_rename
//    - task_newtask
//    - sched_process_free
//    - print
//
// Goes through a trace packet and modifies pid and comm:
//
//    - task_newtask
//    - sched_process_free
//    - task_rename
//
// 'print' does not support modification.
//
// These operations are separate from the scheduling events in an effort to make
// the code easier to understand, however they use the same filter and modifier
// types and should have the same values when used together.
class RedactProcessEvents : public TransformPrimitive {
 public:
  base::Status Transform(const Context& context,
                         std::string* packet) const override;

  template <class Modifier>
  void emplace_modifier() {
    modifier_ = std::make_unique<Modifier>();
  }

  template <class Filter>
  void emplace_filter() {
    filter_ = std::make_unique<Filter>();
  }

  template <class Filter>
  void emplace_filter(std::unique_ptr<Filter> filter) {
    filter_ = std::move(filter);
  }

 private:
  base::Status OnFtraceEvents(const Context& context,
                              protozero::ConstBytes bytes,
                              protos::pbzero::FtraceEventBundle* message) const;

  base::Status OnFtraceEvent(const Context& context,
                             int32_t cpu,
                             protozero::ConstBytes bytes,
                             std::string* shared_comm,
                             protos::pbzero::FtraceEvent* message) const;

  base::Status OnProcessFree(const Context& context,
                             uint64_t ts,
                             int32_t cpu,
                             protozero::ConstBytes bytes,
                             std::string* shared_comm,
                             protos::pbzero::FtraceEvent* parent_message) const;

  base::Status OnNewTask(const Context& context,
                         uint64_t ts,
                         int32_t cpu,
                         protozero::ConstBytes bytes,
                         std::string* shared_comm,
                         protos::pbzero::FtraceEvent* parent_message) const;

  base::Status OnProcessRename(
      const Context& context,
      uint64_t ts,
      int32_t cpu,
      int32_t pid,
      protozero::ConstBytes bytes,
      std::string* shared_comm,
      protos::pbzero::FtraceEvent* parent_message) const;

  // Unlike the other On* functions, this one required the event's byte buffer
  // because it needs the pid from it.
  base::Status OnPrint(const Context& context,
                       uint64_t ts,
                       protozero::ConstBytes event_bytes,
                       protos::pbzero::FtraceEvent* parent_message) const;

  base::Status OnSuspendResume(
      const Context& context,
      uint64_t ts,
      protozero::ConstBytes event_bytes,
      protos::pbzero::FtraceEvent* parent_message) const;

  base::Status OnSchedBlockedReason(
      const Context& context,
      uint64_t ts,
      protozero::ConstBytes event_bytes,
      protos::pbzero::FtraceEvent* parent_message) const;

  std::unique_ptr<PidCommModifier> modifier_;
  std::unique_ptr<PidFilter> filter_;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_REDACT_PROCESS_EVENTS_H_
