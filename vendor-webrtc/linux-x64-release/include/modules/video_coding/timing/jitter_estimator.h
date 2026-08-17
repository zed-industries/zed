/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_TIMING_JITTER_ESTIMATOR_H_
#define MODULES_VIDEO_CODING_TIMING_JITTER_ESTIMATOR_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>

#include "absl/strings/string_view.h"
#include "api/field_trials_view.h"
#include "api/units/data_size.h"
#include "api/units/frequency.h"
#include "api/units/time_delta.h"
#include "api/units/timestamp.h"
#include "modules/video_coding/timing/frame_delay_variation_kalman_filter.h"
#include "modules/video_coding/timing/rtt_filter.h"
#include "rtc_base/experiments/struct_parameters_parser.h"
#include "rtc_base/numerics/moving_percentile_filter.h"
#include "rtc_base/rolling_accumulator.h"

namespace webrtc {

class Clock;

class JitterEstimator {
 public:
  // Configuration struct for statically overriding some constants and
  // behaviour, configurable through field trials.
  struct Config {
    static constexpr char kFieldTrialsKey[] = "WebRTC-JitterEstimatorConfig";

    // Parses a field trial string and validates the values.
    static Config ParseAndValidate(absl::string_view field_trial);

    std::unique_ptr<StructParametersParser> Parser() {
      // clang-format off
      return StructParametersParser::Create(
          "avg_frame_size_median", &avg_frame_size_median,
          "max_frame_size_percentile", &max_frame_size_percentile,
          "frame_size_window", &frame_size_window,
          "num_stddev_delay_clamp", &num_stddev_delay_clamp,
          "num_stddev_delay_outlier", &num_stddev_delay_outlier,
          "num_stddev_size_outlier", &num_stddev_size_outlier,
          "congestion_rejection_factor", &congestion_rejection_factor,
          "estimate_noise_when_congested", &estimate_noise_when_congested);
      // clang-format on
    }

    bool MaxFrameSizePercentileEnabled() const {
      return max_frame_size_percentile.has_value();
    }

    // If true, the "avg" frame size is calculated as the median over a window
    // of recent frame sizes.
    bool avg_frame_size_median = false;

    // If set, the "max" frame size is calculated as this percentile over a
    // window of recent frame sizes.
    std::optional<double> max_frame_size_percentile = std::nullopt;

    // The length of the percentile filters' window, in number of frames.
    std::optional<int> frame_size_window = std::nullopt;

    // The incoming frame delay variation samples are clamped to be at most
    // this number of standard deviations away from zero.
    //
    // Increasing this value clamps fewer samples.
    std::optional<double> num_stddev_delay_clamp = std::nullopt;

    // A (relative) frame delay variation sample is an outlier if its absolute
    // deviation from the Kalman filter model falls outside this number of
    // sample standard deviations.
    //
    // Increasing this value rejects fewer samples.
    std::optional<double> num_stddev_delay_outlier = std::nullopt;

    // An (absolute) frame size sample is an outlier if its positive deviation
    // from the estimated average frame size falls outside this number of sample
    // standard deviations.
    //
    // Increasing this value rejects fewer samples.
    std::optional<double> num_stddev_size_outlier = std::nullopt;

    // A (relative) frame size variation sample is deemed "congested", and is
    // thus rejected, if its value is less than this factor times the estimated
    // max frame size.
    //
    // Decreasing this value rejects fewer samples.
    std::optional<double> congestion_rejection_factor = std::nullopt;

    // If true, the noise estimate will be updated for congestion rejected
    // frames. This is currently enabled by default, but that may not be optimal
    // since congested frames typically are not spread around the line with
    // Gaussian noise. (This is the whole reason for the congestion rejection!)
    bool estimate_noise_when_congested = true;
  };

  JitterEstimator(Clock* clock, const FieldTrialsView& field_trials);
  JitterEstimator(const JitterEstimator&) = delete;
  JitterEstimator& operator=(const JitterEstimator&) = delete;
  ~JitterEstimator();

