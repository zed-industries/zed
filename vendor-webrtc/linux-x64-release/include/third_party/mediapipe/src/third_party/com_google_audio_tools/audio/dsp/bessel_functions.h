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

// Library for numerical evaluation of Bessel functions.
// Currently only I0 and I1 modified Bessel functions of the first kind are
// implemented, but others could be added here as necessary.
//
// Evaluating these transcendental functions is slow, costing ~100 floating
// point operations in series expansions, so values should be precomputed where
// possible. If Bessel functions are needed within speed-critical code, it might
// make sense to develop a lookup table or a local polynomial approximation.

#ifndef AUDIO_DSP_BESSEL_FUNCTIONS_H_
#define AUDIO_DSP_BESSEL_FUNCTIONS_H_

#include "audio/dsp/porting.h"  // auto-added.


namespace audio_dsp {

// Evaluate I0(x), the zeroth-order modified Bessel function of the first kind
// for real argument x [http://dlmf.nist.gov/10.26.F1.mag].
//
// The function is approximated with a power series for small |x| or with an
// asymptotic expansion for large |x|. Results are accurate within 1e-15
// relative error for all points tested for about |x| <= 700, beyond which I0(x)
// exceeds double precision range [I0(x) grows exponentially].
double BesselI0(double x);

// Evaluate I1(x), the first-order modified Bessel function of the first kind,
// using similar approximations and accuracy as with BesselI0().
//
// Higher-order Bessel functions may be computed by the recursion
//   I_{n+1}(x) = I_{n-1}(x) - (2n / x) I_n(x).
// This recursion effectively evaluates I_n(x) for multiple orders n at the same
// point x. This is useful e.g. to evaluate Lindeberg's discrete Gaussian kernel
// [https://en.wikipedia.org/wiki/Scale_space_implementation].
double BesselI1(double x);

}  // namespace audio_dsp

#endif  // AUDIO_DSP_BESSEL_FUNCTIONS_H_
