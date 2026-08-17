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

#ifndef MEDIAPIPE_FRAMEWORK_OUTPUT_SIDE_PACKET_H_
#define MEDIAPIPE_FRAMEWORK_OUTPUT_SIDE_PACKET_H_

#include "mediapipe/framework/packet.h"

namespace mediapipe {

// The OutputSidePacket base class defines the output side packet interface
// exposed to calculators in the CalculatorContext. The framework actually
// creates instances of the OutputSidePacketImpl subclass, which has other
// methods used by the framework.
class OutputSidePacket {
 public:
  OutputSidePacket() = default;
  virtual ~OutputSidePacket() = default;

  // Sets the output side packet. The Packet must contain the data.
  //
  // NOTE: Set() cannot report errors via the return value. It uses an error
  // callback function to report errors.
  virtual void Set(const Packet& packet) = 0;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_OUTPUT_SIDE_PACKET_H_
