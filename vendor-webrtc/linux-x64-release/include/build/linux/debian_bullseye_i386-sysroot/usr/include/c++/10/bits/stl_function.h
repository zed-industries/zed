// Functor implementations -*- C++ -*-

// Copyright (C) 2001-2020 Free Software Foundation, Inc.
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

/*
 *
 * Copyright (c) 1994
 * Hewlett-Packard Company
 *
 * Permission to use, copy, modify, distribute and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appear in all copies and
 * that both that copyright notice and this permission notice appear
 * in supporting documentation.  Hewlett-Packard Company makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 *
 *
 * Copyright (c) 1996-1998
 * Silicon Graphics Computer Systems, Inc.
 *
 * Permission to use, copy, modify, distribute and sell this software
 * and its documentation for any purpose is hereby granted without fee,
 * provided that the above copyright notice appear in all copies and
 * that both that copyright notice and this permission notice appear
 * in supporting documentation.  Silicon Graphics makes no
 * representations about the suitability of this software for any
 * purpose.  It is provided "as is" without express or implied warranty.
 */

/** @file bits/stl_function.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{functional}
 */

#ifndef _STL_FUNCTION_H
#define _STL_FUNCTION_H 1

#if __cplusplus > 201103L
#include <bits/move.h>
#endif

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  // 20.3.1 base classes
  /** @defgroup functors Function Objects
   * @ingroup utilities
   *
   *  Function objects, or @e functors, are objects with an @c operator()
   *  defined and accessible.  They can be passed as arguments to algorithm
   *  templates and used in place of a function pointer.  Not only is the
   *  resulting expressiveness of the library increased, but the generated
   *  code can be more efficient than what you might write by hand.  When we
   *  refer to @a functors, then, generally we include function pointers in
   *  the description as well.
   *
   *  Often, functors are only created as temporaries passed to algorithm
   *  calls, rather than being created as named variables.
   *
   *  Two examples taken from the standard itself follow.  To perform a
   *  by-element addition of two vectors @c a and @c b containing @c double,
   *  and put the result in @c a, use
   *  \code
   *  transform (a.begin(), a.end(), b.begin(), a.begin(), plus<double>());
   *  \endcode
   *  To negate every element in @c a, use
   *  \code
   *  transform(a.begin(), a.end(), a.begin(), negate<double>());
   *  \endcode
   *  The addition and negation functions will be inlined directly.
   *
   *  The standard functors are derived from structs named @c unary_function
   *  and @c binary_function.  These two classes contain nothing but typedefs,
   *  to aid in generic (template) programming.  If you write your own
   *  functors, you might consider doing the same.
   *
   *  @{
   */
  /**
   *  This is one of the @link functors functor base classes@endlink.
   */
  template<typename _Arg, typename _Result>
    struct unary_function
    {
      /// @c argument_type is the type of the argument
      typedef _Arg 	argument_type;   

      /// @c result_type is the return type
      typedef _Result 	result_type;  
    };

  /**
   *  This is one of the @link functors functor base classes@endlink.
   */
  template<typename _Arg1, typename _Arg2, typename _Result>
    struct binary_function
    {
      /// @c first_argument_type is the type of the first argument
      typedef _Arg1 	first_argument_type; 

      /// @c second_argument_type is the type of the second argument
      typedef _Arg2 	second_argument_type;

      /// @c result_type is the return type
      typedef _Result 	result_type;
    };
  /** @}  */

  // 20.3.2 arithmetic
  /** @defgroup arithmetic_functors Arithmetic Classes
   * @ingroup functors
   *
   *  Because basic math often needs to be done during an algorithm,
   *  the library provides functors for those operations.  See the
   *  documentation for @link functors the base classes@endlink
   *  for examples of their use.
   *
   *  @{
   */

#if __cplusplus > 201103L
  struct __is_transparent;  // undefined

  template<typename _Tp = void>
    struct plus;

  template<typename _Tp = void>
    struct minus;

  template<typename _Tp = void>
    struct multiplies;

  template<typename _Tp = void>
    struct divides;

  template<typename _Tp = void>
    struct modulus;

  template<typename _Tp = void>
    struct negate;
#endif

  /// One of the @link arithmetic_functors math functors@endlink.
  template<typename _Tp>
    struct plus : public binary_function<_Tp, _Tp, _Tp>
    {
      _GLIBCXX14_CONSTEXPR
      _Tp
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x + __y; }
    };

  /// One of the @link arithmetic_functors math functors@endlink.
  template<typename _Tp>
    struct minus : public binary_function<_Tp, _Tp, _Tp>
    {
      _GLIBCXX14_CONSTEXPR
      _Tp
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x - __y; }
    };

  /// One of the @link arithmetic_functors math functors@endlink.
  template<typename _Tp>
    struct multiplies : public binary_function<_Tp, _Tp, _Tp>
    {
      _GLIBCXX14_CONSTEXPR
      _Tp
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x * __y; }
    };

  /// One of the @link arithmetic_functors math functors@endlink.
  template<typename _Tp>
    struct divides : public binary_function<_Tp, _Tp, _Tp>
    {
      _GLIBCXX14_CONSTEXPR
      _Tp
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x / __y; }
    };

  /// One of the @link arithmetic_functors math functors@endlink.
  template<typename _Tp>
    struct modulus : public binary_function<_Tp, _Tp, _Tp>
    {
      _GLIBCXX14_CONSTEXPR
      _Tp
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x % __y; }
    };

  /// One of the @link arithmetic_functors math functors@endlink.
  template<typename _Tp>
    struct negate : public unary_function<_Tp, _Tp>
    {
      _GLIBCXX14_CONSTEXPR
      _Tp
      operator()(const _Tp& __x) const
      { return -__x; }
    };

