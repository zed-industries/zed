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

#ifndef MEDIAPIPE_UTIL_TFLITE_OPERATIONS_MAX_UNPOOLING_H_
#define MEDIAPIPE_UTIL_TFLITE_OPERATIONS_MAX_UNPOOLING_H_

#include "tensorflow/lite/kernels/kernel_util.h"

namespace mediapipe {
namespace tflite_operations {

TfLiteRegistration* RegisterMaxUnpooling2D();

}  // namespace tflite_operations
}  // namespace mediapipe

#endif  // MEDIAPIPE_UTIL_TFLITE_OPERATIONS_MAX_UNPOOLING_H_
