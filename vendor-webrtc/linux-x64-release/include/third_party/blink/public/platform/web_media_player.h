/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_PLAYER_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_PLAYER_H_

#include <optional>

#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/time/time.h"
#include "components/viz/common/surfaces/surface_id.h"
#include "media/base/picture_in_picture_events_info.h"
#include "media/base/video_frame.h"
#include "media/base/video_frame_metadata.h"
#include "third_party/blink/public/platform/web_audio_source_provider_impl.h"
#include "third_party/blink/public/platform/web_content_decryption_module.h"
#include "third_party/blink/public/platform/web_media_source.h"
#include "third_party/blink/public/platform/web_set_sink_id_callbacks.h"
#include "third_party/blink/public/platform/web_string.h"
#include "ui/gfx/geometry/rect.h"
#include "ui/gfx/geometry/size.h"
#include "url/gurl.h"

namespace cc {
class PaintCanvas;
class PaintFlags;
}  // namespace cc

namespace media {
class PaintCanvasVideoRenderer;
}

namespace blink {

class WebContentDecryptionModule;
class WebMediaPlayerSource;
class WebString;
class WebURL;
enum class WebFullscreenVideoStatus;

class WebMediaPlayer {
 public:
  enum NetworkState {
    kNetworkStateEmpty,
    kNetworkStateIdle,
    kNetworkStateLoading,
    kNetworkStateLoaded,
    kNetworkStateFormatError,
    kNetworkStateNetworkError,
    kNetworkStateDecodeError,
  };

  enum ReadyState {
    kReadyStateHaveNothing,
    kReadyStateHaveMetadata,
    kReadyStateHaveCurrentData,
    kReadyStateHaveFutureData,
    kReadyStateHaveEnoughData,
  };

  enum Preload {
    kPreloadNone,
    kPreloadMetaData,
    kPreloadAuto,
  };

  enum CorsMode {
    kCorsModeUnspecified,
    kCorsModeAnonymous,
    kCorsModeUseCredentials,
  };

  // Reported to UMA. Do not change existing values.
  enum LoadType {
    kLoadTypeURL = 0,
    kLoadTypeMediaSource = 1,
    kLoadTypeMediaStream = 2,
    kLoadTypeMax = kLoadTypeMediaStream,
  };

  typedef WebString TrackId;
  enum TrackType { kTextTrack, kAudioTrack, kVideoTrack };

  // This must stay in sync with WebGLRenderingContextBase::TexImageFunctionID.
  enum TexImageFunctionID {
    kTexImage2D,
    kTexSubImage2D,
    kTexImage3D,
    kTexSubImage3D
  };

  // Returned by Load() to signal when a players choose to defer (e.g. as part
  // of pre-rendering)
  enum LoadTiming { kImmediate, kDeferred };

  enum class DisplayType {
    // Playback is happening inline.
    kInline,
    // Playback is happening either with the video fullscreen. It may also be
    // set when Blink detects that the video is effectively fullscreen even if
    // the element is not.
    kFullscreen,
    // Playback is happening in a video Picture-in-Picture window.
    kVideoPictureInPicture,
    // Playback is happening in a document Picture-in-Picture window.
    kDocumentPictureInPicture,
  };

  // This is the reason supplied to `WebMediaPlayer::Pause()`. A
  // `WebMediaPlayer` can be paused for many reasons that affect the internal
  // state — including resumption strategies — differently. For example, a
  // player can be paused to optimize a background tab, in which case
  // foregrounding the tab could resume playback. Conversely, a non-optimized
  // backgrounded tab can pause its media explicitly via
  // `HTMLMediaElement::pause()`; in that case, foregrounding the tab should NOT
  // resume playback.
  enum class PauseReason {
    // The player's tab is in the background.
    kPageHidden,
    // The player's frame is not rendered.
    kFrameHidden,
    // The player has been backgrounded for too long and will be paused to save
    // resources.
    kSuspendedPlayerIdleTimeout,
    // The remote cast device has requested to pause the media.
    kRemotePlayStateChange,
    kEndOfPlayback,
    // HTMLMediaElement::pause() was called.
    kPauseCalled,
    // The Browser process has requested to pause the media.
    // TODO(crbug.com/40623496): Make sure that this is only used when there is
    // a user gesture.
    kPauseRequestedByUser,
    kPauseRequestedInternally,
    // The media element has been removed from the document.
    kRemovedFromDocument,
    // The Autoplay policy has requested to pause the media. This can happen
    // when a muted HTMLMediaElement has started autoplaying and is not rendered
    // in the viewport anymore.
    kAutoplayAutoPause,
    // The audio description track is lagging behind and we need to pause for it
    // to catch up.
    kLetAudioDescriptionFinish,
  };

