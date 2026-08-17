// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "base/time/time.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_capture_handle.h"
#include "third_party/blink/renderer/modules/event_target_modules.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_descriptor.h"
#include "third_party/blink/renderer/platform/mediastream/media_stream_source.h"
#include "third_party/blink/renderer/platform/scheduler/public/frame_scheduler.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

static const char kContentHintStringNone[] = "";
static const char kContentHintStringAudioSpeech[] = "speech";
static const char kContentHintStringAudioMusic[] = "music";
static const char kContentHintStringVideoMotion[] = "motion";
static const char kContentHintStringVideoDetail[] = "detail";
static const char kContentHintStringVideoText[] = "text";

class AudioSourceProvider;
class ImageCapture;
class MediaConstraints;
class MediaTrackCapabilities;
class MediaTrackConstraints;
class MediaStream;
class MediaTrackSettings;
class ScriptState;
class SpeechRecognitionMediaStreamAudioSink;
class V8MediaStreamTrackState;
class V8UnionMediaStreamTrackAudioStatsOrMediaStreamTrackVideoStats;

String ContentHintToString(
    const WebMediaStreamTrack::ContentHintType& content_hint);

V8MediaStreamTrackState ReadyStateToV8TrackState(
    const MediaStreamSource::ReadyState& ready_state);

class MODULES_EXPORT MediaStreamTrack
    : public EventTarget,
      public ActiveScriptWrappable<MediaStreamTrack> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  class MODULES_EXPORT Observer : public GarbageCollectedMixin {
   public:
    virtual ~Observer() = default;
    virtual void TrackChangedState() = 0;
  };

  // For carrying data to the FromTransferredState method.
  struct TransferredValues {
    raw_ptr<const WrapperTypeInfo> track_impl_subtype;
    base::UnguessableToken session_id;
    base::UnguessableToken transfer_id;
    String kind;
    String id;
    String label;
    bool enabled;
    bool muted;
    WebMediaStreamTrack::ContentHintType content_hint;
    MediaStreamSource::ReadyState ready_state;
    // Set only if
    // track_impl_subtype->IsSubclass(BrowserCaptureMediaStreamTrack::GetStaticWrapperTypeInfo())
    std::optional<uint32_t> sub_capture_target_version;
  };

  // See SetFromTransferredStateImplForTesting in ./test/transfer_test_utils.h.
  using FromTransferredStateImplForTesting =
      base::RepeatingCallback<MediaStreamTrack*(const TransferredValues&)>;

  // Create a MediaStreamTrack instance as a result of a transfer into this
  // context, eg when receiving a postMessage() with an MST in the transfer
  // list.
  // TODO(https://crbug.com/1288839): Implement to recreate MST after transfer
  static MediaStreamTrack* FromTransferredState(ScriptState* script_state,
                                                const TransferredValues& data);

  MediaStreamTrack();

  // MediaStreamTrack.idl
  virtual String kind() const = 0;
  virtual String id() const = 0;
  virtual String label() const = 0;
  virtual bool enabled() const = 0;
  virtual void setEnabled(bool) = 0;
  virtual bool muted() const = 0;
  virtual String ContentHint() const = 0;
  virtual V8MediaStreamTrackState readyState() const = 0;
  virtual void SetContentHint(const String&) = 0;
  virtual void stopTrack(ExecutionContext*) = 0;
  virtual MediaStreamTrack* clone(ExecutionContext*) = 0;
  virtual MediaTrackCapabilities* getCapabilities() const = 0;
  virtual MediaTrackConstraints* getConstraints() const = 0;
  virtual MediaTrackSettings* getSettings() const = 0;
  virtual V8UnionMediaStreamTrackAudioStatsOrMediaStreamTrackVideoStats*
  stats() = 0;
  virtual CaptureHandle* getCaptureHandle() const = 0;
  virtual ScriptPromise<IDLUndefined> applyConstraints(
      ScriptState*,
      const MediaTrackConstraints*) = 0;

  virtual void applyConstraints(ScriptPromiseResolver<IDLUndefined>*,
                                const MediaTrackConstraints*) = 0;
  virtual void SetInitialConstraints(const MediaConstraints& constraints) = 0;
  virtual void SetConstraints(const MediaConstraints& constraints) = 0;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(mute, kMute)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(unmute, kUnmute)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(ended, kEnded)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(capturehandlechange, kCapturehandlechange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(configurationchange, kConfigurationchange)

  virtual MediaStreamSource::ReadyState GetReadyState() = 0;

  virtual MediaStreamComponent* Component() const = 0;
  virtual bool Ended() const = 0;

  virtual void RegisterMediaStream(MediaStream*) = 0;
  virtual void UnregisterMediaStream(MediaStream*) = 0;
  virtual void RegisterSink(SpeechRecognitionMediaStreamAudioSink* sink) = 0;

  // EventTarget
  const AtomicString& InterfaceName() const override = 0;
  ExecutionContext* GetExecutionContext() const override = 0;
  void AddedEventListener(const AtomicString&,
                          RegisteredEventListener&) override = 0;

  // ScriptWrappable
  bool HasPendingActivity() const override = 0;

  virtual std::unique_ptr<AudioSourceProvider> CreateWebAudioSource(
      int context_sample_rate,
      base::TimeDelta platform_buffer_duration) = 0;

  virtual ImageCapture* GetImageCapture() = 0;
  virtual std::optional<const MediaStreamDevice> device() const = 0;
  // This function is called on the track by the serializer once it has been
  // serialized for transfer to another context.
  // Prepares the track for a potentially cross-renderer transfer. After this
  // is called, the track will be in an ended state and no longer usable.
  virtual void BeingTransferred(const base::UnguessableToken& transfer_id) = 0;
  // Returns true if this track is allowed to be transferred. If a transfer is
  //  not allowed, message will contain an explanatory text that can be
  //  surfaced to the caller.
  virtual bool TransferAllowed(String& message) const = 0;

  virtual void AddObserver(Observer*) = 0;

  void Trace(Visitor* visitor) const override { EventTarget::Trace(visitor); }

 private:
  // Friend in order to allow setting a new impl for FromTransferredState.
  friend void SetFromTransferredStateImplForTesting(
      FromTransferredStateImplForTesting impl);
  // Provides access to the global mock impl of FromTransferredState. Set to
  // base::NullCallback() to restore the real impl.
  static FromTransferredStateImplForTesting&
  GetFromTransferredStateImplForTesting();
};

typedef HeapVector<Member<MediaStreamTrack>> MediaStreamTrackVector;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASTREAM_MEDIA_STREAM_TRACK_H_
