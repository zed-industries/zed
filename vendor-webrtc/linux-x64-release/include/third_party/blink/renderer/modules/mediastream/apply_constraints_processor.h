// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_APPLY_CONSTRAINTS_PROCESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_APPLY_CONSTRAINTS_PROCESSOR_H_

#include "base/feature_list.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "media/base/scoped_async_trace.h"
#include "media/capture/video_capture_types.h"
#include "third_party/blink/public/mojom/mediastream/media_devices.mojom-blink-forward.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_source.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/modules/mediastream/apply_constraints_request.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_constraints_util.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_functional.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {
class MediaStreamAudioSource;
class MediaStreamVideoTrack;

// If this feature is enabled, a call to applyConstraints() on a video content
// source will make the source restart with the new format.
MODULES_EXPORT BASE_DECLARE_FEATURE(
    kApplyConstraintsRestartsVideoContentSources);

// ApplyConstraintsProcessor is responsible for processing applyConstraints()
// requests. Only one applyConstraints() request can be processed at a time.
// ApplyConstraintsProcessor must be created, called and destroyed on the main
// render thread. There should be only one ApplyConstraintsProcessor per frame.
class MODULES_EXPORT ApplyConstraintsProcessor final
    : public GarbageCollected<ApplyConstraintsProcessor> {
 public:
  using MediaDevicesDispatcherCallback = base::RepeatingCallback<
      blink::mojom::blink::MediaDevicesDispatcherHost*()>;
  ApplyConstraintsProcessor(
      LocalFrame* frame,
      MediaDevicesDispatcherCallback media_devices_dispatcher_cb,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner);

  ApplyConstraintsProcessor(const ApplyConstraintsProcessor&) = delete;
  ApplyConstraintsProcessor& operator=(const ApplyConstraintsProcessor&) =
      delete;

  ~ApplyConstraintsProcessor();

  // Starts processing of |request|. When processing of |request| is complete,
  // it notifies by invoking |callback|.
  // This method must be called only if there is no request currently being
  // processed.
  void ProcessRequest(blink::ApplyConstraintsRequest* request,
                      base::OnceClosure callback);

  void Trace(Visitor* visitor) const {
    visitor->Trace(current_request_);
    visitor->Trace(frame_);
  }

 private:
  // Helpers for video device-capture requests.
  void ProcessVideoDeviceRequest();
  void MaybeStopVideoDeviceSourceForRestart(
      const Vector<media::VideoCaptureFormat>& formats);
  void MaybeDeviceSourceStoppedForRestart(
      blink::MediaStreamVideoSource::RestartResult result);
  void FindNewFormatAndRestartDeviceSource(
      const Vector<media::VideoCaptureFormat>& formats);
  blink::VideoCaptureSettings SelectVideoDeviceSettings(
      Vector<media::VideoCaptureFormat> formats);

  // Helpers for video content-capture requests.
  void ProcessVideoContentRequest();
  void MaybeStopVideoContentSourceForRestart();
  void MaybeRestartStoppedVideoContentSource(
      blink::MediaStreamVideoSource::RestartResult result);
  blink::VideoCaptureSettings SelectVideoContentSettings();

  // Helpers for all video requests.
  void ProcessVideoRequest();
  blink::MediaStreamVideoTrack* GetCurrentVideoTrack();
  blink::MediaStreamVideoSource* GetCurrentVideoSource();
  bool AbortIfVideoRequestStateInvalid();  // Returns true if aborted.
  void FinalizeVideoRequest();
  void MaybeSourceRestarted(
      blink::MediaStreamVideoSource::RestartResult result);

  // Helpers for audio requests.
  void ProcessAudioRequest();
  blink::MediaStreamAudioSource* GetCurrentAudioSource();

  // General helpers
  void ApplyConstraintsSucceeded();
  void ApplyConstraintsFailed(const char* failed_constraint_name);
  void CannotApplyConstraints(const String& message);
  void CleanupRequest(base::OnceClosure user_media_request_callback);
  blink::mojom::blink::MediaDevicesDispatcherHost* GetMediaDevicesDispatcher();

  // ApplyConstraints requests are processed sequentially. |current_request_|
  // contains the request currently being processed, if any.
  // |video_source_| and |request_completed_cb_| are the video source and
  // reply callback for the current request.
  Member<blink::ApplyConstraintsRequest> current_request_;

  using ScopedMediaStreamTrace =
      media::TypedScopedAsyncTrace<media::TraceCategory::kMediaStream>;
  std::unique_ptr<ScopedMediaStreamTrace> video_device_request_trace_;

  // TODO(crbug.com/704136): Change to use Member.
  raw_ptr<blink::MediaStreamVideoSource> video_source_ = nullptr;
  base::OnceClosure request_completed_cb_;

  const Member<LocalFrame> frame_;
  MediaDevicesDispatcherCallback media_devices_dispatcher_cb_;
  THREAD_CHECKER(thread_checker_);

  const scoped_refptr<base::SingleThreadTaskRunner> task_runner_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_APPLY_CONSTRAINTS_PROCESSOR_H_