#if __cplusplus > 201103L

#define __cpp_lib_transparent_operators 201510

  template<>
    struct plus<void>
    {
      template <typename _Tp, typename _Up>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) + std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) + std::forward<_Up>(__u))
	{ return std::forward<_Tp>(__t) + std::forward<_Up>(__u); }

      typedef __is_transparent is_transparent;
    };

  /// One of the @link arithmetic_functors math functors@endlink.
  template<>
    struct minus<void>
    {
      template <typename _Tp, typename _Up>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) - std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) - std::forward<_Up>(__u))
	{ return std::forward<_Tp>(__t) - std::forward<_Up>(__u); }

      typedef __is_transparent is_transparent;
    };

  /// One of the @link arithmetic_functors math functors@endlink.
  template<>
    struct multiplies<void>
    {
      template <typename _Tp, typename _Up>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) * std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) * std::forward<_Up>(__u))
	{ return std::forward<_Tp>(__t) * std::forward<_Up>(__u); }

      typedef __is_transparent is_transparent;
    };

  /// One of the @link arithmetic_functors math functors@endlink.
  template<>
    struct divides<void>
    {
      template <typename _Tp, typename _Up>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) / std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) / std::forward<_Up>(__u))
	{ return std::forward<_Tp>(__t) / std::forward<_Up>(__u); }

      typedef __is_transparent is_transparent;
    };

  /// One of the @link arithmetic_functors math functors@endlink.
  template<>
    struct modulus<void>
    {
      template <typename _Tp, typename _Up>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) % std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) % std::forward<_Up>(__u))
	{ return std::forward<_Tp>(__t) % std::forward<_Up>(__u); }

      typedef __is_transparent is_transparent;
    };

  /// One of the @link arithmetic_functors math functors@endlink.
  template<>
    struct negate<void>
    {
      template <typename _Tp>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t) const
	noexcept(noexcept(-std::forward<_Tp>(__t)))
	-> decltype(-std::forward<_Tp>(__t))
	{ return -std::forward<_Tp>(__t); }

      typedef __is_transparent is_transparent;
    };
#endif
  /** @}  */

  // 20.3.3 comparisons
  /** @defgroup comparison_functors Comparison Classes
   * @ingroup functors
   *
   *  The library provides six wrapper functors for all the basic comparisons
   *  in C++, like @c <.
   *
   *  @{
   */
#if __cplusplus > 201103L
  template<typename _Tp = void>
    struct equal_to;

  template<typename _Tp = void>
    struct not_equal_to;

  template<typename _Tp = void>
    struct greater;

  template<typename _Tp = void>
    struct less;

  template<typename _Tp = void>
    struct greater_equal;

  template<typename _Tp = void>
    struct less_equal;
#endif

  /// One of the @link comparison_functors comparison functors@endlink.
  template<typename _Tp>
    struct equal_to : public binary_function<_Tp, _Tp, bool>
    {
      _GLIBCXX14_CONSTEXPR
      bool
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x == __y; }
    };

  /// One of the @link comparison_functors comparison functors@endlink.
  template<typename _Tp>
    struct not_equal_to : public binary_function<_Tp, _Tp, bool>
    {
      _GLIBCXX14_CONSTEXPR
      bool
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x != __y; }
    };

  /// One of the @link comparison_functors comparison functors@endlink.
  template<typename _Tp>
    struct greater : public binary_function<_Tp, _Tp, bool>
    {
      _GLIBCXX14_CONSTEXPR
      bool
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x > __y; }
    };

  /// One of the @link comparison_functors comparison functors@endlink.
  template<typename _Tp>
    struct less : public binary_function<_Tp, _Tp, bool>
    {
      _GLIBCXX14_CONSTEXPR
      bool
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x < __y; }
    };

  /// One of the @link comparison_functors comparison functors@endlink.
  template<typename _Tp>
    struct greater_equal : public binary_function<_Tp, _Tp, bool>
    {
      _GLIBCXX14_CONSTEXPR
      bool
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x >= __y; }
    };

  /// One of the @link comparison_functors comparison functors@endlink.
  template<typename _Tp>
    struct less_equal : public binary_function<_Tp, _Tp, bool>
    {
      _GLIBCXX14_CONSTEXPR
      bool
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x <= __y; }
    };

  // Partial specialization of std::greater for pointers.
  template<typename _Tp>
    struct greater<_Tp*> : public binary_function<_Tp*, _Tp*, bool>
    {
      _GLIBCXX14_CONSTEXPR bool
      operator()(_Tp* __x, _Tp* __y) const _GLIBCXX_NOTHROW
      {
#if __cplusplus >= 201402L
#ifdef _GLIBCXX_HAVE_BUILTIN_IS_CONSTANT_EVALUATED
	if (__builtin_is_constant_evaluated())
#else
	if (__builtin_constant_p(__x > __y))
#endif
	  return __x > __y;
#endif
	return (__UINTPTR_TYPE__)__x > (__UINTPTR_TYPE__)__y;
      }
    };

  // Partial specialization of std::less for pointers.
  template<typename _Tp>
    struct less<_Tp*> : public binary_function<_Tp*, _Tp*, bool>
    {
      _GLIBCXX14_CONSTEXPR bool
      operator()(_Tp* __x, _Tp* __y) const _GLIBCXX_NOTHROW
      {
#if __cplusplus >= 201402L
#ifdef _GLIBCXX_HAVE_BUILTIN_IS_CONSTANT_EVALUATED
	if (__builtin_is_constant_evaluated())
#else
	if (__builtin_constant_p(__x < __y))
#endif
	  return __x < __y;
#endif
	return (__UINTPTR_TYPE__)__x < (__UINTPTR_TYPE__)__y;
      }
    };

  // Partial specialization of std::greater_equal for pointers.
  template<typename _Tp>
    struct greater_equal<_Tp*> : public binary_function<_Tp*, _Tp*, bool>
    {
      _GLIBCXX14_CONSTEXPR bool
      operator()(_Tp* __x, _Tp* __y) const _GLIBCXX_NOTHROW
      {
#if __cplusplus >= 201402L
#ifdef _GLIBCXX_HAVE_BUILTIN_IS_CONSTANT_EVALUATED
	if (__builtin_is_constant_evaluated())
#else
	if (__builtin_constant_p(__x >= __y))
#endif
	  return __x >= __y;
#endif
	return (__UINTPTR_TYPE__)__x >= (__UINTPTR_TYPE__)__y;
      }
    };

  // Partial specialization of std::less_equal for pointers.
  template<typename _Tp>
    struct less_equal<_Tp*> : public binary_function<_Tp*, _Tp*, bool>
    {
      _GLIBCXX14_CONSTEXPR bool
      operator()(_Tp* __x, _Tp* __y) const _GLIBCXX_NOTHROW
      {
#if __cplusplus >= 201402L
#ifdef _GLIBCXX_HAVE_BUILTIN_IS_CONSTANT_EVALUATED
	if (__builtin_is_constant_evaluated())
#else
	if (__builtin_constant_p(__x <= __y))
#endif
	  return __x <= __y;
#endif
	return (__UINTPTR_TYPE__)__x <= (__UINTPTR_TYPE__)__y;
      }
    };

