/*
 * Copyright 2021 Google LLC
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

// Fast audio resampling by a rational factor.
//
// QResampler ("Q" denoting "Quick" and the set of rational numbers) performs
// resampling using a polyphase FIR filter. We use the Eigen library to leverage
// vector instructions, computing each output sample as a dot product.
//
//
// Benchmarks:
//
// QResampler is faster than most other available resampling libraries. For
// example, for 48 to 16kHz resampling, QResampler is 5x faster than libresample
// and Chromium's SincResampler. More generally, the speed up depends on the
// resampling factor. Benchmark times (ns) for resampling 1024 samples 48->16kHz
// on SkyLake:
//
//
// Terminology:
//
//  * A "sample" is a scalar representing the waveform value at a specific point
//    in time in a specific channel.
//
//  * A "frame" is a vector of sample values for all channels that occur at a
//    specific point in time.
//
//
// Basic use:
//
// If you just want to resample a given signal all at once, and don't need
// streaming, use "QResampleSignal()" to resample the signal:
//
//   #include "audio/dsp/resampler_q.h"
//
//   // Non-streaming resampling.
//   QResamplerParams params;
//   // Set params...
//   std::vector<float> input = ...
//   std::vector<float> output = QResampleSignal<float>(
//       input_sample_rate, output_sample_rate, num_channels, params, input);
//
// To resample an ongoing stream, first create a QResampler instance, then
// call "ProcessSamples()" repeatedly to process the stream:
//
//   audio_dsp::QResampler<float> resampler(
//       input_sample_rate, output_sample_rate, params);
//
//   // Streaming resampling.
//   while (...) {
//     std::vector<float> input = // Get next block of input.
//     std::vector<float> output;
//     resampler.ProcessSamples(input, &output);
//     // Do something with output.
//   }
//
// ProcessSamples() produces as many output frames as possible from the
// available input. Note that the exact number of output frames produced in each
// call may vary even when number of input frames is the same. Optionally, at
// the end of the stream, call Flush() to "flush" it:
//
//    std::vector<float> flush_output;
//    resampler.Flush(&flush_output);
//
// This is equivalent to passing `resampler.flush_frames()` zero-valued frames
// to ProcessSamples().
//
//
// Avoiding allocations:
//
// A problem with the above processing loop is that std::vectors are allocated
// in each iteration, which may be unacceptable e.g. for real time processing in
// an audio thread. To avoid this, ProcessSamples can also be used as:
//
//   // Allocated buffers once up front.
//   Eigen::ArrayXf input_buffer(max_input_frames);
//   Eigen::ArrayXf output_buffer(resampler.MaxOutputFrames(max_input_frames));
//
//   while (...) {
//     // Get the next block of input, up to max_input_frames.
//     int num_input_frames = ...
//     input_buffer.head(num_input_frames) = ...
//
//     // Get the output size.
//     int num_output_frames = resample.NextNumOutputFrames(num_input_frames);
//
//     // Resample.
//     resampler.ProcessSamples(input_buffer.head(num_input_frames),
//                              output_buffer.head(num_output_frames));
//   }
//
// The key is the NextNumOutputFrames() method, which tells how many output
// frames the next call to ProcessSamples will produce, and when allocating the
// output, MaxOutputFrames(), which tells the max number of output frames that
// could be produced.
//
// Similarly, Flush() can also be used without allocating:
//
//   // Allocated buffers once up front.
//   Eigen::ArrayXf input_buffer(max_input_frames);
//   Eigen::ArrayXf output_buffer(
//       resampler.MaxOutputFrames(std::max(max_input_frames,
//                                          resampler.flush_frames()));
//
//   while (...) {
//     // Call ProcessSamples() as above.
//   }
//
//   // Get flush output size.
//   int num_output_frames =
//       resample.NextNumOutputFrames(resampler.flush_frames());
//   resampler.Flush(output_buffer.head(num_output_frames));
//
// In this case, the output buffer needs to be big enough to hold the flush
// output, so we use a std::max to set the buffer size.
//
//
// Multichannel resampling:
//
// For multichannel resampling, set params.num_channels at construction. We
// represent a multichannel signal as an Eigen Array (or Matrix) with
// num_channels rows. A similar approach as before is possible to process
// without allocating:
//
//   // Allocated buffers once up front.
//   Eigen::ArrayXXf input_buffer(num_channels, max_input_frames);
//   Eigen::ArrayXXf output_buffer(num_channels,
//                                 resampler.MaxOutputFrames(max_input_frames));
//
//   while (...) {
//     // Get the next block of input, up to max_input_frames.
//     int num_input_frames = ...
//     input_buffer.leftCols(num_input_frames) = ...
//
//     // Get the output size.
//     int num_output_frames = resample.NextNumOutputFrames(num_input_frames);
//
//     // Resample.
//     resampler.ProcessSamples(input_buffer.leftCols(num_input_frames),
//                              output_buffer.leftCols(num_output_frames));
//   }
//
// As an optimization, if you know the number of channels at compile time, use
// Eigen types with a fixed number rows. ProcessSamples will then statically fix
// the number of channels in the core computation:
//
//   // Hardcode buffers for stereo resampling.
//   constexpr int kNumChannels = 2;
//   using Buffer = Eigen::Array<float, kNumChannels, Eigen::Dynamic>;
//   Buffer input_buffer(kNumChannels, max_input_frames);
//   Buffer output_buffer(kNumChannels,
//                        resampler.MaxOutputFrames(max_input_frames));
//
//   while (...) {
//     // Same as above.
//   }
//
//
// Conversion from RationalFactorResampler kernel params:
//
// QResamplerParams are equivalent to but (usually) more convenient than the
// kernel parameters used in RationalFactorResampler. Here are formulas to
// convert RationalFactorResampler kernel parameters to QResamplerParams:
//
// radius: In RationalFactorResampler, the radius kernel parameter is in units
// of input samples. In QResampler, filter_radius_factor is a scale factor such
// that when upsampling, the kernel radius is filter_radius_factor input
// samples, or when downsampling, the kernel radius is filter_radius_factor
// output samples (that is, units are according to the slower sample rate).
// Conversion formula:
//
//   filter_radius_factor =
//       radius * min(1, output_sample_rate / input_sample_rate)
//
// cutoff: RationalFactorResampler's cutoff parameter is in units of Hz, while
// QResampler's cutoff_proportion is a proportion of the smaller Nyquist
// frequency. Conversion formula:
//
//   cutoff_proportion = 2 * cutoff / min(input_sample_rate, output_sample_rate)
//
// kaiser_beta: The kaiser_beta parameter has the same meaning in
// RationalFactorResampler and QResampler.


#ifndef AUDIO_DSP_RESAMPLER_Q_H_
#define AUDIO_DSP_RESAMPLER_Q_H_

#include <cstring>
#include <type_traits>
#include <vector>

#include "audio/dsp/eigen_types.h"
#include "audio/dsp/resampler.h"
#include "audio/dsp/types.h"
#include "absl/meta/type_traits.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

struct QResamplerParams {
  // `max_denominator` determines the max allowed denominator b in approximating
  // the resampling factor with a rational a/b. This also determines the max
  // number of filter phases. Larger max_denominator allows arbitrary resampling
  // factors to be approximated more accurately at the cost of memory for
  // storing more filter phases.
  //
  // If the factor can be expressed as a rational a/b with b <= max_denominator,
  // the resampling factor is represented exactly. Otherwise, an arbitrary
  // factor is represented with error no greater than 0.5 / max_denominator.
  //
  // The default is 1000, which is large enough to exactly represent the factor
  // between any two of the common sample rates 8 kHz, 11.025 kHz, 16 kHz,
  // 22.05 kHz, 44.1 kHz, and 48 kHz, and large enough to represent any
  // arbitrary factor with error no greater than 0.05%.
  int max_denominator = 1000;

  // Scale factor for the resampling kernel's nonzero support radius. If
  // upsampling, the kernel radius is `filter_radius_factor` input samples. If
  // downsampling, the kernel radius is `filter_radius_factor` *output* samples.
  // A larger radius makes the transition between passband and stopband sharper,
  // but proportionally increases computation and memory cost.
  //
  // filter_radius_factor = 5.0 is the default and is equivalent to
  // libresample's "low quality" mode, which despite the name is quite good.
  //
  // filter_radius_factor = 17.0 is equivalent to libresample's "high quality"
  // mode, which is probably overkill for most applications.
  float filter_radius_factor = 5.0f;

  // Antialiasing cutoff frequency as a proportion of
  //    min(input_sample_rate, output_sample_rate) / 2.
  // The default is 0.9, meaning the cutoff is at 90% of the input Nyquist
  // frequency or the output Nyquist frequency, whichever is smaller.
  float cutoff_proportion = 0.9f;

  // `kaiser_beta` is the positive beta parameter for the Kaiser window shape,
  // where larger value yields a wider transition band and stronger attenuation.
  //
  //   kaiser_beta      Stopband
  //   2.120            -30 dB
  //   3.384            -40 dB
  //   4.538            -50 dB
  //   5.658            -60 dB
  //   6.764            -70 dB
  //   7.865            -80 dB
  //
  // The default is 6.0 for attenuation slightly over 60 dB.
  float kaiser_beta = 6.0f;
};

template <typename ValueType> class QResampler;

// QResampleSignal() is a convenience function that resamples a given signal all
// at once, with flushing. Useful if streaming isn't needed.
//
// Example use:
//   std::vector<float> input = ...
//   std::vector<float> output = QResampleSignal<float>(
//       input_sample_rate, output_sample_rate, 1, {}, input);
template <typename ValueType>
std::vector<ValueType> QResampleSignal(float input_sample_rate,
                                       float output_sample_rate,
                                       int num_channels,
                                       const QResamplerParams& params,
                                       absl::Span<const ValueType> input) {
  ABSL_CHECK_GE(num_channels, 1);
  ABSL_CHECK_EQ(static_cast<int>(input.size()) % num_channels, 0);
  const int num_input_frames = input.size() / num_channels;

  QResampler<ValueType> resampler(
      input_sample_rate, output_sample_rate, num_channels, params);

  // Get how many samples the total processed + flushed output will be.
  const int total_output_size =
      num_channels * resampler.NextNumOutputFrames(num_input_frames +
                                                   resampler.flush_frames());
  std::vector<ValueType> output(total_output_size);
  absl::Span<ValueType> output_span(absl::MakeSpan(output));

  // Get how many samples ProcessSamples() will produce.
  const int process_output_size =
      num_channels * resampler.NextNumOutputFrames(num_input_frames);
  // Call ProcessSamples(), writing the first process_output_size samples.
  resampler.ProcessSamples(input, output_span.subspan(0, process_output_size));
  // Call Flush(), writing the remaining samples.
  resampler.Flush(output_span.subspan(process_output_size));

  return output;
}

namespace qresampler_internal {
template <typename ValueTypeOrImpl, typename = void>
struct UnpackTemplateArg;

template <typename CoeffType>
class QResamplerFilters;
}  // namespace qresampler_internal

// ValueType may be float, double, std::complex<float>, or std::complex<double>.
template <typename ValueType_>
class QResampler
    : public Resampler<typename qresampler_internal::UnpackTemplateArg<
          ValueType_>::ValueType> {
  using Impl = qresampler_internal::UnpackTemplateArg<ValueType_>;
 public:
  using ValueType = typename Impl::ValueType;

  // Constructs an uninitialized resampler. Init() must be called before calling
  // other methods.
  QResampler(): num_channels_(1) {}

  // Constructor for resampling `input_sample_rate` to `output_sample_rate`.
  // `num_channels` is the number of channels. For instance, num_channels = 2 to
  // resample a stereo audio signal. The implementation supports an arbitrary
  // number of channels with an optimized specialization for num_channels = 1.
  QResampler(float input_sample_rate, float output_sample_rate,
             int num_channels = 1, const QResamplerParams& params = {}) {
    Init(input_sample_rate, output_sample_rate, num_channels, params);
  }

  // Initializes (or reinitializes) the resampler.
  bool Init(float input_sample_rate, float output_sample_rate,
            int num_channels = 1, const QResamplerParams& params = {}) {
    if (!filters_.Init(input_sample_rate, output_sample_rate, params) ||
        num_channels <= 0) {
      num_channels_ = 1;
      valid_ = false;
      return false;
    }

    delayed_input_.resize(num_channels, filters_.num_taps() - 1);
    num_channels_ = num_channels;
    valid_ = true;
    this->Reset();
    return true;
  }

  // ProcessSamples() resamples in a streaming manner.
  //
  // See the top level comment in this file for examples of how to use it.
  // ProcessSamples accepts the following types of args:
  //  * std::vector
  //  * absl::Span
  //  * Eigen::Array, Matrix, Vector, and RowVector
  //  * Eigen block expressions like `x.head(n)` and `x.leftCols(n)`
  //  * (only as input) Eigen "nullaryop" expressions Zero, Ones, and Random
  //
  // If the output type is resizable, it is resized as necessary (beware that
  // this may allocate; see top level comment). For a non-resizable type, the
  // the size must match. Use NextNumOutputFrames() to get the correct size.
  //
  // When used for the output, std::vector and Eigen::Array, Matrix, Vector, and
  // RowVector must be passed by pointer. On the other hand, absl::Span and
  // Eigen block expressions must be passed by value.
  //
  // For num_channels > 1, multichannel data is represented in std::vector and
  // absl::Span with interleaved order, e.g. L0, R0, L1, R1, L2, R2, ... for
  // stereo data. Or with Eigen, the Eigen object has num_channels rows, so that
  // the ith row represents the ith channel. As an optimization, if you know the
  // number of channels at compile time, use Eigen types with a fixed number
  // rows like `Eigen::Array<float, 2, Eigen::Dynamic> input`. ProcessSamples
  // will then statically fix the number of channels in the core computation.
  template <typename Input, typename Output>
  void ProcessSamples(Input&& input, Output&& output) {
    ProcessSamplesCommon(
        WrapContainer(std::forward<Input>(input)),
        WrapContainer(DerefIfPointer(std::forward<Output>(output))));
  }

  // Flush() flushes the stream.
  //
  // Flush() adds enough zeros so that any previous non-zeros are fully
  // processed. It is equivalent to passing `flush_frames()` zero-valued frames
  // to ProcessSamples(). The output may be any type that ProcessSamples()
  // supports as an output. Calling this method also resets the resampler.
  //
  // Use `NextNumOutputFrames(flush_frames())` to get the number of output
  // frames that Flush() will produce.
  template <typename Output>
  void Flush(Output&& output) {
    ProcessSamplesCommon(
        WrapContainer(
            Eigen::Matrix<ValueType, Eigen::Dynamic, Eigen::Dynamic>::Zero(
                num_channels_, flush_frames())),
        WrapContainer(DerefIfPointer(std::forward<Output>(output))));
    ResetImpl();
  }

  // Number of channels.
  int num_channels() const { return num_channels_; }
  // Number of zero-valued input frames used to flush the resampler.
  //
  // NOTE: For API simplicity, this flush size is intentionally constant for a
  // resampler instance. It may be larger than necessary for the current state.
  // The flushed output has up to `num_taps / factor` more zeros than necessary.
  // For common parameters and sample rates, this is 10 to 30 samples and under
  // 2 ms, short enough that simplicity outweighs this minor inefficiency.
  int flush_frames() const { return filters_.flush_frames(); }
  // The rational resampling factor approximating the requested factor:
  //
  //   factor_numerator() / factor_denominator()
  //     ~= input_sample_rate / output_sample_rate.
  int factor_numerator() const { return filters_.factor_numerator(); }
  int factor_denominator() const { return filters_.factor_denominator(); }
  // Resampling filter radius in units of input frames.
  int radius() const { return filters_.radius(); }

  // Gets the max possible output size for the given max input size, independent
  // of resampler state.
  int MaxOutputFrames(int max_input_frames) const {
    return filters_.MaxOutputFrames(max_input_frames);
  }

  // Gets how many output frames will be produced in the next call to
  // ProcessSamples() for an input of `num_input_frames` frames, according to
  // the current resampler state. The following always holds:
  //
  //   NextNumOutputFrames(n) <= MaxOutputFrames(n).
  //
  // This function can also be used to get how many output frames will be
  // produced by Flush():
  //
  //   flush_output_frames = NextNumOutputFrames(flush_frames());
  //
  int NextNumOutputFrames(int num_input_frames) const {
    const int min_consumed_input = std::max<int>(
        1 + num_input_frames + delayed_input_frames_ - filters_.num_taps(), 0);
    if (min_consumed_input <= 0) {
      return 0;
    }
    return static_cast<int>(
        (static_cast<int64>(min_consumed_input) * factor_denominator() -
         phase_ + factor_numerator() - 1) /
        factor_numerator());
  }

  // Gets the max needed input size to produce at least `num_output_frames`,
  // independent of resampler state.
  int MaxInputFramesToProduce(int num_output_frames) const {
    return filters_.MaxInputFramesToProduce(num_output_frames);
  }

  // Gets how many input frames are needed in the next call to ProcessSamples()
  // to produce at least `num_output_frames` frames, according to the current
  // resampler state. When downsampling input_sample_rate > output_sample_rate,
  // exactly `num_output_frames` will be produced:
  //
  //   NextNumOutputFrames(NextNumInputFramesToProduce(m)) == m.
  //
  // When upsampling, the actual output size may exceed `num_output_frames` by
  // up to floor((factor_denominator - 1) / factor_numerator) frames. This is
  // because ProcessSamples() always produces as much output as possible from
  // the available input, and when upsampling, multiple output samples may be
  // produced for each consumed input sample.
  int NextNumInputFramesToProduce(int num_output_frames) const {
    if (num_output_frames <= 0) {
      return 0;
    }
    return static_cast<int>(
               (static_cast<int64>(num_output_frames - 1) * factor_numerator() +
                phase_) /
               factor_denominator()) +
           filters_.num_taps() - delayed_input_frames_;
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
    delayed_input_frames_ = filters_.num_taps() - 1;
    delayed_input_.leftCols(delayed_input_frames_).setZero();
  }

 private:
  // Implements the `Reset()` method. Resets QResampler initial
  // state as if it had just been constructed. Resampler state is set such that
  // the input and output streams are time-aligned, with the first output sample
  // corresponding to the same point in time as the first input sample.
  void ResetImpl() override {
    phase_ = 0;
    // The number of filter taps is `2 * radius_ + 1`, and for `phase_ = 0`, the
    // filtered output sample is time-aligned with the center tap's input
    // sample. We prime the buffer with `radius_` zeros such that this center
    // tap lands on the first given input sample.
    delayed_input_frames_ = filters_.radius();
    delayed_input_.leftCols(delayed_input_frames_).setZero();
  }

  bool ValidImpl() const override { return valid_; }

  void ProcessSamplesImpl(absl::Span<const ValueType> input,
                          std::vector<ValueType>* output) override {
    ProcessSamplesCommon(WrapContainer(input), WrapContainer(*output));
  }

  void FlushImpl(std::vector<ValueType>* output) override {
    Flush(output);
  }

  // All ProcessSamples() and Flush() calls are directed here. Args are checked
  // and mapped through several steps. The outline is:
  //
  // 1. ProcessSamplesCommon(input, output) maps `input` as an Eigen object. A
  //    specialized implementation is used for num_channels = 1. If `input` is a
  //    column vector at compile time, it is transposed to a row vector.
  //
  // 2. MapOutputAndProcess(in_map, output) uses `in_map` to determine the
  //    expected output shape. It then checks or resizes output to this shape
  //    and maps it as an Eigen object, again specializing for num_channels = 1.
  //
  // 3. If num_channels > 1, SetRowsAndProcess(in_map, out_map) is called. It
  //    uses the RowsAtCompileTime attribute of the input and output maps to try
  //    to statically infer the number of channels. If it can, input, output,
  //    and delayed_input are remapped as objects with fixed number of rows.
  //
  // 4. derived().ProcessSamplesGenericImpl() is called.
  template <typename Input, typename Output>
  void ProcessSamplesCommon(Input&& input, Output&& output) {
    ABSL_CHECK(valid_);
    int num_input_frames;
    // Validate input shape.
    if /*constexpr*/ (Input::Dims == 1) {
      ABSL_CHECK_EQ(input.size() % num_channels_, 0)
          << "Input size must be divisible by num_channels = " << num_channels_
          << ", got: " << input.size();
      num_input_frames = input.size() / num_channels_;
    } else if /*constexpr*/ (Input::IsVectorAtCompileTime) {
      ABSL_CHECK_EQ(num_channels_, 1)
          << "Eigen vector type allowed only for num_channels = 1.";
      num_input_frames = input.size();
    } else {
      ABSL_CHECK_EQ(num_channels_, input.rows());
      num_input_frames = input.cols();
    }

    const int num_output_frames = NextNumOutputFrames(num_input_frames);
    // Resize or validate output shape.
    if /*constexpr*/ (Output::Dims == 1) {
      ABSL_CHECK(output.resize(num_channels_ * num_output_frames))
          << "Expected output.size() == num_channels * num_output_frames == "
          << num_channels_ << " * " << num_output_frames
          << ", got: " << output.size()
          << ". Use NextNumOutputFrames() to get the correct output size.";
    } else if /*constexpr*/ (Output::IsVectorAtCompileTime) {
      ABSL_CHECK_EQ(num_channels_, 1)
          << "Eigen vector type allowed only for num_channels = 1.";
      ABSL_CHECK(output.resize(num_output_frames))
          << "Expected output.size() == num_output_frames == "
          << num_output_frames << ", got: " << output.size()
          << ". Use NextNumOutputFrames() to get the correct output size.";
    } else {
      ABSL_CHECK(output.resize(num_channels_, num_output_frames))
          << "Expected output shape (num_channels, num_output_frames) == ("
          << num_channels_ << ", " << num_output_frames << "), got: ("
          << output.rows() << ", " << output.cols()
          << "). Use NextNumOutputFrames() to get the correct output size.";
    }

    // As an optimization, a specialized implementation is used for the common
    // num_channels = 1 single channel case.
    if (num_channels_ == 1) {  // Specialize for single channel.
      Impl::ProcessSamplesGeneric(
          filters_,
          mapped_delayed_input<1>(),
          delayed_input_frames_,
          phase_,
          TransposeToRowVector(input.template AsMatrix<1>()).row(0),
          TransposeToRowVector(output.template AsMatrix<1>()).row(0));
    } else {
      // If possible, infer the number of channels at compile time from args.
      constexpr int kInRows =
          (Input::Dims == 1) ? Eigen::Dynamic : Input::RowsAtCompileTime;
      constexpr int kOutRows =
          (Output::Dims == 1) ? Eigen::Dynamic : Output::RowsAtCompileTime;
      constexpr int kRows = (kInRows != Eigen::Dynamic) ? kInRows : kOutRows;

      if /*constexpr*/ (kRows != Eigen::Dynamic) {
        Impl::ProcessSamplesGeneric(
            filters_,
            mapped_delayed_input<kRows>(),
            delayed_input_frames_,
            phase_,
            input.template AsMatrix<kRows>().template topRows<kRows>(),
            output.template AsMatrix<kRows>().template topRows<kRows>());
      } else {
        Impl::ProcessSamplesGeneric(
            filters_,
            delayed_input_,
            delayed_input_frames_,
            phase_,
            input.AsMatrix(num_channels_).matrix(),
            output.AsMatrix(num_channels_).matrix());
      }
    }
  }

  // Map delayed_input_ as an Eigen::Map with kChannels rows.
  template <int kChannels>
  Eigen::Map<Eigen::Matrix<ValueType, kChannels, Eigen::Dynamic>,
             // Set the Map's alignment in bytes. Since delayed_input_ is
             // allocated by Eigen, it should have EIGEN_DEFAULT_ALIGN_BYTES
             // alignment.
             EIGEN_DEFAULT_ALIGN_BYTES> mapped_delayed_input() {
    const int rows =
        (kChannels != Eigen::Dynamic) ? kChannels : delayed_input_.rows();
    ABSL_DCHECK_EQ(rows, num_channels_);
    return {delayed_input_.data(), rows, delayed_input_.cols()};
  }

  using CoeffType = typename RealType<ValueType>::Type;
  qresampler_internal::QResamplerFilters<CoeffType> filters_;

  using Buffer = Eigen::Matrix<ValueType, Eigen::Dynamic, Eigen::Dynamic>;
  // A buffer of recent input frames occurring just before the current stream
  // position, saved between calls to ProcessSamples(). The size of this buffer
  // is always less than num_taps. These frames are needed since they are in the
  // neighborhood of the resampling filter for the next few output samples.
  Buffer delayed_input_;

  int delayed_input_frames_;
  int num_channels_;
  int phase_;
  bool valid_ = false;
};

