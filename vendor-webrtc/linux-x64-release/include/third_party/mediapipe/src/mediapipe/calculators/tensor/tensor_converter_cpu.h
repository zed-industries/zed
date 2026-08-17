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

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_CONVERTER_CPU_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_CONVERTER_CPU_H_

#include <utility>

#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "mediapipe/framework/formats/image_frame.h"
#include "mediapipe/framework/formats/matrix.h"
#include "mediapipe/framework/formats/tensor.h"
#include "mediapipe/framework/memory_manager.h"

namespace mediapipe {

// Converts an ImageFrame to a vector of Tensors.
// @flip_vertically enables to flip the image during conversion.
// @max_num_channels can be used to reserve extra channels in the output
// tensors.
// @memory_manager MemoryManager instance to enable memory pooling during Tensor
// instantiation.
// Returns output Tensor.
absl::StatusOr<Tensor> ConvertImageFrameToTensorOnCpu(
    const ImageFrame& image_frame, const std::pair<float, float>& output_range,
    bool flip_vertically, int max_num_channels, MemoryManager* memory_manager);

// Converts a Matrix to a vector of Tensors.
// @row_major_matrix defines the ordering in the input matrix.
// @max_num_channels can be used to reserve extra channels in the output
// tensors.
// @memory_manager MemoryManager instance to enable memory pooling during Tensor
// instantiation.
// Returns output Tensor.
absl::StatusOr<Tensor> ConvertMatrixToTensorOnCpu(
    const Matrix& matrix, bool row_major_matrix, MemoryManager* memory_manager);

// For testing only below.
absl::Status NormalizeUInt8Image(const ImageFrame& image_frame,
                                 bool flip_vertically,
                                 const std::pair<float, float>& output_range,
                                 int max_num_channels, float* tensor_ptr);

absl::Status NormalizeFloatImage(const ImageFrame& image_frame,
                                 bool flip_vertically,
                                 const std::pair<float, float>& output_range,
                                 int max_num_channels, float* tensor_ptr);

absl::Status CopyMatrixToTensor(const Matrix& matrix, bool is_row_major_matrix,
                                float* tensor_ptr);

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_CONVERTER_CPU_H_
