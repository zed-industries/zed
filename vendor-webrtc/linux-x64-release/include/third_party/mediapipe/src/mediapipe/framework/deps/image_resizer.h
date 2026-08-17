// Copyright 2019 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_DEPS_IMAGE_RESIZER_H_
#define MEDIAPIPE_DEPS_IMAGE_RESIZER_H_

#include "mediapipe/framework/port/opencv_imgproc_inc.h"

namespace mediapipe {

class ImageResizer {
 public:
  ImageResizer(double sharpen_coeff) {}

  bool Resize(const cv::Mat& input_mat, cv::Mat* output_mat) {
    cv::resize(input_mat, *output_mat, output_mat->size(), 0, 0,
               cv::INTER_AREA);
    return true;
  }
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_DEPS_IMAGE_RESIZER_H_
