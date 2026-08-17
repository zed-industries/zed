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

#ifndef MEDIAPIPE_TASKS_CC_COMPONENTS_PROCESSORS_CLASSIFICATION_POSTPROCESSING_GRAPH_H_
#define MEDIAPIPE_TASKS_CC_COMPONENTS_PROCESSORS_CLASSIFICATION_POSTPROCESSING_GRAPH_H_

#include "absl/status/status.h"
#include "mediapipe/tasks/cc/components/processors/proto/classification_postprocessing_graph_options.pb.h"
#include "mediapipe/tasks/cc/components/processors/proto/classifier_options.pb.h"
#include "mediapipe/tasks/cc/core/model_resources.h"
#include "mediapipe/tasks/cc/metadata/metadata_extractor.h"

namespace mediapipe {
namespace tasks {
namespace components {
namespace processors {

// Configures a ClassificationPostprocessingGraph using the provided model
// resources and ClassifierOptions.
// - Accepts CPU input tensors.
//
// Example usage:
//
//   auto& postprocessing =
//       graph.AddNode("mediapipe.tasks.components.processors.ClassificationPostprocessingGraph");
//   MP_RETURN_IF_ERROR(ConfigureClassificationPostprocessingGraph(
//       model_resources,
//       classifier_options,
//       &preprocessing.GetOptions<ClassificationPostprocessingGraphOptions>()));
//
// The resulting ClassificationPostprocessingGraph has the following I/O:
// Inputs:
//   TENSORS - std::vector<Tensor>
//     The output tensors of an InferenceCalculator.
//   TIMESTAMPS - std::vector<Timestamp> @Optional
//     The collection of the timestamps that this calculator should aggregate.
//     This stream is optional: if provided then the TIMESTAMPED_CLASSIFICATIONS
//     output is used for results. Otherwise as no timestamp aggregation is
//     required the CLASSIFICATIONS output is used for results.
// Outputs:
//   CLASSIFICATIONS - ClassificationResult @Optional
//     The classification results aggregated by head. Must be connected if the
//     TIMESTAMPS input is not connected, as it signals that timestamp
//     aggregation is not required.
//   TIMESTAMPED_CLASSIFICATIONS - std::vector<ClassificationResult> @Optional
//     The classification result aggregated by timestamp, then by head. Must be
//     connected if the TIMESTAMPS input is connected, as it signals that
//     timestamp aggregation is required.
absl::Status ConfigureClassificationPostprocessingGraph(
    const tasks::core::ModelResources& model_resources,
    const proto::ClassifierOptions& classifier_options,
    proto::ClassificationPostprocessingGraphOptions* options);

// Utility function to fill in the TensorsToClassificationCalculatorOptions
// based on the classifier options and the (optional) output tensor metadata.
// This is meant to be used by other graphs that may also rely on this
// calculator.
absl::Status ConfigureTensorsToClassificationCalculator(
    const proto::ClassifierOptions& options,
    const metadata::ModelMetadataExtractor& metadata_extractor,
    int tensor_index,
    mediapipe::TensorsToClassificationCalculatorOptions* calculator_options);

}  // namespace processors
}  // namespace components
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_COMPONENTS_PROCESSORS_CLASSIFICATION_POSTPROCESSING_GRAPH_H_
