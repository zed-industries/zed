// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_WEBRTC_VIDEO_TRACK_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_WEBRTC_VIDEO_TRACK_SOURCE_H_

#include "base/feature_list.h"
#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/threading/thread_checker.h"
#include "media/base/video_frame_pool.h"
#include "media/capture/video/video_capture_feedback.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/webrtc/webrtc_video_frame_adapter.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/thread_safe_ref_counted.h"
#include "third_party/webrtc/media/base/adapted_video_track_source.h"
#include "third_party/webrtc/rtc_base/timestamp_aligner.h"

namespace media {
class GpuVideoAcceleratorFactories;
}

namespace blink {

// This class implements webrtc's VideoTrackSourceInterface. To pass frames down
// the webrtc video pipeline, each received a media::VideoFrame is converted to
// a webrtc::VideoFrame, taking any adaptation requested by downstream classes
// into account.
class PLATFORM_EXPORT WebRtcVideoTrackSource
    : public webrtc::AdaptedVideoTrackSource {
 public:
  struct FrameAdaptationParams {
    bool should_drop_frame;
    int crop_x;
    int crop_y;
    int crop_width;
    int crop_height;
    int scale_to_width;
    int scale_to_height;
  };

  WebRtcVideoTrackSource(
      bool is_screencast,
      std::optional<bool> needs_denoising,
      media::VideoCaptureFeedbackCB feedback_callback,
      base::RepeatingClosure request_refresh_frame_callback,
      media::GpuVideoAcceleratorFactories* gpu_factories,
      scoped_refptr<WebRtcVideoFrameAdapter::SharedResources> = nullptr);
  WebRtcVideoTrackSource(const WebRtcVideoTrackSource&) = delete;
  WebRtcVideoTrackSource& operator=(const WebRtcVideoTrackSource&) = delete;
  ~WebRtcVideoTrackSource() override;

  void SetCustomFrameAdaptationParamsForTesting(
      const FrameAdaptationParams& params);

  void SetSinkWantsForTesting(const webrtc::VideoSinkWants& sink_wants);

  SourceState state() const override;

  bool remote() const override;
  bool is_screencast() const override;
  std::optional<bool> needs_denoising() const override;
  void OnFrameCaptured(scoped_refptr<media::VideoFrame> frame);
  void OnNotifyFrameDropped();

  using webrtc::VideoTrackSourceInterface::AddOrUpdateSink;
  using webrtc::VideoTrackSourceInterface::RemoveSink;
  void RequestRefreshFrame() override;

  void Dispose();

 private:
  // Need a separate ref-counted part for posting map callbacks because
  // WebrtcVideoTrackSource is used on network thread and destroyed on the
  // main thread and it's risky to move the destruction to the network
  // thread too. Also, callbacks must not extend the lifetime of the object.
  // Therefore can't use WeakPtr nor scoped_refptr, because the first one
  // requires destruction on the same thread, while the second one extends
  // the lifetime of the object.
  class CallbackProxy : public ThreadSafeRefCounted<CallbackProxy> {
   public:
    explicit CallbackProxy(WebRtcVideoTrackSource* track_source);
    void ProcessMappedFrame(int64_t id,
                            scoped_refptr<media::VideoFrame> mapped_frame);
    void Reset();

   private:
    base::Lock lock_;
    raw_ptr<WebRtcVideoTrackSource> parent_ GUARDED_BY(lock_);
  };

  void SendFeedback();

  FrameAdaptationParams ComputeAdaptationParams(int width,
                                                int height,
                                                int64_t time_us);

  // Applies adaptation parameters and updates all the metadata: e.g.
  // `update_rect` and timestamps. Calls `DeliverFrame()`.
  void ComputeMetadataAndDeliverFrame(scoped_refptr<media::VideoFrame> frame,
                                      int64_t time_posted_us);

  // Delivers |frame| to base class method
  // webrtc::AdaptedVideoTrackSource::OnFrame(). If the cropping (given via
  // |frame->visible_rect()|) has changed since the last delivered frame,
  // the whole frame is marked as updated. |timestamp_us| is
  // |frame->timestamp()| in Microseconds but clipped to ensure that it
  // doesn't exceed the current system time. However,
  // |capture_time_identifier| is just |frame->timestamp()|.
  // |reference_time| corresponds to a monotonically increasing clock time
  // and represents the time when the frame was captured. Not all platforms
  // provide the "true" sample capture time in |reference_time| but might
  // instead use a somewhat delayed (by the time it took to capture the
  // frame) version of it.
  void DeliverFrame(scoped_refptr<media::VideoFrame> frame,
                    std::optional<gfx::Rect> update_rect,
                    int64_t timestamp_us,
                    std::optional<webrtc::Timestamp> presentation_timestamp,
                    std::optional<webrtc::Timestamp> reference_time);

  // Receives result of asynchronous mapping of a frame.
  // It will then be passed to `ComputeMetadataAndDeliverFrame()`;
  void ProcessMappedFrame(int64_t id,
                          scoped_refptr<media::VideoFrame> mapped_frame);

  void TryProcessPendingFrames();

  // This checks if the colorspace information should be passed to webrtc. Avoid
  // sending unknown or unnecessary color space.
  bool ShouldSetColorSpace(const gfx::ColorSpace& color_space);

  // |thread_checker_| is bound to the libjingle worker thread.
  THREAD_CHECKER(thread_checker_);
  scoped_refptr<WebRtcVideoFrameAdapter::SharedResources> adapter_resources_;
  // State for the timestamp translation.
  webrtc::TimestampAligner timestamp_aligner_;

  const bool is_screencast_;
  const std::optional<bool> needs_denoising_;

  // Stores the accumulated value of CAPTURE_UPDATE_RECT in case that frames
  // are dropped.
  std::optional<gfx::Rect> accumulated_update_rect_;
  std::optional<int> previous_capture_counter_;
  gfx::Rect cropping_rect_of_previous_delivered_frame_;
  gfx::Size natural_size_of_previous_delivered_frame_;

  std::optional<FrameAdaptationParams>
      custom_frame_adaptation_params_for_testing_;

  const media::VideoCaptureFeedbackCB feedback_callback_;
  const base::RepeatingClosure request_refresh_frame_callback_;

  struct PendingFrame {
    scoped_refptr<media::VideoFrame> frame;
    int64_t time_posted_us;
    int64_t id;
    bool can_be_delivered = false;
  };
  WTF::Deque<PendingFrame> pending_frames_;

  scoped_refptr<CallbackProxy> callback_proxy_;

  int64_t next_frame_id_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_WEBRTC_VIDEO_TRACK_SOURCE_H_
