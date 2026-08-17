// -*- C++ -*- C library enhancements header.

// Copyright (C) 2016-2020 Free Software Foundation, Inc.
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

/** @file include/bits/std_abs.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{cmath, cstdlib}
 */

#ifndef _GLIBCXX_BITS_STD_ABS_H
#define _GLIBCXX_BITS_STD_ABS_H

#pragma GCC system_header

#include <bits/c++config.h>

#define _GLIBCXX_INCLUDE_NEXT_C_HEADERS
#include_next <stdlib.h>
#ifdef __CORRECT_ISO_CPP_MATH_H_PROTO
# include_next <math.h>
#endif
#undef _GLIBCXX_INCLUDE_NEXT_C_HEADERS

#undef abs

extern "C++"
{
namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  using ::abs;

#ifndef __CORRECT_ISO_CPP_STDLIB_H_PROTO
  inline long
  abs(long __i) { return __builtin_labs(__i); }
#endif

#ifdef _GLIBCXX_USE_LONG_LONG
  inline long long
  abs(long long __x) { return __builtin_llabs (__x); }
#endif

// _GLIBCXX_RESOLVE_LIB_DEFECTS
// 2192. Validity and return type of std::abs(0u) is unclear
// 2294. <cstdlib> should declare abs(double)
// 2735. std::abs(short), std::abs(signed char) and others should return int

#ifndef __CORRECT_ISO_CPP_MATH_H_PROTO
  inline _GLIBCXX_CONSTEXPR double
  abs(double __x)
  { return __builtin_fabs(__x); }

  inline _GLIBCXX_CONSTEXPR float
  abs(float __x)
  { return __builtin_fabsf(__x); }

  inline _GLIBCXX_CONSTEXPR long double
  abs(long double __x)
  { return __builtin_fabsl(__x); }
#endif

#if defined(__GLIBCXX_TYPE_INT_N_0)
  inline _GLIBCXX_CONSTEXPR __GLIBCXX_TYPE_INT_N_0
  abs(__GLIBCXX_TYPE_INT_N_0 __x) { return __x >= 0 ? __x : -__x; }
#endif
#if defined(__GLIBCXX_TYPE_INT_N_1)
  inline _GLIBCXX_CONSTEXPR __GLIBCXX_TYPE_INT_N_1
  abs(__GLIBCXX_TYPE_INT_N_1 __x) { return __x >= 0 ? __x : -__x; }
#endif
#if defined(__GLIBCXX_TYPE_INT_N_2)
  inline _GLIBCXX_CONSTEXPR __GLIBCXX_TYPE_INT_N_2
  abs(__GLIBCXX_TYPE_INT_N_2 __x) { return __x >= 0 ? __x : -__x; }
#endif
#if defined(__GLIBCXX_TYPE_INT_N_3)
  inline _GLIBCXX_CONSTEXPR __GLIBCXX_TYPE_INT_N_3
  abs(__GLIBCXX_TYPE_INT_N_3 __x) { return __x >= 0 ? __x : -__x; }
#endif

#if !defined(__STRICT_ANSI__) && defined(_GLIBCXX_USE_FLOAT128) && !defined(__CUDACC__)
  inline _GLIBCXX_CONSTEXPR
  __float128
  abs(__float128 __x)
  { return __x < 0 ? -__x : __x; }
#endif

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace
} // extern "C"++"

#endif // _GLIBCXX_BITS_STD_ABS_H
