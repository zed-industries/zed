// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIASTREAM_WEB_MEDIA_PLAYER_MS_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIASTREAM_WEB_MEDIA_PLAYER_MS_H_

#include <memory>
#include <string>

#include "base/memory/raw_ptr.h"
#include "base/memory/weak_ptr.h"
#include "base/synchronization/lock.h"
#include "base/task/sequenced_task_runner.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/thread_checker.h"
#include "base/time/time.h"
#include "build/build_config.h"
#include "media/mojo/mojom/media_types.mojom-shared.h"
#include "media/renderers/paint_canvas_video_renderer.h"
#include "media/video/gpu_video_accelerator_factories.h"
#include "third_party/blink/public/platform/media/web_media_player_delegate.h"
#include "third_party/blink/public/platform/modules/mediastream/web_media_stream.h"
#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_media_player.h"
#include "third_party/blink/public/platform/web_surface_layer_bridge.h"

namespace media {
class GpuMemoryBufferVideoFramePool;
class MediaLog;
}  // namespace media

namespace cc {
class VideoLayer;
}

namespace blink {

using CreateSurfaceLayerBridgeCB =
    base::OnceCallback<std::unique_ptr<WebSurfaceLayerBridge>(
        WebSurfaceLayerBridgeObserver*,
        cc::UpdateSubmissionStateCB)>;

class MediaPlayerClient;
class MediaStreamAudioRenderer;
class MediaStreamInternalFrameWrapper;
class MediaStreamRendererFactory;
class MediaStreamVideoRenderer;
template <typename TimerFiredClass>
class TaskRunnerTimer;
class TimerBase;
class WatchTimeReporter;
class WebLocalFrame;
class WebMediaPlayerMSCompositor;
class WebString;
class WebVideoFrameSubmitter;
class WebMediaPlayerClient;

// WebMediaPlayerMS delegates calls from WebCore::MediaPlayerPrivate to
// Chrome's media player when "src" is from media stream.
//
// All calls to WebMediaPlayerMS methods must be from the main thread of
// Renderer process.
//
// WebMediaPlayerMS works with multiple objects, the most important ones are:
//
// MediaStreamVideoRenderer
//   provides video frames for rendering.
//
// MediaPlayerClient
//   WebKit client of this media player object.
class BLINK_MODULES_EXPORT WebMediaPlayerMS
    : public WebMediaStreamObserver,
      public WebMediaPlayer,
      public WebMediaPlayerDelegate::Observer,
      public WebSurfaceLayerBridgeObserver {
 public:
  // Construct a WebMediaPlayerMS with reference to the client, and
  // a MediaStreamClient which provides MediaStreamVideoRenderer.
  // |delegate| must not be null.
  WebMediaPlayerMS(
      WebLocalFrame* frame,
      WebMediaPlayerClient* client,
      WebMediaPlayerDelegate* delegate,
      std::unique_ptr<media::MediaLog> media_log,
      scoped_refptr<base::SingleThreadTaskRunner> main_render_task_runner,
      scoped_refptr<base::SequencedTaskRunner> video_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> compositor_task_runner,
      scoped_refptr<base::SequencedTaskRunner> media_task_runner,
      scoped_refptr<base::TaskRunner> worker_task_runner,
      media::GpuVideoAcceleratorFactories* gpu_factories,
      const WebString& sink_id,
      CreateSurfaceLayerBridgeCB create_bridge_callback,
      std::unique_ptr<WebVideoFrameSubmitter> submitter_,
      bool use_surface_layer);

  WebMediaPlayerMS(const WebMediaPlayerMS&) = delete;
  WebMediaPlayerMS& operator=(const WebMediaPlayerMS&) = delete;

  ~WebMediaPlayerMS() override;

  WebMediaPlayer::LoadTiming Load(LoadType load_type,
                                  const WebMediaPlayerSource& source,
                                  CorsMode cors_mode,
                                  bool is_cache_disabled) override;

  // WebSurfaceLayerBridgeObserver implementation.
  void OnWebLayerUpdated() override;
  void RegisterContentsLayer(cc::Layer* layer) override;
  void UnregisterContentsLayer(cc::Layer* layer) override;
  void OnSurfaceIdUpdated(viz::SurfaceId surface_id) override;

  // Playback controls.
  void Play() override;
  void Pause(PauseReason pause_reason) override;
  void Seek(double seconds) override;
  void SetRate(double rate) override;
  void SetVolume(double volume) override;
  void SetLatencyHint(double seconds) override;
  void SetPreservesPitch(bool preserves_pitch) override;
  void SetWasPlayedWithUserActivationAndHighMediaEngagement(
      bool was_played_with_user_activation_and_high_media_engagement) override;
  void OnRequestPictureInPicture() override;
  bool SetSinkId(const WebString& sink_id,
                 WebSetSinkIdCompleteCallback completion_callback) override;
  void SetPreload(WebMediaPlayer::Preload preload) override;
  WebTimeRanges Buffered() const override;
  WebTimeRanges Seekable() const override;
  void OnFrozen() override;

  // Methods for painting.
  void Paint(cc::PaintCanvas* canvas,
             const gfx::Rect& rect,
             cc::PaintFlags& flags) override;
  scoped_refptr<media::VideoFrame> GetCurrentFrameThenUpdate() override;
  std::optional<media::VideoFrame::ID> CurrentFrameId() const override;
  media::PaintCanvasVideoRenderer* GetPaintCanvasVideoRenderer() override;
  void ResetCanvasCache();

  // Methods to trigger resize event.
  void TriggerResize();

  // True if the loaded media has a playable video/audio track.
  bool HasVideo() const override;
  bool HasAudio() const override;

  // Dimensions of the video.
  gfx::Size NaturalSize() const override;

  gfx::Size VisibleSize() const override;

  // Getters of playback state.
  bool Paused() const override;
  bool Seeking() const override;
  double Duration() const override;
  double CurrentTime() const override;
  bool IsEnded() const override;

  // Internal states of loading and network.
  WebMediaPlayer::NetworkState GetNetworkState() const override;
  WebMediaPlayer::ReadyState GetReadyState() const override;

  WebString GetErrorMessage() const override;
  bool DidLoadingProgress() override;

  bool WouldTaintOrigin() const override;

  double MediaTimeForTimeValue(double timeValue) const override;

  unsigned DecodedFrameCount() const override;
  unsigned DroppedFrameCount() const override;
  uint64_t AudioDecodedByteCount() const override;
  uint64_t VideoDecodedByteCount() const override;

  // WebRTC doesn't need TAO checks, as the timing is already available through
  // getStats().
  bool PassedTimingAllowOriginCheck() const override { return true; }
  bool HasAvailableVideoFrame() const override;
  bool HasReadableVideoFrame() const override;

  void SetVolumeMultiplier(double multiplier) override;
  void SuspendForFrameClosed() override;

  void SetShouldPauseWhenFrameIsHidden(
      bool should_pause_when_frame_is_hidden) override;
  bool GetShouldPauseWhenFrameIsHidden() override;

  bool ShouldPausePlaybackWhenHidden() const;

  // WebMediaPlayerDelegate::Observer implementation.
  void OnFrameHidden() override;
  void OnFrameShown() override;
  void OnPageHidden() override;
  void OnPageShown() override;
  void OnIdleTimeout() override;

  void OnFirstFrameReceived(media::VideoTransformation video_transform,
                            bool is_opaque);
  void OnOpacityChanged(bool is_opaque);
  void OnTransformChanged(media::VideoTransformation video_transform);

  // WebMediaStreamObserver implementation
  void TrackAdded(const WebString& track_id) override;
  void TrackRemoved(const WebString& track_id) override;
  void ActiveStateChanged(bool is_active) override;
  void EnabledStateChangedForWebRtcAudio(bool is_enabled) override;
  int GetPlayerId() override { return player_id_; }
  std::optional<viz::SurfaceId> GetSurfaceId() override;

  base::WeakPtr<WebMediaPlayer> AsWeakPtr() override;

  void OnDisplayTypeChanged(WebMediaPlayer::DisplayType) override;

  void RequestVideoFrameCallback() override;
  std::unique_ptr<WebMediaPlayer::VideoFramePresentationMetadata>
  GetVideoFramePresentationMetadata() override;

  void RegisterFrameSinkHierarchy() override;
  void UnregisterFrameSinkHierarchy() override;

  void RecordAutoPictureInPictureInfo(
      const media::PictureInPictureEventsInfo::AutoPipInfo&
          auto_picture_in_picture_info) override {}

 private:
  friend class WebMediaPlayerMSTest;

#if BUILDFLAG(IS_WIN)
  static const gfx::Size kUseGpuMemoryBufferVideoFramesMinResolution;
#endif  // BUILDFLAG(IS_WIN)

  void ReplaceCurrentFrameWithACopy();

  bool IsInPictureInPicture() const;

  // Switch to SurfaceLayer, either initially or from VideoLayer.
  void ActivateSurfaceLayerForVideo(media::VideoTransformation video_transform);

  // Need repaint due to state change.
  void RepaintInternal();

  // Helpers that set the network/ready state and notifies the client if
  // they've changed.
  void SetNetworkState(WebMediaPlayer::NetworkState state);
  void SetReadyState(WebMediaPlayer::ReadyState state);

  // Getter method to |client_|.
  MediaPlayerClient* get_client() { return client_; }

  // To be run when tracks are added or removed.
  void Reload();
  void ReloadVideo();
  void ReloadAudio();

  // Helper method used for testing.
  void SetGpuMemoryBufferVideoForTesting(
      media::GpuMemoryBufferVideoFramePool* gpu_memory_buffer_pool);
  void SetMediaStreamRendererFactoryForTesting(
      std::unique_ptr<MediaStreamRendererFactory>);

  // Callback used to fulfill video.requestVideoFrameCallback() requests.
  void OnNewFramePresentedCallback();

  // Callback used to detect and propagate a render error.
  void OnAudioRenderErrorCallback();

  void SendLogMessage(const WTF::String& message) const;

  void StopForceBeginFrames(TimerBase*);

  void MaybeCreateWatchTimeReporter();
  void UpdateWatchTimeReporterSecondaryProperties();
  base::TimeDelta GetCurrentTimeInterval();
  media::PipelineStatistics GetPipelineStatistics();

  std::optional<media::mojom::MediaStreamType> GetMediaStreamType();

  std::unique_ptr<MediaStreamInternalFrameWrapper> internal_frame_;

  WebMediaPlayer::NetworkState network_state_;
  WebMediaPlayer::ReadyState ready_state_;

  const WebTimeRanges buffered_;

  const raw_ptr<MediaPlayerClient> client_;

  // WebMediaPlayer notifies the |delegate_| of playback state changes using
  // |delegate_id_|; an id provided after registering with the delegate.  The
  // WebMediaPlayer may also receive directives (play, pause) from the delegate
  // via the WebMediaPlayerDelegate::Observer interface after
  // registration.
  //
  // NOTE: HTMLMediaElement is a blink::ExecutionContextLifecycleObserver, and
  // will receive a call to contextDestroyed() when Document::shutdown() is
  // called. Document::shutdown() is called before the frame detaches (and
  // before the frame is destroyed). RenderFrameImpl owns of |delegate_|, and is
  // guaranteed to outlive |this|. It is therefore safe use a raw pointer
  // directly.
  raw_ptr<WebMediaPlayerDelegate> delegate_;
  int delegate_id_;

  const int player_id_;

  // Inner class used for transferring frames on compositor thread to
  // |compositor_|.
  class FrameDeliverer;
  std::unique_ptr<FrameDeliverer> frame_deliverer_;

  scoped_refptr<MediaStreamVideoRenderer> video_frame_provider_;  // Weak

  scoped_refptr<cc::VideoLayer> video_layer_;

  scoped_refptr<MediaStreamAudioRenderer> audio_renderer_;  // Weak
  media::PaintCanvasVideoRenderer video_renderer_;

  // Indicated whether an outstanding VideoFrameCallback request needs to be
  // forwarded to |compositor_|. Set when RequestVideoFrameCallback() is called
  // before Load().
  bool pending_rvfc_request_ = false;

  bool paused_;

  std::unique_ptr<media::MediaLog> media_log_;

  std::unique_ptr<MediaStreamRendererFactory> renderer_factory_;

  const scoped_refptr<base::SingleThreadTaskRunner> main_render_task_runner_;
  const scoped_refptr<base::SequencedTaskRunner> video_task_runner_;
  const scoped_refptr<base::SingleThreadTaskRunner> compositor_task_runner_;
  const scoped_refptr<base::SequencedTaskRunner> media_task_runner_;

  const scoped_refptr<base::TaskRunner> worker_task_runner_;
  raw_ptr<media::GpuVideoAcceleratorFactories> gpu_factories_;

  // Used for DCHECKs to ensure methods calls executed in the correct thread.
  THREAD_CHECKER(thread_checker_);

  std::unique_ptr<WebMediaPlayerMSCompositor> compositor_;

  const WebString initial_audio_output_device_id_;

  // The last volume received by setVolume() and the last volume multiplier from
  // SetVolumeMultiplier().  The multiplier is typical 1.0, but may be less
  // if the WebMediaPlayerDelegate has requested a volume reduction
  // (ducking) for a transient sound.  Playout volume is derived by volume *
  // multiplier.
  double volume_;
  double volume_before_muted_;
  double volume_multiplier_;
  bool enabled_ = true;

  // True if playback should be started upon the next call to OnShown(). Only
  // used on Android.
  bool should_play_upon_shown_;

  bool should_pause_when_frame_is_hidden_ = false;

  WebMediaStream web_stream_;
  // IDs of the tracks currently played.
  WebString current_video_track_id_;
  WebString current_audio_track_id_;

  CreateSurfaceLayerBridgeCB create_bridge_callback_;

  // Resets the ForceBeginFrames flag once we stop receiving calls to
  // requestVideoFrameCallback().
  std::unique_ptr<TaskRunnerTimer<WebMediaPlayerMS>>
      stop_force_begin_frames_timer_;

  std::unique_ptr<WebVideoFrameSubmitter> submitter_;

  // Whether the use of a surface layer instead of a video layer is enabled.
  bool use_surface_layer_ = false;

  // Owns the weblayer and obtains/maintains SurfaceIds for
  // kUseSurfaceLayerForVideo feature.
  std::unique_ptr<WebSurfaceLayerBridge> bridge_;

  // Whether the video is known to be opaque or not.
  bool opaque_ = true;

  bool has_first_frame_ = false;

  // Monitors the duration of the media stream.
  std::unique_ptr<WatchTimeReporter> watch_time_reporter_;
  base::TimeDelta compositor_initial_time_;
  base::TimeDelta compositor_last_time_;
  base::TimeDelta audio_initial_time_;
  base::TimeDelta audio_last_time_;

  base::WeakPtr<WebMediaPlayerMS> weak_this_;
  base::WeakPtrFactory<WebMediaPlayerMS> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_MODULES_MEDIASTREAM_WEB_MEDIA_PLAYER_MS_H_
