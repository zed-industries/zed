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

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_TENSORS_TO_SEGMENTATION_CONVERTER_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_TENSORS_TO_SEGMENTATION_CONVERTER_H_

#include <memory>

#include "absl/status/statusor.h"
#include "mediapipe/framework/formats/image.h"
#include "mediapipe/framework/formats/tensor.h"

namespace mediapipe {

class TensorsToSegmentationConverter {
 public:
  virtual ~TensorsToSegmentationConverter() = default;

  // Converts a tensor to image mask.
  // Returns a unique pointer containing the converted image.
  // @input_tensor is the tensor to be processed.
  // @output_width/height describes output dimensions to reshape the output mask
  // into.
  virtual absl::StatusOr<std::unique_ptr<Image>> Convert(
      const Tensor& input_tensor, int output_width, int output_height) = 0;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_TENSORS_TO_SEGMENTATION_CONVERTER_H_
