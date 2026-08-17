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

/* Infinite impulse response (IIR) filter design library.
 *
 * This library implements Butterworth, Chebyshev type I and II, and elliptic
 * IIR designs for lowpass, highpass, bandpass, and bandstop filters of
 * arbitrary order. Also provided are utilities for working with filters in
 * zero-pole-gain ("ZPK") form.
 *
 * A few general tips:
 *
 *  - Start with Butterworth since of the designs here it has the most
 *    predictable behavior. Beware that Chebyshev and elliptic filters tend to
 *    have more strongly nonlinear phase response.
 *
 *  - Use 4th-order or lower IIR designs when possible for better numerical
 *    behavior and lower computation cost. Design order is limited in this
 *    library by kZpkMaxDegree. Instead of a high-order IIR design, you might be
 *    better served by a large FIR filter or frequency domain strategy.
 *
 *  - When running an IIR filter, beware that numerical round-off error builds
 *    up in the feedback loop. This problem can be severe for IIR designs where
 *    passband or stopband is too narrow. Generally, accumulation of round-off
 *    error gets worse the closer the filter poles are to the unit circle.
 *
 * Filters are returned as a cascade of biquads (second-order sections), which
 * can be implemented using the biquad_filter library. For instance Butterworth
 * lowpass filters are designed by the function
 *
 * int DesignButterworthLowpass(int order,
 *                              double cutoff_hz,
 *                              double sample_rate_hz,
 *                              BiquadFilterCoeffs* coeffs,
 *                              int max_biquads);
 *
 * where the biquad coefficients are written to `coeffs`. Other filter design
 * functions follow a similar pattern.
 *
 * It is possible to determine the number of biquads in advance for a given
 * order of a design (also noted with each design function below):
 *
 *   Filter type   Number of biquads
 *   Lowpass       `ceil(0.5 * order)`
 *   Highpass      `ceil(0.5 * order)`
 *   Bandpass      `order`
 *   Bandstop      `order`
 *
 * As a safety check, the `max_biquads` arg tells the capacity of the output
 * `coeffs` array. On success, the function returns the number of biquads
 * written. On failure (e.g. `max_biquads` is too small), an error message is
 * logged to stderr and the return value is 0.
 *
 * Example use:
 * // Design a 4th-order Butterworth lowpass filter.
 * BiquadFilterCoeffs coeffs[2];  // Two biquads is enough for 4th-order LPF.
 * assert(DesignButterworthLowpass(4, cutoff, sample_rate, &coeffs, 2) == 2);
 *
 * // Run the filter.
 * BiquadFilterState state[2];
 * BiquadFilterInitZero(&state[0]);
 * BiquadFilterInitZero(&state[1]);
 *
 * while (...) {
 *   float input = // Get next input sample.
 *   float output = BiquadFilterProcessOneSample(&coeffs[0], &state[0], input);
 *   output = BiquadFilterProcessOneSample(&coeffs[1], &state[1], output);
 *
 *   // Do something with output...
 * }
 */

#ifndef AUDIO_DSP_PORTABLE_IIR_DESIGN_H_
#define AUDIO_DSP_PORTABLE_IIR_DESIGN_H_

#include "audio/dsp/portable/biquad_filter.h"
#include "audio/dsp/portable/complex.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Max number of zeros or poles in Zpk. We limit to 30, enough for up to order
 * 30 lowpass and highpass designs and up to order 15 bandpass and bandstop
 * designs. This keeps Zpk's inline size tolerable at about 1 kilobyte, yet
 * should be high enough for most practical use (see "general tips" above if you
 * need more than order 15 IIR filters). Compared to heap allocation, this
 * allows for a simpler API and implementation.
 */
#define kZpkMaxDegree 30

/* Zero-pole-gain ("ZPK") struct, representing a filter by its s-plane (or
 * z-plane, if discretized) zero and pole locations and a scalar gain.
 *
 * NOTE: Instead of manipulating this struct directly, use the functions
 * ZpkInit(), ZpkEval(), etc. defined below.
 */
