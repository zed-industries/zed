/* Copyright 2023 The MediaPipe Authors.

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

#ifndef MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_DETECTION_RESULT_H_
#define MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_DETECTION_RESULT_H_

#include <stdint.h>

#include "mediapipe/tasks/c/components/containers/rect.h"

#ifdef __cplusplus
extern "C" {
#endif

// Detection for a single bounding box.
struct Detection {
  // An array of detected categories.
  struct Category* categories;

  // The number of elements in the categories array.
  uint32_t categories_count;

  // The bounding box location.
  struct MPRect bounding_box;

  // Optional list of keypoints associated with the detection. Keypoints
  // represent interesting points related to the detection. For example, the
  // keypoints represent the eye, ear and mouth from face detection model. Or
  // in the template matching detection, e.g. KNIFT, they can represent the
  // feature points for template matching.
  // `nullptr` if keypoints is not present.
  struct NormalizedKeypoint* keypoints;

  // The number of elements in the keypoints array. 0 if keypoints do not exist.
  uint32_t keypoints_count;
};

// Detection results of a model.
struct DetectionResult {
  // An array of Detections.
  struct Detection* detections;

  // The number of detections in the detections array.
  uint32_t detections_count;
};

#ifdef __cplusplus
}  // extern C
#endif

#endif  // MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_DETECTION_RESULT_H_
