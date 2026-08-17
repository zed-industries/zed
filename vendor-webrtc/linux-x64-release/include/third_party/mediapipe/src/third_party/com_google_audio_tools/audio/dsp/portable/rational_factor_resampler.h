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

/* Audio resampler.
 *
 * This library changes audio sample rate by a rational factor, using polyphase
 * FIR filtering. For instance by factor 3 to convert 48kHz audio to 16kHz or by
 * factor 160/441 to convert 16kHz audio to 44.1kHz. Arbitrary sample rate
 * conversions are approximately supported by finding the closest rational
 * factor for a specified max denominator.
 *
 * Algorithm:
 *
 * The input audio of sample rate F is interpolated according to
 *
 *   x(t) = sum_n x[n] h(t F - n)
 *
 * where h(t) is a Kaiser-windowed sinc. The nonzero support radius of h(t)
 * trades between anti-aliasing filtering quality and computation cost: a larger
 * radius allows for sharper transition but costs more terms in the sum over n.
 *
 * To resample with output sample rate F', the interpolant x(t) is sampled as
 *
 *   x'[m] = x(m/F') = sum_n x[n] h(m F/F' - n).
 *
 * The ratio d = F/F' is the resampling factor. This library replaces the ratio
 * d with a rational approximation a/b ~= d for some small a and b. Then we have
 *
 *   x'[m] = sum_n x[n] h(m a/b - n).
 *
 * The expression (m a/b) is the current position in units of input samples. Let
 * m a/b = p/b + q, where p and q are integer and 0 <= p < b, so that q is the
 * floor of the current position and p/b is the fractional part. We can then
 * express resampling as polyphase filtering:
 *
 *   x'[m] = sum_n x[n] h(p/b + q - n)
 *         = sum_k h(p/b + k) x[q - k]  (change of variables k = q - n)
 *         = (h_p * x)(q)
 *
 * where the last line views the sum as a convolution with filter h_p defined by
 *
 *   h_p[k] := h(p/b + k),  p = 0, 1, ..., b - 1.
 */

#ifndef AUDIO_DSP_PORTABLE_RATIONAL_FACTOR_RESAMPLER_H_
#define AUDIO_DSP_PORTABLE_RATIONAL_FACTOR_RESAMPLER_H_

#include "audio/dsp/portable/number_util.h"

