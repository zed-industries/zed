/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_INTERPOLATED_GAIN_CURVE_H_
#define MODULES_AUDIO_PROCESSING_AGC2_INTERPOLATED_GAIN_CURVE_H_

#include <array>

#include "absl/strings/string_view.h"
#include "modules/audio_processing/agc2/agc2_common.h"
#include "rtc_base/gtest_prod_util.h"
#include "system_wrappers/include/metrics.h"

namespace webrtc {

class ApmDataDumper;

constexpr float kInputLevelScalingFactor = 32768.0f;

// Defined as DbfsToLinear(kLimiterMaxInputLevelDbFs)
constexpr float kMaxInputLevelLinear = static_cast<float>(36766.300710566735);

// Interpolated gain curve using under-approximation to avoid saturation.
//
// The goal of this class is allowing fast look ups to get an accurate
// estimates of the gain to apply given an estimated input level.
class InterpolatedGainCurve {
 public:
  enum class GainCurveRegion {
    kIdentity = 0,
    kKnee = 1,
    kLimiter = 2,
    kSaturation = 3
  };

  struct Stats {
    // Region in which the output level equals the input one.
    size_t look_ups_identity_region = 0;
    // Smoothing between the identity and the limiter regions.
    size_t look_ups_knee_region = 0;
    // Limiter region in which the output and input levels are linearly related.
    size_t look_ups_limiter_region = 0;
    // Region in which saturation may occur since the input level is beyond the
    // maximum expected by the limiter.
    size_t look_ups_saturation_region = 0;
    // True if stats have been populated.
    bool available = false;

    // The current region, and for how many frames the level has been
    // in that region.
    GainCurveRegion region = GainCurveRegion::kIdentity;
    int64_t region_duration_frames = 0;
  };

  InterpolatedGainCurve(ApmDataDumper* apm_data_dumper,
                        absl::string_view histogram_name_prefix);
  ~InterpolatedGainCurve();

  InterpolatedGainCurve(const InterpolatedGainCurve&) = delete;
  InterpolatedGainCurve& operator=(const InterpolatedGainCurve&) = delete;

  Stats get_stats() const { return stats_; }

  // Given a non-negative input level (linear scale), a scalar factor to apply
  // to a sub-frame is returned.
  // Levels above kLimiterMaxInputLevelDbFs will be reduced to 0 dBFS
  // after applying this gain
  float LookUpGainToApply(float input_level) const;

 private:
  // For comparing 'approximation_params_*_' with ones computed by
  // ComputeInterpolatedGainCurve.
  FRIEND_TEST_ALL_PREFIXES(GainController2InterpolatedGainCurve,
                           CheckApproximationParams);

  struct RegionLogger {
    metrics::Histogram* identity_histogram;
    metrics::Histogram* knee_histogram;
    metrics::Histogram* limiter_histogram;
    metrics::Histogram* saturation_histogram;

    RegionLogger(absl::string_view identity_histogram_name,
                 absl::string_view knee_histogram_name,
                 absl::string_view limiter_histogram_name,
                 absl::string_view saturation_histogram_name);

    ~RegionLogger();

    void LogRegionStats(const InterpolatedGainCurve::Stats& stats) const;
  } region_logger_;

  void UpdateStats(float input_level) const;

  ApmDataDumper* const apm_data_dumper_;

  static constexpr std::array<float, kInterpolatedGainCurveTotalPoints>
      approximation_params_x_ = {
          {30057.296875,    30148.986328125, 30240.67578125,  30424.052734375,
           30607.4296875,   30790.806640625, 30974.18359375,  31157.560546875,
           31340.939453125, 31524.31640625,  31707.693359375, 31891.0703125,
           32074.447265625, 32257.82421875,  32441.201171875, 32624.580078125,
           32807.95703125,  32991.33203125,  33174.7109375,   33358.08984375,
           33541.46484375,  33724.84375,     33819.53515625,  34009.5390625,
           34200.05859375,  34389.81640625,  34674.48828125,  35054.375,
           35434.86328125,  35814.81640625,  36195.16796875,  36575.03125}};
  static constexpr std::array<float, kInterpolatedGainCurveTotalPoints>
      approximation_params_m_ = {
          {-3.515235675877192989e-07, -1.050251626111275982e-06,
           -2.085213736791047268e-06, -3.443004743530764244e-06,
           -4.773849468620028347e-06, -6.077375928725814447e-06,
           -7.353257842623861507e-06, -8.601219633419532329e-06,
           -9.821013009059242904e-06, -1.101243378798244521e-05,
           -1.217532644659513608e-05, -1.330956911260727793e-05,
           -1.441507538402220234e-05, -1.549179251014720649e-05,
           -1.653970684856176376e-05, -1.755882840370759368e-05,
           -1.854918446042574942e-05, -1.951086778717581183e-05,
           -2.044398024736437947e-05, -2.1348627342376858e-05,
           -2.222496914328075945e-05, -2.265374678245279938e-05,
           -2.242570917587727308e-05, -2.220122041762806475e-05,
           -2.19802095671184361e-05,  -2.176260204578284174e-05,
           -2.133731686626560986e-05, -2.092481918225530535e-05,
           -2.052459603874012828e-05, -2.013615448959171772e-05,
           -1.975903069251216948e-05, -1.939277899509761482e-05}};

  static constexpr std::array<float, kInterpolatedGainCurveTotalPoints>
      approximation_params_q_ = {
          {1.010565876960754395, 1.031631827354431152, 1.062929749488830566,
           1.104239225387573242, 1.144973039627075195, 1.185109615325927734,
           1.224629044532775879, 1.263512492179870605, 1.301741957664489746,
           1.339300632476806641, 1.376173257827758789, 1.412345528602600098,
           1.447803974151611328, 1.482536554336547852, 1.516532182693481445,
           1.549780607223510742, 1.582272171974182129, 1.613999366760253906,
           1.644955039024353027, 1.675132393836975098, 1.704526185989379883,
           1.718986630439758301, 1.711274504661560059, 1.703639745712280273,
           1.696081161499023438, 1.688597679138183594, 1.673851132392883301,
           1.659391283988952637, 1.645209431648254395, 1.631297469139099121,
           1.617647409439086914, 1.604251742362976074}};

  // Stats.
  mutable Stats stats_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_INTERPOLATED_GAIN_CURVE_H_
