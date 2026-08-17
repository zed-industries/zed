// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_VIDEO_DECODE_STATS_REPORTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_VIDEO_DECODE_STATS_REPORTER_H_

#include <memory>
#include <optional>
#include <string>

#include "base/memory/raw_ptr.h"
#include "base/task/single_thread_task_runner.h"
#include "base/time/default_tick_clock.h"
#include "base/time/tick_clock.h"
#include "base/time/time.h"
#include "base/timer/timer.h"
#include "media/base/cdm_config.h"
#include "media/base/pipeline_status.h"
#include "media/base/video_codecs.h"
#include "media/mojo/mojom/video_decode_stats_recorder.mojom.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/size.h"

namespace blink {

// Reports on playback smoothness and for a given video codec profile, natural
// size, and fps. When these properties change the current report will be
// finalized and a new report will be started. Ongoing reports are also
// finalized at destruction and process tear-down.
class PLATFORM_EXPORT VideoDecodeStatsReporter {
 public:
  using GetPipelineStatsCB =
      base::RepeatingCallback<media::PipelineStatistics(void)>;

  VideoDecodeStatsReporter(
      mojo::PendingRemote<media::mojom::VideoDecodeStatsRecorder>
          recorder_remote,
      GetPipelineStatsCB get_pipeline_stats_cb,
      media::VideoCodecProfile codec_profile,
      const gfx::Size& natural_size,
      std::optional<media::CdmConfig> cdm_config,
      scoped_refptr<base::SingleThreadTaskRunner> task_runner,
      const base::TickClock* tick_clock =
          base::DefaultTickClock::GetInstance());
  VideoDecodeStatsReporter(const VideoDecodeStatsReporter&) = delete;
  VideoDecodeStatsReporter& operator=(const VideoDecodeStatsReporter&) = delete;
  ~VideoDecodeStatsReporter();

  void OnPlaying();
  void OnPaused();
  void OnHidden();
  void OnShown();

  // Returns true if given |natural_size| is a match for our internal bucketed
  // size. This allows callers to check a size change is significant enough to
  // motivate recreating the reporter with a the new size.
  bool MatchesBucketedNaturalSize(const gfx::Size& natural_size) const;

  // NOTE: We do not listen for playback rate changes. These implicitly change
  // the frame rate and surface via media::PipelineStatistics'
  // video_frame_duration_average.

 private:
  // Friends so it can see the static constants and inspect when the timer is
  // running / should be running.
  friend class VideoDecodeStatsReporterTest;
  // Friends to grab internal state and verify construction parameters.
  friend class WebMediaPlayerImplTest;

  // Constants placed in header file for test visibility.
  enum : int {
    // Timer interval for recording stats when frame rate and other stream
    // properties are steady.
    kRecordingIntervalMs = 2000,

    // FrameRates must remain stable for a duration greater than this amount to
    // avoid being classified as a "tiny fps window". See |kMaxTinyFpsWindows|.
    kTinyFpsWindowMs = 5000,

    // Limits the number of consecutive "tiny fps windows", as defined by
    // |kTinyFpsWindowMs|. If this limit is met, we will give up until some
    // stream property (e.g. decoder config) changes before trying again. We do
    // not wish to record stats for variable frame rate content.
    kMaxTinyFpsWindows = 5,

    // Number of consecutive samples that must bucket to the same frame rate in
    // order for frame rate to be considered "stable" enough to start recording
    // stats.
    kRequiredStableFpsSamples = 5,

    // Limits the number attempts to detect stable frame rate. When this limit
    // is reached, we will give up until some stream property (e.g. decoder
    // config) changes before trying again. We do not wish to record stats for
    // variable frame rate content.
    kMaxUnstableFpsChanges = 10,
  };

  // Main driver of report updates. Queries |get_pipeline_stats_cb_| for the
  // latest stats. Detects frame rate changes and playback stalls. Sends stats
  // the VideoDecodeStatsRecorder (browser process) for saving. Should only be
  // called when ShouldBeReporting() == true.
  void UpdateStats();

  // Run |stats_cb_timer_| at the specified |interval|. If the timer is already
  // running, any existing callbacks will be canceled/delayed until
  // |interval| has elapsed.
  void RunStatsTimerAtInterval(base::TimeDelta interval);

  // Called to begin a new report following changes to stream metadata (e.g.
  // natural size). Arguments used to update |frames_decoded_offset_|,
  // |frames_dropped_offset_|, and |frames_decoded_power_efficient_offset| so
  // that frame counts for this report begin at 0.
  void StartNewRecord(uint32_t frames_decoded_offset,
                      uint32_t frames_dropped_offset,
                      uint32_t frames_decoded_power_efficient_offset);

  // Reset frame rate tracking state to force a fresh attempt at detection. When
  // a stable frame rate is successfully detected, UpdateStats() will begin a
  // new record will begin with the detected frame rate. Note: callers must
  // separately ensure the |stats_cb_timer_| is running for frame rate detection
  // to occur.
  void ResetFrameRateState();

