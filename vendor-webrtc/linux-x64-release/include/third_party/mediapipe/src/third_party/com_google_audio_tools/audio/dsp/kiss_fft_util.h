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

// Functions for frequency-domain based signal processing. FFTs are computed
// using the kiss_fft library.

#ifndef AUDIO_DSP_KISS_FFT_UTIL_H_
#define AUDIO_DSP_KISS_FFT_UTIL_H_

#include <complex>
#include <type_traits>
#include <vector>

#include "audio/dsp/eigen_types.h"
#include "audio/dsp/kiss_fft.h"
#include "third_party/eigen3/Eigen/Core"

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// Upsample using an FFT-based method.  This method is designed to
// give identical results to scipy.signal.resample; see the source
// code of that method for details.
void Upsample(const std::vector<float>& input, int upsampling_factor,
              std::vector<float>* upsampled_output);

// Lowpass filter a time domain signal by converting to the frequency domain
// and using a hard brickwall cutoff. Bins strictly above the bin containing
// high_band_edge are set to zero.  The high_band_edge and sample_rate
// parameters may be any consistent units of frequency. The parameters must
// satisfy 0 <= high_band_edge <= sample_rate / 2.
void FftHardLowpass(const std::vector<float>& input, float high_band_edge,
                    float sample_rate, std::vector<float>* output);

// Lowpass filter a spectrum using a hard brickwall cutoff. Bins strictly
// above the bin containing high_band_edge are set to zero. The high_band_edge
// and bin_width must be in the same frequency units.
// The parameters must satisfy:
// 0 <= high_band_edge <= bin_width * spectrum_size (with one half bin of
// tolerance on the upper limit to account for numerical imprecision).
//
// The spectrum is assumed to be of a real signal having frequencies from
// DC to Nyquist, omitting the Hermitian-redundant negative frequencies,
// as you would get with a real-to-complex FFT like RealFFTTransformer.
// The bin_width argument should be equal to the sampling rate
// divided by the number of time domain samples.
void HardLowpassSpectrum(const std::vector<std::complex<float>>& input_spectrum,
                         float high_band_edge, float bin_width,
                         std::vector<std::complex<float>>* output_spectrum);

// Highpass filter a time domain signal by converting to the frequency domain
// and using a hard brickwall cutoff. The parameters must satisfy:
// 0 <= low_band_edge <= sample_rate / 2. The band edges and bin_width must all
// be in the same frequency units.
void FftHardHighpass(const std::vector<float>& input, float low_band_edge,
                     float sample_rate, std::vector<float>* output);

// Highpass filter a spectrum using a hard brickwall cutoff. The parameters
// must satisfy 0 <= low_band_edge <= bin_width * spectrum_size.
//
// The spectrum is assumed to be of a real signal having frequencies from
// DC to Nyquist, omitting the Hermitian-redundant negative frequencies,
// as you would get with a real-to-complex FFT like RealFFTTransformer.
// The bin_width argument should be equal to the sampling rate
// divided by the number of time domain samples.
void HardHighpassSpectrum(
    const std::vector<std::complex<float>>& input_spectrum, float low_band_edge,
    float bin_width, std::vector<std::complex<float>>* output_spectrum);

// Bandpass filter a time domain signal by converting to the frequency domain
// and using a hard brickwall cutoff. The parameters must satisfy
// 0 <= low_band_edge <= high_band_edge <= sample_rate / 2. The band edges
// and bin_width must all be in the same frequency units.
void FftHardBandpass(const std::vector<float>& input, float low_band_edge,
                     float high_band_edge, float sample_rate,
                     std::vector<float>* output);

