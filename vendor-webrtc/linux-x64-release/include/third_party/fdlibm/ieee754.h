// Copyright 2016 the V8 project authors. All rights reserved.
// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_FDLIBM_IEEE754_H_
#define THIRD_PARTY_FDLIBM_IEEE754_H_

namespace fdlibm {

// Returns the arc cosine of |x|; that is the value whose cosine is |x|.
double acos(double x);

// Returns the inverse hyperbolic cosine of |x|; that is the value whose
// hyperbolic cosine is |x|.
double acosh(double x);

// Returns the arc sine of |x|; that is the value whose sine is |x|.
double asin(double x);

// Returns the inverse hyperbolic sine of |x|; that is the value whose
// hyperbolic sine is |x|.
double asinh(double x);

// Returns the principal value of the arc tangent of |x|; that is the value
// whose tangent is |x|.
double atan(double x);

// Returns the principal value of the arc tangent of |y/x|, using the signs of
// the two arguments to determine the quadrant of the result.
double atan2(double y, double x);

// Returns the cosine of |x|, where |x| is given in radians.
double cos(double x);

// Returns the base-e exponential of |x|.
double exp(double x);

double atanh(double x);

// Returns the natural logarithm of |x|.
double log(double x);

// Returns a value equivalent to |log(1+x)|, but computed in a way that is
// accurate even if the value of |x| is near zero.
double log1p(double x);

// Returns the base 2 logarithm of |x|.
double log2(double x);

// Returns the base 10 logarithm of |x|.
double log10(double x);

// Returns the cube root of |x|.
double cbrt(double x);

// Returns exp(x)-1, the exponential of |x| minus 1.
double expm1(double x);

// Returns |x| to the power of |y|.
// The result of base ** exponent when base is 1 or -1 and exponent is
// +Infinity or -Infinity differs from IEEE 754-2008. The first edition
// of ECMAScript specified a result of NaN for this operation, whereas
// later versions of IEEE 754-2008 specified 1. The historical ECMAScript
// behaviour is preserved for compatibility reasons.
double pow(double x, double y);

// Returns the sine of |x|, where |x| is given in radians.
double sin(double x);

// Returns the tangent of |x|, where |x| is given in radians.
double tan(double x);

// Returns the hyperbolic cosine of |x|, where |x| is given radians.
double cosh(double x);

// Returns the hyperbolic sine of |x|, where |x| is given radians.
double sinh(double x);

// Returns the hyperbolic tangent of |x|, where |x| is given radians.
double tanh(double x);

// NOTE(caraitto): These functions are not present in the V8 math library --
// they are defined in terms of other functions.

float powf(float x, float y);

float expf(float x);

float log10f(float x);

float sinf(double x);

float asinf(double x);

}  // namespace fdlibm

#endif  // THIRD_PARTY_FDLIBM_IEEE754_H_

