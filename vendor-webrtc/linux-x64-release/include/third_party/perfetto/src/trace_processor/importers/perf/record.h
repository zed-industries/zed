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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_RECORD_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_RECORD_H_

#include <optional>

#include "perfetto/trace_processor/ref_counted.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "protos/perfetto/trace/profiling/profile_packet.pbzero.h"
#include "src/trace_processor/importers/perf/perf_event.h"
#include "src/trace_processor/importers/perf/perf_event_attr.h"
#include "src/trace_processor/importers/perf/perf_session.h"

namespace perfetto::trace_processor::perf_importer {

// Minimally parsed perf event record. Contains enough information to be able to
// send the record to the sorting stage.
struct Record {
 public:
  protos::pbzero::Profiling::CpuMode GetCpuMode() const {
    switch (header.misc & kPerfRecordMiscCpumodeMask) {
      case PERF_RECORD_MISC_KERNEL:
        return protos::pbzero::Profiling::MODE_KERNEL;
      case PERF_RECORD_MISC_USER:
        return protos::pbzero::Profiling::MODE_USER;
      case PERF_RECORD_MISC_HYPERVISOR:
        return protos::pbzero::Profiling::MODE_HYPERVISOR;
      case PERF_RECORD_MISC_GUEST_KERNEL:
        return protos::pbzero::Profiling::MODE_GUEST_KERNEL;
      case PERF_RECORD_MISC_GUEST_USER:
        return protos::pbzero::Profiling::MODE_GUEST_USER;
      default:
        return protos::pbzero::Profiling::MODE_UNKNOWN;
    }
  }

  bool has_trailing_sample_id() const {
    if (!attr) {
      return false;
    }
    return attr->sample_id_all() && header.type != PERF_RECORD_SAMPLE &&
           header.type < PERF_RECORD_USER_TYPE_START;
  }

  bool mmap_has_build_id() const {
    return header.misc & PERF_RECORD_MISC_MMAP_BUILD_ID;
  }

  // Returns the payload offset to the time field if present.
  std::optional<size_t> time_offset() const {
    if (!attr) {
      return std::nullopt;
    }
    return header.type == PERF_RECORD_SAMPLE ? attr->time_offset_from_start()
                                             : attr->time_offset_from_end();
  }

  RefPtr<PerfSession> session;
  RefPtr<PerfEventAttr> attr;
  perf_event_header header;
  TraceBlobView payload;
};

}  // namespace perfetto::trace_processor::perf_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_RECORD_H_
