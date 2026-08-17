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

// Compute coefficients for filters from complex poles and zeros.
#ifndef AUDIO_LINEAR_FILTERS_FILTER_POLES_AND_ZEROS_H_
#define AUDIO_LINEAR_FILTERS_FILTER_POLES_AND_ZEROS_H_

#include <complex>
#include <vector>

#include "audio/linear_filters/biquad_filter_coefficients.h"

#include "audio/dsp/porting.h"  // auto-added.


namespace linear_filters {
// A representation for filter poles and zeros. Lone complex poles and zeros are
// disallowed because only real-valued filter coefficients are supported.
//
// This class also holds a gain, k. This gain does not represent the gain at a
// particular frequency. It is simply a scalar gain applied to the function
//           (s - z0)(s - z1) ... (s - zn)
//  G(s) =  -------------------------------,
//           (s - p0)(s - p1) ... (s - pn)
// or for discrete time
//          (1 - z0*z^-1)(1 - z1*z^-1) ... (1 - zn*z^-1)
//  G(z) = ----------------------------------------------.
//          (1 - p0*z^-1)(1 - p1*z^-1) ... (1 - pn*z^-1)
// This class represents a system H(s) = k * G(s) or H(z) = k * G(z).
class FilterPolesAndZeros {
 public:
  FilterPolesAndZeros()
      : gain_(1.0) {}

  // Adds poles and zeros to the filter either as a single real root or as
  // a conjugate pair.
  void AddPole(double pole_position);
  void AddConjugatePolePair(std::complex<double> pole_position);
  void AddZero(double pole_position);
  void AddConjugateZeroPair(std::complex<double> pole_position);

  // Gets the gain of the filter.
  double GetGain() const { return gain_; }

  // Sets the gain of the filter.
  void SetGain(double gain) { gain_ = gain; }

  // NOTE: There is no restriction on whether the member variables represent
  // continuous or discrete-time poles and zeros, but GetCoefficients() assumes
  // that the representation is discrete-time.

  // Creates coefficients for the stored poles and zeros. If coeffs is not an
  // identity filter when it is passed in, filter stages are appended.
  BiquadFilterCascadeCoefficients GetCoefficients() const;

  int GetPolesDegree() const;
  int GetZerosDegree() const;

  // GetNumPoles() - GetNumZeros().
  int RelativeDegree() const;

  // For analog coefficients, complex_freq = s, for digital complex_freq = z.
  std::complex<double> Eval(const std::complex<double>& complex_freq) const;

  const std::vector<double>& GetRealPoles() const { return lone_poles_; }

  const std::vector<double>& GetRealZeros() const { return lone_zeros_; }

  // Only returns one from each pair of conjugate poles.
  const std::vector<std::complex<double>>& GetConjugatedPoles() const {
    return paired_poles_;
  }

  // Only returns one from each pair of conjugate poles.
  const std::vector<std::complex<double>>& GetConjugatedZeros() const {
    return paired_zeros_;
  }

  // TODO: Consider adding functionality to take polynomial
  // coefficients and find roots.

 private:
  // The roots of the numerator and demonimator, respectively, that do not
  // appear in conjugate pairs.
  std::vector<double> lone_zeros_;
  std::vector<double> lone_poles_;
  // Roots that are conjugated. Since one is the conjugate of the other, only
  // one is stored. For example, if paired_zeros_ contains 1 + i3, a filter
  // with zeros 1 + i3 and 1 - i3 will be created when GetCoefficients() is
  // called.
  std::vector<std::complex<double>> paired_zeros_;
  std::vector<std::complex<double>> paired_poles_;
  // A gain factor for the filter.
  double gain_;
};

}  // namespace linear_filters
#endif  // AUDIO_LINEAR_FILTERS_FILTER_POLES_AND_ZEROS_H_
