// decimal classes -*- C++ -*-

// Copyright (C) 2009-2020 Free Software Foundation, Inc.

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

/** @file decimal/decimal.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{decimal}
 */

// ISO/IEC TR 24733
// Written by Janis Johnson <janis187@us.ibm.com>

#ifndef _GLIBCXX_DECIMAL_IMPL
#define _GLIBCXX_DECIMAL_IMPL 1

#pragma GCC system_header

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

namespace decimal
{
  // ISO/IEC TR 24733  3.2.[234].1  Construct/copy/destroy.

  inline decimal32::decimal32(decimal64 __r)	: __val(__r.__getval()) {}
  inline decimal32::decimal32(decimal128 __r)	: __val(__r.__getval()) {}
  inline decimal64::decimal64(decimal32 __r)	: __val(__r.__getval()) {}
  inline decimal64::decimal64(decimal128 __r)	: __val(__r.__getval()) {}
  inline decimal128::decimal128(decimal32 __r)	: __val(__r.__getval()) {}
  inline decimal128::decimal128(decimal64 __r)	: __val(__r.__getval()) {}

  // ISO/IEC TR 24733  3.2.[234].6  Compound assignment.

#define _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_DEC(_Op1, _Op2, _T1, _T2)	 \
  inline _T1& _T1::operator _Op1(_T2 __rhs)				 \
  {									 \
    __setval(__getval() _Op2 __rhs.__getval());				 \
    return *this;							 \
  }

#define _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_INT(_Op1, _Op2, _T1, _T2)	 \
  inline _T1& _T1::operator _Op1(_T2 __rhs)				 \
  {									 \
    __setval(__getval() _Op2 __rhs);					 \
    return *this;							 \
  }

#define _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(_Op1, _Op2, _T1)		 \
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_DEC(_Op1, _Op2, _T1, decimal32)	 \
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_DEC(_Op1, _Op2, _T1, decimal64)	 \
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_DEC(_Op1, _Op2, _T1, decimal128)	 \
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_INT(_Op1, _Op2, _T1, int)		 \
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_INT(_Op1, _Op2, _T1, unsigned int) \
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_INT(_Op1, _Op2, _T1, long)	 \
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_INT(_Op1, _Op2, _T1, unsigned long)\
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_INT(_Op1, _Op2, _T1, long long)	 \
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_INT(_Op1, _Op2, _T1, unsigned long long)

  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(+=, +, decimal32)
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(-=, -, decimal32)
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(*=, *, decimal32)
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(/=, /, decimal32)

  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(+=, +, decimal64)
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(-=, -, decimal64)
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(*=, *, decimal64)
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(/=, /, decimal64)

  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(+=, +, decimal128)
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(-=, -, decimal128)
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(*=, *, decimal128)
  _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS(/=, /, decimal128)

#undef _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_DEC
#undef _DEFINE_DECIMAL_COMPOUND_ASSIGNMENT_INT
#undef _DEFINE_DECIMAL_COMPOUND_ASSIGNMENTS

  // Extension: Conversion to integral type.

  inline long long decimal32_to_long_long(decimal32 __d)
  { return (long long)__d.__getval(); }

  inline long long decimal64_to_long_long(decimal64 __d)
  { return (long long)__d.__getval(); }

  inline long long decimal128_to_long_long(decimal128 __d)
  { return (long long)__d.__getval(); }

  inline long long decimal_to_long_long(decimal32 __d)
  { return (long long)__d.__getval(); }

  inline long long decimal_to_long_long(decimal64 __d)
  { return (long long)__d.__getval(); }

  inline long long decimal_to_long_long(decimal128 __d)
  { return (long long)__d.__getval(); }

  // ISO/IEC TR 24733  3.2.5  Initialization from coefficient and exponent.

  static decimal32 make_decimal32(long long __coeff, int __exponent)
  {
    decimal32 __decexp = 1, __multiplier;

    if (__exponent < 0)
      {
	__multiplier = 1.E-1DF;
	__exponent = -__exponent;
      }
    else
      __multiplier = 1.E1DF;

    for (int __i = 0; __i < __exponent; ++__i)
      __decexp *= __multiplier;

    return __coeff * __decexp;
  }

  static decimal32 make_decimal32(unsigned long long __coeff, int __exponent)
  {
    decimal32 __decexp = 1, __multiplier;

    if (__exponent < 0)
      {
	__multiplier = 1.E-1DF;
	__exponent = -__exponent;
      }
    else
      __multiplier = 1.E1DF;

    for (int __i = 0; __i < __exponent; ++__i)
      __decexp *= __multiplier;

    return __coeff * __decexp;
  }