#if __cplusplus >= 201402L
  /// One of the @link comparison_functors comparison functors@endlink.
  template<>
    struct equal_to<void>
    {
      template <typename _Tp, typename _Up>
	constexpr auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) == std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) == std::forward<_Up>(__u))
	{ return std::forward<_Tp>(__t) == std::forward<_Up>(__u); }

      typedef __is_transparent is_transparent;
    };

  /// One of the @link comparison_functors comparison functors@endlink.
  template<>
    struct not_equal_to<void>
    {
      template <typename _Tp, typename _Up>
	constexpr auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) != std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) != std::forward<_Up>(__u))
	{ return std::forward<_Tp>(__t) != std::forward<_Up>(__u); }

      typedef __is_transparent is_transparent;
    };

  /// One of the @link comparison_functors comparison functors@endlink.
  template<>
    struct greater<void>
    {
      template <typename _Tp, typename _Up>
	constexpr auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) > std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) > std::forward<_Up>(__u))
	{
	  return _S_cmp(std::forward<_Tp>(__t), std::forward<_Up>(__u),
			__ptr_cmp<_Tp, _Up>{});
	}

      template<typename _Tp, typename _Up>
	constexpr bool
	operator()(_Tp* __t, _Up* __u) const noexcept
	{ return greater<common_type_t<_Tp*, _Up*>>{}(__t, __u); }

      typedef __is_transparent is_transparent;

    private:
      template <typename _Tp, typename _Up>
	static constexpr decltype(auto)
	_S_cmp(_Tp&& __t, _Up&& __u, false_type)
	{ return std::forward<_Tp>(__t) > std::forward<_Up>(__u); }

      template <typename _Tp, typename _Up>
	static constexpr bool
	_S_cmp(_Tp&& __t, _Up&& __u, true_type) noexcept
	{
	  return greater<const volatile void*>{}(
	      static_cast<const volatile void*>(std::forward<_Tp>(__t)),
	      static_cast<const volatile void*>(std::forward<_Up>(__u)));
	}

      // True if there is no viable operator> member function.
      template<typename _Tp, typename _Up, typename = void>
	struct __not_overloaded2 : true_type { };

      // False if we can call T.operator>(U)
      template<typename _Tp, typename _Up>
	struct __not_overloaded2<_Tp, _Up, __void_t<
	  decltype(std::declval<_Tp>().operator>(std::declval<_Up>()))>>
	: false_type { };

      // True if there is no overloaded operator> for these operands.
      template<typename _Tp, typename _Up, typename = void>
	struct __not_overloaded : __not_overloaded2<_Tp, _Up> { };

      // False if we can call operator>(T,U)
      template<typename _Tp, typename _Up>
	struct __not_overloaded<_Tp, _Up, __void_t<
	  decltype(operator>(std::declval<_Tp>(), std::declval<_Up>()))>>
	: false_type { };

      template<typename _Tp, typename _Up>
	using __ptr_cmp = __and_<__not_overloaded<_Tp, _Up>,
	      is_convertible<_Tp, const volatile void*>,
	      is_convertible<_Up, const volatile void*>>;
    };

  /// One of the @link comparison_functors comparison functors@endlink.
  template<>
    struct less<void>
    {
      template <typename _Tp, typename _Up>
	constexpr auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) < std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) < std::forward<_Up>(__u))
	{
	  return _S_cmp(std::forward<_Tp>(__t), std::forward<_Up>(__u),
			__ptr_cmp<_Tp, _Up>{});
	}

      template<typename _Tp, typename _Up>
	constexpr bool
	operator()(_Tp* __t, _Up* __u) const noexcept
	{ return less<common_type_t<_Tp*, _Up*>>{}(__t, __u); }

      typedef __is_transparent is_transparent;

    private:
      template <typename _Tp, typename _Up>
	static constexpr decltype(auto)
	_S_cmp(_Tp&& __t, _Up&& __u, false_type)
	{ return std::forward<_Tp>(__t) < std::forward<_Up>(__u); }

      template <typename _Tp, typename _Up>
	static constexpr bool
	_S_cmp(_Tp&& __t, _Up&& __u, true_type) noexcept
	{
	  return less<const volatile void*>{}(
	      static_cast<const volatile void*>(std::forward<_Tp>(__t)),
	      static_cast<const volatile void*>(std::forward<_Up>(__u)));
	}

      // True if there is no viable operator< member function.
      template<typename _Tp, typename _Up, typename = void>
	struct __not_overloaded2 : true_type { };

      // False if we can call T.operator<(U)
      template<typename _Tp, typename _Up>
	struct __not_overloaded2<_Tp, _Up, __void_t<
	  decltype(std::declval<_Tp>().operator<(std::declval<_Up>()))>>
	: false_type { };

      // True if there is no overloaded operator< for these operands.
      template<typename _Tp, typename _Up, typename = void>
	struct __not_overloaded : __not_overloaded2<_Tp, _Up> { };

      // False if we can call operator<(T,U)
      template<typename _Tp, typename _Up>
	struct __not_overloaded<_Tp, _Up, __void_t<
	  decltype(operator<(std::declval<_Tp>(), std::declval<_Up>()))>>
	: false_type { };

      template<typename _Tp, typename _Up>
	using __ptr_cmp = __and_<__not_overloaded<_Tp, _Up>,
	      is_convertible<_Tp, const volatile void*>,
	      is_convertible<_Up, const volatile void*>>;
    };

  /// One of the @link comparison_functors comparison functors@endlink.
  template<>
    struct greater_equal<void>
    {
      template <typename _Tp, typename _Up>
	constexpr auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) >= std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) >= std::forward<_Up>(__u))
	{
	  return _S_cmp(std::forward<_Tp>(__t), std::forward<_Up>(__u),
			__ptr_cmp<_Tp, _Up>{});
	}

      template<typename _Tp, typename _Up>
	constexpr bool
	operator()(_Tp* __t, _Up* __u) const noexcept
	{ return greater_equal<common_type_t<_Tp*, _Up*>>{}(__t, __u); }

      typedef __is_transparent is_transparent;

    private:
      template <typename _Tp, typename _Up>
	static constexpr decltype(auto)
	_S_cmp(_Tp&& __t, _Up&& __u, false_type)
	{ return std::forward<_Tp>(__t) >= std::forward<_Up>(__u); }

      template <typename _Tp, typename _Up>
	static constexpr bool
	_S_cmp(_Tp&& __t, _Up&& __u, true_type) noexcept
	{
	  return greater_equal<const volatile void*>{}(
	      static_cast<const volatile void*>(std::forward<_Tp>(__t)),
	      static_cast<const volatile void*>(std::forward<_Up>(__u)));
	}

      // True if there is no viable operator>= member function.
      template<typename _Tp, typename _Up, typename = void>
	struct __not_overloaded2 : true_type { };

      // False if we can call T.operator>=(U)
      template<typename _Tp, typename _Up>
	struct __not_overloaded2<_Tp, _Up, __void_t<
	  decltype(std::declval<_Tp>().operator>=(std::declval<_Up>()))>>
	: false_type { };

      // True if there is no overloaded operator>= for these operands.
      template<typename _Tp, typename _Up, typename = void>
	struct __not_overloaded : __not_overloaded2<_Tp, _Up> { };

      // False if we can call operator>=(T,U)
      template<typename _Tp, typename _Up>
	struct __not_overloaded<_Tp, _Up, __void_t<
	  decltype(operator>=(std::declval<_Tp>(), std::declval<_Up>()))>>
	: false_type { };

      template<typename _Tp, typename _Up>
	using __ptr_cmp = __and_<__not_overloaded<_Tp, _Up>,
	      is_convertible<_Tp, const volatile void*>,
	      is_convertible<_Up, const volatile void*>>;
    };

  /// One of the @link comparison_functors comparison functors@endlink.
  template<>
    struct less_equal<void>
    {
      template <typename _Tp, typename _Up>
	constexpr auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) <= std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) <= std::forward<_Up>(__u))
	{
	  return _S_cmp(std::forward<_Tp>(__t), std::forward<_Up>(__u),
			__ptr_cmp<_Tp, _Up>{});
	}

      template<typename _Tp, typename _Up>
	constexpr bool
	operator()(_Tp* __t, _Up* __u) const noexcept
	{ return less_equal<common_type_t<_Tp*, _Up*>>{}(__t, __u); }

      typedef __is_transparent is_transparent;

    private:
      template <typename _Tp, typename _Up>
	static constexpr decltype(auto)
	_S_cmp(_Tp&& __t, _Up&& __u, false_type)
	{ return std::forward<_Tp>(__t) <= std::forward<_Up>(__u); }

      template <typename _Tp, typename _Up>
	static constexpr bool
	_S_cmp(_Tp&& __t, _Up&& __u, true_type) noexcept
	{
	  return less_equal<const volatile void*>{}(
	      static_cast<const volatile void*>(std::forward<_Tp>(__t)),
	      static_cast<const volatile void*>(std::forward<_Up>(__u)));
	}

      // True if there is no viable operator<= member function.
      template<typename _Tp, typename _Up, typename = void>
	struct __not_overloaded2 : true_type { };

      // False if we can call T.operator<=(U)
      template<typename _Tp, typename _Up>
	struct __not_overloaded2<_Tp, _Up, __void_t<
	  decltype(std::declval<_Tp>().operator<=(std::declval<_Up>()))>>
	: false_type { };

      // True if there is no overloaded operator<= for these operands.
      template<typename _Tp, typename _Up, typename = void>
	struct __not_overloaded : __not_overloaded2<_Tp, _Up> { };

      // False if we can call operator<=(T,U)
      template<typename _Tp, typename _Up>
	struct __not_overloaded<_Tp, _Up, __void_t<
	  decltype(operator<=(std::declval<_Tp>(), std::declval<_Up>()))>>
	: false_type { };

      template<typename _Tp, typename _Up>
	using __ptr_cmp = __and_<__not_overloaded<_Tp, _Up>,
	      is_convertible<_Tp, const volatile void*>,
	      is_convertible<_Up, const volatile void*>>;
    };
