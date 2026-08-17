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

#ifndef MEDIAPIPE_UTIL_FRAME_BUFFER_HALIDE_COMMON_H_
#define MEDIAPIPE_UTIL_FRAME_BUFFER_HALIDE_COMMON_H_

#include "Halide.h"

namespace mediapipe {
namespace frame_buffer {
namespace halide {
namespace common {

template <typename T>
Halide::Expr is_planar(const T& buffer) {
  return buffer.dim(0).stride() == 1;
}

template <typename T>
Halide::Expr is_interleaved(const T& buffer) {
  return buffer.dim(0).stride() == buffer.dim(2).extent() &&
         buffer.dim(2).stride() == 1;
}

// Resize scale parameters (fx, fy) are the ratio of source size to output
// size; thus if you want to produce an image half as wide and twice as tall
// as the input, (fx, fy) should be (2, 0.5).

// Nearest-neighbor resize: fast, but low-quality (prone to aliasing).
void resize_nn(Halide::Func input, Halide::Func result, Halide::Expr fx,
               Halide::Expr fy);

// Resize with bilinear interpolation: slower but higher-quality.
void resize_bilinear(Halide::Func input, Halide::Func result, Halide::Expr fx,
                     Halide::Expr fy);
// Identical to the above, except that it uses fixed point integer math.
void resize_bilinear_int(Halide::Func input, Halide::Func result,
                         Halide::Expr fx, Halide::Expr fy);

// Note: width and height are the source image dimensions; angle must be one
// of [0, 90, 180, 270] or the result is undefined.
void rotate(Halide::Func input, Halide::Func result, Halide::Expr width,
            Halide::Expr height, Halide::Expr angle);

}  // namespace common
}  // namespace halide
}  // namespace frame_buffer
}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_FRAME_BUFFER_HALIDE_COMMON_H_
