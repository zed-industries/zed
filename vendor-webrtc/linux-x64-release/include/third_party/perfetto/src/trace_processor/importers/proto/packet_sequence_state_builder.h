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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PACKET_SEQUENCE_STATE_BUILDER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PACKET_SEQUENCE_STATE_BUILDER_H_

#include <cstdint>

#include "perfetto/trace_processor/ref_counted.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

// Helper class to generate a stream of PacketSequenceStateGeneration as we
// receive packets for a sequence. This class deals with various events that
// incrementally build up state that can be accessed by packet handling code
// (tokenization nad parsing). An example of such state are interned messages or
// trace packet defaults.
class PacketSequenceStateBuilder {
 public:
  explicit PacketSequenceStateBuilder(TraceProcessorContext* context) {
    generation_ = PacketSequenceStateGeneration::CreateFirst(context);
  }

  // Intern a message into the current generation.
  void InternMessage(uint32_t field_id, TraceBlobView message) {
    generation_->InternMessage(field_id, std::move(message));
  }

  // Set the trace packet defaults for the current generation. If the current
  // generation already has defaults set, starts a new generation without
  // invalidating other incremental state (such as interned data).
  void UpdateTracePacketDefaults(TraceBlobView defaults) {
    generation_ = generation_->OnNewTracePacketDefaults(std::move(defaults));
  }

  void OnPacketLoss() {
    generation_ = generation_->OnPacketLoss();
    packet_loss_ = true;
  }

  // Starts a new generation with clean-slate incremental state and defaults.
  void OnIncrementalStateCleared() {
    packet_loss_ = false;
    generation_ = generation_->OnIncrementalStateCleared();
  }

  bool IsIncrementalStateValid() const { return !packet_loss_; }

  // Returns a ref-counted ptr to the current generation.
  RefPtr<PacketSequenceStateGeneration> current_generation() const {
    return generation_;
  }

 private:
  // If true, incremental state on the sequence is considered invalid until we
  // see the next packet with incremental_state_cleared. We assume that we
  // missed some packets at the beginning of the trace.
  bool packet_loss_ = true;

  RefPtr<PacketSequenceStateGeneration> generation_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PACKET_SEQUENCE_STATE_BUILDER_H_
