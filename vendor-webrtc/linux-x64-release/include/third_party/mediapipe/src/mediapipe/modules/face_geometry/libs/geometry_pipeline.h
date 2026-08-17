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

#ifndef MEDIAPIPE_FACE_GEOMETRY_LIBS_GEOMETRY_PIPELINE_H_
#define MEDIAPIPE_FACE_GEOMETRY_LIBS_GEOMETRY_PIPELINE_H_

#include <memory>
#include <vector>

#include "mediapipe/framework/formats/landmark.pb.h"
#include "mediapipe/framework/port/statusor.h"
#include "mediapipe/modules/face_geometry/protos/environment.pb.h"
#include "mediapipe/modules/face_geometry/protos/face_geometry.pb.h"
#include "mediapipe/modules/face_geometry/protos/geometry_pipeline_metadata.pb.h"

namespace mediapipe::face_geometry {

// Encapsulates a stateless estimator of facial geometry in a Metric space based
// on the normalized face landmarks in the Screen space.
class GeometryPipeline {
 public:
  virtual ~GeometryPipeline() = default;

  // Estimates geometry data for multiple faces.
  //
  // Returns an error status if any of the passed arguments is invalid.
  //
  // The result includes face geometry data for a subset of the input faces,
  // however geometry data for some faces might be missing. This may happen if
  // it'd be unstable to estimate the facial geometry based on a corresponding
  // face landmark list for any reason (for example, if the landmark list is too
  // compact).
  //
  // Each face landmark list must have the same number of landmarks as was
  // passed upon initialization via the canonical face mesh (as a part of the
  // geometry pipeline metadata).
  //
  // Both `frame_width` and `frame_height` must be positive.
  virtual absl::StatusOr<std::vector<FaceGeometry>> EstimateFaceGeometry(
      const std::vector<NormalizedLandmarkList>& multi_face_landmarks,
      int frame_width, int frame_height) const = 0;
};

// Creates an instance of `GeometryPipeline`.
//
// Both `environment` and `metadata` must be valid (for details, please refer to
// the proto message definition comments and/or `validation_utils.h/cc`).
//
// Canonical face mesh (defined as a part of `metadata`) must have the
// `POSITION` and the `TEX_COORD` vertex components.
absl::StatusOr<std::unique_ptr<GeometryPipeline>> CreateGeometryPipeline(
    const Environment& environment, const GeometryPipelineMetadata& metadata);

}  // namespace mediapipe::face_geometry

#endif  // MEDIAPIPE_FACE_GEOMETRY_LIBS_GEOMETRY_PIPELINE_H_
