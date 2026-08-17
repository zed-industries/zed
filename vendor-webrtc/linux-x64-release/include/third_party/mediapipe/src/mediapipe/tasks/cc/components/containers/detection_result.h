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

#ifndef MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_DETECTION_RESULT_H_
#define MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_DETECTION_RESULT_H_

#include <optional>
#include <string>
#include <vector>

#include "mediapipe/framework/formats/detection.pb.h"
#include "mediapipe/tasks/cc/components/containers/category.h"
#include "mediapipe/tasks/cc/components/containers/keypoint.h"
#include "mediapipe/tasks/cc/components/containers/rect.h"

namespace mediapipe::tasks::components::containers {

// Detection for a single bounding box.
struct Detection {
  // A vector of detected categories.
  std::vector<Category> categories;
  // The bounding box location.
  Rect bounding_box;
  // Optional list of keypoints associated with the detection. Keypoints
  // represent interesting points related to the detection. For example, the
  // keypoints represent the eye, ear and mouth from face detection model. Or
  // in the template matching detection, e.g. KNIFT, they can represent the
  // feature points for template matching.
  std::optional<std::vector<NormalizedKeypoint>> keypoints = std::nullopt;
};

// Detection results of a model.
struct DetectionResult {
  // A vector of Detections.
  std::vector<Detection> detections;
};

// Utility function to convert from Detection proto to Detection struct.
Detection ConvertToDetection(const mediapipe::Detection& detection_proto);

// Utility function to convert from list of Detection proto to DetectionResult
// struct.
DetectionResult ConvertToDetectionResult(
    std::vector<mediapipe::Detection> detections_proto);

}  // namespace mediapipe::tasks::components::containers
#endif  // MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_DETECTION_RESULT_H_
