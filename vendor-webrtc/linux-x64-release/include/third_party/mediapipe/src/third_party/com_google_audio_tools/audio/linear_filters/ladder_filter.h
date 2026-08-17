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

// An implementation of the power-normalized ladder filter from
// [1] Gray & Markel (1973), Digital Lattice and Ladder Filter Synthesis:
//
// This implementation supports changing coefficients. The coefficients are
// smoothed with a cutoff of 0.5 percent of the sample rate. This is slow enough
// to prevent audible artifacts but quick enough to maintain a responsive
// filter.
//
// Most of the variables in the coefficient computation are named rather
// cryptically, but their naming is consistent with the formulae in Gray &
// Markel (1973). Equation numbers throughout are written in parentheses
// followed by the reference number in square brackets.
//
// Gray & Markel also demonstrate that by introducing a power normalization,
// ladder filters structures become stable under time-varying filter
// coefficients. Moreover, these power-normalized ladder filters support
// instantaneous changing of filter coefficients without fluctuations in the
// energy signal, though some coefficient smoothing is done to keep higher order
// signal derivatives continuous. The details of the derivation can be found
// here:
// [2] Gray & Markel (1975), A Normalized Digital Filter Structure:

//
// Because our the lattice stage of our filter is implemented as a scattering
// junction, the internal state of the ladder filter refers to these 'k'
// variables as reflection coefficients and the values sqrt(1 - k^2) as
// scattering coefficients as described here:
// https://ccrma.stanford.edu/~jos/pasp/Normalized_Scattering_Junctions.html

#ifndef AUDIO_LINEAR_FILTERS_LADDER_FILTER_H_
#define AUDIO_LINEAR_FILTERS_LADDER_FILTER_H_

#include <cmath>
#include <cstdlib>
#include <cstdint>
#include <type_traits>
#include <vector>

#include "audio/linear_filters/biquad_filter.h"
#include "audio/linear_filters/biquad_filter_coefficients.h"
#include "audio/linear_filters/biquad_filter_design.h"
#include "audio/linear_filters/filter_traits.h"
#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {

// Note for multichannel processing:
// If you know the number of channels at compile time (for example, if your
// device has 4 channels), you can get substantial speedups by doing
//   LadderFilter<Eigen::Array<ScalarType, 4, 1>> ladder_filter;
// instead of
//   LadderFilter<Eigen::ArrayXf> ladder_filter;
// TODO: Look into a possible alignment issue causing LadderFilter
// to be slower for ArrayXf than for ArrayNf for small numbers of channels
// despite special-case templating on the channel number in filter_traits.h.
template <typename _SampleType>
class LadderFilter {
 public:
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

  LadderFilter()
    : num_channels_(0),
      filter_order_(0),
      initialized_(false),
      smoothing_samples_(0),
      smoothing_samples_max_(0) {}

  // Initialize the ladder coefficients directly. The order of the coefficients
  // is such that outermost stage (leftmost, in the figures in Gray & Markel)
  // has the highest index in the coeffs_k and coeffs_v array. coeffs_v must
  // have one more element than coeffs_k. All elements of coeffs_k must have an
  // absolute value of less than one or this will ABSL_CHECK fail.
  // The implemented transfer function is
  //   G(z) = sum_{m=0}^M coeffs_v[m] * z B_m(z) / A_M(z),
  // where M := coeffs_k.size() is the filter order and for m = 0, ..., M - 1,
  //   [ A_{m+1}(z) ] = [ sqrt(1 - k[m]^2),            -k[m] ] * [ A_m(z) ],
  //   [ B_{m+1}(z) ]   [ k[m],      sqrt(1 - k[m]^2) * z^-1 ]   [ B_m(z) ]
  // and A_0(z) = 1, B_0(z) = z^-1.
  //
  // Internally, the ladder filter refers to k and sqrt(1 - k^2) as
  // reflection_ and scattering_, respectively.
  //
  // It's a little difficult to refer to the stages as 'first' or 'last'
  // because the leftmost lattice stage is the first one to see the incoming
  // signal, but it is also the last to have a nonzero output (which happens
  // coeffs_k.size() samples later). We will instead refer to the leftmost and
  // rightmost (w.r.t. the paper figures) stages this the as outermost and
  // innermost, respectively.
  //
  // NOTE: If your ladder filter implements an allpole filter, only the final
  // element of coeffs_v will be nonzero. coeffs_v represents the v variable in
  // the paper, not v-hat.
  void InitFromLadderCoeffs(int num_channels,
                            const std::vector<double>& coeffs_k,
                            const std::vector<double>& coeffs_v);

