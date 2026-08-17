// Copyright 2022 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_LEGACY_FRAME_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_LEGACY_FRAME_H

#include <grpc/support/port_platform.h>

#include <cstdint>
#include <string>
#include <variant>

#include "absl/random/bit_gen_ref.h"
#include "absl/status/status.h"
#include "src/core/call/message.h"
#include "src/core/call/metadata.h"
#include "src/core/ext/transport/chaotic_good/chaotic_good_frame.pb.h"
#include "src/core/ext/transport/chaotic_good_legacy/frame_header.h"
#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/slice/slice_buffer.h"
#include "src/core/util/match.h"

namespace grpc_core {
namespace chaotic_good_legacy {

class FrameInterface {
 public:
  virtual absl::Status Deserialize(const FrameHeader& header,
                                   SliceBuffer payload) = 0;
  virtual FrameHeader MakeHeader() const = 0;
  virtual void SerializePayload(SliceBuffer& payload) const = 0;
  virtual std::string ToString() const = 0;

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const FrameInterface& frame) {
    sink.Append(frame.ToString());
  }

 protected:
  ~FrameInterface() = default;
};

inline std::ostream& operator<<(std::ostream& os, const FrameInterface& frame) {
  return os << frame.ToString();
}

chaotic_good_frame::ClientMetadata ClientMetadataProtoFromGrpc(
    const ClientMetadata& md);
absl::StatusOr<ClientMetadataHandle> ClientMetadataGrpcFromProto(
    chaotic_good_frame::ClientMetadata& metadata);
chaotic_good_frame::ServerMetadata ServerMetadataProtoFromGrpc(
    const ServerMetadata& md);
absl::StatusOr<ServerMetadataHandle> ServerMetadataGrpcFromProto(
    chaotic_good_frame::ServerMetadata& metadata);

absl::Status ReadProto(SliceBuffer payload, google::protobuf::MessageLite& msg);
void WriteProto(const google::protobuf::MessageLite& msg, SliceBuffer& output);
uint32_t ProtoPayloadSize(const google::protobuf::MessageLite& msg);
absl::Status ReadTransportProto(const FrameHeader& header, SliceBuffer payload,
                                google::protobuf::MessageLite& body);
absl::Status ReadStreamProto(const FrameHeader& header, SliceBuffer payload,
                             google::protobuf::MessageLite& body,
                             uint32_t& stream_id);
absl::Status ReadEmptyFrame(const FrameHeader& header, uint32_t& stream_id);

// Generic implementation of a transport-bound frame (stream_id is zero always)
// with a protobuf specified body.

template <FrameType frame_type, typename Body>
struct ProtoTransportFrame final : public FrameInterface {
  absl::Status Deserialize(const FrameHeader& header,
                           SliceBuffer payload) override {
    DCHECK_EQ(header.type, frame_type);
    return ReadTransportProto(header, std::move(payload), body);
  }
  FrameHeader MakeHeader() const override {
    return FrameHeader{frame_type, 0, 0, ProtoPayloadSize(body)};
  }
  void SerializePayload(SliceBuffer& payload) const override {
    WriteProto(body, payload);
  }
  std::string ToString() const override {
    return absl::StrCat(FrameTypeString(frame_type), "{",
                        body.ShortDebugString(), "}");
  }

  Body body;
};

// Generic implementation of a stream-bound frame with a protobuf specified
// body.
template <FrameType frame_type, typename Body>
struct ProtoStreamFrame final : public FrameInterface {
  absl::Status Deserialize(const FrameHeader& header,
                           SliceBuffer payload) override {
    DCHECK_EQ(header.type, frame_type);
    return ReadStreamProto(header, std::move(payload), body, stream_id);
  }
  FrameHeader MakeHeader() const override {
    return FrameHeader{frame_type, 0, stream_id, ProtoPayloadSize(body)};
  }
  void SerializePayload(SliceBuffer& payload) const override {
    DCHECK_NE(stream_id, 0u);
    WriteProto(body, payload);
  }
  std::string ToString() const override {
    return absl::StrCat(FrameTypeString(frame_type), "{@", stream_id, "; ",
                        body.ShortDebugString(), "}");
  }

  Body body;
  uint32_t stream_id;
};

// Generic implementation of an empty stream frame.
template <FrameType frame_type>
struct EmptyStreamFrame final : public FrameInterface {
  EmptyStreamFrame() = default;
  explicit EmptyStreamFrame(uint32_t stream_id) : stream_id(stream_id) {}
  absl::Status Deserialize(const FrameHeader& header, SliceBuffer) override {
    DCHECK_EQ(header.type, frame_type);
    return ReadEmptyFrame(header, stream_id);
  }
  FrameHeader MakeHeader() const override {
    return FrameHeader{frame_type, 0, stream_id, 0};
  }
  void SerializePayload(SliceBuffer&) const override {}
  std::string ToString() const override { return FrameTypeString(frame_type); }

  uint32_t stream_id;
};

using SettingsFrame =
    ProtoTransportFrame<FrameType::kSettings, chaotic_good_frame::Settings>;
using ClientInitialMetadataFrame =
    ProtoStreamFrame<FrameType::kClientInitialMetadata,
                     chaotic_good_frame::ClientMetadata>;
using BeginMessageFrame = ProtoStreamFrame<FrameType::kBeginMessage,
                                           chaotic_good_frame::BeginMessage>;
using ClientEndOfStream = EmptyStreamFrame<FrameType::kClientEndOfStream>;
using ServerInitialMetadataFrame =
    ProtoStreamFrame<FrameType::kServerInitialMetadata,
                     chaotic_good_frame::ServerMetadata>;
using ServerTrailingMetadataFrame =
    ProtoStreamFrame<FrameType::kServerTrailingMetadata,
                     chaotic_good_frame::ServerMetadata>;
using CancelFrame = EmptyStreamFrame<FrameType::kCancel>;

struct MessageFrame final : public FrameInterface {
  absl::Status Deserialize(const FrameHeader& header,
                           SliceBuffer payload) override;
  FrameHeader MakeHeader() const override;
  void SerializePayload(SliceBuffer& payload) const override;
  std::string ToString() const override;

  uint32_t stream_id;
  MessageHandle message;
};

struct MessageChunkFrame final : public FrameInterface {
  absl::Status Deserialize(const FrameHeader& header,
                           SliceBuffer payload) override;
  FrameHeader MakeHeader() const override;
  void SerializePayload(SliceBuffer& payload) const override;
  std::string ToString() const override;

  uint32_t stream_id;
  SliceBuffer payload;
};

using ClientFrame =
    std::variant<ClientInitialMetadataFrame, MessageFrame, BeginMessageFrame,
                 MessageChunkFrame, ClientEndOfStream, CancelFrame>;
using ServerFrame =
    std::variant<ServerInitialMetadataFrame, MessageFrame, BeginMessageFrame,
                 MessageChunkFrame, ServerTrailingMetadataFrame>;

}  // namespace chaotic_good_legacy
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_LEGACY_FRAME_H
