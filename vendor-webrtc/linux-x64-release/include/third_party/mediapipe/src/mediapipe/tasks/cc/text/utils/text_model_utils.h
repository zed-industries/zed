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

#ifndef MEDIAPIPE_TASKS_CC_TEXT_UTILS_TEXT_MODEL_UTILS_H_
#define MEDIAPIPE_TASKS_CC_TEXT_UTILS_TEXT_MODEL_UTILS_H_

#include "absl/status/statusor.h"
#include "mediapipe/tasks/cc/components/processors/proto/text_model_type.pb.h"
#include "mediapipe/tasks/cc/core/model_resources.h"

namespace mediapipe::tasks::text::utils {

// Determines the ModelType for the model based on its metadata as well
// as its input tensors' type and count. Returns an error if there is no
// compatible model type.
absl::StatusOr<components::processors::proto::TextModelType::ModelType>
GetModelType(const core::ModelResources& model_resources);

}  // namespace mediapipe::tasks::text::utils

#endif  // MEDIAPIPE_TASKS_CC_TEXT_UTILS_TEXT_MODEL_UTILS_H_
