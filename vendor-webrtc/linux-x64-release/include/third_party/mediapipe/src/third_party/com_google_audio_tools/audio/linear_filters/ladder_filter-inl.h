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

#ifndef AUDIO_LINEAR_FILTERS_LADDER_FILTER_INL_H_
#define AUDIO_LINEAR_FILTERS_LADDER_FILTER_INL_H_

#include <cmath>
#include <vector>

#include "glog/logging.h"

#include "audio/linear_filters/ladder_filter.h"
#include "absl/base/optimization.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {
namespace internal {

// Adjust the coefficients of an unstable filter so that the filter is still
// stable.
template <typename CoeffVectorType>
void ForceStability(CoeffVectorType* reflection_ptr) {
  CoeffVectorType& reflection = *reflection_ptr;
  using ScalarType = typename CoeffVectorType::Scalar;
  // Written with negative logic to catch NaNs.
  for (int m = 0; m < reflection.size(); ++m) {
    if (!(std::abs(reflection[m]) < 1.0)) {
      DLOG_EVERY_N(WARNING, 1000) << "LOG_EVERY_N(1000): Computed ladder " <<
          "filter coefficient (" << reflection[m] << ") magnitude exceeds 1.";
      if (reflection[m] > 0) {
        reflection[m] = 1 - 10 * std::numeric_limits<ScalarType>::epsilon();
      } else {
        reflection[m] = -1 + 10 * std::numeric_limits<ScalarType>::epsilon();
      }
    }
  }
}

template <typename CoeffVectorType>
void SetLadderCoefficientsWithStabilityCheck(
    std::vector<double> coeffs_k,
    std::vector<double> coeffs_v,
    CoeffVectorType* reflection_ptr,
    CoeffVectorType* tap_gains_ptr) {
  CoeffVectorType& reflection = *reflection_ptr;
  CoeffVectorType& tap_gains = *tap_gains_ptr;
  int filter_order = coeffs_k.size();
  reflection.resize(filter_order);
  tap_gains.resize(filter_order + 1);

  for (int m = 0; m < filter_order; ++m) {
    tap_gains[m] = coeffs_v[m];
    reflection[m] = coeffs_k[m];
  }
  ForceStability(&reflection);
  tap_gains[filter_order] = coeffs_v[filter_order];
}

template <typename CoeffVectorType>
void ConvertTapGains(const CoeffVectorType& coeffs_k,
                     CoeffVectorType* coeffs_v_ptr) {
  CoeffVectorType& coeffs_v = *coeffs_v_ptr;
  // The pi scalar coefficient is introduced in Equation (22) [1] and is used to
  // convert the v coefficients to v-hat coefficients for the one multiply
  // lattice. Reference [2] (starting with Equation (5)) redefines these
  // coefficients for the power-normalized ladder filter in Equation (13) [2].
  double pi = 1.0;
  for (int m = coeffs_k.size(); m >= 0; --m) {
    coeffs_v[m] /= pi;
    if (m >= 1) {
      pi *= std::sqrt(1 - coeffs_k[m - 1] * coeffs_k[m - 1]);
    }
  }
}

template <typename CoeffVectorType>
void GetScatteringCoefficients(const CoeffVectorType& reflection,
                               CoeffVectorType* scattering_ptr) {
  CoeffVectorType& scattering = *scattering_ptr;
  scattering.resize(reflection.size());
  for (int m = 0; m < reflection.size(); ++m) {
    scattering[m] = std::sqrt(1 - reflection[m] * reflection[m]);
  }
  ForceStability(scattering_ptr);
}

}  // namespace internal

template <typename _SampleType>
void LadderFilter<_SampleType>::InitFromLadderCoeffs(
    int num_channels,
    const std::vector<double>& coeffs_k,
    const std::vector<double>& coeffs_v) {
  initialized_ = false;
  if (kNumChannelsAtCompileTime == Eigen::Dynamic) {
    ABSL_CHECK_GE(num_channels, 1);
  } else {
    ABSL_CHECK_EQ(num_channels, kNumChannelsAtCompileTime);
  }
  num_channels_ = num_channels;
  ChangeLadderCoeffs(coeffs_k, coeffs_v);
  initialized_ = true;
  Reset();
}

