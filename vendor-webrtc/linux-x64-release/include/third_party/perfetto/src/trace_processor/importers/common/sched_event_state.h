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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_SCHED_EVENT_STATE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_SCHED_EVENT_STATE_H_

#include <limits>
#include <vector>

#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto {
namespace trace_processor {

// Responsible for keeping the state of pending sched events.
// TODO(rsavitski): consider folding back into ftrace parser. The ETW parser is
// probably better off replicating its own pending state struct.
class SchedEventState {
 public:
  // Information retained from the preceding sched_switch seen on a given cpu.
  struct PendingSchedInfo {
    // The pending scheduling slice that the next event will complete.
    uint32_t pending_slice_storage_idx = std::numeric_limits<uint32_t>::max();

    // pid/utid/prio corresponding to the last sched_switch seen on this cpu
    // (its "next_*" fields). There is some duplication with respect to the
    // slices storage, but we don't always have a slice when decoding events in
    // the compact format.
    uint32_t last_pid = std::numeric_limits<uint32_t>::max();
    UniqueTid last_utid = std::numeric_limits<UniqueTid>::max();
    int32_t last_prio = std::numeric_limits<int32_t>::max();
  };

  SchedEventState() {
    // Pre-allocate space for 128 CPUs, which should be enough for most hosts.
    // It's OK if this number is too small, the vector will be grown on-demand.
    pending_sched_per_cpu_.reserve(128);
  }
  SchedEventState(const SchedEventState&) = delete;
  ~SchedEventState() = default;

  // Get the sched info for the given CPU, resizing the vector if necessary.
  PendingSchedInfo* GetPendingSchedInfoForCpu(uint32_t cpu) {
    if (PERFETTO_UNLIKELY(cpu >= pending_sched_per_cpu_.size())) {
      pending_sched_per_cpu_.resize(cpu + 1);
    }
    return &pending_sched_per_cpu_[cpu];
  }

 private:
  // Information retained from the preceding sched_switch seen on a given cpu.
  std::vector<PendingSchedInfo> pending_sched_per_cpu_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_SCHED_EVENT_STATE_H_
