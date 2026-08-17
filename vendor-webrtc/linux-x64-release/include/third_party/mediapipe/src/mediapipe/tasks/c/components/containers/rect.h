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

#ifndef MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_RECT_H_
#define MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_RECT_H_

#ifdef __cplusplus
extern "C" {
#endif

// Defines a rectangle, used e.g. as part of detection results or as input
// region-of-interest.
struct MPRect {
  int left;
  int top;
  int bottom;
  int right;
};

// The coordinates are normalized wrt the image dimensions, i.e. generally in
// [0,1] but they may exceed these bounds if describing a region overlapping the
// image. The origin is on the top-left corner of the image.
struct MPRectF {
  float left;
  float top;
  float bottom;
  float right;
};

#ifdef __cplusplus
}  // extern C
#endif

#endif  // MEDIAPIPE_TASKS_C_COMPONENTS_CONTAINERS_RECT_H_
