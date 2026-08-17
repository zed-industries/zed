// Copyright 2023 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_UTIL_POSE_UTIL_H_
#define MEDIAPIPE_UTIL_POSE_UTIL_H_

#include "mediapipe/framework/formats/landmark.pb.h"
#include "mediapipe/framework/port/opencv_core_inc.h"

namespace mediapipe {

void DrawPose(const mediapipe::NormalizedLandmarkList& pose, bool flip_y,
              cv::Mat* image);

void DrawFace(const mediapipe::NormalizedLandmarkList& face,
              const std::pair<int, int>& image_size, const cv::Mat& affine,
              bool flip_y, bool draw_nose, int color_style, bool reverse_color,
              int draw_line_width, cv::Mat* image);

}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_POSE_UTIL_H_