#endif // C++14
  /** @}  */

  // 20.3.4 logical operations
  /** @defgroup logical_functors Boolean Operations Classes
   * @ingroup functors
   *
   *  Here are wrapper functors for Boolean operations: @c &&, @c ||,
   *  and @c !.
   *
   *  @{
   */
#if __cplusplus > 201103L
  template<typename _Tp = void>
    struct logical_and;

  template<typename _Tp = void>
    struct logical_or;

  template<typename _Tp = void>
    struct logical_not;
#endif

  /// One of the @link logical_functors Boolean operations functors@endlink.
  template<typename _Tp>
    struct logical_and : public binary_function<_Tp, _Tp, bool>
    {
      _GLIBCXX14_CONSTEXPR
      bool
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x && __y; }
    };

  /// One of the @link logical_functors Boolean operations functors@endlink.
  template<typename _Tp>
    struct logical_or : public binary_function<_Tp, _Tp, bool>
    {
      _GLIBCXX14_CONSTEXPR
      bool
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x || __y; }
    };

  /// One of the @link logical_functors Boolean operations functors@endlink.
  template<typename _Tp>
    struct logical_not : public unary_function<_Tp, bool>
    {
      _GLIBCXX14_CONSTEXPR
      bool
      operator()(const _Tp& __x) const
      { return !__x; }
    };

