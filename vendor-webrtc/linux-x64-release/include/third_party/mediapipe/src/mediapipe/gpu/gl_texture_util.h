#ifndef MEDIAPIPE_GPU_GL_TEXTURE_UTIL_H_
#define MEDIAPIPE_GPU_GL_TEXTURE_UTIL_H_

#include "mediapipe/gpu/gl_base.h"
#include "mediapipe/gpu/gl_texture_view.h"

namespace mediapipe {

// Copies a texture to another.
// Assumes a framebuffer is already set up
void CopyGlTexture(const GlTextureView& src, GlTextureView& dst);

// Fills a texture with a color.
void FillGlTextureRgba(GlTextureView& view, float r, float g, float b, float a);

// RAII class to set up a temporary framebuffer. Mainly for test use.
class TempGlFramebuffer {
 public:
  TempGlFramebuffer() {
    glGenFramebuffers(1, &framebuffer_);
    glBindFramebuffer(GL_FRAMEBUFFER, framebuffer_);
  }
  ~TempGlFramebuffer() {
    glBindFramebuffer(GL_FRAMEBUFFER, 0);
    glDeleteFramebuffers(1, &framebuffer_);
  }

 private:
  GLuint framebuffer_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GL_TEXTURE_UTIL_H_
