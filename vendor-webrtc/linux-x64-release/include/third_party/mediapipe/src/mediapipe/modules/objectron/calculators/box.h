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

#ifndef MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_BOX_H_
#define MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_BOX_H_

#include <vector>

#include "mediapipe/modules/objectron/calculators/model.h"

namespace mediapipe {

// Model for the bounding box in 3D
// The box has 9 degrees of freedom, which uniquely defines 8 keypoints in the
// fixed world-coordinate system.
//
// The 8 keypoints are defined as follows
//
//  kp-id  axis
//  0      000    ---
//  1      001    --+
//  2      010    -+-
//  3      011    -++
//  4      100    +--
//  5      101    +-+
//  6      110    ++-
//  7      111    +++
//
// where xyz means positive or negative vector along the axis where the center
// of the box is the origin. The resulting bounding box is
//
//              x                              x
//      0 + + + + + + + + 4                 .-------
//      +\                +\                |\
//      + \ y             + \             z | \ y
//      +  \              +  \              |  \
//      +   2 + + + + + + + + 6
//    z +   +             +   +
//      +   +             +   +
//      +   +     C       +   +
//      +   +             +   +
//      1 + + + + + + + + 5   +
//       \  +              \  +
//        \ +               \ +
//         \+                \+
//          3 + + + + + + + + 7
//
// World coordinate system: +y is up (aligned with gravity),
// +z is toward the user, +x follows right hand rule.
// The front face is defined as +z axis on xy plane.
// The top face is defined as +y axis on xz plane.
//

class Box : public Model {
 public:
  EIGEN_MAKE_ALIGNED_OPERATOR_NEW

  explicit Box(const std::string& category);
  ~Box() override = default;

  bool InsideTest(const Vector3f& point, int check_axis) const;

  const std::vector<Face>& GetFaces() const { return faces_; }
  const Face& GetFace(size_t face_id) const { return faces_[face_id]; }

  const std::vector<std::array<int, 2>>& GetEdges() const { return edges_; }
  const std::array<int, 2>& GetEdge(size_t edge_id) const {
    return edges_[edge_id];
  }

  // Returns the keypoints for the front face of the box.
  // The front face is defind as a face with +z normal vector on xy plane
  // In Box's c'tor, the top face is set to {1, 3, 7, 5}
  const Face& GetFrontFace() const;

  // Returns the keypoints for the top face of the box.
  // The top face is defind as a face with +z normal vector on xy plane
  // In Box's c'tor, the top face is set to {1, 3, 7, 5}
  const Face& GetTopFace() const;

  void Update() override;
  void Adjust(const std::vector<float>& variables) override;
  float* GetVertex(size_t vertex_id) override;
  const float* GetVertex(size_t vertex_id) const override;
  void Deserialize(const Object& obj) override;
  void Serialize(Object* obj) override;

  // Computes the plane center and the normal vector for the plane the object
  // is sitting on in the world cooordinate system. The normal vector is roughly
  // aligned with gravity.
  std::pair<Vector3f, Vector3f> GetGroundPlane() const;

  // Estimates a box 9-dof parameters from the given vertices. Directly computes
  // the scale of the box, then solves for orientation and translation.
  // Expects a std::vector of size 9 of a Eigen::Vector3f or mapped Vector3f.
  // If mapping proto messages, we recommend to use the Map<const Vector3f>.
  // For example:
  //
  // using T = Map<const Vector3f>;
  // std::vector<T> vertices;
  // for (const auto& point : message) { // point is a repeated float message.
  //   T p(point.data());
  //   vertices.emplace_back(p);
  // }
  // box.Fit<T>(vertices);
  //
  // The Points must be arranged as 1 + 8 (center keypoint followed by 8 box
  // vertices) vector. This function will overwrite the scale and transformation
  // properties of the class.
  template <typename T = Eigen::Map<const Vector3f>>
  void Fit(const std::vector<T>& vertices);

 private:
  std::vector<Face> faces_;
  std::vector<std::array<int, 2>> edges_;
  std::vector<Vector3f> bounding_box_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_MODULES_OBJECTRON_CALCULATORS_BOX_H_
