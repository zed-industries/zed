// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_WEBRTC_VIDEO_FRAME_ADAPTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_WEBRTC_VIDEO_FRAME_ADAPTER_H_

#include "base/feature_list.h"
#include "base/logging.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock.h"
#include "base/time/default_tick_clock.h"
#include "base/time/time.h"
#include "components/viz/common/gpu/raster_context_provider.h"
#include "media/base/video_frame.h"
#include "media/base/video_frame_converter.h"
#include "media/base/video_frame_pool.h"
#include "media/base/video_types.h"
#include "media/capture/video/video_capture_feedback.h"
#include "media/video/gpu_video_accelerator_factories.h"
#include "media/video/renderable_gpu_memory_buffer_video_frame_pool.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc/api/scoped_refptr.h"
#include "third_party/webrtc/api/video/video_frame_buffer.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

// The WebRtcVideoFrameAdapter implements webrtc::VideoFrameBuffer and is backed
// by one or more media::VideoFrames.
// * Upon CropAndScale(), the crop and scale values are soft-applied.
// * Upon GetMappedFrameBuffer(), any outstanding crop and scale is hard-applied
//   before returning the resulting buffer. This also happens on ToI420().
//
// This eliminates any intermediary downscales by only hard-applying what
// actually needs to be mapped.
//
// When crop and scale is hard-applied, the media::VideoFrame of closest size
// that is greater than or equal the requested size is chosen, minimizing
// scaling costs. If a media::VideoFrame exists in the desired size, no scaling
// is needed.
//
// WebRtcVideoFrameAdapter keeps track of which crops and scales were
// hard-applied during its lifetime.

class PLATFORM_EXPORT WebRtcVideoFrameAdapterInterface
    : public webrtc::VideoFrameBuffer {
 public:
  virtual scoped_refptr<media::VideoFrame> getMediaVideoFrame() const = 0;

  // Regardless of the pixel format used internally, kNative is returned
  // indicating that GetMappedFrameBuffer() or ToI420() is required to obtain
  // the pixels.
  webrtc::VideoFrameBuffer::Type type() const override {
    return webrtc::VideoFrameBuffer::Type::kNative;
  }
};

