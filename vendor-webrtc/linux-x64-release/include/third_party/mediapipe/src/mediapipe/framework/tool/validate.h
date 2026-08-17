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

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_VALIDATE_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_VALIDATE_H_

#include <string>

#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {

namespace tool {

// Returns absl::OkStatus() if the InputCollection is valid.  An input
// collection is invalid if it does not have the proper fields set
// depending on what its input_type field is.  Furthermore, if it uses
// INLINE, then the number of value fields in each inputs must match
// the number of input_side_packet_name fields.
absl::Status ValidateInput(const InputCollection& input);

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_VALIDATE_H_