// Bandpass filter a spectrum with hard brickwall cutoff. The parameters must
// satisfy:
// 0 <= low_band_edge <= high_band_edge <= bin_width * spectrum_size.
// (with one half bin of tolerance on the upper limit to account for
// numerical imprecision).
//
// The spectrum is assumed to be of a real signal having frequencies from
// DC to Nyquist, omitting the Hermitian-redundant negative frequencies,
// as you would get with a real-to-complex FFT like RealFFTTransformer.
// The bin_width argument should be equal to the sampling rate
// divided by the number of time domain samples.
void HardBandpassSpectrum(
    const std::vector<std::complex<float>>& input_spectrum, float low_band_edge,
    float high_band_edge, float bin_width,
    std::vector<std::complex<float>>* output_spectrum);

// Resample a signal at input_sample_rate to output_sample_rate using using FFT
// zero padding or truncation, depending on which sample rate is larger. The
// boundary handling of the resampling is periodic.
// NOTE: In-place computation is allowed with output = &input.
void FftPeriodicResample(const std::vector<float>& input,
                         float input_sample_rate, float output_sample_rate,
                         std::vector<float>* output);

// Convolves input with a Gaussian with periodic boundary handling,
//             +inf
// result[m] =  sum   input[m - n] C exp(-n^2 / (2 sigma^2)),
//             n=-inf
// where the (m - n) index is modulo input.size() and C is the normalization
// constant such that the filter has unit DC gain. The convolution is computed
// via FFTs. result and input may be the same vector for in-place computation.
void PeriodicGaussianConvolution(const std::vector<float>& input, float sigma,
                                 std::vector<float>* result);

// Computes the linear convolution of elements in input_a and input_b and stores
// the result in output.
// output must have enough space for (size_a + size_b - 1) elements.
// The function appends zeros after the elements in input_a and input_b, so
// that their lenghts become greater than (a_size + b.size - 1). The
// convolution is implemented as multiplication in Fourier domain, hence exact
// lengths are chosen to allow FFT.
void LinearConvolution(const float* input_a, int size_a, const float* input_b,
                       int size_b, float* output);

// Computes the linear convolution of elements in input to itself num_times and
// stores the result in output.
// output must have enough space for at (size - 1) * num_times + 1 elements.
// The function appends zeros after the elements in input so
// that it length becomes greater than the above mentioned size. The
// convolution is implemented as power in Fourier domain, hence exact
// length is chosen to allow FFT.
void LinearConvolution(const float* input, int size, int num_times,
                       float* output);

namespace internal {
template <typename Container>
struct IsValidConvolveArg {
  enum { Value = IsContiguous1DEigenType<Container>::Value };
};
template <>
struct IsValidConvolveArg<std::vector<float>> {
  enum { Value = true };
};
}  // namespace internal

// Generic wrapper of LinearConvolution where the arguments can be any mix of
// vector<float>, ArrayXf, or VectorXf, enforced by the std::enable_if.
template <typename ContainerA, typename ContainerB, typename ContainerC>
typename std::enable_if<internal::IsValidConvolveArg<ContainerA>::Value &&
                        internal::IsValidConvolveArg<ContainerB>::Value &&
                        internal::IsValidConvolveArg<ContainerC>::Value>::type
LinearConvolution(const ContainerA& input_a, const ContainerB& input_b,
                  ContainerC* output) {
  output->resize(input_a.size() + input_b.size() - 1);
  LinearConvolution(input_a.data(), input_a.size(), input_b.data(),
                    input_b.size(), output->data());
}

// Generic wrapper of LinearConvolution of num_times where the arguments can be
// any mix of vector<float>, ArrayXf, or VectorXf,
// enforced by the std::enable_if.
template <typename ContainerA, typename ContainerB>
typename std::enable_if<internal::IsValidConvolveArg<ContainerA>::Value &&
                        internal::IsValidConvolveArg<ContainerB>::Value>::type
LinearConvolution(const ContainerA& input, int num_times, ContainerB* output) {
  output->resize((input.size() - 1) * num_times + 1);
  LinearConvolution(input.data(), input.size(), num_times, output->data());
}

}  // namespace audio_dsp

#endif  // AUDIO_DSP_KISS_FFT_UTIL_H_
