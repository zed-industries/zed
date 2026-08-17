// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_STATIC_BITMAP_IMAGE_TO_VIDEO_FRAME_COPIER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_STATIC_BITMAP_IMAGE_TO_VIDEO_FRAME_COPIER_H_

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/threading/thread_checker.h"
#include "media/base/video_frame_pool.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/skia/include/gpu/ganesh/GrTypes.h"

namespace media {
class VideoFrame;
class VideoFramePool;
}  // namespace media

namespace blink {

class StaticBitmapImage;
class WebGraphicsContext3DProvider;
class WebGraphicsContext3DProviderWrapper;
class WebGraphicsContext3DVideoFramePool;

class PLATFORM_EXPORT StaticBitmapImageToVideoFrameCopier {
 public:
  using FrameReadyCallback =
      base::OnceCallback<void(scoped_refptr<media::VideoFrame>)>;

  explicit StaticBitmapImageToVideoFrameCopier(
      bool accelerated_frame_pool_enabled);
  ~StaticBitmapImageToVideoFrameCopier();

  WebGraphicsContext3DVideoFramePool* GetAcceleratedVideoFramePool(
      base::WeakPtr<blink::WebGraphicsContext3DProviderWrapper>
          context_provider);

  void Convert(scoped_refptr<StaticBitmapImage> image,
               bool can_discard_alpha,
               base::WeakPtr<blink::WebGraphicsContext3DProviderWrapper>
                   context_provider,
               FrameReadyCallback callback);

 private:
  // Helper functions to read pixel content.
  void ReadARGBPixelsSync(scoped_refptr<StaticBitmapImage> image,
                          FrameReadyCallback callback);
  void ReadARGBPixelsAsync(
      scoped_refptr<StaticBitmapImage> image,
      blink::WebGraphicsContext3DProvider* context_provider,
      FrameReadyCallback callback);
  void ReadYUVPixelsAsync(scoped_refptr<StaticBitmapImage> image,
                          blink::WebGraphicsContext3DProvider* context_provider,
                          FrameReadyCallback callback);
  void OnARGBPixelsReadAsync(scoped_refptr<StaticBitmapImage> image,
                             scoped_refptr<media::VideoFrame> temp_argb_frame,
                             FrameReadyCallback callback,
                             bool success);
  void OnYUVPixelsReadAsync(scoped_refptr<media::VideoFrame> yuv_frame,
                            FrameReadyCallback callback,
                            bool success);
  void OnReleaseMailbox(scoped_refptr<StaticBitmapImage> image);

  media::VideoFramePool frame_pool_;
  std::unique_ptr<WebGraphicsContext3DVideoFramePool> accelerated_frame_pool_;
  bool can_discard_alpha_ = false;
  const bool accelerated_frame_pool_enabled_;

  // Bound to Main Render thread.
  THREAD_CHECKER(main_render_thread_checker_);
  base::WeakPtrFactory<StaticBitmapImageToVideoFrameCopier> weak_ptr_factory_{
      this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_STATIC_BITMAP_IMAGE_TO_VIDEO_FRAME_COPIER_H_
