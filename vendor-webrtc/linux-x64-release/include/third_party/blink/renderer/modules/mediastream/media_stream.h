/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 * Copyright (C) 2011 Ericsson AB. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_track.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_descriptor.h"
#include "third_party/blink/renderer/platform/timer.h"

namespace blink {

class ExceptionState;
class ExecutionContext;
class ScriptState;
class TransferredMediaStreamTrack;

class MODULES_EXPORT MediaStreamObserver : public GarbageCollectedMixin {
 public:
  virtual ~MediaStreamObserver() = default;

  // Invoked when |MediaStream::addTrack| is called.
  virtual void OnStreamAddTrack(MediaStream*,
                                MediaStreamTrack*,
                                ExceptionState& exception_state) = 0;
  // Invoked when |MediaStream::removeTrack| is called.
  virtual void OnStreamRemoveTrack(MediaStream*,
                                   MediaStreamTrack*,
                                   ExceptionState& exception_state) = 0;

  void Trace(Visitor* visitor) const override {}
};

class MODULES_EXPORT MediaStream final
    : public EventTarget,
      public ExecutionContextClient,
      public ActiveScriptWrappable<MediaStream>,
      public MediaStreamDescriptorClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static MediaStream* Create(ExecutionContext*);
  static MediaStream* Create(ExecutionContext*, MediaStream*);
  static MediaStream* Create(ExecutionContext*, const MediaStreamTrackVector&);
  // Creates a MediaStream matching the MediaStreamDescriptor. MediaStreamTracks
  // are created for any MediaStreamComponents attached to the descriptor.
  static MediaStream* Create(ExecutionContext*, MediaStreamDescriptor*);
  // Creates a MediaStream matching the MediaStreamDescriptor. MediaStreamTracks
  // are created for any MediaStreamComponents attached to the descriptor. If
  // TransferredMediaStreamTrack != nullptr, exactly one track is expected in
  // the MediaStreamDescriptor, and the created track will be used as the
  // underlying implementation of this track. The stream is returned via
  // callback.
  static void Create(ExecutionContext*,
                     MediaStreamDescriptor*,
                     TransferredMediaStreamTrack*,
                     base::OnceCallback<void(MediaStream*)> callback);
  // Creates a MediaStream with the specified MediaStreamDescriptor and
  // MediaStreamTracks. The tracks must match the MediaStreamComponents attached
  // to the descriptor (or else a DCHECK fails). This allows you to create
  // multiple streams from descriptors containing the same components without
  // creating duplicate MediaStreamTracks for those components, provided the
  // caller knows about existing tracks.
  // This is motivated by WebRTC, where remote streams can be created for tracks
  // that already exist in Blink (e.g. multiple remote streams containing the
  // same track).
  static MediaStream* Create(ExecutionContext*,
                             MediaStreamDescriptor*,
                             const MediaStreamTrackVector& audio_tracks,
                             const MediaStreamTrackVector& video_tracks);

  MediaStream(ExecutionContext*,
              MediaStreamDescriptor*,
              TransferredMediaStreamTrack*,
              base::OnceCallback<void(MediaStream*)> callback);
  MediaStream(ExecutionContext*,
              MediaStreamDescriptor*,
              const MediaStreamTrackVector& audio_tracks,
              const MediaStreamTrackVector& video_tracks);
  MediaStream(ExecutionContext*,
              const MediaStreamTrackVector& audio_tracks,
              const MediaStreamTrackVector& video_tracks);
  ~MediaStream() override;

  String id() const { return descriptor_->Id(); }

  // Adds the track, this may cause "onactive" to fire but it won't cause
  // "onaddtrack" because the track was added explicitly by the JavaScript
  // application.
  void addTrack(MediaStreamTrack*, ExceptionState&);
  void removeTrack(MediaStreamTrack*, ExceptionState&);
  MediaStreamTrack* getTrackById(String);
  MediaStream* clone(ScriptState*);

  MediaStreamTrackVector getAudioTracks() const { return audio_tracks_; }
  MediaStreamTrackVector getVideoTracks() const { return video_tracks_; }
  MediaStreamTrackVector getTracks();

  bool active() const { return descriptor_->Active(); }

  DEFINE_ATTRIBUTE_EVENT_LISTENER(active, kActive)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(inactive, kInactive)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(addtrack, kAddtrack)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(removetrack, kRemovetrack)

  void TrackEnded();

  void RegisterObserver(MediaStreamObserver*);
  void UnregisterObserver(MediaStreamObserver*);

  void StreamEnded();

  void NotifyEnabledStateChangeForWebRtcAudio(bool enabled);

  // MediaStreamDescriptorClient implementation
  void AddTrackByComponentAndFireEvents(MediaStreamComponent*,
                                        DispatchEventTiming) override;
  void RemoveTrackByComponentAndFireEvents(MediaStreamComponent*,
                                           DispatchEventTiming) override;

  // Adds the track and, unlike JavaScript-invoked addTrack(), fires related
  // events like "onaddtrack" either synchronously or in a scheduled event.
  void AddTrackAndFireEvents(MediaStreamTrack*, DispatchEventTiming);
  void RemoveTrackAndFireEvents(MediaStreamTrack*, DispatchEventTiming);

  void AddRemoteTrack(MediaStreamTrack*);
  void RemoveRemoteTrack(MediaStreamTrack*);

  MediaStreamDescriptor* Descriptor() const { return descriptor_.Get(); }

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override {
    return ExecutionContextClient::GetExecutionContext();
  }

  // ActiveScriptWrappable
  bool HasPendingActivity() const override;

  void Trace(Visitor*) const override;

 protected:
  bool AddEventListenerInternal(
      const AtomicString& event_type,
      EventListener*,
      const AddEventListenerOptionsResolved*) override;

 private:
  bool EmptyOrOnlyEndedTracks();
  bool TracksMatchDescriptor();

  void ScheduleDispatchEvent(Event*);
  void ScheduledEventTimerFired(TimerBase*);

  void OnMediaStreamTrackInitialized();

  MediaStreamTrackVector audio_tracks_;
  MediaStreamTrackVector video_tracks_;
  Member<MediaStreamDescriptor> descriptor_;
  // Observers are informed when |addTrack| and |removeTrack| are called.
  HeapHashSet<WeakMember<MediaStreamObserver>> observers_;

  // The callback to be called when the media stream is fully initialized,
  // including image capture for video tracks.
  base::OnceCallback<void(MediaStream*)> media_stream_initialized_callback_;

  HeapTaskRunnerTimer<MediaStream> scheduled_event_timer_;
  HeapVector<Member<Event>> scheduled_events_;

  uint32_t number_of_video_tracks_initialized_ = 0;
};

using MediaStreamVector = HeapVector<Member<MediaStream>>;

MediaStream* ToMediaStream(MediaStreamDescriptor*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_H_
