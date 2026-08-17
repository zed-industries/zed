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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_CPU_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_CPU_TRACKER_H_

#include <bitset>
#include <cstdint>
#include <optional>

#include "perfetto/base/logging.h"
#include "perfetto/ext/base/string_view.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/metadata_tables_py.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

class CpuTracker {
 public:
  // The CPU table Id serves as the 'ucpu' in sched_slice and related for
  // joining with this table. To optimize for single-machine traces, this class
  // assumes a maximum of |kMaxCpusPerMachine| CPUs per machine to maintain a
  // relative order of |cpu| and |ucpu| by pre-allocating |kMaxCpusPerMachine|
  // records in the CPU table. The mapping between |ucpu| and |cpu| becomes
  // |cpu| = |ucpu| % |kMaxCpusPerMachine|.
  static constexpr uint32_t kMaxCpusPerMachine = 4096;

  explicit CpuTracker(TraceProcessorContext*);

  tables::CpuTable::Id GetOrCreateCpu(uint32_t cpu) {
    // CPU core number is in the range of 0..kMaxCpusPerMachine-1.
    PERFETTO_CHECK(cpu < kMaxCpusPerMachine);
    auto ucpu = ucpu_offset_ + cpu;
    if (PERFETTO_LIKELY(cpu_ids_[cpu]))
      return tables::CpuTable::Id(ucpu);
    cpu_ids_.set(cpu);

    // Populate the optional |cpu| column.
    auto& cpu_table = *context_->storage->mutable_cpu_table();
    cpu_table[ucpu].set_cpu(cpu);
    return tables::CpuTable::Id(ucpu);
  }

  void MarkCpuValid(uint32_t cpu) {
    PERFETTO_CHECK(cpu < kMaxCpusPerMachine);
    if (PERFETTO_LIKELY(cpu_ids_[cpu])) {
      return;
    }
    auto ucpu = ucpu_offset_ + cpu;
    auto& cpu_table = *context_->storage->mutable_cpu_table();
    cpu_table[ucpu].set_cpu(cpu);
  }

  // Sets or updates the information for the specified CPU in the CpuTable.
  tables::CpuTable::Id SetCpuInfo(uint32_t cpu,
                                  base::StringView processor,
                                  uint32_t cluster_id,
                                  std::optional<uint32_t> capacity);

 private:
  TraceProcessorContext* const context_;

  // Tracks the mapping of CPU number to CpuTable::Id of the current
  // machine.
  std::bitset<kMaxCpusPerMachine> cpu_ids_;
  uint32_t ucpu_offset_ = 0;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_CPU_TRACKER_H_
