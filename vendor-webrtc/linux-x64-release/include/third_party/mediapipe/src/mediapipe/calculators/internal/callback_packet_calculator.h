// Copyright 2023 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_CALCULATORS_INTERNAL_CALLBACK_PACKET_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_INTERNAL_CALLBACK_PACKET_CALCULATOR_H_

#include "absl/status/status.h"
#include "mediapipe/framework/calculator_base.h"

namespace mediapipe {

// Creates a callback which takes a packet and stores it either in a
// vector of packets or stores only the packet at PostStream timestamp.
// The kind of callback is controlled by an option.  The callback is
// a std::function and is directly usable by CallbackCalculator.
// Since the options for the packet generator include a serialized pointer
// value, the resulting callback is only valid on the original machine
// while that pointer is still alive.
class CallbackPacketCalculator : public CalculatorBase {
 public:
  static absl::Status GetContract(CalculatorContract* cc);
  absl::Status Open(CalculatorContext* cc) override;
  absl::Status Process(CalculatorContext* cc) override;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_INTERNAL_CALLBACK_PACKET_CALCULATOR_H_
