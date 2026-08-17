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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PERF_MMAP_RECORD_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PERF_MMAP_RECORD_H_

#include <cstdint>
#include <optional>
#include <string>
#include "perfetto/base/status.h"
#include "protos/perfetto/trace/profiling/profile_packet.pbzero.h"
#include "src/trace_processor/util/build_id.h"

namespace perfetto::trace_processor::perf_importer {

struct Record;

struct CommonMmapRecordFields {
  uint32_t pid;
  uint32_t tid;
  uint64_t addr;
  uint64_t len;
  uint64_t pgoff;
};

struct MmapRecord : public CommonMmapRecordFields {
  std::string filename;
  protos::pbzero::Profiling::CpuMode cpu_mode;

  base::Status Parse(const Record& record);
};

struct BaseMmap2Record : public CommonMmapRecordFields {
  struct BuildIdFields {
    static constexpr size_t kMaxBuildIdSize = 20;
    uint8_t build_id_size;
    uint8_t reserved_1;
    uint16_t reserved_2;
    char build_id_buf[kMaxBuildIdSize];
  };
  struct InodeFields {
    uint32_t maj;
    uint32_t min;
    int64_t ino;
    uint64_t ino_generation;
  };
  static_assert(sizeof(BuildIdFields) == sizeof(InodeFields));

  union {
    BuildIdFields build_id;
    InodeFields inode;
  };
  uint32_t prot;
  uint32_t flags;
};

struct Mmap2Record : public BaseMmap2Record {
  std::string filename;
  protos::pbzero::Profiling::CpuMode cpu_mode;
  bool has_build_id;

  base::Status Parse(const Record& record);
  std::optional<BuildId> GetBuildId() const;
};

}  // namespace perfetto::trace_processor::perf_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PERF_MMAP_RECORD_H_
