/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_UTILITY_QUALITY_SCALER_H_
#define MODULES_VIDEO_CODING_UTILITY_QUALITY_SCALER_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>

#include "api/field_trials_view.h"
#include "api/sequence_checker.h"
#include "api/video_codecs/video_encoder.h"
#include "rtc_base/experiments/quality_scaling_experiment.h"
#include "rtc_base/numerics/moving_average.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {

class QualityScalerQpUsageHandlerCallbackInterface;
class QualityScalerQpUsageHandlerInterface;

// QualityScaler runs asynchronously and monitors QP values of encoded frames.
// It holds a reference to a QualityScalerQpUsageHandlerInterface implementation
// to signal an overuse or underuse of QP (which indicate a desire to scale the
// video stream down or up).
class QualityScaler {
 public:
  // Construct a QualityScaler with given `thresholds` and `handler`.
  // This starts the quality scaler periodically checking what the average QP
  // has been recently.
  QualityScaler(QualityScalerQpUsageHandlerInterface* handler,
                VideoEncoder::QpThresholds thresholds,
                const FieldTrialsView& field_trials);
  virtual ~QualityScaler();
  // Should be called each time a frame is dropped at encoding.
  void ReportDroppedFrameByMediaOpt();
  void ReportDroppedFrameByEncoder();
  // Inform the QualityScaler of the last seen QP.
  void ReportQp(int qp, int64_t time_sent_us);

  void SetQpThresholds(VideoEncoder::QpThresholds thresholds);

  // The following members declared protected for testing purposes.
 protected:
  QualityScaler(QualityScalerQpUsageHandlerInterface* handler,
                VideoEncoder::QpThresholds thresholds,
                const FieldTrialsView& field_trials,
                int64_t sampling_period_ms);

 private:
  class QpSmoother;
  class CheckQpTask;
  class CheckQpTaskHandlerCallback;

  enum class CheckQpResult {
    kInsufficientSamples,
    kNormalQp,
    kHighQp,
    kLowQp,
  };

  // Starts checking for QP in a delayed task. When the resulting CheckQpTask
  // completes, it will invoke this method again, ensuring that we always
  // periodically check for QP. See CheckQpTask for more details. We never run
  // more than one CheckQpTask at a time.
  void StartNextCheckQpTask();

  CheckQpResult CheckQp() const;
  void ClearSamples();

  std::unique_ptr<CheckQpTask> pending_qp_task_ RTC_GUARDED_BY(&task_checker_);
  QualityScalerQpUsageHandlerInterface* const handler_
      RTC_GUARDED_BY(&task_checker_);
  RTC_NO_UNIQUE_ADDRESS SequenceChecker task_checker_;

  VideoEncoder::QpThresholds thresholds_ RTC_GUARDED_BY(&task_checker_);
  const int64_t sampling_period_ms_;
  bool fast_rampup_ RTC_GUARDED_BY(&task_checker_);
  MovingAverage average_qp_ RTC_GUARDED_BY(&task_checker_);
  MovingAverage framedrop_percent_media_opt_ RTC_GUARDED_BY(&task_checker_);
  MovingAverage framedrop_percent_all_ RTC_GUARDED_BY(&task_checker_);

  // Used by QualityScalingExperiment.
  const bool experiment_enabled_;
  QualityScalingExperiment::Config config_ RTC_GUARDED_BY(&task_checker_);
  std::unique_ptr<QpSmoother> qp_smoother_high_ RTC_GUARDED_BY(&task_checker_);
  std::unique_ptr<QpSmoother> qp_smoother_low_ RTC_GUARDED_BY(&task_checker_);

  const size_t min_frames_needed_;
  const double initial_scale_factor_;
  const std::optional<double> scale_factor_;
};

// Reacts to QP being too high or too low. For best quality, when QP is high it
// is desired to decrease the resolution or frame rate of the stream and when QP
// is low it is desired to increase the resolution or frame rate of the stream.
// Whether to reconfigure the stream is ultimately up to the handler, which is
// able to respond asynchronously.
class QualityScalerQpUsageHandlerInterface {
 public:
  virtual ~QualityScalerQpUsageHandlerInterface();

  virtual void OnReportQpUsageHigh() = 0;
  virtual void OnReportQpUsageLow() = 0;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_UTILITY_QUALITY_SCALER_H_
