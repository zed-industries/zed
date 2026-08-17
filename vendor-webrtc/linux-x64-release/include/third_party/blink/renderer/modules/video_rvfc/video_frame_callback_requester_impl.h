// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_VIDEO_RVFC_VIDEO_FRAME_CALLBACK_REQUESTER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_VIDEO_RVFC_VIDEO_FRAME_CALLBACK_REQUESTER_IMPL_H_

#include "third_party/blink/renderer/core/html/media/html_video_element.h"
#include "third_party/blink/renderer/core/html/media/video_frame_callback_requester.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/video_rvfc/video_frame_request_callback_collection.h"
#include "third_party/blink/renderer/modules/xr/xr_frame_provider.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/weak_cell.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class HTMLVideoElement;

// Implementation of the <video>.requestVideoFrameCallback() API.
// Extends HTMLVideoElement via the VideoFrameCallbackRequester interface.
class MODULES_EXPORT VideoFrameCallbackRequesterImpl final
    : public VideoFrameCallbackRequester,
      public XRFrameProvider::ImmersiveSessionObserver {
 public:
  static VideoFrameCallbackRequesterImpl& From(HTMLVideoElement&);

  // Web API entry points for requestAnimationFrame().
  static int requestVideoFrameCallback(HTMLVideoElement&,
                                       V8VideoFrameRequestCallback*);
  static void cancelVideoFrameCallback(HTMLVideoElement&, int);

  explicit VideoFrameCallbackRequesterImpl(HTMLVideoElement&);

  VideoFrameCallbackRequesterImpl(const VideoFrameCallbackRequesterImpl&) =
      delete;
  VideoFrameCallbackRequesterImpl& operator=(
      const VideoFrameCallbackRequesterImpl&) = delete;

  ~VideoFrameCallbackRequesterImpl() override;

  void Trace(Visitor*) const override;

  int requestVideoFrameCallback(V8VideoFrameRequestCallback*);
  void cancelVideoFrameCallback(int);

  void OnWebMediaPlayerCreated() override;
  void OnWebMediaPlayerCleared() override;
  void OnRequestVideoFrameCallback() override;

  // Called by ScriptedAnimationController as part of the rendering steps,
  // right before executing window.rAF callbacks. Also called by OnXRFrame().
  void OnExecution(double high_res_now_ms);

  // XRFrameProvider::ImmersiveSessionObserver implementation.
  void OnImmersiveSessionStart() override;
  void OnImmersiveSessionEnd() override;
  void OnImmersiveFrame() override;

 private:
  friend class VideoFrameCallbackRequesterImplTest;

  // Utility functions to limit the clock resolution of fields, for security
  // reasons.
  static double GetClampedTimeInMillis(base::TimeDelta time,
                                       bool cross_origin_isolated_capability);
  static double GetCoarseClampedTimeInSeconds(base::TimeDelta time);

  void ExecuteVideoFrameCallbacks(
      double high_res_now_ms,
      std::unique_ptr<WebMediaPlayer::VideoFramePresentationMetadata>);

  // Register a non-V8 callback for testing. Also sets |pending_execution_| to
  // true, to allow calling into ExecuteFrameCallbacks() directly.
  void RegisterCallbackForTest(
      VideoFrameRequestCallbackCollection::VideoFrameCallback*);

  // Queues up |callback_collection_| to be run before the next window.rAF, or
  // xr_session.rAF if we are an immersive XR session.
  void ScheduleExecution();

  // Adds |this| to the ScriptedAnimationController's queue of video.rVFC
  // callbacks that should be executed during the next rendering steps.
  // Also causes rendering steps to be scheduled if needed.
  void ScheduleWindowRaf();

  // Check whether there is an immersive XR session, and adds |this| to the list
  // of video.rVFC callbacks that should be run the next time there is an XR
  // frame. Requests a new XR frame if needed.
  // Returns true if we scheduled ourselves, false if there is no immersive XR
  // session.
  bool TryScheduleImmersiveXRSessionRaf();

  XRFrameProvider* GetXRFrameProvider();

  // Used to keep track of whether or not we have already scheduled a call to
  // ExecuteFrameCallbacks() in the next rendering steps.
  bool pending_execution_ = false;

  // The value of the |metadata->presented_frames| field the last time called
  // ExecuteFrameCallbacks. Used to determine whether or not a new frame was
  // presented since we last executed the frame callbacks.
  // The values coming from the compositor should start at 1, we can use 0
  // as a "null" starting value.
  uint32_t last_presented_frames_ = 0;

  // Number of times OnRenderingSteps() was called in a row, without us having a
  // new frame. Used to abort auto-rescheduling if we aren't consistently
  // getting new frames.
  int consecutive_stale_frames_ = 0;

  // Indicates whether or not we have registered ourselves with the XR Frame
  // provider to be notified of immersive XR session events.
  bool observing_immersive_session_ = false;

  // Indicates if we are currently in an XR session.
  bool in_immersive_session_ = false;

  // Indicates we are cross-origin isolated.
  bool cross_origin_isolated_capability_ = false;

  Member<VideoFrameRequestCallbackCollection> callback_collection_;

  // Only used to invalidate pending OnExecution() calls.
  WeakCellFactory<VideoFrameCallbackRequesterImpl> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_VIDEO_RVFC_VIDEO_FRAME_CALLBACK_REQUESTER_IMPL_H_
