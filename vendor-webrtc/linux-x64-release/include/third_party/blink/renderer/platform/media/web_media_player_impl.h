// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_MEDIA_PLAYER_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_MEDIA_PLAYER_IMPL_H_

#include <stdint.h>

#include <memory>
#include <optional>
#include <string>
#include <string_view>
#include <vector>

#include "base/cancelable_callback.h"
#include "base/compiler_specific.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/ref_counted.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "base/threading/thread.h"
#include "base/time/default_tick_clock.h"
#include "base/time/time.h"
#include "base/timer/elapsed_timer.h"
#include "base/timer/timer.h"
#include "build/build_config.h"
#include "media/base/cdm_config.h"
#include "media/base/data_source.h"
#include "media/base/demuxer.h"
#include "media/base/eme_constants.h"
#include "media/base/encryption_scheme.h"
#include "media/base/media_observer.h"
#include "media/base/media_tracks.h"
#include "media/base/overlay_info.h"
#include "media/base/picture_in_picture_events_info.h"
#include "media/base/pipeline_impl.h"
#include "media/base/renderer_factory_selector.h"
#include "media/base/routing_token_callback.h"
#include "media/base/simple_watch_timer.h"
#include "media/filters/demuxer_manager.h"
#include "media/mojo/mojom/media_metrics_provider.mojom.h"
#include "media/mojo/mojom/playback_events_recorder.mojom.h"
#include "media/renderers/paint_canvas_video_renderer.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "services/media_session/public/cpp/media_position.h"
#include "third_party/blink/public/platform/media/web_media_player_builder.h"
#include "third_party/blink/public/platform/media/web_media_player_delegate.h"
#include "third_party/blink/public/platform/web_audio_source_provider.h"
#include "third_party/blink/public/platform/web_content_decryption_module_result.h"
#include "third_party/blink/public/platform/web_media_player.h"
#include "third_party/blink/public/platform/web_surface_layer_bridge.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/bindings/v8_external_memory_accounter.h"
#include "third_party/blink/renderer/platform/media/learning_experiment_helper.h"
#include "third_party/blink/renderer/platform/media/media_player_client.h"
#include "third_party/blink/renderer/platform/media/multi_buffer_data_source.h"
#include "third_party/blink/renderer/platform/media/smoothness_helper.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "url/gurl.h"

namespace base {
class SingleThreadTaskRunner;
class TaskRunner;
}  // namespace base

namespace cc {
class VideoLayer;
}

namespace gfx {
class Size;
}

namespace media {
class CdmContextRef;
class ChunkDemuxer;
class Demuxer;
class MediaLog;
class MemoryDumpProviderProxy;
class PipelineController;
class SwitchableAudioRendererSink;

namespace learning {
class LearningTaskController;
}
}  // namespace media

namespace viz {
class RasterContextProvider;
}