  // For video.requestVideoFrameCallback(). https://wicg.github.io/video-rvfc/
  struct VideoFramePresentationMetadata {
    uint32_t presented_frames;
    base::TimeTicks presentation_time;
    base::TimeTicks expected_display_time;
    int width;
    int height;
    base::TimeDelta media_time;
    media::VideoFrameMetadata metadata;
    base::TimeDelta rendering_interval;
    base::TimeDelta average_frame_duration;
  };

  virtual ~WebMediaPlayer() = default;

  virtual LoadTiming Load(LoadType,
                          const WebMediaPlayerSource&,
                          CorsMode,
                          bool is_cache_disabled) = 0;

  // Playback controls.
  virtual void Play() = 0;
  virtual void Pause(PauseReason pause_reason) = 0;
  virtual void Seek(double seconds) = 0;
  virtual void SetRate(double) = 0;
  virtual void SetVolume(double) = 0;

  // Set a target value for media pipeline latency for post-decode buffering.
  // |seconds| is a target value for post-decode buffering latency. As a default
  // |seconds| may also be NaN, indicating no preference. NaN will also be the
  // value if the hint is cleared.
  virtual void SetLatencyHint(double seconds) = 0;

  // Sets a flag indicating that the WebMediaPlayer should apply pitch
  // adjustments when using a playback rate other than 1.0.
  virtual void SetPreservesPitch(bool preserves_pitch) = 0;

  // Sets a flag indicating whether the audio stream was played with user
  // activation and high media engagement.
  virtual void SetWasPlayedWithUserActivationAndHighMediaEngagement(
      bool was_played_with_user_activation_and_high_media_engagement) = 0;

  // Sets a flag indicating whether media playback should be paused when the
  // the iframe is hidden.
  virtual void SetShouldPauseWhenFrameIsHidden(
      bool should_pause_when_frame_is_hidden) = 0;
  virtual bool GetShouldPauseWhenFrameIsHidden() { return false; }

  // The associated media element is going to enter Picture-in-Picture. This
  // method should make sure the player is set up for this and has a SurfaceId
  // as it will be needed.
  virtual void OnRequestPictureInPicture() = 0;

  // Called to notify about changes of the associated media element's media
  // time, playback rate, and duration. During uninterrupted playback, the
  // calls are still made periodically.
  virtual void OnTimeUpdate() {}

  virtual void RequestRemotePlaybackDisabled(bool disabled) {}
  virtual void RequestMediaRemoting() {}
  virtual void FlingingStarted() {}
  virtual void FlingingStopped() {}

  virtual void SetPreload(Preload) {}
  virtual WebTimeRanges Buffered() const = 0;
  virtual WebTimeRanges Seekable() const = 0;

  // Called when the backing media element and the page it is attached to is
  // frozen, meaning that the page is no longer being rendered but nothing has
  // yet been deconstructed. This may occur in several cases, such as bfcache
  // for instant backwards and forwards navigation.
  virtual void OnFrozen() = 0;

  // Attempts to switch the audio output device.
  virtual bool SetSinkId(const WebString& sing_id,
                         WebSetSinkIdCompleteCallback) = 0;

  // True if the loaded media has a playable video/audio track.
  virtual bool HasVideo() const = 0;
  virtual bool HasAudio() const = 0;

  // Dimension of the video.
  virtual gfx::Size NaturalSize() const = 0;

  virtual gfx::Size VisibleSize() const = 0;

  // Getters of playback state.
  virtual bool Paused() const = 0;
  virtual bool Seeking() const = 0;
  // MSE allows authors to assign double values for duration.
  // Here, we return double rather than TimeDelta to ensure
  // that authors are returned exactly the value that they assign.
  virtual double Duration() const = 0;
  virtual double CurrentTime() const = 0;
  virtual bool IsEnded() const = 0;

