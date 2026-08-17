// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_WEB_GRAPHICS_CONTEXT_3D_VIDEO_FRAME_POOL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_WEB_GRAPHICS_CONTEXT_3D_VIDEO_FRAME_POOL_H_

#include "base/atomic_sequence_num.h"
#include "base/cancelable_callback.h"
#include "base/functional/callback.h"
#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/skia/include/gpu/ganesh/GrTypes.h"

namespace gfx {
class ColorSpace;
class Size;
}  // namespace gfx

namespace gpu {
class ClientSharedImage;
struct SyncToken;
namespace raster {
class RasterInterface;
}  // namespace raster
}  // namespace gpu

namespace media {
class RenderableGpuMemoryBufferVideoFramePool;
class VideoFrame;
}  // namespace media

namespace blink {

class WebGraphicsContext3DProviderWrapper;

// A video frame pool that will use a WebGraphicsContext3D to do an accelerated
// RGB to YUV conversion directly into a GpuMemoryBuffer-backed
// media::VideoFrame.
class PLATFORM_EXPORT WebGraphicsContext3DVideoFramePool {
 public:
  // This constructor is valid only on the main thread.
  explicit WebGraphicsContext3DVideoFramePool(
      base::WeakPtr<WebGraphicsContext3DProviderWrapper> weak_context_provider);
  ~WebGraphicsContext3DVideoFramePool();

  gpu::raster::RasterInterface* GetRasterInterface() const;

  using FrameReadyCallback =
      base::OnceCallback<void(scoped_refptr<media::VideoFrame>)>;

  // On success, this function will return the completion sync token for the
  // read operations on `src_shared_image` and will call the specified
  // FrameCallback with the resulting VideoFrame when the frame is ready. On
  // failure this will return std::nullopt. The resulting VideoFrame will always
  // be NV12. Note: If the YUV to RGB matrix of `dst_color_space` is not Rec601,
  // then this function will use the matrix for Rec709 (it supports no other
  // values). See https://crbug.com/skia/12545.
  std::optional<gpu::SyncToken> CopyRGBATextureToVideoFrame(
      const gfx::Size& src_size,
      scoped_refptr<gpu::ClientSharedImage> src_shared_image,
      const gpu::SyncToken& acquire_sync_token,
      const gfx::ColorSpace& dst_color_space,
      FrameReadyCallback callback);

  // Same as CopyRGBATextureToVideoFrame, but obtains the arguments from
  // src_video_frame, and applies relevant metadata to the resulting VideoFrame.
  // Always discards alpha. DCHECKs that src_video_frame is backed by a single
  // RGB texture.
  bool ConvertVideoFrame(scoped_refptr<media::VideoFrame> src_video_frame,
                         const gfx::ColorSpace& dst_color_space,
                         FrameReadyCallback callback);

  // This is a helper function to get whether GpuMemoryBuffer readback from
  // texture is enabled.
  static bool IsGpuMemoryBufferReadbackFromTextureEnabled();

 private:
  base::WeakPtr<blink::WebGraphicsContext3DProviderWrapper>
      weak_context_provider_;
  const std::unique_ptr<media::RenderableGpuMemoryBufferVideoFramePool> pool_;
  base::AtomicSequenceNumber trace_flow_seqno_;

  WTF::Deque<std::unique_ptr<base::CancelableOnceClosure>>
      pending_gpu_completion_callbacks_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_WEB_GRAPHICS_CONTEXT_3D_VIDEO_FRAME_POOL_H_
