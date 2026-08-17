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

/* Resampling kernel for RationalFactorResampler, a Kaiser-windowed sinc.
 *
 * For a cutoff frequency of B, the ideal brick-wall lowpass filter is the sinc
 *
 *   sin(2 pi B t) / (pi t)
 *
 * [https://en.wikipedia.org/wiki/Sinc_filter]. The code evaluates the sinc in
 * units of input samples x = t * Fs_in (where Fs_in is input sample rate) as
 *
 *   (B / (Fs_in / 2)) sin(radians_per_sample * x) / (radians_per_sample * x),
 *
 * with theta = B / (Fs_in / 2) = "cutoff as a proportion of input Nyquist" and
 * `radians_per_sample` = pi * theta.
 *
 * The Kaiser window [https://en.wikipedia.org/wiki/Kaiser_window] is
 *
 *   w(x) = I0[beta * sqrt(1 - (x / radius)^2)] / I0(beta) for |x| <= radius.
 *
 * The code evaluates this as
 *
 *   KaiserWindow(y) = I0[beta * sqrt(1 - y^2)],
 *   w(x) = KaiserWindow(x / radius) / I0(beta).
 *
 * The complete Kaiser-windowed sinc kernel is
 *
 *  K(x) = [theta * sin(radians_per_sample * x) / (radians_per_sample * x)]
 *       * [KaiserWindow(x / radius) / I0(beta)].
 *
 * We compute `normalization` = theta / I0(beta) once at construction as an
 * optimization.
 *
 * For more background, see for instance:
 * https://tomroelandts.com/articles/how-to-create-a-configurable-filter-using-a-kaiser-window
 * http://www.ee.ic.ac.uk/hp/staff/dmb/courses/DSPDF/00600_WindowFIR.pdf
 */

#ifndef AUDIO_DSP_PORTABLE_RATIONAL_FACTOR_RESAMPLER_KERNEL_H_
#define AUDIO_DSP_PORTABLE_RATIONAL_FACTOR_RESAMPLER_KERNEL_H_

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
  /* Resampling factor, > 1 if downsampling. */
  double factor;
  /* Nonzero support radius in units of input samples. */
  double radius;
  /* Frequency of kernel sinc in radians per input sample. */
  double radians_per_sample;
  /* Normalization factor such that the kernel has unit DC gain. */
  double normalization;
  /* Kaiser window beta parameter. */
  double kaiser_beta;
} RationalFactorResamplerKernel;

/* Initialize resampler kernel. Returns 1 on success, 0 on failure. */
int RationalFactorResamplerKernelInit(RationalFactorResamplerKernel* kernel,
                                      float input_sample_rate_hz,
                                      float output_sample_rate_hz,
                                      float filter_radius_factor,
                                      float cutoff_proportion,
                                      float kaiser_beta);

/* Evaluate the kernel at x, where x is in units of input samples. */
double RationalFactorResamplerKernelEval(
    const RationalFactorResamplerKernel* kernel, double x);

#ifdef __cplusplus
}  /* extern "C" */
#endif
#endif /* AUDIO_DSP_PORTABLE_RATIONAL_FACTOR_RESAMPLER_KERNEL_H_ */
