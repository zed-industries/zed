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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PROFILE_PACKET_SEQUENCE_STATE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PROFILE_PACKET_SEQUENCE_STATE_H_

#include <cstdint>
#include "perfetto/base/flat_set.h"
#include "perfetto/ext/base/flat_hash_map.h"

#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/importers/proto/stack_profile_sequence_state.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto {
namespace trace_processor {

class VirtualMemoryMapping;

// Keeps sequence specific state for profile packets.
class ProfilePacketSequenceState final
    : public PacketSequenceStateGeneration::CustomState {
 public:
  using SourceStringId = uint64_t;

  struct SourceMapping {
    SourceStringId build_id = 0;
    uint64_t exact_offset = 0;
    uint64_t start_offset = 0;
    uint64_t start = 0;
    uint64_t end = 0;
    uint64_t load_bias = 0;
    std::vector<SourceStringId> name_ids;
  };
  using SourceMappingId = uint64_t;

  struct SourceFrame {
    SourceStringId name_id = 0;
    SourceMappingId mapping_id = 0;
    uint64_t rel_pc = 0;
  };
  using SourceFrameId = uint64_t;

  using SourceCallstack = std::vector<SourceFrameId>;
  using SourceCallstackId = uint64_t;
  struct SourceAllocation {
    uint64_t pid = 0;
    // This is int64_t, because we get this from the TraceSorter which also
    // converts this for us.
    int64_t timestamp = 0;
    StringId heap_name;
    uint64_t callstack_id = 0;
    uint64_t self_allocated = 0;
    uint64_t self_freed = 0;
    uint64_t alloc_count = 0;
    uint64_t free_count = 0;
  };

  explicit ProfilePacketSequenceState(TraceProcessorContext* context);
  virtual ~ProfilePacketSequenceState() override;

  // Profile packets keep track of a index to detect packet loss. Call this
  // method to update this index with the latest seen value.
  void SetProfilePacketIndex(uint64_t index);

  // In Android version Q we did not intern Mappings, Frames nor Callstacks,
  // instead the profile packed "interned these". The following methods are used
  // to support this old use case. They add the given object to a sequence local
  // index for them to be retrieved later (see Find* Lookup* methods).
  void AddString(SourceStringId id, base::StringView str);
  void AddMapping(SourceMappingId id, const SourceMapping& mapping);
  void AddFrame(SourceFrameId id, const SourceFrame& frame);
  void AddCallstack(SourceCallstackId id, const SourceCallstack& callstack);

  void StoreAllocation(const SourceAllocation& allocation);
  void FinalizeProfile();
  void CommitAllocations();

  FrameId GetDatabaseFrameIdForTesting(SourceFrameId);

 private:
  struct SourceAllocationIndex {
    UniquePid upid;
    SourceCallstackId src_callstack_id;
    StringPool::Id heap_name;
    bool operator==(const SourceAllocationIndex& o) const {
      return std::tie(upid, src_callstack_id, heap_name) ==
             std::tie(o.upid, o.src_callstack_id, o.heap_name);
    }
    struct Hasher {
      size_t operator()(const SourceAllocationIndex& o) const {
        return static_cast<size_t>(base::Hasher::Combine(
            o.upid, o.src_callstack_id, o.heap_name.raw_id()));
      }
    };
  };

  void AddAllocation(const SourceAllocation& alloc);

  // The following methods deal with interned data. In Android version Q we did
  // not intern Mappings, Frames nor Callstacks, instead the profile packed
  // "interned these" and this class keeps those ina  sequence local index. In
  // newer versions, these objects are in InternedData (see
  // protos/perfetto/trace/interned_data) and are shared across multiple
  // ProfilePackets. For backwards compatibility, the following methods first
  // look up interned data in the private sequence local index (for values added
  // via the Add* methods), and then, if this lookup fails, in the InternedData
  // instead.
  std::optional<MappingId> FindOrInsertMapping(uint64_t iid);
  std::optional<CallsiteId> FindOrInsertCallstack(UniquePid upid, uint64_t iid);

  TraceProcessorContext* const context_;

  base::FlatHashMap<SourceStringId, std::string> strings_;
  base::FlatHashMap<SourceMappingId, VirtualMemoryMapping*> mappings_;
  base::FlatHashMap<SourceFrameId, FrameId> frames_;
  base::FlatHashMap<SourceCallstackId, CallsiteId> callstacks_;

  std::vector<SourceAllocation> pending_allocs_;

  struct Hasher {
    size_t operator()(const std::pair<UniquePid, CallsiteId>& p) const {
      return static_cast<size_t>(
          base::Hasher::Combine(p.first, p.second.value));
    }
  };
  base::FlatHashMap<std::pair<UniquePid, CallsiteId>,
                    tables::HeapProfileAllocationTable::Row,
                    Hasher>
      prev_alloc_;
  base::FlatHashMap<std::pair<UniquePid, CallsiteId>,
                    tables::HeapProfileAllocationTable::Row,
                    Hasher>
      prev_free_;

  // For continuous dumps, we only store the delta in the data-base. To do
  // this, we subtract the previous dump's value. Sometimes, we should not
  // do that subtraction, because heapprofd garbage collects stacks that
  // have no unfreed allocations. If the application then allocations again
  // at that stack, it gets recreated and initialized to zero.
  //
  // To correct for this, we add the previous' stacks value to the current
  // one, and then handle it as normal. If it is the first time we see a
  // SourceCallstackId for a CallsiteId, we put the previous value into
  // the correction maps below.
  base::FlatHashMap<SourceAllocationIndex,
                    base::FlatSet<CallsiteId>,
                    SourceAllocationIndex::Hasher>
      seen_callstacks_;
  base::FlatHashMap<SourceCallstackId, tables::HeapProfileAllocationTable::Row>
      alloc_correction_;
  base::FlatHashMap<SourceCallstackId, tables::HeapProfileAllocationTable::Row>
      free_correction_;

  std::optional<uint64_t> prev_index;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PROFILE_PACKET_SEQUENCE_STATE_H_
