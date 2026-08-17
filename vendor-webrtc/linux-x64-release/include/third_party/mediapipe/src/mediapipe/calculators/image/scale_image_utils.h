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
//
// Utilities for scaling operations defined by ScaleImageCalculatorOptions.
#ifndef MEDIAPIPE_IMAGE_SCALE_IMAGE_UTILS_H_
#define MEDIAPIPE_IMAGE_SCALE_IMAGE_UTILS_H_

#include <string>

#include "mediapipe/framework/port/status.h"

namespace mediapipe {
namespace scale_image {

// Given a width and height and min and max aspect ratios, determine the
// target width and height and column and row starts such that the target
// is a centered, cropped portion of the image that falls within the min
// and max aspect ratio.  If either the min or max aspect ratio argument
// is empty or has a 0 in the numerator or denominator then it is ignored.
absl::Status FindCropDimensions(int input_width, int input_height,    //
                                const std::string& min_aspect_ratio,  //
                                const std::string& max_aspect_ratio,  //
                                int* crop_width, int* crop_height,    //
                                int* col_start, int* row_start);

// Given an input width and height, a target width and height or max area,
// whether to preserve the aspect ratio, and whether to round-down to the
// multiple of a given number nearest to the targets, determine the output width
// and height. If target_width or target_height is non-positive, then they will
// be set to the input_width and input_height respectively. If target_area is
// non-positive, then it will be ignored. If scale_to_multiple_of is less than
// 1, it will be treated like 1. The output_width and output_height will be
// reduced as necessary to preserve_aspect_ratio if the option is specified. If
// preserving the aspect ratio is desired, you must set scale_to_multiple_of
// to 2.
absl::Status FindOutputDimensions(int input_width, int input_height,  //
                                  int target_width,
                                  int target_height,           //
                                  int target_max_area,         //
                                  bool preserve_aspect_ratio,  //
                                  int scale_to_multiple_of,    //
                                  int* output_width, int* output_height);

// Backwards compatible helper.
absl::Status FindOutputDimensions(int input_width, int input_height,  //
                                  int target_width,
                                  int target_height,           //
                                  bool preserve_aspect_ratio,  //
                                  int scale_to_multiple_of,    //
                                  int* output_width, int* output_height);

}  // namespace scale_image
}  // namespace mediapipe

#endif  // MEDIAPIPE_IMAGE_SCALE_IMAGE_UTILS_H_
