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

#ifndef MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_CLASSIFICATION_RESULT_H_
#define MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_CLASSIFICATION_RESULT_H_

#include <optional>
#include <string>
#include <vector>

#include "mediapipe/framework/formats/classification.pb.h"
#include "mediapipe/tasks/cc/components/containers/category.h"
#include "mediapipe/tasks/cc/components/containers/proto/classifications.pb.h"

namespace mediapipe::tasks::components::containers {

// Defines classification results for a given classifier head.
struct Classifications {
  // The array of predicted categories, usually sorted by descending scores,
  // e.g. from high to low probability.
  std::vector<Category> categories;
  // The index of the classifier head (i.e. output tensor) these categories
  // refer to. This is useful for multi-head models.
  int head_index;
  // The optional name of the classifier head, as provided in the TFLite Model
  // Metadata [1] if present. This is useful for multi-head models.
  //
  // [1]: https://www.tensorflow.org/lite/convert/metadata
  std::optional<std::string> head_name = std::nullopt;
};

// Defines classification results of a model.
struct ClassificationResult {
  // The classification results for each head of the model.
  std::vector<Classifications> classifications;
  // The optional timestamp (in milliseconds) of the start of the chunk of data
  // corresponding to these results.
  //
  // This is only used for classification on time series (e.g. audio
  // classification). In these use cases, the amount of data to process might
  // exceed the maximum size that the model can process: to solve this, the
  // input data is split into multiple chunks starting at different timestamps.
  std::optional<int64_t> timestamp_ms = std::nullopt;
};

// Utility function to convert from Classifications proto to
// Classifications struct.
Classifications ConvertToClassifications(const proto::Classifications& proto);

// Utility function to convert from ClassificationList proto to
// Classifications struct.
Classifications ConvertToClassifications(
    const mediapipe::ClassificationList& proto, int head_index = 0,
    std::optional<std::string> head_name = std::nullopt);

// Utility function to convert from ClassificationResult proto to
// ClassificationResult struct.
ClassificationResult ConvertToClassificationResult(
    const proto::ClassificationResult& proto);

}  // namespace mediapipe::tasks::components::containers

#endif  // MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_CLASSIFICATION_RESULT_H_
