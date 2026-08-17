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

#ifndef MEDIAPIPE_TASKS_CC_CORE_UTILS_H_
#define MEDIAPIPE_TASKS_CC_CORE_UTILS_H_

#include <algorithm>
#include <cstring>
#include <numeric>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "flatbuffers/flatbuffers.h"
#include "mediapipe/framework/api2/builder.h"
#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/tasks/metadata/metadata_schema_generated.h"

namespace mediapipe {
namespace tasks {
namespace core {

// Loads binary content of a file into a string.
std::string LoadBinaryContent(const char* filename);

// Finds the tensor index of the specified tensor name from a vector of tensors
// by checking the metadata tensor name.
// The range of the return value should be [0, tensor_size). Return -1 if no
// tensor is found by name.
int FindTensorIndexByMetadataName(
    const flatbuffers::Vector<flatbuffers::Offset<tflite::TensorMetadata>>*
        tensor_metadata,
    absl::string_view name);

// Finds the tensor index of the specified tensor name from a vector of tensors
// by first checking the metadata tensor name, and then the model tensor name.
// The range of the return value should be [0, tensor_size). Return -1 if no
// tensor is found by name.
template <typename TensorType>
int FindTensorIndexByName(
    const std::vector<TensorType*>& tensors,
    const flatbuffers::Vector<flatbuffers::Offset<tflite::TensorMetadata>>*
        tensor_metadata,
    absl::string_view metadata_tensor_name,
    absl::string_view model_tensor_name) {
  if (tensor_metadata != nullptr && tensor_metadata->size() == tensors.size()) {
    int index =
        FindTensorIndexByMetadataName(tensor_metadata, metadata_tensor_name);
    if (index > -1) return index;
  }

  return FindTensorIndexByModelName(tensors, model_tensor_name);
}

// Finds the tensor from a vector of tensors with name specified inside
// metadata.
template <typename TensorType>
static TensorType* FindTensorByName(
    const std::vector<TensorType*>& tensors,
    const flatbuffers::Vector<flatbuffers::Offset<tflite::TensorMetadata>>*
        tensor_metadata,
    absl::string_view metadata_tensor_name) {
  int index = FindTensorIndexByName(tensors, tensor_metadata,
                                    metadata_tensor_name, absl::string_view());
  return index == -1 ? nullptr : tensors[index];
}

// Adds a FlowLimiterCalculator to limit the number of packets in flight and
// in queue.
::mediapipe::CalculatorGraphConfig AddFlowLimiterCalculator(
    api2::builder::Graph& graph, api2::builder::GenericNode& task_subgraph,
    std::vector<std::string> input_stream_tags, std::string finished_stream_tag,
    int max_in_flight = 1, int max_in_queue = 1);

// Fixs the graph config containing PreviousLoopbackCalculator where the edge
// forming a loop needs to be tagged as back edge.
void FixGraphBackEdges(::mediapipe::CalculatorGraphConfig& graph_config);

}  // namespace core
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_CORE_UTILS_H_
