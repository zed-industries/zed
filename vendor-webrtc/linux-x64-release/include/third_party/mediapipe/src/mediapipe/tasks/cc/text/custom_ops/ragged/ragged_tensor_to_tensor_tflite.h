/* Copyright 2023 The MediaPipe Authors.

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

#ifndef MEDIAPIPE_TASKS_CC_TEXT_CUSTOM_OPS_RAGGED_RAGGED_TENSOR_TO_TENSOR_TFLITE_H_
#define MEDIAPIPE_TASKS_CC_TEXT_CUSTOM_OPS_RAGGED_RAGGED_TENSOR_TO_TENSOR_TFLITE_H_

#include "tensorflow/lite/kernels/register.h"

namespace mediapipe::tflite_operations {

TfLiteRegistration* Register_RAGGED_TENSOR_TO_TENSOR();

}  // namespace mediapipe::tflite_operations

#endif  // MEDIAPIPE_TASKS_CC_TEXT_CUSTOM_OPS_RAGGED_RAGGED_TENSOR_TO_TENSOR_TFLITE_H_
