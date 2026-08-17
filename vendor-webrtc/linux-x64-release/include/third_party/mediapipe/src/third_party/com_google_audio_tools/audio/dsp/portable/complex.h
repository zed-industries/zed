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

/* Complex math in C.
 *
 * There is no standard library for complex math in C before C99. This is a
 * small complex math library computed in double-precision floats.
 *
 * Complex values are represented with the `ComplexDouble` type. Arithmetic and
 * other operations are implemented as functions [C does not support operator
 * overloading, unfortunately].
 *
 * Summary of operations:
 *   Construct complex value.         ComplexDouble z = ComplexDoubleMake(a, b)
 *   Complex conjugate.               ComplexDoubleConj(z)
 *   Negation `-z`.                   ComplexDoubleNeg(z)
 *   Adds complex values `w + z`.     ComplexDoubleAdd(w, z)
 *   Complex subtract `w - z`.        ComplexDoubleSub(w, z)
 *   Complex multiply `w * z`.        ComplexDoubleMul(w, z)
 *   Complex-real multiply `z * x`.   ComplexDoubleMulReal(z, x)
 *   Complex divide `w / z`.          ComplexDoubleDiv(w, z)
 *   Complex phase angle in radians.  ComplexDoubleArg(z)
 *   Complex magnitude `|z|`.         ComplexDoubleAbs(z)
 *   Square magnitude `|z|^2`.        ComplexDoubleAbs2(z)
 *   Complex square `z^2`.            ComplexDoubleSquare(z)
 *   Complex square root `sqrt(z)`.   ComplexDoubleSqrt(z)
 *   Exponential `exp(z)`.            ComplexDoubleExp(z)
 *   Logarithm `log(z)`.              ComplexDoubleLog(z)
 *   Hyberbolic cosine `cosh(z)`.     ComplexDoubleCosh(z)
 *   Hyberbolic sine `sinh(z)`.       ComplexDoubleSinh(z)
 *   Cosine `cos(z)`.                 ComplexDoubleCos(z)
 *   Sine `sin(z)`.                   ComplexDoubleSin(z)
 *   Inverse cosh `acosh(z)`.         ComplexDoubleACosh(z)
 *   Inverse sinh `asinh(z)`.         ComplexDoubleASinh(z)
 *   Inverse cos `acos(z)`.           ComplexDoubleACos(z)
 *   Inverse sin `asin(z)`.           ComplexDoubleASin(z)
 *
 * NOTE: Lighter ops below are marked `static` [the C analogy for `inline`] so
 * that ideally they get inlined. We indeed see e.g. that Clang and gcc inline
 * calls in an expression like `ComplexDoubleMul(ComplexDoubleMul(z, z), z)`.
 *
 * NOTE: Complex inverse functions log, acosh, asinh, acos, asin are defined to
 * return the principal branch [https://en.wikipedia.org/wiki/Principal_branch]
 * according to a branch cut in the complex plane. For each function we follow
 * the conventional choice of branch cut as used in std::complex, numpy, and
 * other software.
 */

#ifndef AUDIO_DSP_PORTABLE_COMPLEX_H_
#define AUDIO_DSP_PORTABLE_COMPLEX_H_

#include <math.h>

