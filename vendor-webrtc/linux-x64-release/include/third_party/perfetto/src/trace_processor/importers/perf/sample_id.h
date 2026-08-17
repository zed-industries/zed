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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SAMPLE_ID_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SAMPLE_ID_H_

#include <cstdint>
#include <optional>

#include "perfetto/base/status.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/perf/perf_event.h"
#include "src/trace_processor/importers/perf/record.h"

namespace perfetto::trace_processor::perf_importer {

class PerfEventAttr;
class Reader;

class SampleId {
 public:
  base::Status ParseFromRecord(const Record& record);
  bool ReadFrom(const PerfEventAttr& attr, Reader& reader);

  SampleId() : sample_type_(0) {}

  std::optional<uint32_t> tid() const {
    return sample_type_ & PERF_SAMPLE_TID ? std::make_optional(tid_)
                                          : std::nullopt;
  }
  std::optional<uint32_t> pid() const {
    return sample_type_ & PERF_SAMPLE_TID ? std::make_optional(pid_)
                                          : std::nullopt;
  }
  std::optional<uint64_t> time() const {
    return sample_type_ & PERF_SAMPLE_TIME ? std::make_optional(time_)
                                           : std::nullopt;
  }
  std::optional<uint64_t> id() const {
    return sample_type_ & (PERF_SAMPLE_ID | PERF_SAMPLE_IDENTIFIER)
               ? std::make_optional(id_)
               : std::nullopt;
  }
  std::optional<uint64_t> stream_id() const {
    return sample_type_ & PERF_SAMPLE_STREAM_ID ? std::make_optional(stream_id_)
                                                : std::nullopt;
  }
  std::optional<uint32_t> cpu() const {
    return sample_type_ & PERF_SAMPLE_CPU ? std::make_optional(cpu_)
                                          : std::nullopt;
  }

  void set_cpu(std::optional<uint32_t> cpu) {
    if (cpu.has_value()) {
      sample_type_ |= PERF_SAMPLE_CPU;
      cpu_ = *cpu;
    } else {
      sample_type_ &= ~static_cast<uint64_t>(PERF_SAMPLE_CPU);
    }
  }

 private:
  uint32_t tid_;
  uint32_t pid_;
  uint64_t time_;
  uint64_t id_;
  uint64_t stream_id_;
  uint32_t cpu_;
  uint64_t sample_type_;
};

}  // namespace perfetto::trace_processor::perf_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_SAMPLE_ID_H_
