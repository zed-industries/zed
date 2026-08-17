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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETW_ETW_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETW_ETW_PARSER_H_

#include "perfetto/protozero/field.h"
#include "perfetto/trace_processor/status.h"
#include "src/trace_processor/importers/common/parser_types.h"
#include "src/trace_processor/importers/common/sched_event_state.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

class EtwParser {
 public:
  explicit EtwParser(TraceProcessorContext* context);

  base::Status ParseEtwEvent(uint32_t cpu,
                             int64_t ts,
                             const TracePacketData& data);

 private:
  void ParseCswitch(int64_t timestamp, uint32_t cpu, protozero::ConstBytes);
  void ParseReadyThread(int64_t timestamp, protozero::ConstBytes);
  void PushSchedSwitch(uint32_t cpu,
                       int64_t timestamp,
                       uint32_t prev_pid,
                       int64_t prev_state,
                       uint32_t next_pid,
                       int32_t next_prio);
  StringId TaskStateToStringId(int64_t task_state_int);

  TraceProcessorContext* context_;

  SchedEventState sched_event_state_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETW_ETW_PARSER_H_
