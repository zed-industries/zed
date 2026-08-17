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

#ifndef MEDIAPIPE_FRAMEWORK_TOOL_NAME_UTIL_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_NAME_UTIL_H_

#include <string>

#include "mediapipe/framework/calculator.pb.h"

namespace mediapipe {

namespace tool {
// Get an unused InputSidePacket name which is (or starts with)
// input_side_packet_name_base.
std::string GetUnusedSidePacketName(const CalculatorGraphConfig& /*config*/,
                                    const std::string& side_packet_name_base);

// Get an usused node name which is (or starts with) node_name_base.
std::string GetUnusedNodeName(const CalculatorGraphConfig& config,
                              const std::string& node_name_base);

// Returns a short unique name for a Node in a CalculatorGraphConfig.
// This is the Node.name (if specified) or the Node.calculator.
// If there are multiple calculators with similar name in the graph, the name
// will be postfixed by "_<COUNT>". For example, in the following graph the node
// names will be as mentiond.
//
// node { // Name will be "CalcA"
//   calculator: "CalcA"
// }
// node { // Name will be "NameB"
//   calculator: "CalcB"
//   name: "NameB"
// }
// node { // Name will be "CalcC_1" due to duplicate "calculator" field.
//   calculator: "CalcC"
// }
// node { // Name will be "CalcC_2" due to duplicate "calculator" field.
//   calculator: "CalcC"
// }
// node { // Name will be "NameX".
//   calculator: "CalcD"
//   name: "NameX"
// }
// node { // Name will be "NameY".
//   calculator: "CalcD"
//   name: "NameY"
// }
// node { // Name will be "NameZ_1". due to "name" field duplicate.
//   calculator: "CalcE"
//   name: "NameZ"
// }
// node { // Name will be "NameZ_2". due to "name" field duplicate.
//   calculator: "CalcF"
//   name: "NameZ"
// }
//
// TODO: Update GraphNode.UniqueName in MediaPipe Visualizer to match
// this logic.
// TODO: Fix the edge case mentioned in the bug.
std::string CanonicalNodeName(const CalculatorGraphConfig& graph_config,
                              int node_id);

// Parses the name from a "tag:index:name".
std::string ParseNameFromStream(const std::string& stream);

// Parses the TagIndex from a "tag:index".
std::pair<std::string, int> ParseTagIndex(const std::string& tag_index);

// Parses the TagIndex from a "tag:index:name".
std::pair<std::string, int> ParseTagIndexFromStream(const std::string& stream);

// Formats to "tag:index".
std::string CatTag(const std::string& tag, int index);

// Concatenates "tag:index:name" into a single string.
std::string CatStream(const std::pair<std::string, int>& tag_index,
                      const std::string& name);

}  // namespace tool
}  // namespace mediapipe

namespace mediapipe {
using mediapipe::tool::CanonicalNodeName;
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_NAME_UTIL_H_
