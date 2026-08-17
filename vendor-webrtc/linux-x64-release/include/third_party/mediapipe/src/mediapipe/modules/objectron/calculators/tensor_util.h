// Copyright 2020 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_TENSOR_UTIL_H_
#define MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_TENSOR_UTIL_H_

#include "mediapipe/framework/formats/tensor.h"
#include "mediapipe/framework/port/opencv_core_inc.h"
#include "tensorflow/lite/interpreter.h"

namespace mediapipe {

// Converts a single channel tflite tensor to a grayscale image
cv::Mat ConvertTfliteTensorToCvMat(const TfLiteTensor& tensor);

// Converts a single channel tensor to grayscale image
cv::Mat ConvertTensorToCvMat(const mediapipe::Tensor& tensor);
}  // namespace mediapipe

#endif  // MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_TENSOR_UTIL_H_
