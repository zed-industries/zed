// -*- C++ -*- compatibility header.

// Copyright (C) 2002-2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the
// Free Software Foundation; either version 3, or (at your option)
// any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/** @file math.h
 *  This is a Standard C++ Library header.
 */

#if !defined __cplusplus || defined _GLIBCXX_INCLUDE_NEXT_C_HEADERS
# include_next <math.h>
#else

#ifndef _GLIBCXX_MATH_H
#define _GLIBCXX_MATH_H 1

# include <cmath>

using std::abs;
using std::acos;
using std::asin;
using std::atan;
using std::atan2;
using std::cos;
using std::sin;
using std::tan;
using std::cosh;
using std::sinh;
using std::tanh;
using std::exp;
using std::frexp;
using std::ldexp;
using std::log;
using std::log10;
using std::modf;
using std::pow;
using std::sqrt;
using std::ceil;
using std::fabs;
using std::floor;
using std::fmod;

#if _GLIBCXX_USE_C99_MATH
using std::fpclassify;
using std::isfinite;
using std::isinf;
using std::isnan;
using std::isnormal;
using std::signbit;
using std::isgreater;
using std::isgreaterequal;
using std::isless;
using std::islessequal;
using std::islessgreater;
using std::isunordered;
#endif

#if __cplusplus >= 201103L && defined(_GLIBCXX_USE_C99_MATH_TR1)
using std::acosh;
using std::asinh;
using std::atanh;
using std::cbrt;
using std::copysign;
using std::erf;
using std::erfc;
using std::exp2;
using std::expm1;
using std::fdim;
using std::fma;
using std::fmax;
using std::fmin;
using std::hypot;
using std::ilogb;
using std::lgamma;
using std::llrint;
using std::llround;
using std::log1p;
using std::log2;
using std::logb;
using std::lrint;
using std::lround;
using std::nearbyint;
using std::nextafter;
using std::nexttoward;
using std::remainder;
using std::remquo;
using std::rint;
using std::round;
using std::scalbln;
using std::scalbn;
using std::tgamma;
using std::trunc;
#endif // C++11 && _GLIBCXX_USE_C99_MATH_TR1

// The mathematical special functions are only added to the global namespace
// by IS 29124, but not by C++17.
#if __cplusplus >= 201103L && __STDCPP_WANT_MATH_SPEC_FUNCS__ != 0
using std::assoc_laguerref;
using std::assoc_laguerrel;
using std::assoc_laguerre;
using std::assoc_legendref;
using std::assoc_legendrel;
using std::assoc_legendre;
using std::betaf;
using std::betal;
using std::beta;
using std::comp_ellint_1f;
using std::comp_ellint_1l;
using std::comp_ellint_1;
using std::comp_ellint_2f;
using std::comp_ellint_2l;
using std::comp_ellint_2;
using std::comp_ellint_3f;
using std::comp_ellint_3l;
using std::comp_ellint_3;
using std::cyl_bessel_if;
using std::cyl_bessel_il;
using std::cyl_bessel_i;
using std::cyl_bessel_jf;
using std::cyl_bessel_jl;
using std::cyl_bessel_j;
using std::cyl_bessel_kf;
using std::cyl_bessel_kl;
using std::cyl_bessel_k;
using std::cyl_neumannf;
using std::cyl_neumannl;
using std::cyl_neumann;
using std::ellint_1f;
using std::ellint_1l;
using std::ellint_1;
using std::ellint_2f;
using std::ellint_2l;
using std::ellint_2;
using std::ellint_3f;
using std::ellint_3l;
using std::ellint_3;
using std::expintf;
using std::expintl;
using std::expint;
using std::hermitef;
using std::hermitel;
using std::hermite;
using std::laguerref;
using std::laguerrel;
using std::laguerre;
using std::legendref;
using std::legendrel;
using std::legendre;
using std::riemann_zetaf;
using std::riemann_zetal;
using std::riemann_zeta;
using std::sph_besself;
using std::sph_bessell;
using std::sph_bessel;
using std::sph_legendref;
using std::sph_legendrel;
using std::sph_legendre;
using std::sph_neumannf;
using std::sph_neumannl;
using std::sph_neumann;
#endif // _GLIBCXX_USE_STD_SPEC_FUNCS

#if __cplusplus > 201703L
using std::lerp;
#endif // C++20

#endif // _GLIBCXX_MATH_H
#endif // __cplusplus