  // Internal states of loading and network.
  virtual NetworkState GetNetworkState() const = 0;
  virtual ReadyState GetReadyState() const = 0;

  // Returns an implementation-specific human readable error message, or an
  // empty string if no message is available. The message should begin with a
  // UA-specific-error-code (without any ':'), optionally followed by ': ' and
  // further description of the error.
  virtual WebString GetErrorMessage() const = 0;

  virtual bool DidLoadingProgress() = 0;

  // Returns true if the response is CORS-cross-origin and so we shouldn't be
  // allowing media to play through webaudio.
  // This should be called after the response has arrived.
  virtual bool WouldTaintOrigin() const = 0;

  virtual double MediaTimeForTimeValue(double time_value) const = 0;

  virtual unsigned DecodedFrameCount() const = 0;
  virtual unsigned DroppedFrameCount() const = 0;
  virtual unsigned CorruptedFrameCount() const { return 0; }
  virtual uint64_t AudioDecodedByteCount() const = 0;
  virtual uint64_t VideoDecodedByteCount() const = 0;

  // Returns false if any of the HTTP responses which make up the video data
  // loaded so far have failed the TAO check as defined by Fetch
  // (https://fetch.spec.whatwg.org/#tao-check), or true otherwise. Video
  // streams which do not originate from HTTP responses should return true here.
  // This check is used to determine if timing information from those responses
  // may be exposed to the page in Largest Contentful Paint performance entries.
  virtual bool PassedTimingAllowOriginCheck() const = 0;

  // Set the volume multiplier to control audio ducking.
  // Output volume should be set to |player_volume| * |multiplier|. The range
  // of |multiplier| is [0, 1], where 1 indicates normal (non-ducked) playback.
  virtual void SetVolumeMultiplier(double multiplier) = 0;

  // Set the player as the persistent video. Persistent video should hide its
  // controls and go fullscreen.
  virtual void SetPersistentState(bool persistent) {}

  // Notify the player that it is now eligible to start recording power
  // measurements if |state| is true, else it is no longer eligible.
  virtual void SetPowerExperimentState(bool enabled) {}

  // Suspends the player for the host frame closed.
  virtual void SuspendForFrameClosed() = 0;

  // Returns true if the player has a frame available for presentation. Usually
  // this just means the first frame has been delivered.
  virtual bool HasAvailableVideoFrame() const = 0;

  // Returns true if the player has a frame available for presentation, and the
  // frame is readable, i.e. it's not protected and can be read back into CPU
  // memory.
  virtual bool HasReadableVideoFrame() const = 0;

  // Renders the current frame into the provided cc::PaintCanvas.
  virtual void Paint(cc::PaintCanvas*, const gfx::Rect&, cc::PaintFlags&) = 0;

  // Similar to Paint(), but just returns the frame directly instead of trying
  // to upload or convert it. Note: This may kick off a process to update the
  // current frame for a future call in some cases. Returns nullptr if no frame
  // is available.
  virtual scoped_refptr<media::VideoFrame> GetCurrentFrameThenUpdate() = 0;

  // Return current video frame unique id from compositor. The query is readonly
  // and should avoid any extra ops. Function returns std::nullopt if current
  // frame is invalid or fails to access current frame.
  virtual std::optional<media::VideoFrame::ID> CurrentFrameId() const = 0;

  // Provides a PaintCanvasVideoRenderer instance owned by this WebMediaPlayer.
  // Useful for ensuring that the paint/texturing operation for current frame is
  // cached in cases of repainting/retexturing (since clients may not know that
  // the underlying frame is unchanged). May only be used on the main thread and
  // should not be held outside the scope of a single call site.
  virtual media::PaintCanvasVideoRenderer* GetPaintCanvasVideoRenderer() {
    return nullptr;
  }

  virtual scoped_refptr<WebAudioSourceProviderImpl> GetAudioSourceProvider() {
    return nullptr;
  }

  virtual void SetContentDecryptionModule(
      WebContentDecryptionModule* cdm,
      WebContentDecryptionModuleResult result) {
    result.CompleteWithError(
        kWebContentDecryptionModuleExceptionNotSupportedError, 0, "ERROR");
  }

  // Sets a flag indicating whether to render muted audio to the active sink or
  // switch to a null sink.
  virtual void SetRenderMutedAudio(bool render_muted_audio) {}

