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

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_IMAGE_TO_TENSOR_CONVERTER_FRAME_BUFFER_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_IMAGE_TO_TENSOR_CONVERTER_FRAME_BUFFER_H_

#include <memory>

#include "absl/status/statusor.h"
#include "mediapipe/calculators/tensor/image_to_tensor_converter.h"
#include "mediapipe/calculators/tensor/image_to_tensor_utils.h"
#include "mediapipe/framework/calculator_context.h"

namespace mediapipe {

// Creates FrameBuffer-based image-to-tensor converter relying on Halide.
absl::StatusOr<std::unique_ptr<ImageToTensorConverter>>
CreateFrameBufferConverter(CalculatorContext* cc, BorderMode border_mode,
                           Tensor::ElementType tensor_type);

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_IMAGE_TO_TENSOR_CONVERTER_FRAME_BUFFER_H_
