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

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_TENSORS_TO_SEGMENTATION_CONVERTER_OPENCV_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_TENSORS_TO_SEGMENTATION_CONVERTER_OPENCV_H_

#include <memory>

#include "absl/status/statusor.h"
#include "mediapipe/calculators/tensor/tensors_to_segmentation_calculator.pb.h"
#include "mediapipe/calculators/tensor/tensors_to_segmentation_converter.h"

namespace mediapipe {
// Creates OpenCV tensors-to-segmentation converter.
absl::StatusOr<std::unique_ptr<TensorsToSegmentationConverter>>
CreateOpenCvConverter(
    const mediapipe::TensorsToSegmentationCalculatorOptions& options);
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_TENSORS_TO_SEGMENTATION_CONVERTER_OPENCV_H_
