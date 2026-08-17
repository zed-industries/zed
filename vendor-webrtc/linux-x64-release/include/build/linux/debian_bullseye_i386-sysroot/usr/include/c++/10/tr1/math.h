// TR1 math.h -*- C++ -*-

// Copyright (C) 2006-2020 Free Software Foundation, Inc.
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

/** @file tr1/math.h
 *  This is a TR1 C++ Library header. 
 */

#ifndef _GLIBCXX_TR1_MATH_H
#define _GLIBCXX_TR1_MATH_H 1

#include <tr1/cmath>

#if _GLIBCXX_USE_C99_MATH_TR1

using std::tr1::acos;
using std::tr1::acosh;
using std::tr1::asin;
using std::tr1::asinh;
using std::tr1::atan;
using std::tr1::atan2;
using std::tr1::atanh;
using std::tr1::cbrt;
using std::tr1::ceil;
using std::tr1::copysign;
using std::tr1::cos;
using std::tr1::cosh;
using std::tr1::erf;
using std::tr1::erfc;
using std::tr1::exp;
using std::tr1::exp2;
using std::tr1::expm1;
using std::tr1::fabs;
using std::tr1::fdim;
using std::tr1::floor;
using std::tr1::fma;
using std::tr1::fmax;
using std::tr1::fmin;
using std::tr1::fmod;
using std::tr1::frexp;
using std::tr1::hypot;
using std::tr1::ilogb;
using std::tr1::ldexp;
using std::tr1::lgamma;
using std::tr1::llrint;
using std::tr1::llround;
using std::tr1::log;
using std::tr1::log10;
using std::tr1::log1p;
using std::tr1::log2;
using std::tr1::logb;
using std::tr1::lrint;
using std::tr1::lround;
using std::tr1::nearbyint;
using std::tr1::nextafter;
using std::tr1::nexttoward;
using std::tr1::pow;
using std::tr1::remainder;
using std::tr1::remquo;
using std::tr1::rint;
using std::tr1::round;
using std::tr1::scalbln;
using std::tr1::scalbn;
using std::tr1::sin;
using std::tr1::sinh;
using std::tr1::sqrt;
using std::tr1::tan;
using std::tr1::tanh;
using std::tr1::tgamma;
using std::tr1::trunc;

#endif

using std::tr1::assoc_laguerref;
using std::tr1::assoc_laguerre;
using std::tr1::assoc_laguerrel;

using std::tr1::assoc_legendref;
using std::tr1::assoc_legendre;
using std::tr1::assoc_legendrel;

using std::tr1::betaf;
using std::tr1::beta;
using std::tr1::betal;

using std::tr1::comp_ellint_1f;
using std::tr1::comp_ellint_1;
using std::tr1::comp_ellint_1l;

using std::tr1::comp_ellint_2f;
using std::tr1::comp_ellint_2;
using std::tr1::comp_ellint_2l;

using std::tr1::comp_ellint_3f;
using std::tr1::comp_ellint_3;
using std::tr1::comp_ellint_3l;

using std::tr1::conf_hypergf;
using std::tr1::conf_hyperg;
using std::tr1::conf_hypergl;

using std::tr1::cyl_bessel_if;
using std::tr1::cyl_bessel_i;
using std::tr1::cyl_bessel_il;

using std::tr1::cyl_bessel_jf;
using std::tr1::cyl_bessel_j;
using std::tr1::cyl_bessel_jl;

using std::tr1::cyl_bessel_kf;
using std::tr1::cyl_bessel_k;
using std::tr1::cyl_bessel_kl;

using std::tr1::cyl_neumannf;
using std::tr1::cyl_neumann;
using std::tr1::cyl_neumannl;

using std::tr1::ellint_1f;
using std::tr1::ellint_1;
using std::tr1::ellint_1l;

using std::tr1::ellint_2f;
using std::tr1::ellint_2;
using std::tr1::ellint_2l;

using std::tr1::ellint_3f;
using std::tr1::ellint_3;
using std::tr1::ellint_3l;

using std::tr1::expintf;
using std::tr1::expint;
using std::tr1::expintl;

using std::tr1::hermitef;
using std::tr1::hermite;
using std::tr1::hermitel;

using std::tr1::hypergf;
using std::tr1::hyperg;
using std::tr1::hypergl;

using std::tr1::laguerref;
using std::tr1::laguerre;
using std::tr1::laguerrel;

using std::tr1::legendref;
using std::tr1::legendre;
using std::tr1::legendrel;

using std::tr1::riemann_zetaf;
using std::tr1::riemann_zeta;
using std::tr1::riemann_zetal;

using std::tr1::sph_besself;
using std::tr1::sph_bessel;
using std::tr1::sph_bessell;

using std::tr1::sph_legendref;
using std::tr1::sph_legendre;
using std::tr1::sph_legendrel;

using std::tr1::sph_neumannf;
using std::tr1::sph_neumann;
using std::tr1::sph_neumannl;

#endif // _GLIBCXX_TR1_MATH_H
