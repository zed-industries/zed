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

// A biquad filtering library for single or multi-channel signal processing.


// Biquad filter with transfer function
//
//         b0 + b1 z^-1 + b2 z^-2
//  H(z) = ----------------------.
//         a0 + a1 z^-1 + a2 z^-2
//
// In the time domain, the nth output sample is
//
//  y[n] = (b0 x[n] + b1 x[n - 1] + b2 x[n - 2]
//          - a1 y[n - 1] - a2 y[n - 2]) / a0.
//
// The filter state is initially zero, i.e., the left boundary is handled by
// zero padding extension. The filter is implemented using direct form 2
// structure [https://en.wikipedia.org/wiki/Digital_biquad_filter].
//
// CoefficientType specifies the type for the filter coefficients b0,
// b1, b2, a0, a1, a2. It may be float, double, complex<float>, or
// complex<double>.
//
// SampleType specifies the type for the input and output samples. Use a scalar
// numeric type float, double, complex<float>, or complex<double> for a
// single-channel signal.
//
// For a multichannel signal, use Eigen::ArrayXf, Eigen::ArrayXd,
// Eigen::ArrayXcf, or Eigen::ArrayXcd. Or if the number of channels is known at
// compile time, use a fixed-sized Eigen type like Eigen::Array2f for a stereo
// signal for better computational efficiency.
//
// Note: BiquadFilter is stateful, if you have a streaming audio application,
//       it is important to use the same filter for each block of samples
//       rather than repeatedly creating a new one.
//
// Examples:
//   /* Single-channel float-valued samples and float-valued coefficients. */
//   BiquadFilter<float> filter;
//   filter.Init(1, {{b0, b1, b2}, {a0, a1, a2}});
//   while (...) {
//     vector<float> input_samples = GetNextBlockOfInputSamples();
//     vector<float> output_samples;
//     filter.ProcessBlock(input_samples, &output_samples);
//     /* Do something with output_samples. */
//   }
//
//   /* Multichannel signal, where each sample is an Eigen::ArrayXf. */
//   BiquadFilter<Eigen::ArrayXf> filter;
//   filter.Init(num_channels, {{b0, b1, b2}, {a0, a1, a2}});
//   while (...) {
//     /* input_samples has num_channels rows. */
//     Eigen::ArrayXXf input_samples = ...
//     Eigen::ArrayXXf output_samples;
//     filter.ProcessBlock(input_samples, &output_samples);
//     ...
//   }
//
//   /* Filter optimized for 7-channel signal using fixed number of rows. */
//   BiquadFilter<float, Eigen::Array<float, 7, 1>> filter;
//   filter.Init(7, {{b0, b1, b2}, {a0, a1, a2}});
//   while (...) {
//     Eigen::Array<float, 7, Eigen::Dynamic> input_samples = ...
//     Eigen::Array<float, 7, Eigen::Dynamic> output_samples;
//     filter.ProcessBlock(input_samples, &output_samples);
//     ...
//   }

#ifndef AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_H_
#define AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_H_

#include <memory>
#include <vector>

#include "audio/dsp/types.h"
#include "audio/linear_filters/biquad_filter_coefficients.h"
#include "audio/linear_filters/filter_traits.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {

template <typename _SampleType>
class BiquadFilter {
  using Traits = typename internal::FilterTraits<
     _SampleType, internal::IsEigenType<_SampleType>::Value>;

 public:
  using CoefficientType = typename Traits::CoefficientType;
  using SampleType = typename Traits::SampleType;
  using ScalarType = typename Traits::template GetScalarType<SampleType>::Type;
  static constexpr int kNumChannelsAtCompileTime =
      Traits::kNumChannelsAtCompileTime;
  using AccumType = typename Traits::AccumType;

  // Necessary to align fixed-sized Eigen members, see
  // http://eigen.tuxfamily.org/dox/group__TopicStructHavingEigenMembers.html
  EIGEN_MAKE_ALIGNED_OPERATOR_NEW

  BiquadFilter(): num_channels_(0 /* Mark filter as uninitialized. */) {}

  // Initialize the filter with zero state, specifying number of channels and
  // filter coefficients. This function may be called more than once.
  //
  // The number of channels must be compatible with SampleType. If SampleType is
  // scalar, num_channels must be 1. If SampleType is an Eigen type with a fixed
  // number of rows such as Array2f, then num_channels must match
  // SampleType::RowsAtCompileTime.
  void Init(int num_channels, const BiquadFilterCoefficients& coeffs);

  // Reset the filter to zero state (left boundary is extended by zero padding).
  void Reset();

  // This sets the state of the filter as if it has been processing a DC signal
  // equal to initial_value forever.
  template <typename InputType>
  void SetSteadyStateCondition(const InputType& initial_value);

  int num_channels() const { return num_channels_; }

