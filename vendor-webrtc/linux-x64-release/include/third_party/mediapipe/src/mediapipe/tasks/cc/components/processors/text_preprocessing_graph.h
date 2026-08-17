/* Copyright 2022 The MediaPipe Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef MEDIAPIPE_TASKS_CC_COMPONENTS_PROCESSORS_TEXT_PREPROCESSING_GRAPH_H_
#define MEDIAPIPE_TASKS_CC_COMPONENTS_PROCESSORS_TEXT_PREPROCESSING_GRAPH_H_

#include "absl/status/status.h"
#include "mediapipe/tasks/cc/components/processors/proto/text_preprocessing_graph_options.pb.h"
#include "mediapipe/tasks/cc/core/model_resources.h"

namespace mediapipe {
namespace tasks {
namespace components {
namespace processors {

// Configures a TextPreprocessingGraph using the provided `model_resources`
// and TextPreprocessingGraphOptions.
// - Accepts a std::string input and outputs CPU tensors.
//
// Example usage:
//
//   auto& preprocessing =
//       graph.AddNode("mediapipe.tasks.components.processors.TextPreprocessingSubgraph");
//   MP_RETURN_IF_ERROR(ConfigureTextPreprocessingSubgraph(
//       model_resources,
//       &preprocessing.GetOptions<TextPreprocessingGraphOptions>()));
//
// The resulting TextPreprocessingGraph has the following I/O:
// Inputs:
//   TEXT - std::string
//     The text to preprocess.
// Side inputs:
//   METADATA_EXTRACTOR - ModelMetadataExtractor
//     The metadata extractor for the TFLite model. Used to determine the order
//     for input tensors and to extract tokenizer information.
// Outputs:
//   TENSORS - std::vector<Tensor>
//     Vector containing the preprocessed input tensors for the TFLite model.
absl::Status ConfigureTextPreprocessingGraph(
    const core::ModelResources& model_resources,
    proto::TextPreprocessingGraphOptions& options);

}  // namespace processors
}  // namespace components
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_COMPONENTS_PROCESSORS_TEXT_PREPROCESSING_GRAPH_H_
