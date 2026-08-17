// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_VIDEO_CAPTURE_VIDEO_CAPTURER_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_VIDEO_CAPTURE_VIDEO_CAPTURER_SOURCE_H_

#include <string>
#include <vector>

#include "base/functional/callback.h"
#include "base/time/time.h"
#include "base/token.h"
#include "media/capture/video/video_capture_feedback.h"
#include "media/capture/video_capture_types.h"
#include "third_party/blink/public/platform/media/video_capture.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {

enum class RunState {
  kRunning = 0,
  kStopped,
  kSystemPermissionsError,
  kCameraBusyError,
  kStartTimeoutError,
};

// VideoCapturerSource is an interface representing the source for captured
// video.  An implementation will periodically call the frame callback with new
// video frames.
class PLATFORM_EXPORT VideoCapturerSource {
 public:
  virtual ~VideoCapturerSource();

  using RunningCallback = base::RepeatingCallback<void(RunState)>;

  // Returns formats that are preferred and can currently be used. May be empty
  // if no formats are available or known.
  virtual media::VideoCaptureFormats GetPreferredFormats() = 0;

  // Starts capturing frames using the capture |params|. |new_frame_callback| is
  // triggered when a new video frame is available.
  //
  // If capturing is started successfully then |running_callback| will be
  // called with a parameter of true. Note that some implementations may
  // simply reject StartCapture (by calling running_callback with a false
  // argument) if called with the wrong task runner.
  //
  // If capturing fails to start or stopped due to an external event then
  // |running_callback| will be called with a parameter of false.
  // |running_callback| will always be called on the same thread as the
  // StartCapture.
  //
  // |sub_capture_target_version_callback| will be called when it is guaranteed
  // that all subsequent frames |new_frame_callback| is called for, have a
  // sub-capture-target version that is equal-to-or-greater-than the given
  // sub-capture-target version.
  //
  // |frame_dropped_callback| will be called when a frame was dropped prior to
  // delivery (i.e. |new_frame_callback| was not called for this frame).
  virtual void StartCapture(
      const media::VideoCaptureParams& params,
      const VideoCaptureDeliverFrameCB& new_frame_callback,
      const VideoCaptureSubCaptureTargetVersionCB&
          sub_capture_target_version_callback,
      const VideoCaptureNotifyFrameDroppedCB& frame_dropped_callback,
      const RunningCallback& running_callback) = 0;

  // Returns a callback for providing the feedback from the consumer.
  // The callback can be called on any thread.
  virtual media::VideoCaptureFeedbackCB GetFeedbackCallback() const;

  // Asks source to send a refresh frame. In cases where source does not provide
  // a continuous rate of new frames (e.g. canvas capture, screen capture where
  // the screen's content has not changed in a while), consumers may request a
  // "refresh frame" to be delivered. For instance, this would be needed when
  // a new sink is added to a MediaStreamTrack.
  //
  // The default implementation is a no-op and implementations are not required
  // to honor this request. If they decide to and capturing is started
  // successfully, then |new_frame_callback| should be called with a frame.
  //
  // Note: This should only be called after StartCapture() and before
  // StopCapture(). Otherwise, its behavior is undefined.
  virtual void RequestRefreshFrame() {}

  // Optionally suspends frame delivery. The source may or may not honor this
  // request. Thus, the caller cannot assume frame delivery will actually
  // stop. Even if frame delivery is suspended, this might not take effect
  // immediately.
  //
  // The purpose of this is to allow minimizing resource usage while
  // there are no frame consumers present.
  //
  // Note: This should only be called after StartCapture() and before
  // StopCapture(). Otherwise, its behavior is undefined.
  virtual void MaybeSuspend() {}

  // Resumes frame delivery, if it was suspended. If frame delivery was not
  // suspended, this is a no-op, and frame delivery will continue.
  //
  // Note: This should only be called after StartCapture() and before
  // StopCapture(). Otherwise, its behavior is undefined.
  virtual void Resume() {}

  // Stops capturing frames and clears all callbacks including the
  // SupportedFormatsCallback callback. Note that queued frame callbacks
  // may still occur after this call, so the caller must take care to
  // use refcounted or weak references in |new_frame_callback|.
  virtual void StopCapture() = 0;

  // Hints to the source that if it has an alpha channel, that alpha channel
  // will be ignored and can be discarded.
  virtual void SetCanDiscardAlpha(bool can_discard_alpha) {}

  // Sends a log message to the source.
  virtual void OnLog(const std::string& message) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_VIDEO_CAPTURE_VIDEO_CAPTURER_SOURCE_H_