#if __cplusplus > 201103L
  /// One of the @link logical_functors Boolean operations functors@endlink.
  template<>
    struct logical_and<void>
    {
      template <typename _Tp, typename _Up>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) && std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) && std::forward<_Up>(__u))
	{ return std::forward<_Tp>(__t) && std::forward<_Up>(__u); }

      typedef __is_transparent is_transparent;
    };

  /// One of the @link logical_functors Boolean operations functors@endlink.
  template<>
    struct logical_or<void>
    {
      template <typename _Tp, typename _Up>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) || std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) || std::forward<_Up>(__u))
	{ return std::forward<_Tp>(__t) || std::forward<_Up>(__u); }

      typedef __is_transparent is_transparent;
    };

  /// One of the @link logical_functors Boolean operations functors@endlink.
  template<>
    struct logical_not<void>
    {
      template <typename _Tp>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t) const
	noexcept(noexcept(!std::forward<_Tp>(__t)))
	-> decltype(!std::forward<_Tp>(__t))
	{ return !std::forward<_Tp>(__t); }

      typedef __is_transparent is_transparent;
    };
#endif
  /** @}  */

#if __cplusplus > 201103L
  template<typename _Tp = void>
    struct bit_and;

  template<typename _Tp = void>
    struct bit_or;

  template<typename _Tp = void>
    struct bit_xor;

  template<typename _Tp = void>
    struct bit_not;
#endif

  // _GLIBCXX_RESOLVE_LIB_DEFECTS
  // DR 660. Missing Bitwise Operations.
  template<typename _Tp>
    struct bit_and : public binary_function<_Tp, _Tp, _Tp>
    {
      _GLIBCXX14_CONSTEXPR
      _Tp
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x & __y; }
    };

  template<typename _Tp>
    struct bit_or : public binary_function<_Tp, _Tp, _Tp>
    {
      _GLIBCXX14_CONSTEXPR
      _Tp
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x | __y; }
    };

  template<typename _Tp>
    struct bit_xor : public binary_function<_Tp, _Tp, _Tp>
    {
      _GLIBCXX14_CONSTEXPR
      _Tp
      operator()(const _Tp& __x, const _Tp& __y) const
      { return __x ^ __y; }
    };

  template<typename _Tp>
    struct bit_not : public unary_function<_Tp, _Tp>
    {
    _GLIBCXX14_CONSTEXPR
      _Tp
      operator()(const _Tp& __x) const
      { return ~__x; }
    };

