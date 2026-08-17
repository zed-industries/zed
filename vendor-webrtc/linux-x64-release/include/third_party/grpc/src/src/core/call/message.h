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

#ifndef GRPC_SRC_CORE_CALL_MESSAGE_H
#define GRPC_SRC_CORE_CALL_MESSAGE_H

#include <grpc/support/port_platform.h>

#include "src/core/lib/resource_quota/arena.h"
#include "src/core/lib/slice/slice_buffer.h"

/// Internal bit flag for grpc_begin_message's \a flags signaling the use of
/// compression for the message. (Does not apply for stream compression.)
#define GRPC_WRITE_INTERNAL_COMPRESS (0x80000000u)
/// Internal bit flag for determining whether the message was compressed and had
/// to be decompressed by the message_decompress filter. (Does not apply for
/// stream compression.)
#define GRPC_WRITE_INTERNAL_TEST_ONLY_WAS_COMPRESSED (0x40000000u)
/// Mask of all valid internal flags.
#define GRPC_WRITE_INTERNAL_USED_MASK \
  (GRPC_WRITE_INTERNAL_COMPRESS | GRPC_WRITE_INTERNAL_TEST_ONLY_WAS_COMPRESSED)

namespace grpc_core {

class Message {
 public:
  Message() = default;
  ~Message() = default;
  Message(SliceBuffer payload, uint32_t flags)
      : payload_(std::move(payload)), flags_(flags) {}
  Message(const Message&) = delete;
  Message& operator=(const Message&) = delete;

  uint32_t flags() const { return flags_; }
  uint32_t& mutable_flags() { return flags_; }
  SliceBuffer* payload() { return &payload_; }
  const SliceBuffer* payload() const { return &payload_; }

  Arena::PoolPtr<Message> Clone() const {
    return Arena::MakePooled<Message>(payload_.Copy(), flags_);
  }

  std::string DebugString() const;

  template <typename Sink>
  friend void AbslStringify(Sink& sink, const Message& message) {
    sink.Append(message.DebugString());
  }

 private:
  SliceBuffer payload_;
  uint32_t flags_ = 0;
};

using MessageHandle = Arena::PoolPtr<Message>;

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_CALL_MESSAGE_H
