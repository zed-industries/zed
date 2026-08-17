// Heap implementation -*- C++ -*-

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
 * Copyright (c) 1997
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

/** @file bits/stl_heap.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{queue}
 */

#ifndef _STL_HEAP_H
#define _STL_HEAP_H 1

#include <debug/debug.h>
#include <bits/move.h>
#include <bits/predefined_ops.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /**
   * @defgroup heap_algorithms Heap
   * @ingroup sorting_algorithms
   */

  template<typename _RandomAccessIterator, typename _Distance,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _Distance
    __is_heap_until(_RandomAccessIterator __first, _Distance __n,
		    _Compare& __comp)
    {
      _Distance __parent = 0;
      for (_Distance __child = 1; __child < __n; ++__child)
	{
	  if (__comp(__first + __parent, __first + __child))
	    return __child;
	  if ((__child & 1) == 0)
	    ++__parent;
	}
      return __n;
    }

  // __is_heap, a predicate testing whether or not a range is a heap.
  // This function is an extension, not part of the C++ standard.
  template<typename _RandomAccessIterator, typename _Distance>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __is_heap(_RandomAccessIterator __first, _Distance __n)
    {
      __gnu_cxx::__ops::_Iter_less_iter __comp;
      return std::__is_heap_until(__first, __n, __comp) == __n;
    }

  template<typename _RandomAccessIterator, typename _Compare,
	   typename _Distance>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __is_heap(_RandomAccessIterator __first, _Compare __comp, _Distance __n)
    {
      typedef __decltype(__comp) _Cmp;
      __gnu_cxx::__ops::_Iter_comp_iter<_Cmp> __cmp(_GLIBCXX_MOVE(__comp));
      return std::__is_heap_until(__first, __n, __cmp) == __n;
    }

  template<typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __is_heap(_RandomAccessIterator __first, _RandomAccessIterator __last)
    { return std::__is_heap(__first, std::distance(__first, __last)); }

  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline bool
    __is_heap(_RandomAccessIterator __first, _RandomAccessIterator __last,
	      _Compare __comp)
    {
      return std::__is_heap(__first, _GLIBCXX_MOVE(__comp),
			    std::distance(__first, __last));
    }

  // Heap-manipulation functions: push_heap, pop_heap, make_heap, sort_heap,
  // + is_heap and is_heap_until in C++0x.

  template<typename _RandomAccessIterator, typename _Distance, typename _Tp,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    __push_heap(_RandomAccessIterator __first,
		_Distance __holeIndex, _Distance __topIndex, _Tp __value,
		_Compare& __comp)
    {
      _Distance __parent = (__holeIndex - 1) / 2;
      while (__holeIndex > __topIndex && __comp(__first + __parent, __value))
	{
	  *(__first + __holeIndex) = _GLIBCXX_MOVE(*(__first + __parent));
	  __holeIndex = __parent;
	  __parent = (__holeIndex - 1) / 2;
	}
      *(__first + __holeIndex) = _GLIBCXX_MOVE(__value);
    }

  /**
   *  @brief  Push an element onto a heap.
   *  @param  __first  Start of heap.
   *  @param  __last   End of heap + element.
   *  @ingroup heap_algorithms
   *
   *  This operation pushes the element at last-1 onto the valid heap
   *  over the range [__first,__last-1).  After completion,
   *  [__first,__last) is a valid heap.
  */
  template<typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    inline void
    push_heap(_RandomAccessIterator __first, _RandomAccessIterator __last)
    {
      typedef typename iterator_traits<_RandomAccessIterator>::value_type
	  _ValueType;
      typedef typename iterator_traits<_RandomAccessIterator>::difference_type
	  _DistanceType;

      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<_ValueType>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);
      __glibcxx_requires_heap(__first, __last - 1);

      __gnu_cxx::__ops::_Iter_less_val __comp;
      _ValueType __value = _GLIBCXX_MOVE(*(__last - 1));
      std::__push_heap(__first, _DistanceType((__last - __first) - 1),
		       _DistanceType(0), _GLIBCXX_MOVE(__value), __comp);
    }

  /**
   *  @brief  Push an element onto a heap using comparison functor.
   *  @param  __first  Start of heap.
   *  @param  __last   End of heap + element.
   *  @param  __comp   Comparison functor.
   *  @ingroup heap_algorithms
   *
   *  This operation pushes the element at __last-1 onto the valid
   *  heap over the range [__first,__last-1).  After completion,
   *  [__first,__last) is a valid heap.  Compare operations are
   *  performed using comp.
  */
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline void
    push_heap(_RandomAccessIterator __first, _RandomAccessIterator __last,
	      _Compare __comp)
    {
      typedef typename iterator_traits<_RandomAccessIterator>::value_type
	  _ValueType;
      typedef typename iterator_traits<_RandomAccessIterator>::difference_type
	  _DistanceType;

      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);
      __glibcxx_requires_heap_pred(__first, __last - 1, __comp);

      __decltype(__gnu_cxx::__ops::__iter_comp_val(_GLIBCXX_MOVE(__comp)))
	__cmp(_GLIBCXX_MOVE(__comp));
      _ValueType __value = _GLIBCXX_MOVE(*(__last - 1));
      std::__push_heap(__first, _DistanceType((__last - __first) - 1),
		       _DistanceType(0), _GLIBCXX_MOVE(__value), __cmp);
    }

  template<typename _RandomAccessIterator, typename _Distance,
	   typename _Tp, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    __adjust_heap(_RandomAccessIterator __first, _Distance __holeIndex,
		  _Distance __len, _Tp __value, _Compare __comp)
    {
      const _Distance __topIndex = __holeIndex;
      _Distance __secondChild = __holeIndex;
      while (__secondChild < (__len - 1) / 2)
	{
	  __secondChild = 2 * (__secondChild + 1);
	  if (__comp(__first + __secondChild,
		     __first + (__secondChild - 1)))
	    __secondChild--;
	  *(__first + __holeIndex) = _GLIBCXX_MOVE(*(__first + __secondChild));
	  __holeIndex = __secondChild;
	}
      if ((__len & 1) == 0 && __secondChild == (__len - 2) / 2)
	{
	  __secondChild = 2 * (__secondChild + 1);
	  *(__first + __holeIndex) = _GLIBCXX_MOVE(*(__first
						     + (__secondChild - 1)));
	  __holeIndex = __secondChild - 1;
	}
      __decltype(__gnu_cxx::__ops::__iter_comp_val(_GLIBCXX_MOVE(__comp)))
	__cmp(_GLIBCXX_MOVE(__comp));
      std::__push_heap(__first, __holeIndex, __topIndex,
		       _GLIBCXX_MOVE(__value), __cmp);
    }

  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline void
    __pop_heap(_RandomAccessIterator __first, _RandomAccessIterator __last,
	       _RandomAccessIterator __result, _Compare& __comp)
    {
      typedef typename iterator_traits<_RandomAccessIterator>::value_type
	_ValueType;
      typedef typename iterator_traits<_RandomAccessIterator>::difference_type
	_DistanceType;

      _ValueType __value = _GLIBCXX_MOVE(*__result);
      *__result = _GLIBCXX_MOVE(*__first);
      std::__adjust_heap(__first, _DistanceType(0),
			 _DistanceType(__last - __first),
			 _GLIBCXX_MOVE(__value), __comp);
    }

  /**
   *  @brief  Pop an element off a heap.
   *  @param  __first  Start of heap.
   *  @param  __last   End of heap.
   *  @pre    [__first, __last) is a valid, non-empty range.
   *  @ingroup heap_algorithms
   *
   *  This operation pops the top of the heap.  The elements __first
   *  and __last-1 are swapped and [__first,__last-1) is made into a
   *  heap.
  */
  template<typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    inline void
    pop_heap(_RandomAccessIterator __first, _RandomAccessIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	typename iterator_traits<_RandomAccessIterator>::value_type>)
      __glibcxx_requires_non_empty_range(__first, __last);
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);
      __glibcxx_requires_heap(__first, __last);

      if (__last - __first > 1)
	{
	  --__last;
	  __gnu_cxx::__ops::_Iter_less_iter __comp;
	  std::__pop_heap(__first, __last, __last, __comp);
	}
    }

  /**
   *  @brief  Pop an element off a heap using comparison functor.
   *  @param  __first  Start of heap.
   *  @param  __last   End of heap.
   *  @param  __comp   Comparison functor to use.
   *  @ingroup heap_algorithms
   *
   *  This operation pops the top of the heap.  The elements __first
   *  and __last-1 are swapped and [__first,__last-1) is made into a
   *  heap.  Comparisons are made using comp.
  */
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline void
    pop_heap(_RandomAccessIterator __first,
	     _RandomAccessIterator __last, _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);
      __glibcxx_requires_non_empty_range(__first, __last);
      __glibcxx_requires_heap_pred(__first, __last, __comp);

      if (__last - __first > 1)
	{
	  typedef __decltype(__comp) _Cmp;
	  __gnu_cxx::__ops::_Iter_comp_iter<_Cmp> __cmp(_GLIBCXX_MOVE(__comp));
	  --__last;
	  std::__pop_heap(__first, __last, __last, __cmp);
	}
    }

  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    __make_heap(_RandomAccessIterator __first, _RandomAccessIterator __last,
		_Compare& __comp)
    {
      typedef typename iterator_traits<_RandomAccessIterator>::value_type
	  _ValueType;
      typedef typename iterator_traits<_RandomAccessIterator>::difference_type
	  _DistanceType;

      if (__last - __first < 2)
	return;

      const _DistanceType __len = __last - __first;
      _DistanceType __parent = (__len - 2) / 2;
      while (true)
	{
	  _ValueType __value = _GLIBCXX_MOVE(*(__first + __parent));
	  std::__adjust_heap(__first, __parent, __len, _GLIBCXX_MOVE(__value),
			     __comp);
	  if (__parent == 0)
	    return;
	  __parent--;
	}
    }
  
  /**
   *  @brief  Construct a heap over a range.
   *  @param  __first  Start of heap.
   *  @param  __last   End of heap.
   *  @ingroup heap_algorithms
   *
   *  This operation makes the elements in [__first,__last) into a heap.
  */
  template<typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    inline void
    make_heap(_RandomAccessIterator __first, _RandomAccessIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_RandomAccessIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      __gnu_cxx::__ops::_Iter_less_iter __comp;
      std::__make_heap(__first, __last, __comp);
    }

  /**
   *  @brief  Construct a heap over a range using comparison functor.
   *  @param  __first  Start of heap.
   *  @param  __last   End of heap.
   *  @param  __comp   Comparison functor to use.
   *  @ingroup heap_algorithms
   *
   *  This operation makes the elements in [__first,__last) into a heap.
   *  Comparisons are made using __comp.
  */
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline void
    make_heap(_RandomAccessIterator __first, _RandomAccessIterator __last,
	      _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      typedef __decltype(__comp) _Cmp;
      __gnu_cxx::__ops::_Iter_comp_iter<_Cmp> __cmp(_GLIBCXX_MOVE(__comp));
      std::__make_heap(__first, __last, __cmp);
    }

  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    __sort_heap(_RandomAccessIterator __first, _RandomAccessIterator __last,
		_Compare& __comp)
    {
      while (__last - __first > 1)
	{
	  --__last;
	  std::__pop_heap(__first, __last, __last, __comp);
	}
    }

  /**
   *  @brief  Sort a heap.
   *  @param  __first  Start of heap.
   *  @param  __last   End of heap.
   *  @ingroup heap_algorithms
   *
   *  This operation sorts the valid heap in the range [__first,__last).
  */
  template<typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    inline void
    sort_heap(_RandomAccessIterator __first, _RandomAccessIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_RandomAccessIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);
      __glibcxx_requires_heap(__first, __last);

      __gnu_cxx::__ops::_Iter_less_iter __comp;
      std::__sort_heap(__first, __last, __comp);
    }

  /**
   *  @brief  Sort a heap using comparison functor.
   *  @param  __first  Start of heap.
   *  @param  __last   End of heap.
   *  @param  __comp   Comparison functor to use.
   *  @ingroup heap_algorithms
   *
   *  This operation sorts the valid heap in the range [__first,__last).
   *  Comparisons are made using __comp.
  */
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline void
    sort_heap(_RandomAccessIterator __first, _RandomAccessIterator __last,
	      _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);
      __glibcxx_requires_heap_pred(__first, __last, __comp);

      typedef __decltype(__comp) _Cmp;
      __gnu_cxx::__ops::_Iter_comp_iter<_Cmp> __cmp(_GLIBCXX_MOVE(__comp));
      std::__sort_heap(__first, __last, __cmp);
    }