namespace qresampler_internal {

// In non-test code, QResampler's `ValueType` template arg should be either
// float, double, std::complex<float>, or std::complex<double>. Within
// QResampler's unit test, `ValueType` may be a class defining a static method
// ProcessSamplesGeneric() in order to substitute a mock implementation.
// `UnpackTemplateArg` is a helper to unpack the actual scalar ValueType and
// definition of ProcessSamplesGeneric().
template <typename ValueTypeOrImpl, typename>
struct UnpackTemplateArg {
  using ValueType = typename absl::decay_t<ValueTypeOrImpl>;
  using CoeffType = typename RealType<ValueType>::Type;

  // Resampling implementation.
  template <typename DelayedInput, typename Input, typename Output>
  static void ProcessSamplesGeneric(
      // QResampler class members.
      const qresampler_internal::QResamplerFilters<CoeffType>& filters,
      DelayedInput&& delayed_input,
      int& delayed_input_frames_ref,
      int& phase_ref,
      // Input and output containers, represented as Eigen Matrix types.
      Input&& input,
      Output&& output) {
    const int num_channels = delayed_input.rows();
    const int factor_denominator = filters.factor_denominator();
    const int num_taps = filters.num_taps();
    int delayed_input_frames = delayed_input_frames_ref;
    int phase = phase_ref;
    ABSL_DCHECK_EQ(input.rows(), num_channels);
    ABSL_DCHECK_EQ(output.rows(), num_channels);
    ABSL_DCHECK_LT(delayed_input_frames, num_taps);
    ABSL_DCHECK_LT(phase, factor_denominator);
    const int num_input_frames = input.cols();

    if (ABSL_PREDICT_FALSE(delayed_input_frames + num_input_frames <
                           num_taps)) {
      if (num_input_frames > 0) {
        // Append input to delayed_input.
        delayed_input.middleCols(delayed_input_frames, num_input_frames) =
            input;
        delayed_input_frames_ref += num_input_frames;
      }
      return;  // Not enough frames available to produce any output yet.
    }

    const int num_output_frames = output.cols();
    const int factor_floor = filters.factor_floor();
    const int phase_step = filters.phase_step();

    // Below, the position in the input is (i + phase / factor_denominator) in
    // units of input samples, with `phase` tracking the fractional part.
    int i = 0;
    // `i` is the start index for applying the filters. To stay within the
    // available `delayed_input_frames + num_input_frames` input samples, need:
    //
    //   i + num_taps - 1 < delayed_input_frames + num_input_frames,
    //
    // or i < i_end = delayed_input_frames + num_input_frames - num_taps + 1.
    int i_end = delayed_input_frames + num_input_frames - num_taps + 1;
    int num_written = 0;
    const int i_straddle_end = std::min(delayed_input_frames, i_end);
    const auto& filters_data = filters.coeffs().data();

    // Process samples where the filter straddles delayed_input and input.
    while (i < i_straddle_end) {
      ABSL_DCHECK_LT(num_written, num_output_frames);
      const int num_state = delayed_input_frames - i;
      const int num_input = num_taps - num_state;
      const auto& filter = filters_data[phase];

      // NOTE: It is important to use `.noalias()` in matrix multiplications,
      // otherwise Eigen heap-allocates a temporary for evaluating the rhs.
      // [https://eigen.tuxfamily.org/dox/TopicLazyEvaluation.html]
      output.col(num_written).noalias() =
          delayed_input.middleCols(i, num_state) * filter.head(num_state) +
          input.leftCols(num_input) * filter.tail(num_input);

      ++num_written;
      i += factor_floor;
      phase += phase_step;
      if (phase >= factor_denominator) {
        phase -= factor_denominator;
        ++i;
      }
    }

    if (i < delayed_input_frames) {
      // Ran out of input samples before consuming everything in delayed_input.
      // Discard the samples of delayed_input that have been consumed and append
      // the input.
      ABSL_DCHECK_EQ(num_written, num_output_frames);
      const int remaining = delayed_input_frames - i;
      // NOTE: We would do
      //
      //   delayed_input.leftCols(remaining).noalias() =
      //       delayed_input.rightCols(remaining);
      //
      // But it is not clear whether this is a safe case of memory aliasing
      // (see http://eigen.tuxfamily.org/dox/group__TopicAliasing.html). So
      // instead we manually memmove() the underlying data.
      std::memmove(delayed_input.data(),
                   delayed_input.data() + num_channels * i,
                   sizeof(ValueType) * num_channels * remaining);
      delayed_input.middleCols(remaining, num_input_frames) = input;
      delayed_input_frames += num_input_frames - i;
      ABSL_DCHECK_LT(delayed_input_frames, num_taps);
      delayed_input_frames_ref = delayed_input_frames;
      phase_ref = phase;
      return;
    }

    // Consumed everything in delayed_input. Now process output samples that
    // depend on only the input.
    i -= delayed_input_frames;
    i_end -= delayed_input_frames;

    while (i < i_end) {
      ABSL_DCHECK_LT(num_written, num_output_frames);

      output.col(num_written).noalias() =
          input.middleCols(i, num_taps) * filters_data[phase];

      ++num_written;
      i += factor_floor;
      phase += phase_step;
      if (phase >= factor_denominator) {
        phase -= factor_denominator;
        ++i;
      }
    }

    ABSL_DCHECK_EQ(num_written, num_output_frames);
    ABSL_DCHECK_LE(i, num_input_frames);
    delayed_input_frames = num_input_frames - i;
    delayed_input.leftCols(delayed_input_frames) =
        input.rightCols(delayed_input_frames);
    delayed_input_frames_ref = delayed_input_frames;
    phase_ref = phase;
  }
};

template <typename ValueTypeOrImpl>
struct UnpackTemplateArg<
    ValueTypeOrImpl, typename absl::void_t<typename ValueTypeOrImpl::ValueType>>
    : public ValueTypeOrImpl {};

template <typename CoeffType /* either float or double */>
class QResamplerFilters {
 public:
  using Filter = Eigen::Matrix<CoeffType, Eigen::Dynamic, 1>;