  static decimal64 make_decimal64(long long __coeff, int __exponent)
  {
    decimal64 __decexp = 1, __multiplier;

    if (__exponent < 0)
      {
	__multiplier = 1.E-1DD;
	__exponent = -__exponent;
      }
    else
      __multiplier = 1.E1DD;

    for (int __i = 0; __i < __exponent; ++__i)
      __decexp *= __multiplier;

    return __coeff * __decexp;
  }

  static decimal64 make_decimal64(unsigned long long __coeff, int __exponent)
  {
    decimal64 __decexp = 1, __multiplier;

    if (__exponent < 0)
      {
	__multiplier = 1.E-1DD;
	__exponent = -__exponent;
      }
    else
      __multiplier = 1.E1DD;

    for (int __i = 0; __i < __exponent; ++__i)
      __decexp *= __multiplier;

    return __coeff * __decexp;
  }

  static decimal128 make_decimal128(long long __coeff, int __exponent)
  {
    decimal128 __decexp = 1, __multiplier;

    if (__exponent < 0)
      {
	__multiplier = 1.E-1DL;
	__exponent = -__exponent;
      }
    else
      __multiplier = 1.E1DL;

    for (int __i = 0; __i < __exponent; ++__i)
      __decexp *= __multiplier;

    return __coeff * __decexp;
  }

  static decimal128 make_decimal128(unsigned long long __coeff, int __exponent)
  {
    decimal128 __decexp = 1, __multiplier;

    if (__exponent < 0)
      {
	__multiplier = 1.E-1DL;
	__exponent = -__exponent;
      }
    else
      __multiplier = 1.E1DL;

    for (int __i = 0; __i < __exponent; ++__i)
      __decexp *= __multiplier;

    return __coeff * __decexp;
  }

  // ISO/IEC TR 24733  3.2.6  Conversion to generic floating-point type.

  inline float decimal32_to_float(decimal32 __d)
  { return (float)__d.__getval(); }

  inline float decimal64_to_float(decimal64 __d)
  { return (float)__d.__getval(); }

  inline float decimal128_to_float(decimal128 __d)
  { return (float)__d.__getval(); }

  inline float decimal_to_float(decimal32 __d)
  { return (float)__d.__getval(); }

  inline float decimal_to_float(decimal64 __d)
  { return (float)__d.__getval(); }

  inline float decimal_to_float(decimal128 __d)
  { return (float)__d.__getval(); }

  inline double decimal32_to_double(decimal32 __d)
  { return (double)__d.__getval(); }

  inline double decimal64_to_double(decimal64 __d)
  { return (double)__d.__getval(); }

  inline double decimal128_to_double(decimal128 __d)
  { return (double)__d.__getval(); }

  inline double decimal_to_double(decimal32 __d)
  { return (double)__d.__getval(); }

  inline double decimal_to_double(decimal64 __d)
  { return (double)__d.__getval(); }

  inline double decimal_to_double(decimal128 __d)
  { return (double)__d.__getval(); }

  inline long double decimal32_to_long_double(decimal32 __d)
  { return (long double)__d.__getval(); }

  inline long double decimal64_to_long_double(decimal64 __d)
  { return (long double)__d.__getval(); }

  inline long double decimal128_to_long_double(decimal128 __d)
  { return (long double)__d.__getval(); }

  inline long double decimal_to_long_double(decimal32 __d)
  { return (long double)__d.__getval(); }

  inline long double decimal_to_long_double(decimal64 __d)
  { return (long double)__d.__getval(); }

  inline long double decimal_to_long_double(decimal128 __d)
  { return (long double)__d.__getval(); }

  // ISO/IEC TR 24733  3.2.7  Unary arithmetic operators.

#define _DEFINE_DECIMAL_UNARY_OP(_Op, _Tp)	\
  inline _Tp operator _Op(_Tp __rhs)		\
  {						\
    _Tp __tmp;					\
    __tmp.__setval(_Op __rhs.__getval());	\
    return __tmp;				\
  }

  _DEFINE_DECIMAL_UNARY_OP(+, decimal32)
  _DEFINE_DECIMAL_UNARY_OP(+, decimal64)
  _DEFINE_DECIMAL_UNARY_OP(+, decimal128)
  _DEFINE_DECIMAL_UNARY_OP(-, decimal32)
  _DEFINE_DECIMAL_UNARY_OP(-, decimal64)
  _DEFINE_DECIMAL_UNARY_OP(-, decimal128)

#undef _DEFINE_DECIMAL_UNARY_OP

