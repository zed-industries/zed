// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_CROSS_THREAD_MEDIA_SOURCE_ATTACHMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_CROSS_THREAD_MEDIA_SOURCE_ATTACHMENT_H_

#include <memory>

#include "base/synchronization/lock.h"
#include "base/task/single_thread_task_runner.h"
#include "base/thread_annotations.h"
#include "base/types/pass_key.h"
#include "third_party/blink/public/platform/web_time_range.h"
#include "third_party/blink/renderer/core/html/track/audio_track.h"
#include "third_party/blink/renderer/core/html/track/audio_track_list.h"
#include "third_party/blink/renderer/core/html/track/video_track.h"
#include "third_party/blink/renderer/core/html/track/video_track_list.h"
#include "third_party/blink/renderer/modules/mediasource/attachment_creation_pass_key_provider.h"
#include "third_party/blink/renderer/modules/mediasource/media_source.h"
#include "third_party/blink/renderer/modules/mediasource/media_source_attachment_supplement.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace blink {

// Concrete attachment that supports operation between a media element on the
// main thread and the MSE API on a dedicated worker thread.
class CrossThreadMediaSourceAttachment final
    : public MediaSourceAttachmentSupplement {
 public:
  // For use by Remove{Audio,Video}TracksFromMediaElements' and
  // AddMainThread{Audio,Video}TrackToMediaElements' internal helpers.
  enum class TrackAddRemovalType { kAudio, kVideo };

  // The only intended callers of this constructor are restricted to those able
  // to obtain an AttachmentCreationPasskeyProvider's pass key. This method is
  // expected to only be called in a worker thread context. The raw pointer is
  // then adopted into a scoped_refptr by the caller (e.g.,
  // URLMediaSource::createObjectUrl will lead to
  // MediaSourceRegistryImpl::RegisterURL doing this scoped_refptr adoption;
  // separately, MediaSource::handle() does this adoption immediately.)
  CrossThreadMediaSourceAttachment(MediaSource* media_source,
                                   AttachmentCreationPassKeyProvider::PassKey);

  CrossThreadMediaSourceAttachment(const CrossThreadMediaSourceAttachment&) =
      delete;
  CrossThreadMediaSourceAttachment& operator=(
      const CrossThreadMediaSourceAttachment&) = delete;

  // MediaSourceAttachmentSupplement, called by MSE API on worker thread.
  // These generally require the MSE implementation to issue these calls from
  // the target of a RunExclusively() callback to ensure thread safety: much of
  // the MSE API implementation (such as readyState, and knowing whether or not
  // the underlying WebSourceBuffers and WebMediaSource are still usable, since
  // they access a main-thread-owned demuxer) needs to use RunExclusively().
  // Meanwhile, the main thread element could cause changes to such state (via
  // this attachment) and therefore also require exclusion using the same
  // |attachment_state_lock_|.
  void NotifyDurationChanged(MediaSourceTracer* tracer, double duration) final
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);
  base::TimeDelta GetRecentMediaTime(MediaSourceTracer* tracer) final
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);
  bool GetElementError(MediaSourceTracer* tracer) final
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);
  AudioTrackList* CreateAudioTrackList(MediaSourceTracer* tracer) final;
  VideoTrackList* CreateVideoTrackList(MediaSourceTracer* tracer) final;
  void AddAudioTrackToMediaElement(MediaSourceTracer* tracer,
                                   AudioTrack* track) final;
  void AddVideoTrackToMediaElement(MediaSourceTracer* tracer,
                                   VideoTrack* track) final;
  void RemoveAudioTracksFromMediaElement(MediaSourceTracer* tracer,
                                         Vector<String> audio_ids,
                                         bool enqueue_change_event) final
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);
  void RemoveVideoTracksFromMediaElement(MediaSourceTracer* tracer,
                                         Vector<String> video_ids,
                                         bool enqueue_change_event) final
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);
  void AddMainThreadAudioTrackToMediaElement(String id,
                                             String kind,
                                             String label,
                                             String language,
                                             bool enabled) final
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);
  void AddMainThreadVideoTrackToMediaElement(String id,
                                             String kind,
                                             String label,
                                             String language,
                                             bool selected) final
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);
  void OnMediaSourceContextDestroyed() final
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);

  // Method meant to protect MSE API operations on worker thread MSE API from
  // colliding with concurrently executing operations from the main thread
  // running in this attachment. If |abort_if_not_fully_attached| is true, then
  // conditionally runs |cb| iff the media element is still attached and has not
  // ever issued a Close operation on this attachment, and if those conditions
  // fail, |cb| is not run and this method returns false. If
  // |abort_if_not_fully_attached| is false, then unconditionally runs |cb| and
  // returns true. If run, |cb| is run synchronously while holding the
  // attachment's internal |attachment_state_lock_|. Any return values needed by
  // the caller from |cb| should be passed by pointer, enabling usage of this
  // helper to provide safety while still retaining synchronous worker-thread
  // MSE API operation.
  bool RunExclusively(bool abort_if_not_fully_attached,
                      RunExclusivelyCB cb) final
      LOCKS_EXCLUDED(attachment_state_lock_);

  // See MediaSourceAttachmentSupplement for details. Simply, if this returns
  // true, then SourceBuffer::RemovedFromMediaSource() can safely access the
  // underlying demuxer, so long as the |attachment_state_lock_| is held
  // continuously throughout this call and such accesses.
  bool FullyAttachedOrSameThread(SourceBufferPassKey) const final
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);

  // MediaSourceAttachment methods called on main thread by media element,
  // except Unregister is called on either main or dedicated worker thread by
  // MediaSourceRegistryImpl.
  void Unregister() final;
  MediaSourceTracer* StartAttachingToMediaElement(HTMLMediaElement*,
                                                  bool* success) final
      LOCKS_EXCLUDED(attachment_state_lock_);
  void CompleteAttachingToMediaElement(MediaSourceTracer* tracer,
                                       std::unique_ptr<WebMediaSource>) final
      LOCKS_EXCLUDED(attachment_state_lock_);

  void Close(MediaSourceTracer* tracer) final
      LOCKS_EXCLUDED(attachment_state_lock_);

  WebTimeRanges BufferedInternal(MediaSourceTracer* tracer) const final
      LOCKS_EXCLUDED(attachment_state_lock_);

  WebTimeRanges SeekableInternal(MediaSourceTracer* tracer) const final
      LOCKS_EXCLUDED(attachment_state_lock_);
  void OnTrackChanged(MediaSourceTracer* tracer, TrackBase*) final
      LOCKS_EXCLUDED(attachment_state_lock_);

  void OnElementTimeUpdate(double time) final
      LOCKS_EXCLUDED(attachment_state_lock_);
  void OnElementError() final LOCKS_EXCLUDED(attachment_state_lock_);

  void OnElementContextDestroyed() final LOCKS_EXCLUDED(attachment_state_lock_);

  void AssertCrossThreadMutexIsAcquiredForDebugging() final
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);

  void SendUpdatedInfoToMainThreadCache() final
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);

 private:
  ~CrossThreadMediaSourceAttachment() override;

  void RemoveTracksFromMediaElementInternal(TrackAddRemovalType track_type,
                                            Vector<String> track_ids,
                                            bool enqueue_change_event)
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);
  void RemoveTracksFromMediaElementOnMainThread(TrackAddRemovalType track_type,
                                                Vector<String> track_ids,
                                                bool enqueue_change_event)
      LOCKS_EXCLUDED(attachment_state_lock_);

  void AddTrackToMediaElementInternal(TrackAddRemovalType track_type,
                                      String id,
                                      String kind,
                                      String label,
                                      String language,
                                      bool enable_or_select)
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);
  void AddTrackToMediaElementOnMainThread(TrackAddRemovalType track_type,
                                          String id,
                                          String kind,
                                          String label,
                                          String language,
                                          bool enable_or_select)
      LOCKS_EXCLUDED(attachment_state_lock_);

  void CompleteAttachingToMediaElementOnWorkerThread(
      std::unique_ptr<WebMediaSource> web_media_source)
      LOCKS_EXCLUDED(attachment_state_lock_);

  void CloseOnWorkerThread() LOCKS_EXCLUDED(attachment_state_lock_);

  void UpdateWorkerThreadTimeCache(base::TimeDelta time)
      LOCKS_EXCLUDED(attachment_state_lock_);
  void HandleElementErrorOnWorkerThread()
      LOCKS_EXCLUDED(attachment_state_lock_);

  void SendUpdatedInfoToMainThreadCacheInternal(bool has_new_duration,
                                                double new_duration)
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);

  void UpdateMainThreadInfoCache(WebTimeRanges new_buffered,
                                 WebTimeRanges new_seekable,
                                 bool has_new_duration,
                                 double new_duration)
      LOCKS_EXCLUDED(attachment_state_lock_);

  // In this cross-thread implementation, this helper is used to verify
  // assumption of "liveness" of the attachment while the caller holds
  // |attachment_state_lock_| for common operations.
  void VerifyCalledWhileContextsAliveForDebugging() const
      EXCLUSIVE_LOCKS_REQUIRED(attachment_state_lock_);

  mutable base::Lock attachment_state_lock_;

  // Cache of the registered worker-thread MediaSource. Retains strong reference
  // on all Oilpan heaps, from construction of this object until Unregister() is
  // called. This lets the main thread successfully attach (modulo normal
  // reasons why StartAttaching..() can fail) to the worker-thread MediaSource
  // even if there were no other strong references other than this one on the
  // worker-thread Oilpan heap to the MediaSource.
  CrossThreadPersistent<MediaSource> registered_media_source_
      GUARDED_BY(attachment_state_lock_);

  // Task runner for posting cross-context information from the main thread to
  // the dedicated worker worker thread that owns |registered_media_source_|
  // (and |attached_media_source_| if currently attached). Rather than using
  // kMediaElementEvent task-type for this, we use kPostedMessage to have
  // similar scheduling priority as if the app instead postMessage'd the
  // information across context while maintaining similar causality. This is
  // used for servicing main thread element operations that require operation in
  // the MSE thread context. This is only valid until
  // |media_source_context_destroyed_| becomes true.
  scoped_refptr<base::SingleThreadTaskRunner> worker_runner_
      GUARDED_BY(attachment_state_lock_);

  // Task runner for posting cross-context information from the worker thread to
  // the main thread that owns |attached_element_| when currently attached.
  // Rather than using kMediaElementEvent task-type for this, we use
  // kPostedMessage to have similar scheduling priority as if the app instead
  // postMessage'd the information across context while maintaining similar
  // causality. This is used for servicing worker thread operations that require
  // operation in the main thread context. This is only valid until
  // |media_element_context_destroyed_| becomes true.
  scoped_refptr<base::SingleThreadTaskRunner> main_runner_
      GUARDED_BY(attachment_state_lock_);

  // In addition to serving as targets for cross-thread communication during a
  // live attachment, these two members function to keep Oilpan GC from
  // collecting either side of the cross-thread attached HTMLME+MSE object group
  // until explicit detachment. Unlike same-thread attachment's usage of Member
  // tracing to detect idle unused attached groups, cross-thread idle detection
  // is not available due to Oilpan's lack of CrossThreadMember.
  CrossThreadPersistent<MediaSource> attached_media_source_
      GUARDED_BY(attachment_state_lock_);
  CrossThreadPersistent<HTMLMediaElement> attached_element_
      GUARDED_BY(attachment_state_lock_);

  bool media_source_context_destroyed_ GUARDED_BY(attachment_state_lock_);
  bool media_element_context_destroyed_ GUARDED_BY(attachment_state_lock_);

  // Updated on worker thread as eventual result of kPostMessage-ing the time
  // received in OnElementTimeUpdate() on the main thread. Read on worker thread
  // synchronous to servicing GetRecentMediaTime().
  // See MediaSourceAttachment::OnElementTimeUpdate() interface comments for
  // more detail.
  base::TimeDelta recent_element_time_ GUARDED_BY(attachment_state_lock_);

  // Updated on worker thread as eventual result of kPostMessage-ing the
  // notification of element error received in OnElementError() on the main
  // thread. Read on worker thread synchronous to servicing GetElementError().
  // See MediaSourceAttachment::OnElementError() interface comments for more
  // detail.
  bool element_has_error_ GUARDED_BY(attachment_state_lock_);

  // TODO(https://crbug.com/878133): Handle supporting attachment-start success
  // even if RevokeMediaSourceObjectURLOnAttach is *not* enabled, and this
  // attachment instance (== object URL) is used sequentially for multiple
  // attachment lifetimes. Solution could be to ship that feature always-on soon
  // (making this scenario unsupported also in same-thread), or instead use a
  // counter here and in any cross-thread attachment task posting to ensure that
  // the cross-thread posted task is meant for the current attachment lifetime.
  // For now, this flag is used to prevent sequential usage of this attachment
  // instance to avoid this problem. (MSE-in-Workers is experimental, and
  // RevokeMediaSourceObjectURLOnAttach is on-by-default.)
  bool have_ever_attached_ GUARDED_BY(attachment_state_lock_);

  // TODO(https://crbug.com/878133): Similarly to |have_ever_attached_|, this
  // member becomes true once this attachment receives a Close() call. Some
  // operations, such as preventing usage of the underlying demuxer, or
  // nullifying cross-thread track notifications, need to know immediately in
  // the worker context once the asynchronous Close() call has ever occurred. If
  // support for sequential multiple attachment lifetimes is needed (for
  // instance, if MSE-in-Workers support is needed when
  // RevokeMediaSourceObjectURLOnAttach is *not* enabled), then a counter-based
  // solution may be required instead of this flag.
  bool have_ever_started_closing_ GUARDED_BY(attachment_state_lock_);

  WebTimeRanges cached_buffered_;
  WebTimeRanges cached_seekable_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_CROSS_THREAD_MEDIA_SOURCE_ATTACHMENT_H_
