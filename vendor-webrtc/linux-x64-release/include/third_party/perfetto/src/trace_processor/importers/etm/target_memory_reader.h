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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_TARGET_MEMORY_READER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_TARGET_MEMORY_READER_H_

#include <cstdint>
#include <optional>
#include "src/trace_processor/importers/common/address_range.h"

#include "src/trace_processor/importers/common/address_range.h"
#include "src/trace_processor/importers/etm/opencsd.h"
namespace perfetto::trace_processor::etm {

class MappingVersion;
class TargetMemory;
class MemoryContentProvider;
class BinaryIndex;

class TargetMemoryReader : public ITargetMemAccess {
 public:
  explicit TargetMemoryReader(const TargetMemory* memory) : memory_(memory) {}

  ocsd_err_t ReadTargetMemory(const ocsd_vaddr_t address,
                              const uint8_t cs_trace_id,
                              const ocsd_mem_space_acc_t mem_space,
                              uint32_t* num_bytes,
                              uint8_t* p_buffer) override;
  void InvalidateMemAccCache(const uint8_t cs_trace_id) override;

  void SetTs(int64_t ts);
  void SetPeContext(const ocsd_pe_context&);

  const MappingVersion* FindMapping(uint64_t address) const;

 private:
  const TargetMemory* memory_;

  std::optional<uint32_t> tid_;
  int64_t ts_;
  // Cache last mapping to speedup lookups.
  mutable const MappingVersion* cached_mapping_;
};

}  // namespace perfetto::trace_processor::etm

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_TARGET_MEMORY_READER_H_
