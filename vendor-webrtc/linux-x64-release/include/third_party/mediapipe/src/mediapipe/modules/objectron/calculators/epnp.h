// Copyright 2021 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_EPNP_H_
#define MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_EPNP_H_

#include <vector>

#include "Eigen/Dense"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "mediapipe/framework/port/logging.h"

namespace mediapipe {

// This function performs EPnP algorithm, lifting normalized 2D points in pixel
// space to 3D points in camera coordinate.
//
// Inputs:
//   focal_x: camera focal length along x.
//   focal_y: camera focal length along y.
//   center_x: camera center along x.
//   center_y: camera center along y.
//   portrait: a boolen variable indicating whether our images are obtained in
//     portrait orientation or not.
//   input_points_2d: input 2D points to be lifted to 3D.
//   output_points_3d: ouput 3D points in camera coordinate.
absl::Status SolveEpnp(const float focal_x, const float focal_y,
                       const float center_x, const float center_y,
                       const bool portrait,
                       const std::vector<Eigen::Vector2f>& input_points_2d,
                       std::vector<Eigen::Vector3f>* output_points_3d);

// This function performs EPnP algorithm, lifting normalized 2D points in pixel
// space to 3D points in camera coordinate.
//
// Inputs:
//   projection_matrix: the projection matrix from 3D coordinate
//     to screen coordinate.
//   portrait: a boolen variable indicating whether our images are obtained in
//     portrait orientation or not.
//   input_points_2d: input 2D points to be lifted to 3D.
//   output_points_3d: ouput 3D points in camera coordinate.
absl::Status SolveEpnp(const Eigen::Matrix4f& projection_matrix,
                       const bool portrait,
                       const std::vector<Eigen::Vector2f>& input_points_2d,
                       std::vector<Eigen::Vector3f>* output_points_3d);

}  // namespace mediapipe

#endif  // MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_EPNP_H_
