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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_STACK_PROFILE_SEQUENCE_STATE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_STACK_PROFILE_SEQUENCE_STATE_H_

#include <cstdint>
#include <optional>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

class TraceProcessorContext;
class VirtualMemoryMapping;

class StackProfileSequenceState final
    : public PacketSequenceStateGeneration::CustomState {
 public:
  explicit StackProfileSequenceState(TraceProcessorContext* context);

  StackProfileSequenceState(const StackProfileSequenceState&);

  virtual ~StackProfileSequenceState() override;

  // Returns `nullptr`if non could be found.
  VirtualMemoryMapping* FindOrInsertMapping(uint64_t iid);
  std::optional<CallsiteId> FindOrInsertCallstack(UniquePid upid, uint64_t iid);

 private:
  std::optional<base::StringView> LookupInternedBuildId(uint64_t iid);
  std::optional<base::StringView> LookupInternedMappingPath(uint64_t iid);
  std::optional<base::StringView> LookupInternedFunctionName(uint64_t iid);

  // Returns `nullptr`if non could be found.
  VirtualMemoryMapping* FindOrInsertMappingImpl(std::optional<UniquePid> upid,
                                                uint64_t iid);
  std::optional<FrameId> FindOrInsertFrame(UniquePid upid, uint64_t iid);

  TraceProcessorContext* const context_;

  struct OptionalUniquePidAndIid {
    std::optional<UniquePid> upid;
    uint64_t iid;

    bool operator==(const OptionalUniquePidAndIid& o) const {
      return upid == o.upid && iid == o.iid;
    }

    struct Hasher {
      size_t operator()(const OptionalUniquePidAndIid& o) const {
        base::Hasher h;
        h.Update(o.iid);
        if (o.upid) {
          h.Update(*o.upid);
        }
        return static_cast<size_t>(h.digest());
      }
    };
  };

  struct UniquePidAndIid {
    UniquePid upid;
    uint64_t iid;

    bool operator==(const UniquePidAndIid& o) const {
      return upid == o.upid && iid == o.iid;
    }

    struct Hasher {
      size_t operator()(const UniquePidAndIid& o) const {
        return static_cast<size_t>(base::Hasher::Combine(o.upid, o.iid));
      }
    };
  };
  base::FlatHashMap<OptionalUniquePidAndIid,
                    VirtualMemoryMapping*,
                    OptionalUniquePidAndIid::Hasher>
      cached_mappings_;
  base::FlatHashMap<UniquePidAndIid, FrameId, UniquePidAndIid::Hasher>
      cached_frames_;
  base::FlatHashMap<UniquePidAndIid, CallsiteId, UniquePidAndIid::Hasher>
      cached_callstacks_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_STACK_PROFILE_SEQUENCE_STATE_H_
