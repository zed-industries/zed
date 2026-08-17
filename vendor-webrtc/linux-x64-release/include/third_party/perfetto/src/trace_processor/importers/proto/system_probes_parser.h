/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_SYSTEM_PROBES_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_SYSTEM_PROBES_PARSER_H_

#include <cstddef>
#include <cstdint>
#include <vector>

#include "perfetto/protozero/field.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

class SystemProbesParser {
 public:
  using ConstBytes = protozero::ConstBytes;
  using ConstChars = protozero::ConstChars;

  explicit SystemProbesParser(TraceProcessorContext*);

  void ParseProcessTree(ConstBytes);
  void ParseProcessStats(int64_t ts, ConstBytes);
  void ParseSysStats(int64_t ts, ConstBytes);
  void ParseSystemInfo(ConstBytes);
  void ParseCpuInfo(ConstBytes);

 private:
  void ParseThreadStats(int64_t timestamp, uint32_t pid, ConstBytes);
  void ParseDiskStats(int64_t ts, ConstBytes blob);
  void ParseProcessFds(int64_t ts, uint32_t pid, ConstBytes);
  void ParseCpuIdleStats(int64_t ts, ConstBytes);

  TraceProcessorContext* const context_;

  const StringId utid_name_id_;
  const StringId is_kthread_id_;

  // Arm CPU identifier string IDs
  const StringId arm_cpu_implementer;
  const StringId arm_cpu_architecture;
  const StringId arm_cpu_variant;
  const StringId arm_cpu_part;
  const StringId arm_cpu_revision;

  std::vector<const char*> meminfo_strs_;
  std::vector<const char*> vmstat_strs_;

  uint32_t page_size_ = 0;

  int64_t prev_read_amount = -1;
  int64_t prev_write_amount = -1;
  int64_t prev_discard_amount = -1;
  int64_t prev_flush_count = -1;
  int64_t prev_read_time = -1;
  int64_t prev_write_time = -1;
  int64_t prev_discard_time = -1;
  int64_t prev_flush_time = -1;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_SYSTEM_PROBES_PARSER_H_
