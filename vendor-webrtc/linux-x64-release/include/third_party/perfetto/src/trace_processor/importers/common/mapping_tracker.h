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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_MAPPING_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_MAPPING_TRACKER_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/importers/common/address_range.h"
#include "src/trace_processor/importers/common/virtual_memory_mapping.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"
#include "src/trace_processor/util/build_id.h"

namespace perfetto {
namespace trace_processor {

class JitCache;

// Keeps track of all aspects relative to memory mappings.
// This class keeps track of 3 types of mappings: UserMemoryMapping,
// KernelMemoryMapping and others. The others are used to represent mapping
// where we do not have enough information to determine what type of
// mapping (user, kernel) we are dealing with. This is usually the case with
// data sources that do not provide enough information about the mappings.
//
// TODO(carlscab): Hopefully we can slowly get rid of cases where these other
// mappings are needed. The biggest blocker right now is determining the upid.
// we could infer this from the actual samples that use said mapping (those
// usually have a pid attached). So we would need to have a "fake" mapping that
// actually materializes when we see a sample with a pid.
//
// ATTENTION: No overlaps allowed (for now). Eventually the order in which
// mappings are create will matter as newer mappings will delete old ones.
// This is how tools like linux perf behave, mmap event have a timestamp
// associated and there are no "delete events" just new mmap events that
// overlap (to be deleted) mappings.
class MappingTracker {
 public:
  explicit MappingTracker(TraceProcessorContext* context) : context_(context) {}

  // Create a new kernel space mapping. Returned reference will be valid for the
  // duration of this instance.
  KernelMemoryMapping& CreateKernelMemoryMapping(CreateMappingParams params);

  // Create a new user space mapping. Returned reference will be valid for the
  // duration of this instance.
  UserMemoryMapping& CreateUserMemoryMapping(UniquePid upid,
                                             CreateMappingParams params);

  // Sometimes we just need a mapping and we are lacking trace data to create a
  // proper one. Use this mapping in those cases.
  DummyMemoryMapping& CreateDummyMapping(std::string name);

  // Create an "other" mapping. Returned reference will be valid for the
  // duration of this instance.
  VirtualMemoryMapping& InternMemoryMapping(CreateMappingParams params);

  // Given an absolute address find the kernel mapping where this address
  // belongs to. Returns `nullptr` if none is found.
  KernelMemoryMapping* FindKernelMappingForAddress(uint64_t address) const;

  // Given an absolute address find the user mapping where this address
  // belongs to. Returns `nullptr` if none is found.
  UserMemoryMapping* FindUserMappingForAddress(UniquePid upid,
                                               uint64_t address) const;

  std::vector<VirtualMemoryMapping*> FindMappings(
      base::StringView name,
      const BuildId& build_id) const;

  // Marks a range of memory as containing jitted code.
  // If the added region overlaps with other existing ranges the latter are all
  // deleted.
  // Jitted ranges will only be applied to UserMemoryMappings
  void AddJitRange(UniquePid upid, AddressRange range, JitCache* jit_cache);

 private:
  template <typename MappingImpl>
  MappingImpl& AddMapping(std::unique_ptr<MappingImpl> mapping);

  TraceProcessorContext* const context_;
  base::FlatHashMap<MappingId, std::unique_ptr<VirtualMemoryMapping>>
      mappings_by_id_;

  base::FlatHashMap<CreateMappingParams,
                    VirtualMemoryMapping*,
                    CreateMappingParams::Hasher>
      interned_mappings_;

  struct NameAndBuildId {
    base::StringView name;
    std::optional<BuildId> build_id;

    bool operator==(const NameAndBuildId& o) const {
      return name == o.name && build_id == o.build_id;
    }

    bool operator!=(const NameAndBuildId& o) const { return !(*this == o); }

    struct Hasher {
      size_t operator()(const NameAndBuildId& o) const {
        base::Hasher hasher;
        hasher.Update(o.name);
        if (o.build_id) {
          hasher.Update(*o.build_id);
        }
        return static_cast<size_t>(hasher.digest());
      }
    };
  };
  base::FlatHashMap<NameAndBuildId,
                    std::vector<VirtualMemoryMapping*>,
                    NameAndBuildId::Hasher>
      mappings_by_name_and_build_id_;

  base::FlatHashMap<UniquePid, AddressRangeMap<UserMemoryMapping*>>
      user_memory_;
  AddressRangeMap<KernelMemoryMapping*> kernel_modules_;
  KernelMemoryMapping* kernel_ = nullptr;

  base::FlatHashMap<UniquePid, AddressRangeMap<JitCache*>> jit_caches_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_MAPPING_TRACKER_H_
