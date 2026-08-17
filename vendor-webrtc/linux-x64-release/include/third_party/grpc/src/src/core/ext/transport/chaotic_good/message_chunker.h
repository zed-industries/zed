// Copyright 2024 gRPC authors.
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

#ifndef GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_MESSAGE_CHUNKER_H
#define GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_MESSAGE_CHUNKER_H

#include <cstdint>

#include "src/core/ext/transport/chaotic_good/frame.h"
#include "src/core/ext/transport/chaotic_good/frame_transport.h"
#include "src/core/lib/promise/if.h"
#include "src/core/lib/promise/loop.h"
#include "src/core/lib/promise/map.h"
#include "src/core/lib/promise/seq.h"
#include "src/core/lib/promise/status_flag.h"

namespace grpc_core {
namespace chaotic_good {

namespace message_chunker_detail {
struct ChunkResult {
  MessageChunkFrame frame;
  bool done;
};

// Given a set of parameters defining the desired size, produce frames of chunks
// of payload to send one at a time.
class PayloadChunker {
 public:
  PayloadChunker(uint32_t max_chunk_size, uint32_t alignment,
                 uint32_t stream_id, SliceBuffer payload)
      : max_chunk_size_(max_chunk_size),
        alignment_(alignment),
        stream_id_(stream_id),
        payload_(std::move(payload)) {}
  PayloadChunker(const PayloadChunker&) = delete;
  PayloadChunker(PayloadChunker&&) = default;
  PayloadChunker& operator=(const PayloadChunker&) = delete;
  PayloadChunker& operator=(PayloadChunker&&) = delete;

  ChunkResult NextChunk() {
    auto remaining = payload_.Length();
    ChunkResult result;
    if (remaining > max_chunk_size_) {
      auto take = max_chunk_size_;
      if (remaining / 2 < max_chunk_size_) {
        // If there's very little remaining, then we make the last two chunks
        // about equal sized to make later load balancing a little easier to
        // reason about (nobody likes a teeny tiny straggler).
        take = remaining / 2;
        // But we try to keep alignment rounded lengths for the first chunk.
        // This way we don't accidentally split things up such that padding is
        // needed (and potentially then copying elsewhere).
        if (alignment_ != 0 && take % alignment_ != 0) {
          take += alignment_ - (take % alignment_);
          if (take > max_chunk_size_) take = max_chunk_size_;
        }
      }
      payload_.MoveFirstNBytesIntoSliceBuffer(take, result.frame.payload);
      result.frame.stream_id = stream_id_;
      result.done = false;
    } else {
      result.frame.payload = std::move(payload_);
      result.frame.stream_id = stream_id_;
      result.done = true;
    }
    return result;
  }

 private:
  uint32_t max_chunk_size_;
  uint32_t alignment_;
  uint32_t stream_id_;
  SliceBuffer payload_;
};
}  // namespace message_chunker_detail

// Helper to send message payloads (possibly chunked!) between client & server.
class MessageChunker {
 public:
  MessageChunker(uint32_t max_chunk_size, uint32_t alignment)
      : max_chunk_size_(max_chunk_size), alignment_(alignment) {}

  template <typename Output>
  auto Send(MessageHandle message, uint32_t stream_id,
            std::shared_ptr<TcpCallTracer> call_tracer, Output& output) {
    return If(
        ShouldChunk(*message),
        [&]() {
          BeginMessageFrame begin;
          begin.body.set_length(message->payload()->Length());
          begin.stream_id = stream_id;
          return Seq(
              output.Send(OutgoingFrame{std::move(begin), call_tracer}),
              Loop([chunker = message_chunker_detail::PayloadChunker(
                        max_chunk_size_, alignment_, stream_id,
                        std::move(*message->payload())),
                    &output, call_tracer = std::move(call_tracer)]() mutable {
                auto next = chunker.NextChunk();
                return Map(
                    output.Send(
                        OutgoingFrame{std::move(next.frame), call_tracer}),
                    [done = next.done](StatusFlag x) -> LoopCtl<StatusFlag> {
                      if (!done) return Continue{};
                      return x;
                    });
              }));
        },
        [&]() {
          MessageFrame frame;
          frame.message = std::move(message);
          frame.stream_id = stream_id;
          return output.Send(OutgoingFrame{std::move(frame), nullptr});
        });
  }

  uint32_t max_chunk_size() const { return max_chunk_size_; }
  uint32_t alignment() const { return alignment_; }

 private:
  bool ShouldChunk(Message& message) {
    return max_chunk_size_ != 0 &&
           message.payload()->Length() > max_chunk_size_;
  }

  const uint32_t max_chunk_size_;
  const uint32_t alignment_;
};

}  // namespace chaotic_good
}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_EXT_TRANSPORT_CHAOTIC_GOOD_MESSAGE_CHUNKER_H