#if __cplusplus >= 201103L
  /**
   *  @brief  Search the end of a heap.
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @return  An iterator pointing to the first element not in the heap.
   *  @ingroup heap_algorithms
   *
   *  This operation returns the last iterator i in [__first, __last) for which
   *  the range [__first, i) is a heap.
  */
  template<typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    inline _RandomAccessIterator
    is_heap_until(_RandomAccessIterator __first, _RandomAccessIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_RandomAccessIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      __gnu_cxx::__ops::_Iter_less_iter __comp;
      return __first + 
	std::__is_heap_until(__first, std::distance(__first, __last), __comp);
    }

  /**
   *  @brief  Search the end of a heap using comparison functor.
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @param  __comp   Comparison functor to use.
   *  @return  An iterator pointing to the first element not in the heap.
   *  @ingroup heap_algorithms
   *
   *  This operation returns the last iterator i in [__first, __last) for which
   *  the range [__first, i) is a heap.  Comparisons are made using __comp.
  */
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _RandomAccessIterator
    is_heap_until(_RandomAccessIterator __first, _RandomAccessIterator __last,
		  _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      typedef __decltype(__comp) _Cmp;
      __gnu_cxx::__ops::_Iter_comp_iter<_Cmp> __cmp(_GLIBCXX_MOVE(__comp));
      return __first
	+ std::__is_heap_until(__first, std::distance(__first, __last), __cmp);
    }

  /**
   *  @brief  Determines whether a range is a heap.
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @return  True if range is a heap, false otherwise.
   *  @ingroup heap_algorithms
  */
  template<typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    is_heap(_RandomAccessIterator __first, _RandomAccessIterator __last)
    { return std::is_heap_until(__first, __last) == __last; }

  /**
   *  @brief  Determines whether a range is a heap using comparison functor.
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @param  __comp   Comparison functor to use.
   *  @return  True if range is a heap, false otherwise.
   *  @ingroup heap_algorithms
  */
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline bool
    is_heap(_RandomAccessIterator __first, _RandomAccessIterator __last,
	    _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      const auto __dist = std::distance(__first, __last);
      typedef __decltype(__comp) _Cmp;
      __gnu_cxx::__ops::_Iter_comp_iter<_Cmp> __cmp(_GLIBCXX_MOVE(__comp));
      return std::__is_heap_until(__first, __dist, __cmp) == __dist;
    }
#endif

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace

#endif /* _STL_HEAP_H */