#ifdef __cplusplus
extern "C" {
#endif

struct RationalFactorResampler; /* Forward declaration. */
typedef struct RationalFactorResampler RationalFactorResampler;

/* Detail options for RationalFactorResampler. */
typedef struct {
  /* `max_denominator` determines the max allowed denominator b in approximating
   * the resampling factor with a rational a/b. This also determines the max
   * number of filter phases. Larger max_denominator allows arbitrary resampling
   * factors to be approximated more accurately at the cost of memory for
   * storing more filter phases.
   *
   * If the factor can be expressed as a rational a/b with b <= max_denominator,
   * the resampling factor is represented exactly. Otherwise, an arbitrary
   * factor is represented with error no greater than 0.5 / max_denominator.
   *
   * The default is 1000, which is large enough to exactly represent the factor
   * between any two of the common sample rates 8kHz, 16kHz, 22.05kHz, 32kHz,
   * 44.1kHz, and 48kHz, and large enough to represent any arbitrary factor with
   * error no greater than 0.05%.
   */
  int max_denominator;
  /* Detail options for rational approximation. Set to NULL to use defaults. */
  RationalApproximationOptions* rational_approximation_options;
  /* Scale factor for the resampling kernel's nonzero support radius. If
   * upsampling, the kernel radius is `filter_radius_factor` input samples. If
   * downsampling, the kernel radius is `filter_radius_factor` *output* samples.
   * A larger radius makes the transition between passband and stopband sharper,
   * but proportionally increases computation and memory cost.
   *
   * filter_radius_factor = 5.0 is the default and is equivalent to
   * libresample's "low quality" mode, which despite the name is quite good.
   *
   * filter_radius_factor = 17.0 is equivalent to libresample's "high quality"
   * mode, which is probably overkill for most applications.
   */
  float filter_radius_factor;
  /* Antialiasing cutoff frequency as a proportion of
   *    min(input_sample_rate_hz, output_sample_rate_hz) / 2.
   * The default is 0.9, meaning the cutoff is at 90% of the input Nyquist
   * frequency or the output Nyquist frequency, whichever is smaller.
   */
  float cutoff_proportion;
  /* kaiser_beta is the positive beta parameter for the Kaiser window shape,
   * where larger value yields a wider transition band and stronger attenuation.
   *
   *   kaiser_beta      Stopband
   *   2.120            -30 dB
   *   3.384            -40 dB
   *   4.538            -50 dB
   *   5.658 (default)  -60 dB
   *   6.764            -70 dB
   *   7.865            -80 dB
   */
  float kaiser_beta;
} RationalFactorResamplerOptions;
extern const RationalFactorResamplerOptions
    kRationalFactorResamplerDefaultOptions;

/* Makes a RationalFactorResampler. The caller should free it when done with
 * `RationalFactorResamplerFree()`.
 *
 * The resampling factor input_sample_rate_hz / output_sample_rate_hz
 * is approximated by a rational factor a/b where 0 < b <= max_denominator.
 *
 * `max_input_size` arg is the max number of input samples that will be passed
 * per call to `RationalFactorResamplerProcessSamples()`.
 *
 * An options struct may be passed to fine tune resampler details, or pass NULL
 * to use the default options.
 */
RationalFactorResampler* RationalFactorResamplerMake(
    float input_sample_rate_hz,
    float output_sample_rate_hz,
    int max_input_size,
    const RationalFactorResamplerOptions* options);

/* Frees a RationalFactorResampler. */
void RationalFactorResamplerFree(RationalFactorResampler* resampler);

/* Resets to initial state. */
void RationalFactorResamplerReset(RationalFactorResampler* resampler);

/* Processes samples in a streaming manner.
 *
 * If `input_size` exceeds `max_input_size`, samples are dropped. The function
 * returns the number of resampled output samples, and the output buffer holding
 * the samples themselves is obtained with `RationalFactorResamplerOutput()`.
 *
 * Example use:
 *
 *   while (...) {
 *     float* input = // Get input_size samples...
 *     int output_size = RationalFactorResamplerProcessSamples(
 *         resampler, input, input_size);
 *     float* output = RationalFactorResamplerOutput(resampler);
 *     // Do something with output.
 *   }
 *
 * WARNING: Beware that the output size typically varies between calls, even
 * with a fixed input_size. The exact output size depends on input_size,
 * resampling factor, filter_radius_factor, and current resampling phase.
 */
int RationalFactorResamplerProcessSamples(
    RationalFactorResampler* resampler, const float* input, int input_size);

/* Gets the resampled output buffer. */
float* RationalFactorResamplerOutput(const RationalFactorResampler* resampler);

/* Gets the max input size that can be passed to ProcessSamples(). */
int RationalFactorResamplerMaxInputSize(
    const RationalFactorResampler* resampler);

/* Gets the rational used to approximate the requested resampling factor,
 *
 *   factor_numerator / factor_denominator
 *     ~= input_sample_rate_hz / output_sample_rate_hz.
 */
void RationalFactorResamplerGetRationalFactor(
    const RationalFactorResampler* resampler,
    int* factor_numerator, int* factor_denominator);

/* Gets how many output samples will be produced in the next call to
 * ProcessSamples() for an input of size `input_size`, according to the current
 * resampler state. If `input_size` exceeds max_input_size, it returns how many
 * output samples would have been produced supposing no samples were dropped.
 */
int RationalFactorResamplerNextOutputSize(
    const RationalFactorResampler* resampler, int input_size);

/* Gets a number of zero-valued input samples guaranteed to flush the resampler.
 * Calling ProcessSamples() on FlushSize() number of zero-valued input samples
 * extracts all the nonzero output samples.
 *
 * The value returned by FlushSize() is constant for a given resampler instance.
 *
 * Example:
 *   int flush_size = RationalFactorResamplerFlushSize(resampler);
 *   float* flush_input = (float*) malloc(sizeof(float) * flush_size);
 *   int n;
 *   for (n = 0; n < flush_size; ++n) { flush_input[n] = 0.0f; }
 *
 *   // Flush the resampler. The resampler reserves enough space that
 *   // an input of size flush_size is allowed without dropping samples.
 *   int flush_output_size = RationalFactorResamplerProcessSamples(
 *      resampler, flush_input, flush_size);
 *   float* flush_output = RationalFactorResamplerOutput(resampler);
 */
int RationalFactorResamplerFlushSize(const RationalFactorResampler* resampler);

#ifdef __cplusplus
}  /* extern "C" */
#endif
#endif  /* AUDIO_DSP_PORTABLE_RATIONAL_FACTOR_RESAMPLER_H_ */
