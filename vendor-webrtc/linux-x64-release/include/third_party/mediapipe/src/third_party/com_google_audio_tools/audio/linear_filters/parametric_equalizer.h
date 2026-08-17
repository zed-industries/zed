/*
 * Copyright 2020 Google LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// A multi-band parametric equalizer.
//
// All bands can be manipulated independently. In addition to supporting an
// arbitrary number of filters for boosting/attenuating specific frequencies.


#ifndef AUDIO_LINEAR_FILTERS_PARAMETRIC_EQUALIZER_H_
#define AUDIO_LINEAR_FILTERS_PARAMETRIC_EQUALIZER_H_

#include "audio/linear_filters/biquad_filter_coefficients.h"
#include "audio/linear_filters/equalizer_filter_params.h"
#include "absl/strings/str_format.h"
#include "absl/types/span.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


// Example Use:
//   ParametricEqualizerParams eq_params;
//   /* Set parameters as desired. The test shows a good example of this. */
//
//   /* See biquad_filter.h for other processing options. */
//   BiquadFilterCascade<float, ArrayXf> filter;
//   filter.Init(num_channels, DesignParametricEqualizer(48000));
//   while (...) {
//     ArrayXXf input = ...
//     ArrayXXf output;
//     filter.ProcessBlock(input, &output);
//     ...
//   }

namespace linear_filters {

class ParametricEqualizerParams {
 public:
  ParametricEqualizerParams()
    : gain_db_(0) {}

  ~ParametricEqualizerParams() {}
  // Bypassed stages create a stage of identity coefficients such that the order
  // of the filter is always the same.
  BiquadFilterCascadeCoefficients GetCoefficients(float sample_rate_hz) const;

  float GetGainDb() const;
  void SetGainDb(float gain);

  // Getters and setters for the filter stages.
  bool IsStageEnabled(int index) const;
  void SetStageEnabled(int index, bool enabled);
  const EqualizerFilterParams& StageParams(int index) const;
  EqualizerFilterParams* MutableStageParams(int index);

  // The new stage is enabled (not bypassed) by default.
  void AddStage(const EqualizerFilterParams& params);
  void AddStage(EqualizerFilterParams::Type type, float frequency_hz,
                float quality_factor, float gain_db);

  // Sets the EQ parameters such that the filter leaves the signal unchanged.
  // - Sets all gains to zero.
  // - Disables the lowpass and highpass filters.
  // - The center frequencies, Q values, and bypass status of the
  //   shelf/peak filters is unchanged.
  void ClearAllGains();

  int GetNumEnabledStages() const;
  int GetTotalNumStages() const;

  std::string ToString() const;

  const std::vector<EqualizerFilterParams>& AllStages() const {
    return all_stages_;
  }

 private:
  float gain_db_;

