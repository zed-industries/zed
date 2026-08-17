/*
 * Copyright 2020-2021 Google LLC
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

// WARNING: This library is DEPRECATED as of 2021-01-23.
//
// Fast audio resampling by a rational factor.
//
// RationalFactorResampler performs resampling using a polyphase FIR filter. We
// use the Eigen library to leverage SIMD optimization, computing each output
// sample as a dot product.
//
//
// Migration notes:
//
// RationalFactorResampler is superseded by QResampler, audio/dsp/resampler_q.h.
// Compared to RationalFactorResampler, QResampler is easier to use without
// allocations after construction, has a simpler interface, and adds
// multichannel support.
//
//  * Same output: QResampler produces the same resampled output as
//    RationalFactorResampler up to near machine precision, so migrating should
//    have negligible effect on how signals are resampled. The underlying kernel
//    and filtering is the same as before, yet removing the cost of allocating
//    the output improves benchmarks by ~10%.
//
//  * Construction: QResampler has a revised (and usually easier) construction
//    interface. Instead of DefaultResamplingKernel, set QResamplerParams. To
//    convert DefaultResamplingKernel settings to equivalent QResamplerParams,
//    see "Conversion from RationalFactorResampler kernel params" at the top of
//    audio/dsp/resampler_q.h.
//
//  * Passing Eigen containers: QResampler's `ProcessSamples()` and `Flush()`
//    methods have the same meaning as in RationalFactorResampler. However,
//    QResampler supports more container types for the input and ouput args,
//    including most Eigen containers. So QResampler does not have
//    `ProcessSamplesEigen()` or `FlushEigen()`; use `ProcessSamples()` and
//    `Flush()` instead.
//
//  * QResampler otherwise has a compatible API. `Reset()` and
//    `ResetFullyPrimed()` have the same meaning as in RationalFactorResampler.
//
// See also the top-level comment of audio/dsp/resampler_q.h for further notes
// about how to use QResampler's new features, such as multichannel resampling.

#ifndef AUDIO_DSP_RESAMPLER_RATIONAL_FACTOR_H_
#define AUDIO_DSP_RESAMPLER_RATIONAL_FACTOR_H_

#include <type_traits>

#include "audio/dsp/number_util.h"
#include "audio/dsp/resampler.h"
#include "audio/dsp/types.h"
#include "glog/logging.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// Base class for resampling kernel functors.
class ResamplingKernel {
 public:
  ResamplingKernel(
      double input_sample_rate, double output_sample_rate, double radius);

  virtual ~ResamplingKernel() {}

  // Evaluate the kernel at (possibly non-integer) x, where x is in units of
  // input samples.
  virtual double Eval(double x) const = 0;

  // Return whether the ResamplingKernel was initialized with valid parameters.
  virtual bool Valid() const = 0;

  double input_sample_rate() const { return input_sample_rate_; }

  double output_sample_rate() const { return output_sample_rate_; }

  // Get the window radius in units of input samples.
  double radius() const { return radius_; }

 private:
  double input_sample_rate_;
  double output_sample_rate_;
  double radius_;
};

// Functor for a Kaiser-windowed sinc resampling kernel. Check the validity of
// parameters by calling Valid() before use.
class DefaultResamplingKernel: public ResamplingKernel {
 public:
  // Convenience constructor for resampling from input_sample_rate to
  // output_sample_rate using the same kernel as libresample's low-quality mode.
  // The second constructor below exposes all the parameters.
  //
  // If input_sample_rate <= output_sample_rate (upsampling), then the kernel
  // radius is set to 5 input samples and the anti-aliasing cutoff frequency to
  // 90% of the input Nyquist rate.
  //
  // Otherwise if input_sample_rate > output_sample_rate (downsampling), the
  // kernel radius set to 5 *output* samples and the cutoff to 90% of the
  // *output* Nyquist rate.
  //
  // For either upsampling or downsampling, the Kaiser window beta parameter is
  // 6.0 for -63dB stopband ripple [see second constructor for details].
  //
  // The above default parameters match libresample's low-quality mode. The same
  // parameters but with radius of 17 input/output samples corresponds to
  // libresample's high-quality mode.
  DefaultResamplingKernel(double input_sample_rate, double output_sample_rate);

  // Constructor allowing to set radius, cutoff, and kaiser_beta.
  //
  // radius is a positive value specifying the support radius of the kernel in
  // units of input samples. The radius need not be integer.
  //
  // cutoff is the anti-aliasing cutoff frequency, viewing the kernel as the
  // impulse response for a lowpass filter. A typical setting is
  // cutoff = 0.45 * min(input_sample_rate, output_sample_rate).
  //
  // kaiser_beta is the positive beta parameter for the Kaiser window shape,
  // where larger value yields a wider transition band and stronger attenuation.
  //
  //   beta  Stopband     B
  //  2.120    -30 dB  0.79
  //  3.384    -40 dB  1.13
  //  4.538    -50 dB  1.48
  //  5.658    -60 dB  1.82
  //  6.000    -63 dB  1.93
  //  6.764    -70 dB  2.17
  //  7.865    -80 dB  2.51
  //
  // The above table lists for different beta the maximum stopband gain [first
  // sidelobe peak] and a quantity B related to the width of the transition
  // band. In units of Hz, the transition band width is
  //   transition band width = B * input_sample_rate / radius.
  DefaultResamplingKernel(double input_sample_rate, double output_sample_rate,
                          double radius, double cutoff, double kaiser_beta);

  double Eval(double x) const override;

  bool Valid() const override {
    return valid_;
  }

 private:
  // Initialize from constructor parameters.
  void Init(double cutoff, double kaiser_beta);
  // Evaluate the Kaiser window at x, where x is in units of input samples.
  // The nonzero support of the window is |x| <= radius_.
  //
  // NOTE: The window intentionally has nonzero value at the endpoints
  // |x| = radius. This helps reduce the amplitude of the first side lobe.
  double KaiserWindow(double x) const;

  bool valid_ = false;
  double normalization_;
  double radians_per_sample_;
  double kaiser_beta_;
  double kaiser_denominator_;
};

// Resampler for rational resampling factors. ValueType may be float, double,
// complex<float>, or complex<double>.
//
// The input audio of sample rate F is interpolated according to
//   x(t) = sum_n x[n] h(t F - n)
// where h(n) is a windowed sinc implemented with ResamplingKernel and
// having nonzero value only in |n| <= r [see e.g. Julius Smith's discussion
// https://ccrma.stanford.edu/~jos/resample]. The support implies that x(t)
// depends only on x[n] for sample indices n from ceil(t F - r) to floor(t F +
// r), and the number of nonzero taps is bounded by 2 r + 1:
//   floor(t F + r) - ceil(t F - r) + 1
//       <= (t F + r) - (t F - r) + 1
//       = 2 r + 1.
//
// To resample with output sample rate F', the interpolant x(t) is sampled as
//   x'[m] = x(m/F') = sum_n x[n] h(m F/F' - n).
// The ratio d = F/F' is the resampling factor. This class assumes that d is
// rational, d = a/b for some small a and b, so that
//   x'[m] = sum_n x[n] h(m a/b - n).
// The expression (m a/b) is the current position in units of input samples. Let
// m a/b = p/b + q, where p and q are integer and 0 <= p < b, so that q is the
// floor of the current position and p/b is the fractional part. Then
//   x'[m] = sum_n x[n] h(p/b + q - n)
//         = sum_k h(p/b + k) x[q - k]  (change of variables k = q - n)
//         = (h_p * x)(q)
// where the last line views the sum as a convolution with filter h_p defined by
//   h_p[k] := h(p/b + k),  p = 0, 1, ..., b - 1.
// The filter is nonzero for k from ceil(-p/b - r) to floor(-p/b + r). Supposing
// that r is a positive integer, and since 0 <= p < b, the range is contained by
// -r <= k <= r.
template <typename ValueType>
class RationalFactorResampler: public Resampler<ValueType> {
 public:
  static_assert(
      std::is_floating_point<typename RealType<ValueType>::Type>::value,
      "ValueType must be a floating point or complex type");
  // Construct a RationalFactorResampler where the resampling factor is obtained
  // from kernel. The resampling factor input_sample_rate / output_sample_rate
  // is approximated by a rational factor a/b where 0 < b <= max_denominator.
  //
  // If the resampling factor is equivalent to a rational with a/b with b <=
  // max_denominator, the resampling factor is represented exactly. This is the
  // case for resampling between many common rates, e.g. between 8kHz, 16kHz,
  // 22.05kHz, 32kHz, 44.1kHz, and 48kHz, with max_denominator >= 640.
  //
  // For arbitrary rates, the actual output sample rate may be up to
  // max(input_sample_rate, output_sample_rate) / (2 * max_denominator - 1) Hz
  // faster or slower than the specified output sample rate. For example,
  // downsampling (input_sample_rate > output_sample_rate) with max_denominator
  // = 500 guarantees error no greater than 0.1% of the input sample rate.
  RationalFactorResampler(const ResamplingKernel& kernel,
                          int max_denominator = 1000) {
    // Skip initialization on invalid parameters.
    if (!kernel.Valid() || max_denominator <= 0) {
      return;
    }
    Init(kernel, RationalApproximation(
        kernel.input_sample_rate() / kernel.output_sample_rate(),
        max_denominator));
  }

  // Construct a RationalFactorResampler where the rational resampling factor is
  // specified directly by factor_numerator / factor_denominator. A factor of
  // 2 / 1 indicates that the signal is being downsampled by a factor of 2.
  RationalFactorResampler(const ResamplingKernel& kernel,
                          int factor_numerator, int factor_denominator) {
    // Skip initialization on invalid parameters.
    if (!kernel.Valid() || factor_numerator <= 0 || factor_denominator <= 0) {
      return;
    }
    const int gcd = GreatestCommonDivisor(factor_numerator, factor_denominator);
    const int reduced_numerator = factor_numerator / gcd;
    const int reduced_denominator = factor_denominator / gcd;

    // Warn if filters_ will be unusually large.
    if (reduced_denominator > 1000) {
      LOG(WARNING) << "Resampling factor "
          << factor_numerator << "/" << factor_denominator
          << " = " << reduced_numerator << "/" << reduced_denominator
          << " is not a ratio of small integers, so a large table of "
          << reduced_denominator << " filters is needed.";
    }
    Init(kernel, {reduced_numerator, reduced_denominator});
  }

  // Construct a RationalFactorResampler where the rational resampling factor is
  // specified directly by factor_numerator / factor_denominator. A factor of
  // 2 / 1 indicates that the signal is being downsampled by a factor of 2.
  RationalFactorResampler(float input_rate_hz, float output_rate_hz,
                          int max_denominator = 1000)
      : RationalFactorResampler(
          DefaultResamplingKernel(input_rate_hz, output_rate_hz),
          max_denominator) {}

  ~RationalFactorResampler() override {}

  int factor_denominator() const { return factor_denominator_; }
  int factor_numerator() const { return factor_numerator_; }
  int radius() const { return radius_; }

  // Implements the `Reset()` method. Resets RationalFactorResampler initial
  // state as if it had just been constructed. Resampler state is set such that
  // the input and output streams are time-aligned, with the first output sample
  // corresponding to the same point in time as the first input sample.
  void ResetImpl() override {
    phase_ = 0;
    delayed_input_.reserve(num_taps_);
    // The number of filter taps is `2 * radius_ + 1`, and for `phase_ = 0`, the
    // filtered output sample is time-aligned with the center tap's input
    // sample. We prime the buffer with `radius_` zeros such that this center
    // tap lands on the first given input sample.
    delayed_input_.assign(radius_, ValueType(0));
  }

  // Like `Reset()`, but sets the Resampler with a "fully primed" delayed input
  // buffer, such that the first output sample can be produced as soon as the
  // first input sample is available.
  //
  // When using this method, and provided that the input buffer size is a
  // multiple of `factor_numerator / gcd(factor_numerator, factor_denominator)`,
  // the output size is always exactly (input.size() * factor_denominator) /
  // factor_numerator.
  //
  // NOTE: When using `ResetFullyPrimed()`, the input and output streams are
  // *not* time aligned. The output stream is delayed by `radius()` input
  // samples, i.e. the filtering latency of the resampler.
  void ResetFullyPrimed() {
    ABSL_DCHECK(valid_);
    phase_ = 0;
    delayed_input_.reserve(num_taps_);
    // Initialize with `num_taps - 1` zeros, so that when the first sample is
    // given, we will have exactly enough to compute the first output sample.
    delayed_input_.assign(num_taps_ - 1, ValueType(0));
  }

  bool ValidImpl() const override {
    return valid_;
  }

  void ProcessSamplesImpl(absl::Span<const ValueType> input,
                          std::vector<ValueType>* output) override {
    output->resize(ComputeOutputSizeFromCurrentState(
        static_cast<int>(input.size())));
    using EigenVectorType = Eigen::Matrix<ValueType, Eigen::Dynamic, 1>;
    Eigen::Map<const EigenVectorType> input_map(input.data(), input.size());
    Eigen::Map<EigenVectorType> output_map(output->data(), output->size());
    ProcessSamplesEigen(input_map, &output_map);
  }

  // These templates allow one dimensional Eigen types to be processed by the
  // resampler.
  template <typename EigenType1, typename EigenType2>
  void ProcessSamplesEigen(const EigenType1& input, EigenType2* output) {
    ABSL_DCHECK(output != nullptr);
    ABSL_DCHECK_LT(static_cast<int>(delayed_input_.size()), num_taps_);
    ABSL_DCHECK_LT(phase_, static_cast<int>(filters_.size()));

    static_assert(std::is_same<typename EigenType1::Scalar,
                               typename EigenType2::Scalar>::value,
                  "input and output must have the same scalar type");

    if (static_cast<int>(delayed_input_.size() + input.size()) < num_taps_) {
      // Because this function should support processing on Eigen generator and
      // expression types, like MatrixXf::Zero(...) that do not have a .data()
      // member, we cannot use functions like insert(input.data(), ...).
      for (int sample = 0; sample < input.size(); ++sample) {
        delayed_input_.push_back(input[sample]);
      }
      // Not enough samples available to produce any output yet.
      return;
    }

    using EigenVectorType = Eigen::Matrix<ValueType, Eigen::Dynamic, 1>;
    Eigen::Map<const EigenVectorType> delayed_input_map(
        delayed_input_.data(), delayed_input_.size());
    int i_end = static_cast<int>(
        delayed_input_.size() + input.size() - num_taps_ + 1);
    // Below, the position in the input is (i + phase_ / factor_denominator) in
    // units of input samples, with phase_ tracking the fractional part.
    int i = 0;
    output->resize(ComputeOutputSizeFromCurrentState(
        static_cast<int>(input.size())));
    int output_samples = 0;

    // Process samples where the filter straddles delayed_input_ and input.
    while (i < static_cast<int>(delayed_input_.size()) && i < i_end) {
      const int num_state = static_cast<int>(delayed_input_map.size() - i);
      const int num_input = num_taps_ - num_state;
      const auto& filter = filters_[phase_];
      ABSL_DCHECK_LT(output_samples, output->size());
      (*output)[output_samples] =
          filter.head(num_state).dot(delayed_input_map.tail(num_state)) +
          filter.tail(num_input).dot(input.matrix().head(num_input));
      ++output_samples;
      i += factor_floor_;
      phase_ += phase_step_;
      if (phase_ >= factor_denominator_) {
        phase_ -= factor_denominator_;
        ++i;
      }
    }

    if (i < static_cast<int>(delayed_input_.size())) {
      // Ran out of input samples before consuming everything in
      // delayed_input_. Discard the samples of delayed_input_ that have been
      // consumed and append the input.
      delayed_input_.erase(delayed_input_.begin(), delayed_input_.begin() + i);
      for (int sample = 0; sample < input.size(); ++sample) {
        delayed_input_.push_back(input[sample]);
      }
      return;
    }

    // Consumed everything in delayed_input_. Now process output samples that
    // depend on only the input.
    i -= delayed_input_map.size();
    i_end -= delayed_input_map.size();
    while (i < i_end) {
      ABSL_DCHECK_LT(output_samples, output->size());
      (*output)[output_samples] =
          filters_[phase_].dot(input.matrix().segment(i, num_taps_));
      ++output_samples;
      i += factor_floor_;
      phase_ += phase_step_;
      if (phase_ >= factor_denominator_) {
        phase_ -= factor_denominator_;
        ++i;
      }
    }

    ABSL_DCHECK_LE(i, static_cast<int>(input.size()));
    delayed_input_.resize(input.size() - i);
    for (size_t sample = 0; sample < delayed_input_.size();  ++sample) {
      delayed_input_[sample] = input[i + sample];
    }
  }

  void FlushImpl(std::vector<ValueType>* output) override {
    // The goal of flushing is to extract all the non-zero
    // outputs. Because ProcessSamples always continues until there
    // are < num_taps_ left to process, by using num_taps_ - 1 zeros,
    // we guarantee that after the call to ProcessSamples,
    // delayed_input contains only zeros.
    const std::vector<ValueType> input(num_taps_ - 1, ValueType(0));
    this->ProcessSamples(input, output);
  }

  // A version of Flush() that supports Eigen types.
  template <typename EigenType>
  void FlushEigen(EigenType* output) {
    this->ProcessSamplesEigen(
        Eigen::Matrix<ValueType, Eigen::Dynamic, 1>::Zero(num_taps_ - 1),
        output);
    this->Reset();
  }

  // Accessors for testing.
  int delayed_input_size() const { return delayed_input_.size(); }
  int phase() const { return phase_; }

 private:
  typedef typename RealType<ValueType>::Type CoefficientType;

  void Init(const ResamplingKernel& kernel, const std::pair<int, int>& factor) {
    factor_numerator_ = factor.first;
    factor_denominator_ = factor.second;
    factor_floor_ = factor_numerator_ / factor_denominator_;  // Integer divide.
    radius_ = ceil(kernel.radius());
    phase_step_ = factor_numerator_ % factor_denominator_;
    num_taps_ = 2 * radius_ + 1;

    filters_.resize(factor_denominator_);
    for (int phase = 0; phase < static_cast<int>(filters_.size()); ++phase) {
      auto& filter = filters_[phase];
      filter.resize(num_taps_);
      const double offset = static_cast<double>(phase) / factor_denominator_;
      for (int k = -radius_; k <= radius_; ++k) {
        // Store the filter backwards so that convolution becomes a dot product.
        filter[radius_ - k] =
            static_cast<CoefficientType>(kernel.Eval(offset + k));
      }
    }
    valid_ = true;
    this->Reset();
  }

  // Computes the expected number of output samples for a given number of input
  // symbols for the current internal state.
  // Notations:
  //   a = input samples available = delayed_input.size() + input_size
  //   c = input samples consumed
  //   o = output samples produced
  //   fn = factor_numerator
  //   fd = factor_denominator
  //   ff = factor_floor = floor(fn / fd)
  //   ps = phase_step = fn - ff * fd
  //   p = phase
  //
  // To produce o output samples, the number of input samples consumed is
  //   c = o * ff + floor((p + o * ps) / fd).
  // The second term counts the number of times that the phase counter is
  // wrapped.
  //
  // No output is produced if a < num_taps. Supposing a >= num_taps,
  // ProcessSamples produces output while
  //   c < a - num_taps + 1.
  // That is, it stops once
  //   c >= a - num_taps + 1.
  // Therefore, the number of output samples o is the smallest integer
  // satisfying
  //   c = o * ff + floor((p + o * ps) / fd) >= a - num_taps + 1
  //   floor((p + o * ps) / fd) >= a - num_taps + 1 - o * ff
  //   p + o * ps >= fd * (a - num_taps + 1 - o * ff)
  //   o * (ff * fd + ps) >= fd * (a - num_taps + 1) - p,
  // which is
  //   o = ceil((fd * (a - num_taps + 1) - p) / fn).
  int ComputeOutputSizeFromCurrentState(int input_size) {
    const int64 min_consumed_input =
        static_cast<int64>(delayed_input_.size()) + input_size - num_taps_ + 1;
    if (min_consumed_input <= 0) {
      return 0;
    }
    // (x + y - 1) / y = ceil(x / float(y)),  where y is an integer.
    return (min_consumed_input * factor_denominator_ - phase_ +
            factor_numerator_ - 1) /
           factor_numerator_;
  }

  bool valid_ = false;
  int factor_numerator_;
  int factor_denominator_;
  int factor_floor_;
  int radius_;
  int phase_step_;
  int num_taps_;
  std::vector<Eigen::Matrix<CoefficientType, 1, Eigen::Dynamic>> filters_;

  // A buffer of recent input samples occurring just before the current stream
  // position, saved between calls to ProcessSamples(). The size of this buffer
  // is always less than num_taps_. These samples are needed since they are in
  // the neighborhood of the resampling filter for the next few output samples.
  std::vector<ValueType> delayed_input_;
  int phase_;
};

}  // namespace audio_dsp

#endif  // AUDIO_DSP_RESAMPLER_RATIONAL_FACTOR_H_
