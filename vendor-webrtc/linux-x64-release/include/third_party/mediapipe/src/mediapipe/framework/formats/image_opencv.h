// Copyright 2022 The MediaPipe Authors.
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
//
// Helper functions for working with ImageFrame and OpenCV.
#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_IMAGE_OPENCV_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_IMAGE_OPENCV_H_

#include "mediapipe/framework/formats/image.h"
#include "mediapipe/framework/port/opencv_core_inc.h"

namespace mediapipe {
namespace formats {

// Image to OpenCV helper conversion function.
// A view into existing data is created (zero copy).
// The pixel data remains owned and maintained by mediapipe::Image.
// When converting a const Image into a cv::Mat,
// the const modifier is lost.  The caller must be careful
// not to use the returned object to modify the data in a const Image,
// even though the returned data is mutable.
// Note: this returns a shared_ptr so it can keep the CPU memory referenced
// by the Mat alive.
std::shared_ptr<cv::Mat> MatView(const mediapipe::Image* image);

}  // namespace formats
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_IMAGE_FRAME_OPENCV_H_