  // Filters for boosting or attenuating specific frequencies.
  std::vector<EqualizerFilterParams> all_stages_;
  std::vector<uint8 /* bool */> stages_enabled_;
};


struct ConvergenceParams {
  constexpr ConvergenceParams()
      : max_iterations(2), convergence_threshold_db(0.2) {}
  // The highest number of iterations allowed. Note that unless it is known that
  // the corner frequencies and the Q's are very well chosen, it is probably the
  // case that the error from badly matched parameters exceeds the improvement
  // gained by iterating until a very tight convergence.
  int max_iterations;
  // If the largest change between two iterations is less than this threshold,
  // the algorithm will stop iterating.
  float convergence_threshold_db;
};

struct NelderMeadFitParams {
  NelderMeadFitParams()
      : min_frequency_hz(1),
        max_frequency_hz(24000),
        min_quality_factor(0.1),
        max_quality_factor(5),
        max_iterations(1500),
        magnitude_db_rms_error_tol(0.1),
        magnitude_db_rms_error_stddev_tol(0) {
    inner_convergence_params.convergence_threshold_db =
        magnitude_db_rms_error_stddev_tol / 2;
    inner_convergence_params.max_iterations = 3;
  }
  // Lower bound on all filter frequencies. Must be positive and less than
  // max_frequency_hz.
  float min_frequency_hz;
  // Upper bound on all filter frequencies. Must be positive and greater than
  // min_frequency_hz.
  float max_frequency_hz;
  // Lower bound on all filter quality factors.
  float min_quality_factor;
  // Upper bound on all filter quality factors.
  float max_quality_factor;
  // Stop if more than this number of iterations were performed.
  int max_iterations;
  // Stop if the smallest RMS error achieved is equal or less.
  float magnitude_db_rms_error_tol;
  // Stop if the standard deviation of the RMS error for
  // the simplex points is less or equal.
  float magnitude_db_rms_error_stddev_tol;
  // Convergence parameters for the inner iterated linear-least squares
  // algorithm.
  ConvergenceParams inner_convergence_params;
};

// Computes an estimate of parametric equalizer parameters to match a target
// response in the least squares sense.
//
// Note that the number of filters must already be set in eq_params. Corner
// frequencies and quality factors should be set to a reasonable starting
// guess and must satisfy the constraints set in `fit_params` (see next
// paragraph for details). Note also that the enabled/disabled status of a
// filter is currently ignored. All filters in the EQ will be used for the fit.
//
// Do not use parametric equalizers with lowpass or highpass filters.
//
// This function sets the gains, corner frequencies, and the quality factors in
// `eq_params` to a least-squares fit to the response specified by
// `frequencies_hz` and `magnitude_target_db`. It uses the Nelder-Mead
// derivative free optimization algorithm to search through the space of corner
// frequencies and quality factors, and for each such point calls
// SetParametricEqualizerGainsToMatchMagnitudeResponse() to compute the optimal
// gains using (unconstrained) linear least-squares.
// Corner frequencies and quality factors are constrained to the intervals
// [-`fit_params.min_frequency_hz`, `fit_params.max_frequency_hz`] and
// [0:`fit_params.max_quality_factor`], respectively.
//
// The algorithm stops when either
//   - more than `fit_params.max_iterations` calls to
//     SetParametricEqualizerGainsToMatchMagnitudeResponse() have been done, or
//   - the root-mean-square error of the fit is less than
//     `fit_params.gain_residual_rms_db_tol`, or
//   - the standard deviation of the equalizer gain evaluated at the simplex
//     points maintained by the Nelder-Mead algorithm is less than
//     `fit_params.gain_residual_stddev_db_tol`.
//
// Returns the root-mean-square error (in dB) of the response of filter
// corresponding to `eq_params` w.r.t. `magnitude_target_db`.
//
// TODO: We should probably apply a bit of smoothing to
// magnitude_target_db before running the fit. Nelder-Mead is not always
// terribly robust in the presence of noise, and we don't want to
// accidentally fit peak filters to narrow peaks or dips caused by noise.
float FitParametricEqualizer(absl::Span<const float> frequencies_hz,
                             absl::Span<const float> magnitude_target_db,
                             float sample_rate_hz,
                             const NelderMeadFitParams& fit_params,
                             ParametricEqualizerParams* eq_params);

// Provides a least-squares fit for a magnitude response using a parametric
// equalizer. Note that the number of filters, the corner frequencies, and the
// quality factors of the filters must already be set in eq_params. This
// function only sets the gains.
//
// Note that the optimizer will only try to match the points of
// magnitude_target_db. The response in between these samples is only
// regularized by the order of the filter specified in eq_params. Using high Q
// filters is not advised, especially for sparse samplings of the spectrum. For
// a 5-band equalizer, Q values exceeding 5 are probably a bad idea.
//
// Runtime is cubic in the number of samples provided
// O(k^2*(n + k)) = O(k^3), where n is the number filters in eq_params and k is
// the number of points in magnitude_target_db.
void SetParametricEqualizerGainsToMatchMagnitudeResponse(
    absl::Span<const float> frequencies_hz,
    absl::Span<const float> magnitude_target_db, float sample_rate_hz,
    ParametricEqualizerParams* eq_params);

// Same as above, but allows for control over the amount of computation
// performed.
void SetParametricEqualizerGainsToMatchMagnitudeResponse(
    absl::Span<const float> frequencies_hz,
    absl::Span<const float> magnitude_target_db,
    ConvergenceParams convergence_params, float sample_rate_hz,
    ParametricEqualizerParams* eq_params);

// Returns the overall gain factor of the parametric equalizer at the specified
// frequencies.
Eigen::ArrayXf ParametricEqualizerGainMagnitudeAtFrequencies(
    const ParametricEqualizerParams& eq_params, float sample_rate_hz,
    const Eigen::ArrayXf& frequencies_hz);

}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_PARAMETRIC_EQUALIZER_H_
