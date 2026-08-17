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

#ifndef MEDIAPIPE_FACE_GEOMETRY_LIBS_PROCRUSTES_SOLVER_H_
#define MEDIAPIPE_FACE_GEOMETRY_LIBS_PROCRUSTES_SOLVER_H_

#include <memory>

#include "Eigen/Dense"
#include "mediapipe/framework/port/status.h"

namespace mediapipe::face_geometry {

// Encapsulates a stateless solver for the Weighted Extended Orthogonal
// Procrustes (WEOP) Problem, as defined in Section 2.4 of
// https://doi.org/10.3929/ethz-a-004656648.
//
// Given the source and the target point clouds, the algorithm estimates
// a 4x4 transformation matrix featuring the following semantic components:
//
//   * Uniform scale
//   * Rotation
//   * Translation
//
// The matrix maps the source point cloud into the target point cloud minimizing
// the Mean Squared Error.
class ProcrustesSolver {
 public:
  virtual ~ProcrustesSolver() = default;

  // Solves the Weighted Extended Orthogonal Procrustes (WEOP) Problem.
  //
  // All `source_points`, `target_points` and `point_weights` must define the
  // same number of points. Elements of `point_weights` must be non-negative.
  //
  // A too small diameter of either of the point clouds will likely lead to
  // numerical instabilities and failure to estimate the transformation.
  //
  // A too small point cloud total weight will likely lead to numerical
  // instabilities and failure to estimate the transformation too.
  //
  // Small point coordinate deviation for either of the point cloud will likely
  // result in a failure as it will make the solution very unstable if possible.
  //
  // Note: the output `transform_mat` argument is used instead of `StatusOr<>`
  // return type in order to avoid Eigen memory alignment issues. Details:
  // https://eigen.tuxfamily.org/dox/group__TopicStructHavingEigenMembers.html
  virtual absl::Status SolveWeightedOrthogonalProblem(
      const Eigen::Matrix3Xf& source_points,  //
      const Eigen::Matrix3Xf& target_points,  //
      const Eigen::VectorXf& point_weights,   //
      Eigen::Matrix4f& transform_mat) const = 0;
};

std::unique_ptr<ProcrustesSolver> CreateFloatPrecisionProcrustesSolver();

}  // namespace mediapipe::face_geometry

#endif  // MEDIAPIPE_FACE_GEOMETRY_LIBS_PROCRUSTES_SOLVER_H_
