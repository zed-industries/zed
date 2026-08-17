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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_LEGACY_V8_CPU_PROFILE_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_LEGACY_V8_CPU_PROFILE_TRACKER_H_

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/status_or.h"
#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/importers/common/virtual_memory_mapping.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor {

// Stores interned callsites for given pid for legacy v8 samples.
class LegacyV8CpuProfileTracker {
 public:
  explicit LegacyV8CpuProfileTracker(TraceProcessorContext*);

  // Sets the start timestamp for the given pid.
  void SetStartTsForSessionAndPid(uint64_t session_id,
                                  uint32_t pid,
                                  int64_t ts);

  // Adds the callsite with for the given session and pid and given raw callsite
  // id.
  base::Status AddCallsite(
      uint64_t session_id,
      uint32_t pid,
      uint32_t raw_callsite_id,
      std::optional<uint32_t> parent_raw_callsite_id,
      base::StringView script_url,
      base::StringView function_name,
      const std::vector<uint32_t>& raw_children_callsite_ids);

  // Increments the current timestamp for the given session and pid by
  // |delta_ts| and returns the resulting full timestamp.
  base::StatusOr<int64_t> AddDeltaAndGetTs(uint64_t session_id,
                                           uint32_t pid,
                                           int64_t delta_ts);

  // Adds the sample with for the given session and pid/tid and given raw
  // callsite id.
  base::Status AddSample(int64_t ts,
                         uint64_t session_id,
                         uint32_t pid,
                         uint32_t tid,
                         uint32_t raw_callsite_id);

 private:
  struct State {
    int64_t ts;
    base::FlatHashMap<uint32_t, CallsiteId> callsites;
    base::FlatHashMap<uint32_t, uint32_t> callsite_inferred_parents;
    DummyMemoryMapping* mapping;
  };
  struct Hasher {
    uint64_t operator()(const std::pair<uint64_t, uint32_t>& res) {
      return base::Hasher::Combine(res.first, res.second);
    }
  };
  base::FlatHashMap<std::pair<uint64_t, uint32_t>, State, Hasher>
      state_by_session_and_pid_;

  TraceProcessorContext* const context_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_LEGACY_V8_CPU_PROFILE_TRACKER_H_
