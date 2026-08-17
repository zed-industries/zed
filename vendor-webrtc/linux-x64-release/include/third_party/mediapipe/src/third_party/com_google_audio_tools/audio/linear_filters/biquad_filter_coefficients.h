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

// The coefficient representation, design, and analysis part of a
// biquad filtering library for single or multi-channel signal processing.

// Coefficients for biquad filter with real coefficents and transfer function
//
//         b0 + b1 z^-1 + b2 z^-2
//  H(z) = ----------------------.
//         a0 + a1 z^-1 + a2 z^-2
//
// We use doubles for the filter coefficients b0, b1, b2, a0, a1, a2.
//
// Also included are vectors of these to represent cascades of such biquad
// (ratio of quadratics) stages to make arbitrary rational transfer functions.

#ifndef AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_COEFFICIENTS_H_
#define AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_COEFFICIENTS_H_

#include <cmath>
#include <complex>
#include <functional>
#include <string>
#include <utility>
#include <vector>

#include "glog/logging.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {

void MakeLadderCoefficientsFromTransferFunction(
    const std::vector<double>& coeffs_b, const std::vector<double>& coeffs_a,
    std::vector<double>* coeffs_k_ptr, std::vector<double>* coeffs_v_ptr);

// Use bisection to find the maximum of an arbitrary function over some range.
// This has some hardcoded constants and works fine for use with filter transfer
// functions, but may not be suitable for generic optimization problems.
std::pair<double, double> FindPeakByBisection(
    const std::function<double(double)>& fun, double minimum, double maximum);

struct BiquadFilterCoefficients {
  // The default constructor makes biquad coefficients that, if used in a
  // BiquadFilter, would not alter the incoming signal (an identity filter).
  BiquadFilterCoefficients()
      : BiquadFilterCoefficients({1.0, 0.0, 0.0}, {1.0, 0.0, 0.0}) {}

  // The args must each be 3-element vectors with feedforward coefficients
  // coeff_b = {b0, b1, b2} and feedback coefficients coeff_a = {a0, a1, a2},
  // where a0 is nonzero.
  BiquadFilterCoefficients(const std::vector<double>& coeffs_b,
                           const std::vector<double>& coeffs_a)
      : b(coeffs_b), a(coeffs_a) {
    ABSL_CHECK_EQ(a.size(), 3);
    ABSL_CHECK_EQ(b.size(), 3);
  }

  // Check for filter stability (assuming a causal filter).
  bool IsStable() const;

  // Divide out a gain term such that the a[0] term is equal to 1.0.
  void Normalize() {
    const double normalize = a[0];
    for (double& coeff : b) { coeff /= normalize; }
    for (double& coeff : a) { coeff /= normalize; }
  }

  // Evaluate the transfer function at complex value z,
  //          b[0] + b[1] z^-1 + b[2] z^-2
  //   H(z) = ----------------------------
  //          a[0] + a[1] z^-1 + a[2] z^-2
  //
  //          b[0] z^2 + b[1] z + b[2]
  //        = ------------------------.
  //          a[0] z^2 + a[1] z + a[2]
  std::complex<double> EvalTransferFunction(
      const std::complex<double>& z) const {
    return ((b[0] * z + b[1]) * z + b[2]) / ((a[0] * z + a[1]) * z + a[2]);
  }

  // Get the two poles of the biquad filter as a pair with no particular order.
  std::pair<std::complex<double>, std::complex<double>> GetPoles() const;

  // Find the frequency at which the transfer function is maximal.
  // {frequency, gain at frequency} is returned. If multiple frequencies have
  // the same gain, the lowest frequency is returned.
  std::pair<double, double> FindPeakFrequencyRadiansPerSample() const;

  double GainMagnitudeAtFrequency(double frequency_radians_per_sample) const {
    std::complex<double> z = std::polar(1.0, frequency_radians_per_sample);
    return std::abs(EvalTransferFunction(z));
  }

  double GainMagnitudeAtFrequency(double frequency, double sample_rate) const {
    return GainMagnitudeAtFrequency(2 * M_PI * frequency / sample_rate);
  }

  double PhaseResponseAtFrequency(double frequency_radians_per_sample) const {
    std::complex<double> z = std::polar(1.0, frequency_radians_per_sample);
    return std::arg(EvalTransferFunction(z));
  }

  // frequency and sample_rate must be in same units.
  double PhaseResponseAtFrequency(double frequency, double sample_rate) const {
    return PhaseResponseAtFrequency(2 * M_PI * frequency / sample_rate);
  }

  void AsLadderFilterCoefficients(std::vector<double>* k,
                                  std::vector<double>* v) const {
    MakeLadderCoefficientsFromTransferFunction(b, a, k, v);
  }

  // Estimate the time needed in units of samples for the filter's impulse
  // response to decay below decay_db dB from its peak value.
  double EstimateDecayTime(double decay_db) const;

  void SetGainAtFrequency(double gain, double frequency_radians_per_sample) {
    double old_gain = GainMagnitudeAtFrequency(frequency_radians_per_sample);
    AdjustGain(gain / old_gain);
  }

  void SetPeakGain(double new_peak_gain) {
    SetGainAtFrequency(new_peak_gain,
                       FindPeakFrequencyRadiansPerSample().first);
  }

  void AdjustGain(double gain_multiplier) {
    for (auto& numerator_coeff : b) {
      numerator_coeff *= gain_multiplier;
    }
  }

  std::string ToString() const;

  std::vector<double> b;
  std::vector<double> a;
};

