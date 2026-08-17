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

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_CONVERTER_METAL_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_CONVERTER_METAL_H_

#include "mediapipe/framework/port.h"

#if MEDIAPIPE_METAL_ENABLED

#include <memory>
#include <optional>

#include "absl/status/status.h"
#include "mediapipe/calculators/tensor/tensor_converter_gpu.h"
#include "mediapipe/framework/memory_manager.h"
#import "mediapipe/gpu/MPPMetalHelper.h"

namespace mediapipe {

// Instantiates and initializes an Metal-based TensorConverterGpu instance.
// @gpu_helper helper to manage the Metal context.
// @memory_manager enables buffer pooling. Must outlive the TensorConverterGpu
// instance.
// @output_range defines output floating point scale.
// @include_alpha enables the inclusion of the alpha channel.
// @single_channel limites the conversion to the first channel in input image.
// @flip_vertically enables to v-flip the image during the conversion process.
// @num_output_channels defines the number of channels in output tensor. Note
// that the selected number of converted channels must match
// num_output_channels.
absl::StatusOr<std::unique_ptr<TensorConverterGpu>> CreateTensorConverterMetal(
    MPPMetalHelper* gpu_helper, MemoryManager* memory_manager,
    std::optional<std::pair<float, float>> output_range, bool include_alpha,
    bool single_channel, bool flip_vertically, int num_output_channels);

}  // namespace mediapipe

#endif  // MEDIAPIPE_METAL_ENABLED

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_CONVERTER_METAL_H_
