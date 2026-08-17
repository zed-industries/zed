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

#ifndef MEDIAPIPE_GPU_GL_TEXTURE_VIEW_H_
#define MEDIAPIPE_GPU_GL_TEXTURE_VIEW_H_

#include <functional>
#include <memory>
#include <utility>

#include "mediapipe/gpu/gl_base.h"
#include "mediapipe/gpu/gpu_buffer_storage.h"

namespace mediapipe {

class GlContext;

class GlTextureView {
 public:
  GlTextureView() {}
  ~GlTextureView() { Release(); }
  GlTextureView(const GlTextureView&) = delete;
  GlTextureView(GlTextureView&& other) { *this = std::move(other); }
  GlTextureView& operator=(const GlTextureView&) = delete;
  GlTextureView& operator=(GlTextureView&& other) {
    DoneWriting();
    if (detach_) detach_(*this);
    gl_context_ = other.gl_context_;
    target_ = other.target_;
    name_ = other.name_;
    width_ = other.width_;
    height_ = other.height_;
    plane_ = other.plane_;
    detach_ = std::exchange(other.detach_, nullptr);
    done_writing_ = std::exchange(other.done_writing_, nullptr);
    return *this;
  }

  GlContext* gl_context() const { return gl_context_; }
  int width() const { return width_; }
  int height() const { return height_; }
  GLenum target() const { return target_; }
  GLuint name() const { return name_; }
  int plane() const { return plane_; }

  using DetachFn = std::function<void(GlTextureView&)>;
  using DoneWritingFn = std::function<void(const GlTextureView&)>;

 private:
  friend class GlTextureBuffer;
  friend class GpuBufferStorageCvPixelBuffer;
  friend class GpuBufferStorageAhwb;
  GlTextureView(GlContext* context, GLenum target, GLuint name, int width,
                int height, int plane, DetachFn detach,
                DoneWritingFn done_writing)
      : gl_context_(context),
        target_(target),
        name_(name),
        width_(width),
        height_(height),
        plane_(plane),
        detach_(std::move(detach)),
        done_writing_(std::move(done_writing)) {}

  // TODO: remove this friend declaration.
  friend class GlTexture;

  void Release();

  // TODO: make this non-const.
  void DoneWriting() const;

  GlContext* gl_context_ = nullptr;
  GLenum target_ = GL_TEXTURE_2D;
  GLuint name_ = 0;
  // Note: when scale is not 1, we still give the nominal size of the image.
  int width_ = 0;
  int height_ = 0;
  int plane_ = 0;
  DetachFn detach_;
  mutable DoneWritingFn done_writing_;
};

namespace internal {

template <>
class ViewProvider<GlTextureView> {
 public:
  virtual ~ViewProvider() = default;
  // Note that the view type is encoded in an argument to allow overloading,
  // so a storage class can implement GetRead/WriteView for multiple view types.
  // We cannot use a template function because it cannot be virtual; we want to
  // have a virtual function here to enforce that different storages supporting
  // the same view implement the same signature.
  // Note that we allow different views to have custom signatures, providing
  // additional view-specific arguments that may be needed.
  virtual GlTextureView GetReadView(types<GlTextureView>, int plane) const = 0;
  virtual GlTextureView GetWriteView(types<GlTextureView>, int plane) = 0;
};

}  // namespace internal
}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GL_TEXTURE_VIEW_H_