class PLATFORM_EXPORT WebRtcVideoFrameAdapter
    : public WebRtcVideoFrameAdapterInterface {
 public:
  class PLATFORM_EXPORT SharedResources
      : public ThreadSafeRefCounted<SharedResources> {
   public:
    explicit SharedResources(
        media::GpuVideoAcceleratorFactories* gpu_factories);

    // Create frames for requested output format and resolution.
    virtual scoped_refptr<media::VideoFrame> CreateFrame(
        media::VideoPixelFormat format,
        const gfx::Size& coded_size,
        const gfx::Rect& visible_rect,
        const gfx::Size& natural_size,
        base::TimeDelta timestamp);

    // Uses a media::VideoFrameConverter to copy pixel data from `src_frame` to
    // `dest_frame` applying scaling and pixel format conversion as needed.
    // See media::VideoFrameConverter for supported input and output formats.
    virtual media::EncoderStatus ConvertAndScale(
        const media::VideoFrame& src_frame,
        media::VideoFrame& dest_frame);

    virtual scoped_refptr<viz::RasterContextProvider>
    GetRasterContextProvider();

    // Constructs a VideoFrame from a texture by invoking RasterInterface,
    // which would perform a blocking call to a GPU process.
    // The pixel data is copied and may be in ARGB pixel format in some cases,
    // So additional conversion to I420 would be needed.
    virtual scoped_refptr<media::VideoFrame> ConstructVideoFrameFromTexture(
        scoped_refptr<media::VideoFrame> source_frame);

    // Constructs a VideoFrame from a GMB. Unless it's a Windows DXGI GMB,
    // the buffer is mapped to the memory and wrapped by a VideoFrame with no
    // copies. For DXGI buffers a blocking call to GPU process is made to
    // copy pixel data.
    virtual scoped_refptr<media::VideoFrame> ConstructVideoFrameFromGpu(
        scoped_refptr<media::VideoFrame> source_frame);

    // Used to report feedback from an adapter upon destruction.
    void SetFeedback(const media::VideoCaptureFeedback& feedback);

    // Returns the most recently stored feedback.
    media::VideoCaptureFeedback GetFeedback();

   protected:
    friend class ThreadSafeRefCounted<SharedResources>;
    virtual ~SharedResources();

   private:
    media::VideoFramePool pool_;
    media::VideoFramePool pool_for_mapped_frames_;

    std::unique_ptr<media::RenderableGpuMemoryBufferVideoFramePool>
        accelerated_frame_pool_;
    bool disable_gmb_frames_ = false;

    base::Lock context_provider_lock_;
    scoped_refptr<viz::RasterContextProvider> raster_context_provider_
        GUARDED_BY(context_provider_lock_);

    raw_ptr<media::GpuVideoAcceleratorFactories> gpu_factories_;

    // Handles frame conversions. Maintains an internal scratch space buffer.
    media::VideoFrameConverter frame_converter_;

    base::Lock feedback_lock_;

    // Contains feedback from the most recently destroyed Adapter.
    media::VideoCaptureFeedback last_feedback_ GUARDED_BY(feedback_lock_);
  };

  struct PLATFORM_EXPORT ScaledBufferSize {
    ScaledBufferSize(gfx::Rect visible_rect, gfx::Size natural_size);

    bool operator==(const ScaledBufferSize& rhs) const;
    bool operator!=(const ScaledBufferSize& rhs) const;

    // Applies crop-and-scale relative to the current natural size.
    ScaledBufferSize CropAndScale(int offset_x,
                                  int offset_y,
                                  int crop_width,
                                  int crop_height,
                                  int scaled_width,
                                  int scaled_height) const;

    // Cropping relative to the original image.
    gfx::Rect visible_rect;
    // The size (after scaling) of the visible rect.
    gfx::Size natural_size;
  };

  // Implements a soft-applied "view" of the parent WebRtcVideoFrameAdapter. Its
  // size only gets hard-applied if GetMappedFrameBuffer() or ToI420() is
  // called, in which case the result is cached inside the parent.
  class ScaledBuffer : public WebRtcVideoFrameAdapterInterface {
   public:
    ScaledBuffer(scoped_refptr<WebRtcVideoFrameAdapter> parent,
                 ScaledBufferSize size);

    scoped_refptr<media::VideoFrame> getMediaVideoFrame() const override;

    int width() const override { return size_.natural_size.width(); }
    int height() const override { return size_.natural_size.height(); }

    // Obtains a mapped I420 buffer with this ScaledBuffer's size hard-applied.
    // If I420 is not used internally, a conversion happens.
    webrtc::scoped_refptr<webrtc::I420BufferInterface> ToI420() override;

    // Obtains a mapped buffer of this ScaledBuffer's size hard-applied. The
    // resulting buffer's type is the non-kNative type used internally.
    webrtc::scoped_refptr<webrtc::VideoFrameBuffer> GetMappedFrameBuffer(
        webrtc::ArrayView<webrtc::VideoFrameBuffer::Type> types) override;

    // Soft-applies cropping and scaling. The result is another ScaledBuffer.
    webrtc::scoped_refptr<webrtc::VideoFrameBuffer> CropAndScale(
        int offset_x,
        int offset_y,
        int crop_width,
        int crop_height,
        int scaled_width,
        int scaled_height) override;

    std::string storage_representation() const override;

    const ScaledBufferSize& size() const { return size_; }

   private:
    scoped_refptr<WebRtcVideoFrameAdapter> parent_;
    ScaledBufferSize size_;
  };

  explicit WebRtcVideoFrameAdapter(scoped_refptr<media::VideoFrame> frame);
  WebRtcVideoFrameAdapter(
      scoped_refptr<media::VideoFrame> frame,
      scoped_refptr<SharedResources> shared_resources);

  scoped_refptr<media::VideoFrame> getMediaVideoFrame() const override {
    return frame_;
  }

  int width() const override { return frame_->natural_size().width(); }
  int height() const override { return frame_->natural_size().height(); }

  webrtc::scoped_refptr<webrtc::I420BufferInterface> ToI420() override;
  webrtc::scoped_refptr<webrtc::VideoFrameBuffer> GetMappedFrameBuffer(
      webrtc::ArrayView<webrtc::VideoFrameBuffer::Type> types) override;

  // Soft-applies cropping and scaling. The result is a ScaledBuffer.
  webrtc::scoped_refptr<webrtc::VideoFrameBuffer> CropAndScale(
      int offset_x,
      int offset_y,
      int crop_width,
      int crop_height,
      int scaled_width,
      int scaled_height) override;

  // If this exact size has been hard-applied by wrapping or using a pre-scaled
  // media::VideoFrame, the associated media::VideoFrame is returned (the
  // wrapping or original). If this size has not been hard-applied, or it was
  // hard-applied by scaling a previously adapted webrtc::VideoFrameBuffer, then
  // null is returned.
  scoped_refptr<media::VideoFrame> GetAdaptedVideoBufferForTesting(
      const ScaledBufferSize& size);

  std::string storage_representation() const override;

 protected:
  ~WebRtcVideoFrameAdapter() override;

 private:
  struct AdaptedFrame {
    AdaptedFrame(ScaledBufferSize size,
                 scoped_refptr<media::VideoFrame> video_frame,
                 webrtc::scoped_refptr<webrtc::VideoFrameBuffer> frame_buffer)
        : size(std::move(size)),
          video_frame(std::move(video_frame)),
          frame_buffer(std::move(frame_buffer)) {}

    ScaledBufferSize size;
    // If |frame_buffer| was produced without a media::VideoFrame this is null.
    scoped_refptr<media::VideoFrame> video_frame;
    webrtc::scoped_refptr<webrtc::VideoFrameBuffer> frame_buffer;
  };

  webrtc::scoped_refptr<webrtc::VideoFrameBuffer> GetOrCreateFrameBufferForSize(
      const ScaledBufferSize& size);
  AdaptedFrame AdaptBestFrame(const ScaledBufferSize& size) const
      EXCLUSIVE_LOCKS_REQUIRED(adapted_frames_lock_);

  base::Lock adapted_frames_lock_;
  const scoped_refptr<media::VideoFrame> frame_;
  const scoped_refptr<SharedResources> shared_resources_;
  const ScaledBufferSize full_size_;
  // Frames that have been adapted, i.e. that were "hard-applied" and mapped.
  Vector<AdaptedFrame> adapted_frames_ GUARDED_BY(adapted_frames_lock_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBRTC_WEBRTC_VIDEO_FRAME_ADAPTER_H_
