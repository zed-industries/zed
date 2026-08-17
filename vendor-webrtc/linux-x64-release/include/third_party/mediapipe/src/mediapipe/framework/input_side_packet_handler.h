// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_FRAMEWORK_INPUT_SIDE_PACKET_HANDLER_H_
#define MEDIAPIPE_FRAMEWORK_INPUT_SIDE_PACKET_HANDLER_H_

#include <atomic>
#include <functional>
#include <map>
#include <memory>
#include <string>

#include "mediapipe/framework/collection_item_id.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {

// The input side packet handler is invoked every time any of the input side
// packets is set and notifies the calculator node when all the input side
// packets become available. The input side packet handler owns and manages
// the input side packets. OutputSidePacket sets an input side packet through
// its input side packet handler.
class InputSidePacketHandler {
 public:
  InputSidePacketHandler() = default;
  ~InputSidePacketHandler() = default;

  // Resets the input side packet handler and its underlying input side packets
  // for another run of the graph.
  absl::Status PrepareForRun(
      const PacketTypeSet* input_side_packet_types,
      const std::map<std::string, Packet>& all_side_packets,
      std::function<void()> input_side_packets_ready_callback,
      std::function<void(absl::Status)> error_callback);

  // Sets a particular input side packet.
  void Set(CollectionItemId id, const Packet& packet);

  const PacketSet& InputSidePackets() const { return *input_side_packets_; }

  // Returns true if the set of input-side-packets has changed since the
  // previous run.
  bool InputSidePacketsChanged();

  // Returns the number of missing input side packets.
  int MissingInputSidePacketCount() const {
    return missing_input_side_packet_count_.load(std::memory_order_relaxed);
  }

 private:
  // Called by Set().
  absl::Status SetInternal(CollectionItemId id, const Packet& packet);

  // Triggers the error callback with absl::Status info when an error
  // occurs.
  void TriggerErrorCallback(const absl::Status& status) const;

  const PacketTypeSet* input_side_packet_types_;

  std::unique_ptr<PacketSet> input_side_packets_;
  std::unique_ptr<PacketSet> prev_input_side_packets_;

  std::atomic<int> missing_input_side_packet_count_{0};

  std::function<void()> input_side_packets_ready_callback_;
  std::function<void(absl::Status)> error_callback_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_INPUT_SIDE_PACKET_HANDLER_H_
