// -*- C++ -*-

// Copyright (C) 2007-2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the terms
// of the GNU General Public License as published by the Free Software
// Foundation; either version 3, or (at your option) any later
// version.

// This library is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/** @file ext/numeric_traits.h
 *  This file is a GNU extension to the Standard C++ Library.
 */

#ifndef _EXT_NUMERIC_TRAITS
#define _EXT_NUMERIC_TRAITS 1

#pragma GCC system_header

#include <bits/cpp_type_traits.h>
#include <ext/type_traits.h>

namespace __gnu_cxx _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  // Compile time constants for builtin types.
  // In C++98 std::numeric_limits member functions are not constant expressions
  // (that changed in C++11 with the addition of 'constexpr').
  // Even for C++11, this header is smaller than <limits> and can be used
  // when only is_signed, digits, min, or max values are needed for integers,
  // or is_signed, digits10, max_digits10, or max_exponent10 for floats.

  // Unlike __is_integer (and std::is_integral) this trait is true for
  // non-standard built-in integer types such as __int128 and __int20.
  template<typename _Tp>
    struct __is_integer_nonstrict
    : public std::__is_integer<_Tp>
    {
      using std::__is_integer<_Tp>::__value;

      // The number of bits in the value representation.
      enum { __width = __value ? sizeof(_Tp) * __CHAR_BIT__ : 0 };
    };

  template<typename _Value>
    struct __numeric_traits_integer
    {
#if __cplusplus >= 201103L
      static_assert(__is_integer_nonstrict<_Value>::__value,
		    "invalid specialization");
#endif

      // NB: these two are also available in std::numeric_limits as compile
      // time constants, but <limits> is big and we can avoid including it.
      static const bool __is_signed = (_Value)(-1) < 0;
      static const int __digits
	= __is_integer_nonstrict<_Value>::__width - __is_signed;

      // The initializers must be constants so that __max and __min are too.
      static const _Value __max = __is_signed
	? (((((_Value)1 << (__digits - 1)) - 1) << 1) + 1)
	: ~(_Value)0;
      static const _Value __min = __is_signed ? -__max - 1 : (_Value)0;
    };

  template<typename _Value>
    const _Value __numeric_traits_integer<_Value>::__min;

  template<typename _Value>
    const _Value __numeric_traits_integer<_Value>::__max;

  template<typename _Value>
    const bool __numeric_traits_integer<_Value>::__is_signed;

  template<typename _Value>
    const int __numeric_traits_integer<_Value>::__digits;

  // Enable __numeric_traits_integer for types where the __is_integer_nonstrict
  // primary template doesn't give the right answer.
#define _GLIBCXX_INT_N_TRAITS(T, WIDTH)			\
  template<> struct __is_integer_nonstrict<T>		\
  {							\
    enum { __value = 1 };				\
    typedef std::__true_type __type;			\
    enum { __width = WIDTH };				\
  };							\
  template<> struct __is_integer_nonstrict<unsigned T>	\
  {							\
    enum { __value = 1 };				\
    typedef std::__true_type __type;			\
    enum { __width = WIDTH };				\
  };

  // We need to specify the width for some __intNN types because they
  // have padding bits, e.g. the object representation of __int20 has 32 bits,
  // but its width (number of bits in the value representation) is only 20.
#if defined __GLIBCXX_TYPE_INT_N_0 && __GLIBCXX_BITSIZE_INT_N_0 % __CHAR_BIT__
  _GLIBCXX_INT_N_TRAITS(__GLIBCXX_TYPE_INT_N_0, __GLIBCXX_BITSIZE_INT_N_0)
#endif
#if defined __GLIBCXX_TYPE_INT_N_1 && __GLIBCXX_BITSIZE_INT_N_1 % __CHAR_BIT__
  _GLIBCXX_INT_N_TRAITS(__GLIBCXX_TYPE_INT_N_1, __GLIBCXX_BITSIZE_INT_N_1)
#endif
#if defined __GLIBCXX_TYPE_INT_N_2 && __GLIBCXX_BITSIZE_INT_N_2 % __CHAR_BIT__
  _GLIBCXX_INT_N_TRAITS(__GLIBCXX_TYPE_INT_N_2, __GLIBCXX_BITSIZE_INT_N_2)
#endif
#if defined __GLIBCXX_TYPE_INT_N_3 && __GLIBCXX_BITSIZE_INT_N_3 % __CHAR_BIT__
  _GLIBCXX_INT_N_TRAITS(__GLIBCXX_TYPE_INT_N_3, __GLIBCXX_BITSIZE_INT_N_3)
#endif

#if defined __STRICT_ANSI__ && defined __SIZEOF_INT128__
  // In strict modes __is_integer<__int128> is false,
  // but we still want to define __numeric_traits_integer<__int128>.
  _GLIBCXX_INT_N_TRAITS(__int128, 128)
#endif

#undef _GLIBCXX_INT_N_TRAITS

#if __cplusplus >= 201103L
  /// Convenience alias for __numeric_traits<integer-type>.
  template<typename _Tp>
    using __int_traits = __numeric_traits_integer<_Tp>;
#endif

#define __glibcxx_floating(_Tp, _Fval, _Dval, _LDval) \
  (std::__are_same<_Tp, float>::__value ? _Fval \
   : std::__are_same<_Tp, double>::__value ? _Dval : _LDval)

#define __glibcxx_max_digits10(_Tp) \
  (2 + __glibcxx_floating(_Tp, __FLT_MANT_DIG__, __DBL_MANT_DIG__, \
			  __LDBL_MANT_DIG__) * 643L / 2136)

#define __glibcxx_digits10(_Tp) \
  __glibcxx_floating(_Tp, __FLT_DIG__, __DBL_DIG__, __LDBL_DIG__)

#define __glibcxx_max_exponent10(_Tp) \
  __glibcxx_floating(_Tp, __FLT_MAX_10_EXP__, __DBL_MAX_10_EXP__, \
		     __LDBL_MAX_10_EXP__)

  // N.B. this only supports float, double and long double (no __float128 etc.)
  template<typename _Value>
    struct __numeric_traits_floating
    {
      // Only floating point types. See N1822.
      static const int __max_digits10 = __glibcxx_max_digits10(_Value);

      // See above comment...
      static const bool __is_signed = true;
      static const int __digits10 = __glibcxx_digits10(_Value);
      static const int __max_exponent10 = __glibcxx_max_exponent10(_Value);
    };

  template<typename _Value>
    const int __numeric_traits_floating<_Value>::__max_digits10;

  template<typename _Value>
    const bool __numeric_traits_floating<_Value>::__is_signed;

  template<typename _Value>
    const int __numeric_traits_floating<_Value>::__digits10;

  template<typename _Value>
    const int __numeric_traits_floating<_Value>::__max_exponent10;

  template<typename _Value>
    struct __numeric_traits
    : public __conditional_type<__is_integer_nonstrict<_Value>::__value,
				__numeric_traits_integer<_Value>,
				__numeric_traits_floating<_Value> >::__type
    { };

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#undef __glibcxx_floating
#undef __glibcxx_max_digits10
#undef __glibcxx_digits10
#undef __glibcxx_max_exponent10

#endif
