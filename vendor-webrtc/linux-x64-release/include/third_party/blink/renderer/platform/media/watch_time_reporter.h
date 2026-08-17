// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WATCH_TIME_REPORTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WATCH_TIME_REPORTER_H_

#include <vector>

#include "base/functional/callback.h"
#include "base/power_monitor/power_observer.h"
#include "base/task/sequenced_task_runner.h"
#include "base/time/time.h"
#include "base/timer/timer.h"
#include "media/base/audio_codecs.h"
#include "media/base/media_log.h"
#include "media/base/timestamp_constants.h"
#include "media/base/video_codecs.h"
#include "media/mojo/mojom/media_metrics_provider.mojom.h"
#include "media/mojo/mojom/watch_time_recorder.mojom.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/platform/web_media_player.h"
#include "third_party/blink/renderer/platform/allow_discouraged_type.h"
#include "third_party/blink/renderer/platform/media/watch_time_component.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/size.h"
#include "url/origin.h"

namespace blink {

// Class for monitoring and reporting watch time in response to various state
// changes during the playback of media. We record metrics for audio only
// playbacks as well as video only or audio+video playbacks of sufficient size.
//
// Watch time for our purposes is defined as the amount of elapsed media time.
// Any amount of elapsed time is reported to the WatchTimeRecorder, but only
// amounts above limits::kMinimumElapsedWatchTimeSecs are reported to UMA. Watch
// time is checked every 5 seconds from then on and reported to multiple
// buckets: All, MSE, SRC, EME, AC, and battery.
//
// Either of paused or muted is sufficient to stop watch time metric reports.
// Each of these has a hysteresis where if the state change is undone within 5
// seconds, the watch time will be counted as uninterrupted.
//
// There are both foreground and background buckets for watch time. E.g., when
// media goes into the background foreground collection stops and background
// collection starts. As with other events, there is hysteresis on change
// between the foreground and background.
//
// Similarly, there are both muted and unmuted buckets for watch time. E.g., if
// a playback is muted the unmuted collection stops and muted collection starts.
// As with other events, there is hysteresis between mute and unmute.
//
// Power events (on/off battery power), native controls changes, or display type
// changes have a similar hysteresis, but unlike the aforementioned properties,
// will not stop metric collection.
//
// Each seek event will result in a new watch time metric being started and the
// old metric finalized as accurately as possible.
class PLATFORM_EXPORT WatchTimeReporter : base::PowerStateObserver {
 public:
  using GetMediaTimeCB = base::RepeatingCallback<base::TimeDelta(void)>;
  using GetPipelineStatsCB =
      base::RepeatingCallback<media::PipelineStatistics(void)>;

  // Constructor for the reporter; all requested metadata should be fully known
  // before attempting construction as incorrect values will result in the wrong
  // watch time metrics being reported.
  //
  // |properties| Properties describing the playback; these are considered
  // immutable over the lifetime of the reporter. If any of them change a new
  // WatchTimeReporter should be created with updated properties.
  //
  // |get_media_time_cb| must return the current playback time in terms of media
  // time, not wall clock time! Using media time instead of wall clock time
  // allows us to avoid a whole class of issues around clock changes during
  // suspend and resume.
  //
  // |provider| A provider of mojom::WatchTimeRecorder instances which will be
  // created and used to handle caching of metrics outside of the current
  // process.
  //
  // TODO(dalecurtis): Should we only report when rate == 1.0? Should we scale
  // the elapsed media time instead?
  WatchTimeReporter(media::mojom::PlaybackPropertiesPtr properties,
                    const gfx::Size& natural_size,
                    GetMediaTimeCB get_media_time_cb,
                    GetPipelineStatsCB get_pipeline_stats_cb,
                    media::mojom::MediaMetricsProvider* provider,
                    scoped_refptr<base::SequencedTaskRunner> task_runner,
                    const base::TickClock* tick_clock = nullptr);
  WatchTimeReporter(const WatchTimeReporter&) = delete;
  WatchTimeReporter& operator=(const WatchTimeReporter&) = delete;
  ~WatchTimeReporter() override;

