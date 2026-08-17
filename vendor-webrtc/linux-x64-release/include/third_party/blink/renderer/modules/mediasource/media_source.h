// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_MEDIA_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_MEDIA_SOURCE_H_

#include <memory>
#include <tuple>
#include <utility>

#include "base/memory/scoped_refptr.h"
#include "base/synchronization/lock.h"
#include "third_party/blink/public/platform/web_media_source.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/html/media/media_source_tracer.h"
#include "third_party/blink/renderer/core/html/time_ranges.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/mediasource/media_source_attachment_supplement.h"
#include "third_party/blink/renderer/modules/mediasource/media_source_handle_impl.h"
#include "third_party/blink/renderer/modules/mediasource/source_buffer.h"
#include "third_party/blink/renderer/modules/mediasource/source_buffer_list.h"
#include "third_party/blink/renderer/platform/bindings/exception_code.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace media {
class AudioDecoderConfig;
class VideoDecoderConfig;
}  // namespace media

namespace blink {

class EventQueue;
class ExceptionState;
class HTMLMediaElement;
class CrossThreadMediaSourceAttachment;
class SameThreadMediaSourceAttachment;
class SourceBufferConfig;
class TrackBase;
class V8EndOfStreamError;
class WebSourceBuffer;

// Media Source Extensions (MSE) API's MediaSource object implementation (see
// also https://w3.org/TR/media-source/). Web apps can extend an
// HTMLMediaElement's instance to use the MSE API (also known as "attaching MSE
// to a media element") by using a Media Source object URL as the media
// element's src attribute or the src attribute of a <source> inside the media
// element, or by using a MediaSourceHandle for a worker-owned MediaSource as
// the srcObject of the media element. A MediaSourceAttachmentSupplement
// encapsulates the linkage of that object URL or handle to a MediaSource
// instance, and allows communication between the media element and the MSE API.
class MediaSource final : public EventTarget,
                          public ActiveScriptWrappable<MediaSource>,
                          public ExecutionContextLifecycleObserver {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum class ReadyState { kOpen, kClosed, kEnded };

  static MediaSource* Create(ExecutionContext*);

  explicit MediaSource(ExecutionContext*);
  ~MediaSource() override;

  static void LogAndThrowDOMException(ExceptionState&,
                                      DOMExceptionCode error,
                                      const String& message);
  static void LogAndThrowTypeError(ExceptionState&, const String&);

  // Web-exposed methods from media_source.idl
  SourceBufferList* sourceBuffers() { return source_buffers_.Get(); }
  SourceBufferList* activeSourceBuffers() {
    return active_source_buffers_.Get();
  }
  SourceBuffer* addSourceBuffer(const String& type, ExceptionState&)
      LOCKS_EXCLUDED(attachment_link_lock_);
  SourceBuffer* AddSourceBufferUsingConfig(ExecutionContext* execution_context,
                                           const SourceBufferConfig*,
                                           ExceptionState&)
      LOCKS_EXCLUDED(attachment_link_lock_);
  void removeSourceBuffer(SourceBuffer*, ExceptionState&)
      LOCKS_EXCLUDED(attachment_link_lock_);
  void setDuration(double, ExceptionState&)
      LOCKS_EXCLUDED(attachment_link_lock_);
  double duration() LOCKS_EXCLUDED(attachment_link_lock_);

  DEFINE_ATTRIBUTE_EVENT_LISTENER(sourceopen, kSourceopen)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(sourceended, kSourceended)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(sourceclose, kSourceclose)

  AtomicString readyState() const;
  void endOfStream(ExceptionState&) LOCKS_EXCLUDED(attachment_link_lock_);
  void endOfStream(std::optional<V8EndOfStreamError> error, ExceptionState&)
      LOCKS_EXCLUDED(attachment_link_lock_);
  void setLiveSeekableRange(double start, double end, ExceptionState&)
      LOCKS_EXCLUDED(attachment_link_lock_);
  void clearLiveSeekableRange(ExceptionState&)
      LOCKS_EXCLUDED(attachment_link_lock_);

  MediaSourceHandleImpl* handle() LOCKS_EXCLUDED(attachment_link_lock_);

  static bool isTypeSupported(ExecutionContext* context, const String& type);

  // Helper for isTypeSupported, addSourceBuffer and SourceBuffer changeType.
  // Set |enforce_codec_specificity| true to require fully specified mime and
  // codecs, false otherwise.
  // TODO(https://crbug.com/535738): When |enforce_codec_specificity| is set to
  // false, then fully relax codec requirements.
  static bool IsTypeSupportedInternal(ExecutionContext* context,
                                      const String& type,
                                      bool enforce_codec_specificity);

  static bool canConstructInDedicatedWorker(ExecutionContext* context);

  // Methods needed by a MediaSourceAttachmentSupplement to service operations
  // proxied from an HTMLMediaElement.
  MediaSourceTracer* StartAttachingToMediaElement(
      scoped_refptr<SameThreadMediaSourceAttachment> attachment,
      HTMLMediaElement* element) LOCKS_EXCLUDED(attachment_link_lock_);
  // Same method as above, but for starting an attachment when we are
  // MSE-in-Workers and therefore using a CrossThreadMediaSourceAttachment.
  // Returns true iff successfully started. Even in this case, this method is
  // called on the main thread and operates synchronously throughout.
  bool StartWorkerAttachingToMainThreadMediaElement(
      scoped_refptr<CrossThreadMediaSourceAttachment> attachment)
      LOCKS_EXCLUDED(attachment_link_lock_);
  // Completing the attachment always occurs on the thread/context that owns
  // this MediaSource.
  void CompleteAttachingToMediaElement(std::unique_ptr<WebMediaSource>)
      LOCKS_EXCLUDED(attachment_link_lock_);
  void Close();
  bool IsClosed() const;
  double GetDuration_Locked(
      MediaSourceAttachmentSupplement::ExclusiveKey /* passkey */) const
      LOCKS_EXCLUDED(attachment_link_lock_);
  WebTimeRanges BufferedInternal(
      MediaSourceAttachmentSupplement::ExclusiveKey /* passkey */) const
      LOCKS_EXCLUDED(attachment_link_lock_);
  WebTimeRanges SeekableInternal(
      MediaSourceAttachmentSupplement::ExclusiveKey /* passkey */) const
      LOCKS_EXCLUDED(attachment_link_lock_);
  void OnTrackChanged(TrackBase*);

  // EventTarget interface
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;

  // ScriptWrappable
  bool HasPendingActivity() const final;

  // ExecutionContextLifecycleObserver interface
  void ContextDestroyed() override LOCKS_EXCLUDED(attachment_link_lock_);

  // Methods used by SourceBuffer.
  //
  // OpenIfInEndedState must not be called when IsClosed() is true. Furthermore,
  // the caller must ensure this is only called from the attachment's
  // RunExclusively() callback.
  void OpenIfInEndedState() LOCKS_EXCLUDED(attachment_link_lock_);
  bool IsOpen() const;
  void SetSourceBufferActive(SourceBuffer*, bool);
  std::pair<scoped_refptr<MediaSourceAttachmentSupplement>, MediaSourceTracer*>
  AttachmentAndTracer() const LOCKS_EXCLUDED(attachment_link_lock_);
  // EndOfStreamAlgorithm must not be called when IsClosed() is true.
  // Furthermore, the caller must ensure this is only called from the
  // attachment's RunExclusively() callback.
  void EndOfStreamAlgorithm(
      const WebMediaSource::EndOfStreamStatus,
      MediaSourceAttachmentSupplement::ExclusiveKey pass_key)
      LOCKS_EXCLUDED(attachment_link_lock_);

  // Helper to run operations while holding cross-thread attachment's exclusive
  // |attachment_state_lock_|. This is used for safe cross-thread operation when
  // MSE is in worker context, especially when accessing underlying demuxer.
  // Returns true if |cb| was run. Returns false if |cb| was not run (because
  // the element is gone or is closing us). Caller must ensure that we
  // currently have an attachment (typically by checking that our readyState is
  // not closed, or similar).
  bool RunUnlessElementGoneOrClosingUs(
      MediaSourceAttachmentSupplement::RunExclusivelyCB cb)
      LOCKS_EXCLUDED(attachment_link_lock_);

  // Helper to verify cross-thread attachment's |attachment_state_lock_| mutex
  // is acquired whenever we are accessing the underlying demuxer.
  void AssertAttachmentsMutexHeldIfCrossThreadForDebugging() const
      LOCKS_EXCLUDED(attachment_link_lock_);

  // Helper to tell the attachment to send updated buffered and seekable
  // information to the media element if cross-thread.
  void SendUpdatedInfoToMainThreadCache() LOCKS_EXCLUDED(attachment_link_lock_);

  void Trace(Visitor*) const override LOCKS_EXCLUDED(attachment_link_lock_);

 private:
  // Helpers used as bound callbacks with RunExclusively() or
  // RunUnlessElementGoneOrClosingUs() because they access underlying demuxer
  // resources owned by the main thread. Other methods without "_Locked" may
  // also require the same, since they can be called from within these methods.
  void AddSourceBuffer_Locked(
      const String& type /* in */,
      std::unique_ptr<media::AudioDecoderConfig> audio_config /* in */,
      std::unique_ptr<media::VideoDecoderConfig> video_config /* in */,
      ExceptionState* exception_state /* in/out */,
      SourceBuffer** created_buffer /* out */,
      MediaSourceAttachmentSupplement::ExclusiveKey /* passkey */)
      LOCKS_EXCLUDED(attachment_link_lock_);
  void RemoveSourceBuffer_Locked(
      SourceBuffer* buffer /* in */,
      MediaSourceAttachmentSupplement::ExclusiveKey /* passkey */)
      LOCKS_EXCLUDED(attachment_link_lock_);
  void SetLiveSeekableRange_Locked(
      double start,
      double end,
      MediaSourceAttachmentSupplement::ExclusiveKey /* passkey */)
      LOCKS_EXCLUDED(attachment_link_lock_);
  void ClearLiveSeekableRange_Locked(
      MediaSourceAttachmentSupplement::ExclusiveKey /* passkey */)
      LOCKS_EXCLUDED(attachment_link_lock_);
  void DetachWorkerOnContextDestruction_Locked(
      bool notify_close,
      MediaSourceAttachmentSupplement::ExclusiveKey /* passkey */)
      LOCKS_EXCLUDED(attachment_link_lock_);

  // Other helpers.
  void SetReadyState(const ReadyState state)
      LOCKS_EXCLUDED(attachment_link_lock_);
  void OnReadyStateChange(const ReadyState old_state,
                          const ReadyState new_state)
      LOCKS_EXCLUDED(attachment_link_lock_);

  bool IsUpdating() const;

  std::unique_ptr<WebSourceBuffer> CreateWebSourceBuffer(
      const String& type,
      const String& codecs,
      std::unique_ptr<media::AudioDecoderConfig> audio_config,
      std::unique_ptr<media::VideoDecoderConfig> video_config,
      ExceptionState&) LOCKS_EXCLUDED(attachment_link_lock_);
  void ScheduleEvent(const AtomicString& event_name);
  static void RecordIdentifiabilityMetric(ExecutionContext* context,
                                          const String& type,
                                          bool result);

  // Implements the duration change algorithm.
  // http://w3c.github.io/media-source/#duration-change-algorithm
  void DurationChangeAlgorithm(
      double new_duration,
      ExceptionState*,
      MediaSourceAttachmentSupplement::ExclusiveKey pass_key)
      LOCKS_EXCLUDED(attachment_link_lock_);

  // Usage of |*web_media_source_| must be within scope of attachment's
  // RunExclusively() callback to prevent potential UAF of underlying demuxer
  // resources when MSE is in worker thread. Setting it or resetting it do not
  // require being in that critical section though.
  // TODO(https://crbug.com/878133): Add comment to blink::WebMediaSource and
  // blink::WebSourceBuffer to indicate which methods (currently only their
  // destructors) are safe to be called from MSE-in-Worker unless measures such
  // as the CrossThreadMediaSourceAttachment's RunExclusively() callback are
  // ensuring the underlying demuxer is still alive.
  std::unique_ptr<WebMediaSource> web_media_source_;

  ReadyState ready_state_;
  Member<EventQueue> async_event_queue_;

  // Keep the attached element (via attachment_tracer_), |source_buffers_|,
  // |active_source_buffers_|, and their wrappers from being collected if we are
  // alive or traceable from a GC root. Activity by this MediaSource or on
  // references to objects returned by exercising this MediaSource (such as an
  // app manipulating a SourceBuffer retrieved via activeSourceBuffers()) may
  // cause events to be dispatched by these other objects.
  // |media_source_attachment_| and |attachment_tracer_| must be carefully set
  // and reset: the actual derived type of the attachment (same-thread vs
  // cross-thread, for instance) must be the same semantic as the actual derived
  // type of the tracer. Further, if there is no attachment, then there must be
  // no tracer that's tracking an active attachment.
  scoped_refptr<MediaSourceAttachmentSupplement> media_source_attachment_
      GUARDED_BY(attachment_link_lock_);
  Member<MediaSourceTracer> attachment_tracer_
      GUARDED_BY(attachment_link_lock_);
  bool context_already_destroyed_ GUARDED_BY(attachment_link_lock_);

  Member<MediaSourceHandleImpl> worker_media_source_handle_;

  // |attachment_link_lock_| protects read/write of |media_source_attachment_|,
  // |attachment_tracer_|, |context_already_destroyed_|, and execution of
  // handle().
  // It is only truly necessary for CrossThreadAttachment usage of worker MSE,
  // to prevent read/write collision on main thread versus worker thread. Note
  // that |attachment_link_lock_| must be released before attempting
  // CrossThreadMediaSourceAttachment RunExclusively() to avoid deadlock. Many
  // scenarios initiated by worker thread need to get the attachment to be able
  // to invoke operations on it. The attachment then takes internal
  // |attachment_state_lock_|and verifies state. Note that between releasing
  // |attachment_link_lock_| and the RunExclusively() operation on the
  // attachment taking its internal |attachment_state_lock_|, the attachment
  // state could have changed, but the attachment understands how to resolve
  // such cases.  Note that |web_media_source_| and child SourceBuffers'
  // |web_source_buffer_|s usage are protected by only being attempted in scope
  // of a RunExclusively callback (to prevent usage of them if the underlying
  // demuxer owned by the main thread is no longer available).
  // TODO(https://crbug.com/878133): Consider optimizing away (e.g., using
  // macros, conditional logic, or virtual implementations) usage of
  // |attachment_link_lock_| and
  // callbacks for RunExclusively, when using SameThreadMediaSourceAttachment,
  // on main thread).
  mutable base::Lock attachment_link_lock_;

  Member<SourceBufferList> source_buffers_;
  Member<SourceBufferList> active_source_buffers_;

  // These are kept as raw data (not an Oilpan managed GC-able TimeRange) to
  // avoid need to take lock during ::Trace, which could lead to deadlock. They
  // are updated or read only while the attachment's internal lock is held
  // (during the scope of a RunExclusively callback, for example).
  bool has_live_seekable_range_;
  double live_seekable_range_start_;
  double live_seekable_range_end_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_MEDIA_SOURCE_H_
