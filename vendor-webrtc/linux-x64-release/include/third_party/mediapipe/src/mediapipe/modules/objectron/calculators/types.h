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

#ifndef MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_TYPES_H_
#define MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_TYPES_H_

#include <array>

#include "Eigen/Geometry"

namespace mediapipe {

using Eigen::Map;
using Eigen::Vector2f;
using Eigen::Vector3f;
using Eigen::Vector4f;
using Matrix4f_RM = Eigen::Matrix<float, 4, 4, Eigen::RowMajor>;
using Matrix3f_RM = Eigen::Matrix<float, 3, 3, Eigen::RowMajor>;

using Face = std::array<int, 4>;

struct SuperPoint {
  enum PointSourceType { kPointCloud = 0, kBoundingBox = 1, kSkeleton = 2 };
  // The id of the point in the point-cloud
  int reference_point;
  // The source of the
  PointSourceType source;
  // The id of the point in set of points in current frame
  int id;
  // If source is kBoundingBox or kSkeleton, object_id stores the id of which \
  // object this point belongs to.
  int object_id;
  // projected u-v value
  Vector2f uv;
  Vector2f pixel;
  // the 3D point
  Vector3f point_3d;
  // Color
  Eigen::Matrix<unsigned char, 4, 1> color;
  bool rendered;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_TYPES_H_
