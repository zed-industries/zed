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

#ifndef AUDIO_DSP_PORTABLE_NUMBER_UTIL_H_
#define AUDIO_DSP_PORTABLE_NUMBER_UTIL_H_

#ifdef __cplusplus
extern "C" {
#endif

/* Rounds up to nearest integer multiple of a positive factor, equivalent to
 * ceil(value / factor) * factor, but implemented in integer arithmetic for
 * efficiency and to avoid possibility of round-off error.
 */
int RoundUpToMultiple(int value, int factor);

/* Computes the greatest common divisor (GCD) using Euclid's algorithm. */
int GreatestCommonDivisor(int a, int b);

/* Detail options for RationalApproximation(). */
typedef struct {
  /* Max number of terms used in the continued fraction expansion. Terms are
   * counted including the integer part, e.g. [a0; a1, a2, a3] is 4 terms. The
   * default is 47 terms, enough to find the best rational for
   *
   *   max_denominator <= 2^32 - 1 < F47 = 2971215073.
   *
   * So enough for any max_denominator in 32-bit int range.
   *
   * The slowest progress is if every continued fraction term is 1, which gives
   * the golden ratio:
   *
   *   [1; 1, 1, 1, ...] = (1 + sqrt(5)) / 2 = golden ratio.
   *
   * The nth convergent denominator in this worst case is the nth Fibonacci
   * number, or F47 = 2971215073 at 47 terms.
   */
  int max_terms;
  /* Truncate continued fraction when residual is less than this tolerance. Must
   * be nonnegative. Default is 1e-9.
   */
  double convergence_tolerance;
} RationalApproximationOptions;

extern const RationalApproximationOptions kRationalApproximationDefaultOptions;

/* Approximate a given real number with a rational, a ratio a/b of integers a, b
 * with 0 < b <= max_denominator, i.e., a Diophantine approximation
 * [https://en.wikipedia.org/wiki/Diophantine_approximation]. The returned
 * rational a/b minimizes the error |value - a/b| among all rationals a'/b' with
 * 0 < b' <= max_denominator, and a/b is in reduced form (a and b have no common
 * factor).
 *
 * An options struct may be passed to fine tune algorithm details, or pass NULL
 * to use the default options: max_terms = 20, convergence_tolerance = 1e-9.
 *
 * Rational approximation error was evaluated empirically over a logarithmic
 * sweep of max_denominator from 1 to 10^6. For each value of max_denominator,
 * the worst-case (max) and average approximation errors were computed over 10^7
 * random values selected uniformly over [-1, 1]. The worst approximation error
 * |value - a/b| is 0.5 / max_denominator, the same as the max error obtained by
 * direct quantization a/b with b = max_denominator, a = round(value * b).
 * However, for most values the approximation is much better with an average
 * approximation error of about max_denominator^-1.88.
 *
 * Algorithm:
 * The given value is expanded in continued fraction representation, e.g.,
 * pi = [3; 7, 15, 1, 292, ...] which is concise notation for pi =
 * 3 + 1/(7 + 1/(15 + 1/(1 + 1/(292 + ...)))). Rational approximations called
 * convergents are made by truncating the continued fraction, e.g. pi is about
 * 3 + 1/7 = 22/7. Each convergent a/b is a best rational approximation, meaning
 * that a/b is closer than any other ratio with a smaller denominator,
 *   |value - a/b| < |value - a'/b'|  for any a'/b' != a/b, b' <= b.
 * However, the convergents are not all of the best rational approximations. To
 * cover them all, we consider also intermediate fractions, "semiconvergents,"
 * between the convergents.
 *
 * To illustrate how convergents and semiconvergents provide progressively finer
 * approximations, the table lists all best rational approximations of pi with
 * denominator up to 106 [http://oeis.org/A063674 and http://oeis.org/A063673].
 *
 *                                   a/b  |pi - a/b|
 *                               -------------------
 *   Convergent [3;]           =     3/1      1.4e-1
 *   Semiconvergent [3; 4]     =    13/4      1.1e-1
 *   Semiconvergent [3; 5]     =    16/5      5.8e-2
 *   Semiconvergent [3; 6]     =    19/6      2.5e-2
 *   Convergent [3; 7]         =    22/7      1.3e-3
 *   Semiconvergent [3; 7, 8]  =  179/57      1.2e-3
 *   Semiconvergent [3; 7, 9]  =  201/64      9.7e-4
 *   Semiconvergent [3; 7, 10] =  223/71      7.5e-4
 *   Semiconvergent [3; 7, 11] =  245/78      5.7e-4
 *   Semiconvergent [3; 7, 12] =  267/85      4.2e-4
 *   Semiconvergent [3; 7, 13] =  289/92      2.9e-4
 *   Semiconvergent [3; 7, 14] =  311/99      1.8e-4
 *   Convergent [3; 7, 15]     = 333/106      8.3e-5
 *
 * For a given max_denominator, semiconvergents are useful since they may be
 * more accurate than the best convergent. E.g. with max_denominator = 100, the
 * best possible approximation of pi is the semiconvergent 311/99 with error
 * 1.8e-4, while the best convergent is 22/7 with error 1.3e-3.
 *
 * References:
 * https://en.wikipedia.org/wiki/Continued_fraction#Best_rational_approximations
 * http://www.maths.surrey.ac.uk/hosted-sites/R.Knott/Fibonacci/cfINTRO.html
 */
void RationalApproximation(double value, int max_denominator,
                           const RationalApproximationOptions* options,
                           int* out_numerator,
                           int* out_denominator);

#ifdef __cplusplus
}  /* extern "C" */
#endif
#endif  /* AUDIO_DSP_PORTABLE_NUMBER_UTIL_H_ */