#if __cplusplus > 201103L
  template <>
    struct bit_and<void>
    {
      template <typename _Tp, typename _Up>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) & std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) & std::forward<_Up>(__u))
	{ return std::forward<_Tp>(__t) & std::forward<_Up>(__u); }

      typedef __is_transparent is_transparent;
    };

  template <>
    struct bit_or<void>
    {
      template <typename _Tp, typename _Up>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) | std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) | std::forward<_Up>(__u))
	{ return std::forward<_Tp>(__t) | std::forward<_Up>(__u); }

      typedef __is_transparent is_transparent;
    };

  template <>
    struct bit_xor<void>
    {
      template <typename _Tp, typename _Up>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t, _Up&& __u) const
	noexcept(noexcept(std::forward<_Tp>(__t) ^ std::forward<_Up>(__u)))
	-> decltype(std::forward<_Tp>(__t) ^ std::forward<_Up>(__u))
	{ return std::forward<_Tp>(__t) ^ std::forward<_Up>(__u); }

      typedef __is_transparent is_transparent;
    };

  template <>
    struct bit_not<void>
    {
      template <typename _Tp>
	_GLIBCXX14_CONSTEXPR
	auto
	operator()(_Tp&& __t) const
	noexcept(noexcept(~std::forward<_Tp>(__t)))
	-> decltype(~std::forward<_Tp>(__t))
	{ return ~std::forward<_Tp>(__t); }

      typedef __is_transparent is_transparent;
    };