namespace blink {
class BufferedDataSourceHostImpl;
class PowerStatusHelper;
class ThreadSafeBrowserInterfaceBrokerProxy;
class UrlIndex;
class VideoDecodeStatsReporter;
class VideoFrameCompositor;
class WatchTimeReporter;
class WebAudioSourceProviderImpl;
class WebContentDecryptionModule;
class WebLocalFrame;
class WebMediaPlayerEncryptedMediaClient;

// The set of split histograms that are supported. Keeping them in an enum
// helps prevent raw strings from being scattered throughout the source, and
// can provide a convenient way to remember to update the histograms.xml file
// when changes or additions are made.
enum class SplitHistogramName {
  kTimeToMetadata,
  kTimeToPlayReady,
  kUnderflowDuration2,
  kVideoHeightInitial,
  kTimeToFirstFrame,
};

// The canonical implementation of WebMediaPlayer that's backed by
// Pipeline. Handles normal resource loading, Media Source, and
// Encrypted Media.
class PLATFORM_EXPORT WebMediaPlayerImpl
    : public WebMediaPlayer,
      public WebMediaPlayerDelegate::Observer,
      public media::Pipeline::Client,
      public media::MediaObserverClient,
      public media::DemuxerManager::Client,
      public WebSurfaceLayerBridgeObserver,
      public SmoothnessHelper::Client {
 public:
  // Constructs a WebMediaPlayer implementation using Chromium's media stack.
  // |delegate| and |renderer_factory_selector| must not be null.
  WebMediaPlayerImpl(
      WebLocalFrame* frame,
      MediaPlayerClient* client,
      WebMediaPlayerEncryptedMediaClient* encrypted_client,
      WebMediaPlayerDelegate* delegate,
      std::unique_ptr<media::RendererFactorySelector> renderer_factory_selector,
      UrlIndex* url_index,
      std::unique_ptr<VideoFrameCompositor> compositor,
      std::unique_ptr<media::MediaLog> media_log,
      media::MediaPlayerLoggingID player_id,
      WebMediaPlayerBuilder::DeferLoadCB defer_load_cb,
      scoped_refptr<media::SwitchableAudioRendererSink> audio_renderer_sink,
      scoped_refptr<base::SequencedTaskRunner> media_task_runner,
      scoped_refptr<base::TaskRunner> worker_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner> compositor_task_runner,
      scoped_refptr<base::SingleThreadTaskRunner>
          video_frame_compositor_task_runner,
      WebContentDecryptionModule* initial_cdm,
      media::RequestRoutingTokenCallback request_routing_token_cb,
      base::WeakPtr<media::MediaObserver> media_observer,
      bool enable_instant_source_buffer_gc,
      bool embedded_media_experience_enabled,
      mojo::PendingRemote<media::mojom::MediaMetricsProvider> metrics_provider,
      CreateSurfaceLayerBridgeCB create_bridge_callback,
      scoped_refptr<viz::RasterContextProvider> raster_context_provider,
      bool use_surface_layer,
      bool is_background_suspend_enabled,
      bool is_background_video_play_enabled,
      bool is_background_video_track_optimization_supported,
      std::unique_ptr<media::Demuxer> demuxer_override,
      scoped_refptr<ThreadSafeBrowserInterfaceBrokerProxy> remote_interfaces);
  WebMediaPlayerImpl(const WebMediaPlayerImpl&) = delete;
  WebMediaPlayerImpl& operator=(const WebMediaPlayerImpl&) = delete;
  ~WebMediaPlayerImpl() override;

  // WebSurfaceLayerBridgeObserver implementation.
  void OnWebLayerUpdated() override;
  void RegisterContentsLayer(cc::Layer* layer) override;
  void UnregisterContentsLayer(cc::Layer* layer) override;
  void OnSurfaceIdUpdated(viz::SurfaceId surface_id) override;

  WebMediaPlayer::LoadTiming Load(LoadType load_type,
                                  const WebMediaPlayerSource& source,
                                  CorsMode cors_mode,
                                  bool is_cache_disabled) override;

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
  void OnTimeUpdate() override;
  bool SetSinkId(const WebString& sink_id,
                 WebSetSinkIdCompleteCallback completion_callback) override;
  void SetPoster(const WebURL& poster) override;
  void SetPreload(WebMediaPlayer::Preload preload) override;
  WebTimeRanges Buffered() const override;
  WebTimeRanges Seekable() const override;

  void OnFrozen() override;

  // paint() the current video frame into |canvas|. This is used to support
  // various APIs and functionalities, including but not limited to: <canvas>,
  // ImageBitmap, printing and capturing capabilities.
  void Paint(cc::PaintCanvas* canvas,
             const gfx::Rect& rect,
             cc::PaintFlags& flags) override;
  scoped_refptr<media::VideoFrame> GetCurrentFrameThenUpdate() override;
  std::optional<media::VideoFrame::ID> CurrentFrameId() const override;
  media::PaintCanvasVideoRenderer* GetPaintCanvasVideoRenderer() override;

  // True if the loaded media has a playable video/audio track.
  bool HasVideo() const override;
  bool HasAudio() const override;

  void EnabledAudioTracksChanged(
      const std::vector<WebMediaPlayer::TrackId>& enabled_track_ids) override;
  void SelectedVideoTrackChanged(
      std::optional<WebMediaPlayer::TrackId> selected_track_id) override;

  void OnEnabledAudioTracksChanged(std::vector<media::MediaTrack::Id>);
  void OnSelectedVideoTrackChanged(std::optional<media::MediaTrack::Id>);

  // Dimensions of the video.
  gfx::Size NaturalSize() const override;

  gfx::Size VisibleSize() const override;

  // Getters of playback state.
  bool Paused() const override;
  bool Seeking() const override;
  double Duration() const override;
  virtual double timelineOffset() const;
  bool IsEnded() const override;

  // Shared between the WebMediaPlayer and DemuxerManager::Client interfaces.
  double CurrentTime() const override;

  // Internal states of loading and network.
  // TODO(hclam): Ask the pipeline about the state rather than having reading
  // them from members which would cause race conditions.
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

  bool PassedTimingAllowOriginCheck() const override;

  void SetVolumeMultiplier(double multiplier) override;
  void SetPersistentState(bool persistent) override;
  void SetPowerExperimentState(bool state) override;
  void SuspendForFrameClosed() override;

  void SetShouldPauseWhenFrameIsHidden(
      bool should_pause_when_frame_is_hidden) override;
  bool GetShouldPauseWhenFrameIsHidden() override;

  bool HasAvailableVideoFrame() const override;
  bool HasReadableVideoFrame() const override;

  scoped_refptr<WebAudioSourceProviderImpl> GetAudioSourceProvider() override;

  void SetContentDecryptionModule(
      WebContentDecryptionModule* cdm,
      WebContentDecryptionModuleResult result) override;
  void SetRenderMutedAudio(bool render_muted_audio) override;

  void EnteredFullscreen() override;
  void ExitedFullscreen() override;
  void BecameDominantVisibleContent(bool is_dominant) override;
  void SetIsEffectivelyFullscreen(
      WebFullscreenVideoStatus fullscreen_video_status) override;
  void OnHasNativeControlsChanged(bool) override;
  void OnDisplayTypeChanged(WebMediaPlayer::DisplayType display_type) override;

  // WebMediaPlayerDelegate::Observer implementation.
  void OnPageHidden() override;
  void OnPageShown() override;
  void OnFrameHidden() override;
  void OnFrameShown() override;
  void OnIdleTimeout() override;

  void RequestRemotePlaybackDisabled(bool disabled) override;
  void RequestMediaRemoting() override;

#if BUILDFLAG(IS_ANDROID)
  // TODO(https://crbug.com/839651): Rename Flinging[Started/Stopped] to
  // RemotePlayback[Started/Stopped] once the other RemotePlayback methods have
  // been removed
  void FlingingStarted() override;
  void FlingingStopped() override;

  // Called when the play/pause state of media playing on a remote cast device
  // changes, and WMPI wasn't the originator of that change (e.g. a phone on the
  // same network paused the cast device via the casting notification).
  // This is only used by the FlingingRenderer/FlingingRendererClient, when we
  // are flinging media (a.k.a. RemotePlayback).
  // The consistency between the WMPI state and the cast device state is not
  // guaranteed, and it a best effort, which can always be fixed by the user by
  // tapping play/pause once. Attempts to enfore stronger consistency guarantees
  // have lead to unstable states, and a worse user experience.
  void OnRemotePlayStateChange(media::MediaStatus::State state);
#endif

  // media::MediaObserverClient implementation.
  void SwitchToRemoteRenderer(
      const std::string& remote_device_friendly_name) override;
  void SwitchToLocalRenderer(
      media::MediaObserverClient::ReasonToSwitchToLocal reason) override;
  void UpdateRemotePlaybackCompatibility(bool is_compatible) override;

  // Test helper methods for exercising media suspension. Once called, when
  // |target_state| is reached or exceeded the stale flag will be set when
  // computing the play state, which will trigger suspend if the player is
  // paused; see UpdatePlayState_ComputePlayState() for the exact details.
  void ForceStaleStateForTesting(ReadyState target_state) override;
  bool IsSuspendedForTesting() override;
  bool DidLazyLoad() const override;
  void OnBecameVisible() override;
  bool IsOpaque() const override;
  int GetPlayerId() override { return player_id_; }
  std::optional<viz::SurfaceId> GetSurfaceId() override;
  GURL GetSrcAfterRedirects() override;
  void RequestVideoFrameCallback() override;
  std::unique_ptr<WebMediaPlayer::VideoFramePresentationMetadata>
  GetVideoFramePresentationMetadata() override;
  void UpdateFrameIfStale() override;

  base::WeakPtr<WebMediaPlayer> AsWeakPtr() override;
  void RegisterFrameSinkHierarchy() override;
  void UnregisterFrameSinkHierarchy() override;

  void RecordVideoOcclusionState(std::string_view occlusion_state) override;

  void RecordAutoPictureInPictureInfo(
      const media::PictureInPictureEventsInfo::AutoPipInfo&
          auto_picture_in_picture_info) override;

  bool IsBackgroundMediaSuspendEnabled() const {
    return is_background_suspend_enabled_;
  }

  // Distinct states that |delegate_| can be in. (Public for testing.)
  enum class DelegateState {
    GONE,
    PLAYING,
    PAUSED,
  };

  // Playback state variables computed together in UpdatePlayState().
  // (Public for testing.)
  struct PlayState {
    DelegateState delegate_state;
    bool is_idle;
    bool is_memory_reporting_enabled;
    bool is_suspended;
  };

  // Allow background video tracks with ~5 second keyframes (rounding down) to
  // be disabled to save resources.
  constexpr static base::TimeDelta
      kMaxKeyframeDistanceToDisableBackgroundVideo = base::Milliseconds(5500);

 private:
  friend class WebMediaPlayerImplTest;
  friend class WebMediaPlayerImplBackgroundBehaviorTest;

  // Helper function that can be used for generating different UMA records:
  // |key| is the UMA prefix, such as "Media.TimeToPlayReady", for example.
  // |UmaFunction| is some UMA function, like base::UmaHistogramMediumTimes
  //               which the record actually gets logged with
  // |T...| are the arguments passed. Usually this is only one piece of data,
  //        such as a base::TimeDelta, in the case of UmaHistogramMediumTimes,
  //        but could also be a series of fields that customize bucket sizes
  //        or the like.
  //
  // Finally, the |Flags| template argument is used to determine which suffixes
  // are logged - An integer enum is provided |SplitHistogramTypes| which is
  // a bitmask, and can be used to require logging:
  //   PlaybackType: {".SRC", ".MSE", ".HLS" (in the future)} based on demuxer
  //                 type
  //   Encrypted:    {".EME"} based on the value of |is_encrypted_|
  //   All:          {".All"} all the time.
  // |Flags| is provided as a template argument instead of a function argument
  // in order to guard different components in "if constexpr" conditionals,
  // so we won't even compile in strings such as "Media.TimeToPlayReady.All"
  // if it's not specified.
  template <uint32_t Flags, typename... T>
  void WriteSplitHistogram(void (*UmaFunction)(std::string_view, T...),
                           SplitHistogramName key,
                           const T&... value);

  void EnableOverlay();
  void DisableOverlay();

  // Do we have overlay information?  For CVV, this is a surface id.  For
  // AndroidOverlay, this is the routing token.
  bool HaveOverlayInfo();

  // Send OverlayInfo to the decoder.
  //
  // If we've requested but not yet received the surface id or routing token, or
  // if there's no decoder-provided callback to send the overlay info, then this
  // call will do nothing.
  void MaybeSendOverlayInfoToDecoder();

  void OnPipelineStarted(media::PipelineStatus status);
  void OnPipelineSuspended();
  void OnBeforePipelineResume();
  void OnPipelineResumed();
  void OnPipelineSeeked(bool time_updated);

  // media::Pipeline::Client overrides.
  void OnError(media::PipelineStatus status) override;
  void OnFallback(media::PipelineStatus status) override;
  void OnEnded() override;
  void OnMetadata(const media::PipelineMetadata& metadata) override;
  void OnBufferingStateChange(
      media::BufferingState state,
      media::BufferingStateChangeReason reason) override;
  void OnDurationChange() override;
  void OnWaiting(media::WaitingReason reason) override;
  void OnAudioConfigChange(const media::AudioDecoderConfig& config) override;
  void OnVideoConfigChange(const media::VideoDecoderConfig& config) override;
  void OnVideoNaturalSizeChange(const gfx::Size& size) override;
  void OnVideoOpacityChange(bool opaque) override;
  void OnVideoFrameRateChange(std::optional<int> fps) override;
  void OnVideoAverageKeyframeDistanceUpdate() override;
  void OnAudioPipelineInfoChange(const media::AudioPipelineInfo& info) override;
  void OnVideoPipelineInfoChange(const media::VideoPipelineInfo& info) override;

  // media::DemuxerManager::Client overrides.
  void OnChunkDemuxerOpened(media::ChunkDemuxer* demuxer) override;
  void OnProgress() override;
  void OnEncryptedMediaInitData(media::EmeInitDataType init_data_type,
                                const std::vector<uint8_t>& init_data) override;
  void MakeDemuxerThreadDumper(media::Demuxer* demuxer) override;
  bool CouldPlayIfEnoughData() override;
  void StopForDemuxerReset() override;
  void RestartForHls() override;
  bool IsSecurityOriginCryptographic() const override;
  void UpdateLoadedUrl(const GURL& url) override;
  void DemuxerRequestsSeek(base::TimeDelta seek_time) override;

#if BUILDFLAG(ENABLE_FFMPEG) || BUILDFLAG(ENABLE_HLS_DEMUXER)
  void AddMediaTrack(const media::MediaTrack&) override;
  void RemoveMediaTrack(const media::MediaTrack&) override;
#endif  // BUILDFLAG(ENABLE_FFMPEG) || BUILDFLAG(ENABLE_HLS_DEMUXER)

#if BUILDFLAG(ENABLE_HLS_DEMUXER)
  void GetUrlData(const GURL& gurl,
                  bool ignore_cache,
                  base::OnceCallback<void(scoped_refptr<UrlData>)> cb);
  base::SequenceBound<media::HlsDataSourceProvider> GetHlsDataSourceProvider()
      override;
#endif  // BUILDFLAG(ENABLE_HLS_DEMUXER)

  // Simplified watch time reporting.
  void OnSimpleWatchTimerTick();

  // Actually seek. Avoids causing |should_notify_time_changed_| to be set when
  // |time_updated| is false.
  void DoSeek(base::TimeDelta time, bool time_updated);

  // Called after |defer_load_cb_| has decided to allow the load. If
  // |defer_load_cb_| is null this is called immediately.
  void DoLoad(LoadType load_type,
              const KURL& url,
              CorsMode cors_mode,
              bool is_cache_disabled);

  // Called after synchronous initialization of a data source completes.
  void DataSourceInitialized(bool success);

  // Called after asynchronous initialization of a multibuffer data source
  // completes.
  void MultiBufferDataSourceInitialized(bool success);

  // Called after synchronous or asynchronous MemoryDataSource initialization.
  void MemoryDataSourceInitialized(bool success, size_t data_size);

  // Called if the |MultiBufferDataSource| is redirected.
  void OnDataSourceRedirected();

  // Called when the data source is downloading or paused.
  void NotifyDownloading(bool is_downloading);

  // Called by RenderFrameImpl with the overlay routing token, if we request it.
  void OnOverlayRoutingToken(const base::UnguessableToken& token);

  // Called by GpuVideoDecoder on Android to request a surface to render to (if
  // necessary).
  void OnOverlayInfoRequested(
      bool decoder_requires_restart_for_overlay,
      media::ProvideOverlayInfoCB provide_overlay_info_cb);

  // Creates a Renderer via the |renderer_factory_selector_|. If the
  // |renderer_type| is std::nullopt, create the base Renderer. Otherwise, set
  // the base type to be |renderer_type| and create a Renderer of that type.
  std::unique_ptr<media::Renderer> CreateRenderer(
      std::optional<media::RendererType> renderer_type);

  // Finishes starting the pipeline due to a call to load().
  void StartPipeline();

  // Restart the player/pipeline as soon as possible. This will destroy the
  // current renderer, if any, and create a new one via the RendererFactory; and
  // then seek to resume playback at the current position.
  void ScheduleRestart();

  // Helpers that set the network/ready state and notifies the client if
  // they've changed.
  void SetNetworkState(WebMediaPlayer::NetworkState state);
  void SetReadyState(WebMediaPlayer::ReadyState state);

  // Returns the current video frame from |compositor_|, and asks the compositor
  // to update its frame if it is stale.
  // Can return a nullptr.
  scoped_refptr<media::VideoFrame> GetCurrentFrameFromCompositor() const;

  // Sets CdmContext from |cdm| on the pipeline and calls OnCdmAttached()
  // when done.
  void SetCdmInternal(WebContentDecryptionModule* cdm);

  // Called when a CDM has been attached to the |pipeline_|.
  void OnCdmAttached(bool success);

  // Inspects the current playback state and:
  //   - notifies |delegate_|,
  //   - toggles the memory usage reporting timer, and
  //   - toggles suspend/resume as necessary.
  //
  // This method should be called any time its dependent values change. These
  // are:
  //   - is_flinging_,
  //   - hasVideo(),
  //   - delegate_->IsHidden(),
  //   - network_state_, ready_state_,
  //   - is_idle_, must_suspend_,
  //   - paused_, ended_,
  //   - pending_suspend_resume_cycle_,
  //   - enter_pip_callback_,
  void UpdatePlayState();

  // Methods internal to UpdatePlayState().
  PlayState UpdatePlayState_ComputePlayState(bool is_flinging,
                                             bool can_auto_suspend,
                                             bool is_suspended,
                                             bool is_backgrounded,
                                             bool is_in_picture_in_picture);
  void SetDelegateState(DelegateState new_state, bool is_idle);
  void SetMemoryReportingState(bool is_memory_reporting_enabled);
  void SetSuspendState(bool is_suspended);

  // Called at low frequency to tell external observers how much memory we're
  // using for video playback.  Called by |memory_usage_reporting_timer_|.
  // Memory usage reporting is done in two steps, because |demuxer_| must be
  // accessed on the media thread.
  void ReportMemoryUsage();
  void FinishMemoryUsageReport(int64_t demuxer_memory_usage);

  void OnMainThreadMemoryDump(int32_t id,
                              const base::trace_event::MemoryDumpArgs& args,
                              base::trace_event::ProcessMemoryDump* pmd);
  static void OnMediaThreadMemoryDump(
      int32_t id,
      media::Demuxer* demuxer,
      const base::trace_event::MemoryDumpArgs& args,
      base::trace_event::ProcessMemoryDump* pmd);

  // Called during OnHidden() when we want a suspended player to enter the
  // paused state after some idle timeout.
  void ScheduleIdlePauseTimer();

  // Returns |true| before HaveFutureData whenever there has been loading
  // progress and we have not been resumed for at least kLoadingToIdleTimeout
  // since then.
  //
  // This is used to delay suspension long enough for preroll to complete, which
  // is necessay because play() will not be called before HaveFutureData (and
  // thus we think we are idle forever).
  bool IsPrerollAttemptNeeded();

  void CreateWatchTimeReporter();
  void UpdateSecondaryProperties();

  void CreateVideoDecodeStatsReporter();

  // Returns true if the player's hosting page (WebView) is hidden or closed.
  bool IsPageHidden() const;

  // Returns true if the player's host frame is hidden or closed in the host
  // page.
  bool IsFrameHidden() const;

  bool IsPausedBecausePageHidden() const;
  bool IsPausedBecauseFrameHidden() const;

  bool ShouldResetVisibilityPauseReason(PauseReason pause_reason) const;

  // Returns true if the player is in streaming mode, meaning that the source
  // or the demuxer doesn't support timeline or seeking.
  bool IsStreaming() const;

  // Return whether |pipeline_metadata_| is compatible with an overlay. This
  // is intended for android.
  bool DoesOverlaySupportMetadata() const;

  // Whether the playback should be paused when hidden. Uses metadata so has
  // meaning only after the pipeline has started, otherwise returns false.
  // Doesn't check if the playback can actually be paused depending on the
  // pipeline's state.
  bool ShouldPausePlaybackWhenHidden() const;

  // Whether the video track should be disabled when hidden. Uses metadata so
  // has meaning only after the pipeline has started, otherwise returns false.
  // Doesn't check if the video track can actually be disabled depending on the
  // pipeline's state.
  bool ShouldDisableVideoWhenHidden() const;

  // If enabling or disabling background video optimization has been delayed,
  // because of the pipeline not running, seeking or resuming, this method
  // needs to be called to update the optimization state.
  void UpdateBackgroundVideoOptimizationState();

  // Pauses a hidden video only player to save power if possible.
  // Must be called when either of the following happens:
  // - right after the video was hidden,
  // - right after the pipeline has resumed if the video is hidden.
  void PauseVideoIfNeeded(PauseReason pause_reason);

  // Disables the video track to save power if possible.
  // Must be called when either of the following happens:
  // - right after the video was hidden,
  // - right after the pipeline has started (|seeking_| is used to detect the
  //   when pipeline started) if the video is hidden,
  // - right after the pipeline has resumed if the video is hidden.
  void DisableVideoTrackIfNeeded();

  // Enables the video track if it was disabled before to save power.
  // Must be called when either of the following happens:
  // - right after the video was shown,
  // - right before the pipeline is requested to resume
  //   (see https://crbug.com/678374),
  // - right after the pipeline has resumed if the video is not hidden.
  void EnableVideoTrackIfNeeded();

  // Overrides the pipeline statistics returned by GetPiplineStatistics() for
  // tests.
  void SetPipelineStatisticsForTest(const media::PipelineStatistics& stats);

  // Returns the pipeline statistics or the value overridden by tests.
  media::PipelineStatistics GetPipelineStatistics() const;

  // Overrides the pipeline media duration returned by
  // GetPipelineMediaDuration() for tests.
  void SetPipelineMediaDurationForTest(base::TimeDelta duration);

  // Returns the pipeline media duration or the value overridden by tests.
  base::TimeDelta GetPipelineMediaDuration() const;

  media::MediaContentType GetMediaContentType() const;

  // Records |duration| to the appropriate metric based on whether we're
  // handling a src= or MSE based playback.
  void RecordUnderflowDuration(base::TimeDelta duration);

  // Returns true when we estimate that we can play the rest of the media
  // without buffering.
  bool CanPlayThrough();

  // Internal implementation of Pipeline::Client::OnBufferingStateChange(). When
  // |for_suspended_start| is true, the given state will be set even if the
  // pipeline is not currently stable.
  void OnBufferingStateChangeInternal(media::BufferingState state,
                                      media::BufferingStateChangeReason reason,
                                      bool for_suspended_start = false);

  // Records |natural_size| to MediaLog and video height to UMA.
  void RecordVideoNaturalSize(const gfx::Size& natural_size);

  void SetTickClockForTest(const base::TickClock* tick_clock);

  // Returns the current time without clamping to Duration() as required by
  // HTMLMediaElement for handling ended. This method will never return a
  // negative or kInfiniteDuration value. See http://crbug.com/409280,
  // http://crbug.com/645998, and http://crbug.com/751823 for reasons why.
  base::TimeDelta GetCurrentTimeInternal() const;

  // Called by the compositor the very first time a frame is received.
  void OnFirstFrame(base::TimeTicks frame_time, bool is_frame_readable);

  // Records the encryption scheme used by the stream |stream_name|. This is
  // only recorded when metadata is available.
  void RecordEncryptionScheme(const std::string& stream_name,
                              media::EncryptionScheme encryption_scheme);

  // Returns whether the player is currently displayed in video
  // Picture-in-Picture. It will return true even if the player is in AutoPIP
  // (Android) mode. The player MUST have a `client_` when this call happen.
  bool IsInVideoPictureInPicture() const;

  // Sets the UKM container name if needed.
  void MaybeSetContainerNameForMetrics();

  // Switch to SurfaceLayer, either initially or from VideoLayer.
  void ActivateSurfaceLayerForVideo();

  // Called by |compositor_| upon presenting a frame, after
  // RequestAnimationFrame() is called.
  void OnNewFramePresentedCallback();

  // Notifies |demuxer_manager_| of playback and rate changes which may increase
  // the amount of data the DataSource buffers. Does nothing prior to reaching
  // kReadyStateHaveEnoughData for the first time.
  void MaybeUpdateBufferSizesForPlayback();

  // Create / recreate |smoothness_helper_|, with current features.  Will take
  // no action if we already have a smoothness helper with the same features
  // that we want now.  Will destroy the helper if we shouldn't be measuring
  // smoothness right now.
  void UpdateSmoothnessHelper();

  // Get the LearningTaskController for |task_name|.
  std::unique_ptr<media::learning::LearningTaskController>
  GetLearningTaskController(const char* task_name);

  // Returns whether the player has an audio track and whether it should be
  // allowed to play it.
  bool HasUnmutedAudio() const;

  // Returns true if the video frame from this player are being captured.
  bool IsVideoBeingCaptured() const;

  // Report UMAs when this object instance is destroyed.
  void ReportSessionUMAs() const;

  std::optional<media::DemuxerType> GetDemuxerType() const;

  // Useful to bind for a cb to be called when a demuxer is created.
  media::PipelineStatus OnDemuxerCreated(media::Demuxer* demuxer,
                                         media::Pipeline::StartType start_type,
                                         bool is_streaming,
                                         bool is_static);

  // Notifies the `client_` and the `delegate_` about metadata change.
  void DidMediaMetadataChange();

  const raw_ptr<WebLocalFrame> frame_;

  WebMediaPlayer::NetworkState network_state_ =
      WebMediaPlayer::kNetworkStateEmpty;
  WebMediaPlayer::ReadyState ready_state_ =
      WebMediaPlayer::kReadyStateHaveNothing;
  WebMediaPlayer::ReadyState highest_ready_state_ =
      WebMediaPlayer::kReadyStateHaveNothing;

  // Preload state for when a DataSource is created after setPreload().
  media::DataSource::Preload preload_ = media::DataSource::METADATA;

  // Poster state (for UMA reporting).
  bool has_poster_ = false;

  // Task runner for posting tasks on Chrome's main thread. Also used
  // for DCHECKs so methods calls won't execute in the wrong thread.
  const scoped_refptr<base::SingleThreadTaskRunner> main_task_runner_;
  const scoped_refptr<base::SequencedTaskRunner> media_task_runner_;
  const scoped_refptr<base::TaskRunner> worker_task_runner_;

  // This is the ID that is used within the internals of the media element
  // primarily for correlating logs.
  const media::MediaPlayerLoggingID media_player_id_;

  std::unique_ptr<media::MediaLog> media_log_;

  // |pipeline_controller_| owns an instance of Pipeline.
  std::unique_ptr<media::PipelineController> pipeline_controller_;

  // The LoadType passed in the |load_type| parameter of the load() call.
  LoadType load_type_ = kLoadTypeURL;

  // Cache of metadata for answering hasAudio(), hasVideo(), and naturalSize().
  media::PipelineMetadata pipeline_metadata_;

  // Whether the video is known to be opaque or not.
  bool opaque_ = false;

  // Playback state.
  //
  // TODO(scherkus): we have these because Pipeline favours the simplicity of a
  // single "playback rate" over worrying about paused/stopped etc...  It forces
  // all clients to manage the pause+playback rate externally, but is that
  // really a bad thing?
  //
  // TODO(scherkus): since SetPlaybackRate(0) is asynchronous and we don't want
  // to hang the render thread during pause(), we record the time at the same
  // time we pause and then return that value in currentTime().  Otherwise our
  // clock can creep forward a little bit while the asynchronous
  // SetPlaybackRate(0) is being executed.
  double playback_rate_ = 0.0;

  // Counter that limits spam to |media_log_| of |playback_rate_| changes.
  int num_playback_rate_logs_ = 0;

  // Set while paused. |paused_time_| is only valid when |paused_| is true.
  bool paused_ = true;
  base::TimeDelta paused_time_;

  // Set if paused automatically when hidden. Reset if paused for any other
  // reason. If set to PauseReason::kPageHidden, playback should be resumed when
  // the page becomes visible.
  std::optional<PauseReason> visibility_pause_reason_;

  // Set when starting, seeking, and resuming (all of which require a Pipeline
  // seek). |seek_time_| is only valid when |seeking_| is true.
  bool seeking_ = false;
  base::TimeDelta seek_time_;

  // Set when doing a restart (a suspend and resume in sequence) of the pipeline
  // in order to destruct and reinitialize the decoders. This is separate from
  // |pending_resume_| and |pending_suspend_| because they can be elided in
  // certain cases, whereas for a restart they must happen.
  // TODO(sandersd,watk): Create a simpler interface for a pipeline restart.
  bool pending_suspend_resume_cycle_ = false;

  // TODO(scherkus): Replace with an explicit ended signal to HTMLMediaElement,
  // see http://crbug.com/409280
  bool ended_ = false;

  // Tracks whether to issue time changed notifications during buffering state
  // changes.
  bool should_notify_time_changed_ = false;

  bool overlay_enabled_ = false;

  // Cors and Caching flags set during `Load` and used while creating demuxers.
  CorsMode cors_mode_ = kCorsModeUnspecified;
  bool is_cache_disabled_ = false;

  // Whether the current decoder requires a restart on overlay transitions.
  bool decoder_requires_restart_for_overlay_ = false;

  const raw_ptr<MediaPlayerClient> client_;
  const raw_ptr<WebMediaPlayerEncryptedMediaClient> encrypted_client_;

  // WebMediaPlayer notifies the |delegate_| of playback state changes using
  // |delegate_id_|; an id provided after registering with the delegate.  The
  // WebMediaPlayer may also receive directives (play, pause) from the delegate
  // via the WebMediaPlayerDelegate::Observer interface after registration.
  //
  // NOTE: HTMLMediaElement is a ExecutionContextLifecycleObserver, and
  // will receive a call to contextDestroyed() when Document::shutdown()
  // is called. Document::shutdown() is called before the frame detaches (and
  // before the frame is destroyed). RenderFrameImpl owns |delegate_| and is
  // guaranteed to outlive |this|; thus it is safe to store |delegate_| as a raw
  // pointer.
  raw_ptr<WebMediaPlayerDelegate> delegate_;
  int delegate_id_ = 0;

  // The playback state last reported to |delegate_|, to avoid setting duplicate
  // states.
  // TODO(sandersd): The delegate should be implementing deduplication.
  DelegateState delegate_state_ = DelegateState::GONE;
  bool delegate_has_audio_ = false;

  const int player_id_;

  WebMediaPlayerBuilder::DeferLoadCB defer_load_cb_;

  // Members for notifying upstream clients about internal memory usage.  The
  // |adjust_allocated_memory_cb_| must only be called on |main_task_runner_|.
  base::RepeatingTimer memory_usage_reporting_timer_;
  raw_ptr<v8::Isolate> isolate_;
  NO_UNIQUE_ADDRESS V8ExternalMemoryAccounterBase external_memory_accounter_;
  int64_t last_reported_memory_usage_ = 0;
  std::unique_ptr<media::MemoryDumpProviderProxy> main_thread_mem_dumper_;
  std::unique_ptr<media::MemoryDumpProviderProxy> media_thread_mem_dumper_;

  // Routes audio playback to either AudioRendererSink or WebAudio.
  scoped_refptr<WebAudioSourceProviderImpl> audio_source_provider_;

  // Manages the lifetime of the DataSource, and soon the Demuxer.
  std::unique_ptr<media::DemuxerManager> demuxer_manager_;

  raw_ptr<const base::TickClock> tick_clock_ = nullptr;

  std::unique_ptr<BufferedDataSourceHostImpl> buffered_data_source_host_;
  const raw_ptr<UrlIndex> url_index_;
  scoped_refptr<viz::RasterContextProvider> raster_context_provider_;

  // Video rendering members.
  // The |compositor_| runs on the compositor thread, or if
  // kEnableSurfaceLayerForVideo is enabled, the media thread. This task runner
  // posts tasks for the |compositor_| on the correct thread.
  scoped_refptr<base::SingleThreadTaskRunner> vfc_task_runner_;
  std::unique_ptr<VideoFrameCompositor>
      compositor_;  // Deleted on |vfc_task_runner_|.
  media::PaintCanvasVideoRenderer video_renderer_;

  // The compositor layer for displaying the video content when using composited
  // playback.
  scoped_refptr<cc::VideoLayer> video_layer_;

  std::unique_ptr<WebContentDecryptionModuleResult> set_cdm_result_;

  // If a CdmContext is attached keep a reference to the CdmContextRef, so that
  // it is not destroyed until after the pipeline is done with it.
  std::unique_ptr<media::CdmContextRef> cdm_context_ref_;

  // Keep track of the CdmContextRef while it is in the process of attaching to
  // the pipeline.
  std::unique_ptr<media::CdmContextRef> pending_cdm_context_ref_;

  // True when encryption is detected, either by demuxer or by presence of a
  // ContentDecryptionModule (CDM).
  bool is_encrypted_ = false;

  // Captured once the cdm is provided to SetCdmInternal(). Used in creation of
  // |video_decode_stats_reporter_|.
  std::optional<media::CdmConfig> cdm_config_;

  // Whether the EME playback has been blocked waiting for the CDM to be set
  // or waiting for decryption key.
  bool has_waiting_for_key_ = false;

  // Tracks if we are currently flinging a video (e.g. in a RemotePlayback
  // session). Used to prevent videos from being paused when hidden.
  // TODO(https://crbug.com/839651): remove or rename this flag, when removing
  // IsRemote().
  bool is_flinging_ = false;

  // Tracks if we are currently using a remote renderer. See
  // SwitchToRemoteRenderer().
  bool is_remote_rendering_ = false;

  // The last volume received by setVolume() and the last volume multiplier from
  // SetVolumeMultiplier().  The multiplier is typical 1.0, but may be less
  // if the WebMediaPlayerDelegate has requested a volume reduction (ducking)
  // for a transient sound.  Playout volume is derived by volume * multiplier.
  double volume_ = 1.0;
  double volume_multiplier_ = 1.0;

  std::unique_ptr<media::RendererFactorySelector> renderer_factory_selector_;

  // For canceling AndroidOverlay routing token requests.
  base::CancelableOnceCallback<void(const base::UnguessableToken&)>
      token_available_cb_;

  // If overlay info is requested before we have it, then the request is saved
  // and satisfied once the overlay info is available. If the decoder does not
  // require restart to change surfaces, this is callback is kept until cleared
  // by the decoder.
  media::ProvideOverlayInfoCB provide_overlay_info_cb_;

  // On Android an overlay surface means using
  // SurfaceView instead of SurfaceTexture.

  // Allow overlays for all video on android.
  bool always_enable_overlays_ = false;

  // Suppresses calls to OnPipelineError() after destruction / shutdown has been
  // started; prevents us from spuriously logging errors that are transient or
  // unimportant.
  bool suppress_destruction_errors_ = false;

  // Stores the current position state of the media.
  media_session::MediaPosition media_position_state_;

  // Called sometime after the media is suspended in a playing state in
  // OnFrameHidden(), causing the state to change to paused.
  base::OneShotTimer background_pause_timer_;

  // Monitors the watch time of the played content.
  std::unique_ptr<WatchTimeReporter> watch_time_reporter_;
  media::AudioDecoderType audio_decoder_type_ =
      media::AudioDecoderType::kUnknown;
  media::VideoDecoderType video_decoder_type_ =
      media::VideoDecoderType::kUnknown;

  // The time at which DoLoad() is executed.
  base::TimeTicks load_start_time_;

  // Time elapsed time from |load_start_time_| to OnMetadata(). Used to later
  // adjust |load_start_time_| if a suspended startup occurred.
  base::TimeDelta time_to_metadata_;
  bool skip_metrics_due_to_startup_suspend_ = false;

  bool have_reported_time_to_play_ready_ = false;

  // Records pipeline statistics for describing media capabilities.
  std::unique_ptr<VideoDecodeStatsReporter> video_decode_stats_reporter_;

  // Elapsed time since we've last reached BUFFERING_HAVE_NOTHING.
  std::unique_ptr<base::ElapsedTimer> underflow_timer_;

  // Used to track loading progress, used by IsPrerollAttemptNeeded().
  // |preroll_attempt_pending_| indicates that the clock has been reset
  // (awaiting a resume to start), while |preroll_attempt_start_time_| tracks
  // when a preroll attempt began.
  bool preroll_attempt_pending_ = false;
  base::TimeTicks preroll_attempt_start_time_;

  // Monitors the player events.
  base::WeakPtr<media::MediaObserver> observer_;

  // Owns the weblayer and obtains/maintains SurfaceIds.
  std::unique_ptr<WebSurfaceLayerBridge> bridge_;

  // The maximum video keyframe distance that allows triggering background
  // playback optimizations (non-MSE).
  base::TimeDelta max_keyframe_distance_to_disable_background_video_;

  // The maximum video keyframe distance that allows triggering background
  // playback optimizations (MSE).
  base::TimeDelta max_keyframe_distance_to_disable_background_video_mse_;

  // Whether disabled the video track as an optimization.
  bool video_track_disabled_ = false;

  // Whether the pipeline is being resumed at the moment.
  bool is_pipeline_resuming_ = false;

  // When this is true, pipeline will not be auto suspended.
  bool disable_pipeline_auto_suspend_ = false;

  // Pipeline statistics overridden by tests.
  std::optional<media::PipelineStatistics> pipeline_statistics_for_test_;

  // Pipeline media duration overridden by tests.
  std::optional<base::TimeDelta> pipeline_media_duration_for_test_;

  // Whether the video requires a user gesture to resume after it was paused in
  // the background. Affects the value of ShouldPausePlaybackWhenHidden().
  bool video_locked_when_paused_when_hidden_ = false;

  // Whether embedded media experience is currently enabled.
  bool embedded_media_experience_enabled_ = false;

  // When should we use SurfaceLayer for video?
  bool use_surface_layer_ = false;

  // Whether surface layer is currently in use to display frames.
  bool surface_layer_for_video_enabled_ = false;

  CreateSurfaceLayerBridgeCB create_bridge_callback_;

  bool initial_video_height_recorded_ = false;

  enum class OverlayMode {
    // All overlays are turned off.
    kNoOverlays,

    // Use AndroidOverlay for overlays.
    kUseAndroidOverlay,
  };

  OverlayMode overlay_mode_ = OverlayMode::kNoOverlays;

  // Optional callback to request the routing token for AndroidOverlay.
  media::RequestRoutingTokenCallback request_routing_token_cb_;

  // If |overlay_routing_token_is_pending_| is false, then
  // |overlay_routing_token_| contains the routing token we should send, if any.
  // Otherwise, |overlay_routing_token_| is undefined.  We set the flag while
  // we have a request for the token that hasn't been answered yet; i.e., it
  // means that we don't know what, if any, token we should be using.
  bool overlay_routing_token_is_pending_ = false;
  media::OverlayInfo::RoutingToken overlay_routing_token_;

  media::OverlayInfo overlay_info_;

  base::CancelableOnceClosure update_background_status_cb_;

  // We cannot use `update_background_status_cb_.IsCancelled()` as that changes
  // when the callback is run, even if not explicitly cancelled. This is
  // initialized to true to keep in line with the existing behavior of
  // base::CancellableOnceClosure.
  bool is_background_status_change_cancelled_ = true;

  mojo::Remote<media::mojom::MediaMetricsProvider> media_metrics_provider_;
  mojo::Remote<media::mojom::PlaybackEventsRecorder> playback_events_recorder_;

  std::optional<ReadyState> stale_state_override_for_testing_;

  // True if we attempt to start the media pipeline in a suspended state for
  // preload=metadata. Cleared upon pipeline startup.
  bool attempting_suspended_start_ = false;

  // True if a frame has ever been rendered.
  bool has_first_frame_ = false;

  // True if we have not yet rendered a first frame, but one is needed. Set back
  // to false as soon as |has_first_frame_| is set to true.
  bool needs_first_frame_ = false;

  // Whether the rendered frame is readable, e.g. can be converted to image.
  bool is_frame_readable_ = false;

  // True if StartPipeline() completed a lazy load startup.
  bool did_lazy_load_ = false;

  // Whether the renderer should automatically suspend media playback in
  // background tabs.
  bool is_background_suspend_enabled_ = false;

  // If disabled, video will be auto paused when in background. Affects the
  // value of ShouldPausePlaybackWhenHidden().
  bool is_background_video_playback_enabled_ = true;

  // Whether background video optimization is supported on current platform.
  bool is_background_video_track_optimization_supported_ = true;

  const bool should_pause_background_muted_audio_;

  bool was_suspended_for_frame_closed_ = false;

  // Request pipeline to suspend. It should not block other signals after
  // suspended.
  bool pending_oneshot_suspend_ = false;

  bool should_pause_when_frame_is_hidden_ = false;

  base::CancelableOnceClosure have_enough_after_lazy_load_cb_;

  media::RendererType renderer_type_ = media::RendererType::kRendererImpl;
  media::SimpleWatchTimer simple_watch_timer_;

  LearningExperimentHelper will_play_helper_;

  std::unique_ptr<PowerStatusHelper> power_status_helper_;

  // Created while playing, deleted otherwise.
  std::unique_ptr<SmoothnessHelper> smoothness_helper_;
  std::optional<int> last_reported_fps_;

  // Time of the last call to GetCurrentFrameFromCompositor(). Used to prevent
  // background optimizations from being applied when capturing is active.
  base::TimeTicks last_frame_request_time_;

  // Count the number of times a video frame is being readback.
  unsigned video_frame_readback_count_ = 0;

  base::WeakPtr<WebMediaPlayerImpl> weak_this_;
  base::WeakPtrFactory<WebMediaPlayerImpl> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WEB_MEDIA_PLAYER_IMPL_H_
