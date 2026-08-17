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

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_IMAGE_TO_TENSOR_CONVERTER_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_IMAGE_TO_TENSOR_CONVERTER_H_

#include "mediapipe/calculators/tensor/image_to_tensor_utils.h"
#include "mediapipe/framework/formats/image.h"
#include "mediapipe/framework/formats/tensor.h"
#include "mediapipe/framework/port/statusor.h"

namespace mediapipe {

struct Size {
  int width;
  int height;
};

// Converts image to tensor.
class ImageToTensorConverter {
 public:
  virtual ~ImageToTensorConverter() = default;

  // Converts image to tensor.
  // @image contains image to extract from.
  // @roi describes region of interest within the image to extract (absolute
  // values).
  // @range_min/max describes output tensor range image pixels should converted
  // to.
  // @tensor_buffer_offset an inteter representing the offset of the tensor
  // buffer the result should be written to.
  // @output_tensor a tensor with pre-defined shape. The "Convert" is
  // responsible of populating the content into the output tensor.
  virtual absl::Status Convert(const mediapipe::Image& input,
                               const RotatedRect& roi, float range_min,
                               float range_max, int tensor_buffer_offset,
                               Tensor& output_tensor) = 0;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_IMAGE_TO_TENSOR_CONVERTER_H_
