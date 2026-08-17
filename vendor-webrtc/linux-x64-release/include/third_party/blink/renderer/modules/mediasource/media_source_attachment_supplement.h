// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_MEDIA_SOURCE_ATTACHMENT_SUPPLEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_MEDIA_SOURCE_ATTACHMENT_SUPPLEMENT_H_

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/html/media/media_source_attachment.h"
#include "third_party/blink/renderer/core/html/media/media_source_tracer.h"
#include "third_party/blink/renderer/core/html/track/audio_track.h"
#include "third_party/blink/renderer/core/html/track/video_track.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

class AudioTrackList;
class MediaSource;
class SourceBuffer;
class VideoTrackList;

// Modules-specific common extension of the core MediaSourceAttachment
// interface. Includes extra interface methods used by concrete attachments to
// communicate with the media element, as well as method implementations and
// members common to all concrete attachments.
class MediaSourceAttachmentSupplement : public MediaSourceAttachment {
 public:
  using ExclusiveKey = base::PassKey<MediaSourceAttachmentSupplement>;
  using RunExclusivelyCB = base::OnceCallback<void(ExclusiveKey)>;
  using SourceBufferPassKey = base::PassKey<SourceBuffer>;

  MediaSourceAttachmentSupplement(const MediaSourceAttachmentSupplement&) =
      delete;
  MediaSourceAttachmentSupplement& operator=(
      const MediaSourceAttachmentSupplement&) = delete;

  // Communicates a change in the media resource duration to the attached media
  // element. In a same-thread attachment, communicates this information
  // synchronously. In a cross-thread attachment, underlying WebMediaSource
  // should already be asynchronously communicating this information to the
  // media element, so attachment operation is a no-op. Same-thread synchronous
  // notification here is primarily to preserve compliance of API behavior when
  // not using MSE-in-Worker (setting MediaSource.duration should be
  // synchronously in agreement with subsequent retrieval of
  // MediaElement.duration, all on the main thread).
  virtual void NotifyDurationChanged(MediaSourceTracer* tracer,
                                     double duration) = 0;

  // Retrieves the current (or a recent) media element time. Implementations may
  // choose to either directly, synchronously consult the attached media element
  // (via |tracer| in a same thread implementation) or rely on a "recent"
  // currentTime pumped by the attached element via the MediaSourceAttachment
  // interface (in a cross-thread implementation).
  virtual base::TimeDelta GetRecentMediaTime(MediaSourceTracer* tracer) = 0;

  // Retrieves whether or not the media element currently has an error.
  // Implementations may choose to either directly, synchronously consult the
  // attached media element (via |tracer| in a same thread implementation) or
  // rely on the element to correctly pump when it has an error to this
  // attachment (in a cross-thread implementation).
  virtual bool GetElementError(MediaSourceTracer* tracer) = 0;

  // Construct track lists for use by a SourceBuffer.
  // TODO(https://crbug.com/878133): Update these to support worker-owned
  // SourceBuffer lists when completing AudioVideoTracks feature support with
  // MSE-in-Workers.
  virtual AudioTrackList* CreateAudioTrackList(MediaSourceTracer* tracer) = 0;
  virtual VideoTrackList* CreateVideoTrackList(MediaSourceTracer* tracer) = 0;

  // Add/Remove tracks to/from the media Element's audioTracks() or
  // videoTracks() list. Note that this is synchronous in
  // SameThreadMediaSourceAttachment, but the CrossThreadMediaSourceAttachment
  // does a cross-thread task post and performs the operations on the main
  // thread to enable correct context ownership of created tracks and correct
  // context for track list operations.
  virtual void AddAudioTrackToMediaElement(MediaSourceTracer* tracer,
                                           AudioTrack* track) = 0;
  virtual void AddVideoTrackToMediaElement(MediaSourceTracer* tracer,
                                           VideoTrack* track) = 0;
  virtual void RemoveAudioTracksFromMediaElement(MediaSourceTracer* tracer,
                                                 Vector<String> audio_ids,
                                                 bool enqueue_change_event) = 0;
  virtual void RemoveVideoTracksFromMediaElement(MediaSourceTracer* tracer,
                                                 Vector<String> video_ids,
                                                 bool enqueue_change_event) = 0;
  // TODO(https://crbug.com/878133): Update the implementations to remove these
  // short-term cross-thread helpers and instead use the methods, above, once
  // track creation outside of main thread is supported. These helpers create
  // the tracks on the main thread from parameters (not from a currently-
  // uncreatable worker thread track).
  virtual void AddMainThreadAudioTrackToMediaElement(String id,
                                                     String kind,
                                                     String label,
                                                     String language,
                                                     bool enabled);
  virtual void AddMainThreadVideoTrackToMediaElement(String id,
                                                     String kind,
                                                     String label,
                                                     String language,
                                                     bool selected);

  virtual void OnMediaSourceContextDestroyed() = 0;

  // Default is to just run the cb (e.g., for same-thread implementation of the
  // attachment, since both the media element and the MSE API operate on the
  // same thread and no mutex is required.) Cross-thread implementation will
  // first take a lock, then run the cb conditionally on
  // |abort_if_not_fully_attached|, then release a lock (all synchronously on
  // the same thread.) Any return values needed by the caller should be passed
  // by pointer, aka as "out" arguments. For cross-thread case, see further
  // detail in it's override declaration and implementation. PassKey pattern
  // usage (with ExclusiveKey instance passed to |cb| when running it)
  // statically ensures that only a MediaSourceAttachmentSupplement
  // implementation can run such a |cb|. |cb| can then be assured that it is run
  // within the scope of this method.
  virtual bool RunExclusively(bool abort_if_not_fully_attached,
                              RunExclusivelyCB cb);

  // Simpler than RunExclusively(), the default implementation returns true
  // always. CrossThreadMediaSourceAttachment implementation should first verify
  // the lock is already held, then return true iff the media element is still
  // attached, has not yet signaled that its context's destruction has begun,
  // and has not yet told the attachment to Close() - these conditions mirror
  // the cross-thread RunExclusively checks when |abort_if_not_fully_attached|
  // is true. This helper is expected to only be used by SourceBuffer's
  // RemovedFromMediaSource(), hence we use the PassKey pattern to help enforce
  // that.
  virtual bool FullyAttachedOrSameThread(SourceBufferPassKey) const;

  // Default implementation fails DCHECK. See CrossThreadMediaSourceAttachment
  // for override. MediaSource and SourceBuffer use this to help verify they
  // only use underlying demuxer in cross-thread debug case while the
  // cross-thread mutex is held.
  virtual void AssertCrossThreadMutexIsAcquiredForDebugging();

  // No-op for same-thread attachmenets. For cross-thread attachments,
  // calculates current buffered and seekable on the worker thread, then posts
  // the results to the main thread to service media element queries of that
  // information with latency and causality similar to app postMessage() from
  // worker to main thread.
  virtual void SendUpdatedInfoToMainThreadCache();

 protected:
  MediaSourceAttachmentSupplement();
  ~MediaSourceAttachmentSupplement() override;

  ExclusiveKey GetExclusiveKey() const;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_MEDIA_SOURCE_ATTACHMENT_SUPPLEMENT_H_
