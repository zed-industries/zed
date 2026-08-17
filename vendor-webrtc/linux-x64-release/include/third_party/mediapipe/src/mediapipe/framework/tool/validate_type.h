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

// Helper functions for doing type validation on CalculatorGraphConfig.
#ifndef MEDIAPIPE_FRAMEWORK_TOOL_VALIDATE_TYPE_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_VALIDATE_TYPE_H_

#include <map>

#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {

namespace tool {
// Equivalent functions for PacketGenerators.
absl::Status RunGeneratorFillExpectations(
    const PacketGeneratorConfig& config,
    const std::string& package = "mediapipe");

// Run PacketGenerator::Generate() on the given generator, options,
// and inputs to produce outputs.  Validate the types of the inputs and
// outputs using PacketGenerator::FillExpectations.
absl::Status RunGenerateAndValidateTypes(
    const std::string& packet_generator_name,
    const PacketGeneratorOptions& extendable_options,
    const PacketSet& input_side_packets, PacketSet* output_side_packets,
    const std::string& package = "mediapipe");

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_VALIDATE_TYPE_H_