typedef struct {
  ComplexDouble zeros[kZpkMaxDegree];  /* Array of zeros.             */
  ComplexDouble poles[kZpkMaxDegree];  /* Array of poles.             */
  int num_zeros;                       /* Number of zeros in `zeros`. */
  int num_poles;                       /* Number of poles in `poles`. */
  double gain;                         /* Scalar gain.                */
} Zpk;

/* ------------------------ Butterworth filter designs -------------------------
 *
 * These functions design Butterworth filters, aka maximally flat magnitude
 * filters [https://en.wikipedia.org/wiki/Butterworth_filter]. The are optimal
 * in the sense that the response is as flat as possible in the passband. We
 * recommend Butterworth designs as a starting point when making a filter since
 * they are simpler and more predictable than Chebyshev and elliptic designs.
 *
 * `DesignButterworthLowpass()` [Number of biquads: `ceil(0.5 * order)`]
 * designs a Butterworth lowpass filter of order up to `kZpkMaxDegree` as a
 * cascade of biquads (second-order seconds). The gain is 1/sqrt(2) (half-power)
 * at frequency `cutoff_hz`, which must satisfy
 *
 *   0 <= cutoff_hz < sample_rate_hz / 2.
 *
 * The designed filter always has `ceil(0.5 * order)` biquads, which are written
 * to the output `coeffs` array. `max_biquads` tells the capacity of `coeffs` as
 * a safety measure. If `max_biquads < ceil(0.5 * order)`, error is reported.
 *
 * On success, the return value is the number of biquads written
 * [`ceil(0.5 * order)`]. On failure, the return value is 0 and an error message
 * is logged to stderr.
 */
int DesignButterworthLowpass(int order,
                             double cutoff_hz,
                             double sample_rate_hz,
                             BiquadFilterCoeffs* coeffs,
                             int max_biquads);

/* Butterworth highpass filter. [Number of biquads: `ceil(0.5 * order)`] */
int DesignButterworthHighpass(int order,
                              double cutoff_hz,
                              double sample_rate_hz,
                              BiquadFilterCoeffs* coeffs,
                              int max_biquads);

/* Designs a Butterworth bandpass filter. [Number of biquads: `order`]
 * `low_edge_hz` and `high_edge_hz` are the half-power edges, which must satisfy
 *
 *   0 <= low_edge_hz <= high_edge_hz < sample_rate_hz / 2.
 */
int DesignButterworthBandpass(int order,
                              double low_edge_hz,
                              double high_edge_hz,
                              double sample_rate_hz,
                              BiquadFilterCoeffs* coeffs,
                              int max_biquads);

/* Butterworth bandstop filter. [Number of biquads: `order`] */
int DesignButterworthBandstop(int order,
                              double low_edge_hz,
                              double high_edge_hz,
                              double sample_rate_hz,
                              BiquadFilterCoeffs* coeffs,
                              int max_biquads);

/* ---------------------- Chebyshev type 1 filter designs ----------------------
 *
 * These functions design Chebyshev type 1 filters
 * [https://en.wikipedia.org/wiki/Chebyshev_filter]. They have steeper roll-off
 * than Butterworth filters of the same order, but have ripples in the passband.
 * The `passband_ripple_db` specifies the magnitude of the ripples in decibels.
 *
 * `DesignChebyshev1Lowpass()` [Number of biquads: `ceil(0.5 * order)`]
 * designs a Chebyshev type 1 lowpass filter, where `cutoff_hz` specifies the
 * frequency at which the gain first drops below `-passband_ripple_db`.
 */
int DesignChebyshev1Lowpass(int order,
                            double passband_ripple_db,
                            double cutoff_hz,
                            double sample_rate_hz,
                            BiquadFilterCoeffs* coeffs,
                            int max_biquads);

/* Chebyshev type 1 highpass filter. [Number of biquads: `ceil(0.5 * order)`] */
int DesignChebyshev1Highpass(int order,
                             double passband_ripple_db,
                             double cutoff_hz,
                             double sample_rate_hz,
                             BiquadFilterCoeffs* coeffs,
                             int max_biquads);

/* Chebyshev type 1 bandpass filter. [Number of biquads: `order`] */
int DesignChebyshev1Bandpass(int order,
                             double passband_ripple_db,
                             double low_edge_hz,
                             double high_edge_hz,
                             double sample_rate_hz,
                             BiquadFilterCoeffs* coeffs,
                             int max_biquads);

