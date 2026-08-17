// Copyright 2023 gRPC authors.
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

#ifndef GRPC_TEST_CORE_TRANSPORT_CHAOTIC_GOOD_TRANSPORT_TEST_HELPER_H
#define GRPC_TEST_CORE_TRANSPORT_CHAOTIC_GOOD_TRANSPORT_TEST_HELPER_H

#include <google/protobuf/text_format.h>

#include "gmock/gmock.h"
#include "gtest/gtest.h"
#include "src/core/call/call_arena_allocator.h"
#include "src/core/call/call_spine.h"
#include "src/core/ext/transport/chaotic_good/frame.h"
#include "src/core/lib/event_engine/event_engine_context.h"
#include "src/core/lib/iomgr/timer_manager.h"
#include "src/core/lib/resource_quota/memory_quota.h"
#include "src/core/lib/resource_quota/resource_quota.h"
#include "test/core/event_engine/fuzzing_event_engine/fuzzing_event_engine.h"
#include "test/core/event_engine/fuzzing_event_engine/fuzzing_event_engine.pb.h"

namespace grpc_core {
namespace chaotic_good {
namespace testing {

grpc_event_engine::experimental::Slice SerializedFrameHeader(
    FrameType type, uint16_t payload_connection_id, uint32_t stream_id,
    uint32_t payload_length);

grpc_event_engine::experimental::Slice Zeros(uint32_t length);

template <typename T>
grpc_event_engine::experimental::Slice EncodeProto(const std::string& fields) {
  T msg;
  CHECK(google::protobuf::TextFormat::ParseFromString(fields, &msg));
  std::string out;
  CHECK(msg.SerializeToString(&out));
  return grpc_event_engine::experimental::Slice::FromCopiedString(out);
}

}  // namespace testing
}  // namespace chaotic_good
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TRANSPORT_CHAOTIC_GOOD_TRANSPORT_TEST_HELPER_H
