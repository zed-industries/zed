// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_VIDEO_CAPTURER_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_VIDEO_CAPTURER_SOURCE_H_

#include <memory>
#include <string>

#include "base/functional/callback.h"
#include "base/gtest_prod_util.h"
#include "base/memory/weak_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/token.h"
#include "build/build_config.h"
#include "media/capture/mojom/video_capture_types.mojom-blink.h"
#include "media/capture/video_capture_types.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/mojom/mediastream/media_stream.mojom-blink.h"
#include "third_party/blink/public/platform/media/video_capture.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_source.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/video_capture/video_capturer_source.h"

namespace blink {

class LocalFrame;
class VideoCapturerSource;

// Representation of a video stream coming from a camera, owned by Blink as
// WebMediaStreamSource. Objects of this class are created and live on main
// Render thread. Objects can be constructed either by indicating a |device| to
// look for, or by plugging in a |source| constructed elsewhere.
class MODULES_EXPORT MediaStreamVideoCapturerSource
    : public MediaStreamVideoSource {
 public:
  using DeviceCapturerFactoryCallback =
      base::RepeatingCallback<std::unique_ptr<VideoCapturerSource>(
          const base::UnguessableToken& session_id)>;
  MediaStreamVideoCapturerSource(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      LocalFrame* frame,
      SourceStoppedCallback stop_callback,
      std::unique_ptr<VideoCapturerSource> source);
  MediaStreamVideoCapturerSource(
      scoped_refptr<base::SingleThreadTaskRunner> main_task_runner,
      LocalFrame* frame,
      SourceStoppedCallback stop_callback,
      const MediaStreamDevice& device,
      const media::VideoCaptureParams& capture_params,
      DeviceCapturerFactoryCallback device_capturer_factory_callback);

  MediaStreamVideoCapturerSource(const MediaStreamVideoCapturerSource&) =
      delete;
  MediaStreamVideoCapturerSource& operator=(
      const MediaStreamVideoCapturerSource&) = delete;

  ~MediaStreamVideoCapturerSource() override;

  void SetDeviceCapturerFactoryCallbackForTesting(
      DeviceCapturerFactoryCallback testing_factory_callback);

  void SetMediaStreamDispatcherHostForTesting(
      mojo::PendingRemote<mojom::blink::MediaStreamDispatcherHost>
          dispatcher_host);

  VideoCapturerSource* GetSourceForTesting();

 private:
  friend class MediaStreamVideoCapturerSourceTest;
  FRIEND_TEST_ALL_PREFIXES(MediaStreamVideoCapturerSourceTest, StartAndStop);
  FRIEND_TEST_ALL_PREFIXES(MediaStreamVideoCapturerSourceTest,
                           CaptureTimeAndMetadataPlumbing);
  FRIEND_TEST_ALL_PREFIXES(MediaStreamVideoCapturerSourceTest, ChangeSource);

  // MediaStreamVideoSource overrides.
  void OnSourceCanDiscardAlpha(bool can_discard_alpha) override;
  void RequestRefreshFrame() override;
  void OnLog(const std::string& message) override;
  void OnHasConsumers(bool has_consumers) override;
  void OnCapturingLinkSecured(bool is_secure) override;
  void StartSourceImpl(
      MediaStreamVideoSourceCallbacks media_stream_callbacks) override;
  media::VideoCaptureFeedbackCB GetFeedbackCallback() const override;
  void StopSourceImpl() override;
  void StopSourceForRestartImpl() override;
  void RestartSourceImpl(const media::VideoCaptureFormat& new_format) override;
  std::optional<media::VideoCaptureFormat> GetCurrentFormat() const override;
  void ChangeSourceImpl(const MediaStreamDevice& new_device) override;
#if !BUILDFLAG(IS_ANDROID) && !BUILDFLAG(IS_IOS)
  void ApplySubCaptureTarget(
      media::mojom::blink::SubCaptureTargetType type,
      const base::Token& sub_capture_target,
      uint32_t sub_capture_target_version,
      base::OnceCallback<void(media::mojom::ApplySubCaptureTargetResult)>
          callback) override;
  std::optional<uint32_t> GetNextSubCaptureTargetVersion() override;
#endif
  uint32_t GetSubCaptureTargetVersion() const override;
  base::WeakPtr<MediaStreamVideoSource> GetWeakPtr() override;

  // Method to bind as RunningCallback in VideoCapturerSource::StartCapture().
  void OnRunStateChanged(const media::VideoCaptureParams& new_capture_params,
                         RunState run_state);

  mojom::blink::MediaStreamDispatcherHost* GetMediaStreamDispatcherHost();

  WeakPersistent<LocalFrame> frame_;
  mojo::Remote<mojom::blink::MediaStreamDispatcherHost> host_;

  // The source that provides video frames.
  std::unique_ptr<VideoCapturerSource> source_;

  enum State {
    kStarting,
    kStarted,
    kStoppingForRestart,
    kStoppingForChangeSource,
    kStoppedForRestart,
    kRestarting,
    kRestartingAfterSourceChange,
    kStopped
  };
  State state_ = kStopped;

  media::VideoCaptureParams capture_params_;
  VideoCaptureDeliverFrameCB frame_callback_;
  VideoCaptureSubCaptureTargetVersionCB sub_capture_target_version_callback_;
  VideoCaptureNotifyFrameDroppedCB frame_dropped_callback_;
  DeviceCapturerFactoryCallback device_capturer_factory_callback_;

  // Each time Crop() is called, the source sub-capture-target version
  // increments. Associate each Promise with its sub-capture-target version, so
  // that Viz can easily stamp each frame. When we see the first such frame, or
  // an equivalent message, we can resolve the Promise. (An "equivalent message"
  // can be a notification of a dropped frame, or a notification that a frame
  // was not produced due to consisting of 0 pixels after the crop was applied,
  // or anything similar.)
  //
  // Note that frames before the first call to cropTo() will be associated
  // with a version of 0, both here and in Viz.
  uint32_t current_sub_capture_target_version_ = 0;

  base::WeakPtrFactory<MediaStreamVideoCapturerSource> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_VIDEO_CAPTURER_SOURCE_H_
