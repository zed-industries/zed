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

// Consider this file an implementation detail. None of this is part of the
// public API.

#ifndef MEDIAPIPE_GPU_GL_TEXTURE_BUFFER_H_
#define MEDIAPIPE_GPU_GL_TEXTURE_BUFFER_H_

#include <cstddef>
#include <functional>
#include <memory>

#include "absl/base/thread_annotations.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/formats/image_frame.h"
#include "mediapipe/gpu/gl_base.h"
#include "mediapipe/gpu/gl_context.h"
#include "mediapipe/gpu/gl_texture_view.h"
#include "mediapipe/gpu/gpu_buffer_format.h"
#include "mediapipe/gpu/gpu_buffer_storage.h"

namespace mediapipe {

class GlCalculatorHelperImpl;

// Implements a GPU memory buffer as an OpenGL texture. For internal use.
class GlTextureBuffer
    : public internal::GpuBufferStorageImpl<
          GlTextureBuffer, internal::ViewProvider<GlTextureView>>,
      public std::enable_shared_from_this<GlTextureBuffer> {
 public:
  // This is called when the texture buffer is deleted. It is passed a sync
  // token created at that time on the GlContext. If the GlTextureBuffer has
  // been created from a texture not owned by MediaPipe, the sync token can be
  // used to wait until a point when it is certain that MediaPipe's GPU tasks
  // are done reading from the texture. This is improtant if the code outside
  // of MediaPipe is going to reuse the texture.
  using DeletionCallback =
      std::function<void(std::shared_ptr<GlSyncPoint> sync_token)>;

  // Wraps an existing texture, but does not take ownership of it.
  // deletion_callback is invoked when the GlTextureBuffer is released, so
  // the caller knows that the texture is no longer in use.
  // The commands producing the texture are assumed to be completed at the
  // time of this call. If not, call Updated on the result.
  static std::unique_ptr<GlTextureBuffer> Wrap(
      GLenum target, GLuint name, int width, int height, GpuBufferFormat format,
      DeletionCallback deletion_callback);

  // Same as Wrap above, but saves the given context for future use.
  static std::unique_ptr<GlTextureBuffer> Wrap(
      GLenum target, GLuint name, int width, int height, GpuBufferFormat format,
      std::shared_ptr<GlContext> context, DeletionCallback deletion_callback);

  // Creates a texture of dimensions width x height and allocates space for it.
  // If data is provided, it is uploaded to the texture; otherwise, it can be
  // provided later via glTexSubImage2D.
  static std::unique_ptr<GlTextureBuffer> Create(int width, int height,
                                                 GpuBufferFormat format,
                                                 const void* data = nullptr,
                                                 int alignment = 4);

  // Create a texture with a copy of the data in image_frame.
  static std::unique_ptr<GlTextureBuffer> Create(const ImageFrame& image_frame);

  static std::unique_ptr<GlTextureBuffer> Create(
      const internal::GpuBufferSpec& spec) {
    return Create(spec.width, spec.height, spec.format);
  }

  // Wraps an existing texture, but does not take ownership of it.
  // deletion_callback is invoked when the GlTextureBuffer is released, so
  // the caller knows that the texture is no longer in use.
  // The commands producing the texture are assumed to be completed at the
  // time of this call. If not, call Updated on the result.
  GlTextureBuffer(GLenum target, GLuint name, int width, int height,
                  GpuBufferFormat format, DeletionCallback deletion_callback,
                  std::shared_ptr<GlContext> producer_context = nullptr);
  ~GlTextureBuffer();

  // Included to support nativeGetGpuBuffer* in Java.
  // TODO: turn into a single call?
  GLuint name() const { return name_; }
  GLenum target() const { return target_; }
  int width() const override { return width_; }
  int height() const override { return height_; }
  GpuBufferFormat format() const override { return format_; }

  GlTextureView GetReadView(internal::types<GlTextureView>,
                            int plane) const override;
  GlTextureView GetWriteView(internal::types<GlTextureView>,
                             int plane) override;

  // If this texture is going to be used outside of the context that produced
  // it, this method should be called to ensure that its updated contents are
  // available. When this method returns, all changed made before the call to
  // Updated have become visible.
  // This is necessary because texture changes are not synchronized across
  // contexts in a sharegroup.
  // NOTE: This blocks the current CPU thread and makes the changes visible
  // to the CPU. If you want to access the data via OpenGL, use WaitOnGpu
  // instead.
  void WaitUntilComplete() const;

  // Call this method to synchronize the current GL context with the texture's
  // producer. This will not block the current CPU thread, but will ensure that
  // subsequent GL commands see the texture in its complete status, with all
  // rendering done on the GPU by the generating context.
  void WaitOnGpu() const;

  // Informs the buffer that its contents are going to be overwritten.
  // This invalidates the current sync token.
  // NOTE: this must be called on the context that will become the new
  // producer.
  void Reuse();

  // Informs the buffer that its contents have been updated.
  // The provided sync token marks the point when the producer has finished
  // writing the new contents.
  void Updated(std::shared_ptr<GlSyncPoint> prod_token);

  // Informs the buffer that a consumer has finished reading from it.
  void DidRead(std::shared_ptr<GlSyncPoint> cons_token) const;

  // Waits for all pending consumers to finish accessing the current content
  // of the texture. This (preferably the OnGpu version) should be called
  // before overwriting the texture's contents.
  void WaitForConsumers();
  void WaitForConsumersOnGpu();

  // Returns the GL context this buffer was created with.
  const std::shared_ptr<GlContext>& GetProducerContext() {
    return producer_context_;
  }

#if MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER
  static constexpr bool kDisableGpuBufferRegistration = true;
#endif  // MEDIAPIPE_GPU_BUFFER_USE_CV_PIXEL_BUFFER

 private:
  // Creates a texture of dimensions width x height and allocates space for it.
  // If data is provided, it is uploaded to the texture; otherwise, it can be
  // provided later via glTexSubImage2D.
  // Returns true on success.
  bool CreateInternal(const void* data, int alignment = 4);

  void ViewDoneWriting(const GlTextureView& view);

  friend class GlCalculatorHelperImpl;

  GLuint name_ = 0;
  const int width_ = 0;
  const int height_ = 0;
  const GpuBufferFormat format_ = GpuBufferFormat::kUnknown;
  const GLenum target_ = GL_TEXTURE_2D;
  // Token tracking changes to this texture. Used by WaitUntilComplete.
  std::shared_ptr<GlSyncPoint> producer_sync_;
  mutable absl::Mutex consumer_sync_mutex_;
  // Tokens tracking the point when consumers finished using this texture.
  mutable std::unique_ptr<GlMultiSyncPoint> consumer_multi_sync_
      ABSL_GUARDED_BY(consumer_sync_mutex_) =
          std::make_unique<GlMultiSyncPoint>();
  DeletionCallback deletion_callback_;
  std::shared_ptr<GlContext> producer_context_;
};

// Reads `texture_view` into `output`.
// NOTE: It's clients responsibility to allocate `output` properly and provide
// the right `size`.
// NOTE: Must be invoked on a thread with GL context.
void ReadTexture(GlContext& ctx, const GlTextureView& texture_view,
                 GpuBufferFormat format, void* output, size_t size);

using GlTextureBufferSharedPtr = std::shared_ptr<GlTextureBuffer>;

}  // namespace mediapipe

#endif  // MEDIAPIPE_GPU_GL_TEXTURE_BUFFER_H_
