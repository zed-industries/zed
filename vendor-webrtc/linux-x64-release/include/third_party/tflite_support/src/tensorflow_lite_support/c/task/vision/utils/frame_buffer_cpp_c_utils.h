/* Copyright 2021 The TensorFlow Authors. All Rights Reserved.

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
#ifndef TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_FRAME_BUFFER_CPP_C_UTILS_H_
#define TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_FRAME_BUFFER_CPP_C_UTILS_H_

#include "tensorflow_lite_support/c/task/vision/core/frame_buffer.h"
#include "tensorflow_lite_support/cc/task/vision/utils/frame_buffer_common_utils.h"

// Utils for Conversions between C and C++ FrameBuffer
// -----------------------------------------------------------------
// Meant to be used with vision C apis.

// Creates the C++ FrameBuffer from the C FrameBuffer
namespace tflite {
namespace task {
namespace vision {

tflite::support::StatusOr<std::unique_ptr<tflite::task::vision::FrameBuffer>>
CreateCppFrameBuffer(const TfLiteFrameBuffer* frame_buffer);

}  // namespace vision
}  // namespace task
}  // namespace tflite
#endif  // TENSORFLOW_LITE_SUPPORT_C_TASK_VISION_FRAME_BUFFER_CPP_C_UTILS_H_