  // ISO/IEC TR 24733  3.2.8  Binary arithmetic operators.

#define _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(_Op, _T1, _T2, _T3)	\
  inline _T1 operator _Op(_T2 __lhs, _T3 __rhs)			\
  {								\
    _T1 __retval;						\
    __retval.__setval(__lhs.__getval() _Op __rhs.__getval());	\
    return __retval;						\
  }

#define _DEFINE_DECIMAL_BINARY_OP_BOTH(_Op, _T1, _T2, _T3)	\
  inline _T1 operator _Op(_T2 __lhs, _T3 __rhs)			\
  {								\
    _T1 __retval;						\
    __retval.__setval(__lhs.__getval() _Op __rhs.__getval());	\
    return __retval;						\
  }

#define _DEFINE_DECIMAL_BINARY_OP_LHS(_Op, _T1, _T2)		\
  inline _T1 operator _Op(_T1 __lhs, _T2 __rhs)			\
  {								\
    _T1 __retval;						\
    __retval.__setval(__lhs.__getval() _Op __rhs);		\
    return __retval;						\
  }

#define _DEFINE_DECIMAL_BINARY_OP_RHS(_Op, _T1, _T2)		\
  inline _T1 operator _Op(_T2 __lhs, _T1 __rhs)			\
  {								\
    _T1 __retval;						\
    __retval.__setval(__lhs _Op __rhs.__getval());		\
    return __retval;						\
  }

#define _DEFINE_DECIMAL_BINARY_OP_WITH_INT(_Op, _T1)		\
  _DEFINE_DECIMAL_BINARY_OP_LHS(_Op, _T1, int);			\
  _DEFINE_DECIMAL_BINARY_OP_LHS(_Op, _T1, unsigned int);	\
  _DEFINE_DECIMAL_BINARY_OP_LHS(_Op, _T1, long);		\
  _DEFINE_DECIMAL_BINARY_OP_LHS(_Op, _T1, unsigned long);	\
  _DEFINE_DECIMAL_BINARY_OP_LHS(_Op, _T1, long long);		\
  _DEFINE_DECIMAL_BINARY_OP_LHS(_Op, _T1, unsigned long long);	\
  _DEFINE_DECIMAL_BINARY_OP_RHS(_Op, _T1, int);			\
  _DEFINE_DECIMAL_BINARY_OP_RHS(_Op, _T1, unsigned int);	\
  _DEFINE_DECIMAL_BINARY_OP_RHS(_Op, _T1, long);		\
  _DEFINE_DECIMAL_BINARY_OP_RHS(_Op, _T1, unsigned long);	\
  _DEFINE_DECIMAL_BINARY_OP_RHS(_Op, _T1, long long);		\
  _DEFINE_DECIMAL_BINARY_OP_RHS(_Op, _T1, unsigned long long);	\

  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(+, decimal32, decimal32, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_INT(+, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(+, decimal64, decimal32, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(+, decimal64, decimal64, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(+, decimal64, decimal64, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_INT(+, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(+, decimal128, decimal32, decimal128)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(+, decimal128, decimal64, decimal128)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(+, decimal128, decimal128, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(+, decimal128, decimal128, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(+, decimal128, decimal128, decimal128)
  _DEFINE_DECIMAL_BINARY_OP_WITH_INT(+, decimal128)

  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(-, decimal32, decimal32, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_INT(-, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(-, decimal64, decimal32, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(-, decimal64, decimal64, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(-, decimal64, decimal64, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_INT(-, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(-, decimal128, decimal32, decimal128)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(-, decimal128, decimal64, decimal128)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(-, decimal128, decimal128, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(-, decimal128, decimal128, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(-, decimal128, decimal128, decimal128)
  _DEFINE_DECIMAL_BINARY_OP_WITH_INT(-, decimal128)

  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(*, decimal32, decimal32, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_INT(*, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(*, decimal64, decimal32, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(*, decimal64, decimal64, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(*, decimal64, decimal64, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_INT(*, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(*, decimal128, decimal32, decimal128)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(*, decimal128, decimal64, decimal128)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(*, decimal128, decimal128, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(*, decimal128, decimal128, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(*, decimal128, decimal128, decimal128)
  _DEFINE_DECIMAL_BINARY_OP_WITH_INT(*, decimal128)

  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(/, decimal32, decimal32, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_INT(/, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(/, decimal64, decimal32, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(/, decimal64, decimal64, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(/, decimal64, decimal64, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_INT(/, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(/, decimal128, decimal32, decimal128)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(/, decimal128, decimal64, decimal128)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(/, decimal128, decimal128, decimal32)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(/, decimal128, decimal128, decimal64)
  _DEFINE_DECIMAL_BINARY_OP_WITH_DEC(/, decimal128, decimal128, decimal128)
  _DEFINE_DECIMAL_BINARY_OP_WITH_INT(/, decimal128)

#undef _DEFINE_DECIMAL_BINARY_OP_WITH_DEC
#undef _DEFINE_DECIMAL_BINARY_OP_BOTH
#undef _DEFINE_DECIMAL_BINARY_OP_LHS
#undef _DEFINE_DECIMAL_BINARY_OP_RHS
#undef _DEFINE_DECIMAL_BINARY_OP_WITH_INT

  // ISO/IEC TR 24733  3.2.9  Comparison operators.

#define _DEFINE_DECIMAL_COMPARISON_BOTH(_Op, _T1, _T2)	\
  inline bool operator _Op(_T1 __lhs, _T2 __rhs)	\
  { return __lhs.__getval() _Op __rhs.__getval(); }

#define _DEFINE_DECIMAL_COMPARISON_LHS(_Op, _T1, _T2)	\
  inline bool operator _Op(_T1 __lhs, _T2 __rhs)	\
  { return __lhs.__getval() _Op __rhs; }

#define _DEFINE_DECIMAL_COMPARISON_RHS(_Op, _T1, _T2)	\
  inline bool operator _Op(_T1 __lhs, _T2 __rhs)	\
  { return __lhs _Op __rhs.__getval(); }

#define _DEFINE_DECIMAL_COMPARISONS(_Op, _Tp)			\
  _DEFINE_DECIMAL_COMPARISON_BOTH(_Op, _Tp, decimal32)		\
  _DEFINE_DECIMAL_COMPARISON_BOTH(_Op, _Tp, decimal64)		\
  _DEFINE_DECIMAL_COMPARISON_BOTH(_Op, _Tp, decimal128)		\
  _DEFINE_DECIMAL_COMPARISON_LHS(_Op, _Tp, int)			\
  _DEFINE_DECIMAL_COMPARISON_LHS(_Op, _Tp, unsigned int)	\
  _DEFINE_DECIMAL_COMPARISON_LHS(_Op, _Tp, long)		\
  _DEFINE_DECIMAL_COMPARISON_LHS(_Op, _Tp, unsigned long)	\
  _DEFINE_DECIMAL_COMPARISON_LHS(_Op, _Tp, long long)		\
  _DEFINE_DECIMAL_COMPARISON_LHS(_Op, _Tp, unsigned long long)	\
  _DEFINE_DECIMAL_COMPARISON_RHS(_Op, int, _Tp)			\
  _DEFINE_DECIMAL_COMPARISON_RHS(_Op, unsigned int, _Tp)	\
  _DEFINE_DECIMAL_COMPARISON_RHS(_Op, long, _Tp)		\
  _DEFINE_DECIMAL_COMPARISON_RHS(_Op, unsigned long, _Tp)	\
  _DEFINE_DECIMAL_COMPARISON_RHS(_Op, long long, _Tp)		\
  _DEFINE_DECIMAL_COMPARISON_RHS(_Op, unsigned long long, _Tp)

  _DEFINE_DECIMAL_COMPARISONS(==, decimal32)
  _DEFINE_DECIMAL_COMPARISONS(==, decimal64)
  _DEFINE_DECIMAL_COMPARISONS(==, decimal128)
  _DEFINE_DECIMAL_COMPARISONS(!=, decimal32)
  _DEFINE_DECIMAL_COMPARISONS(!=, decimal64)
  _DEFINE_DECIMAL_COMPARISONS(!=, decimal128)
  _DEFINE_DECIMAL_COMPARISONS(<,  decimal32)
  _DEFINE_DECIMAL_COMPARISONS(<,  decimal64)
  _DEFINE_DECIMAL_COMPARISONS(<,  decimal128)
  _DEFINE_DECIMAL_COMPARISONS(<=, decimal32)
  _DEFINE_DECIMAL_COMPARISONS(<=, decimal64)
  _DEFINE_DECIMAL_COMPARISONS(<=, decimal128)
  _DEFINE_DECIMAL_COMPARISONS(>,  decimal32)
  _DEFINE_DECIMAL_COMPARISONS(>,  decimal64)
  _DEFINE_DECIMAL_COMPARISONS(>,  decimal128)
  _DEFINE_DECIMAL_COMPARISONS(>=, decimal32)
  _DEFINE_DECIMAL_COMPARISONS(>=, decimal64)
  _DEFINE_DECIMAL_COMPARISONS(>=, decimal128)

#undef _DEFINE_DECIMAL_COMPARISON_BOTH
#undef _DEFINE_DECIMAL_COMPARISON_LHS
#undef _DEFINE_DECIMAL_COMPARISON_RHS
#undef _DEFINE_DECIMAL_COMPARISONS
} // namespace decimal

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif /* _GLIBCXX_DECIMAL_IMPL */
