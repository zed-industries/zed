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

// Compute coefficients for linear biquad filters.
//
// This file contains design functions for many generic types of filters
// including lowpass, highpass, bandpass, bandstop, allpass, low/high shelf,
// and peak filters.
//
// There are also design functions for several of the popular filter types:
// Butterworth, Chebyshev (Type I & II), and elliptic filters, each with
// the lowpass, highpass, bandpass and bandstop variants.


#ifndef AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_DESIGN_H_
#define AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_DESIGN_H_

#include <vector>

#include "audio/linear_filters/biquad_filter_coefficients.h"
#include "audio/linear_filters/filter_poles_and_zeros.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {
// Create coefficients for second-order IIR filters. These filters are derived
// from passive analog filters. These functions can be used to generate
// coefficients for filters whose transfer functions vary as a function of time
// while maintaining properties like passivity, constant corner frequency, or
// constant Q (quality filter).
//
// Included are lowpass and highpass filters; bandpass and bandstop filters
// specified about a center frequency; and bandpass and bandstop filters
// specified by their band edges. These filters are parameterized in terms of
// corner frequencies and their Q. For the lowpass and highpass, the Q specifies
// the gain at the corner frequency. Gains greater than 1/sqrt(2) will show a
// maximum in the response near the corner frequency, which becomes more extreme
// as Q increases (practical values of Q will be less than 10, and quite often
// need not exceed 1, but this is application dependent). For bandpass and
// bandstop, the steepness of rolloff is determined by the Q. When specifying
// bandpass and bandstop using band edges, Q is not a specified parameter but
// is a function of those band edges.
//
// Example use:
//
//   /* Any of the functions below. */
//   BiquadFilterCoefficients coeffs = LowpassBiquadFilterCoefficients(
//        sample_rate_hz, corner_frequency_hz, quality_factor);
//
//   /* See linear_filter.h */
//   BiquadFilter<float> filter;
//   filter.Init(1, coeffs);
//   while (...) {
//       float input_sample = GetNextInput();
//       float output_sample = filter.ProcessSample(next_input);
//       (do something with output_sample ...)
//   }
//

// Create coefficients for a biquad lowpass filter. The Q of this filter
// specifies the height of the resonant peak at the corner frequency.
// The DC gain of this filter is 1.0. For values of Q greater than 1 / sqrt(2),
// these filters become resonant at the corner frequency. Large Q values
// (above 4 or so) will the resonant peak to be very narrow. The gain at the
// corner frequency is equal to Q.
//
// This is a discretized version of the continuous time filter:
//                 w^2
//   H(s) = ------------------ ,
//           s^2 + sw/Q + w^2
// where w is the corner frequency in rads/sec and Q is the quality factor.
BiquadFilterCoefficients LowpassBiquadFilterCoefficients(
    double sample_rate_hz, double corner_frequency_hz, double quality_factor);

// Create coefficients for a biquad highpass filter. The Q of this filter
// specifies the width and height of the resonant peak at the corner frequency.
// The high frequency gain of this filter is 1.0. For values of Q greater than
// 1 / sqrt(2), these filters become resonant at the corner frequency. Large Q
// values (above 4 or so) will the resonant peak to be very narrow. The gain at
// the corner frequency is equal to Q.
//
// This is a discretized version of the continuous time filter:
//                 s^2
//   H(s) = ------------------ ,
//           s^2 + sw/Q + w^2
// where w is the corner frequency in rads/sec and Q is the quality factor.
BiquadFilterCoefficients HighpassBiquadFilterCoefficients(
    double sample_rate_hz, double corner_frequency_hz, double quality_factor);

// Create coefficients for a bandpass biquad filter specified by a center
// frequency and a quality factor. The mapping of the center frequency is
// exact. Note that due to the warping of the bilinear transform, the actual
// quality factor of this filter will deviate from quality_factor as
// center_frequency_hz approaches the Nyquist rate.
//
// This is a discretized version of the continuous time filter:
//                 sw/Q
//   H(s) = ------------------ ,
//           s^2 + sw/Q + w^2
// where w is the corner frequency in rads/sec and Q is the quality factor.
BiquadFilterCoefficients BandpassBiquadFilterCoefficients(
    double sample_rate_hz, double center_frequency_hz, double quality_factor);

// Create coefficients for a bandstop biquad filter specified by a center
// frequency and a quality factor. The mapping of the center frequency is
// exact. Note that due to the warping of the bilinear transform, the actual
// quality factor of this filter will deviate from quality_factor as
// center_frequency_hz approaches the Nyquist rate.
//
// This is a discretized version of the continuous time filter:
//               s^2 +w^2
//   H(s) = ------------------ ,
//           s^2 + sw/Q + w^2
// where w is the corner frequency in rads/sec and Q is the quality factor.
BiquadFilterCoefficients BandstopBiquadFilterCoefficients(
    double sample_rate_hz, double center_frequency_hz, double quality_factor);

