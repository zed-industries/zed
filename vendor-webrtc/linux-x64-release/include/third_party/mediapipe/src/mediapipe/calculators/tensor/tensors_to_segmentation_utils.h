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

#ifndef MEDIAPIPE_CALCULATORS_TENSOR_TENSORS_TO_SEGMENTATION_UTILS_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_TENSORS_TO_SEGMENTATION_UTILS_H_

#include <tuple>
#include <vector>

#include "absl/status/statusor.h"

namespace mediapipe {
namespace tensors_to_segmentation_utils {

// Commonly used to compute the number of blocks to launch in a kernel.
int NumGroups(const int size, const int group_size);  // NOLINT

bool CanUseGpu();

absl::StatusOr<std::tuple<int, int, int>> GetHwcFromDims(
    const std::vector<int>& dims);

void GlRender();

}  // namespace tensors_to_segmentation_utils
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_TENSORS_TO_SEGMENTATION_UTILS_H_
