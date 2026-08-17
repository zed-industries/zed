// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_VIDEO_SINK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_VIDEO_SINK_H_

#include "base/memory/weak_ptr.h"
#include "media/base/video_frame.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/platform/media/video_capture.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_sink.h"

namespace blink {

class MockMediaStreamVideoSink : public MediaStreamVideoSink {
 public:
  MockMediaStreamVideoSink();
  ~MockMediaStreamVideoSink() override;

  void ConnectToTrack(const WebMediaStreamTrack& track) {
    MediaStreamVideoSink::ConnectToTrack(track, GetDeliverFrameCB(),
                                         MediaStreamVideoSink::IsSecure::kYes,
                                         uses_alpha_);
  }

  void ConnectEncodedToTrack(const WebMediaStreamTrack& track) {
    MediaStreamVideoSink::ConnectEncodedToTrack(
        track, GetDeliverEncodedVideoFrameCB());
  }

  void ConnectToTrackWithCallback(const WebMediaStreamTrack& track,
                                  const VideoCaptureDeliverFrameCB& callback) {
    MediaStreamVideoSink::ConnectToTrack(
        track, callback, MediaStreamVideoSink::IsSecure::kYes, uses_alpha_);
  }

  using MediaStreamVideoSink::DisconnectEncodedFromTrack;
  using MediaStreamVideoSink::DisconnectFromTrack;

  void OnReadyStateChanged(WebMediaStreamSource::ReadyState state) override;
  void OnEnabledChanged(bool enabled) override;
  void OnContentHintChanged(
      WebMediaStreamTrack::ContentHintType content_hint) override;
  MOCK_METHOD(void,
              OnVideoConstraintsChanged,
              (std::optional<double>, std::optional<double>),
              (override));

  // Triggered when OnVideoFrame(scoped_refptr<media::VideoFrame> frame)
  // is called.
  MOCK_METHOD(void, OnVideoFrame, (base::TimeTicks));
  MOCK_METHOD(void, OnEncodedVideoFrame, (base::TimeTicks));

  // Triggered when a frame is dropped.
  MOCK_METHOD(void, OnNotifyFrameDropped, (media::VideoCaptureFrameDropReason));

  VideoCaptureDeliverFrameCB GetDeliverFrameCB();
  EncodedVideoFrameCB GetDeliverEncodedVideoFrameCB();
  VideoCaptureNotifyFrameDroppedCB GetNotifyFrameDroppedCB();

  int number_of_frames() const { return number_of_frames_; }
  media::VideoPixelFormat format() const { return format_; }
  gfx::Size frame_size() const { return frame_size_; }
  scoped_refptr<media::VideoFrame> last_frame() const { return last_frame_; }

  bool enabled() const { return enabled_; }
  WebMediaStreamSource::ReadyState state() const { return state_; }
  std::optional<WebMediaStreamTrack::ContentHintType> content_hint() const {
    return content_hint_;
  }

  void SetUsesAlpha(MediaStreamVideoSink::UsesAlpha uses_alpha) {
    uses_alpha_ = uses_alpha;
  }

 private:
  void DeliverVideoFrame(
      scoped_refptr<media::VideoFrame> frame,
      base::TimeTicks estimated_capture_time);
  void DeliverEncodedVideoFrame(scoped_refptr<EncodedVideoFrame> frame,
                                base::TimeTicks estimated_capture_time);
  void NotifyFrameDropped(media::VideoCaptureFrameDropReason reason);

  MediaStreamVideoSink::UsesAlpha uses_alpha_ =
      MediaStreamVideoSink::UsesAlpha::kDefault;
  int number_of_frames_;
  bool enabled_;
  media::VideoPixelFormat format_;
  WebMediaStreamSource::ReadyState state_;
  gfx::Size frame_size_;
  scoped_refptr<media::VideoFrame> last_frame_;
  std::optional<WebMediaStreamTrack::ContentHintType> content_hint_;
  base::WeakPtrFactory<MockMediaStreamVideoSink> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_VIDEO_SINK_H_
