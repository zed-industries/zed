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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SAMPLE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SAMPLE_H_

#include <cstdint>
#include <optional>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/trace_processor/ref_counted.h"
#include "protos/perfetto/trace/profiling/profile_packet.pbzero.h"
#include "src/trace_processor/importers/perf/perf_event_attr.h"
#include "src/trace_processor/importers/perf/perf_session.h"

namespace perfetto::trace_processor::perf_importer {

struct Record;

struct Sample {
  struct Frame {
    protos::pbzero::Profiling::CpuMode cpu_mode;
    uint64_t ip;
  };

  struct PidTid {
    uint32_t pid;
    uint32_t tid;
  };

  struct ReadGroup {
    std::optional<uint64_t> event_id;
    uint64_t value;
  };

  int64_t trace_ts;
  protos::pbzero::Profiling::CpuMode cpu_mode;
  RefPtr<PerfSession> perf_session;
  RefPtr<PerfEventAttr> attr;

  std::optional<uint64_t> ip;
  std::optional<PidTid> pid_tid;
  std::optional<uint64_t> time;
  std::optional<uint64_t> addr;
  std::optional<uint64_t> id;
  std::optional<uint64_t> stream_id;
  std::optional<uint32_t> cpu;
  std::optional<uint64_t> period;
  std::vector<ReadGroup> read_groups;
  std::vector<Frame> callchain;

  base::Status Parse(int64_t trace_ts, const Record& record);
};

}  // namespace perfetto::trace_processor::perf_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SAMPLE_H_
