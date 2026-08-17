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

#ifndef MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_CATEGORY_H_
#define MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_CATEGORY_H_

#include <optional>
#include <string>

#include "mediapipe/framework/formats/classification.pb.h"

namespace mediapipe::tasks::components::containers {

// Defines a single classification result.
//
// The label maps packed into the TFLite Model Metadata [1] are used to populate
// the 'category_name' and 'display_name' fields.
//
// [1]: https://www.tensorflow.org/lite/convert/metadata
struct Category {
  // The index of the category in the classification model output.
  int index;
  // The score for this category, e.g. (but not necessarily) a probability in
  // [0,1].
  float score;
  // The optional ID for the category, read from the label map packed in the
  // TFLite Model Metadata if present. Not necessarily human-readable.
  std::optional<std::string> category_name = std::nullopt;
  // The optional human-readable name for the category, read from the label map
  // packed in the TFLite Model Metadata if present.
  std::optional<std::string> display_name = std::nullopt;
};

// Utility function to convert from mediapipe::Classification proto to Category
// struct.
Category ConvertToCategory(const mediapipe::Classification& proto);

}  // namespace mediapipe::tasks::components::containers

#endif  // MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_CATEGORY_H_