  // These methods are used to ensure that watch time is only reported for media
  // that is actually playing. They should be called whenever the media starts
  // or stops playing for any reason. If the media is currently hidden,
  // OnPlaying() will start background watch time reporting.
  void OnPlaying();
  void OnPaused();

  // This will immediately finalize any outstanding watch time reports and stop
  // the reporting timer. Clients should call OnPlaying() upon seek completion
  // to restart the reporting timer.
  void OnSeeking();

  // This method is used to ensure that watch time is only reported for media
  // that is actually audible to the user. It should be called whenever the
  // volume changes.
  //
  // Note: This does not catch all cases. E.g., headphones that are not being
  // listened too, or even OS level volume state.
  void OnVolumeChange(double volume);

  // These methods are used to ensure that watch time is only reported for media
  // that is actually visible to the user. They should be called when the media
  // is shown or hidden respectively. OnHidden() will start background watch
  // time reporting.
  void OnShown();
  void OnHidden();

  // Called when a playback ends in error.
  void OnError(media::PipelineStatus status);

  // Indicates a rebuffering event occurred during playback. When watch time is
  // finalized the total watch time for a given category will be divided by the
  // number of rebuffering events. Reset to zero after a finalize event.
  void OnUnderflow();
  void OnUnderflowComplete(base::TimeDelta elapsed);

  // These methods are used to ensure that the watch time is reported relative
  // to whether the media is using native controls.
  void OnNativeControlsEnabled();
  void OnNativeControlsDisabled();

  // These methods are used to ensure that the watch time is reported relative
  // to the display type of the media.
  void OnDisplayTypeInline();
  void OnDisplayTypeFullscreen();
  void OnDisplayTypeVideoPictureInPicture();
  void OnDisplayTypeDocumentPictureInPicture();

  // Mutates various properties that may change over the lifetime of a playback
  // but for which we don't want to interrupt reporting for. UMA watch time will
  // not be interrupted by changes to these properties, while UKM will.
  //
  // Note: Both UMA and UMK watch time will be interrupted if the natural size
  // transitions above/below kMinimumVideoSize.
  void UpdateSecondaryProperties(
      media::mojom::SecondaryPlaybackPropertiesPtr secondary_properties);

  // Notifies the autoplay status of the playback. Must not be called multiple
  // times with different values.
  void SetAutoplayInitiated(bool autoplay_initiated);

  // Updates the duration maintained by the recorder. May be called any number
  // of times during playback.
  void OnDurationChanged(base::TimeDelta duration);

 private:
  friend class WatchTimeReporterTest;

  // Internal constructor for marking background status.
  WatchTimeReporter(media::mojom::PlaybackPropertiesPtr properties,
                    bool is_background,
                    bool is_muted,
                    const gfx::Size& natural_size,
                    GetMediaTimeCB get_media_time_cb,
                    GetPipelineStatsCB get_pipeline_stats_cb,
                    media::mojom::MediaMetricsProvider* provider,
                    scoped_refptr<base::SequencedTaskRunner> task_runner,
                    const base::TickClock* tick_clock);

  // base::PowerStateObserver implementation.
  //
  // We only observe power source changes. We don't need to observe suspend and
  // resume events because we report watch time in terms of elapsed media time
  // and not in terms of elapsed real time.
  void OnBatteryPowerStatusChange(base::PowerStateObserver::BatteryPowerStatus
                                      battery_power_status) override;

  void OnNativeControlsChanged(bool has_native_controls);
  void OnDisplayTypeChanged(WebMediaPlayer::DisplayType display_type);

  bool ShouldReportWatchTime() const;
  bool ShouldReportingTimerRun() const;
  void MaybeStartReportingTimer(base::TimeDelta start_timestamp);
  enum class FinalizeTime { IMMEDIATELY, ON_NEXT_UPDATE };
  void MaybeFinalizeWatchTime(FinalizeTime finalize_time);
  void RestartTimerForHysteresis();