/* Chebyshev type 1 bandstop filter. [Number of biquads: `order`] */
int DesignChebyshev1Bandstop(int order,
                             double passband_ripple_db,
                             double low_edge_hz,
                             double high_edge_hz,
                             double sample_rate_hz,
                             BiquadFilterCoeffs* coeffs,
                             int max_biquads);

/* ---------------------- Chebyshev type 2 filter designs ----------------------
 *
 * These functions design Chebyshev type 2 filters, also known as inverse
 * Chebyshev filters [https://en.wikipedia.org/wiki/Chebyshev_filter]. They have
 * steeper roll-off than Butterworth filters of the same order, but have ripples
 * in the stopband. The `stopband_ripple_db` specifies the magnitude of the
 * ripples in decibels.
 *
 * `DesignChebyshev2Lowpass()` [Number of biquads: `ceil(0.5 * order)`]
 * designs a Chebyshev type 2 lowpass filter, where `cutoff_hz` specifies the
 * frequency at which the gain first reaches `-stopband_ripple_db`.
 */
int DesignChebyshev2Lowpass(int order,
                            double stopband_ripple_db,
                            double cutoff_hz,
                            double sample_rate_hz,
                            BiquadFilterCoeffs* coeffs,
                            int max_biquads);

/* Chebyshev type 2 highpass filter. [Number of biquads: `ceil(0.5 * order)`] */
int DesignChebyshev2Highpass(int order,
                             double stopband_ripple_db,
                             double cutoff_hz,
                             double sample_rate_hz,
                             BiquadFilterCoeffs* coeffs,
                             int max_biquads);

/* Chebyshev type 2 bandpass filter. [Number of biquads: `order`] */
int DesignChebyshev2Bandpass(int order,
                             double stopband_ripple_db,
                             double low_edge_hz,
                             double high_edge_hz,
                             double sample_rate_hz,
                             BiquadFilterCoeffs* coeffs,
                             int max_biquads);

/* Chebyshev type 2 bandstop filter. [Number of biquads: `order`] */
int DesignChebyshev2Bandstop(int order,
                             double stopband_ripple_db,
                             double low_edge_hz,
                             double high_edge_hz,
                             double sample_rate_hz,
                             BiquadFilterCoeffs* coeffs,
                             int max_biquads);

/* -------------------------- Elliptic filter designs --------------------------
 *
 * These functions design elliptic filters, also known as Cauer filters or
 * Zolotarev filters [https://en.wikipedia.org/wiki/Elliptic_filter]. They have
 * the fastest transition in gain between passband and stopband for filters of
 * a given order.
 *
 * Elliptic filters have ripples in the passband and stopband, and they are
 * independently adjustable with `passband_ripple_db` and `stopband_ripple_db`.
 *
 * `DesignEllipticLowpass()` [Number of biquads: `ceil(0.5 * order)`]
 * designs an elliptic lowpass filter, where `cutoff_hz` specifies the
 * frequency at which the gain first drops below `-passband_ripple_db`.
 */
int DesignEllipticLowpass(int order,
                          double passband_ripple_db,
                          double stopband_ripple_db,
                          double cutoff_hz,
                          double sample_rate_hz,
                          BiquadFilterCoeffs* coeffs,
                          int max_biquads);

/* Elliptic highpass filter. [Number of biquads: `ceil(0.5 * order)`] */
int DesignEllipticHighpass(int order,
                           double passband_ripple_db,
                           double stopband_ripple_db,
                           double cutoff_hz,
                           double sample_rate_hz,
                           BiquadFilterCoeffs* coeffs,
                           int max_biquads);

/* Elliptic bandpass filter. [Number of biquads: `order`] */
int DesignEllipticBandpass(int order,
                           double passband_ripple_db,
                           double stopband_ripple_db,
                           double low_edge_hz,
                           double high_edge_hz,
                           double sample_rate_hz,
                           BiquadFilterCoeffs* coeffs,
                           int max_biquads);

