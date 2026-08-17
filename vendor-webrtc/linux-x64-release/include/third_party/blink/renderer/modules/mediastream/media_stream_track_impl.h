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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_IMPL_H_

#include <memory>

#include "build/build_config.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_capture_handle.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/mediastream/media_constraints.h"
#include "third_party/blink/renderer/modules/mediastream/media_stream_track.h"
#include "third_party/blink/renderer/modules/mediastream/speech_recognition_media_stream_audio_sink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_descriptor.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_source.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class AudioSourceProvider;
class ImageCapture;
class MediaTrackCapabilities;
class MediaTrackConstraints;
class MediaStream;
class MediaTrackSettings;
class ScriptState;

// Primary implementation of the MediaStreamTrack interface and idl type.
class MODULES_EXPORT MediaStreamTrackImpl : public MediaStreamTrack,
                                            public MediaStreamSource::Observer {
 public:
  // Create a MediaStreamTrackImpl of the appropriate type for the display
  // surface type.
  static MediaStreamTrack* Create(ExecutionContext* context,
                                  MediaStreamComponent* component,
                                  base::OnceClosure callback);

  MediaStreamTrackImpl(ExecutionContext*, MediaStreamComponent*);
  MediaStreamTrackImpl(ExecutionContext*,
                       MediaStreamComponent*,
                       base::OnceClosure callback);
  MediaStreamTrackImpl(ExecutionContext*,
                       MediaStreamComponent*,
                       MediaStreamSource::ReadyState,
                       base::OnceClosure callback,
                       bool is_clone = false);
  ~MediaStreamTrackImpl() override;

  // MediaStreamTrack
  String kind() const override;
  String id() const override;
  String label() const override;
  bool enabled() const override;
  void setEnabled(bool) override;
  bool muted() const override;
  String ContentHint() const override;
  void SetContentHint(const String&) override;
  V8MediaStreamTrackState readyState() const override;
  MediaStreamTrack* clone(ExecutionContext*) override;
  void stopTrack(ExecutionContext*) override;
  MediaTrackCapabilities* getCapabilities() const override;
  MediaTrackConstraints* getConstraints() const override;
  MediaTrackSettings* getSettings() const override;
  V8UnionMediaStreamTrackAudioStatsOrMediaStreamTrackVideoStats* stats()
      override;
  CaptureHandle* getCaptureHandle() const override;
  ScriptPromise<IDLUndefined> applyConstraints(
      ScriptState*,
      const MediaTrackConstraints*) override;

  // These two functions are called when constraints have been successfully
  // applied.
  // Called from UserMediaRequest when it succeeds. It is not IDL-exposed.
  // SetInitialConstraints() is expected to be called once when capture starts.
  // SetConstraints() is called later, when changing the set of constraints.
  void SetInitialConstraints(const MediaConstraints& constraints) override;
  void SetConstraints(const MediaConstraints& constraints) override;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(mute, kMute)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(unmute, kUnmute)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(ended, kEnded)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(capturehandlechange, kCapturehandlechange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(configurationchange, kConfigurationchange)

  // Returns the enum value of the ready state.
  MediaStreamSource::ReadyState GetReadyState() override {
    return ready_state_;
  }

  MediaStreamComponent* Component() const override { return component_.Get(); }
  bool Ended() const override;

  void RegisterMediaStream(MediaStream*) override;
  void UnregisterMediaStream(MediaStream*) override;

  void RegisterSink(SpeechRecognitionMediaStreamAudioSink*) override;

  // EventTarget
  const AtomicString& InterfaceName() const override;
  ExecutionContext* GetExecutionContext() const override;
  void AddedEventListener(const AtomicString&,
                          RegisteredEventListener&) override;

  // ScriptWrappable
  bool HasPendingActivity() const final;

  std::unique_ptr<AudioSourceProvider> CreateWebAudioSource(
      int context_sample_rate,
      base::TimeDelta platform_buffer_duration) override;

  MediaStreamTrackPlatform::VideoFrameStats GetVideoFrameStats() const;

  void TransferAudioFrameStatsTo(
      MediaStreamTrackPlatform::AudioFrameStats& destination);

  ImageCapture* GetImageCapture() override { return image_capture_.Get(); }

  std::optional<const MediaStreamDevice> device() const override;

  void BeingTransferred(const base::UnguessableToken& transfer_id) override;
  bool TransferAllowed(String& message) const override;

  void AddObserver(MediaStreamTrack::Observer*) override;

  void Trace(Visitor*) const override;

  std::optional<int> GetZoomLevelForTesting() const { return zoom_level_; }

  bool IsCapturedSurfaceResolutionActive(
      const MediaStreamTrackPlatform::Settings& platform_settings) const;

 protected:
  // Given a partially built MediaStreamTrackImpl, finishes the job of making it
  // into a clone of |this|.
  // Useful for sub-classes, as they need to clone both state from
  // this class as well as of their own class.
  void CloneInternal(MediaStreamTrackImpl*);

 private:
  friend class CanvasCaptureMediaStreamTrack;
  friend class InternalsMediaStream;

  // MediaStreamTrack
  void applyConstraints(ScriptPromiseResolver<IDLUndefined>*,
                        const MediaTrackConstraints*) override;

  // MediaStreamSource::Observer
  void SourceChangedState() override;
  void SourceChangedCaptureConfiguration() override;
  void SourceChangedCaptureHandle() override;
#if !BUILDFLAG(IS_ANDROID) && !BUILDFLAG(IS_IOS)
  void SourceChangedZoomLevel(int) override;
#endif
  void PropagateTrackEnded();

  void PropagateTrackEnabled(bool enabled);

  void SendLogMessage(const WTF::String& message);

  // Ensures that |feature_handle_for_scheduler_| is initialized.
  void EnsureFeatureHandleForScheduler();

  void SetConstraintsInternal(const MediaConstraints& constraints,
                              bool initial_values);

  void setReadyState(MediaStreamSource::ReadyState ready_state);

  // Callback used after getting the current image capture capabilities and
  // settings to dispatch a configurationchange event if they differ from the
  // old ones.
  void MaybeDispatchConfigurationChange(bool has_changed);

  // This handle notifies the scheduler about a live media stream track
  // associated with a frame. The handle should be destroyed when the track
  // is stopped.
  FrameScheduler::SchedulingAffectingFeatureHandle
      feature_handle_for_scheduler_;

  // This handle notifies the scheduler about a live media stream track
  // for the purpose of disabling/enabling BFCache. When there is a live stream
  // track, the page should not be BFCached.
  // TODO(crbug.com/1502395): Currently we intentionally use this handler for
  // BFCache although its behavior is almost the same as the one above. The one
  // above uses the WebRTC feature even though it's not necessarily related to
  // Web RTC. Discuss with those who own the handler and merge the two handlers
  // into one.
  FrameScheduler::SchedulingAffectingFeatureHandle
      feature_handle_for_scheduler_on_live_media_stream_track_;

  MediaStreamSource::ReadyState ready_state_;
  HeapHashSet<Member<MediaStream>> registered_media_streams_;
  HeapHashSet<Member<SpeechRecognitionMediaStreamAudioSink>> registered_sinks_;
  bool is_iterating_registered_media_streams_ = false;
  const Member<MediaStreamComponent> component_;
  Member<ImageCapture> image_capture_;
  WeakMember<ExecutionContext> execution_context_;
  HeapHashSet<WeakMember<MediaStreamTrack::Observer>> observers_;
  bool muted_ = false;
  std::optional<int> zoom_level_;
  MediaConstraints constraints_;
  std::optional<bool> suppress_local_audio_playback_setting_;
  Member<V8UnionMediaStreamTrackAudioStatsOrMediaStreamTrackVideoStats> stats_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_IMPL_H_
