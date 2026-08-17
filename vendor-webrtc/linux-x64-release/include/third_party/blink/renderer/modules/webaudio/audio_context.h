// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_CONTEXT_H_

#include <atomic>

#include "base/gtest_prod_util.h"
#include "base/time/time.h"
#include "media/mojo/mojom/media_player.mojom-blink.h"
#include "third_party/blink/public/mojom/mediastream/media_devices.mojom-blink.h"
#include "third_party/blink/public/mojom/permissions/permission.mojom-blink.h"
#include "third_party/blink/public/mojom/webaudio/audio_context_manager.mojom-blink.h"
#include "third_party/blink/public/platform/web_audio_sink_descriptor.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_audio_context_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_union_audiosinkinfo_string.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_union_audiosinkoptions_string.h"
#include "third_party/blink/renderer/core/frame/frame_visibility_observer.h"
#include "third_party/blink/renderer/core/html/media/autoplay_policy.h"
#include "third_party/blink/renderer/modules/webaudio/base_audio_context.h"
#include "third_party/blink/renderer/modules/webaudio/setsinkid_resolver.h"
#include "third_party/blink/renderer/platform/audio/audio_frame_stats_accumulator.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/self_keep_alive.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_receiver.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class AudioContextOptions;
class AudioTimestamp;
class AudioPlayoutStats;
class ExceptionState;
class ExecutionContext;
class HTMLMediaElement;
class LocalDOMWindow;
class MediaElementAudioSourceNode;
class MediaStream;
class MediaStreamAudioDestinationNode;
class MediaStreamAudioSourceNode;
class RealtimeAudioDestinationNode;
class ScriptState;
class WebAudioLatencyHint;