template <typename _SampleType>
void LadderFilter<_SampleType>::ChangeLadderCoeffs(
    const std::vector<double>& coeffs_k,
    const std::vector<double>& coeffs_v) {
  ABSL_CHECK(!coeffs_k.empty());
  ABSL_CHECK_EQ(coeffs_v.size(), coeffs_k.size() + 1);
  if (initialized_) {
    ABSL_DCHECK_EQ(filter_order_, coeffs_k.size());
    // Compute new target coefficients.
    internal::SetLadderCoefficientsWithStabilityCheck(
        coeffs_k, coeffs_v, &reflection_target_, &tap_gains_target_);
    internal::ConvertTapGains(reflection_target_, &tap_gains_target_);
    // Make sure that this was set properly.
    ABSL_DCHECK_GT(smoothing_samples_max_, 0);
    smoothing_samples_ = smoothing_samples_max_;
  } else {
    // We are initializing for the first time.
    internal::SetLadderCoefficientsWithStabilityCheck(
        coeffs_k, coeffs_v, &reflection_, &tap_gains_);
    filter_order_ = reflection_.size();
    internal::ConvertTapGains(reflection_, &tap_gains_);
    internal::GetScatteringCoefficients(reflection_, &scattering_);
    reflection_target_ = reflection_;
    tap_gains_target_ = tap_gains_;

    // At 48k, this smoother has a corner frequency at 240Hz. The difference
    // between a 40Hz cutoff and a 2k cutoff was surprisingly inaudible
    // when tested using ./examples/ladder_filter_autowah.cc. 240Hz should be
    // a good balance between audible artifacts under extreme conditions
    // and excessive computation.
    //
    // It is critical that the Q of this filter be very low so that the step
    // response has no overshoot whatsoever. Overshoot will cause the
    // coefficients to exceed 1.0 in magnitude during the smoothing.
    // Q = 0.5 is overdamped, we go slightly less to be safe.
    constexpr float kOverdamped = 0.49;
    BiquadFilterCoefficients smoothing_coeffs =
         LowpassBiquadFilterCoefficients(1.0, 0.005, kOverdamped);
    // We don't stop smoothing until the samples are within -80dB of
    // their target value.
    smoothing_samples_max_ =
      std::ceil(smoothing_coeffs.EstimateDecayTime(80.0));
    reflection_smoother_.Init(reflection_.size(), smoothing_coeffs);
    tap_gains_smoother_.Init(tap_gains_.size(), smoothing_coeffs);
  }
}

template <typename _SampleType>
void LadderFilter<_SampleType>::InitFromTransferFunction(
    int num_channels,
    const std::vector<double>& coeffs_b,
    const std::vector<double>& coeffs_a) {
  std::vector<double> coeffs_k;
  std::vector<double> coeffs_v;
  MakeLadderCoefficientsFromTransferFunction(coeffs_b, coeffs_a,
                                             &coeffs_k, &coeffs_v);
  InitFromLadderCoeffs(num_channels, coeffs_k, coeffs_v);
}

template <typename _SampleType>
void LadderFilter<_SampleType>::ChangeCoeffsFromTransferFunction(
    const std::vector<double>& coeffs_b,
                                      const std::vector<double>& coeffs_a) {
  std::vector<double> coeffs_k;
  std::vector<double> coeffs_v;
  MakeLadderCoefficientsFromTransferFunction(coeffs_b, coeffs_a,
                                             &coeffs_k, &coeffs_v);
  ChangeLadderCoeffs(coeffs_k, coeffs_v);
}

template <typename _SampleType>
void LadderFilter<_SampleType>::Reset() {
  ABSL_CHECK(initialized_) << "Reset() called before initialization.";
  state_ = Traits::LadderStateType::Zero(num_channels_, filter_order_ + 1);
  reflection_smoother_.Reset();
  tap_gains_smoother_.Reset();

  reflection_smoother_.SetSteadyStateCondition(reflection_target_);
  tap_gains_smoother_.SetSteadyStateCondition(tap_gains_target_);
  smoothing_samples_ = 0;
}

