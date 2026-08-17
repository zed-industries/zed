/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_ADAPTATION_OVERUSE_FRAME_DETECTOR_H_
#define VIDEO_ADAPTATION_OVERUSE_FRAME_DETECTOR_H_

#include <list>
#include <memory>
#include <optional>

#include "api/environment/environment.h"
#include "api/field_trials_view.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "rtc_base/experiments/field_trial_parser.h"
#include "rtc_base/numerics/exp_filter.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/task_utils/repeating_task.h"
#include "rtc_base/thread_annotations.h"
#include "video/video_stream_encoder_observer.h"

namespace webrtc {

class VideoFrame;

struct CpuOveruseOptions {
  // Threshold for triggering overuse.
  int high_encode_usage_threshold_percent = 85;
  // Threshold for triggering underuse.
  // Note that we make the interval 2x+epsilon wide, since libyuv scaling steps
  // are close to that (when squared). This wide interval makes sure that
  // scaling up or down does not jump all the way across the interval.
  int low_encode_usage_threshold_percent =
      (high_encode_usage_threshold_percent - 1) / 2;
  // General settings.
  // The maximum allowed interval between two frames before resetting
  // estimations.
  int frame_timeout_interval_ms = 1500;
  // The minimum number of frames required.
  int min_frame_samples = 120;

  // The number of initial process times required before
  // triggering an overuse/underuse.
  int min_process_count = 3;
  // The number of consecutive checks above the high threshold before triggering
  // an overuse.
  int high_threshold_consecutive_count = 2;
  // New estimator enabled if this is set non-zero.
  int filter_time_ms = 0;  // Time constant for averaging
};

class OveruseFrameDetectorObserverInterface {
 public:
  // Called to signal that we can handle larger or more frequent frames.
  virtual void AdaptUp() = 0;
  // Called to signal that the source should reduce the resolution or framerate.
  virtual void AdaptDown() = 0;

 protected:
  virtual ~OveruseFrameDetectorObserverInterface() {}
};

// Use to detect system overuse based on the send-side processing time of
// incoming frames. All methods must be called on a single task queue but it can
// be created and destroyed on an arbitrary thread.
// OveruseFrameDetector::StartCheckForOveruse  must be called to periodically
// check for overuse.
class OveruseFrameDetector {
 public:
  OveruseFrameDetector(const Environment& env,
                       CpuOveruseMetricsObserver* metrics_observer);
  virtual ~OveruseFrameDetector();

  OveruseFrameDetector(const OveruseFrameDetector&) = delete;
  OveruseFrameDetector& operator=(const OveruseFrameDetector&) = delete;

  // Start to periodically check for overuse.
  void StartCheckForOveruse(
      TaskQueueBase* task_queue_base,
      const CpuOveruseOptions& options,
      OveruseFrameDetectorObserverInterface* overuse_observer);

  // StopCheckForOveruse must be called before destruction if
  // StartCheckForOveruse has been called.
  void StopCheckForOveruse();

  // Defines the current maximum framerate targeted by the capturer. This is
  // used to make sure the encode usage percent doesn't drop unduly if the
  // capturer has quiet periods (for instance caused by screen capturers with
  // variable capture rate depending on content updates), otherwise we might
  // experience adaptation toggling.
  virtual void OnTargetFramerateUpdated(int framerate_fps);

  // Called for each captured frame.
  void FrameCaptured(const VideoFrame& frame, int64_t time_when_first_seen_us);

  // Called for each sent frame.
  void FrameSent(uint32_t timestamp,
                 int64_t time_sent_in_us,
                 int64_t capture_time_us,
                 std::optional<int> encode_duration_us);

  // Interface for cpu load estimation. Intended for internal use only.
  class ProcessingUsage {
   public:
    virtual void Reset() = 0;
    virtual void SetMaxSampleDiffMs(float diff_ms) = 0;
    virtual void FrameCaptured(const VideoFrame& frame,
                               int64_t time_when_first_seen_us,
                               int64_t last_capture_time_us) = 0;
    // Returns encode_time in us, if there's a new measurement.
    virtual std::optional<int> FrameSent(
        // These two argument used by old estimator.
        uint32_t timestamp,
        int64_t time_sent_in_us,
        // And these two by the new estimator.
        int64_t capture_time_us,
        std::optional<int> encode_duration_us) = 0;

    virtual int Value() = 0;
    virtual ~ProcessingUsage() = default;
  };

 protected:
  // Protected for test purposes.
  void CheckForOveruse(OveruseFrameDetectorObserverInterface* overuse_observer);
  void SetOptions(const CpuOveruseOptions& options);

  CpuOveruseOptions options_;

 private:
  void EncodedFrameTimeMeasured(int encode_duration_ms);
  bool IsOverusing(int encode_usage_percent);
  bool IsUnderusing(int encode_usage_percent, int64_t time_now);

  bool FrameTimeoutDetected(int64_t now) const;
  bool FrameSizeChanged(int num_pixels) const;

  void ResetAll(int num_pixels);

  static std::unique_ptr<ProcessingUsage> CreateProcessingUsage(
      const FieldTrialsView& field_trials,
      const CpuOveruseOptions& options);

  const Environment env_;
  RTC_NO_UNIQUE_ADDRESS SequenceChecker task_checker_;
  // Owned by the task queue from where StartCheckForOveruse is called.
  RepeatingTaskHandle check_overuse_task_ RTC_GUARDED_BY(task_checker_);

  // Stats metrics.
  CpuOveruseMetricsObserver* const metrics_observer_;
  std::optional<int> encode_usage_percent_ RTC_GUARDED_BY(task_checker_);

  int64_t num_process_times_ RTC_GUARDED_BY(task_checker_);

  int64_t last_capture_time_us_ RTC_GUARDED_BY(task_checker_);

  // Number of pixels of last captured frame.
  int num_pixels_ RTC_GUARDED_BY(task_checker_);
  int max_framerate_ RTC_GUARDED_BY(task_checker_);
  int64_t last_overuse_time_ms_ RTC_GUARDED_BY(task_checker_);
  int checks_above_threshold_ RTC_GUARDED_BY(task_checker_);
  int num_overuse_detections_ RTC_GUARDED_BY(task_checker_);
  int64_t last_rampup_time_ms_ RTC_GUARDED_BY(task_checker_);
  bool in_quick_rampup_ RTC_GUARDED_BY(task_checker_);
  int current_rampup_delay_ms_ RTC_GUARDED_BY(task_checker_);

  std::unique_ptr<ProcessingUsage> usage_ RTC_PT_GUARDED_BY(task_checker_);

  // If set by field trial, overrides CpuOveruseOptions::filter_time_ms.
  FieldTrialOptional<TimeDelta> filter_time_constant_{"tau"};
};

}  // namespace webrtc

#endif  // VIDEO_ADAPTATION_OVERUSE_FRAME_DETECTOR_H_
