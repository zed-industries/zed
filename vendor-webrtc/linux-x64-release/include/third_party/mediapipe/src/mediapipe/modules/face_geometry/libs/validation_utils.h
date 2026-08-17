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

#ifndef MEDIAPIPE_FACE_GEOMETRY_LIBS_VALIDATION_UTILS_H_
#define MEDIAPIPE_FACE_GEOMETRY_LIBS_VALIDATION_UTILS_H_

#include "mediapipe/framework/port/status.h"
#include "mediapipe/modules/face_geometry/protos/environment.pb.h"
#include "mediapipe/modules/face_geometry/protos/face_geometry.pb.h"
#include "mediapipe/modules/face_geometry/protos/geometry_pipeline_metadata.pb.h"
#include "mediapipe/modules/face_geometry/protos/mesh_3d.pb.h"

namespace mediapipe::face_geometry {

// Validates `perspective_camera`.
//
// Near Z must be greater than 0 with a margin of `1e-9`.
// Far Z must be greater than Near Z with a margin of `1e-9`.
// Vertical FOV must be in range (0, 180) with a margin of `1e-9` on the range
// edges.
absl::Status ValidatePerspectiveCamera(
    const PerspectiveCamera& perspective_camera);

// Validates `environment`.
//
// Environment's perspective camera must be valid.
absl::Status ValidateEnvironment(const Environment& environment);

// Validates `mesh_3d`.
//
// Mesh vertex buffer size must a multiple of the vertex size.
// Mesh index buffer size must a multiple of the primitive size.
// All mesh indices must reference an existing mesh vertex.
absl::Status ValidateMesh3d(const Mesh3d& mesh_3d);

// Validates `face_geometry`.
//
// Face mesh must be valid.
// Face pose transformation matrix must be a 4x4 matrix.
absl::Status ValidateFaceGeometry(const FaceGeometry& face_geometry);

// Validates `metadata`.
//
// Canonical face mesh must be valid.
// Procrustes landmark basis must be non-empty.
// All Procrustes basis indices must reference an existing canonical mesh
// vertex.
// All Procrustes basis landmarks must have a non-negative weight.
absl::Status ValidateGeometryPipelineMetadata(
    const GeometryPipelineMetadata& metadata);

// Validates frame dimensions.
//
// Both frame width and frame height must be positive.
absl::Status ValidateFrameDimensions(int frame_width, int frame_height);

}  // namespace mediapipe::face_geometry

#endif  // MEDIAPIPE_FACE_GEOMETRY_LIBS_VALIDATION_UTILS_H_