#endif

  // 20.3.5 negators
  /** @defgroup negators Negators
   * @ingroup functors
   *
   *  The functions @c not1 and @c not2 each take a predicate functor
   *  and return an instance of @c unary_negate or
   *  @c binary_negate, respectively.  These classes are functors whose
   *  @c operator() performs the stored predicate function and then returns
   *  the negation of the result.
   *
   *  For example, given a vector of integers and a trivial predicate,
   *  \code
   *  struct IntGreaterThanThree
   *    : public std::unary_function<int, bool>
   *  {
   *      bool operator() (int x) { return x > 3; }
   *  };
   *
   *  std::find_if (v.begin(), v.end(), not1(IntGreaterThanThree()));
   *  \endcode
   *  The call to @c find_if will locate the first index (i) of @c v for which
   *  <code>!(v[i] > 3)</code> is true.
   *
   *  The not1/unary_negate combination works on predicates taking a single
   *  argument.  The not2/binary_negate combination works on predicates which
   *  take two arguments.
   *
   *  @{
   */
  /// One of the @link negators negation functors@endlink.
  template<typename _Predicate>
    class unary_negate
    : public unary_function<typename _Predicate::argument_type, bool>
    {
    protected:
      _Predicate _M_pred;

    public:
      _GLIBCXX14_CONSTEXPR
      explicit
      unary_negate(const _Predicate& __x) : _M_pred(__x) { }

      _GLIBCXX14_CONSTEXPR
      bool
      operator()(const typename _Predicate::argument_type& __x) const
      { return !_M_pred(__x); }
    };

  /// One of the @link negators negation functors@endlink.
  template<typename _Predicate>
    _GLIBCXX14_CONSTEXPR
    inline unary_negate<_Predicate>
    not1(const _Predicate& __pred)
    { return unary_negate<_Predicate>(__pred); }

  /// One of the @link negators negation functors@endlink.
  template<typename _Predicate>
    class binary_negate
    : public binary_function<typename _Predicate::first_argument_type,
			     typename _Predicate::second_argument_type, bool>
    {
    protected:
      _Predicate _M_pred;

    public:
      _GLIBCXX14_CONSTEXPR
      explicit
      binary_negate(const _Predicate& __x) : _M_pred(__x) { }

      _GLIBCXX14_CONSTEXPR
      bool
      operator()(const typename _Predicate::first_argument_type& __x,
		 const typename _Predicate::second_argument_type& __y) const
      { return !_M_pred(__x, __y); }
    };

  /// One of the @link negators negation functors@endlink.
  template<typename _Predicate>
    _GLIBCXX14_CONSTEXPR
    inline binary_negate<_Predicate>
    not2(const _Predicate& __pred)
    { return binary_negate<_Predicate>(__pred); }
  /** @}  */

  // 20.3.7 adaptors pointers functions
  /** @defgroup pointer_adaptors Adaptors for pointers to functions
   * @ingroup functors
   *
   *  The advantage of function objects over pointers to functions is that
   *  the objects in the standard library declare nested typedefs describing
   *  their argument and result types with uniform names (e.g., @c result_type
   *  from the base classes @c unary_function and @c binary_function).
   *  Sometimes those typedefs are required, not just optional.
   *
   *  Adaptors are provided to turn pointers to unary (single-argument) and
   *  binary (double-argument) functions into function objects.  The
   *  long-winded functor @c pointer_to_unary_function is constructed with a
   *  function pointer @c f, and its @c operator() called with argument @c x
   *  returns @c f(x).  The functor @c pointer_to_binary_function does the same
   *  thing, but with a double-argument @c f and @c operator().
   *
   *  The function @c ptr_fun takes a pointer-to-function @c f and constructs
   *  an instance of the appropriate functor.
   *
   *  @{
   */
  /// One of the @link pointer_adaptors adaptors for function pointers@endlink.
  template<typename _Arg, typename _Result>
    class pointer_to_unary_function : public unary_function<_Arg, _Result>
    {
    protected:
      _Result (*_M_ptr)(_Arg);

    public:
      pointer_to_unary_function() { }

      explicit
      pointer_to_unary_function(_Result (*__x)(_Arg))
      : _M_ptr(__x) { }

      _Result
      operator()(_Arg __x) const
      { return _M_ptr(__x); }
    };

  /// One of the @link pointer_adaptors adaptors for function pointers@endlink.
  template<typename _Arg, typename _Result>
    inline pointer_to_unary_function<_Arg, _Result>
    ptr_fun(_Result (*__x)(_Arg))
    { return pointer_to_unary_function<_Arg, _Result>(__x); }

  /// One of the @link pointer_adaptors adaptors for function pointers@endlink.
  template<typename _Arg1, typename _Arg2, typename _Result>
    class pointer_to_binary_function
    : public binary_function<_Arg1, _Arg2, _Result>
    {
    protected:
      _Result (*_M_ptr)(_Arg1, _Arg2);

    public:
      pointer_to_binary_function() { }

      explicit
      pointer_to_binary_function(_Result (*__x)(_Arg1, _Arg2))
      : _M_ptr(__x) { }

      _Result
      operator()(_Arg1 __x, _Arg2 __y) const
      { return _M_ptr(__x, __y); }
    };

  /// One of the @link pointer_adaptors adaptors for function pointers@endlink.
  template<typename _Arg1, typename _Arg2, typename _Result>
    inline pointer_to_binary_function<_Arg1, _Arg2, _Result>
    ptr_fun(_Result (*__x)(_Arg1, _Arg2))
    { return pointer_to_binary_function<_Arg1, _Arg2, _Result>(__x); }
  /** @}  */

  template<typename _Tp>
    struct _Identity
    : public unary_function<_Tp, _Tp>
    {
      _Tp&
      operator()(_Tp& __x) const
      { return __x; }

      const _Tp&
      operator()(const _Tp& __x) const
      { return __x; }
    };

  // Partial specialization, avoids confusing errors in e.g. std::set<const T>.
  template<typename _Tp> struct _Identity<const _Tp> : _Identity<_Tp> { };

  template<typename _Pair>
    struct _Select1st
    : public unary_function<_Pair, typename _Pair::first_type>
    {
      typename _Pair::first_type&
      operator()(_Pair& __x) const
      { return __x.first; }

      const typename _Pair::first_type&
      operator()(const _Pair& __x) const
      { return __x.first; }

#if __cplusplus >= 201103L
      template<typename _Pair2>
        typename _Pair2::first_type&
        operator()(_Pair2& __x) const
        { return __x.first; }

      template<typename _Pair2>
        const typename _Pair2::first_type&
        operator()(const _Pair2& __x) const
        { return __x.first; }
#endif
    };

  template<typename _Pair>
    struct _Select2nd
    : public unary_function<_Pair, typename _Pair::second_type>
    {
      typename _Pair::second_type&
      operator()(_Pair& __x) const
      { return __x.second; }

      const typename _Pair::second_type&
      operator()(const _Pair& __x) const
      { return __x.second; }
    };

  // 20.3.8 adaptors pointers members
  /** @defgroup memory_adaptors Adaptors for pointers to members
   * @ingroup functors
   *
   *  There are a total of 8 = 2^3 function objects in this family.
   *   (1) Member functions taking no arguments vs member functions taking
   *        one argument.
   *   (2) Call through pointer vs call through reference.
   *   (3) Const vs non-const member function.
   *
   *  All of this complexity is in the function objects themselves.  You can
   *   ignore it by using the helper function mem_fun and mem_fun_ref,
   *   which create whichever type of adaptor is appropriate.
   *
   *  @{
   */
  /// One of the @link memory_adaptors adaptors for member
  /// pointers@endlink.
  template<typename _Ret, typename _Tp>
    class mem_fun_t : public unary_function<_Tp*, _Ret>
    {
    public:
      explicit
      mem_fun_t(_Ret (_Tp::*__pf)())
      : _M_f(__pf) { }

      _Ret
      operator()(_Tp* __p) const
      { return (__p->*_M_f)(); }

    private:
      _Ret (_Tp::*_M_f)();
    };

  /// One of the @link memory_adaptors adaptors for member
  /// pointers@endlink.
  template<typename _Ret, typename _Tp>
    class const_mem_fun_t : public unary_function<const _Tp*, _Ret>
    {
    public:
      explicit
      const_mem_fun_t(_Ret (_Tp::*__pf)() const)
      : _M_f(__pf) { }

      _Ret
      operator()(const _Tp* __p) const
      { return (__p->*_M_f)(); }

    private:
      _Ret (_Tp::*_M_f)() const;
    };

  /// One of the @link memory_adaptors adaptors for member
  /// pointers@endlink.
  template<typename _Ret, typename _Tp>
    class mem_fun_ref_t : public unary_function<_Tp, _Ret>
    {
    public:
      explicit
      mem_fun_ref_t(_Ret (_Tp::*__pf)())
      : _M_f(__pf) { }

      _Ret
      operator()(_Tp& __r) const
      { return (__r.*_M_f)(); }

    private:
      _Ret (_Tp::*_M_f)();
  };

  /// One of the @link memory_adaptors adaptors for member
  /// pointers@endlink.
  template<typename _Ret, typename _Tp>
    class const_mem_fun_ref_t : public unary_function<_Tp, _Ret>
    {
    public:
      explicit
      const_mem_fun_ref_t(_Ret (_Tp::*__pf)() const)
      : _M_f(__pf) { }

      _Ret
      operator()(const _Tp& __r) const
      { return (__r.*_M_f)(); }

    private:
      _Ret (_Tp::*_M_f)() const;
    };

  /// One of the @link memory_adaptors adaptors for member
  /// pointers@endlink.
  template<typename _Ret, typename _Tp, typename _Arg>
    class mem_fun1_t : public binary_function<_Tp*, _Arg, _Ret>
    {
    public:
      explicit
      mem_fun1_t(_Ret (_Tp::*__pf)(_Arg))
      : _M_f(__pf) { }

      _Ret
      operator()(_Tp* __p, _Arg __x) const
      { return (__p->*_M_f)(__x); }

    private:
      _Ret (_Tp::*_M_f)(_Arg);
    };

  /// One of the @link memory_adaptors adaptors for member
  /// pointers@endlink.
  template<typename _Ret, typename _Tp, typename _Arg>
    class const_mem_fun1_t : public binary_function<const _Tp*, _Arg, _Ret>
    {
    public:
      explicit
      const_mem_fun1_t(_Ret (_Tp::*__pf)(_Arg) const)
      : _M_f(__pf) { }

      _Ret
      operator()(const _Tp* __p, _Arg __x) const
      { return (__p->*_M_f)(__x); }

    private:
      _Ret (_Tp::*_M_f)(_Arg) const;
    };

  /// One of the @link memory_adaptors adaptors for member
  /// pointers@endlink.
  template<typename _Ret, typename _Tp, typename _Arg>
    class mem_fun1_ref_t : public binary_function<_Tp, _Arg, _Ret>
    {
    public:
      explicit
      mem_fun1_ref_t(_Ret (_Tp::*__pf)(_Arg))
      : _M_f(__pf) { }

      _Ret
      operator()(_Tp& __r, _Arg __x) const
      { return (__r.*_M_f)(__x); }

    private:
      _Ret (_Tp::*_M_f)(_Arg);
    };

  /// One of the @link memory_adaptors adaptors for member
  /// pointers@endlink.
  template<typename _Ret, typename _Tp, typename _Arg>
    class const_mem_fun1_ref_t : public binary_function<_Tp, _Arg, _Ret>
    {
    public:
      explicit
      const_mem_fun1_ref_t(_Ret (_Tp::*__pf)(_Arg) const)
      : _M_f(__pf) { }

      _Ret
      operator()(const _Tp& __r, _Arg __x) const
      { return (__r.*_M_f)(__x); }

    private:
      _Ret (_Tp::*_M_f)(_Arg) const;
    };

  // Mem_fun adaptor helper functions.  There are only two:
  // mem_fun and mem_fun_ref.
  template<typename _Ret, typename _Tp>
    inline mem_fun_t<_Ret, _Tp>
    mem_fun(_Ret (_Tp::*__f)())
    { return mem_fun_t<_Ret, _Tp>(__f); }

  template<typename _Ret, typename _Tp>
    inline const_mem_fun_t<_Ret, _Tp>
    mem_fun(_Ret (_Tp::*__f)() const)
    { return const_mem_fun_t<_Ret, _Tp>(__f); }

  template<typename _Ret, typename _Tp>
    inline mem_fun_ref_t<_Ret, _Tp>
    mem_fun_ref(_Ret (_Tp::*__f)())
    { return mem_fun_ref_t<_Ret, _Tp>(__f); }

  template<typename _Ret, typename _Tp>
    inline const_mem_fun_ref_t<_Ret, _Tp>
    mem_fun_ref(_Ret (_Tp::*__f)() const)
    { return const_mem_fun_ref_t<_Ret, _Tp>(__f); }

  template<typename _Ret, typename _Tp, typename _Arg>
    inline mem_fun1_t<_Ret, _Tp, _Arg>
    mem_fun(_Ret (_Tp::*__f)(_Arg))
    { return mem_fun1_t<_Ret, _Tp, _Arg>(__f); }

  template<typename _Ret, typename _Tp, typename _Arg>
    inline const_mem_fun1_t<_Ret, _Tp, _Arg>
    mem_fun(_Ret (_Tp::*__f)(_Arg) const)
    { return const_mem_fun1_t<_Ret, _Tp, _Arg>(__f); }

  template<typename _Ret, typename _Tp, typename _Arg>
    inline mem_fun1_ref_t<_Ret, _Tp, _Arg>
    mem_fun_ref(_Ret (_Tp::*__f)(_Arg))
    { return mem_fun1_ref_t<_Ret, _Tp, _Arg>(__f); }

  template<typename _Ret, typename _Tp, typename _Arg>
    inline const_mem_fun1_ref_t<_Ret, _Tp, _Arg>
    mem_fun_ref(_Ret (_Tp::*__f)(_Arg) const)
    { return const_mem_fun1_ref_t<_Ret, _Tp, _Arg>(__f); }

  /** @}  */

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#if (__cplusplus < 201103L) || _GLIBCXX_USE_DEPRECATED
# include <backward/binders.h>
#endif

#endif /* _STL_FUNCTION_H */
