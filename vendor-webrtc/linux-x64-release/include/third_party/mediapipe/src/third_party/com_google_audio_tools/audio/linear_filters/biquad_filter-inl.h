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

#ifndef AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_INL_H_
#define AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_INL_H_

#include "audio/linear_filters/biquad_filter.h"

namespace linear_filters {

template <typename _SampleType>
void BiquadFilter<_SampleType>::Init(
    int num_channels, const BiquadFilterCoefficients& coeffs) {
  if (kNumChannelsAtCompileTime == Eigen::Dynamic) {
    ABSL_CHECK_GE(num_channels, 1);
  } else {
    ABSL_CHECK_EQ(num_channels, kNumChannelsAtCompileTime);
  }
  ABSL_CHECK_NE(coeffs.a[0], 0.0) << "Filter coefficient a0 cannot be zero.";
  num_channels_ = num_channels;
  // Scale the coefficients by 1 / filter_coeff_a[0].
  double scale = 1.0 / coeffs.a[0];
  feedforward0_ = scale * coeffs.b[0];
  feedforward12_ << scale * coeffs.b[1], scale * coeffs.b[2];
  feedback12_ << scale * coeffs.a[1], scale * coeffs.a[2];
  // state_ is a 2-sample sliding buffer holding the last two time steps of
  // filter state s[n - 1] and s[n - 2].
  Reset();
}

template <typename _SampleType>
template <typename InputType>
void BiquadFilter<_SampleType>::SetSteadyStateCondition(
    const InputType& initial_value) {
  // To get the correct state, we use the formulae for the sample updates
  //   s[n] = x[n] - a1 * s[n - 1] - a2 * s[n - 2] and
  //   y[n] = b0 * s[n] + b1 * s[n - 1] + b2 * s[n - 2]
  // and note that at steady state for a DC input, s[n - 1] = s[n - 2] := s.
  // Noting that the feedforward update does not change the state, this gives us
  //   s = x[n] - a1 * s - a2 * s
  // which is equivalent to
  //   s = x[n] / (1 + a1 + a2).
  Eigen::Matrix<AccumType, Eigen::Dynamic, 1> initial =
      Traits::AsEigenArray(initial_value / (feedback12_.sum() + 1))
      .template cast<AccumType>();
  state_.col(0) = initial;
  state_.col(1) = initial;
}

template <typename _SampleType>
void BiquadFilter<_SampleType>::Reset() {
  ABSL_CHECK_GE(num_channels_, 1) << "Reset() called before Init().";
  state_ = Traits::BiquadStateType::Zero(num_channels_, 2);
}

template <typename _SampleType>
template <typename InputType, typename OutputType>
void BiquadFilter<_SampleType>::ProcessSample(const InputType& input,
                                              OutputType* output) {
  // Let x[n] denote the input, y[n] the output, and s[n] the filter state.
  // The biquad filter is updated according to direct form 2 structure as
  //   s[n] = x[n] - a1 * s[n - 1] - a2 * s[n - 2],
  //   y[n] = b0 * s[n] + b1 * s[n - 1] + b2 * s[n - 2].
  // [The above formulas assume that the filter coefficients have been
  // normalized such that a0 = 1, which is done in Init().] The state_ array
  // is a 2-sample sliding buffer to record the previous states s[n - 1] and
  // s[n - 2].

  using InputScalarType = typename Traits::template
      GetScalarType<InputType>::Type;
  using OutputScalarType = typename Traits::template
      GetScalarType<OutputType>::Type;
  constexpr int kFixedNumChannels =
      Traits::template GetFixedNumChannels<InputType, OutputType>();
  ABSL_DCHECK_GE(num_channels_, 1) << "ProcessSample() called before Init().";
  ABSL_DCHECK(output != nullptr);

  // state_.col(0) is s[n - 1] and state_.col(1) is s[n - 2].
  // Correspondingly, feedforward12_ is the column vector (b1, b2); the
  // matrix-vector product state_ * feedforward12_ computes
  // s[n - 1] * b1 + s[n - 2] * b2.
  // Similarly, feedback12_ is the column vector (a1, a2).

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
    OutputScalarType* output_data = Traits::GetMutableData(output);
    for (int channel = 0; channel < num_channels_; ++channel) {
      // Compute the next state s[n] = x[n] - a1 * s[n - 1] - a2 * s[n - 2].
      const AccumType next_state = input_data[channel] -
          (state_.row(channel) * feedback12_)[0];
      // Compute the output y[n] = b0 * s[n] + b1 * s[n - 1] + b2 * s[n - 2].
      output_data[channel] = next_state * feedforward0_ +
          (state_.row(channel) * feedforward12_)[0];
      // Update state by sliding.
      state_(channel, 1) = state_(channel, 0);
      state_(channel, 0) = next_state;
    }
  } else if (kFixedNumChannels == 1) {
    // Optimize the known-single-channel case; 0 index optimization, no loop.
    ABSL_DCHECK_EQ(Traits::AsEigenArray(input).rows(), 1);
    Traits::AsMutableEigenArray(output)->resize(1);
    const InputScalarType* input_data = Traits::GetData(input);
    OutputScalarType* output_data = Traits::GetMutableData(output);
    constexpr int channel = 0;
    // Compute the next state s[n] = x[n] - a1 * s[n - 1] - a2 * s[n - 2].
    const AccumType next_state = input_data[channel] -
        (state_.row(channel) * feedback12_)[0];
    // Compute the output y[n] = b0 * s[n] + b1 * s[n - 1] + b2 * s[n - 2].
    output_data[channel] = next_state * feedforward0_ +
        (state_.row(channel) * feedforward12_)[0];
    // Update state by sliding.
    state_(channel, 1) = state_(channel, 0);
    state_(channel, 0) = next_state;
  } else {
    // When the number of channels is fixed at compile time, it is advantageous
    // to vectorize over channels with Eigen. Particularly, we leverage Eigen's
    // fast matrix-vector multiplication for parts of this computation.
    ABSL_DCHECK_EQ(num_channels_, kFixedNumChannels);
    ABSL_DCHECK_EQ(Traits::AsEigenArray(input).rows(), kFixedNumChannels);
    // Map state_ with fixed rows. Mapping is Aligned only if state_ is
    // "fixed-sized vectorizable," meaning its size is a multiple of 16 bytes.
    ABSL_DCHECK_EQ(Traits::AsEigenArray(input).innerStride(), 1)
        << "Cannot operate on map with inner stride.";
    ABSL_DCHECK_EQ(Traits::AsMutableEigenArray(output)->innerStride(), 1)
        << "Cannot operate on map with inner stride.";

    Eigen::Map<Eigen::Matrix<AccumType, kFixedNumChannels, 2>,
        ((sizeof(AccumType) * kFixedNumChannels * 2) % 16 == 0)
        ? Eigen::Aligned : Eigen::Unaligned> state_map(
            state_.data(), kFixedNumChannels, 2);
    Eigen::Matrix<AccumType, kFixedNumChannels, 1> next_state(
        kFixedNumChannels);

    // Compute the next state s[n] = x[n] - a1 * s[n - 1] - a2 * s[n - 2].
    next_state.noalias() =
        Traits::AsEigenArray(input).matrix().template cast<AccumType>() -
        state_map * feedback12_;
    // Compute the output y[n] = b0 * s[n] + b1 * s[n - 1] + b2 * s[n - 2].
    Traits::AsMutableEigenArray(output)->matrix().noalias() =
        (next_state * feedforward0_ +
        state_map * feedforward12_).template cast<OutputScalarType>();
    // Update state by sliding.
    state_map.col(1) = state_map.col(0);
    state_map.col(0) = next_state;
  }
}


