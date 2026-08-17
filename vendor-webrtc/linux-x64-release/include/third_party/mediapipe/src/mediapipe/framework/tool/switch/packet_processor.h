// Copyright 2022 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_PACKET_PROCESSOR_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_PACKET_PROCESSOR_H_

#include <memory>

#include "mediapipe/framework/collection_item_id.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {

// PacketConsumer accepts several tagged streams of packets.
class PacketConsumer {
 public:
  virtual ~PacketConsumer() = default;

  // Accepts a tagged input packet.
  virtual absl::Status AddPacket(CollectionItemId id, Packet packet) = 0;

  // Returns the id for each input tag.
  virtual std::shared_ptr<tool::TagMap> InputTags() = 0;
};

// PacketConsumer delivers several tagged streams of packets.
class PacketProducer {
 public:
  virtual ~PacketProducer() = default;

  // Connects a consumer to recieve packets from this producer.
  virtual void SetConsumer(PacketConsumer* consumer) = 0;
};

// SidePacketConsumer accepts several tagged constant packets.
class SidePacketConsumer {
 public:
  virtual ~SidePacketConsumer() = default;

  // Accepts a tagged input side-packet.
  virtual absl::Status SetSidePacket(CollectionItemId id, Packet packet) = 0;

  // Returns the id for each input side-packet tag.
  virtual std::shared_ptr<tool::TagMap> SideInputTags() = 0;
};

// SidePacketProducer delivers several tagged constant packets.
class SidePacketProducer {
 public:
  virtual ~SidePacketProducer() = default;

  // Connects a consumer to recieve packets from this producer.
  virtual void SetSideConsumer(SidePacketConsumer* consumer) = 0;
};

// PacketProcessor consumes and produces packet streams and constant packets.
class PacketProcessor : public PacketConsumer,
                        public PacketProducer,
                        public SidePacketConsumer,
                        public SidePacketProducer {
 public:
  virtual ~PacketProcessor() = default;

  // Activate this PacketProcessor.
  virtual absl::Status Start() = 0;

  // Block until this PacketProcessor has no remaining work to do.
  virtual absl::Status WaitUntilIdle() = 0;

  // Deactivate this PacketProcessor.
  virtual absl::Status Shutdown() = 0;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_PACKET_PROCESSOR_H_