template <typename _SampleType>
template <typename InputType, typename OutputType>
void LadderFilter<_SampleType>::ProcessSample(const InputType& input,
                                              OutputType* output) {
  using InputScalarType =
      typename Traits::template GetScalarType<InputType>::Type;
  constexpr int kFixedNumChannels =
      Traits::template GetFixedNumChannels<InputType, OutputType>();
  ABSL_DCHECK(initialized_) << "ProcessBlock() called before initialization.";
  ABSL_DCHECK(output != nullptr);

  // The convention is followed from the paper that outermost stages in the
  // lattice have higher indices/subscripts, i.e. state[0] is the state for
  // the innermost stage.

  // First, we update the internal state of the filter using
  // Equation (14) from [2] (shown in Figure 2).

  // We only need to smooth in the case where the coefficients have
  // changed. Try and avoid a per-sample branch for clients that don't need
  // smoothing.
  if (ABSL_PREDICT_FALSE(smoothing_samples_ > 0)) {
    --smoothing_samples_;
    if (smoothing_samples_ > 0) {
      // Smooth in the regular case.
      reflection_smoother_.ProcessSample(reflection_target_, &reflection_);
      // Stability enforcement is needed for 32-bit precision scalar types.
      internal::ForceStability(&reflection_);
      tap_gains_smoother_.ProcessSample(tap_gains_target_, &tap_gains_);
    } else {
      // The coefficients are close enough to their targets that we stop
      // smoothing. Set them to their targets so that we get exactly the
      // requested filter.
      reflection_ = reflection_target_;
      tap_gains_target_ = tap_gains_;
    }
    // TODO: Computing the scattering coefficients is probably the
    // most expensive part of the ProcessSample() function due to the square
    // root. Consider a lookup table.
    internal::GetScatteringCoefficients(reflection_, &scattering_);
  }

  // TODO: Make loops increment upwards for more potential compiler
  // gains.

  // The compiler should optimize away this if statement at compile time.
  if (kFixedNumChannels == Eigen::Dynamic) {
    // When the number of channels is determined dynamically at run time, loop
    // over the channels. This tends to be faster than vectorizing over the
    // channels with Eigen.
    ABSL_DCHECK_EQ(Traits::AsEigenArray(input).rows(), num_channels_);
    Traits::AsMutableEigenArray(output)->resize(num_channels_);
    ABSL_DCHECK_EQ(Traits::AsEigenArray(input).innerStride(), 1)
        << "Cannot operate on map with inner stride.";
    ABSL_DCHECK_EQ(Traits::AsMutableEigenArray(output)->innerStride(), 1)
        << "Cannot operate on map with inner stride.";

    const InputScalarType* input_data = Traits::GetData(input);
    for (int channel = 0; channel < num_channels_; ++channel) {
      AccumType stage_input = input_data[channel];
      for (int i = filter_order_ - 1; i >= 0; --i) {
        state_(channel, i + 1) =
            reflection_[i] * stage_input + scattering_[i] * state_(channel, i);
        stage_input *= scattering_[i];
        stage_input -= reflection_[i] * state_(channel, i);
      }
      state_(channel, 0) = stage_input;
    }
  } else if (kFixedNumChannels == 1) {
    // Optimize the known-single-channel case; 0 index optimization, no loop.
    ABSL_DCHECK_EQ(Traits::AsEigenArray(input).rows(), 1);
    Traits::AsMutableEigenArray(output)->resize(1);
    const InputScalarType* input_data = Traits::GetData(input);

    constexpr int channel = 0;
    AccumType stage_input = input_data[channel];
    for (int i = filter_order_ - 1; i >= 0; --i) {
      state_(channel, i + 1) =
          reflection_[i] * stage_input + scattering_[i] * state_(channel, i);
      stage_input *= scattering_[i];
      stage_input -= reflection_[i] * state_(channel, i);
    }
    state_(channel, 0) = stage_input;
  } else {
    // When the number of channels is fixed at compile time, it is advantageous
    // to vectorize over channels with Eigen. Particularly, we leverage Eigen's
    // fast matrix-vector multiplication for parts of this computation.
    ABSL_DCHECK_EQ(num_channels_, kFixedNumChannels);
    ABSL_DCHECK_EQ(Traits::AsEigenArray(input).rows(), kFixedNumChannels);
    ABSL_DCHECK_EQ(Traits::AsEigenArray(input).innerStride(), 1)
        << "Cannot operate on map with inner stride.";
    ABSL_DCHECK_EQ(Traits::AsMutableEigenArray(output)->innerStride(), 1)
        << "Cannot operate on map with inner stride.";

    Eigen::Matrix<AccumType, kFixedNumChannels, 1> stage_input;
    stage_input =
        Traits::AsEigenArray(input).matrix().template cast<AccumType>();

    for (int i = filter_order_ - 1; i >= 0; --i) {
      state_.col(i + 1) =
          reflection_[i] * stage_input + scattering_[i] * state_.col(i);
      stage_input *= scattering_[i];
      stage_input -= reflection_[i] * state_.col(i);
    }
    state_.col(0) = stage_input;
  }
  // Finally, scale by each output tap. Equation 17 [1].
  Traits::AsMutableEigenArray(output)->matrix().noalias() =
      state_ * tap_gains_.matrix();
}

}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_LADDER_FILTER_INL_H_
