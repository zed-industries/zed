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

#ifndef MEDIAPIPE_UTIL_FRAME_BUFFER_BUFFER_COMMON_H_
#define MEDIAPIPE_UTIL_FRAME_BUFFER_BUFFER_COMMON_H_

#include "HalideRuntime.h"

namespace mediapipe {
namespace frame_buffer {
namespace common {

// Performs in-place cropping on the given buffer; the provided rectangle
// becomes the full extent of the buffer upon success. Returns false on error.
bool crop_buffer(int x0, int y0, int x1, int y1, halide_buffer_t* buffer);

}  // namespace common
}  // namespace frame_buffer
}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_FRAME_BUFFER_BUFFER_COMMON_H_
