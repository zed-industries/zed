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

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_PACKET_GENERATOR_WRAPPER_CALCULATOR_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_PACKET_GENERATOR_WRAPPER_CALCULATOR_H_

#include "absl/status/status.h"
#include "mediapipe/framework/calculator_base.h"

namespace mediapipe {

class PacketGeneratorWrapperCalculator : public CalculatorBase {
 public:
  static absl::Status GetContract(CalculatorContract* cc);
  absl::Status Open(CalculatorContext* cc) override;
  absl::Status Process(CalculatorContext* cc) override;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_PACKET_GENERATOR_WRAPPER_CALCULATOR_H_
