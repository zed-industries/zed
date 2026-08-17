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

#ifndef MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_KEYPOINT_H_
#define MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_KEYPOINT_H_

#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// A keypoint, defined by the coordinates (x, y), normalized by the image
// dimensions.
struct NormalizedKeypoint {
  // x in normalized image coordinates.
  float x;

  // y in normalized image coordinates.
  float y;

  // Optional label of the keypoint. `nullptr` if the label is not present.
  char* label;

  // Optional score of the keypoint.
  float score;

  // `True` if the score is valid.
  bool has_score;
};

#ifdef __cplusplus
}  // extern C
#endif

#endif  // MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_KEYPOINT_H_
