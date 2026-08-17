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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_TRACK_EVENT_TOKENIZER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_TRACK_EVENT_TOKENIZER_H_

#include <array>
#include <cstddef>
#include <cstdint>

#include "perfetto/base/status.h"
#include "perfetto/protozero/proto_decoder.h"
#include "perfetto/trace_processor/ref_counted.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/importers/proto/proto_importer_module.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace perfetto {

namespace protos::pbzero {
class ChromeThreadDescriptor_Decoder;
class ProcessDescriptor_Decoder;
class ThreadDescriptor_Decoder;
class TracePacket_Decoder;
class TrackEvent_Decoder;
class TrackEvent_LegacyEvent_Decoder;
}  // namespace protos::pbzero

namespace trace_processor {

class TraceProcessorContext;
class TraceBlobView;
class TrackEventTracker;
struct TrackEventData;

class TrackEventTokenizer {
 public:
  explicit TrackEventTokenizer(TraceProcessorContext*, TrackEventTracker*);

  ModuleResult TokenizeRangeOfInterestPacket(
      RefPtr<PacketSequenceStateGeneration> state,
      const protos::pbzero::TracePacket_Decoder&,
      int64_t packet_timestamp);
  ModuleResult TokenizeTrackDescriptorPacket(
      RefPtr<PacketSequenceStateGeneration> state,
      const protos::pbzero::TracePacket_Decoder&,
      int64_t packet_timestamp);
  ModuleResult TokenizeThreadDescriptorPacket(
      RefPtr<PacketSequenceStateGeneration> state,
      const protos::pbzero::TracePacket_Decoder&);
  ModuleResult TokenizeTrackEventPacket(
      RefPtr<PacketSequenceStateGeneration> state,
      const protos::pbzero::TracePacket_Decoder&,
      TraceBlobView* packet,
      int64_t packet_timestamp);

 private:
  void TokenizeThreadDescriptor(
      PacketSequenceStateGeneration& state,
      const protos::pbzero::ThreadDescriptor_Decoder&);
  template <typename T>
  base::Status AddExtraCounterValues(
      TrackEventData& data,
      size_t& index,
      uint32_t trusted_packet_sequence_id,
      protozero::RepeatedFieldIterator<T> value_it,
      protozero::RepeatedFieldIterator<uint64_t> packet_track_uuid_it,
      protozero::RepeatedFieldIterator<uint64_t> default_track_uuid_it);
  base::Status TokenizeLegacySampleEvent(
      const protos::pbzero::TrackEvent_Decoder&,
      const protos::pbzero::TrackEvent_LegacyEvent_Decoder&,
      PacketSequenceStateGeneration& state);

  TraceProcessorContext* const context_;
  TrackEventTracker* const track_event_tracker_;

  const StringId counter_name_thread_time_id_;
  const StringId counter_name_thread_instruction_count_id_;

  std::array<StringId, 4> counter_unit_ids_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_TRACK_EVENT_TOKENIZER_H_
