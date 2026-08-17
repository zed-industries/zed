/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_MALI_GPU_EVENT_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_MALI_GPU_EVENT_TRACKER_H_

#include <array>
#include <cstdint>

#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/util/descriptors.h"

#include "protos/perfetto/trace/ftrace/ftrace_event.pbzero.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

class MaliGpuEventTracker {
 public:
  explicit MaliGpuEventTracker(TraceProcessorContext*);
  void ParseMaliGpuIrqEvent(int64_t timestamp,
                            uint32_t field_id,
                            uint32_t cpu,
                            protozero::ConstBytes blob);
  void ParseMaliGpuMcuStateEvent(int64_t timestamp, uint32_t field_id);

 private:
  void ParseMaliCSFInterruptStart(int64_t timestamp,
                                  TrackId track_id,
                                  protozero::ConstBytes blob);
  void ParseMaliCSFInterruptEnd(int64_t timestamp,
                                TrackId track_id,
                                protozero::ConstBytes blob);

  template <uint32_t FieldId>
  void RegisterMcuState(const char* state_name);

  static constexpr uint32_t kFirstMcuStateId = protos::pbzero::FtraceEvent::
      kMaliMaliPMMCUHCTLCORESDOWNSCALENOTIFYPENDFieldNumber;
  static constexpr uint32_t kLastMcuStateId =
      protos::pbzero::FtraceEvent::kMaliMaliPMMCURESETWAITFieldNumber;

  TraceProcessorContext* context_;
  StringId mali_KCPU_CQS_SET_id_;
  StringId mali_KCPU_CQS_WAIT_id_;
  StringId mali_KCPU_FENCE_SIGNAL_id_;
  StringId mali_KCPU_FENCE_WAIT_id_;
  StringId mali_CSF_INTERRUPT_id_;
  StringId mali_CSF_INTERRUPT_info_val_id_;

  std::array<StringId, (kLastMcuStateId - kFirstMcuStateId) + 1>
      mcu_state_names_;
  StringId current_mcu_state_name_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_FTRACE_MALI_GPU_EVENT_TRACKER_H_
