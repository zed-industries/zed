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

#ifndef MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_LANDMARK_H_
#define MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_LANDMARK_H_

#include <stdbool.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Landmark represents a point in 3D space with x, y, z coordinates. The
// landmark coordinates are in meters. z represents the landmark depth, and the
// smaller the value the closer the world landmark is to the camera.
struct Landmark {
  float x;
  float y;
  float z;

  // For optional visibility.
  bool has_visibility;

  // Landmark visibility. Should stay unset if not supported.
  // Float score of whether landmark is visible or occluded by other objects.
  // Landmark considered as invisible also if it is not present on the screen
  // (out of scene bounds). Depending on the model, visibility value is either
  // a sigmoid or an argument of sigmoid.
  float visibility;

  // For optional presence.
  bool has_presence;

  // Landmark presence. Should stay unset if not supported.
  // Float score of whether landmark is present on the scene (located within
  // scene bounds). Depending on the model, presence value is either a result
  // of sigmoid or an argument of sigmoid function to get landmark presence
  // probability.
  float presence;

  // Landmark name. Should stay unset if not supported.
  // Defaults to nullptr.
  char* name;
};

// A normalized version of above Landmark struct. All coordinates should be
// within [0, 1].
struct NormalizedLandmark {
  float x;
  float y;
  float z;

  bool has_visibility;
  float visibility;

  bool has_presence;
  float presence;

  char* name;
};

// A list of Landmarks.
struct Landmarks {
  struct Landmark* landmarks;
  uint32_t landmarks_count;
};

// A list of NormalizedLandmarks.
struct NormalizedLandmarks {
  struct NormalizedLandmark* landmarks;
  uint32_t landmarks_count;
};

#ifdef __cplusplus
}  // extern C
#endif

#endif  // MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_LANDMARK_H_