struct BiquadFilterCascadeCoefficients {
  // The default constructor makes coefficients that, if used in a
  // BiquadFilterCascade, would be an identity filter.
  BiquadFilterCascadeCoefficients()
      : coeffs(1) {}  // Default to one identity coeffs stage.

  // Constructing from a vector of BiquadFilterCoefficients is willing to
  // accept an empty vector.
  explicit BiquadFilterCascadeCoefficients(
      const std::vector<BiquadFilterCoefficients>& coeffs_vector)
      : coeffs(coeffs_vector) {
    if (coeffs.empty()) {
      // Make usable representation of identity filter.
      coeffs.resize(1);
    }
  }

  // Constructing from a single BiquadFilterCoefficients.
  explicit BiquadFilterCascadeCoefficients(
      const BiquadFilterCoefficients& biquad_coeffs)
      : coeffs(1, biquad_coeffs) {}

  // Check for filter stability (assuming a causal filter).
  bool IsStable() const;

  void AppendBiquad(const BiquadFilterCoefficients& biquad_coeffs) {
    coeffs.push_back(biquad_coeffs);
    Simplify();  // Squeeze out trivial factors.
  }

  void AppendDenominator(const std::vector<double>& a_coeffs) {
    ABSL_CHECK_EQ(a_coeffs.size(), 3);
    ABSL_CHECK_NE(a_coeffs[0], 0.0);
    AppendBiquad({{1.0, 0.0, 0.0}, a_coeffs});
  }

  void AppendNumerator(const std::vector<double>& b_coeffs) {
    ABSL_CHECK_EQ(b_coeffs.size(), 3);
    AppendBiquad({b_coeffs, {1.0, 0.0, 0.0}});
  }

  // Returns the coefficients as a ratio of polynomials (undoing the biquad
  // factorization). This is not advisable for very high order filters.
  //
  // See the "note on numerics" below on AsLadderFilterCoefficients(). Both this
  // routine and the MakeLadderCoefficientsFromTransferFunction() routine cause
  // error to get exponentially worse as order increases. This error is
  // caused by coefficient quantization, which alters the response and could
  // even cause instability. This is not the same as the error that accumulates
  // due to quantization of the filter's state.
  void AsPolynomialRatio(std::vector<double>* b, std::vector<double>* a) const;

  // Helper function for making LadderFilter coefficients. This is not advisable
  // for very high order filters.
  // Test carefully when using filters above order 8 or so. Another effective
  // strategy is to group the BiquadFilterCoefficients and make a couple of
  // lower-order filters.

  // Note on numerics of coefficient computation: "very high" actually depends
  // on the type of filter. Filters with very low cutoff frequencies (a
  // Butterworth lowpass with cutoff 400Hz at 48k, for example) will be
  // problematic after about order 10. However the same class of filter with a
  // cutoff of 10k will have extremely low error even for orders as high as 25.
  // Factors that can negatively influence stability include proximity of poles
  // to each other and to the unit circle.
  void AsLadderFilterCoefficients(std::vector<double>* k,
                                  std::vector<double>* v) const {
    std::vector<double> b;
    std::vector<double> a;
    AsPolynomialRatio(&b, &a);
    MakeLadderCoefficientsFromTransferFunction(b, a, k, v);
  }

  void Simplify();  // Squeeze out trivial stages.

  // Evaluate the transfer function at complex value z,
  std::complex<double> EvalTransferFunction(
      const std::complex<double>& z) const;

  // Find the frequency at which the transfer function is maximal.
  // {frequency, gain at frequency} is returned. If multiple frequencies have
  // the same gain, the lowest frequency is returned.
  std::pair<double, double> FindPeakFrequencyRadiansPerSample() const;

  double GainMagnitudeAtFrequency(double frequency_radians_per_sample) const {
    std::complex<double> z = std::polar(1.0, frequency_radians_per_sample);
    return std::abs(EvalTransferFunction(z));
  }

  double GainMagnitudeAtFrequency(double frequency, double sample_rate) const {
    return GainMagnitudeAtFrequency(2 * M_PI * frequency / sample_rate);
  }

  double PhaseResponseAtFrequency(double frequency_radians_per_sample) const {
    std::complex<double> z = std::polar(1.0, frequency_radians_per_sample);
    return std::arg(EvalTransferFunction(z));
  }

  // frequency and sample_rate must be in same units.
  double PhaseResponseAtFrequency(double frequency, double sample_rate) const {
    return PhaseResponseAtFrequency(2 * M_PI * frequency / sample_rate);
  }

  void SetGainAtFrequency(double gain, double frequency_radians_per_sample) {
    double old_gain = GainMagnitudeAtFrequency(frequency_radians_per_sample);
    AdjustGain(gain / old_gain);
  }

  void SetPeakGain(double new_peak_gain) {
    SetGainAtFrequency(new_peak_gain,
                       FindPeakFrequencyRadiansPerSample().first);
  }

  void AdjustGain(double gain_multiplier) {
    coeffs[0].AdjustGain(gain_multiplier);
  }

  const BiquadFilterCoefficients& operator[](int index) const {
    return coeffs[index];
  }

  BiquadFilterCoefficients& operator[](int index) { return coeffs[index]; }

  size_t size() const { return coeffs.size(); }

  std::string ToString() const;

  std::vector<BiquadFilterCoefficients> coeffs;
};

}  // namespace linear_filters

#endif  // AUDIO_LINEAR_FILTERS_BIQUAD_FILTER_COEFFICIENTS_H_
