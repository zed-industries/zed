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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_GPU_EVENT_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_GPU_EVENT_PARSER_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

#include "perfetto/protozero/field.h"
#include "protos/perfetto/trace/gpu/gpu_render_stage_event.pbzero.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/importers/proto/vulkan_memory_tracker.h"
#include "src/trace_processor/storage/trace_storage.h"

#include "protos/perfetto/trace/gpu/vulkan_memory_event.pbzero.h"

namespace perfetto {

namespace protos::pbzero {
class GpuRenderStageEvent_Decoder;
}  // namespace protos::pbzero

namespace trace_processor {

class TraceProcessorContext;

struct ProtoEnumHasher {
  template <typename T>
  std::size_t operator()(T t) const {
    return static_cast<std::size_t>(t);
  }
};

// Class for parsing graphics related events.
class GpuEventParser {
 public:
  using ConstBytes = protozero::ConstBytes;
  using VulkanMemoryEventSource = protos::pbzero::VulkanMemoryEvent::Source;
  using VulkanMemoryEventOperation =
      protos::pbzero::VulkanMemoryEvent::Operation;
  explicit GpuEventParser(TraceProcessorContext*);

  void ParseGpuCounterEvent(int64_t ts, ConstBytes);
  void ParseGpuRenderStageEvent(int64_t ts,
                                PacketSequenceStateGeneration*,
                                ConstBytes);
  void ParseGraphicsFrameEvent(int64_t timestamp, ConstBytes);
  void ParseGpuLog(int64_t ts, ConstBytes);

  void ParseVulkanMemoryEvent(PacketSequenceStateGeneration*, ConstBytes);
  void UpdateVulkanMemoryAllocationCounters(
      UniquePid,
      const protos::pbzero::VulkanMemoryEvent::Decoder&);

  void ParseVulkanApiEvent(int64_t, ConstBytes);

  void ParseGpuMemTotalEvent(int64_t, ConstBytes);

 private:
  StringId GetFullStageName(
      PacketSequenceStateGeneration* sequence_state,
      const protos::pbzero::GpuRenderStageEvent_Decoder& event) const;
  void InsertTrackForUninternedRenderStage(
      uint32_t id,
      const protos::pbzero::GpuRenderStageEvent::Specifications::Description::
          Decoder&);
  std::optional<std::string> FindDebugName(int32_t vk_object_type,
                                           uint64_t vk_handle) const;
  StringId ParseRenderSubpasses(
      const protos::pbzero::GpuRenderStageEvent_Decoder& event) const;

  TraceProcessorContext* const context_;
  VulkanMemoryTracker vulkan_memory_tracker_;

  const StringId context_id_id_;
  const StringId render_target_id_;
  const StringId render_target_name_id_;
  const StringId render_pass_id_;
  const StringId render_pass_name_id_;
  const StringId render_subpasses_id_;
  const StringId command_buffer_id_;
  const StringId command_buffer_name_id_;
  const StringId frame_id_id_;
  const StringId submission_id_id_;
  const StringId hw_queue_id_id_;
  const StringId upid_id_;
  const StringId pid_id_;
  const StringId tid_id_;

  // For GpuCounterEvent
  std::unordered_map<uint32_t, TrackId> gpu_counter_track_ids_;

  // For GpuRenderStageEvent
  const StringId description_id_;
  std::vector<std::optional<TrackId>> gpu_hw_queue_ids_;

  // Map of stage ID -> pair(stage name, stage description)
  std::vector<std::pair<StringId, StringId>> gpu_render_stage_ids_;

  // For VulkanMemoryEvent
  std::unordered_map<protos::pbzero::VulkanMemoryEvent::AllocationScope,
                     int64_t /*counter_value*/,
                     ProtoEnumHasher>
      vulkan_driver_memory_counters_;
  std::unordered_map<uint32_t /*memory_type*/, int64_t /*counter_value*/>
      vulkan_device_memory_counters_allocate_;
  std::unordered_map<uint32_t /*memory_type*/, int64_t /*counter_value*/>
      vulkan_device_memory_counters_bind_;

  // For GpuLog
  const StringId tag_id_;
  const StringId log_message_id_;
  std::array<StringId, 7> log_severity_ids_;

  // For Vulkan events.
  // For VulkanApiEvent.VkDebugUtilsObjectName.
  // Map of vk handle -> vk object name.
  using DebugMarkerMap = std::unordered_map<uint64_t, std::string>;

  // Map of VkObjectType -> DebugMarkerMap.
  std::unordered_map<int32_t, DebugMarkerMap> debug_marker_names_;

  // For VulkanApiEvent.VkQueueSubmit.
  StringId vk_event_track_id_;
  StringId vk_queue_submit_id_;
};
}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_GPU_EVENT_PARSER_H_
