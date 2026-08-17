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

#ifndef MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_RECT_H_
#define MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_RECT_H_

#include <cmath>
#include <cstdlib>

namespace mediapipe::tasks::components::containers {

inline constexpr float kRectFTolerance = 1e-4;

// Defines a rectangle, used e.g. as part of detection results or as input
// region-of-interest.
//
struct Rect {
  int left;
  int top;
  int right;
  int bottom;
};

inline bool operator==(const Rect& lhs, const Rect& rhs) {
  return lhs.left == rhs.left && lhs.top == rhs.top && lhs.right == rhs.right &&
         lhs.bottom == rhs.bottom;
}

// The coordinates are normalized wrt the image dimensions, i.e. generally in
// [0,1] but they may exceed these bounds if describing a region overlapping the
// image. The origin is on the top-left corner of the image.
struct RectF {
  float left;
  float top;
  float right;
  float bottom;
};

inline bool operator==(const RectF& lhs, const RectF& rhs) {
  return std::fabs(lhs.left - rhs.left) < kRectFTolerance &&
         std::fabs(lhs.top - rhs.top) < kRectFTolerance &&
         std::fabs(lhs.right - rhs.right) < kRectFTolerance &&
         std::fabs(lhs.bottom - rhs.bottom) < kRectFTolerance;
}

RectF ToRectF(const Rect& rect, int image_height, int image_width);

Rect ToRect(const RectF& rect, int image_height, int image_width);

}  // namespace mediapipe::tasks::components::containers
#endif  // MEDIAPIPE_TASKS_CC_COMPONENTS_CONTAINERS_RECT_H_
