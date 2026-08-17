// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_VIDEO_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_VIDEO_SOURCE_H_

#include "build/build_config.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_source.h"
#include "third_party/blink/renderer/modules/mediastream/sub_capture_target.h"

namespace blink {

class MockMediaStreamVideoSource : public blink::MediaStreamVideoSource {
 public:
  MockMediaStreamVideoSource();
  explicit MockMediaStreamVideoSource(bool respond_to_request_refresh_frame);
  MockMediaStreamVideoSource(const media::VideoCaptureFormat& format,
                             bool respond_to_request_refresh_frame);

  MockMediaStreamVideoSource(const MockMediaStreamVideoSource&) = delete;
  MockMediaStreamVideoSource& operator=(const MockMediaStreamVideoSource&) =
      delete;

  ~MockMediaStreamVideoSource() override;

  MOCK_METHOD1(DoSetMutedState, void(bool muted_state));
  MOCK_METHOD0(OnEncodedSinkEnabled, void());
  MOCK_METHOD0(OnEncodedSinkDisabled, void());
  MOCK_METHOD0(OnRequestKeyFrame, void());
  MOCK_METHOD0(OnRequestRefreshFrame, void());
  MOCK_METHOD1(OnCapturingLinkSecured, void(bool));
  MOCK_METHOD1(OnSourceCanDiscardAlpha, void(bool can_discard_alpha));
  MOCK_CONST_METHOD0(SupportsEncodedOutput, bool());
  MOCK_METHOD4(
      ApplySubCaptureTarget,
      void(
          SubCaptureTarget::Type,
          const base::Token&,
          uint32_t,
          base::OnceCallback<void(media::mojom::ApplySubCaptureTargetResult)>));
  MOCK_METHOD0(GetNextSubCaptureTargetVersion, std::optional<uint32_t>());
  MOCK_METHOD(uint32_t, GetSubCaptureTargetVersion, (), (const, override));

  // Simulate that the underlying source start successfully.
  void StartMockedSource();

  // Simulate that the underlying source fail to start.
  void FailToStartMockedSource();

  // Returns true if StartSource  has been called and StartMockedSource
  // or FailToStartMockedSource has not been called.
  bool SourceHasAttemptedToStart() { return attempted_to_start_; }

  // Delivers |frame| to all registered tracks on the video task runner. It's up
  // to the caller to make sure MockMediaStreamVideoSource is not destroyed
  // before the frame has been delivered.
  void DeliverVideoFrame(scoped_refptr<media::VideoFrame> frame);

  // Delivers |frame| to all registered encoded sinks on the video task runner.
  // It's up to the caller to make sure MockMediaStreamVideoSource is not
  // destroyed before the frame has been delivered.
  void DeliverEncodedVideoFrame(scoped_refptr<EncodedVideoFrame> frame);

  // Signal that a frame was dropped. It's up to the caller to make sure
  // MockMediaStreamVideoSource is not destroyed before the frame drop has
  // happened on the video task runner.
  void DropFrame(media::VideoCaptureFrameDropReason reason);

  // Send |sub_capture_target_version| to all registered tracks on the video
  // task runner. It's up to the caller to keep MockMediaStreamVideoSource alive
  // until the sub_capture_target_version_callback (registered with
  // MediaStreamVideoSource::AddTrack) has completed.
  void DeliverNewSubCaptureTargetVersion(uint32_t sub_capture_target_version);

  const media::VideoCaptureFormat& start_format() const { return format_; }
  int max_requested_height() const { return format_.frame_size.height(); }
  int max_requested_width() const { return format_.frame_size.width(); }
  float max_requested_frame_rate() const { return format_.frame_rate; }

  void SetMutedState(bool muted_state) override {
    blink::MediaStreamVideoSource::SetMutedState(muted_state);
    DoSetMutedState(muted_state);
  }

  void EnableStopForRestart() { can_stop_for_restart_ = true; }
  void DisableStopForRestart() { can_stop_for_restart_ = false; }

  void EnableRestart() { can_restart_ = true; }
  void DisableRestart() { can_restart_ = false; }
  int restart_count() const { return restart_count_; }

  bool is_suspended() { return is_suspended_; }

  // Implements blink::MediaStreamVideoSource.
  void RequestKeyFrame() override;
  void RequestRefreshFrame() override;
  void OnHasConsumers(bool has_consumers) override;
  base::WeakPtr<MediaStreamVideoSource> GetWeakPtr() override;

 protected:
  // Implements MediaStreamSource.
  void DoChangeSource(const blink::MediaStreamDevice& new_device) override;

  // Implements blink::MediaStreamVideoSource.
  void StartSourceImpl(
      MediaStreamVideoSourceCallbacks media_stream_callbacks) override;
  void StopSourceImpl() override;
  std::optional<media::VideoCaptureFormat> GetCurrentFormat() const override;
  void StopSourceForRestartImpl() override;
  void RestartSourceImpl(const media::VideoCaptureFormat& new_format) override;

 private:
  media::VideoCaptureFormat format_;
  bool respond_to_request_refresh_frame_;
  bool attempted_to_start_;
  bool is_stopped_for_restart_ = false;
  bool can_stop_for_restart_ = true;
  bool can_restart_ = true;
  int restart_count_ = 0;
  bool is_suspended_ = false;
  blink::VideoCaptureDeliverFrameCB frame_callback_;
  EncodedVideoFrameCB encoded_frame_callback_;
  VideoCaptureSubCaptureTargetVersionCB sub_capture_target_version_callback_;
  VideoCaptureNotifyFrameDroppedCB frame_dropped_callback_;

  base::WeakPtrFactory<MediaStreamVideoSource> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MOCK_MEDIA_STREAM_VIDEO_SOURCE_H_
