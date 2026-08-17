// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_XR_WEBGL_DRAWING_BUFFER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_XR_WEBGL_DRAWING_BUFFER_H_

#include "base/threading/platform_thread.h"
#include "cc/layers/texture_layer_client.h"
#include "gpu/command_buffer/client/client_shared_image.h"
#include "gpu/command_buffer/client/gles2_interface.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

class DrawingBuffer;
class StaticBitmapImage;

class PLATFORM_EXPORT XRWebGLDrawingBuffer
    : public RefCounted<XRWebGLDrawingBuffer> {
 public:
  static scoped_refptr<XRWebGLDrawingBuffer> Create(DrawingBuffer*,
                                                    GLuint framebuffer,
                                                    const gfx::Size&,
                                                    bool want_alpha_channel,
                                                    bool want_depth_buffer,
                                                    bool want_stencil_buffer,
                                                    bool want_antialiasing);

  gpu::gles2::GLES2Interface* ContextGL();
  bool ContextLost();

  const gfx::Size& size() const { return size_; }

  bool antialias() const { return anti_aliasing_mode_ != kNone; }
  bool depth() const { return depth_; }
  bool stencil() const { return stencil_; }
  bool alpha() const { return alpha_; }

  void Resize(const gfx::Size&);

  scoped_refptr<StaticBitmapImage> TransferToStaticBitmapImage();

  void UseSharedBuffer(
      const scoped_refptr<gpu::ClientSharedImage>& buffer_shared_image,
      const gpu::SyncToken& buffer_sync_token);
  void DoneWithSharedBuffer();

  GLuint GetCurrentColorBufferTextureId();

  // Prepare for destruction by breaking reference loops. This must be called to
  // avoid memory leaks, drawing buffer and color buffers are refcounted and
  // store references to each other.
  void BeginDestruction();

 private:
  struct PLATFORM_EXPORT ColorBuffer
      : public ThreadSafeRefCounted<ColorBuffer> {
    ColorBuffer(base::WeakPtr<XRWebGLDrawingBuffer>,
                const gfx::Size&,
                scoped_refptr<gpu::ClientSharedImage> shared_image,
                std::unique_ptr<gpu::SharedImageTexture> texture);
    ColorBuffer(const ColorBuffer&) = delete;
    ColorBuffer& operator=(const ColorBuffer&) = delete;

    // Begin/end the scoped access of |texture|.
    void BeginAccess();
    void EndAccess();

    GLuint texture_id() { return scoped_access_->texture_id(); }

    void CleanUp();

    // The thread on which the ColorBuffer is created and the DrawingBuffer is
    // bound to.
    const base::PlatformThreadRef owning_thread_ref;

    // The owning XRWebGLDrawingBuffer. Note that DrawingBuffer is explicitly
    // destroyed by the BeginDestruction method, which will eventually drain all
    // of its ColorBuffers.
    base::WeakPtr<XRWebGLDrawingBuffer> drawing_buffer;
    const gfx::Size size;

    // The client shared image backing this color buffer.
    scoped_refptr<gpu::ClientSharedImage> shared_image;

    // The sync token for when this buffer was sent to the compositor.
    gpu::SyncToken produce_sync_token;

    // The sync token for when this buffer was received back from the
    // compositor.
    gpu::SyncToken receive_sync_token;

   private:
    friend class ThreadSafeRefCounted<ColorBuffer>;
    ~ColorBuffer() = default;

    // The texture that imports the shared image into the DrawingBuffer's
    // context.
    std::unique_ptr<gpu::SharedImageTexture> texture_;
    std::unique_ptr<gpu::SharedImageTexture::ScopedAccess> scoped_access_;
  };

  XRWebGLDrawingBuffer(DrawingBuffer*,
                       GLuint framebuffer,
                       bool discard_framebuffer_supported,
                       bool want_alpha_channel,
                       bool want_depth_buffer,
                       bool want_stencil_buffer);

  bool Initialize(const gfx::Size&, bool use_multisampling);

  gfx::Size AdjustSize(const gfx::Size&);

  scoped_refptr<ColorBuffer> CreateColorBuffer();
  scoped_refptr<ColorBuffer> CreateOrRecycleColorBuffer();

  bool WantExplicitResolve() const;
  void BindAndResolveDestinationFramebuffer();
  void SwapColorBuffers();

  void ClearBoundFramebuffer();

  static void NotifyMailboxReleased(scoped_refptr<ColorBuffer>,
                                    const gpu::SyncToken&,
                                    bool lost_resource);
  void MailboxReleased(scoped_refptr<ColorBuffer>, bool lost_resource);

  // Reference to the DrawingBuffer that owns the GL context for this object.
  scoped_refptr<DrawingBuffer> drawing_buffer_;

  const GLuint framebuffer_ = 0;
  GLuint resolved_framebuffer_ = 0;
  GLuint multisample_renderbuffer_ = 0;
  scoped_refptr<ColorBuffer> back_color_buffer_;
  scoped_refptr<ColorBuffer> front_color_buffer_;
  GLuint depth_stencil_buffer_ = 0;
  gfx::Size size_;

  // Valid for shared buffer mode from UseSharedBuffer until
  // DoneWithSharedBuffer.
  std::unique_ptr<gpu::SharedImageTexture> shared_buffer_texture_;
  std::unique_ptr<gpu::SharedImageTexture::ScopedAccess>
      shared_buffer_scoped_access_;

  // Checking framebuffer completeness is extremely expensive, it's basically a
  // glFinish followed by a synchronous wait for a reply. Do so only once per
  // code path, and only in DCHECK mode.
  bool framebuffer_complete_checked_for_resize_ = false;
  bool framebuffer_complete_checked_for_swap_ = false;
  bool framebuffer_complete_checked_for_sharedbuffer_ = false;

  // Color buffers that were released by the XR compositor can be used again.
  Deque<scoped_refptr<ColorBuffer>> recycled_color_buffer_queue_;

  bool discard_framebuffer_supported_;
  bool depth_;
  bool stencil_;
  bool alpha_;

  enum AntialiasingMode {
    kNone,
    kMSAAImplicitResolve,
    kMSAAExplicitResolve,
  };

  AntialiasingMode anti_aliasing_mode_ = kNone;

  int max_texture_size_ = 0;
  int sample_count_ = 0;

  base::flat_set<scoped_refptr<ColorBuffer>> exported_color_buffers_;

  base::WeakPtrFactory<XRWebGLDrawingBuffer> weak_factory_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_XR_WEBGL_DRAWING_BUFFER_H_