  // Process a block of samples. For streaming, pass successive nonoverlapping
  // blocks of samples to this function. Init() must be called before calling
  // this function. In-place computation is supported (&input = output).
  // Data must be contiguous in memory.
  //
  // If SampleType is scalar, the input and output must be a 1D array-like type
  // like Eigen::ArrayXf or vector<float>.
  //
  // If SampleType is multichannel, then input and output must be 2D Eigen types
  // with contiguous column-major data like ArrayXXf or MatrixXf, where the
  // number of rows equals GetNumChannels().
  //
  // Example use:
  //   /* Scalar filter. */
  //   BiquadFilter<float> filter;
  //   filter.Init(1, {{b0, b1, b2}, {a0, a1, a2}});
  //   while (...) {
  //     vector<float> input = /* Get next block of input samples. */
  //     vector<float> output;
  //     filter.ProcessBlock(input, &output);
  //     /* Do something with output_samples. */
  //   }
  //
  //   /* Multichannel filter. */
  //   BiquadFilter<ArrayXf> filter;
  //   filter.Init(num_channels, {{b0, b1, b2}, {a0, a1, a2}});
  //   while (...) {
  //     ArrayXXf input = ...
  //     ArrayXXf output;
  //     filter.ProcessBlock(input, &output);
  //     ...
  //   }
  template <typename InputType, typename OutputType>
  void ProcessBlock(const InputType& input, OutputType* output) {
    ABSL_CHECK_GE(num_channels_, 1) << "ProcessBlock() called before Init().";
    Traits::ProcessBlock(this, input, output);
  }

  // Process one sample, taking one input sample and producing one output
  // sample. Use this light function instead of ProcessBlock() to process an
  // individual sample (e.g. to avoid buffers in streaming applications).
  //
  // The input sample should be SampleType or something convertible SampleType.
  // For a multichannel filter, the input and output must be types with
  // contiguous data like ArrayXf. Init() must be called before calling this
  // function.
  //
  // Note that ProcessSample will result in a memory corruption if used on a
  // type with innerStride != 1. This can be caught by running in debug mode.
  //
  // Example use:
  //   /* Scalar filter. */
  //   float input_sample = ...;
  //   float output_sample;
  //   filter.ProcessSample(input_sample, &output_sample);
  //
  //   /* Multichannel filter. */
  //   ArrayXf input_sample = ...;
  //   ArrayXf output_sample;
  //   filter.ProcessSample(input_sample, &output_sample);
  template <typename InputType, typename OutputType>
  void ProcessSample(const InputType& input, OutputType* output);

 private:
  int num_channels_;
  // Feedforward filter coefficient b0.
  CoefficientType feedforward0_;
  // Column vector (b1, b2).
  Eigen::Matrix<CoefficientType, 2, 1> feedforward12_;
  // Column vector (a1, a2).
  Eigen::Matrix<CoefficientType, 2, 1> feedback12_;
  // Matrix of size num_channels-by-2, a 2-sample sliding buffer remembering
  // the previous states s[n - 1] and s[n - 2].
  typename Traits::BiquadStateType state_;
};

// The BiquadFilterCascade is a filter that processes multiple BiquadFilters in
// cascade (output of each is input to the next). It can be used in the exact
// same way that a BiquadFilter is used, with one exception: Init takes a
// BiquadFilterCascadeCoefficients instead of a BiquadFilterCoefficients.
template <typename _SampleType>
class BiquadFilterCascade {
  using Traits = typename internal::FilterTraits<
     _SampleType, internal::IsEigenType<_SampleType>::Value>;

 public:
  using CoefficientType = typename Traits::CoefficientType;
  using SampleType = typename Traits::SampleType;
  static constexpr int kNumChannelsAtCompileTime =
      Traits::kNumChannelsAtCompileTime;
  using ScalarType = typename Traits::template GetScalarType<_SampleType>::Type;

  BiquadFilterCascade() : num_channels_(0) {}

  // Adds and initializes a BiquadFilter to the cascade using the parameters,
  // coefficients. The state of all filters is reset when a new stage is added.
  void Init(int num_channels,
            const BiquadFilterCascadeCoefficients& all_coefficients);

  // Resets the state of all stages of the filter.
  void Reset();

  // Data must be contiguous in memory.
  template <typename InputType, typename OutputType>
  void ProcessBlock(const InputType& input, OutputType* output);

  template <typename InputType, typename OutputType>
  void ProcessSample(const InputType& input, OutputType* output);

  int num_channels() const { return num_channels_; }

 private:
  int num_channels_;
  std::vector<BiquadFilter<SampleType>> filters_;
};

}  // namespace linear_filters

// Hide implementation in another header.
#include "audio/linear_filters/biquad_filter-inl.h"   // IWYU pragma: export

#endif  // AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_H_