// Create coefficients for a bandpass biquad filter specified by its lower and
// upper passband frequencies. The center frequency is not specified, but is
// approximately the geometric mean of the lower and upper band edges.
// This approximation is quite accurate for band edges << sample_rate_hz / 2.
//
// Note: The warping of the bilinear transform causes the center frequency of
// these filters to not be precisely the geometric mean of the two specified
// passband edges. If you require precise center frequency mapping,
// use BandpassBiquadFilterCoefficients() instead.
//
// This is a discretized version of the continuous time filter:
//                 (w2 - w1)s
//   H(s) = ------------------------- ,
//           s^2 + s(w2 - w1) + w1w2
// where w1 and w2 are the lower and upper passband edges in units of rads/sec.
BiquadFilterCoefficients RangedBandpassBiquadFilterCoefficients(
    double sample_rate_hz,
    double lower_passband_edge_hz,
    double upper_passband_edge_hz);

// Create coefficients for a bandstop biquad filter specified by its lower and
// upper stopband frequencies. The center frequency is not specified, but is
// approximately the geometric mean of the lower and upper band edges.
// This approximation is quite accurate for band edges << sample_rate_hz / 2.
//
// Note: The warping of the bilinear transform causes the center frequency of
// these filters to not be precisely the geometric mean of the two specified
// stopband edges. If you require precise center frequency mapping,
// use BandstopBiquadFilterCoefficients() instead.
//
// This is a discretized version of the continuous time filter:
//                  s^2 + w2w1
//   H(s) = --------------------------- ,
//           s^2 + s(w2 - w1) + w1w2
// where w1 and w2 are the lower and upper passband edges in units of rads/sec.
BiquadFilterCoefficients RangedBandstopBiquadFilterCoefficients(
    double sample_rate_hz,
    double lower_stopband_edge_hz,
    double upper_stopband_edge_hz);

// Design a biquad filter with an asymptotic constant boost or attenuation for
// frequencies below (low shelf) or above (high shelf) the corner. Gain
// is specified on a linear scale. By increasing the Q, the transition band can
// be somewhat shortened, but peaking will occur such that the response is no
// longer monotonic. Here is an image of a shelving filter with Q larger than
// 1/sqrt(2), the Q at which the response is no longer monotonic:
// https://bassgorilla.com/wp-content/uploads/2014/10/Pro-Q-2-tilt-shelf.png
BiquadFilterCoefficients LowShelfBiquadFilterCoefficients(
    float sample_rate_hz,
    float corner_frequency_hz,
    float Q,
    float gain);
BiquadFilterCoefficients HighShelfBiquadFilterCoefficients(
    float sample_rate_hz,
    float corner_frequency_hz,
    float Q,
    float gain);

// The parametric peak filter has a gain of 'gain' at the center frequency,
// but returns to a gain of 1.0 as you move away from the peak. The Q of the
// filter is approximate and corresponds to that of a resonator Hs(s), as
// computed by BandpassBiquadFilterCoefficients. The parametric peak filter has
// the transfer function H(s) = 1 + (g - 1)Hs(s).
// See here (page 8) for some notes on this kind of filter. The precise
// relationship between bandwidth and Q is quite nontrivial.
// https://ccrma.stanford.edu/courses/424/handouts.2004/424_Handout22_Filters4_LectureNotes.pdf
//
// NOTE: If you were to look at the transfer function on a log-log plot you
// would find that for the same Q, a filter with gain G (G > 1) is much wider
// than a filter with gain 1 / G. This is because the typical definition of
// bandwidth is the distance between the points at which the gain is -3dB from
// the maximum. In this case, the gain may be less than -3dB, so we could choose
// some gain-dependent reference (sqrt(G) in the link above).
//
// If you are from an audio engineering background, you may find this behavior
// to be unexpected as digital audio workstations account for this so that
// the response (which is invariably presented on a log-log scale) for gains
// G and 1 / G look like the vertical flip of each other. If you expect the
// response to have this 'vertical flip' property for G < 1, use
// ParametricPeakBiquadFilterSymmetricCoefficients instead of
// ParametricPeakBiquadFilterCoefficients. For G > 1, these two functions
// produce the same filter. For G < 1,
// ParametricPeakBiquadFilterSymmetricCoefficients produces a wider filter.
BiquadFilterCoefficients ParametricPeakBiquadFilterCoefficients(
    float sample_rate_hz,
    float center_frequency_hz,
    float Q,
    float gain);

BiquadFilterCoefficients ParametricPeakBiquadFilterSymmetricCoefficients(
    float sample_rate_hz,
    float center_frequency_hz,
    float Q,
    float gain);

// An allpass filter has unity gain magnitude everywhere, but near the
// pole_frequency there is phase loss. As the pole is moved towards the
// unit circle, the amount of phase loss increases.
// This is a discretized version of the continuous time filter:
//            s^2 - ws/Q + w^2
//   H(s) = -------------------- ,
//            s^2 + ws/Q + w^2
// Rather than pick the Q directly, we compute it using more convenient
// parameters like phase/group delay below.
//
// Parameterize an allpass in terms of group delay and phase delay at
// a particular frequency.
// NOTE: Not all values are valid, and this function will ABSL_CHECK fail for an
// invalid configuration. You can verify a configuration using
// CheckAllPassConfiguration() below.
BiquadFilterCoefficients AllpassBiquadFilterCoefficients(
    float sample_rate_hz,
    float corner_frequency_hz,
    float phase_delay_radians /* at corner_frequency_hz */,
    float group_delay_seconds /* at corner_frequency_hz */);

