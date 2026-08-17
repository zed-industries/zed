/* Copyright 2023 The MediaPipe Authors. All Rights Reserved.

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

#include "mediapipe/tasks/cc/components/processors/proto/detection_postprocessing_graph_options.pb.h"
#include "mediapipe/tasks/cc/components/processors/proto/detector_options.pb.h"
#include "mediapipe/tasks/cc/core/model_resources.h"

#ifndef MEDIAPIPE_TASKS_CC_COMPONENTS_PROCESSORS_DETECTION_POSTPROCESSING_GRAPH_H_
#define MEDIAPIPE_TASKS_CC_COMPONENTS_PROCESSORS_DETECTION_POSTPROCESSING_GRAPH_H_

namespace mediapipe {
namespace tasks {
namespace components {
namespace processors {

// Configures a DetectionPostprocessingGraph using the provided model
// resources and DetectorOptions.
//
// Example usage:
//
//   auto& postprocessing =
//       graph.AddNode("mediapipe.tasks.components.processors.DetectionPostprocessingGraph");
//   MP_RETURN_IF_ERROR(ConfigureDetectionPostprocessingGraph(
//       model_resources,
//       detector_options,
//       &preprocessing.GetOptions<DetectionPostprocessingGraphOptions>()));
//
// The resulting DetectionPostprocessingGraph has the following I/O:
// Inputs:
//   TENSORS - std::vector<Tensor>
//     The output tensors of an InferenceCalculator. The tensors vector could be
//     size 4 or size 2. Tensors vector of size 4 expects the tensors from the
//     models with DETECTION_POSTPROCESS ops in the tflite graph. Tensors vector
//     of size 2 expects the tensors from the models without the ops.
//   [1]:
//     https://github.com/tensorflow/tensorflow/blob/master/tensorflow/lite/kernels/detection_postprocess.cc
// Outputs:
//   DETECTIONS - std::vector<Detection>
//     The postprocessed detection results.
absl::Status ConfigureDetectionPostprocessingGraph(
    const tasks::core::ModelResources& model_resources,
    const proto::DetectorOptions& detector_options,
    proto::DetectionPostprocessingGraphOptions& options);

}  // namespace processors
}  // namespace components
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_COMPONENTS_PROCESSORS_DETECTION_POSTPROCESSING_GRAPH_H_
