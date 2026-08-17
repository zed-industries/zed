/*
 * Copyright (C) 2007, 2008, 2009, 2010, 2011, 2012, 2013 Apple Inc. All rights
 * reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_HTML_MEDIA_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_HTML_MEDIA_ELEMENT_H_

#include <memory>
#include <optional>
#include <variant>

#include "base/synchronization/lock.h"
#include "base/thread_annotations.h"
#include "base/time/time.h"
#include "base/timer/elapsed_timer.h"
#include "media/mojo/mojom/media_player.mojom-blink.h"
#include "media/renderers/remote_playback_client_wrapper.h"
#include "third_party/blink/public/platform/web_audio_source_provider_impl.h"
#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_state_observer.h"
#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/core/html/media/media_controls.h"
#include "third_party/blink/renderer/core/html_names.h"
#include "third_party/blink/renderer/core/intersection_observer/intersection_observer.h"
#include "third_party/blink/renderer/core/speech/speech_synthesis_base.h"
#include "third_party/blink/renderer/platform/audio/audio_source_provider.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/heap/disallow_new_wrapper.h"
#include "third_party/blink/renderer/platform/heap/prefinalizer.h"
#include "third_party/blink/renderer/platform/media/media_player_client.h"
#include "third_party/blink/renderer/platform/media/web_audio_source_provider_client.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver_set.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote_set.h"
#include "third_party/blink/renderer/platform/network/mime/mime_type_registry.h"
#include "third_party/blink/renderer/platform/scheduler/public/post_cancellable_task.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc_overrides/low_precision_timer.h"

namespace cc {
class Layer;
}

namespace media {
enum class MediaContentType;
}  // namespace media

namespace blink {

class AudioSourceProviderClient;
class AudioTrack;
class AudioTrackList;
class AutoplayPolicy;
class ContentType;
class CueTimeline;
class Event;
class EventQueue;
class ExceptionState;
class HTMLMediaElementControlsList;
class HTMLSourceElement;
class HTMLTrackElement;
class MediaError;
class MediaSourceAttachment;
class MediaSourceHandle;
class MediaSourceTracer;
class MediaStreamDescriptor;
class RemotePlaybackClient;
class ScriptPromiseResolverBase;
class ScriptState;
class TextTrack;
class TextTrackContainer;
class TextTrackList;
class TimeRanges;
class VideoTrack;
class VideoTrackList;
class V8CanPlayTypeResult;
class V8TextTrackKind;

class CORE_EXPORT HTMLMediaElement
    : public HTMLElement,
      public Supplementable<HTMLMediaElement>,
      public ActiveScriptWrappable<HTMLMediaElement>,
      public ExecutionContextLifecycleStateObserver,
      public media::mojom::blink::MediaPlayer,
      public media::RemotePlaybackClientWrapper,
      private MediaPlayerClient {
  DEFINE_WRAPPERTYPEINFO();
  USING_PRE_FINALIZER(HTMLMediaElement, Dispose);

 public:
  // Limits the range of media playback rate.
  static constexpr double kMinPlaybackRate = 0.0625;
  static constexpr double kMaxPlaybackRate = 16.0;

  enum class PlayPromiseError {
    kNotSupported,
    kPaused_PauseCalled,
    kPaused_EndOfPlayback,
    kPaused_RemovedFromDocument,
    kPaused_AutoplayAutoPause,
    kPaused_PageHidden,
    kPaused_SuspendedPlayerIdleTimeout,
    kPaused_RemotePlayStateChange,
    kPaused_PauseRequestedByUser,
    kPaused_PauseRequestedInternally,
    kPaused_FrameHidden,
    kPaused_LetAudioDescriptionFinish
  };

  bool IsMediaElement() const override { return true; }

  static MIMETypeRegistry::SupportsType GetSupportsType(const ContentType&);

  static bool IsHLSURL(const KURL&);

  // Notify the HTMLMediaElement that the media controls settings have changed
  // for the given document.
  static void OnMediaControlsEnabledChange(Document*);

  // Binds |pending_receiver| and adds it to |media_player_receiver_set_|. Also
  // called from other Blink classes (e.g. PictureInPictureControllerImpl).
  void BindMediaPlayerReceiver(
      mojo::PendingAssociatedReceiver<media::mojom::blink::MediaPlayer>
          pending_receiver);

  void Trace(Visitor*) const override;

  WebMediaPlayer* GetWebMediaPlayer() const { return web_media_player_.get(); }

  // Returns true if the loaded media has a video track.
  // Note that even an audio element can have video track in cases such as
  // <audio src="video.webm">, in which case this function will return true.
  bool HasVideo() const;

  // Returns true if loaded media has an audio track.
  bool HasAudio() const;

  // Whether the media element has encrypted audio or video streams.
  bool IsEncrypted() const;

  bool SupportsSave() const;
  bool SupportsLoop() const;

  cc::Layer* CcLayer() const;

  enum DelayedActionType {
    kLoadMediaResource = 1 << 0,
    kLoadTextTrackResource = 1 << 1
  };
  void ScheduleTextTrackResourceLoad();

  // error state
  MediaError* error() const;

  // network state
  void SetSrc(const AtomicString&);
  const KURL& currentSrc() const { return current_src_.GetSourceIfVisible(); }

  // Return the URL to be used for downloading the media.
  const KURL& downloadURL() const {
    // If we didn't get a redirected URL from the player, then use the original.
    if (current_src_after_redirects_.IsNull() ||
        current_src_after_redirects_.IsEmpty()) {
      return currentSrc();
    }
    return current_src_after_redirects_;
  }

  using SrcObjectVariant =
      std::variant<MediaStreamDescriptor*, MediaSourceHandle*>;
  void SetSrcObjectVariant(SrcObjectVariant src_object_variant);
  SrcObjectVariant GetSrcObjectVariant() const;

  enum NetworkState {
    kNetworkEmpty,
    kNetworkIdle,
    kNetworkLoading,
    kNetworkNoSource
  };
  NetworkState getNetworkState() const;

  String preload() const;
  void setPreload(const AtomicString&);
  WebMediaPlayer::Preload PreloadType() const;
  String EffectivePreload() const;
  WebMediaPlayer::Preload EffectivePreloadType() const;

  WebTimeRanges BufferedInternal() const;
  TimeRanges* buffered() const;
  void load();
  V8CanPlayTypeResult canPlayType(const String& mime_type) const;

  // ready state
  enum ReadyState {
    kHaveNothing,
    kHaveMetadata,
    kHaveCurrentData,
    kHaveFutureData,
    kHaveEnoughData
  };
  ReadyState getReadyState() const;
  bool seeking() const;

  // playback state
  double currentTime() const;
  void setCurrentTime(double);
  double duration() const;
  bool paused() const;
  double defaultPlaybackRate() const;
  void setDefaultPlaybackRate(double);
  double playbackRate() const;
  void setPlaybackRate(double, ExceptionState& = ASSERT_NO_EXCEPTION);
  TimeRanges* played();
  WebTimeRanges SeekableInternal() const;
  TimeRanges* seekable() const;
  bool ended() const;
  bool Autoplay() const;
  bool Loop() const;
  void SetLoop(bool);
  ScriptPromise<IDLUndefined> playForBindings(ScriptState*);
  std::optional<DOMExceptionCode> Play();

  // Called when the video should pause to let audio descriptions finish.
  void PauseToLetDescriptionFinish();

  void pause();
  double latencyHint() const;
  void setLatencyHint(double);
  bool preservesPitch() const;
  void setPreservesPitch(bool);
  void FlingingStarted();
  void FlingingStopped();

  // statistics
  uint64_t webkitAudioDecodedByteCount() const;
  uint64_t webkitVideoDecodedByteCount() const;

  // media source extensions
  void CloseMediaSource();
  void DurationChanged(double duration, bool request_seek);

  // controls
  bool ShouldShowControls() const;
  bool ShouldShowAllControls() const;
  DOMTokenList* controlsList() const;
  HTMLMediaElementControlsList* ControlsListInternal() const;
  double volume() const;
  void setVolume(double, ExceptionState& = ASSERT_NO_EXCEPTION);
  bool muted() const;
  void setMuted(bool);
  virtual bool SupportsPictureInPicture() const { return false; }
  void SetUserWantsControlsVisible(bool visible);
  bool UserWantsControlsVisible() const;

  void TogglePlayState();

  AudioTrackList& audioTracks();
  void AudioTrackChanged(AudioTrack*);

  VideoTrackList& videoTracks();
  void SelectedVideoTrackChanged(VideoTrack*);

  TextTrack* addTextTrack(const V8TextTrackKind& kind,
                          const AtomicString& label,
                          const AtomicString& language,
                          ExceptionState&);

  TextTrackList* textTracks();
  CueTimeline& GetCueTimeline();

  // Implements the "forget the media element's media-resource-specific tracks"
  // algorithm in the HTML5 spec.
  void ForgetResourceSpecificTracks();

  void DidAddTrackElement(HTMLTrackElement*);
  void DidRemoveTrackElement(HTMLTrackElement*);

  void HonorUserPreferencesForAutomaticTextTrackSelection();

  bool TextTracksAreReady() const;
  void ConfigureTextTrackDisplay();
  void UpdateTextTrackDisplay();

  // Get a SpeechSynthesis interface to use for generating speech for audio
  // descriptions.
  SpeechSynthesisBase* SpeechSynthesis();
  double LastSeekTime() const { return last_seek_time_; }
  void TextTrackReadyStateChanged(TextTrack*);

  void TextTrackModeChanged(TextTrack*);
  void DisableAutomaticTextTrackSelection();

  // EventTarget function.
  // Both Node (via HTMLElement) and ExecutionContextLifecycleStateObserver
  // define this method, which causes an ambiguity error at compile time. This
  // class's constructor ensures that both implementations return document, so
  // return the result of one of them here.
  using HTMLElement::GetExecutionContext;

  bool IsFullscreen() const;
  virtual bool UsesOverlayFullscreenVideo() const { return false; }

  bool HasClosedCaptions() const;
  bool TextTracksVisible() const;

  static void SetTextTrackKindUserPreferenceForAllMediaElements(Document*);
  void AutomaticTrackSelectionForUpdatedUserPreference();

  // Returns the MediaControls, or null if they have not been added yet.
  // Note that this can be non-null even if there is no controls attribute.
  MediaControls* GetMediaControls() const;

  // Notifies the media element that the media controls became visible, so
  // that text track layout may be updated to avoid overlapping them.
  void MediaControlsDidBecomeVisible();

  void SourceWasRemoved(HTMLSourceElement*);
  void SourceWasAdded(HTMLSourceElement*);

  // ScriptWrappable functions.
  bool HasPendingActivity() const override;

  AudioSourceProviderClient* AudioSourceNode() {
    return audio_source_node_.Get();
  }
  void SetAudioSourceNode(AudioSourceProviderClient*);

  AudioSourceProvider& GetAudioSourceProvider() {
    return audio_source_provider_;
  }

  enum InvalidURLAction { kDoNothing, kComplain };
  bool IsSafeToLoadURL(const KURL&, InvalidURLAction);

  // Checks to see if current media data is CORS-same-origin.
  bool IsMediaDataCorsSameOrigin() const;

  void ScheduleEvent(Event*);

  // Returns the "effective media volume" value as specified in the HTML5 spec.
  double EffectiveMediaVolume() const;

  // Predicates also used when dispatching wrapper creation (cf.
  // [SpecialWrapFor] IDL attribute usage.)
  virtual bool IsHTMLAudioElement() const { return false; }
  virtual bool IsHTMLVideoElement() const { return false; }

  void VideoWillBeDrawnToCanvas() const;

  const AutoplayPolicy& GetAutoplayPolicy() const { return *autoplay_policy_; }

  WebMediaPlayer::LoadType GetLoadType() const;

  bool HasMediaSource() const { return media_source_attachment_.get(); }

  void DidAudioOutputSinkChanged(const String& hashed_device_id);

  void SetCcLayerForTesting(cc::Layer* layer) { SetCcLayer(layer); }
  void AddMediaTrackForTesting(const media::MediaTrack& t) { AddMediaTrack(t); }

  // This should be called directly after creation.
  void SetMediaPlayerHostForTesting(
      mojo::PendingAssociatedRemote<media::mojom::blink::MediaPlayerHost> host);

  bool IsShowPosterFlagSet() const { return show_poster_flag_; }

  // What LocalFrame should own our player?  Normally, players are tied to their
  // HTMLMediaElement's LocalFrame for metrics, network fetch, etc.  This has
  // the side-effect of requiring that a player is destroyed when the element's
  // frame changes.  That causes playback to be user-visibly interrupted,
  // potentially for multiple seconds.
  //
  // In some very restricted cases, related to picture-in-picture playback, it
  // is okay to keep the player even when the element is moved to a new
  // document.  It requires that everything is same-origin, and that lifetimes
  // are looked after carefully so that the player does not outlive the frame
  // that owns it.  However, it permits seamless playback when transitioning to
  // picture in picture.  In this case, this function will return a different
  // frame than our own.
  //
  // Note that new players can be created in this frame as well, so that a
  // transfer back to the original opener frame when picture in picture is
  // closed can be seamless too, even if the player was recreated for some
  // reason while in picture in picture mode.
  LocalFrame* LocalFrameForPlayer();

  bool IsValidBuiltinCommand(HTMLElement& invoker,
                             CommandEventType command) override;
  bool HandleCommandInternal(HTMLElement& invoker,
                             CommandEventType command) override;

  // media::RemotePlaybackClientWrapper overrides:
  std::string GetActivePresentationId() override;

  // Returns the execution context for player creation. This will be the
  // execution context of the opener document if available, otherwise the
  // execution context of the current document.
  //
  // This method should be used when it is necessary to continue using the
  // execution context of the opener document, when a media element is moved
  // into a new document (e.g. picture in picture window).
  //
  // This is currently only used by the `ModulesInitializer` to properly set the
  // media player inspector context, so that media DevTool logs are routed using
  // the correct execution context.
  ExecutionContext* GetExecutionContextForPlayer() const;

  // WebAudio audio destination node is connected. The HTMLMediaElement could
  // stop the sink at this time if it is still playing.
  void ConnectToDestinationReady();

 protected:
  // Assert the correct order of the children in shadow dom when DCHECK is on.
  static void AssertShadowRootChildren(ShadowRoot&);

  HTMLMediaElement(const QualifiedName&, Document&);
  ~HTMLMediaElement() override;
  void Dispose();

  // Returns a constant reference to the HeapMojoAssociatedRemoteSet holding all
  // the bound remotes for the media::mojom::blink::MediaPlayerObserver
  // interface. Needed to allow sending messages directly from
  // HTMLMediaElement's subclasses.
  const HeapMojoAssociatedRemoteSet<media::mojom::blink::MediaPlayerObserver>&
  GetMediaPlayerObserverRemoteSet() {
    return media_player_observer_remote_set_->Value();
  }

  void ParseAttribute(const AttributeModificationParams&) override;
  void FinishParsingChildren() final;
  bool IsURLAttribute(const Attribute&) const override;
  void AttachLayoutTree(AttachContext&) override;
  void CloneNonAttributePropertiesFrom(const Element&,
                                       NodeCloningData&) override;

  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;

  // Return true if media is cross origin from the current document
  // and has not passed a cors check, meaning that we should return
  // as little information as possible about it.
  bool MediaShouldBeOpaque() const;

  void DidMoveToNewDocument(Document& old_document) override;
  virtual KURL PosterImageURL() const { return KURL(); }

  // Called after the creation of |web_media_player_|.
  virtual void OnWebMediaPlayerCreated() {}
  virtual void OnWebMediaPlayerCleared() {}

  void UpdateLayoutObject();

  virtual void RecordVideoOcclusionState(
      std::string_view occlusion_state) const {}

 private:
  // Friend class for testing.
  friend class ContextMenuControllerTest;
  friend class HTMLMediaElementTest;
  friend class PictureInPictureControllerTestWithWidget;
  friend class VideoWakeLockTest;

  class SourceMetadata {
    DISALLOW_NEW();

   public:
    enum class SourceVisibility { kVisibleToApp, kInvisibleToApp };
    SourceMetadata() = default;
    void SetSource(const KURL& src, SourceVisibility visibility) {
      src_ = src;
      invisible_to_app_ = visibility == SourceVisibility::kInvisibleToApp;
    }
    const KURL& GetSourceIfVisible() const {
      return invisible_to_app_ ? NullURL() : src_;
    }
    const KURL& GetSource() const { return src_; }

   private:
    KURL src_;

    // If true, then |current_src| is used only for internal loading and safety
    // checks, and for logging that is not visible to apps, either. For example,
    // when loading from a MediaSourceHandle as srcObject, this would be true.
    bool invisible_to_app_ = false;
  };

  bool HasPendingActivityInternal() const;

  void ResetMediaPlayerAndMediaSource();

  bool AlwaysCreateUserAgentShadowRoot() const final { return true; }
  bool AreAuthorShadowsAllowed() const final { return false; }

  FocusableState SupportsFocus(UpdateBehavior update_behavior) const final;
  FocusableState IsFocusableState(UpdateBehavior update_behavior) const final;
  int DefaultTabIndex() const final;
  bool LayoutObjectIsNeeded(const DisplayStyle&) const override;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  void DidNotifySubtreeInsertionsToDocument() override;
  void DidRecalcStyle(const StyleRecalcChange) final;

  bool CanStartSelection() const override { return false; }

  bool IsInteractiveContent() const final;

  // ExecutionContextLifecycleStateObserver functions.
  void ContextLifecycleStateChanged(mojom::FrameLifecycleState) override;
  void ContextDestroyed() override;

  virtual void OnPlay() {}
  virtual void OnLoadStarted() {}
  virtual void OnLoadFinished() {}

  // Updates the `MediaVideoVisibilityTracker` state whenever the media play
  // state is updated. This is typically handled during `UpdatePlayState`.
  virtual void UpdateVideoVisibilityTracker() {}

  // Handles playing of media element when audio descriptions are finished
  // speaking.
  void OnSpeakingCompleted();

  void SetShowPosterFlag(bool value);

  void SetReadyState(ReadyState);
  void SetNetworkState(WebMediaPlayer::NetworkState);

  // WebMediaPlayerClient implementation.
  void NetworkStateChanged() final;
  void ReadyStateChanged() final;
  void TimeChanged() final;
  void Repaint() final;
  void DurationChanged() final;
  void SizeChanged() final;
  void OnFirstFrame(base::TimeTicks frame_time,
                    size_t bytes_to_first_frame) override {}

  int GetElementId() override { return GetDomNodeId(); }

  void SetCcLayer(cc::Layer*) override;

  void AddMediaTrack(const media::MediaTrack&) final;
  void RemoveMediaTrack(const media::MediaTrack&) final;

  void MediaSourceOpened(std::unique_ptr<WebMediaSource>) final;
  void RemotePlaybackCompatibilityChanged(const KURL&,
                                          bool is_compatible) final;
  bool HasSelectedVideoTrack() final;
  WebMediaPlayer::TrackId GetSelectedVideoTrackId() final;
  bool WasAlwaysMuted() final;
  bool HasNativeControls() final;
  bool IsAudioElement() final;
  WebMediaPlayer::DisplayType GetDisplayType() const override;
  media::RemotePlaybackClientWrapper* RemotePlaybackClientWrapper() final {
    return this;
  }
  gfx::ColorSpace TargetColorSpace() override;
  bool WasAutoplayInitiated() override;
  bool IsInAutoPIP() const override { return false; }
  void ResumePlayback() final;
  void PausePlayback(WebMediaPlayer::PauseReason) final;
  void DidPlayerStartPlaying() override;
  void DidPlayerPaused(bool stream_ended) override;
  void DidPlayerMutedStatusChange(bool muted) override;
  void DidMediaMetadataChange(bool has_audio,
                              bool has_video,
                              media::AudioCodec audio_codec,
                              media::VideoCodec video_codec,
                              media::MediaContentType media_content_type,
                              bool is_encrypted_media) override;
  void DidPlayerMediaPositionStateChange(double playback_rate,
                                         base::TimeDelta duration,
                                         base::TimeDelta position,
                                         bool end_of_media) override;
  void DidDisableAudioOutputSinkChanges() override;
  void DidUseAudioServiceChange(bool uses_audio_service) override;
  void DidPlayerSizeChange(const gfx::Size& size) override;
  void OnRemotePlaybackDisabled(bool disabled) override;

  // Returns a reference to the mojo remote for the MediaPlayerHost interface,
  // requesting it first from the BrowserInterfaceBroker if needed. It is an
  // error to call this method before having access to the document's frame.
  media::mojom::blink::MediaPlayerHost& GetMediaPlayerHostRemote();

  // media::mojom::MediaPlayer  implementation.
  void RequestPlay() override;
  void RequestPause(bool triggered_by_user) override;
  void RequestSeekForward(base::TimeDelta seek_time) override;
  void RequestSeekBackward(base::TimeDelta seek_time) override;
  void RequestSeekTo(base::TimeDelta seek_time) override;
  void RequestEnterPictureInPicture() override {}
  void RequestMute(bool mute) override;
  void SetVolumeMultiplier(double multiplier) override;
  void SetPersistentState(bool persistent) override {}
  void SetPowerExperimentState(bool enabled) override;
  void SetAudioSinkId(const String&) override;
  void SuspendForFrameClosed() override;
  void RequestMediaRemoting() override {}
  void RequestVisibility(
      RequestVisibilityCallback request_visibility_cb) override {}
  void RecordAutoPictureInPictureInfo(
      const media::PictureInPictureEventsInfo::AutoPipInfo&
          auto_picture_in_picture_info) override;

  void LoadTimerFired(TimerBase*);
  void ProgressEventTimerFired();
  void PlaybackProgressTimerFired();
  void ScheduleTimeupdateEvent(bool periodic_event);
  void StartPlaybackProgressTimer();
  void StartProgressEventTimer();
  void StopPeriodicTimers();

  void Seek(double time);
  void FinishSeek();
  void AddPlayedRange(double start, double end);

  void ScheduleNamedEvent(const AtomicString& event_name);

  // loading
  void InvokeLoadAlgorithm();
  void InvokeResourceSelectionAlgorithm();
  void LoadInternal();
  void SelectMediaResource();
  void LoadResource(const WebMediaPlayerSource&, const String& content_type);
  void StartPlayerLoad();
  void SetPlayerPreload();
  void ScheduleNextSourceChild();
  void LoadSourceFromObject();
  void LoadSourceFromAttribute();
  void LoadNextSourceChild();
  void ClearMediaPlayer();
  void ClearMediaPlayerAndAudioSourceProviderClientWithoutLocking();
  bool HavePotentialSourceChild();
  void NoneSupported(const String&);
  void MediaEngineError(MediaError*);
  void CancelPendingEventsAndCallbacks();
  void WaitForSourceChange();
  void SetIgnorePreloadNone();

  KURL SelectNextSourceChild(String* content_type, InvalidURLAction);

  void MediaLoadingFailed(WebMediaPlayer::NetworkState, const String&);

  // deferred loading (preload=none)
  bool LoadIsDeferred() const;
  void DeferLoad();
  void CancelDeferredLoad();
  void StartDeferredLoad();
  void ExecuteDeferredLoad();
  void DeferredLoadTimerFired(TimerBase*);

  void MarkCaptionAndSubtitleTracksAsUnconfigured();

  // This does not check user gesture restrictions.
  void PlayInternal();

  // This does not stop autoplay visibility observation.
  // By default, will pause the video and speech.
  void PauseInternal(WebMediaPlayer::PauseReason pause_reason);

  // By default, will pause the video and speech.
  void UpdatePlayState(
      std::optional<WebMediaPlayer::PauseReason> pause_reason = std::nullopt);

  bool PotentiallyPlaying() const;
  bool StoppedDueToErrors() const;
  bool CouldPlayIfEnoughData() const override;

  // Generally the presence of the loop attribute should be considered to mean
  // playback has not "ended", as "ended" and "looping" are mutually exclusive.
  // See
  // https://html.spec.whatwg.org/C/#ended-playback
  enum class LoopCondition { kIncluded, kIgnored };
  bool EndedPlayback(LoopCondition = LoopCondition::kIncluded) const;

  void SetShouldDelayLoadEvent(bool);

  double EarliestPossiblePosition() const;
  double CurrentPlaybackPosition() const;
  double OfficialPlaybackPosition() const;
  void SetOfficialPlaybackPosition(double) const;
  void RequireOfficialPlaybackPositionUpdate() const;

  void EnsureMediaControls();
  void UpdateControlsVisibility();

  TextTrackContainer& EnsureTextTrackContainer();

  void ChangeNetworkStateFromLoadingToIdle();

  WebMediaPlayer::CorsMode CorsMode() const;

  // Returns the "direction of playback" value as specified in the HTML5 spec.
  enum DirectionOfPlayback { kBackward, kForward };
  DirectionOfPlayback GetDirectionOfPlayback() const;

  // Creates placeholder AudioTrack and/or VideoTrack objects when
  // WebMediaPlayer objects advertise they have audio and/or video, but don't
  // explicitly signal them via addAudioTrack() and addVideoTrack().
  // FIXME: Remove this once all WebMediaPlayer implementations properly report
  // their track info.
  void CreatePlaceholderTracksIfNecessary();

  void SetNetworkState(NetworkState, bool update_media_controls = true);

  void AudioTracksTimerFired(TimerBase*);

  void ScheduleResolvePlayPromises();
  void ScheduleRejectPlayPromises(PlayPromiseError);
  void ScheduleNotifyPlaying();
  void ResolveScheduledPlayPromises();
  void RejectScheduledPlayPromises();
  void RejectPlayPromises(DOMExceptionCode, const String&);
  void RejectPlayPromisesInternal(DOMExceptionCode, const String&);

  void OnRemovedFromDocumentTimerFired(TimerBase*);

  void SetError(MediaError* error);
  void ReportCurrentTimeToMediaSource();

  void ResetMojoState();
  void OnRemotePlaybackMetadataChange();

  // Determine if we should reuse the player when moving the element from
  // |old_document| to |new_document|
  bool ShouldReusePlayer(Document& old_document, Document& new_document) const;

  // Returns whether the media-playback-while-not-visible permission policy
  // allows this media element to play while not visible.
  bool CanPlayWhileHidden() const;

  // Returns true if the element is in a frame that is not being rendered.
  bool IsFrameHidden() const;

  // Adds a new MediaPlayerObserver remote that will be notified about media
  // player events and returns a receiver that an observer implementation can
  // bind to.
  mojo::PendingAssociatedReceiver<media::mojom::blink::MediaPlayerObserver>
  AddMediaPlayerObserverAndPassReceiver();

  // Timers used to schedule one-shot tasks with no delay.
  HeapTaskRunnerTimer<HTMLMediaElement> load_timer_;
  HeapTaskRunnerTimer<HTMLMediaElement> audio_tracks_timer_;
  HeapTaskRunnerTimer<HTMLMediaElement> removed_from_document_timer_;
  // Use a low precision timer for repeating tasks to avoid excessive Idle Wake
  // Up frequency, especially when WebRTC is used and the page contains many
  // HTMLMediaElements.
  LowPrecisionTimer progress_event_timer_;
  LowPrecisionTimer playback_progress_timer_;

  Member<TimeRanges> played_time_ranges_;
  Member<EventQueue> async_event_queue_;

  double playback_rate_;
  double default_playback_rate_;
  NetworkState network_state_;
  ReadyState ready_state_;
  ReadyState ready_state_maximum_;

  SourceMetadata current_src_;
  KURL current_src_after_redirects_;

  Member<MediaStreamDescriptor> src_object_stream_descriptor_;
  Member<MediaSourceHandle> src_object_media_source_handle_;

  // To prevent potential regression when extended by the MSE API, do not set
  // |error_| outside of constructor and SetError().
  Member<MediaError> error_;

  double volume_;
  double last_seek_time_;

  std::optional<base::ElapsedTimer> previous_progress_time_;

  // Cached duration to suppress duplicate events if duration unchanged.
  double duration_;

  // The last time a timeupdate event was sent in movie time.
  double last_time_update_event_media_time_;

  // The default playback start position.
  double default_playback_start_position_;

  // Loading state.
  enum LoadState {
    kWaitingForSource,
    kLoadingFromSrcObject,
    kLoadingFromSrcAttr,
    kLoadingFromSourceElement
  };
  LoadState load_state_;
  Member<HTMLSourceElement> current_source_node_;
  Member<Node> next_child_node_to_consider_;

  // "Deferred loading" state (for preload=none).
  enum DeferredLoadState {
    // The load is not deferred.
    kNotDeferred,
    // The load is deferred, and waiting for the task to set the
    // delaying-the-load-event flag (to false).
    kWaitingForStopDelayingLoadEventTask,
    // The load is the deferred, and waiting for a triggering event.
    kWaitingForTrigger,
    // The load is deferred, and waiting for the task to set the
    // delaying-the-load-event flag, after which the load will be executed.
    kExecuteOnStopDelayingLoadEventTask
  };
  DeferredLoadState deferred_load_state_;
  HeapTaskRunnerTimer<HTMLMediaElement> deferred_load_timer_;

  std::unique_ptr<WebMediaPlayer> web_media_player_;
  cc::Layer* cc_layer_;

  // These two fields must be carefully set and reset: the actual derived type
  // of the attachment (same-thread vs cross-thread, for instance) must be the
  // same semantic as the actual derived type of the tracer. Further, if there
  // is no attachment, then there must be no tracer that's tracking an active
  // attachment. Note that some kinds of attachments do not require a tracer;
  // see MediaSourceAttachment::StartAttachingToMediaElement() for details.
  scoped_refptr<MediaSourceAttachment> media_source_attachment_;
  Member<MediaSourceTracer> media_source_tracer_;

  // Stores "official playback position", updated periodically from "current
  // playback position". Official playback position should not change while
  // scripts are running. See setOfficialPlaybackPosition().
  mutable double official_playback_position_;
  mutable bool official_playback_position_needs_update_;

  double fragment_end_time_;

  typedef unsigned PendingActionFlags;
  PendingActionFlags pending_action_flags_;

  // FIXME: HTMLMediaElement has way too many state bits.
  bool playing_ : 1;
  bool should_delay_load_event_ : 1;
  bool have_fired_loaded_data_ : 1;
  bool can_autoplay_ : 1;
  bool muted_ : 1;
  bool paused_ : 1;
  bool seeking_ : 1;
  bool paused_by_context_paused_ : 1;
  bool show_poster_flag_ : 1;

  // data has not been loaded since sending a "stalled" event
  bool sent_stalled_event_ : 1;

  bool ignore_preload_none_ : 1;

  bool text_tracks_visible_ : 1;
  bool should_perform_automatic_track_selection_ : 1;

  bool tracks_are_ready_ : 1;
  bool processing_preference_change_ : 1;

  bool was_always_muted_ : 1;

  // Set if the user has used the context menu to set the visibility of the
  // controls.
  std::optional<bool> user_wants_controls_visible_;

  // Whether or not |web_media_player_| should apply pitch adjustments at
  // playback raters other than 1.0.
  bool preserves_pitch_ = true;

  // Whether the player disables the Remote Playback feature.
  bool is_remote_playback_disabled_ = false;
  // Whether the player is rendering remotely.
  bool is_remote_rendering_ = false;
  // Whether the media content is encrypted.
  bool is_encrypted_media_ = false;

  // Whether webaudio destination is connected.
  bool is_audio_destination_connected_ = false;
  WebString remote_device_friendly_name_;
  std::optional<media::AudioCodec> audio_codec_ = std::nullopt;
  std::optional<media::VideoCodec> video_codec_ = std::nullopt;

  Member<AudioTrackList> audio_tracks_;
  Member<VideoTrackList> video_tracks_;
  Member<TextTrackList> text_tracks_;
  HeapVector<Member<TextTrack>> text_tracks_when_resource_selection_began_;

  Member<CueTimeline> cue_timeline_;

  HeapVector<Member<ScriptPromiseResolverBase>> play_promise_resolvers_;
  TaskHandle play_promise_resolve_task_handle_;
  TaskHandle play_promise_reject_task_handle_;
  HeapVector<Member<ScriptPromiseResolverBase>> play_promise_resolve_list_;
  HeapVector<Member<ScriptPromiseResolverBase>> play_promise_reject_list_;
  PlayPromiseError play_promise_error_code_;

  // HTMLMediaElement and its MediaElementAudioSourceNode in case it is provided
  // die together.
  Member<AudioSourceProviderClient> audio_source_node_;

  // Controls browser vocalization within the media element (e.g. to speak cues,
  // to pause utterance).
  Member<SpeechSynthesisBase> speech_synthesis_;

  // AudioClientImpl wraps an AudioSourceProviderClient.
  // When the audio format is known, Chromium calls setFormat().
  class AudioClientImpl final : public GarbageCollected<AudioClientImpl>,
                                public WebAudioSourceProviderClient {
   public:
    explicit AudioClientImpl(AudioSourceProviderClient* client)
        : client_(client) {}

    ~AudioClientImpl() override = default;

    // WebAudioSourceProviderClient
    void SetFormat(uint32_t number_of_channels, float sample_rate) override;

    void Trace(Visitor*) const;

   private:
    Member<AudioSourceProviderClient> client_;
  };

  // AudioSourceProviderImpl wraps a WebAudioSourceProvider.
  // provideInput() calls into Chromium to get a rendered audio stream.
  class AudioSourceProviderImpl final : public AudioSourceProvider {
    DISALLOW_NEW();

   public:
    AudioSourceProviderImpl() = default;
    ~AudioSourceProviderImpl() override = default;

    // Wraps the given WebAudioSourceProvider.
    void Wrap(scoped_refptr<WebAudioSourceProviderImpl>);

    // AudioSourceProvider
    void SetClient(AudioSourceProviderClient*) override;
    void ProvideInput(AudioBus*, int frames_to_process) override;

    void ConnectToDestinationReady() override;

    void Trace(Visitor*) const;

   private:
    base::Lock provide_input_lock;
    scoped_refptr<WebAudioSourceProviderImpl> web_audio_source_provider_
        GUARDED_BY(provide_input_lock);

    // Resampling case, connect to the destination can be called before
    // `audio_source_provider_` is ready. We have to call it during
    // `audio_source_provider_` assignment.
    bool connection_to_destination_ready_ = false;

    Member<AudioClientImpl> client_;
  };

  AudioSourceProviderImpl audio_source_provider_;

  // Notify HTMLMediaElement when a document's ExecutionContext is destroyed.
  // It allows us to disconnect from a previous document's frame if we were
  // using it to support our WebMediaPlayer rather than our current frame.
  class OpenerContextObserver final
      : public GarbageCollected<OpenerContextObserver>,
        public ContextLifecycleObserver {
   public:
    // Notify `element` when our context is destroyed.
    explicit OpenerContextObserver(HTMLMediaElement* element);
    ~OpenerContextObserver() final;

    void Trace(Visitor* visitor) const final;

   protected:
    void ContextDestroyed() final;

    Member<HTMLMediaElement> element_;
  };

  // Clean up things that are tied to any previous frame, including the player
  // and mojo interfaces, when we switch to a new frame.
  void AttachToNewFrame();

  Member<Document> opener_document_;
  Member<OpenerContextObserver> opener_context_observer_;

  friend class AutoplayPolicy;
  friend class AutoplayUmaHelperTest;
  friend class Internals;
  friend class TrackDisplayUpdateScope;
  friend class MediaControlsImplTest;
  friend class HTMLMediaElementTest;
  friend class HTMLMediaElementEventListenersTest;
  friend class HTMLVideoElement;
  friend class MediaControlInputElementTest;
  friend class MediaControlsOrientationLockDelegateTest;
  friend class MediaControlsRotateToFullscreenDelegateTest;
  friend class MediaControlLoadingPanelElementTest;
  friend class ContextMenuControllerTest;
  friend class HTMLVideoElementTest;

  Member<AutoplayPolicy> autoplay_policy_;

  RemotePlaybackClient* remote_playback_client_ = nullptr;

  Member<MediaControls> media_controls_;
  Member<HTMLMediaElementControlsList> controls_list_;

  Member<IntersectionObserver> lazy_load_intersection_observer_;

  Member<DisallowNewWrapper<
      HeapMojoAssociatedRemote<media::mojom::blink::MediaPlayerHost>>>
      media_player_host_remote_;

  // Note: There's only ever one entry in this set.
  Member<DisallowNewWrapper<
      HeapMojoAssociatedRemoteSet<media::mojom::blink::MediaPlayerObserver>>>
      media_player_observer_remote_set_;

  // A receiver set is needed here as there will be different objects in the
  // browser communicating with this object. This is done this way to avoid
  // routing everything through a single class (e.g. RFHI) and to keep this
  // logic contained inside MediaPlayer-related classes.
  Member<DisallowNewWrapper<
      HeapMojoAssociatedReceiverSet<media::mojom::blink::MediaPlayer,
                                    HTMLMediaElement>>>
      media_player_receiver_set_;
};

template <>
struct DowncastTraits<HTMLMediaElement> {
  static bool AllowFrom(const Node& node) {
    auto* html_element = DynamicTo<HTMLElement>(node);
    return html_element && AllowFrom(*html_element);
  }
  static bool AllowFrom(const HTMLElement& html_element) {
    return html_element.HasTagName(html_names::kAudioTag) ||
           html_element.HasTagName(html_names::kVideoTag);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_HTML_MEDIA_ELEMENT_H_