  // Sets the poster image URL.
  virtual void SetPoster(const WebURL& poster) {}

  // Inform WebMediaPlayer when the element has entered/exited fullscreen.
  virtual void EnteredFullscreen() {}
  virtual void ExitedFullscreen() {}

  // Inform WebMediaPlayer when the element starts/stops being the dominant
  // visible content. This will only be called after the monitoring of the
  // intersection with viewport is activated by calling
  // WebMediaPlayerClient::ActivateViewportIntersectionMonitoring().
  virtual void BecameDominantVisibleContent(bool is_dominant) {}

  // Inform WebMediaPlayer when the element starts/stops being the effectively
  // fullscreen video, i.e. being the fullscreen element or child of the
  // fullscreen element, and being dominant in the viewport.
  //
  // TODO(zqzhang): merge with BecameDominantVisibleContent(). See
  // https://crbug.com/696211
  virtual void SetIsEffectivelyFullscreen(WebFullscreenVideoStatus) {}

  virtual void EnabledAudioTracksChanged(
      const std::vector<TrackId>& enabled_track_ids) {}
  virtual void SelectedVideoTrackChanged(
      std::optional<TrackId> selected_track_id) {}

  // Callback called whenever the media element may have received or last native
  // controls. It might be called twice with the same value: the caller has to
  // check if the value have changed if it only wants to handle this case.
  // This method is not used to say express if the native controls are visible
  // but if the element is using them.
  virtual void OnHasNativeControlsChanged(bool) {}

  // Callback called whenever the media element display type changes. By
  // default, the display type is `kInline`.
  virtual void OnDisplayTypeChanged(DisplayType) {}

  // Test helper methods for exercising media suspension.
  virtual void ForceStaleStateForTesting(ReadyState target_state) {}
  virtual bool IsSuspendedForTesting() { return false; }

  virtual bool DidLazyLoad() const { return false; }
  virtual void OnBecameVisible() {}

  virtual bool IsOpaque() const { return false; }

  // Returns a per-process unique ID for this WebMediaPlayer that can
  // be passed to mojo services.
  virtual int GetPlayerId() { return -1; }

  // Returns the SurfaceId the video element is currently using.
  // Returns std::nullopt if the element isn't a video or doesn't have a
  // SurfaceId associated to it.
  virtual std::optional<viz::SurfaceId> GetSurfaceId() { return std::nullopt; }

  // Provide the media URL, after any redirects are applied.  May return an
  // empty GURL, which will be interpreted as "use the original URL".
  virtual GURL GetSrcAfterRedirects() { return GURL(); }

  // Register a request to be notified the next time a video frame is presented
  // to the compositor. The request will be completed via
  // WebMediaPlayerClient::OnRequestVideoFrameCallback(). The frame info can be
  // retrieved via GetVideoFramePresentationMetadata().
  // See https://wicg.github.io/video-rvfc/.
  virtual void RequestVideoFrameCallback() {}
  virtual std::unique_ptr<VideoFramePresentationMetadata>
  GetVideoFramePresentationMetadata() {
    return nullptr;
  }

  // Forces the WebMediaPlayer to update its frame if it is stale. This is used
  // during immersive WebXR sessions with the RequestVideoFrameCallback() API,
  // when compositors aren't driving frame updates.
  virtual void UpdateFrameIfStale() {}

  virtual base::WeakPtr<WebMediaPlayer> AsWeakPtr() = 0;

  // Adjusts the frame sink hierarchy for the media frame sink.
  virtual void RegisterFrameSinkHierarchy() {}
  virtual void UnregisterFrameSinkHierarchy() {}

  // Records the `MediaVideoVisibilityTracker` occlusion state, at the time that
  // HTMLVideoElement visibility is reported. The state is recorded using
  // `MediaLogEvent` s.
  virtual void RecordVideoOcclusionState(std::string_view occlusion_state) {}

  // Request the media player to record auto picture in picture related
  // information. This information helps identify why a request to enter picture
  // in picture automatically is denied/accepted.
  virtual void RecordAutoPictureInPictureInfo(
      const media::PictureInPictureEventsInfo::AutoPipInfo&
          auto_picture_in_picture_info) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_MEDIA_PLAYER_H_
