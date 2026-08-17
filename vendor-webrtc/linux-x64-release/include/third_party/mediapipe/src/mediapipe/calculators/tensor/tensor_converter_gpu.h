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
#ifndef MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_CONVERTER_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_CONVERTER_H_

#include <optional>
#include <utility>

#include "absl/status/status.h"
#include "mediapipe/framework/formats/tensor.h"
#include "mediapipe/gpu/gpu_buffer.h"

namespace mediapipe {

// Pure abstract interface to implement a GPU-based Tensor converter.
class TensorConverterGpu {
 public:
  virtual ~TensorConverterGpu() = default;

  // Converts input GpuBuffer to Tensor.
  virtual Tensor Convert(const GpuBuffer& input) = 0;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_TENSOR_CONVERTER_H_
