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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_ITRACE_START_RECORD_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_ITRACE_START_RECORD_H_

#include <cstdint>
#include <optional>

#include "perfetto/base/status.h"
#include "src/trace_processor/importers/perf/perf_event_attr.h"
#include "src/trace_processor/importers/perf/sample_id.h"

namespace perfetto::trace_processor::perf_importer {

struct Record;

struct ItraceStartRecord {
  RefPtr<PerfEventAttr> attr;
  uint32_t pid;
  uint32_t tid;
  std::optional<SampleId> sample_id;

  std::optional<uint64_t> time() const {
    return sample_id.has_value() ? sample_id->time() : std::nullopt;
  }

  base::Status Parse(const Record& record);
};

}  // namespace perfetto::trace_processor::perf_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_ITRACE_START_RECORD_H_
