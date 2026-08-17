/* Copyright 2020 The TensorFlow Authors. All Rights Reserved.

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

#ifndef TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_UTILS_FRAME_BUFFER_UTILS_INTERFACE_H_
#define TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_UTILS_FRAME_BUFFER_UTILS_INTERFACE_H_

#include "absl/status/status.h"  // from @com_google_absl
#include "tensorflow_lite_support/cc/task/vision/core/frame_buffer.h"

namespace tflite {
namespace task {
namespace vision {

// Interface for the FrameBuffer image processing library.
class FrameBufferUtilsInterface {
 public:
  virtual ~FrameBufferUtilsInterface() = default;

  // Crops `buffer` to the specified points.
  //
  // The coordinate system has its origin at the upper left corner, and
  // positive values extend down and to the right from it. After cropping,
  // the top left point becomes (0, 0). The new width and height are
  // (x1 - x0 + 1, y1 - y0 + 1).
  //
  // The `output_buffer` should have metadata populated and its backing buffer
  // should be big enough to store the operation result.
  virtual absl::Status Crop(const FrameBuffer& buffer, int x0, int y0, int x1,
                            int y1, FrameBuffer* output_buffer) = 0;

  // Resizes `buffer` to the size of the given `output_buffer` using bilinear
  // interpolation.
  //
  // The resize dimension is determined based on the size of `output_buffer`.
  //
  // The `output_buffer` should have metadata populated and its backing buffer
  // should be big enough to store the operation result.
  virtual absl::Status Resize(const FrameBuffer& buffer,
                              FrameBuffer* output_buffer) = 0;

  // Resizes `buffer` to the size of the given `output_buffer` using
  // nearest-neighbor interpolation.
  //
  // The resize dimension is determined based on the size of `output_buffer`.
  //
  // The `output_buffer` should have metadata populated and its backing buffer
  // should be big enough to store the operation result.
  virtual absl::Status ResizeNearestNeighbor(const FrameBuffer& buffer,
                                             FrameBuffer* output_buffer) = 0;

  // Rotates `buffer` counter-clockwise by the given `angle_deg` (in degrees).
  //
  // When rotating by 90 degrees, the top-right corner of `buffer` becomes
  // the top-left corner of `output_buffer`. The given angle must be a multiple
  // of 90 degrees.
  //
  // The `output_buffer` should have metadata populated and its backing buffer
  // should be big enough to store the operation result.
  virtual absl::Status Rotate(const FrameBuffer& buffer, int angle_deg,
                              FrameBuffer* output_buffer) = 0;

  // Flips `buffer` horizontally.
  //
  // The `output_buffer` should have metadata populated and its backing buffer
  // should be big enough to store the operation result.
  virtual absl::Status FlipHorizontally(const FrameBuffer& buffer,
                                        FrameBuffer* output_buffer) = 0;

  // Flips `buffer` vertically.
  //
  // The `output_buffer` should have metadata populated and its backing buffer
  // should be big enough to store the operation result.
  virtual absl::Status FlipVertically(const FrameBuffer& buffer,
                                      FrameBuffer* output_buffer) = 0;

  // Converts `buffer`'s format to the format of the given `output_buffer`.
  //
  // The `output_buffer` should have metadata populated and its backing buffer
  // should be big enough to store the operation result.
  virtual absl::Status Convert(const FrameBuffer& buffer,
                               FrameBuffer* output_buffer) = 0;
};
}  // namespace vision
}  // namespace task
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_CC_TASK_VISION_UTILS_FRAME_BUFFER_UTILS_INTERFACE_H_
