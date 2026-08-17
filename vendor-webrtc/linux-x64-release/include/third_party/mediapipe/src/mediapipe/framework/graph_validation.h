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
//
// Declares CalculatorGraph, which links Calculators into a directed acyclic
// graph, and allows its evaluation.

#ifndef MEDIAPIPE_FRAMEWORK_GRAPH_VALIDATION_H_
#define MEDIAPIPE_FRAMEWORK_GRAPH_VALIDATION_H_

#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {

// Validates a CalculatorGraphConfig, including subgraphs, template graphs,
// and side-packets.
class GraphValidation {
 public:
  // Validates the specified CalculatorGraphConfig.
  absl::Status Validate(
      const CalculatorGraphConfig& config,
      const std::map<std::string, Packet>& side_packets = {}) {
    return graph_.Initialize(config, side_packets);
  }

  // Validates the specified CalculatorGraphConfigs.
  // Template graph and subgraph configs can be specified through
  // |input_templates|.  Every subgraph must have its graph type specified in
  // CalclatorGraphConfig.type.  A subgraph can be validated directly by
  // specifying its type in |graph_type|.  A template graph can be validated
  // directly by specifying its template arguments in |arguments|.
  absl::Status Validate(const std::vector<CalculatorGraphConfig>& configs,
                        const std::vector<CalculatorGraphTemplate>& templates,
                        const std::map<std::string, Packet>& side_packets = {},
                        const std::string& graph_type = "",
                        const Subgraph::SubgraphOptions* options = nullptr) {
    return graph_.Initialize(configs, templates, side_packets, graph_type,
                             options);
  }

 private:
  CalculatorGraph graph_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_GRAPH_VALIDATION_H_
