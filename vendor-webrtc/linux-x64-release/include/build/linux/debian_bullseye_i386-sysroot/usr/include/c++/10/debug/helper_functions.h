// Debugging support implementation -*- C++ -*-

// Copyright (C) 2003-2020 Free Software Foundation, Inc.
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

/** @file debug/helper_functions.h
 *  This file is a GNU debug extension to the Standard C++ Library.
 */

#ifndef _GLIBCXX_DEBUG_HELPER_FUNCTIONS_H
#define _GLIBCXX_DEBUG_HELPER_FUNCTIONS_H 1

#include <bits/move.h>				// for __addressof
#include <bits/stl_iterator_base_types.h>	// for iterator_traits,
						// categories and _Iter_base
#include <bits/cpp_type_traits.h>		// for __is_integer

#include <bits/stl_pair.h>			// for pair

namespace __gnu_debug
{
  template<typename _Iterator, typename _Sequence, typename _Category>
    class _Safe_iterator;

#if __cplusplus >= 201103L
  template<typename _Iterator, typename _Sequence>
    class _Safe_local_iterator;
#endif

  /** The precision to which we can calculate the distance between
   *  two iterators.
   */
  enum _Distance_precision
    {
      __dp_none,		// Not even an iterator type
      __dp_equality,		//< Can compare iterator equality, only
      __dp_sign,		//< Can determine equality and ordering
      __dp_sign_max_size,	//< __dp_sign and gives max range size
      __dp_exact		//< Can determine distance precisely
    };

  template<typename _Iterator,
	   typename = typename std::__is_integer<_Iterator>::__type>
    struct _Distance_traits
    {
    private:
      typedef
      typename std::iterator_traits<_Iterator>::difference_type _ItDiffType;

      template<typename _DiffType,
	       typename = typename std::__is_void<_DiffType>::__type>
	struct _DiffTraits
	{ typedef _DiffType __type; };

      template<typename _DiffType>
	struct _DiffTraits<_DiffType, std::__true_type>
	{ typedef std::ptrdiff_t __type; };

      typedef typename _DiffTraits<_ItDiffType>::__type _DiffType;

    public:
      typedef std::pair<_DiffType, _Distance_precision> __type;
    };

  template<typename _Integral>
    struct _Distance_traits<_Integral, std::__true_type>
    { typedef std::pair<std::ptrdiff_t, _Distance_precision> __type; };

  /** Determine the distance between two iterators with some known
   *	precision.
  */
  template<typename _Iterator>
    _GLIBCXX_CONSTEXPR
    inline typename _Distance_traits<_Iterator>::__type
    __get_distance(_Iterator __lhs, _Iterator __rhs,
		   std::random_access_iterator_tag)
    { return std::make_pair(__rhs - __lhs, __dp_exact); }

  template<typename _Iterator>
    _GLIBCXX14_CONSTEXPR
    inline typename _Distance_traits<_Iterator>::__type
    __get_distance(_Iterator __lhs, _Iterator __rhs,
		   std::input_iterator_tag)
    {
      if (__lhs == __rhs)
	return std::make_pair(0, __dp_exact);

      return std::make_pair(1, __dp_equality);
    }

  template<typename _Iterator>
    _GLIBCXX_CONSTEXPR
    inline typename _Distance_traits<_Iterator>::__type
    __get_distance(_Iterator __lhs, _Iterator __rhs)
    { return __get_distance(__lhs, __rhs, std::__iterator_category(__lhs)); }

  // An arbitrary iterator pointer is not singular.
  inline bool
  __check_singular_aux(const void*) { return false; }

  // We may have an iterator that derives from _Safe_iterator_base but isn't
  // a _Safe_iterator.
  template<typename _Iterator>
    inline bool
    __check_singular(_Iterator const& __x)
    { return __check_singular_aux(std::__addressof(__x)); }

  /** Non-NULL pointers are nonsingular. */
  template<typename _Tp>
    inline bool
    __check_singular(_Tp* const& __ptr)
    { return __ptr == 0; }

  /** We say that integral types for a valid range, and defer to other
   *  routines to realize what to do with integral types instead of
   *  iterators.
  */
  template<typename _Integral>
    _GLIBCXX_CONSTEXPR
    inline bool
    __valid_range_aux(_Integral, _Integral, std::__true_type)
    { return true; }

  template<typename _Integral>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __valid_range_aux(_Integral, _Integral,
		      typename _Distance_traits<_Integral>::__type& __dist,
		      std::__true_type)
    {
      __dist = std::make_pair(0, __dp_none);
      return true;
    }

  template<typename _InputIterator>
    _GLIBCXX_CONSTEXPR
    inline bool
    __valid_range_aux(_InputIterator __first, _InputIterator __last,
		      std::input_iterator_tag)
    {
      return __first == __last
	|| (!__check_singular(__first) && !__check_singular(__last));
    }

