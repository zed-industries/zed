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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_TARGET_MEMORY_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_TARGET_MEMORY_H_

#include <cstdint>
#include <optional>

#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/importers/etm/virtual_address_space.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/destructible.h"

namespace perfetto::trace_processor {
class AddressRange;
class TraceProcessorContext;
namespace etm {

class MappingVersion;

// This class represents tracks the memory contents for all processes.
// It can answer queries in the form: At timestamp t, what was the mapping at
// address x for the thread tid.
class TargetMemory : public Destructible {
 public:
  static bool IsKernelAddress(uint64_t address) {
    return address & (1ull << 63);
  }

  static void InitStorage(TraceProcessorContext* context);
  static const TargetMemory* Get(TraceStorage* storage) {
    PERFETTO_DCHECK(storage->etm_target_memory() != nullptr);
    return static_cast<const TargetMemory*>(storage->etm_target_memory());
  }

  ~TargetMemory() override;

  TraceStorage* storage() const { return storage_; }

  const MappingVersion* FindMapping(int64_t ts,
                                    uint32_t tid,
                                    uint64_t address) const;
  const MappingVersion* FindMapping(int64_t ts,
                                    uint32_t tid,
                                    const AddressRange& range) const;

 private:
  explicit TargetMemory(TraceProcessorContext* context);
  VirtualAddressSpace* FindUserSpaceForTid(uint32_t tid) const;
  std::optional<UniquePid> FindUpidForTid(uint32_t tid) const;

  TraceStorage* const storage_;
  // Kernel memory is shared by all processes.
  VirtualAddressSpace kernel_memory_;

  base::FlatHashMap<UniquePid, VirtualAddressSpace> user_memory_;

  // Cache for quick tid -> upid lookups.
  // TODO(carlscab): This should probably live in `ProcessTracker`
  mutable base::FlatHashMap<uint32_t, VirtualAddressSpace*> tid_to_space_;
};

}  // namespace etm
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_TARGET_MEMORY_H_
