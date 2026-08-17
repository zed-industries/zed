#ifndef MEDIAPIPE_CALCULATORS_TENSOR_IMAGE_TO_TENSOR_CONVERTER_GL_UTILS_H_
#define MEDIAPIPE_CALCULATORS_TENSOR_IMAGE_TO_TENSOR_CONVERTER_GL_UTILS_H_

#include "mediapipe/framework/port.h"

#if MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_30

#include <array>
#include <memory>
#include <vector>

#include "mediapipe/framework/port/statusor.h"
#include "mediapipe/gpu/gl_base.h"
#include "mediapipe/gpu/gl_context.h"

namespace mediapipe {

// Intended to override and automatically revert various OpenGL attributes.
// (e.g. overriding texture parameters like GL_TEXTURE_MIN_FILTER,
// GL_TEXTURE_MAG_FILTER, etc.)
class GlOverride {
 public:
  virtual ~GlOverride() = default;
};

// Creates an object that overrides attributes using `glTexParameteri`
// function during construction and reverts them during destruction. See
// `glTexParameteri` for details on @name and @value.
ABSL_MUST_USE_RESULT std::unique_ptr<GlOverride> OverrideGlTexParametri(
    GLenum name, GLint value);

// Creates an object that overrides attributes using `glTexParameterfv`
// function during construction and reverts them during destruction. See
// `glTexParameterfv` for details on @name and @values.
template <int kNumValues>
ABSL_MUST_USE_RESULT std::unique_ptr<GlOverride> OverrideGlTexParameterfv(
    GLenum name, std::array<GLfloat, kNumValues> values);

bool IsGlClampToBorderSupported(const mediapipe::GlContext& gl_context);

}  // namespace mediapipe

#endif  // MEDIAPIPE_OPENGL_ES_VERSION >= MEDIAPIPE_OPENGL_ES_30

#endif  // MEDIAPIPE_CALCULATORS_TENSOR_IMAGE_TO_TENSOR_CONVERTER_GL_UTILS_H_