  // Called by UpdateStats() to verify decode is progressing and sanity check
  // decode/dropped frame counts. Will manage decoded/dropped frame state and
  // relax timer when no decode progress is made. Returns true iff decode is
  // progressing.
  bool UpdateDecodeProgress(const media::PipelineStatistics& stats);

  // Called by UpdateStats() to do frame rate detection. Will manage frame rate
  // state, stats timer, and will start new capabilities records when frame rate
  // changes. Returns true iff frame rate is stable.
  bool UpdateFrameRateStability(const media::PipelineStatistics& stats);

  // Returns true if the |stats_timer_cb_| should be running. Should be called
  // after any state change (e.g. |is_playing_|) as a check on whether to start
  // the the timer.
  bool ShouldBeReporting() const;

  // Error handler callback when IPC hits an error.
  void OnIpcConnectionError();

  // TimeDelta wrappers around |kRecordingIntervalMs| and |kTimyFpsWindowMs|.
  // Defined as a class member to avoid static initialization.
  // TODO(chcunningham): convert to static constexpr when MSVC support arrives.
  const base::TimeDelta kRecordingInterval;
  const base::TimeDelta kTinyFpsWindowDuration;

  // mojo::Remote for the recorder. The recorder runs in the browser process
  // and finalizes the record in the event of fast render process tear down.
  mojo::Remote<media::mojom::VideoDecodeStatsRecorder> recorder_remote_;

  // Callback for retrieving playback statistics.
  GetPipelineStatsCB get_pipeline_stats_cb_;

  // Current video codec profile, used to index recorded stats.
  const media::VideoCodecProfile codec_profile_;

  // Current video natural size, used to index recorded stats. These dimensions
  // will always be rounded to the nearest size bucket. If the original size is
  // very small, the bucketed size will simply be empty. See GetSizeBucket().
  const gfx::Size natural_size_;

  // The name of the current key system. Empty for unencrypted playback.
  const std::string key_system_;

  // From media::CdmConfig in constructor.
  const bool use_hw_secure_codecs_;

  // Clock for |stats_cb_timer_| and getting current tick count (NowTicks()).
  // Tests may supply a mock clock via the constructor.
  raw_ptr<const base::TickClock> tick_clock_;

  // Timer for all stats callbacks. Timer interval will be dynamically set based
  // on state of reporter. See calls to RunStatsTimerAtIntervalMs().
  base::RepeatingTimer stats_cb_timer_;

  // Latest frame duration bucketed into one of kFrameRateBuckets.
  int last_observed_fps_ = 0;

  // Count of consecutive samples with frame duration matching
  // |last_observed_fps_|.
  int num_stable_fps_samples_ = 0;

  // Count of consecutive samples with frame duration NOT matching
  // |last_obseved_fps_|. Used to throttle/limit attempts to stabilize FPS since
  // some videos may have variable frame rate.
  int num_unstable_fps_changes_ = 0;

  // Count of consecutive stable FPS windows, where stable FPS was detected but
  // it lasted for a very short duration before changing again.
  int num_consecutive_tiny_fps_windows_ = 0;

  // True whenever we fail to determine a stable FPS, or if windows of stable
  // FPS are too tiny to be useful.
  bool fps_stabilization_failed_ = false;

  // Tick time of the most recent FPS stabilization. When FPS changes, this time
  // is compared to TimeTicks::Now() to approximate the duration of the last
  // stable FPS window.
  base::TimeTicks last_fps_stabilized_ticks_;

  // Count of frames decoded observed in last pipeline stats update. Used to
  // check whether decode/playback has actually advanced.
  uint32_t last_frames_decoded_ = 0;

  // Count of frames dropped observed in last pipeline stats update. Used to
  // verify that count never decreases.
  uint32_t last_frames_dropped_ = 0;

  // Notes the number of frames decoded at the start of the current video
  // configuration (profile, resolution, fps). Should be subtracted from
  // pipeline frames decoded stats before sending to recorder.
  uint32_t frames_decoded_offset_ = 0;

  // Notes the number of frames dropped at the start of the current video
  // configuration (profile, resolution, fps). Should be subtracted from
  // pipeline frames dropped stats before sending to recorder.
  uint32_t frames_dropped_offset_ = 0;

  // Notes the number of power efficiently decoded frames at the start of the
  // current video configuration (profile, resolution, fps). Should be
  // subtracted from pipeline frames decoded stats before sending to recorder.
  uint32_t frames_decoded_power_efficient_offset_ = 0;

  // Set true by OnPlaying(), false by OnPaused(). We should not run the
  // |stats_cb_timer_| when not playing.
  bool is_playing_ = false;

  // Set true by OnHidden(), false by OnVisible(). We should not run the
  // |stats_cb_timer_| when player is backgrounded.
  bool is_backgrounded_ = false;

  // Set false by UpdateStats() if an IPC error is encountered. Assumed true
  // until an error is found.
  bool is_ipc_connected_ = true;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_MEDIA_VIDEO_DECODE_STATS_REPORTER_H_
