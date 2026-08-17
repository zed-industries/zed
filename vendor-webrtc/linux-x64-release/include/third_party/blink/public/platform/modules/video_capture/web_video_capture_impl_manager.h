// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_VIDEO_CAPTURE_WEB_VIDEO_CAPTURE_IMPL_MANAGER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_VIDEO_CAPTURE_WEB_VIDEO_CAPTURE_IMPL_MANAGER_H_

#include <memory>
#include <vector>

#include "base/functional/callback.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/token.h"
#include "media/capture/video_capture_types.h"
#include "third_party/blink/public/common/mediastream/media_stream_request.h"
#include "third_party/blink/public/platform/media/video_capture.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

class BrowserInterfaceBrokerProxy;
class VideoCaptureImpl;
class WebString;

// TODO(hclam): This class should be renamed to VideoCaptureService.

// This class provides access to a video capture device in the browser
// process through IPC. The main function is to deliver video frames
// to a client.
//
// THREADING
//
// WebVideoCaptureImplManager lives only on the Render Main thread. All methods
// must be called on this thread.
//
// VideoFrames are delivered on the IO thread. Callbacks provided by
// a client are also called on the IO thread.
class BLINK_PLATFORM_EXPORT WebVideoCaptureImplManager {
 public:
  WebVideoCaptureImplManager();
  WebVideoCaptureImplManager(const WebVideoCaptureImplManager&) = delete;
  WebVideoCaptureImplManager& operator=(const WebVideoCaptureImplManager&) =
      delete;
  virtual ~WebVideoCaptureImplManager();

  // Open a device associated with the session ID.
  // This method must be called before any methods with the same ID
  // is used.
  // Returns a callback that should be used to release the acquired
  // resources.
  base::OnceClosure UseDevice(
      const media::VideoCaptureSessionId& id,
      const BrowserInterfaceBrokerProxy& browser_interface_broker);

  // Start receiving video frames for the given session ID.
  //
  // |state_update_cb| will be called on the IO thread when capturing
  // state changes.
  // States will be one of the following four:
  // * VIDEO_CAPTURE_STATE_STARTED
  // * VIDEO_CAPTURE_STATE_STOPPED
  // * VIDEO_CAPTURE_STATE_PAUSED
  // * VIDEO_CAPTURE_STATE_ERROR
  //
  // |deliver_frame_cb| will be called on the IO thread when a video
  // frame is ready.
  //
  // |sub_capture_target_version_cb| will be called on the IO thread when a new
  // crop version is successfully applied, and it is guaranteed that all
  // subsequent frames delivered to |deliver_frame_cb|, will have this
  // sub-capture-target version or later.
  //
  // |frame_dropped_cb| will be called when a frame was dropped prior to
  // delivery (i.e. |deliver_frame_cb| was not called for this frame).
  //
  // Returns a callback that is used to stop capturing. Note that stopping
  // video capture is not synchronous. Client should handle the case where
  // callbacks are called after capturing is instructed to stop, typically
  // by binding the passed callbacks on a WeakPtr.
  base::OnceClosure StartCapture(
      const media::VideoCaptureSessionId& id,
      const media::VideoCaptureParams& params,
      const VideoCaptureStateUpdateCB& state_update_cb,
      const VideoCaptureDeliverFrameCB& deliver_frame_cb,
      const VideoCaptureSubCaptureTargetVersionCB&
          sub_capture_target_version_cb,
      const VideoCaptureNotifyFrameDroppedCB& frame_dropped_cb);

  // Requests that the video capturer send a frame "soon" (e.g., to resolve
  // picture loss or quality issues).
  void RequestRefreshFrame(const media::VideoCaptureSessionId& id);

  // Requests frame delivery be suspended/resumed for a given capture session.
  void Suspend(const media::VideoCaptureSessionId& id);
  void Resume(const media::VideoCaptureSessionId& id);

  // Get supported formats supported by the device for the given session
  // ID. |callback| will be called on the IO thread.
  void GetDeviceSupportedFormats(const media::VideoCaptureSessionId& id,
                                 VideoCaptureDeviceFormatsCB callback);

  // Get supported formats currently in use for the given session ID.
  // |callback| will be called on the IO thread.
  void GetDeviceFormatsInUse(const media::VideoCaptureSessionId& id,
                             VideoCaptureDeviceFormatsCB callback);

  // Make all VideoCaptureImpl instances in the input |video_devices|
  // stop/resume delivering video frames to their clients, depends on flag
  // |suspend|. This is called in response to a RenderView-wide
  // PageHidden/Shown() event.
  // To suspend/resume an individual session, please call Suspend(id) or
  // Resume(id).
  void SuspendDevices(const MediaStreamDevices& video_devices, bool suspend);

  void OnLog(const media::VideoCaptureSessionId& id, const WebString& message);

  // Get the feedback callback for the corresponding capture session.
  // Consumers may call the returned callback in any thread to provide
  // the capturer with feedback information.
  VideoCaptureFeedbackCB GetFeedbackCallback(
      const media::VideoCaptureSessionId& id) const;

 private:
  // Holds bookkeeping info for each VideoCaptureImpl shared by clients.
  struct DeviceEntry;

  virtual std::unique_ptr<VideoCaptureImpl> CreateVideoCaptureImpl(
      const media::VideoCaptureSessionId& session_id,
      const BrowserInterfaceBrokerProxy& browser_interface_broker) const;

  static void ProcessFeedback(VideoCaptureFeedbackCB callback_to_io_thread,
                              const media::VideoCaptureFeedback& feedback);

  void ProcessFeedbackInternal(const media::VideoCaptureSessionId& id,
                               const media::VideoCaptureFeedback& feedback);

  void StopCapture(int client_id, const media::VideoCaptureSessionId& id);
  void UnrefDevice(const media::VideoCaptureSessionId& id);

  // Devices currently in use.
  std::vector<DeviceEntry> devices_;

  // This is an internal ID for identifying clients of VideoCaptureImpl.
  // The ID is global for the render process.
  int next_client_id_;

  // Hold a pointer to the Render Main message loop to check we operate on the
  // right thread.
  const scoped_refptr<base::SingleThreadTaskRunner> render_main_task_runner_;

  // Set to true if SuspendDevices(true) was called. This, along with
  // DeviceEntry::is_individually_suspended, is used to determine whether to
  // take action when suspending/resuming each device.
  bool is_suspending_all_;

  // Bound to the render thread.
  // NOTE: Weak pointers must be invalidated before all other member variables.
  base::WeakPtrFactory<WebVideoCaptureImplManager> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_MODULES_VIDEO_CAPTURE_WEB_VIDEO_CAPTURE_IMPL_MANAGER_H_
