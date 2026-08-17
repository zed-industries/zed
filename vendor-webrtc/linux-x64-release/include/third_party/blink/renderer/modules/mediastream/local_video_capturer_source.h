// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_LOCAL_VIDEO_CAPTURER_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_LOCAL_VIDEO_CAPTURER_SOURCE_H_

#include <memory>
#include <string>

#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/threading/thread_checker.h"
#include "base/token.h"
#include "base/unguessable_token.h"
#include "media/capture/video_capture_types.h"
#include "third_party/blink/public/common/tokens/tokens.h"
#include "third_party/blink/public/platform/media/video_capture.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/video_capture/video_capturer_source.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace base {
class SingleThreadTaskRunner;
}

namespace blink {

class LocalFrame;
class WebVideoCaptureImplManager;

// LocalVideoCapturerSource is a delegate used by MediaStreamVideoCapturerSource
// for local video capture. It uses the Render singleton
// WebVideoCaptureImplManager to start / stop and receive I420 frames from
// Chrome's video capture implementation. This is a main Render thread only
// object.
class MODULES_EXPORT LocalVideoCapturerSource : public VideoCapturerSource {
 public:
  static std::unique_ptr<VideoCapturerSource> Create(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      LocalFrame* frame,
      const base::UnguessableToken& session_id);
  LocalVideoCapturerSource(
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      LocalFrame* frame,
      const base::UnguessableToken& session_id);

  LocalVideoCapturerSource(const LocalVideoCapturerSource&) = delete;
  LocalVideoCapturerSource& operator=(const LocalVideoCapturerSource&) = delete;

  ~LocalVideoCapturerSource() override;

  // VideoCaptureSource Implementation.
  media::VideoCaptureFormats GetPreferredFormats() override;
  void StartCapture(
      const media::VideoCaptureParams& params,
      const VideoCaptureDeliverFrameCB& new_frame_callback,
      const VideoCaptureSubCaptureTargetVersionCB&
          sub_capture_target_version_callback,
      const VideoCaptureNotifyFrameDroppedCB& frame_dropped_callback,
      const RunningCallback& running_callback) override;
  void RequestRefreshFrame() override;
  void MaybeSuspend() override;
  void Resume() override;
  void StopCapture() override;
  void OnLog(const std::string& message) override;
  media::VideoCaptureFeedbackCB GetFeedbackCallback() const override;

 private:
  void OnStateUpdate(blink::VideoCaptureState state);

  // |session_id_| identifies the capture device used for this capture session.
  const media::VideoCaptureSessionId session_id_;

  const raw_ptr<WebVideoCaptureImplManager> manager_;

  LocalFrameToken frame_token_;
  base::OnceClosure release_device_cb_;

  // These two are valid between StartCapture() and StopCapture().
  // |running_call_back_| is run when capture is successfully started, and when
  // it is stopped or error happens.
  RunningCallback running_callback_;
  base::OnceClosure stop_capture_cb_;

  scoped_refptr<base::SingleThreadTaskRunner> task_runner_;

  // Bound to the main render thread.
  THREAD_CHECKER(thread_checker_);

  base::WeakPtrFactory<LocalVideoCapturerSource> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_LOCAL_VIDEO_CAPTURER_SOURCE_H_