template <typename _SampleType>
void BiquadFilterCascade<_SampleType>::Init(
    int num_channels, const BiquadFilterCascadeCoefficients& all_coefficients) {
  ABSL_DCHECK_GE(num_channels, 1);
  ABSL_DCHECK_GE(all_coefficients.coeffs.size(), 1);
  num_channels_ = num_channels;
  filters_.resize(all_coefficients.coeffs.size());
  for (int i = 0; i < filters_.size(); ++i) {
    filters_[i].Init(num_channels_, all_coefficients.coeffs[i]);
  }
}

template <typename _SampleType>
void BiquadFilterCascade<_SampleType>::Reset() {
  for (auto& filter : filters_) {
    filter.Reset();
  }
}

template <typename _SampleType>
template <typename InputType, typename OutputType>
void BiquadFilterCascade<_SampleType>::ProcessBlock(const InputType& input,
                                             OutputType* output) {
  ABSL_DCHECK_GE(num_channels_, 1);
  filters_[0].ProcessBlock(input, output);
  const int filters_size = filters_.size();
  for (int i = 1; i < filters_size; ++i) {
    filters_[i].ProcessBlock(*output, output);
  }
}

template <typename _SampleType>
template <typename InputType, typename OutputType>
void BiquadFilterCascade<_SampleType>::ProcessSample(const InputType& input,
                                              OutputType* output) {
  ABSL_DCHECK_GE(num_channels_, 1);
  filters_[0].ProcessSample(input, output);
  const int filters_size = filters_.size();
  for (int i = 1; i < filters_size; ++i) {
    filters_[i].ProcessSample(*output, output);
  }
}

}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_INL_H_
