// Copyright 2018 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_TENSORFLOW_UTIL_TENSOR_TO_DETECTION_H_
#define MEDIAPIPE_TENSORFLOW_UTIL_TENSOR_TO_DETECTION_H_

#include <map>

#include "absl/types/variant.h"
#include "mediapipe/framework/formats/detection.pb.h"
#include "mediapipe/framework/port/status.h"
#include "tensorflow/core/framework/tensor.h"

namespace mediapipe {

Detection TensorToDetection(
    const ::tensorflow::TTypes<const float>::Vec& box, float score,
    const ::absl::variant<int, std::string>& class_label);

absl::Status TensorsToDetections(const ::tensorflow::Tensor& num_detections,
                                 const ::tensorflow::Tensor& boxes,
                                 const ::tensorflow::Tensor& scores,
                                 const ::tensorflow::Tensor& classes,
                                 const std::map<int, std::string>& label_map,
                                 std::vector<Detection>* detections);

// Use this version if keypoints or masks are available.
absl::Status TensorsToDetections(const ::tensorflow::Tensor& num_detections,
                                 const ::tensorflow::Tensor& boxes,
                                 const ::tensorflow::Tensor& scores,
                                 const ::tensorflow::Tensor& classes,
                                 const ::tensorflow::Tensor& keypoints,
                                 const ::tensorflow::Tensor& masks,
                                 float mask_threshold,
                                 const std::map<int, std::string>& label_map,
                                 std::vector<Detection>* detections);
}  // namespace mediapipe

#endif  // MEDIAPIPE_TENSORFLOW_UTIL_TENSOR_TO_DETECTION_H_
