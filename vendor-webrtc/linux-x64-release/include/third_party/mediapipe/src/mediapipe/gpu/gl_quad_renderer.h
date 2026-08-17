// Copyright 2019 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_GPU_GL_QUAD_RENDERER_H_
#define MEDIAPIPE_GPU_GL_QUAD_RENDERER_H_

#include <optional>

#include "absl/status/status.h"
#include "mediapipe/gpu/gl_base.h"
#include "mediapipe/gpu/scale_mode.pb.h"

namespace mediapipe {

// Valid rotation values. Counterclockwise.
enum class FrameRotation { kNone, k90, k180, k270 };

enum class FrameScaleMode {
  // Stretch the frame to the exact provided output dimensions.
  kStretch = 0,
  // Scale the frame up to fit the drawing area, preserving aspect ratio; may
  // letterbox.
  kFit,
  // Scale the frame up to fill the drawing area, preserving aspect ratio; may
  // crop.
  kFillAndCrop,
};

// Converts scale_mode.proto enum value typically used in calculator options
// to FrameScaleMode value.
FrameScaleMode FrameScaleModeFromProto(ScaleMode_Mode proto_scale_mode,
                                       FrameScaleMode default_mode);

// This is a utility class containing some common code to render a texture on
// a quadrilateral with aspect ratio correction, (quarter-circle) rotation,
// mirroring and flipping. It is used in various places where rendering is
// done.
class QuadRenderer {
 public:
  QuadRenderer() {}
  // Creates the rendering program. Must be called within the GL context that
  // will be used for rendering.
  absl::Status GlSetup();
  // Creates the rendering program. Must be called within the GL context that
  // will be used for rendering.
  // This version allows you to customize the fragment shader.
  absl::Status GlSetup(const GLchar* custom_frag_shader,
                       const std::vector<const GLchar*>& custom_frame_uniforms);
  // Renders the texture bound to texture unit 1 onto the current viewport.
  // Note: mirroring and flipping are handled differently, by design.
  // - flip_texture is meant to be used when the texture image's rows are stored
  //   top-to-bottom. The OpenGL custom is to store them bottom-to-top, but this
  //   is the opposite of the way most other graphics APIs and formats represent
  //   images, so having flipped textures is quite common.
  //   Because this is a property of the input texture, flipping is applied
  //   BEFORE rotation.
  // - flip_horizontal is meant to be used to flip the output image
  //   horizontally. This is especially useful for the front-facing camera on
  //   smartphones. This flipping is applied AFTER rotation, because that is
  //   what's needed for the front-camera use case.
  // - flip_vertical is meant to be used to flip the output image vertically.
  //   This flipping is applied AFTER rotation.
  absl::Status GlRender(float frame_width, float frame_height, float view_width,
                        float view_height, FrameScaleMode scale_mode,
                        FrameRotation rotation, bool flip_horizontal,
                        bool flip_vertical, bool flip_texture) const;
  // Deletes the rendering program. Must be called withn the GL context where
  // it was created.
  void GlTeardown();

 private:
  void UpdateVertices(FrameRotation rotation) const;

  GLuint program_ = 0;
  GLint scale_unif_ = -1;
  std::vector<GLint> frame_unifs_;
  GLuint vao_ = 0;          // vertex array object
  GLuint vbo_[2] = {0, 0};  // for vertex buffer storage
  mutable std::optional<FrameRotation> rotation_;
};

absl::Status FrameRotationFromInt(FrameRotation* rotation, int degrees_ccw);

// Input degrees must be one of: [0, 90, 180, 270].
FrameRotation FrameRotationFromDegrees(int degrees_ccw);

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GL_QUAD_RENDERER_H_