// This is an BaseAudioContext which actually plays sound, unlike an
// OfflineAudioContext which renders sound into a buffer.
class MODULES_EXPORT AudioContext final
    : public BaseAudioContext,
      public mojom::blink::PermissionObserver,
      public mojom::blink::MediaDevicesListener,
      public FrameVisibilityObserver,
      public media::mojom::blink::MediaPlayer {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AudioContext* Create(ExecutionContext*,
                              const AudioContextOptions*,
                              ExceptionState&);

  AudioContext(LocalDOMWindow&,
               const WebAudioLatencyHint&,
               std::optional<float> sample_rate,
               WebAudioSinkDescriptor sink_descriptor,
               bool update_echo_cancellation_on_first_start);
  AudioContext(const AudioContext&) = delete;
  AudioContext& operator=(const AudioContext&) = delete;
  ~AudioContext() override;

  void Trace(Visitor*) const override;

  // For ContextLifeCycleObserver
  void ContextDestroyed() override;
  bool HasPendingActivity() const override;

  bool IsContextCleared() const override;

  bool HasRealtimeConstraint() override { return true; }

  bool IsPullingAudioGraph() const override;

  // Called by handlers of AudioScheduledSourceNode and AudioBufferSourceNode to
  // notify their associated AudioContext when start() is called. It may resume
  // the AudioContext if it is now allowed to start.
  void NotifySourceNodeStart() override;

  bool HandlePreRenderTasks(uint32_t frames_to_process,
                            const AudioIOPosition* output_position,
                            const AudioCallbackMetric* metric,
                            base::TimeDelta playout_delay,
                            const media::AudioGlitchInfo& glitch_info) override;

  // Called at the end of each render quantum.
  void HandlePostRenderTasks() override;

  // mojom::blink::PermissionObserver
  void OnPermissionStatusChange(mojom::blink::PermissionStatus) override;

  // mojom::blink::MediaDevicesListener
  void OnDevicesChanged(mojom::blink::MediaDeviceType,
                        const Vector<WebMediaDeviceInfo>&) override;

  // FrameVisibilityObserver
  void FrameVisibilityChanged(
      mojom::blink::FrameVisibility frame_visibility) override;

  // media::mojom::MediaPlayer  implementation.
  void RequestPlay() override {}
  void RequestPause(bool triggered_by_user) override {}
  void RequestSeekForward(base::TimeDelta seek_time) override {}
  void RequestSeekBackward(base::TimeDelta seek_time) override {}
  void RequestSeekTo(base::TimeDelta seek_time) override {}
  void RequestEnterPictureInPicture() override {}
  void RequestMute(bool mute) override {}
  void SetVolumeMultiplier(double multiplier) override;
  void SetPersistentState(bool persistent) override {}
  void SetPowerExperimentState(bool enabled) override {}
  void SetAudioSinkId(const String&) override {}
  void SuspendForFrameClosed() override {}
  void RequestMediaRemoting() override {}
  void RequestVisibility(
      RequestVisibilityCallback request_visibility_cb) override {}
  void RecordAutoPictureInPictureInfo(
      const media::PictureInPictureEventsInfo::AutoPipInfo&
          auto_picture_in_picture_info) override {}

  // https://webaudio.github.io/web-audio-api/#AudioContext
  double baseLatency() const;
  double outputLatency() const;
  V8UnionAudioSinkInfoOrString* sinkId() const { return v8_sink_id_.Get(); }
  DEFINE_ATTRIBUTE_EVENT_LISTENER(sinkchange, kSinkchange)
  DEFINE_ATTRIBUTE_EVENT_LISTENER(error, kError)
  AudioTimestamp* getOutputTimestamp(ScriptState*) const;
  ScriptPromise<IDLUndefined> resumeContext(ScriptState*, ExceptionState&);
  ScriptPromise<IDLUndefined> suspendContext(ScriptState*, ExceptionState&);
  ScriptPromise<IDLUndefined> closeContext(ScriptState*, ExceptionState&);
  ScriptPromise<IDLUndefined> setSinkId(ScriptState*,
                                        const V8UnionAudioSinkOptionsOrString*,
                                        ExceptionState&);
  MediaElementAudioSourceNode* createMediaElementSource(HTMLMediaElement*,
                                                        ExceptionState&);
  MediaStreamAudioSourceNode* createMediaStreamSource(MediaStream*,
                                                      ExceptionState&);
  MediaStreamAudioDestinationNode* createMediaStreamDestination(
      ExceptionState&);

  // https://wicg.github.io/web_audio_playout
  AudioPlayoutStats* playoutStats();

  // Cannot be called from the audio thread.
  RealtimeAudioDestinationNode* GetRealtimeAudioDestinationNode() const;

  void HandleAudibility(AudioBus* destination_bus);

  // Adjusts the output volume of the rendered audio in case we are being
  // ducked.
  void HandleVolumeMultiplier(AudioBus* destination_bus);

  AudioCallbackMetric GetCallbackMetric() const;

  // Returns the audio buffer duration of the output driving playout of
  // AudioDestination.
  base::TimeDelta PlatformBufferDuration() const;

  WebAudioSinkDescriptor GetSinkDescriptor() const { return sink_descriptor_; }

  void NotifySetSinkIdBegins();
  void NotifySetSinkIdIsDone(WebAudioSinkDescriptor);

  HeapDeque<Member<SetSinkIdResolver>>& GetSetSinkIdResolver() {
    return set_sink_id_resolvers_;
  }

  // A helper function to validate the given sink descriptor. See:
  // webaudio.github.io/web-audio-api/#validating-sink-identifier
  bool IsValidSinkDescriptor(const WebAudioSinkDescriptor&);

  void OnRenderError();

  // A helper function for AudioPlayoutStats. Passes `audio_frame_stats_` to be
  // absorbed by `receiver`. See:
  // https://wicg.github.io/web_audio_playout
  void TransferAudioFrameStatsTo(AudioFrameStatsAccumulator& receiver);

  // Get the number of pending device list updates, to allow waiting until the
  // device list is refrehsed before using it.  A value of 0 means no updates
  // are pending.
  int PendingDeviceListUpdates();

  void StartContextInterruption();
  void EndContextInterruption();

  // Methods for unit tests
  void set_was_audible_for_testing(bool value) { was_audible_ = value; }
  void invoke_onrendererror_from_platform_for_testing();

 private:
  friend class AudioContextAutoplayTest;
  friend class AudioContextTest;
  FRIEND_TEST_ALL_PREFIXES(AudioContextTest, MediaDevicesService);
  FRIEND_TEST_ALL_PREFIXES(AudioContextTest,
                           OnRenderErrorFromPlatformDestination);

  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum class AutoplayStatus {
    // The AudioContext failed to activate because of user gesture requirements.
    kFailed = 0,
    // Same as AutoplayStatusFailed but start() on a node was called with a user
    // gesture.
    // This value is no longer used but the enum entry should not be re-used
    // because it is used for metrics.
    // kAutoplayStatusFailedWithStart = 1,
    // The AudioContext had user gesture requirements and was able to activate
    // with a user gesture.
    kSucceeded = 2,

    kMaxValue = kSucceeded,
  };

  // Do not change the order of this enum, it is used for metrics.
  enum class AutoplayUnlockType {
    kContextConstructor = 0,
    kContextResume = 1,
    kSourceNodeStart = 2,
    kMaxValue = kSourceNodeStart,
  };

  void Uninitialize() override;

  // Returns the AutoplayPolicy currently applying to this instance.
  AutoplayPolicy::Type GetAutoplayPolicy() const;

  // Returns whether the autoplay requirements are fulfilled.
  bool AreAutoplayRequirementsFulfilled() const;

  // If possible, allows autoplay for the AudioContext and mark it as allowed by
  // the given type.
  void MaybeAllowAutoplayWithUnlockType(AutoplayUnlockType);

  // Returns whether the AudioContext is allowed to start rendering. It takes in
  // a boolean parameter to indicate whether it should suppress warnings or send
  // warning messages to the console about the requirement of user gesture.
  bool IsAllowedToStart(bool should_suppress_warning = false) const;

  // Record the current autoplay metrics.
  void RecordAutoplayMetrics();

  // Starts rendering via AudioDestinationNode. This sets the self-referencing
  // pointer to this object.
  void StartRendering() override;

  // Called when the context is being closed to stop rendering audio and clean
  // up handlers. This clears the self-referencing pointer, making this object
  // available for the potential GC.
  void StopRendering() VALID_CONTEXT_REQUIRED(main_thread_sequence_checker_);

  // Called when suspending the context to stop rendering audio, but don't clean
  // up handlers because we expect to be resuming where we left off.
  void SuspendRendering() VALID_CONTEXT_REQUIRED(main_thread_sequence_checker_);

  void DidClose();

  // Called by the audio thread to handle Promises for resume() and suspend(),
  // posting a main thread task to perform the actual resolving, if needed.
  void ResolvePromisesForUnpause();

  AudioIOPosition OutputPosition() const
      VALID_CONTEXT_REQUIRED(main_thread_sequence_checker_);

  // Send notification to browser that an AudioContext has started or stopped
  // playing audible audio.
  void NotifyAudibleAudioStarted()
      VALID_CONTEXT_REQUIRED(main_thread_sequence_checker_);
  void NotifyAudibleAudioStopped()
      VALID_CONTEXT_REQUIRED(main_thread_sequence_checker_);

  void EnsureAudioContextManagerService();
  void OnAudioContextManagerServiceConnectionError();

  void DidInitialPermissionCheck(mojom::blink::PermissionDescriptorPtr,
                                 mojom::blink::PermissionStatus);
  double GetOutputLatencyQuantizingFactor() const;

  void InitializeMediaDeviceService();
  void UninitializeMediaDeviceService();

  // Callback from blink::mojom::MediaDevicesDispatcherHost::EnumerateDevices().
  void DevicesEnumerated(const Vector<Vector<WebMediaDeviceInfo>>& enumeration,
                         Vector<mojom::blink::VideoInputDeviceCapabilitiesPtr>
                             video_input_capabilities,
                         Vector<mojom::blink::AudioInputDeviceCapabilitiesPtr>
                             audio_input_capabilities)
      VALID_CONTEXT_REQUIRED(main_thread_sequence_checker_);

  // A helper function used to update `v8_sink_id_` whenever `sink_id_` is
  // updated.
  void UpdateV8SinkId();

  // Called on prerendering activation time if this AudioContext is blocked by
  // prerendering.
  void ResumeOnPrerenderActivation();

  void HandleRenderError()
      VALID_CONTEXT_REQUIRED(main_thread_sequence_checker_);

  // https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/media/capture/README.md#logs
  void SendLogMessage(const char* const function_name, const String& message);

  LocalFrame* GetLocalFrame() const;

  // Connects to the MediaPlayerHost to register as a media player.
  void EnsureMediaPlayerConnection();

  // Handles a disconnection from the MediaPlayerHost.
  void OnMediaPlayerDisconnect();

  // Returns whether the media-playback-while-not-visible permission policy
  // allows this audio context to play while not visible.
  bool CanPlayWhileHidden() const;

  // https://webaudio.github.io/web-audio-api/#dom-audiocontext-suspended-by-user-slot
  bool suspended_by_user_ = false;

  unsigned context_id_;
  Member<ScriptPromiseResolver<IDLUndefined>> close_resolver_;

  AudioIOPosition output_position_;
  AudioCallbackMetric callback_metric_;

  // Accessed only on the thread pulling audio from the graph.
  AudioFrameStatsAccumulator pending_audio_frame_stats_;

  // Protected by the graph lock.
  AudioFrameStatsAccumulator audio_frame_stats_;

  Member<AudioPlayoutStats> audio_playout_stats_;

  // Whether a user gesture is required to start this AudioContext.
  bool user_gesture_required_ = false;

  // Whether this AudioContext is blocked to start because the page is still in
  // prerendering state.
  bool blocked_by_prerendering_ = false;

  // Autoplay status associated with this AudioContext, if any.
  // Will only be set if there is an autoplay policy in place.
  // Will never be set for OfflineAudioContext.
  std::optional<AutoplayStatus> autoplay_status_;

  // Autoplay unlock type for this AudioContext.
  // Will only be set if there is an autoplay policy in place.
  // Will never be set for OfflineAudioContext.
  std::optional<AutoplayUnlockType> autoplay_unlock_type_;

  // Records if start() was ever called for any source node in this context.
  bool source_node_started_ = false;

  // baseLatency for this context
  double base_latency_ = 0;

  // AudioContextManager for reporting audibility.
  HeapMojoRemote<mojom::blink::AudioContextManager> audio_context_manager_;

  // Keeps track if the output of this destination was audible, before the
  // current rendering quantum.  Used for recording "playback" time.
  bool was_audible_ = false;

  // Counts the number of render quanta where audible sound was played.  We
  // determine audibility on render quantum boundaries, so counting quanta is
  // all that's needed.
  size_t total_audible_renders_ = 0;

  SelfKeepAlive<AudioContext> keep_alive_{this};

  // Initially, we assume that the microphone permission is denied. But this
  // will be corrected after the actual construction.
  mojom::blink::PermissionStatus microphone_permission_status_ =
      mojom::blink::PermissionStatus::DENIED;

  HeapMojoRemote<mojom::blink::PermissionService> permission_service_;
  HeapMojoReceiver<mojom::blink::PermissionObserver, AudioContext>
      permission_receiver_;

  // Describes the current audio output device.
  WebAudioSinkDescriptor sink_descriptor_;

  // A V8 return value from `AudioContext.sinkId` getter. It gets updated when
  // `sink_descriptor_` above is updated.
  Member<V8UnionAudioSinkInfoOrString> v8_sink_id_;

  // A queue for setSinkId() Promise resolvers. Requests are handled in the
  // order it was received and only one request is handled at a time.
  HeapDeque<Member<SetSinkIdResolver>> set_sink_id_resolvers_;

  // MediaDeviceService for querying device information, and the associated
  // receiver for getting notification.
  HeapMojoRemote<mojom::blink::MediaDevicesDispatcherHost>
      media_device_service_;
  HeapMojoReceiver<mojom::blink::MediaDevicesListener, AudioContext>
      media_device_service_receiver_;

  bool is_media_device_service_initialized_ = false;

  // Stores a list of identifiers for output device.
  HashSet<String> output_device_ids_;

  // `wasRunning` flag for `setSinkId()` state transition. See the
  // implementation of `NotifySetSinkIdBegins()` for details.
  bool sink_transition_flag_was_running_ = false;

  // To keep the record of any render errors reported from the infra during
  // the life cycle of the context.
  bool render_error_occurred_ = false;

  // If a sink ID is given via the constructor or `setSinkId()` method,
  // this is set to `true`.
  bool is_sink_id_given_ = false;

  // The suspended->interrupted transition should not happen immediately when
  // an interruption occurs.  If an interruption happens in
  // the suspended state, we store this state in the
  // `is_interrupted_while_suspended_` flag.  Then, if resume() is called while
  // the context is suspended and the flag is set, we transition to the
  // interrupted state.  This variable should only be modified by
  // StartContextInterruption() and EndContextInterruption().
  bool is_interrupted_while_suspended_ = false;

  // True if the context should transition to running after an interruption
  // ends.
  bool should_transition_to_running_after_interruption_ = false;

  // True if the context should be interrupted when the frame is hidden.
  const bool should_interrupt_when_frame_is_hidden_;

  // True if the host frame's:
  // - 'display' property is set to 'none';
  // - 'visibility' property is set to 'hidden';
  bool is_frame_hidden_ = false;

  // The number of pending device list updates, to allow waiting until the
  // device list is refrehsed before using it.  A value of 0 means no updates
  // are pending.
  int pending_device_list_updates_
      GUARDED_BY_CONTEXT(main_thread_sequence_checker_) = 0;

  // ID used for mojo communication with the MediaPlayerHost.
  const int player_id_;

  // Volume multiplier applied to audio output. Used to duck audio when the
  // MediaPlayerHost requests ducking. Only written on the main thread and only
  // read on the audio thread.
  std::atomic<double> volume_multiplier_ = 1.0;

  HeapMojoAssociatedRemote<media::mojom::blink::MediaPlayerHost>
      media_player_host_;
  HeapMojoAssociatedReceiver<media::mojom::blink::MediaPlayer, AudioContext>
      media_player_receiver_;
  HeapMojoAssociatedRemote<media::mojom::blink::MediaPlayerObserver>
      media_player_observer_;

  SEQUENCE_CHECKER(main_thread_sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_CONTEXT_H_
