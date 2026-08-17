// Copyright 2025 The gRPC authors.
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

#ifndef GRPC_TEST_CORE_TRANSPORT_CHAOTIC_GOOD_TEST_FRAME_H
#define GRPC_TEST_CORE_TRANSPORT_CHAOTIC_GOOD_TEST_FRAME_H

#include "fuzztest/fuzztest.h"
#include "src/core/ext/transport/chaotic_good/frame.h"
#include "test/core/transport/chaotic_good/test_frame.pb.h"

namespace grpc_core {
namespace chaotic_good {

Frame FrameFromTestFrame(const chaotic_good_frame::TestFrame& frame);

inline auto AnyFrame() {
  return ::fuzztest::Filter(
      [](const Frame& frame) {
        return absl::ConvertVariantTo<const FrameInterface&>(frame).IsLegal();
      },
      ::fuzztest::Map(FrameFromTestFrame,
                      ::fuzztest::Arbitrary<chaotic_good_frame::TestFrame>()));
}

}  // namespace chaotic_good
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TRANSPORT_CHAOTIC_GOOD_TEST_FRAME_H