  QResamplerFilters();
  QResamplerFilters(float input_sample_rate, float output_sample_rate,
                    const QResamplerParams& params);

  bool Init(float input_sample_rate, float output_sample_rate,
            const QResamplerParams& params);

  const std::vector<Filter>& coeffs() const { return coeffs_; }
  int factor_numerator() const { return factor_numerator_; }
  int factor_denominator() const { return factor_denominator_; }
  int factor_floor() const { return factor_floor_; }
  int radius() const { return radius_; }
  int phase_step() const { return phase_step_; }
  int num_taps() const { return num_taps_; }

  // ProcessSamples() continues until there are less than num_taps input frames.
  // By appending (num_taps - 1) zeros to the input, we guarantee that after the
  // call to ProcessSamples(), delayed_input is only zeros.
  int flush_frames() const { return num_taps_ - 1; }

  // Gets the max possible output size for the given max input size.
  int MaxOutputFrames(int max_input_frames) const {
    return static_cast<int>(
        (static_cast<int64>(max_input_frames) * factor_denominator_ +
         factor_numerator_ - 1) /
        factor_numerator_);
  }

  // Gets the max needed input size to produce at least `num_output_frames`.
  int MaxInputFramesToProduce(int num_output_frames) const {
    if (num_output_frames <= 0) {
      return 0;
    }
    return static_cast<int>(
               (static_cast<int64>(num_output_frames - 1) * factor_numerator_ +
                factor_denominator_ - 1) /
               factor_denominator_) +
           num_taps_;
  }

 private:
  // Storing the filters this way ensures that each filter is memory aligned.
  // This lets Eigen make better vectorization in the matrix-vector multiplies.
  std::vector<Filter> coeffs_;
  int factor_numerator_;
  int factor_denominator_;
  int factor_floor_;
  int radius_;
  int phase_step_;
  int num_taps_;
};

}  // namespace qresampler_internal

}  // namespace audio_dsp

#endif  // AUDIO_DSP_RESAMPLER_Q_H_