/* Elliptic bandstop filter. [Number of biquads: `order`] */
int DesignEllipticBandstop(int order,
                           double passband_ripple_db,
                           double stopband_ripple_db,
                           double low_edge_hz,
                           double high_edge_hz,
                           double sample_rate_hz,
                           BiquadFilterCoeffs* coeffs,
                           int max_biquads);

/* -------------------------- `Zpk` utility functions --------------------------
 *
 * The following are utilities for zero-pole-gain filter representations. These
 * functions are used internally to make Butterworth, etc. filters as follows:
 *
 * 1. First, make a Zpk struct describing an "analog prototype", a
 *    continuous-time lowpass filter with cutoff of 1 rad/s.
 *    [See e.g. ButterworthAnalogPrototype() in the .c file.]
 *
 * 2. Convert the analog prototype to the desired response with one of
 *    ZpkAnalogPrototypeToLowpass(&zpk, cutoff_rad_s)
 *    ZpkAnalogPrototypeToHighpass(&zpk, cutoff_rad_s)
 *    ZpkAnalogPrototypeToBandpass(&zpk, low_edge_rad_s, high_edge_rad_s)
 *    ZpkAnalogPrototypeToBandstop(&zpk, low_edge_rad_s, high_edge_rad_s)
 *
 * 3. Discretize the filter with ZpkBilinearTransform(&zpk, sample_rate_hz).
 *
 * 4. Convert to biquads with ZpkToBiquads(&zpk, coeffs, max_biquads).
 */

/* Initializes Zpk to identity (num_zeros = num_poles = 0, gain = 1.0). */
void ZpkInit(Zpk* zpk);

/* Evaluates transfer function at `complex_freq`. For an analog design,
 * complex_freq = s, for discrete complex_freq = z.
 */
ComplexDouble ZpkEval(const Zpk* zpk, ComplexDouble complex_freq);

/* Sorts and pairs complex-conjugate pairs of zeros and poles by increasing real
 * part. The sorted results have purely real-valued roots followed by complex
 * conjugate pairs. Complex conjugate pairs are required to be exact conjugates.
 *
 * Returns 1 on success, or 0 if roots can't be paired.
 */
int /*bool*/ ZpkSort(Zpk* zpk);

/* Discretizes an analog design for `sample_rate_hz` using the bilinear
 * transform [https://en.wikipedia.org/wiki/Bilinear_transform]. No prewarping
 * is done in this function. Returns 1 on success, 0 on failure.
 */
int /*bool*/ ZpkBilinearTransform(Zpk* zpk, double sample_rate_hz);

/* Converts an analog prototype lowpass filter with cutoff 1 rad/s to a lowpass
 * filter with cutoff `cutoff_rad_s` of the same degree.
 */
void ZpkAnalogPrototypeToLowpass(Zpk* zpk, double cutoff_rad_s);

/* Like ZpkAnalogPrototypeToLowpass(), but results in a highpass filter. */
void ZpkAnalogPrototypeToHighpass(Zpk* zpk, double cutoff_rad_s);

/* Converts an analog prototype lowpass filter with cutoff 1 rad/s to a bandpass
 * filter with band edges `low_edge_rad_s` and `high_edge_rad_s`. Note that the
 * resulting filter has double the order. Returns 1 on success, or 0 on failure
 * (i.e. if the result would exceed kZpkMaxDegree).
 */
int /*bool*/ ZpkAnalogPrototypeToBandpass(Zpk* zpk,
                                          double low_edge_rad_s,
                                          double high_edge_rad_s);

/* Like ZpkAnalogPrototypeToBandpass(), but results in a bandstop filter. */
int /*bool*/ ZpkAnalogPrototypeToBandstop(Zpk* zpk,
                                          double low_edge_rad_s,
                                          double high_edge_rad_s);

/* Converts from zero-pole-gain representation to biquads. Biquads are written
 * to the output `coeffs` array, and `max_biquads` tells the capacity of the
 * `coeffs`. Returns the number of biquads written.
 */
int ZpkToBiquads(Zpk* zpk, BiquadFilterCoeffs* coeffs, int max_biquads);

#ifdef __cplusplus
}  /* extern "C" */
#endif
#endif /* AUDIO_DSP_PORTABLE_IIR_DESIGN_H_ */