  // Change filter coefficients without resetting the state of the filter.
  // The passed-in coefficients must be such that the order of the filter does
  // not change.
  //
  // For radical filter changes, it is better to interpolate in smaller steps.
  // Ladder filter interpolation insures stability, but does not guarantee
  // a nice transition of filter shapes. You can control the transition of
  // filter shapes better by computing an update for the filter shape once per
  // audio block and updating gradually. See examples/ladder_filter_autowah.cc
  // for an example.
  //
  // Note that you only pay the additional cost of the coefficient smoothing in
  // the several milliseconds after requesting a coefficient change. Once the
  // coefficients settle, the filter is cheap again.

  void ChangeLadderCoeffs(const std::vector<double>& coeffs_k,
                          const std::vector<double>& coeffs_v);

  // Convert a rational transfer function of arbitrary order to coefficients
  // for a ladder filter.
  void InitFromTransferFunction(int num_channels,
                                const std::vector<double>& coeffs_b,
                                const std::vector<double>& coeffs_a);

  // Change filter coefficients without resetting the state of the filter.
  // The passed-in coefficients must be such that the order of the filter does
  // not change.
  void ChangeCoeffsFromTransferFunction(const std::vector<double>& coeffs_b,
                                        const std::vector<double>& coeffs_a);

  // Clears the state but not the computed coefficients.
  void Reset();

  int num_channels() const { return num_channels_; }

  // Process a block of samples. For streaming, pass successive nonoverlapping
  // blocks of samples to this function. Must be initialized before calling
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
  //   LadderFilter<float> filter;
  //   filter.InitFromTransferFunction(1, {b0, b1, b2}, {a0, a1, a2});
  //   while (...) {
  //     vector<float> input = /* Get next block of input samples. */
  //     vector<float> output;
  //     filter.ProcessBlock(input, &output);
  //     /* Do something with output_samples. */
  //   }
  //
  //   /* Multichannel filter. */
  //   LadderFilter<ArrayXf> filter;
  //   filter.InitFromTransferFunction(1, {b0, b1, b2}, {a0, a1, a2});
  //   while (...) {
  //     ArrayXXf input = ...
  //     ArrayXXf output;
  //     filter.ProcessBlock(input, &output);
  //     ...
  //   }
  template <typename InputType, typename OutputType>
  void ProcessBlock(const InputType& input, OutputType* output) {
    ABSL_CHECK(initialized_) << "ProcessBlock() called before initialization.";
    Traits::ProcessBlock(this, input, output);
  }

  // Process one sample, taking one input sample and producing one output
  // sample. Use this light function instead of ProcessBlock() to process an
  // individual sample (e.g. to avoid buffers in streaming applications).
  //
  // The input sample should be SampleType or something convertible SampleType.
  // For a multichannel filter, the input and output must be types with
  // contiguous data like ArrayXf. Must be initialized before calling this
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

  // Number of memory elements in the filter.
  int filter_order_;

  // Is the filter initialized?
  bool initialized_;

  // Filter coefficients. reflection_ and scattering_ represent the poles of the
  // filter and must have a magnitude less than one for stability. tap_gains_
  // represents the zeros and does not have an impact on stablity, it is the
  // weight placed on the tap of each lattice stage's output.
  // reflection_ = k, using the notation from [1, 2] and
  // scattering_ = sqrt(1 - k^2).
  Eigen::Matrix<CoefficientType, Eigen::Dynamic, 1> reflection_;
  Eigen::Matrix<CoefficientType, Eigen::Dynamic, 1> scattering_;
  // tap_gains_ is v (not v-hat) in the paper.
  Eigen::Matrix<CoefficientType, Eigen::Dynamic, 1> tap_gains_;

  BiquadFilter<Eigen::Matrix<CoefficientType, Eigen::Dynamic, 1>>
      reflection_smoother_;
  BiquadFilter<Eigen::Matrix<CoefficientType, Eigen::Dynamic, 1>>
      tap_gains_smoother_;
  // The coefficient smoothing will always be moving towards the "target"
  // values.
  Eigen::Matrix<CoefficientType, Eigen::Dynamic, 1> reflection_target_;
  Eigen::Matrix<CoefficientType, Eigen::Dynamic, 1> tap_gains_target_;

  typename Traits::LadderStateType state_;

  // The remaining number of samples before we can assume coefficient values
  // are at steady state and stop smoothing.
  int smoothing_samples_;
  // The number of samples it takes for the impulse response of the
  // smoothing filter to reach approximately zero.
  int smoothing_samples_max_;
};

}  // namespace linear_filters

// Hide implementation in another header.
#include "audio/linear_filters/ladder_filter-inl.h"   // IWYU pragma: export

#endif  // AUDIO_LINEAR_FILTERS_LADDER_FILTER_H_
