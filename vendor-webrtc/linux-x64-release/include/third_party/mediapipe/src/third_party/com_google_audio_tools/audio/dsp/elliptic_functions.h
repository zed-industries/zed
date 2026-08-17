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

// Numerical evaluation of elliptic functions
// [https://en.wikipedia.org/wiki/Elliptic_integral]. Currently only the
// elliptic integral of the first kind F(phi|m) and its inverse am(u|m) are
// implemented. Others could be added here as necessary.
//
// NOTE: There are three common notation conventions for elliptic functions:
// the modular angle alpha, the elliptic modulus k, and the parameter m. They
// are related by m = k^2 = (sin(alpha))^2. Like scipy.special and Mathematica,
// we use parameter m.
//
// The implementation is based on Landen transformation
// [https://en.wikipedia.org/wiki/Landen%27s_transformation], which expresses
// elliptic integrals in terms of elliptic integrals with mapped parameters. A
// good reference is Orfanidis' lecture notes linked below.
//
// For general background on elliptic functions, see e.g. chapters 16 and 17 of
// Abramowitz and Stegun.
//
// References:
// * Sophocles J. Orfanidis, "Lecture Notes on Elliptic Filter Design," 2006.
//   [http://www.ece.rutgers.edu/~orfanidi/ece521/notes.pdf]
// * L.M. Milne-Thompson, "16 Jacobian Elliptic Functions and Theta Functions"
//   and "17 Elliptic Integrals" in Abramowitz and Stegun, "Handbook of
//   Mathematical Functions," 1964. [http://people.math.sfu.ca/~cbm/aands]

#ifndef AUDIO_DSP_ELLIPTIC_FUNCTIONS_H_
#define AUDIO_DSP_ELLIPTIC_FUNCTIONS_H_

#include <complex>

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// Evaluate the complete elliptic integral of the first kind,
//   K(m) = integral_[0, pi/2] 1 / sqrt(1 - m (sin(t))^2) dt,
// where m is the parameter, 0 <= m < 1.
double EllipticK(double m);

// Evaluate the incomplete elliptic integral of the first kind,
//   F(phi|m) = integral_[0, phi] 1 / sqrt(1 - m (sin(t))^2) dt,
// for |real(phi)| <= pi/2 and m is the parameter, 0 <= m < 1. The inverse
// Jacobi elliptic functions can be evaluated in terms of F(phi|m), e.g.
//   sn^-1(w|m) = F(asin(w)|m).
std::complex<double> EllipticF(const std::complex<double>& phi, double m);

// Compute the amplitude phi = am(u|m) for Jacobi elliptic functions
// [https://en.wikipedia.org/wiki/Jacobi_elliptic_functions]. The Jacobi
// elliptic functions can be evaluated in terms of the amplitude:
//   sn(u|m) = sin(phi),
//   cn(u|m) = cos(phi),
//   dn(u|m) = sqrt(1 - m (sin(phi))^2).
// This function and EllipticF are inverses: F(phi|m) = u, am(u|m) = phi for
// |u| <= K(m) (or equivalently, |phi| <= pi/2).
std::complex<double> JacobiAmplitude(const std::complex<double>& u, double m);

}  // namespace audio_dsp

#endif  // AUDIO_DSP_ELLIPTIC_FUNCTIONS_H_
