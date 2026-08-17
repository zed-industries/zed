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

#ifndef MEDIAPIPE_CALCULATORS_IMAGE_WARP_AFFINE_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_IMAGE_WARP_AFFINE_CALCULATOR_H_

#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/formats/image.h"
#include "mediapipe/framework/formats/image_frame.h"

#if !MEDIAPIPE_DISABLE_GPU
#include "mediapipe/gpu/gpu_buffer.h"
#endif  // !MEDIAPIPE_DISABLE_GPU

namespace mediapipe {

// Runs affine transformation.
//
// Input:
//   IMAGE - Image/ImageFrame/GpuBuffer
//
//   MATRIX - std::array<float, 16>
//     Used as following:
//       output(x, y) = input(matrix[0] * x + matrix[1] * y + matrix[3],
//                            matrix[4] * x + matrix[5] * y + matrix[7])
//     where x and y ranges are defined by @OUTPUT_SIZE.
//
//   OUTPUT_SIZE - std::pair<int, int>
//     Size of the output image.
//
// Output:
//   IMAGE - Image/ImageFrame/GpuBuffer
//
//   Note:
//   - Output image type and format are the same as the input one.
//
// Usage example:
//   node {
//     calculator: "WarpAffineCalculator(Cpu|Gpu)"
//     input_stream: "IMAGE:image"
//     input_stream: "MATRIX:matrix"
//     input_stream: "OUTPUT_SIZE:size"
//     output_stream: "IMAGE:transformed_image"
//     options: {
//       [mediapipe.WarpAffineCalculatorOptions.ext] {
//         border_mode: BORDER_ZERO
//       }
//     }
//   }
template <typename ImageT>
class WarpAffineCalculatorIntf : public mediapipe::api2::NodeIntf {
 public:
  static constexpr mediapipe::api2::Input<ImageT> kInImage{"IMAGE"};
  static constexpr mediapipe::api2::Input<std::array<float, 16>> kMatrix{
      "MATRIX"};
  static constexpr mediapipe::api2::Input<std::pair<int, int>> kOutputSize{
      "OUTPUT_SIZE"};
  static constexpr mediapipe::api2::Output<ImageT> kOutImage{"IMAGE"};
};

#if !MEDIAPIPE_DISABLE_OPENCV
class WarpAffineCalculatorCpu : public WarpAffineCalculatorIntf<ImageFrame> {
 public:
  MEDIAPIPE_NODE_INTERFACE(WarpAffineCalculatorCpu, kInImage, kMatrix,
                           kOutputSize, kOutImage);
};
#endif  // !MEDIAPIPE_DISABLE_OPENCV
#if !MEDIAPIPE_DISABLE_GPU
class WarpAffineCalculatorGpu
    : public WarpAffineCalculatorIntf<mediapipe::GpuBuffer> {
 public:
  MEDIAPIPE_NODE_INTERFACE(WarpAffineCalculatorGpu, kInImage, kMatrix,
                           kOutputSize, kOutImage);
};
#endif  // !MEDIAPIPE_DISABLE_GPU
class WarpAffineCalculator : public WarpAffineCalculatorIntf<mediapipe::Image> {
 public:
  MEDIAPIPE_NODE_INTERFACE(WarpAffineCalculator, kInImage, kMatrix, kOutputSize,
                           kOutImage);
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_IMAGE_WARP_AFFINE_CALCULATOR_H_