// Uses the minimum achievable group delay. The function above can be tricky to
// use as the range of valid group delays quickly enters the range of
// numerically challenging filters (very rapid phase changes, high Q).
// http://faculty.tru.ca/rtaylor/publications/allpass2_align.pdf
BiquadFilterCoefficients AllpassBiquadFilterCoefficients(
    float sample_rate_hz,
    float corner_frequency_hz,
    float phase_delay_radians /* at corner_frequency_hz */);

bool /* success */ CheckAllPassConfiguration(
    float sample_rate_hz,
    float corner_frequency_hz,
    float phase_delay_radians /* at corner_frequency_hz */,
    float group_delay_seconds /* at corner_frequency_hz */);

double MinimumGroupDelayForAllPass(
    float sample_rate_hz,
    float corner_frequency_hz,
    float phase_delay_radians /* at corner_frequency_hz */);

// Base class for IIR filter design using poles and zeros.
//
// Example:
//   // Design a Butterworth lowpass filter.
//   BiquadFilterCascadeCoefficients coeffs =
//       ButterworthFilterDesign(order)
//       .LowpassCoefficients(sample_rate_hz, corner_frequency_hz);
class PoleZeroFilterDesign {
 public:
  virtual ~PoleZeroFilterDesign() {}

  BiquadFilterCascadeCoefficients LowpassCoefficients(
      double sample_rate_hz, double corner_frequency_hz) const;

  BiquadFilterCascadeCoefficients HighpassCoefficients(
      double sample_rate_hz, double corner_frequency_hz) const;

  // NOTE: The bandpass and bandstop filters are designed from the same analog
  // prototype but have twice the order.
  BiquadFilterCascadeCoefficients BandpassCoefficients(
      double sample_rate_hz, double low_frequency_hz, double high_frequency_hz)
      const;

  BiquadFilterCascadeCoefficients BandstopCoefficients(
      double sample_rate_hz, double low_frequency_hz, double high_frequency_hz)
      const;

  // Each derived class should compute an "analog prototype", an analog lowpass
  // filter with cutoff 1 rad/s from which other filters can be constructed by
  // algebraic manipulations.
  virtual FilterPolesAndZeros GetAnalogPrototype() const = 0;
};

// Butterworth filter, with a maximally flat passband.
class ButterworthFilterDesign: public PoleZeroFilterDesign {
 public:
  explicit ButterworthFilterDesign(int order) : order_(order) {}

  FilterPolesAndZeros GetAnalogPrototype() const override;

 private:
  int order_;
};

// Chebyshev type 1 filters have the property that they minimize the error
// between the ideal and actual filter at the expense of having equiripple
// behavior in the passband.
class ChebyshevType1FilterDesign: public PoleZeroFilterDesign {
 public:
  ChebyshevType1FilterDesign(int order, double passband_ripple_db)
      : order_(order),
        passband_ripple_db_(passband_ripple_db) {}

  FilterPolesAndZeros GetAnalogPrototype() const override;

 private:
  int order_;
  double passband_ripple_db_;
};

// Chebyshev type 2 filters have the property that they minimize the error
// between the ideal and actual filter at the expense of having equiripple
// behavior in the stopband.
//
// The `stopband_ripple_db` parameter is the stopband attenuation, for instance
// stopband_ripple_db = 40.0 for a stopband below -40dB.
class ChebyshevType2FilterDesign: public PoleZeroFilterDesign {
 public:
  ChebyshevType2FilterDesign(int order, double stopband_ripple_db)
      : order_(order),
        stopband_ripple_db_(stopband_ripple_db) {}

  FilterPolesAndZeros GetAnalogPrototype() const override;

 private:
  int order_;
  double stopband_ripple_db_;
};

// Elliptic (a.k.a. Cauer or Zolotarev) filter, having equiripple in both the
// passband and stopband with a maximally steep rolloff.
//
// The `stopband_ripple_db` parameter is the stopband attenuation, for instance
// stopband_ripple_db = 40.0 for a stopband below -40dB.
class EllipticFilterDesign: public PoleZeroFilterDesign {
 public:
  EllipticFilterDesign(int order, double passband_ripple_db,
                       double stopband_ripple_db)
      : order_(order),
        passband_ripple_db_(passband_ripple_db),
        stopband_ripple_db_(stopband_ripple_db) {
    ABSL_CHECK_LT(passband_ripple_db, stopband_ripple_db);
  }
  FilterPolesAndZeros GetAnalogPrototype() const override;

 private:
  int order_;
  double passband_ripple_db_;
  double stopband_ripple_db_;
};

}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_DESIGN_H_
