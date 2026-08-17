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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_MACHINE_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_MACHINE_TRACKER_H_

#include <cstdint>

#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor {

// Tracks information in the machine table.
class MachineTracker {
 public:
  MachineTracker(TraceProcessorContext* context, uint32_t raw_machine_id);
  ~MachineTracker();

  std::optional<MachineId> machine_id() const { return machine_id_; }

 private:
  std::optional<MachineId> machine_id_;
  TraceProcessorContext* const context_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_MACHINE_TRACKER_H_
