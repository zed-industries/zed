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

#ifndef MEDIAPIPE_UTIL_GRAPH_BUILDER_UTILS_H_
#define MEDIAPIPE_UTIL_GRAPH_BUILDER_UTILS_H_

#include <string>

#include "absl/strings/string_view.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/calculator_framework.h"

namespace mediapipe {

// Checks if @node has input with the specified @tag.
bool HasInput(const CalculatorGraphConfig::Node& node, absl::string_view tag);

// Checks if @node has input side-packet with the specified @tag.
bool HasSideInput(const CalculatorGraphConfig::Node& node,
                  absl::string_view tag);

// Checks if @node has output with the specified @tag.
bool HasOutput(const CalculatorGraphConfig::Node& node, absl::string_view tag);

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_GRAPH_BUILDER_UTILS_H_