  template<typename _InputIterator>
    _GLIBCXX_CONSTEXPR
    inline bool
    __valid_range_aux(_InputIterator __first, _InputIterator __last,
		      std::random_access_iterator_tag)
    {
      return
	__valid_range_aux(__first, __last, std::input_iterator_tag())
	&& __first <= __last;
    }

  /** We have iterators, so figure out what kind of iterators they are
   *  to see if we can check the range ahead of time.
  */
  template<typename _InputIterator>
    _GLIBCXX_CONSTEXPR
    inline bool
    __valid_range_aux(_InputIterator __first, _InputIterator __last,
		      std::__false_type)
    {
      return __valid_range_aux(__first, __last,
			       std::__iterator_category(__first));
    }

  template<typename _InputIterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __valid_range_aux(_InputIterator __first, _InputIterator __last,
		      typename _Distance_traits<_InputIterator>::__type& __dist,
		      std::__false_type)
    {
      if (!__valid_range_aux(__first, __last, std::input_iterator_tag()))
	return false;

      __dist = __get_distance(__first, __last);
      switch (__dist.second)
	{
	case __dp_none:
	  break;
	case __dp_equality:
	  if (__dist.first == 0)
	    return true;
	  break;
	case __dp_sign:
	case __dp_sign_max_size:
	case __dp_exact:
	  return __dist.first >= 0;
	}

      // Can't tell so assume it is fine.
      return true;
    }

  /** Don't know what these iterators are, or if they are even
   *  iterators (we may get an integral type for InputIterator), so
   *  see if they are integral and pass them on to the next phase
   *  otherwise.
  */
  template<typename _InputIterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __valid_range(_InputIterator __first, _InputIterator __last,
		  typename _Distance_traits<_InputIterator>::__type& __dist)
    {
#ifdef __cpp_lib_is_constant_evaluated
      if (std::is_constant_evaluated())
	// Detected by the compiler directly.
	return true;
#endif
      typedef typename std::__is_integer<_InputIterator>::__type _Integral;
      return __valid_range_aux(__first, __last, __dist, _Integral());
    }

  template<typename _Iterator, typename _Sequence, typename _Category>
    bool
    __valid_range(const _Safe_iterator<_Iterator, _Sequence, _Category>&,
		  const _Safe_iterator<_Iterator, _Sequence, _Category>&,
		  typename _Distance_traits<_Iterator>::__type&);

#if __cplusplus >= 201103L
  template<typename _Iterator,typename _Sequence>
    bool
    __valid_range(const _Safe_local_iterator<_Iterator, _Sequence>&,
		  const _Safe_local_iterator<_Iterator, _Sequence>&,
		  typename _Distance_traits<_Iterator>::__type&);
#endif

  template<typename _InputIterator>
    _GLIBCXX14_CONSTEXPR
    inline bool
    __valid_range(_InputIterator __first, _InputIterator __last)
    {
#ifdef _GLIBCXX_HAVE_BUILTIN_IS_CONSTANT_EVALUATED
      if (__builtin_is_constant_evaluated())
	// Detected by the compiler directly.
	return true;
#endif
      typedef typename std::__is_integer<_InputIterator>::__type _Integral;
      return __valid_range_aux(__first, __last, _Integral());
    }

  template<typename _Iterator, typename _Sequence, typename _Category>
    bool
    __valid_range(const _Safe_iterator<_Iterator, _Sequence, _Category>&,
		  const _Safe_iterator<_Iterator, _Sequence, _Category>&);

#if __cplusplus >= 201103L
  template<typename _Iterator, typename _Sequence>
    bool
    __valid_range(const _Safe_local_iterator<_Iterator, _Sequence>&,
		  const _Safe_local_iterator<_Iterator, _Sequence>&);
#endif

  // Fallback method, always ok.
  template<typename _InputIterator, typename _Size>
    _GLIBCXX_CONSTEXPR
    inline bool
    __can_advance(_InputIterator, _Size)
    { return true; }

  template<typename _Iterator, typename _Sequence, typename _Category,
	   typename _Size>
    bool
    __can_advance(const _Safe_iterator<_Iterator, _Sequence, _Category>&,
		  _Size);

  /** Helper function to extract base iterator of random access safe iterator
   *  in order to reduce performance impact of debug mode.  Limited to random
   *  access iterator because it is the only category for which it is possible
   *  to check for correct iterators order in the __valid_range function
   *  thanks to the < operator.
   */
  template<typename _Iterator>
    _GLIBCXX_CONSTEXPR
    inline _Iterator
    __base(_Iterator __it)
    { return __it; }

#if __cplusplus < 201103L
  template<typename _Iterator>
    struct _Unsafe_type
    { typedef _Iterator _Type; };
#endif

  /* Remove debug mode safe iterator layer, if any. */
  template<typename _Iterator>
    inline _Iterator
    __unsafe(_Iterator __it)
    { return __it; }
}

#endif
