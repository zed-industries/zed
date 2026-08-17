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

#ifndef MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_LANDMARK_H_
#define MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_LANDMARK_H_

#include <cstdlib>
#include <optional>
#include <string>
#include <vector>

#include "mediapipe/framework/formats/landmark.pb.h"

namespace mediapipe::tasks::components::containers {
inline constexpr float kLandmarkTolerance = 1e-6;

// Landmark represents a point in 3D space with x, y, z coordinates. The
// landmark coordinates are in meters. z represents the landmark depth, and the
// smaller the value the closer the world landmark is to the camera.
struct Landmark {
  float x;
  float y;
  float z;
  // Landmark visibility. Should stay unset if not supported.
  // Float score of whether landmark is visible or occluded by other objects.
  // Landmark considered as invisible also if it is not present on the screen
  // (out of scene bounds). Depending on the model, visibility value is either a
  // sigmoid or an argument of sigmoid.
  std::optional<float> visibility = std::nullopt;
  // Landmark presence. Should stay unset if not supported.
  // Float score of whether landmark is present on the scene (located within
  // scene bounds). Depending on the model, presence value is either a result of
  // sigmoid or an argument of sigmoid function to get landmark presence
  // probability.
  std::optional<float> presence = std::nullopt;
  // Landmark name. Should stay unset if not supported.
  std::optional<std::string> name = std::nullopt;
};

inline bool operator==(const Landmark& lhs, const Landmark& rhs) {
  return abs(lhs.x - rhs.x) < kLandmarkTolerance &&
         abs(lhs.y - rhs.y) < kLandmarkTolerance &&
         abs(lhs.z - rhs.z) < kLandmarkTolerance;
}

// A normalized version of above Landmark struct. All coordinates should be
// within [0, 1].
struct NormalizedLandmark {
  float x;
  float y;
  float z;
  std::optional<float> visibility = std::nullopt;
  std::optional<float> presence = std::nullopt;
  std::optional<std::string> name = std::nullopt;
};

inline bool operator==(const NormalizedLandmark& lhs,
                       const NormalizedLandmark& rhs) {
  return abs(lhs.x - rhs.x) < kLandmarkTolerance &&
         abs(lhs.y - rhs.y) < kLandmarkTolerance &&
         abs(lhs.z - rhs.z) < kLandmarkTolerance;
}

// A list of Landmarks.
struct Landmarks {
  std::vector<Landmark> landmarks;
};

// A list of NormalizedLandmarks.
struct NormalizedLandmarks {
  std::vector<NormalizedLandmark> landmarks;
};

// Utility function to convert from Landmark proto to Landmark struct.
Landmark ConvertToLandmark(const mediapipe::Landmark& proto);

// Utility function to convert from NormalizedLandmark proto to
// NormalizedLandmark struct.
NormalizedLandmark ConvertToNormalizedLandmark(
    const mediapipe::NormalizedLandmark& proto);

// Utility function to convert from LandmarkList proto to Landmarks struct.
Landmarks ConvertToLandmarks(const mediapipe::LandmarkList& proto);

// Utility function to convert from NormalizedLandmarkList proto to
// NormalizedLandmarks struct.
NormalizedLandmarks ConvertToNormalizedLandmarks(
    const mediapipe::NormalizedLandmarkList& proto);

}  // namespace mediapipe::tasks::components::containers

#endif  // MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_LANDMARK_H_
