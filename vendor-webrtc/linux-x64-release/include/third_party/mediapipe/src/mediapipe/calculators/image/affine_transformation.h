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

#ifndef MEDIAPIPE_CALCULATORS_IMAGE_AFFINE_TRANSFORMATION_H_
#define MEDIAPIPE_CALCULATORS_IMAGE_AFFINE_TRANSFORMATION_H_

#include <array>

#include "absl/status/statusor.h"

namespace mediapipe {

class AffineTransformation {
 public:
  // Pixel extrapolation method.
  // When converting image to tensor it may happen that tensor needs to read
  // pixels outside image boundaries. Border mode helps to specify how such
  // pixels will be calculated.
  enum class BorderMode { kZero, kReplicate };

  // Pixel sampling interpolation method.
  enum class Interpolation { kLinear, kCubic };

  struct Size {
    int width;
    int height;
  };

  template <typename InputT, typename OutputT>
  class Runner {
   public:
    virtual ~Runner() = default;

    // Transforms input into output using @matrix as following:
    //   output(x, y) = input(matrix[0] * x + matrix[1] * y + matrix[3],
    //                        matrix[4] * x + matrix[5] * y + matrix[7])
    // where x and y ranges are defined by @output_size.
    virtual absl::StatusOr<OutputT> Run(const InputT& input,
                                        const std::array<float, 16>& matrix,
                                        const Size& output_size,
                                        BorderMode border_mode) = 0;
  };
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_IMAGE_AFFINE_TRANSFORMATION_H_
