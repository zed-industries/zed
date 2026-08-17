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

#ifndef MEDIAPIPE_MODULES_FACE_GEOMETRY_LIBS_EFFECT_RENDERER_H_
#define MEDIAPIPE_MODULES_FACE_GEOMETRY_LIBS_EFFECT_RENDERER_H_

#include <memory>
#include <vector>

#include "absl/types/optional.h"
#include "mediapipe/framework/formats/image_frame.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/port/statusor.h"
#include "mediapipe/gpu/gl_base.h"
#include "mediapipe/modules/face_geometry/protos/environment.pb.h"
#include "mediapipe/modules/face_geometry/protos/face_geometry.pb.h"
#include "mediapipe/modules/face_geometry/protos/mesh_3d.pb.h"

namespace mediapipe::face_geometry {

// Encapsulates a stateful face effect renderer.
class EffectRenderer {
 public:
  virtual ~EffectRenderer() = default;

  // Renders a face effect based on the multiple facial geometries.
  //
  // Must be called in the same GL context as was used upon initialization.
  //
  // Each of the `multi_face_geometry` must be valid (for details, please refer
  // to the proto message definition comments and/or `validation_utils.h/cc`).
  // Additionally, all face mesh index buffer elements must fit into the
  // `uint16` type in order to be renderable.
  //
  // Both `frame_width` and `frame_height` must be positive.
  //
  // Both `src_texture_name` and `dst_texture_name` must be positive and
  // reference existing OpenGL textures in the current context. They should also
  // reference different textures as the in-place effect rendering is not yet
  // supported.
  virtual absl::Status RenderEffect(
      const std::vector<FaceGeometry>& multi_face_geometry,
      int frame_width,            //
      int frame_height,           //
      GLenum src_texture_target,  //
      GLuint src_texture_name,    //
      GLenum dst_texture_target,  //
      GLuint dst_texture_name) = 0;
};

// Creates an instance of `EffectRenderer`.
//
// `effect_mesh_3d` defines a rigid 3d mesh which is "attached" to the face and
// is driven by the face pose transformation matrix. If is not present, the
// runtime face mesh will be used as the effect mesh - this mode is handy for
// facepaint effects. In both rendering modes, the face mesh is first rendered
// as an occluder straight into the depth buffer. This step helps to create a
// more believable effect via hiding invisible elements behind the face surface.
//
// `effect_texture` defines the color texture to be rendered on top of the
// effect mesh. Please be aware about the difference between the CPU texture
// memory layout and the GPU texture sampler coordinate space. This renderer
// follows conventions discussed here: https://open.gl/textures
//
// Must be called in the same GL context as will be used for rendering.
//
// Both `environment` and `effect_mesh_3d` (is present) must be valid (for
// details, please refer to the proto message definition comments and/or
// `validation_utils.h/cc`). Additionally, `effect_mesh_3d`s index buffer
// elements must fit into the `uint16` type in order to be renderable.
//
// `effect_texture` must have positive dimensions. Its format must be either
// `SRGB` or `SRGBA`. Its memory must be aligned for GL usage.
absl::StatusOr<std::unique_ptr<EffectRenderer>> CreateEffectRenderer(
    const Environment& environment,                //
    const absl::optional<Mesh3d>& effect_mesh_3d,  //
    ImageFrame&& effect_texture);

}  // namespace mediapipe::face_geometry

#endif  // MEDIAPIPE_MODULES_FACE_GEOMETRY_LIBS_EFFECT_RENDERER_H_
