// Copyright 2021 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_RECTANGLE_UTIL_H_
#define MEDIAPIPE_RECTANGLE_UTIL_H_

#include "absl/status/statusor.h"
#include "absl/types/span.h"
#include "mediapipe/framework/formats/rect.pb.h"
#include "mediapipe/framework/port/rectangle.h"

namespace mediapipe {

// Converts a NormalizedRect into a Rectangle_f.
absl::StatusOr<Rectangle_f> ToRectangle(const mediapipe::NormalizedRect& input);

// If the new_rect overlaps with any of the rectangles in
// existing_rects, then return true. Otherwise, return false.
absl::StatusOr<bool> DoesRectOverlap(
    const mediapipe::NormalizedRect& new_rect,
    absl::Span<const mediapipe::NormalizedRect> existing_rects,
    float min_similarity_threshold);

// Computes the Intersection over Union (IoU) between two rectangles.
float CalculateIou(const Rectangle_f& rect1, const Rectangle_f& rect2);

}  // namespace mediapipe

#endif  // MEDIAPIPE_RECTANGLE_UTIL_H_
