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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_ACTIVE_CHROME_PROCESSES_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_ACTIVE_CHROME_PROCESSES_TRACKER_H_

#include <optional>
#include <set>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

struct ProcessWithDataLoss {
  UniquePid upid;
  // If not std::nullopt, the process data is reliable from this point until
  // the end of the trace.
  std::optional<int64_t> reliable_from;
};

// Tracks ActiveProcesses metadata packets from ChromeTrackEvent,
// and process descriptors.
// Computes a list of processes with missing data based on this information.
class ActiveChromeProcessesTracker {
 public:
  explicit ActiveChromeProcessesTracker(TraceProcessorContext* context)
      : context_(context) {}

  void AddActiveProcessMetadata(int64_t timestamp, UniquePid upid);
  void AddProcessDescriptor(int64_t timestamp, UniquePid upid) {
    process_data_[upid].descriptor_timestamps.insert(timestamp);
  }
  std::vector<ProcessWithDataLoss> GetProcessesWithDataLoss() const;
  void NotifyEndOfFile();

 private:
  struct ProcessData {
    std::set<int64_t> metadata_timestamps;
    std::set<int64_t> descriptor_timestamps;
  };

  TraceProcessorContext* context_;
  base::FlatHashMap<UniquePid, ProcessData> process_data_;
  // Metadata timestamps across all processes.
  std::set<int64_t> global_metadata_timestamps_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_ACTIVE_CHROME_PROCESSES_TRACKER_H_
