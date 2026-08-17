// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_FRAME_QUEUE_UNDERLYING_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_FRAME_QUEUE_UNDERLYING_SOURCE_H_

#include "base/synchronization/lock.h"
#include "base/task/sequenced_task_runner.h"
#include "base/threading/thread_checker.h"
#include "media/base/audio_buffer.h"
#include "media/base/video_frame.h"
#include "third_party/blink/renderer/core/streams/underlying_source_base.h"
#include "third_party/blink/renderer/modules/breakout_box/frame_queue.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

MODULES_EXPORT BASE_DECLARE_FEATURE(kBreakoutBoxInsertVideoCaptureTimestamp);

template <typename NativeFrameType>
class FrameQueueUnderlyingSource : public UnderlyingSourceBase {
 public:
  using TransferFramesCB = CrossThreadFunction<void(NativeFrameType)>;

  // Initializes a new FrameQueueUnderlyingSource with a new internal circular
  // queue that can hold up to |queue_size| elements. If a nonempty |device_id|
  // is given, it will be used as a key to monitor open frames.
  FrameQueueUnderlyingSource(ScriptState*,
                             wtf_size_t queue_size,
                             std::string device_id,
                             wtf_size_t frame_pool_size);
  FrameQueueUnderlyingSource(ScriptState*, wtf_size_t queue_size);
  ~FrameQueueUnderlyingSource() override = default;

  FrameQueueUnderlyingSource(const FrameQueueUnderlyingSource&) = delete;
  FrameQueueUnderlyingSource& operator=(const FrameQueueUnderlyingSource&) =
      delete;

  // UnderlyingSourceBase
  ScriptPromise<IDLUndefined> Pull(ScriptState*, ExceptionState&) override;
  ScriptPromise<IDLUndefined> Start(ScriptState*) override;
  ScriptPromise<IDLUndefined> Cancel(ScriptState*,
                                     ScriptValue reason,
                                     ExceptionState&) override;

  // ExecutionLifecycleObserver
  void ContextDestroyed() override;

  wtf_size_t MaxQueueSize() const;

  // Clears all internal state and closes the UnderlyingSource's Controller.
  // Must be called on |realm_task_runner_|.
  void Close();

  bool IsClosed() { return is_closed_; }

  // Start or stop the delivery of frames via QueueFrame().
  // Must be called on |realm_task_runner_|.
  virtual bool StartFrameDelivery() = 0;
  virtual void StopFrameDelivery() = 0;

  // Delivers a new frame to this source.
  void QueueFrame(NativeFrameType);

  int NumPendingPullsForTesting() const;
  double DesiredSizeForTesting() const;

  void Trace(Visitor*) const override;

 protected:
  // Initializes a new FrameQueueUnderlyingSource containing a
  // |frame_queue_handle_| that references the same internal circular queue as
  // |other_source|.
  FrameQueueUnderlyingSource(
      ScriptState*,
      FrameQueueUnderlyingSource<NativeFrameType>* other_source);
  scoped_refptr<base::SequencedTaskRunner> GetRealmRunner() {
    return realm_task_runner_;
  }

  // Sets |transferred_source| as the new target for frames arriving via
  // QueueFrame(). |transferred_source| will pull frames from the same circular
  // queue. Must be called on |realm_task_runner_|.
  void TransferSource(
      CrossThreadPersistent<FrameQueueUnderlyingSource<NativeFrameType>>
          transferred_source);

  // Due to a potential race condition between |transferred_source_|'s heap
  // being destroyed and the Close() method being called, we need to explicitly
  // clear |transferred_source_| when its context is being destroyed.
  void ClearTransferredSource();

 protected:
  bool MustUseMonitor() const;

 private:
  // Must be called on |realm_task_runner_|.
  void CloseController();

  // If the internal queue is not empty, pops a frame from it and enqueues it
  // into the the stream's controller. Must be called on |realm_task_runner|.
  void MaybeSendFrameFromQueueToStream();

  base::Lock& GetMonitorLock();

  void MaybeMonitorPopFrameId(media::VideoFrame::ID frame_id);
  void MonitorPopFrameLocked(const NativeFrameType& media_frame)
      EXCLUSIVE_LOCKS_REQUIRED(GetMonitorLock());
  void MonitorPushFrameLocked(const NativeFrameType& media_frame)
      EXCLUSIVE_LOCKS_REQUIRED(GetMonitorLock());

  enum class NewFrameAction { kPush, kReplace, kDrop };
  NewFrameAction AnalyzeNewFrameLocked(
      const NativeFrameType& media_frame,
      const std::optional<NativeFrameType>& old_frame);

  // Creates a JS frame (VideoFrame or AudioData) backed by |media_frame|.
  // Must be called on |realm_task_runner_|.
  ScriptWrappable* MakeBlinkFrame(NativeFrameType media_frame);

  void EnqueueBlinkFrame(ScriptWrappable* blink_frame) const;

  bool is_closed_ = false;

  // Main task runner for the window or worker context this source runs on.
  const scoped_refptr<base::SequencedTaskRunner> realm_task_runner_;

  // |frame_queue_handle_| is a handle containing a reference to an internal
  // circular queue prior to the stream controller's queue. It allows dropping
  // older frames in case frames accumulate due to slow consumption.
  // Frames are normally pushed on a background task runner (for example, the
  // IO thread) and popped on |realm_task_runner_|.
  FrameQueueHandle<NativeFrameType> frame_queue_handle_;

  mutable base::Lock lock_;
  // If the stream backed by this source is transferred to another in-process
  // realm (e.g., a Worker), |transferred_source_| is the source of the
  // transferred stream.
  CrossThreadPersistent<FrameQueueUnderlyingSource<NativeFrameType>>
      transferred_source_ GUARDED_BY(lock_);
  int num_pending_pulls_ GUARDED_BY(lock_) = 0;
  // When nonempty, |device_id_| is used to monitor all frames queued by this
  // source or exposed to JS via the stream connected to this source.
  // Frame monitoring applies only to video. Audio is never monitored.
  const std::string device_id_;
  // Maximum number of distinct frames allowed to be used by this source.
  // This limit applies only when |device_id_| is nonempty.
  const wtf_size_t frame_pool_size_ = 0;

  std::optional<base::TimeTicks> first_frame_ticks_;
};

template <>
ScriptWrappable* FrameQueueUnderlyingSource<scoped_refptr<media::VideoFrame>>::
    MakeBlinkFrame(scoped_refptr<media::VideoFrame>);

template <>
ScriptWrappable* FrameQueueUnderlyingSource<scoped_refptr<media::AudioBuffer>>::
    MakeBlinkFrame(scoped_refptr<media::AudioBuffer>);

template <>
bool FrameQueueUnderlyingSource<
    scoped_refptr<media::AudioBuffer>>::MustUseMonitor() const;

extern template class MODULES_EXTERN_TEMPLATE_EXPORT
    FrameQueueUnderlyingSource<scoped_refptr<media::VideoFrame>>;
extern template class MODULES_EXTERN_TEMPLATE_EXPORT
    FrameQueueUnderlyingSource<scoped_refptr<media::AudioBuffer>>;

using VideoFrameQueueUnderlyingSource =
    FrameQueueUnderlyingSource<scoped_refptr<media::VideoFrame>>;
using AudioDataQueueUnderlyingSource =
    FrameQueueUnderlyingSource<scoped_refptr<media::AudioBuffer>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BREAKOUT_BOX_FRAME_QUEUE_UNDERLYING_SOURCE_H_
