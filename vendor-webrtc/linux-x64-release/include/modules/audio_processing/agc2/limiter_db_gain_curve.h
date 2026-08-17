/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_LIMITER_DB_GAIN_CURVE_H_
#define MODULES_AUDIO_PROCESSING_AGC2_LIMITER_DB_GAIN_CURVE_H_

#include <array>

#include "modules/audio_processing/agc2/agc2_testing_common.h"

namespace webrtc {

// A class for computing a limiter gain curve (in dB scale) given a set of
// hard-coded parameters (namely, kLimiterDbGainCurveMaxInputLevelDbFs,
// kLimiterDbGainCurveKneeSmoothnessDb, and
// kLimiterDbGainCurveCompressionRatio). The generated curve consists of four
// regions: identity (linear), knee (quadratic polynomial), compression
// (linear), saturation (linear). The aforementioned constants are used to shape
// the different regions.
class LimiterDbGainCurve {
 public:
  LimiterDbGainCurve();

  double max_input_level_db() const { return max_input_level_db_; }
  double max_input_level_linear() const { return max_input_level_linear_; }
  double knee_start_linear() const { return knee_start_linear_; }
  double limiter_start_linear() const { return limiter_start_linear_; }

  // These methods can be marked 'constexpr' in C++ 14.
  double GetOutputLevelDbfs(double input_level_dbfs) const;
  double GetGainLinear(double input_level_linear) const;
  double GetGainFirstDerivativeLinear(double x) const;
  double GetGainIntegralLinear(double x0, double x1) const;

 private:
  double GetKneeRegionOutputLevelDbfs(double input_level_dbfs) const;
  double GetCompressorRegionOutputLevelDbfs(double input_level_dbfs) const;

  static constexpr double max_input_level_db_ = test::kLimiterMaxInputLevelDbFs;
  static constexpr double knee_smoothness_db_ = test::kLimiterKneeSmoothnessDb;
  static constexpr double compression_ratio_ = test::kLimiterCompressionRatio;

  const double max_input_level_linear_;

  // Do not modify signal with level <= knee_start_dbfs_.
  const double knee_start_dbfs_;
  const double knee_start_linear_;

  // The upper end of the knee region, which is between knee_start_dbfs_ and
  // limiter_start_dbfs_.
  const double limiter_start_dbfs_;
  const double limiter_start_linear_;

  // Coefficients {a, b, c} of the knee region polynomial
  // ax^2 + bx + c in the DB scale.
  const std::array<double, 3> knee_region_polynomial_;

  // Parameters for the computation of the first derivative of GetGainLinear().
  const double gain_curve_limiter_d1_;
  const double gain_curve_limiter_d2_;

  // Parameters for the computation of the integral of GetGainLinear().
  const double gain_curve_limiter_i1_;
  const double gain_curve_limiter_i2_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_LIMITER_DB_GAIN_CURVE_H_
