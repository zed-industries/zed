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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_VULKAN_MEMORY_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_VULKAN_MEMORY_TRACKER_H_

#include <cstdint>
#include <vector>

#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

#include "protos/perfetto/trace/gpu/vulkan_memory_event.pbzero.h"
#include "protos/perfetto/trace/profiling/profile_common.pbzero.h"

namespace perfetto::trace_processor {

class VulkanMemoryTracker {
 public:
  using VulkanMemoryEvent = protos::pbzero::VulkanMemoryEvent;

  explicit VulkanMemoryTracker(TraceProcessorContext* context);

  template <int32_t FieldId>
  StringId GetInternedString(PacketSequenceStateGeneration* state,
                             uint64_t iid) {
    auto* decoder =
        state->LookupInternedMessage<FieldId, protos::pbzero::InternedString>(
            iid);
    if (!decoder)
      return kNullStringId;
    return context_->storage->InternString(
        base::StringView(reinterpret_cast<const char*>(decoder->str().data),
                         decoder->str().size));
  }

  StringId FindSourceString(VulkanMemoryEvent::Source);
  StringId FindOperationString(VulkanMemoryEvent::Operation);
  StringId FindAllocationScopeString(VulkanMemoryEvent::AllocationScope);

 private:
  TraceProcessorContext* const context_;

  std::vector<StringId> source_strs_id_;
  std::vector<StringId> operation_strs_id_;
  std::vector<StringId> scope_strs_id_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_VULKAN_MEMORY_TRACKER_H_
