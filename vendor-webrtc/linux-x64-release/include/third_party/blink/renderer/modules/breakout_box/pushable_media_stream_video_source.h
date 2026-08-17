// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_PUSHABLE_MEDIA_STREAM_VIDEO_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_PUSHABLE_MEDIA_STREAM_VIDEO_SOURCE_H_

#include "base/memory/raw_ptr.h"
#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/public/web/modules/mediastream/media_stream_video_source.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

// Simplifies the creation of video tracks.  Just do this:
// auto source = std::make_unique<PushableMediaStreamVideoSource>();
// auto* track = CreateVideoTrackFromSource(script_state, source);
// for each frame:
//   source->PushFrame(video_frame, capture_time);
// source->Stop();
class MODULES_EXPORT PushableMediaStreamVideoSource
    : public MediaStreamVideoSource {
 public:
  // Helper class that facilitates interacting with a
  // PushableMediaStreamVideoSource from multiple threads. This also includes
  // safely posting tasks to/from outside the main thread.
  // The public methods of this class can be called on any thread.
  class MODULES_EXPORT Broker : public WTF::ThreadSafeRefCounted<Broker> {
   public:
    Broker(const Broker&) = delete;
    Broker& operator=(const Broker&) = delete;

    // Increases the count of connected clients.
    void OnClientStarted();
    // Decreases the count of connected clients. If the count reaches zero,
    // StopSource() is called.
    void OnClientStopped();
    bool IsRunning();
    // Indicates if connected sinks require the alpha channel.
    bool CanDiscardAlpha();
    // Indicates if connected sinks require a memory-mapped video frame.
    bool RequireMappedFrame();
    void PushFrame(scoped_refptr<media::VideoFrame> video_frame,
                   base::TimeTicks estimated_capture_time);
    void StopSource();
    bool IsMuted();
    void SetMuted(bool);

   private:
    friend class PushableMediaStreamVideoSource;

    explicit Broker(PushableMediaStreamVideoSource* source);

    // These functions must be called on |main_task_runner_|.
    void OnSourceStarted(VideoCaptureDeliverFrameCB);
    void OnSourceDestroyedOrStopped();
    void StopSourceOnMain();
    void SetCanDiscardAlpha(bool can_discard_alpha);
    void ProcessFeedback(const media::VideoCaptureFeedback& feedback);

    base::Lock lock_;
    // |source_| can only change its value on |main_task_runner_|. We use
    // |lock_| to guard it for value changes and for reads outside
    // |main_task_runner_|. It is not necessary to guard it with |lock_| to
    // read its value on |main_task_runner_|. This helps avoid deadlocks in
    // Stop()/OnSourceDestroyedOrStopped() interactions.
    raw_ptr<PushableMediaStreamVideoSource> source_;
    // The same apples to |frame_callback_|, but since it does not have
    // complex interactions with owners, like |source_| does, we always guard
    // it for simplicity.
    VideoCaptureDeliverFrameCB frame_callback_ GUARDED_BY(lock_);
    int num_clients_ GUARDED_BY(lock_) = 0;
    bool muted_ GUARDED_BY(lock_) = false;
    bool can_discard_alpha_ GUARDED_BY(lock_) = false;
    media::VideoCaptureFeedback feedback_ GUARDED_BY(lock_);

    scoped_refptr<base::SingleThreadTaskRunner> main_task_runner_;
    scoped_refptr<base::SequencedTaskRunner> video_task_runner_;
  };

  explicit PushableMediaStreamVideoSource(
      scoped_refptr<base::SingleThreadTaskRunner>);
  ~PushableMediaStreamVideoSource() override;

  // See the definition of VideoCaptureDeliverFrameCB in
  // third_party/blink/public/platform/media/video_capture.h
  // for the documentation of |estimated_capture_time| and the difference with
  // media::VideoFrame::timestamp().
  // This function can be called on any thread.
  void PushFrame(scoped_refptr<media::VideoFrame> video_frame,
                 base::TimeTicks estimated_capture_time);

  // These functions can be called on any thread.
  bool IsRunning() const { return broker_->IsRunning(); }
  scoped_refptr<Broker> GetBroker() const { return broker_; }

  // MediaStreamVideoSource
  void StartSourceImpl(
      MediaStreamVideoSourceCallbacks media_stream_callbacks) override;
  void StopSourceImpl() override;
  base::WeakPtr<MediaStreamVideoSource> GetWeakPtr() override;
  void OnSourceCanDiscardAlpha(bool can_discard_alpha) override;
  // This function can be called on any thread.
  media::VideoCaptureFeedbackCB GetFeedbackCallback() const override;

 private:
  void ProcessFeedbackInternal(const media::VideoCaptureFeedback& feedback);

  scoped_refptr<Broker> broker_;
  base::WeakPtrFactory<PushableMediaStreamVideoSource> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_PUSHABLE_MEDIA_STREAM_VIDEO_SOURCE_H_