  // Resets the estimate to the initial state.
  void Reset();

  // Updates the jitter estimate with the new data.
  //
  // Input:
  //          - frame_delay      : Delay-delta calculated by UTILDelayEstimate.
  //          - frame_size       : Frame size of the current frame.
  void UpdateEstimate(TimeDelta frame_delay, DataSize frame_size);

  // Returns the current jitter estimate and adds an RTT dependent term in cases
  // of retransmission.
  //  Input:
  //          - rtt_multiplier   : RTT param multiplier (when applicable).
  //          - rtt_mult_add_cap : Multiplier cap from the RTTMultExperiment.
  //
  // Return value              : Jitter estimate.
  TimeDelta GetJitterEstimate(double rtt_multiplier,
                              std::optional<TimeDelta> rtt_mult_add_cap);

  // Updates the nack counter.
  void FrameNacked();

  // Updates the RTT filter.
  //
  // Input:
  //          - rtt          : Round trip time.
  void UpdateRtt(TimeDelta rtt);

  // Returns the configuration. Only to be used by unit tests.
  Config GetConfigForTest() const;

 private:
  // Updates the random jitter estimate, i.e. the variance of the time
  // deviations from the line given by the Kalman filter.
  //
  // Input:
  //          - d_dT              : The deviation from the kalman estimate.
  void EstimateRandomJitter(double d_dT);

  double NoiseThreshold() const;

  // Calculates the current jitter estimate.
  //
  // Return value                 : The current jitter estimate.
  TimeDelta CalculateEstimate();

  // Post process the calculated estimate.
  void PostProcessEstimate();

  // Returns the estimated incoming frame rate.
  Frequency GetFrameRate() const;

  // Configuration that may override some internals.
  const Config config_;

  // Filters the {frame_delay_delta, frame_size_delta} measurements through
  // a linear Kalman filter.
  FrameDelayVariationKalmanFilter kalman_filter_;

  // TODO(bugs.webrtc.org/14381): Update `avg_frame_size_bytes_` to DataSize
  // when api/units have sufficient precision.
  double avg_frame_size_bytes_;   // Average frame size
  double var_frame_size_bytes2_;  // Frame size variance. Unit is bytes^2.
  // Largest frame size received (descending with a factor kPsi).
  // Used by default.
  // TODO(bugs.webrtc.org/14381): Update `max_frame_size_bytes_` to DataSize
  // when api/units have sufficient precision.
  double max_frame_size_bytes_;
  // Percentile frame sized received (over a window). Only used if configured.
  MovingMedianFilter<int64_t> avg_frame_size_median_bytes_;
  MovingPercentileFilter<int64_t> max_frame_size_bytes_percentile_;
  // TODO(bugs.webrtc.org/14381): Update `startup_frame_size_sum_bytes_` to
  // DataSize when api/units have sufficient precision.
  double startup_frame_size_sum_bytes_;
  size_t startup_frame_size_count_;

  std::optional<Timestamp> last_update_time_;
  // The previously returned jitter estimate
  std::optional<TimeDelta> prev_estimate_;
  // Frame size of the previous frame
  std::optional<DataSize> prev_frame_size_;
  // Average of the random jitter. Unit is milliseconds.
  double avg_noise_ms_;
  // Variance of the time-deviation from the line. Unit is milliseconds^2.
  double var_noise_ms2_;
  size_t alpha_count_;
  // The filtered sum of jitter estimates
  TimeDelta filter_jitter_estimate_ = TimeDelta::Zero();

  size_t startup_count_;
  // Time when the latest nack was seen
  Timestamp latest_nack_ = Timestamp::Zero();
  // Keeps track of the number of nacks received, but never goes above
  // kNackLimit.
  size_t nack_count_;
  RttFilter rtt_filter_;

  // Tracks frame rates in microseconds.
  RollingAccumulator<uint64_t> fps_counter_;
  Clock* clock_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_TIMING_JITTER_ESTIMATOR_H_
