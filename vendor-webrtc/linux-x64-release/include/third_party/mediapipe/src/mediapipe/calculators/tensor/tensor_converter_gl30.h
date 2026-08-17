// Copyright 2024 The MediaPipe Authors.
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
#ifndef MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_CONVERTER_GL_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_CONVERTER_GL_H_

#include <memory>
#include <optional>
#include <utility>

#include "mediapipe/framework/port.h"

#if MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_30
#include "absl/status/statusor.h"
#include "mediapipe/calculators/tensor/tensor_converter_gpu.h"
#include "mediapipe/framework/memory_manager.h"
#include "mediapipe/gpu/gl_calculator_helper.h"

namespace mediapipe {

// Instantiates and initializes an OpenGL 3.0-enabled TensorConverterGpu
// instance.
// @gpu_helper helper to manage the OpenGL context.
// @memory_manager Enables buffer pooling. Must outlive the TensorConverterGpu
// instance.
// @input_width width of input image.
// @input_height height of input image.
// @output_range defines output floating point scale.
// @include_alpha enables the inclusion of the alpha channel.
// @single_channel limites the conversion to the first channel in input image.
// @flip_vertically enables to v-flip the image during the conversion process.
// @num_output_channels defines the number of channels in output tensor. Note
// that the selected number of converted channels must match
// num_output_channels.
absl::StatusOr<std::unique_ptr<TensorConverterGpu>> CreateTensorConverterGl30(
    GlCalculatorHelper& gpu_helper, MemoryManager* memory_manager,
    int input_width, int input_height,
    std::optional<std::pair<float, float>> output_range, bool include_alpha,
    bool single_channel, bool flip_vertically, int num_output_channels);

}  // namespace mediapipe
#endif  // MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_30

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_CONVERTER_GL_H_