#ifdef __cplusplus
extern "C" {
#endif

struct ComplexDouble {
  double real;
  double imag;
};
typedef struct ComplexDouble ComplexDouble;

/* Construct a `ComplexDouble` from real and imaginary parts. */
static ComplexDouble ComplexDoubleMake(double real, double imag) {
  ComplexDouble z;
  z.real = real;
  z.imag = imag;
  return z;
}

/* Complex conjugate of `z`. */
static ComplexDouble ComplexDoubleConj(ComplexDouble z) {
  z.imag = -z.imag;
  return z;
}

/* Negation, `-z`. */
static ComplexDouble ComplexDoubleNeg(ComplexDouble z) {
  z.real = -z.real;
  z.imag = -z.imag;
  return z;
}

/* Adds complex values `w + z`. */
static ComplexDouble ComplexDoubleAdd(ComplexDouble w, ComplexDouble z) {
  w.real += z.real;
  w.imag += z.imag;
  return w;
}

/* Subtracts complex values `w - z`. */
static ComplexDouble ComplexDoubleSub(ComplexDouble w, ComplexDouble z) {
  w.real -= z.real;
  w.imag -= z.imag;
  return w;
}

/* Multiplies complex values `w * z`. */
static ComplexDouble ComplexDoubleMul(ComplexDouble w, ComplexDouble z) {
  ComplexDouble result;
  result.real = w.real * z.real - w.imag * z.imag;
  result.imag = w.real * z.imag + w.imag * z.real;
  return result;
}

/* Multiplies complex value with real value `w * x`. */
static ComplexDouble ComplexDoubleMulReal(ComplexDouble z, double x) {
  z.real *= x;
  z.imag *= x;
  return z;
}

/* Divides complex values `w / z`. */
ComplexDouble ComplexDoubleDiv(ComplexDouble w, ComplexDouble z);

/* Complex phase angle in radians of `z`. */
static double ComplexDoubleArg(ComplexDouble z) {
  return atan2(z.imag, z.real);
}

/* Complex magnitude `|z|`. */
double ComplexDoubleAbs(ComplexDouble z);

/* Square complex magnitude `|z|^2`. */
static double ComplexDoubleAbs2(ComplexDouble z) {
  return z.real * z.real + z.imag * z.imag;
}

/* Squares complex value `z^2`. */
static ComplexDouble ComplexDoubleSquare(ComplexDouble z) {
  ComplexDouble result;
  result.real = z.real * z.real - z.imag * z.imag;
  result.imag = 2 * z.real * z.imag;
  return result;
}

/* Complex square root (principal branch) `sqrt(z)`. */
ComplexDouble ComplexDoubleSqrt(ComplexDouble z);

/* Complex exponential function `exp(z)`. */
ComplexDouble ComplexDoubleExp(ComplexDouble z);

/* Complex natural logarithm (principal branch) `log(z)`. The branch cut is
 * along the line segment (-infinity, 0) on the real axis.
 */
ComplexDouble ComplexDoubleLog(ComplexDouble z);

/* Complex cosh `cosh(z)`. */
ComplexDouble ComplexDoubleCosh(ComplexDouble z);

/* Complex sinh `sinh(z)`. */
ComplexDouble ComplexDoubleSinh(ComplexDouble z);

/* Complex cos `cos(z)`. */
ComplexDouble ComplexDoubleCos(ComplexDouble z);

/* Complex sin `sin(z)`. */
ComplexDouble ComplexDoubleSin(ComplexDouble z);

/* Complex acosh (inverse hyperbolic cosine) `acosh(z)`. The branch cut is along
 * the line segment (-infinity, +1) on the real axis.
 */
ComplexDouble ComplexDoubleACosh(ComplexDouble z);

/* Complex asinh (inverse hyperbolic sine) `asinh(z)`. Branch cut is the line
 * segments (-i infinity, -i) and (+i, +i infinity) on the imaginary axis.
 */
ComplexDouble ComplexDoubleASinh(ComplexDouble z);

/* Complex acos (inverse cosine) `acos(z)`. Branch cut is along the line
 * segments (-infinity, -1) and (1, infinity) on the real axis.
 */
ComplexDouble ComplexDoubleACos(ComplexDouble z);

/* Complex asin (inverse sine) `asin(z)`. Branch cut is along the line
 * segments (-infinity, -1) and (1, infinity) on the real axis.
 */
ComplexDouble ComplexDoubleASin(ComplexDouble z);

#ifdef __cplusplus
}  /* extern "C" */
#endif
#endif  /* AUDIO_DSP_PORTABLE_COMPLEX_H_ */