  // UpdateWatchTime() both records watch time and processes any finalize event.
  void RecordWatchTime();
  void UpdateWatchTime();

  void ResetUnderflowState();

  // Helper methods for creating the components that make up the watch time
  // report. All components except the base component require a creation method
  // and a conversion method to get the correct WatchTimeKey.
  std::unique_ptr<WatchTimeComponent<bool>> CreateBaseComponent();
  std::unique_ptr<WatchTimeComponent<bool>> CreatePowerComponent();
  media::WatchTimeKey GetPowerKey(bool is_on_battery_power);
  std::unique_ptr<WatchTimeComponent<bool>> CreateControlsComponent();
  media::WatchTimeKey GetControlsKey(bool has_native_controls);
  std::unique_ptr<WatchTimeComponent<WebMediaPlayer::DisplayType>>
  CreateDisplayTypeComponent();
  media::WatchTimeKey GetDisplayTypeKey(
      WebMediaPlayer::DisplayType display_type);

  // Initialized during construction.
  const media::mojom::PlaybackPropertiesPtr properties_;
  const bool is_background_;
  const bool is_muted_;
  const GetMediaTimeCB get_media_time_cb_;
  const GetPipelineStatsCB get_pipeline_stats_cb_;
  mojo::Remote<media::mojom::WatchTimeRecorder> recorder_;

  // The amount of time between each UpdateWatchTime(); this is the frequency by
  // which the watch times are updated. In the event of a process crash or kill
  // this is also the most amount of watch time that we might lose.
  base::TimeDelta reporting_interval_ = base::Seconds(5);

  base::RepeatingTimer reporting_timer_;

  // Updated by the OnXXX() methods above; controls timer state.
  bool is_playing_ = false;
  bool is_visible_ = true;
  bool is_seeking_ = false;
  bool in_shutdown_ = false;
  bool has_valid_start_timestamp_ = false;
  double volume_ = 1.0;

  // Updated by UpdateSecondaryProperties(); controls timer state when
  // transitioning above/below kMinimumVideoSize.
  gfx::Size natural_size_;

  int total_underflow_count_ = 0;
  int total_completed_underflow_count_ = 0;
  base::TimeDelta total_underflow_duration_;
  struct UnderflowEvent {
    bool reported = false;
    base::TimeDelta timestamp = media::kNoTimestamp;
    base::TimeDelta duration = media::kNoTimestamp;
  };
  std::vector<UnderflowEvent> pending_underflow_events_
      ALLOW_DISCOURAGED_TYPE("TODO(crbug.com/40760651): Pending migration");

  media::PipelineStatistics initial_stats_;
  media::PipelineStatistics last_stats_;

  // The various components making up WatchTime. If the |base_component_| is
  // finalized, all reporting will be stopped and finalized using its ending
  // timestamp.
  //
  // Note: If you are adding a new type of component (i.e., one that is not
  // bool, etc) you must also update the end of the WatchTimeComponent .cc file
  // to add a new template class definition or you will get linking errors.
  std::unique_ptr<WatchTimeComponent<bool>> base_component_;
  std::unique_ptr<WatchTimeComponent<bool>> power_component_;
  std::unique_ptr<WatchTimeComponent<WebMediaPlayer::DisplayType>>
      display_type_component_;
  std::unique_ptr<WatchTimeComponent<bool>> controls_component_;

  // Special case reporter for handling background video watch time. Configured
  // as an audio only WatchTimeReporter with |is_background_| set to true.
  std::unique_ptr<WatchTimeReporter> background_reporter_;

  // Similar to the above, but for muted audio+video watch time. Configured as
  // an audio+video WatchTimeReporter with |is_muted_| set to true.
  std::unique_ptr<WatchTimeReporter> muted_reporter_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_WATCH_TIME_REPORTER_H_
