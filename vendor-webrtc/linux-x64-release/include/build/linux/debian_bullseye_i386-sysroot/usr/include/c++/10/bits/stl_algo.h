// Algorithm implementation -*- C++ -*-

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
 * Copyright (c) 1996
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

/** @file bits/stl_algo.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{algorithm}
 */

#ifndef _STL_ALGO_H
#define _STL_ALGO_H 1

#include <cstdlib>	     // for rand
#include <bits/algorithmfwd.h>
#include <bits/stl_heap.h>
#include <bits/stl_tempbuf.h>  // for _Temporary_buffer
#include <bits/predefined_ops.h>

#if __cplusplus >= 201103L
#include <bits/uniform_int_dist.h>
#endif

// See concept_check.h for the __glibcxx_*_requires macros.

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /// Swaps the median value of *__a, *__b and *__c under __comp to *__result
  template<typename _Iterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    __move_median_to_first(_Iterator __result,_Iterator __a, _Iterator __b,
			   _Iterator __c, _Compare __comp)
    {
      if (__comp(__a, __b))
	{
	  if (__comp(__b, __c))
	    std::iter_swap(__result, __b);
	  else if (__comp(__a, __c))
	    std::iter_swap(__result, __c);
	  else
	    std::iter_swap(__result, __a);
	}
      else if (__comp(__a, __c))
	std::iter_swap(__result, __a);
      else if (__comp(__b, __c))
	std::iter_swap(__result, __c);
      else
	std::iter_swap(__result, __b);
    }

  /// Provided for stable_partition to use.
  template<typename _InputIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline _InputIterator
    __find_if_not(_InputIterator __first, _InputIterator __last,
		  _Predicate __pred)
    {
      return std::__find_if(__first, __last,
			    __gnu_cxx::__ops::__negate(__pred),
			    std::__iterator_category(__first));
    }

  /// Like find_if_not(), but uses and updates a count of the
  /// remaining range length instead of comparing against an end
  /// iterator.
  template<typename _InputIterator, typename _Predicate, typename _Distance>
    _GLIBCXX20_CONSTEXPR
    _InputIterator
    __find_if_not_n(_InputIterator __first, _Distance& __len, _Predicate __pred)
    {
      for (; __len; --__len,  (void) ++__first)
	if (!__pred(__first))
	  break;
      return __first;
    }

  // set_difference
  // set_intersection
  // set_symmetric_difference
  // set_union
  // for_each
  // find
  // find_if
  // find_first_of
  // adjacent_find
  // count
  // count_if
  // search

  template<typename _ForwardIterator1, typename _ForwardIterator2,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator1
    __search(_ForwardIterator1 __first1, _ForwardIterator1 __last1,
	     _ForwardIterator2 __first2, _ForwardIterator2 __last2,
	     _BinaryPredicate  __predicate)
    {
      // Test for empty ranges
      if (__first1 == __last1 || __first2 == __last2)
	return __first1;

      // Test for a pattern of length 1.
      _ForwardIterator2 __p1(__first2);
      if (++__p1 == __last2)
	return std::__find_if(__first1, __last1,
		__gnu_cxx::__ops::__iter_comp_iter(__predicate, __first2));

      // General case.
      _ForwardIterator1 __current = __first1;

      for (;;)
	{
	  __first1 =
	    std::__find_if(__first1, __last1,
		__gnu_cxx::__ops::__iter_comp_iter(__predicate, __first2));

	  if (__first1 == __last1)
	    return __last1;

	  _ForwardIterator2 __p = __p1;
	  __current = __first1;
	  if (++__current == __last1)
	    return __last1;

	  while (__predicate(__current, __p))
	    {
	      if (++__p == __last2)
		return __first1;
	      if (++__current == __last1)
		return __last1;
	    }
	  ++__first1;
	}
      return __first1;
    }

  // search_n

  /**
   *  This is an helper function for search_n overloaded for forward iterators.
  */
  template<typename _ForwardIterator, typename _Integer,
	   typename _UnaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator
    __search_n_aux(_ForwardIterator __first, _ForwardIterator __last,
		   _Integer __count, _UnaryPredicate __unary_pred,
		   std::forward_iterator_tag)
    {
      __first = std::__find_if(__first, __last, __unary_pred);
      while (__first != __last)
	{
	  typename iterator_traits<_ForwardIterator>::difference_type
	    __n = __count;
	  _ForwardIterator __i = __first;
	  ++__i;
	  while (__i != __last && __n != 1 && __unary_pred(__i))
	    {
	      ++__i;
	      --__n;
	    }
	  if (__n == 1)
	    return __first;
	  if (__i == __last)
	    return __last;
	  __first = std::__find_if(++__i, __last, __unary_pred);
	}
      return __last;
    }

  /**
   *  This is an helper function for search_n overloaded for random access
   *  iterators.
  */
  template<typename _RandomAccessIter, typename _Integer,
	   typename _UnaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _RandomAccessIter
    __search_n_aux(_RandomAccessIter __first, _RandomAccessIter __last,
		   _Integer __count, _UnaryPredicate __unary_pred,
		   std::random_access_iterator_tag)
    {
      typedef typename std::iterator_traits<_RandomAccessIter>::difference_type
	_DistanceType;

      _DistanceType __tailSize = __last - __first;
      _DistanceType __remainder = __count;

      while (__remainder <= __tailSize) // the main loop...
	{
	  __first += __remainder;
	  __tailSize -= __remainder;
	  // __first here is always pointing to one past the last element of
	  // next possible match.
	  _RandomAccessIter __backTrack = __first; 
	  while (__unary_pred(--__backTrack))
	    {
	      if (--__remainder == 0)
		return (__first - __count); // Success
	    }
	  __remainder = __count + 1 - (__first - __backTrack);
	}
      return __last; // Failure
    }

  template<typename _ForwardIterator, typename _Integer,
	   typename _UnaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator
    __search_n(_ForwardIterator __first, _ForwardIterator __last,
	       _Integer __count,
	       _UnaryPredicate __unary_pred)
    {
      if (__count <= 0)
	return __first;

      if (__count == 1)
	return std::__find_if(__first, __last, __unary_pred);

      return std::__search_n_aux(__first, __last, __count, __unary_pred,
				 std::__iterator_category(__first));
    }

  // find_end for forward iterators.
  template<typename _ForwardIterator1, typename _ForwardIterator2,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator1
    __find_end(_ForwardIterator1 __first1, _ForwardIterator1 __last1,
	       _ForwardIterator2 __first2, _ForwardIterator2 __last2,
	       forward_iterator_tag, forward_iterator_tag,
	       _BinaryPredicate __comp)
    {
      if (__first2 == __last2)
	return __last1;

      _ForwardIterator1 __result = __last1;
      while (1)
	{
	  _ForwardIterator1 __new_result
	    = std::__search(__first1, __last1, __first2, __last2, __comp);
	  if (__new_result == __last1)
	    return __result;
	  else
	    {
	      __result = __new_result;
	      __first1 = __new_result;
	      ++__first1;
	    }
	}
    }

  // find_end for bidirectional iterators (much faster).
  template<typename _BidirectionalIterator1, typename _BidirectionalIterator2,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _BidirectionalIterator1
    __find_end(_BidirectionalIterator1 __first1,
	       _BidirectionalIterator1 __last1,
	       _BidirectionalIterator2 __first2,
	       _BidirectionalIterator2 __last2,
	       bidirectional_iterator_tag, bidirectional_iterator_tag,
	       _BinaryPredicate __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_BidirectionalIteratorConcept<
				  _BidirectionalIterator1>)
      __glibcxx_function_requires(_BidirectionalIteratorConcept<
				  _BidirectionalIterator2>)

      typedef reverse_iterator<_BidirectionalIterator1> _RevIterator1;
      typedef reverse_iterator<_BidirectionalIterator2> _RevIterator2;

      _RevIterator1 __rlast1(__first1);
      _RevIterator2 __rlast2(__first2);
      _RevIterator1 __rresult = std::__search(_RevIterator1(__last1), __rlast1,
					      _RevIterator2(__last2), __rlast2,
					      __comp);

      if (__rresult == __rlast1)
	return __last1;
      else
	{
	  _BidirectionalIterator1 __result = __rresult.base();
	  std::advance(__result, -std::distance(__first2, __last2));
	  return __result;
	}
    }

  /**
   *  @brief  Find last matching subsequence in a sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first1  Start of range to search.
   *  @param  __last1   End of range to search.
   *  @param  __first2  Start of sequence to match.
   *  @param  __last2   End of sequence to match.
   *  @return   The last iterator @c i in the range
   *  @p [__first1,__last1-(__last2-__first2)) such that @c *(i+N) ==
   *  @p *(__first2+N) for each @c N in the range @p
   *  [0,__last2-__first2), or @p __last1 if no such iterator exists.
   *
   *  Searches the range @p [__first1,__last1) for a sub-sequence that
   *  compares equal value-by-value with the sequence given by @p
   *  [__first2,__last2) and returns an iterator to the __first
   *  element of the sub-sequence, or @p __last1 if the sub-sequence
   *  is not found.  The sub-sequence will be the last such
   *  subsequence contained in [__first1,__last1).
   *
   *  Because the sub-sequence must lie completely within the range @p
   *  [__first1,__last1) it must start at a position less than @p
   *  __last1-(__last2-__first2) where @p __last2-__first2 is the
   *  length of the sub-sequence.  This means that the returned
   *  iterator @c i will be in the range @p
   *  [__first1,__last1-(__last2-__first2))
  */
  template<typename _ForwardIterator1, typename _ForwardIterator2>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator1
    find_end(_ForwardIterator1 __first1, _ForwardIterator1 __last1,
	     _ForwardIterator2 __first2, _ForwardIterator2 __last2)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator1>)
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator2>)
      __glibcxx_function_requires(_EqualOpConcept<
	    typename iterator_traits<_ForwardIterator1>::value_type,
	    typename iterator_traits<_ForwardIterator2>::value_type>)
      __glibcxx_requires_valid_range(__first1, __last1);
      __glibcxx_requires_valid_range(__first2, __last2);

      return std::__find_end(__first1, __last1, __first2, __last2,
			     std::__iterator_category(__first1),
			     std::__iterator_category(__first2),
			     __gnu_cxx::__ops::__iter_equal_to_iter());
    }

  /**
   *  @brief  Find last matching subsequence in a sequence using a predicate.
   *  @ingroup non_mutating_algorithms
   *  @param  __first1  Start of range to search.
   *  @param  __last1   End of range to search.
   *  @param  __first2  Start of sequence to match.
   *  @param  __last2   End of sequence to match.
   *  @param  __comp    The predicate to use.
   *  @return The last iterator @c i in the range @p
   *  [__first1,__last1-(__last2-__first2)) such that @c
   *  predicate(*(i+N), @p (__first2+N)) is true for each @c N in the
   *  range @p [0,__last2-__first2), or @p __last1 if no such iterator
   *  exists.
   *
   *  Searches the range @p [__first1,__last1) for a sub-sequence that
   *  compares equal value-by-value with the sequence given by @p
   *  [__first2,__last2) using comp as a predicate and returns an
   *  iterator to the first element of the sub-sequence, or @p __last1
   *  if the sub-sequence is not found.  The sub-sequence will be the
   *  last such subsequence contained in [__first,__last1).
   *
   *  Because the sub-sequence must lie completely within the range @p
   *  [__first1,__last1) it must start at a position less than @p
   *  __last1-(__last2-__first2) where @p __last2-__first2 is the
   *  length of the sub-sequence.  This means that the returned
   *  iterator @c i will be in the range @p
   *  [__first1,__last1-(__last2-__first2))
  */
  template<typename _ForwardIterator1, typename _ForwardIterator2,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator1
    find_end(_ForwardIterator1 __first1, _ForwardIterator1 __last1,
	     _ForwardIterator2 __first2, _ForwardIterator2 __last2,
	     _BinaryPredicate __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator1>)
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator2>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_BinaryPredicate,
	    typename iterator_traits<_ForwardIterator1>::value_type,
	    typename iterator_traits<_ForwardIterator2>::value_type>)
      __glibcxx_requires_valid_range(__first1, __last1);
      __glibcxx_requires_valid_range(__first2, __last2);

      return std::__find_end(__first1, __last1, __first2, __last2,
			     std::__iterator_category(__first1),
			     std::__iterator_category(__first2),
			     __gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

#if __cplusplus >= 201103L
  /**
   *  @brief  Checks that a predicate is true for all the elements
   *          of a sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first   An input iterator.
   *  @param  __last    An input iterator.
   *  @param  __pred    A predicate.
   *  @return  True if the check is true, false otherwise.
   *
   *  Returns true if @p __pred is true for each element in the range
   *  @p [__first,__last), and false otherwise.
  */
  template<typename _InputIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    all_of(_InputIterator __first, _InputIterator __last, _Predicate __pred)
    { return __last == std::find_if_not(__first, __last, __pred); }

  /**
   *  @brief  Checks that a predicate is false for all the elements
   *          of a sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first   An input iterator.
   *  @param  __last    An input iterator.
   *  @param  __pred    A predicate.
   *  @return  True if the check is true, false otherwise.
   *
   *  Returns true if @p __pred is false for each element in the range
   *  @p [__first,__last), and false otherwise.
  */
  template<typename _InputIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    none_of(_InputIterator __first, _InputIterator __last, _Predicate __pred)
    { return __last == _GLIBCXX_STD_A::find_if(__first, __last, __pred); }

  /**
   *  @brief  Checks that a predicate is true for at least one element
   *          of a sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first   An input iterator.
   *  @param  __last    An input iterator.
   *  @param  __pred    A predicate.
   *  @return  True if the check is true, false otherwise.
   *
   *  Returns true if an element exists in the range @p
   *  [__first,__last) such that @p __pred is true, and false
   *  otherwise.
  */
  template<typename _InputIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    any_of(_InputIterator __first, _InputIterator __last, _Predicate __pred)
    { return !std::none_of(__first, __last, __pred); }

  /**
   *  @brief  Find the first element in a sequence for which a
   *          predicate is false.
   *  @ingroup non_mutating_algorithms
   *  @param  __first  An input iterator.
   *  @param  __last   An input iterator.
   *  @param  __pred   A predicate.
   *  @return   The first iterator @c i in the range @p [__first,__last)
   *  such that @p __pred(*i) is false, or @p __last if no such iterator exists.
  */
  template<typename _InputIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline _InputIterator
    find_if_not(_InputIterator __first, _InputIterator __last,
		_Predicate __pred)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_UnaryPredicateConcept<_Predicate,
	      typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      return std::__find_if_not(__first, __last,
				__gnu_cxx::__ops::__pred_iter(__pred));
    }

  /**
   *  @brief  Checks whether the sequence is partitioned.
   *  @ingroup mutating_algorithms
   *  @param  __first  An input iterator.
   *  @param  __last   An input iterator.
   *  @param  __pred   A predicate.
   *  @return  True if the range @p [__first,__last) is partioned by @p __pred,
   *  i.e. if all elements that satisfy @p __pred appear before those that
   *  do not.
  */
  template<typename _InputIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    is_partitioned(_InputIterator __first, _InputIterator __last,
		   _Predicate __pred)
    {
      __first = std::find_if_not(__first, __last, __pred);
      if (__first == __last)
	return true;
      ++__first;
      return std::none_of(__first, __last, __pred);
    }

  /**
   *  @brief  Find the partition point of a partitioned range.
   *  @ingroup mutating_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __pred    A predicate.
   *  @return  An iterator @p mid such that @p all_of(__first, mid, __pred)
   *           and @p none_of(mid, __last, __pred) are both true.
  */
  template<typename _ForwardIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator
    partition_point(_ForwardIterator __first, _ForwardIterator __last,
		    _Predicate __pred)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_UnaryPredicateConcept<_Predicate,
	      typename iterator_traits<_ForwardIterator>::value_type>)

      // A specific debug-mode test will be necessary...
      __glibcxx_requires_valid_range(__first, __last);

      typedef typename iterator_traits<_ForwardIterator>::difference_type
	_DistanceType;

      _DistanceType __len = std::distance(__first, __last);

      while (__len > 0)
	{
	  _DistanceType __half = __len >> 1;
	  _ForwardIterator __middle = __first;
	  std::advance(__middle, __half);
	  if (__pred(*__middle))
	    {
	      __first = __middle;
	      ++__first;
	      __len = __len - __half - 1;
	    }
	  else
	    __len = __half;
	}
      return __first;
    }
#endif

  template<typename _InputIterator, typename _OutputIterator,
	   typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    __remove_copy_if(_InputIterator __first, _InputIterator __last,
		     _OutputIterator __result, _Predicate __pred)
    {
      for (; __first != __last; ++__first)
	if (!__pred(__first))
	  {
	    *__result = *__first;
	    ++__result;
	  }
      return __result;
    }

  /**
   *  @brief Copy a sequence, removing elements of a given value.
   *  @ingroup mutating_algorithms
   *  @param  __first   An input iterator.
   *  @param  __last    An input iterator.
   *  @param  __result  An output iterator.
   *  @param  __value   The value to be removed.
   *  @return   An iterator designating the end of the resulting sequence.
   *
   *  Copies each element in the range @p [__first,__last) not equal
   *  to @p __value to the range beginning at @p __result.
   *  remove_copy() is stable, so the relative order of elements that
   *  are copied is unchanged.
  */
  template<typename _InputIterator, typename _OutputIterator, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    remove_copy(_InputIterator __first, _InputIterator __last,
		_OutputIterator __result, const _Tp& __value)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_function_requires(_EqualOpConcept<
	    typename iterator_traits<_InputIterator>::value_type, _Tp>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__remove_copy_if(__first, __last, __result,
	__gnu_cxx::__ops::__iter_equals_val(__value));
    }

  /**
   *  @brief Copy a sequence, removing elements for which a predicate is true.
   *  @ingroup mutating_algorithms
   *  @param  __first   An input iterator.
   *  @param  __last    An input iterator.
   *  @param  __result  An output iterator.
   *  @param  __pred    A predicate.
   *  @return   An iterator designating the end of the resulting sequence.
   *
   *  Copies each element in the range @p [__first,__last) for which
   *  @p __pred returns false to the range beginning at @p __result.
   *
   *  remove_copy_if() is stable, so the relative order of elements that are
   *  copied is unchanged.
  */
  template<typename _InputIterator, typename _OutputIterator,
	   typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    remove_copy_if(_InputIterator __first, _InputIterator __last,
		   _OutputIterator __result, _Predicate __pred)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_function_requires(_UnaryPredicateConcept<_Predicate,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__remove_copy_if(__first, __last, __result,
				   __gnu_cxx::__ops::__pred_iter(__pred));
    }

#if __cplusplus >= 201103L
  /**
   *  @brief Copy the elements of a sequence for which a predicate is true.
   *  @ingroup mutating_algorithms
   *  @param  __first   An input iterator.
   *  @param  __last    An input iterator.
   *  @param  __result  An output iterator.
   *  @param  __pred    A predicate.
   *  @return   An iterator designating the end of the resulting sequence.
   *
   *  Copies each element in the range @p [__first,__last) for which
   *  @p __pred returns true to the range beginning at @p __result.
   *
   *  copy_if() is stable, so the relative order of elements that are
   *  copied is unchanged.
  */
  template<typename _InputIterator, typename _OutputIterator,
	   typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    copy_if(_InputIterator __first, _InputIterator __last,
	    _OutputIterator __result, _Predicate __pred)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_function_requires(_UnaryPredicateConcept<_Predicate,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      for (; __first != __last; ++__first)
	if (__pred(*__first))
	  {
	    *__result = *__first;
	    ++__result;
	  }
      return __result;
    }

  template<typename _InputIterator, typename _Size, typename _OutputIterator>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    __copy_n_a(_InputIterator __first, _Size __n, _OutputIterator __result)
    {
      if (__n > 0)
	{
	  while (true)
	    {
	      *__result = *__first;
	      ++__result;
	      if (--__n > 0)
		++__first;
	      else
		break;
	    }
	}
      return __result;
    }
 
  template<typename _CharT, typename _Size>
    __enable_if_t<__is_char<_CharT>::__value, _CharT*>
    __copy_n_a(istreambuf_iterator<_CharT, char_traits<_CharT>>,
	       _Size, _CharT*);

  template<typename _InputIterator, typename _Size, typename _OutputIterator>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    __copy_n(_InputIterator __first, _Size __n,
	     _OutputIterator __result, input_iterator_tag)
    {
      return std::__niter_wrap(__result,
			       __copy_n_a(__first, __n,
					  std::__niter_base(__result)));
    }

  template<typename _RandomAccessIterator, typename _Size,
	   typename _OutputIterator>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    __copy_n(_RandomAccessIterator __first, _Size __n,
	     _OutputIterator __result, random_access_iterator_tag)
    { return std::copy(__first, __first + __n, __result); }

  /**
   *  @brief Copies the range [first,first+n) into [result,result+n).
   *  @ingroup mutating_algorithms
   *  @param  __first  An input iterator.
   *  @param  __n      The number of elements to copy.
   *  @param  __result An output iterator.
   *  @return  result+n.
   *
   *  This inline function will boil down to a call to @c memmove whenever
   *  possible.  Failing that, if random access iterators are passed, then the
   *  loop count will be known (and therefore a candidate for compiler
   *  optimizations such as unrolling).
  */
  template<typename _InputIterator, typename _Size, typename _OutputIterator>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    copy_n(_InputIterator __first, _Size __n, _OutputIterator __result)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_requires_can_increment(__first, __n);
      __glibcxx_requires_can_increment(__result, __n);

      return std::__copy_n(__first, __n, __result,
			   std::__iterator_category(__first));
    }

  /**
   *  @brief Copy the elements of a sequence to separate output sequences
   *         depending on the truth value of a predicate.
   *  @ingroup mutating_algorithms
   *  @param  __first   An input iterator.
   *  @param  __last    An input iterator.
   *  @param  __out_true   An output iterator.
   *  @param  __out_false  An output iterator.
   *  @param  __pred    A predicate.
   *  @return   A pair designating the ends of the resulting sequences.
   *
   *  Copies each element in the range @p [__first,__last) for which
   *  @p __pred returns true to the range beginning at @p out_true
   *  and each element for which @p __pred returns false to @p __out_false.
  */
  template<typename _InputIterator, typename _OutputIterator1,
	   typename _OutputIterator2, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    pair<_OutputIterator1, _OutputIterator2>
    partition_copy(_InputIterator __first, _InputIterator __last,
		   _OutputIterator1 __out_true, _OutputIterator2 __out_false,
		   _Predicate __pred)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator1,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator2,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_function_requires(_UnaryPredicateConcept<_Predicate,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      
      for (; __first != __last; ++__first)
	if (__pred(*__first))
	  {
	    *__out_true = *__first;
	    ++__out_true;
	  }
	else
	  {
	    *__out_false = *__first;
	    ++__out_false;
	  }

      return pair<_OutputIterator1, _OutputIterator2>(__out_true, __out_false);
    }
#endif // C++11

  template<typename _ForwardIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator
    __remove_if(_ForwardIterator __first, _ForwardIterator __last,
		_Predicate __pred)
    {
      __first = std::__find_if(__first, __last, __pred);
      if (__first == __last)
	return __first;
      _ForwardIterator __result = __first;
      ++__first;
      for (; __first != __last; ++__first)
	if (!__pred(__first))
	  {
	    *__result = _GLIBCXX_MOVE(*__first);
	    ++__result;
	  }
      return __result;
    }

  /**
   *  @brief Remove elements from a sequence.
   *  @ingroup mutating_algorithms
   *  @param  __first  An input iterator.
   *  @param  __last   An input iterator.
   *  @param  __value  The value to be removed.
   *  @return   An iterator designating the end of the resulting sequence.
   *
   *  All elements equal to @p __value are removed from the range
   *  @p [__first,__last).
   *
   *  remove() is stable, so the relative order of elements that are
   *  not removed is unchanged.
   *
   *  Elements between the end of the resulting sequence and @p __last
   *  are still present, but their value is unspecified.
  */
  template<typename _ForwardIterator, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    remove(_ForwardIterator __first, _ForwardIterator __last,
	   const _Tp& __value)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_ForwardIteratorConcept<
				  _ForwardIterator>)
      __glibcxx_function_requires(_EqualOpConcept<
	    typename iterator_traits<_ForwardIterator>::value_type, _Tp>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__remove_if(__first, __last,
		__gnu_cxx::__ops::__iter_equals_val(__value));
    }

  /**
   *  @brief Remove elements from a sequence using a predicate.
   *  @ingroup mutating_algorithms
   *  @param  __first  A forward iterator.
   *  @param  __last   A forward iterator.
   *  @param  __pred   A predicate.
   *  @return   An iterator designating the end of the resulting sequence.
   *
   *  All elements for which @p __pred returns true are removed from the range
   *  @p [__first,__last).
   *
   *  remove_if() is stable, so the relative order of elements that are
   *  not removed is unchanged.
   *
   *  Elements between the end of the resulting sequence and @p __last
   *  are still present, but their value is unspecified.
  */
  template<typename _ForwardIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    remove_if(_ForwardIterator __first, _ForwardIterator __last,
	      _Predicate __pred)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_ForwardIteratorConcept<
				  _ForwardIterator>)
      __glibcxx_function_requires(_UnaryPredicateConcept<_Predicate,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__remove_if(__first, __last,
			      __gnu_cxx::__ops::__pred_iter(__pred));
    }

  template<typename _ForwardIterator, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator
    __adjacent_find(_ForwardIterator __first, _ForwardIterator __last,
		    _BinaryPredicate __binary_pred)
    {
      if (__first == __last)
	return __last;
      _ForwardIterator __next = __first;
      while (++__next != __last)
	{
	  if (__binary_pred(__first, __next))
	    return __first;
	  __first = __next;
	}
      return __last;
    }

  template<typename _ForwardIterator, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator
    __unique(_ForwardIterator __first, _ForwardIterator __last,
	     _BinaryPredicate __binary_pred)
    {
      // Skip the beginning, if already unique.
      __first = std::__adjacent_find(__first, __last, __binary_pred);
      if (__first == __last)
	return __last;

      // Do the real copy work.
      _ForwardIterator __dest = __first;
      ++__first;
      while (++__first != __last)
	if (!__binary_pred(__dest, __first))
	  *++__dest = _GLIBCXX_MOVE(*__first);
      return ++__dest;
    }

  /**
   *  @brief Remove consecutive duplicate values from a sequence.
   *  @ingroup mutating_algorithms
   *  @param  __first  A forward iterator.
   *  @param  __last   A forward iterator.
   *  @return  An iterator designating the end of the resulting sequence.
   *
   *  Removes all but the first element from each group of consecutive
   *  values that compare equal.
   *  unique() is stable, so the relative order of elements that are
   *  not removed is unchanged.
   *  Elements between the end of the resulting sequence and @p __last
   *  are still present, but their value is unspecified.
  */
  template<typename _ForwardIterator>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    unique(_ForwardIterator __first, _ForwardIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_ForwardIteratorConcept<
				  _ForwardIterator>)
      __glibcxx_function_requires(_EqualityComparableConcept<
		     typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__unique(__first, __last,
			   __gnu_cxx::__ops::__iter_equal_to_iter());
    }

  /**
   *  @brief Remove consecutive values from a sequence using a predicate.
   *  @ingroup mutating_algorithms
   *  @param  __first        A forward iterator.
   *  @param  __last         A forward iterator.
   *  @param  __binary_pred  A binary predicate.
   *  @return  An iterator designating the end of the resulting sequence.
   *
   *  Removes all but the first element from each group of consecutive
   *  values for which @p __binary_pred returns true.
   *  unique() is stable, so the relative order of elements that are
   *  not removed is unchanged.
   *  Elements between the end of the resulting sequence and @p __last
   *  are still present, but their value is unspecified.
  */
  template<typename _ForwardIterator, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    unique(_ForwardIterator __first, _ForwardIterator __last,
	   _BinaryPredicate __binary_pred)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_ForwardIteratorConcept<
				  _ForwardIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_BinaryPredicate,
		typename iterator_traits<_ForwardIterator>::value_type,
		typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__unique(__first, __last,
			   __gnu_cxx::__ops::__iter_comp_iter(__binary_pred));
    }

  /**
   *  This is an uglified
   *  unique_copy(_InputIterator, _InputIterator, _OutputIterator,
   *              _BinaryPredicate)
   *  overloaded for forward iterators and output iterator as result.
  */
  template<typename _ForwardIterator, typename _OutputIterator,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    __unique_copy(_ForwardIterator __first, _ForwardIterator __last,
		  _OutputIterator __result, _BinaryPredicate __binary_pred,
		  forward_iterator_tag, output_iterator_tag)
    {
      // concept requirements -- iterators already checked
      __glibcxx_function_requires(_BinaryPredicateConcept<_BinaryPredicate,
	  typename iterator_traits<_ForwardIterator>::value_type,
	  typename iterator_traits<_ForwardIterator>::value_type>)

      _ForwardIterator __next = __first;
      *__result = *__first;
      while (++__next != __last)
	if (!__binary_pred(__first, __next))
	  {
	    __first = __next;
	    *++__result = *__first;
	  }
      return ++__result;
    }

  /**
   *  This is an uglified
   *  unique_copy(_InputIterator, _InputIterator, _OutputIterator,
   *              _BinaryPredicate)
   *  overloaded for input iterators and output iterator as result.
  */
  template<typename _InputIterator, typename _OutputIterator,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    __unique_copy(_InputIterator __first, _InputIterator __last,
		  _OutputIterator __result, _BinaryPredicate __binary_pred,
		  input_iterator_tag, output_iterator_tag)
    {
      // concept requirements -- iterators already checked
      __glibcxx_function_requires(_BinaryPredicateConcept<_BinaryPredicate,
	  typename iterator_traits<_InputIterator>::value_type,
	  typename iterator_traits<_InputIterator>::value_type>)

      typename iterator_traits<_InputIterator>::value_type __value = *__first;
      __decltype(__gnu_cxx::__ops::__iter_comp_val(__binary_pred))
	__rebound_pred
	= __gnu_cxx::__ops::__iter_comp_val(__binary_pred);
      *__result = __value;
      while (++__first != __last)
	if (!__rebound_pred(__first, __value))
	  {
	    __value = *__first;
	    *++__result = __value;
	  }
      return ++__result;
    }

  /**
   *  This is an uglified
   *  unique_copy(_InputIterator, _InputIterator, _OutputIterator,
   *              _BinaryPredicate)
   *  overloaded for input iterators and forward iterator as result.
  */
  template<typename _InputIterator, typename _ForwardIterator,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator
    __unique_copy(_InputIterator __first, _InputIterator __last,
		  _ForwardIterator __result, _BinaryPredicate __binary_pred,
		  input_iterator_tag, forward_iterator_tag)
    {
      // concept requirements -- iterators already checked
      __glibcxx_function_requires(_BinaryPredicateConcept<_BinaryPredicate,
	  typename iterator_traits<_ForwardIterator>::value_type,
	  typename iterator_traits<_InputIterator>::value_type>)
      *__result = *__first;
      while (++__first != __last)
	if (!__binary_pred(__result, __first))
	  *++__result = *__first;
      return ++__result;
    }

  /**
   *  This is an uglified reverse(_BidirectionalIterator,
   *                              _BidirectionalIterator)
   *  overloaded for bidirectional iterators.
  */
  template<typename _BidirectionalIterator>
    _GLIBCXX20_CONSTEXPR
    void
    __reverse(_BidirectionalIterator __first, _BidirectionalIterator __last,
	      bidirectional_iterator_tag)
    {
      while (true)
	if (__first == __last || __first == --__last)
	  return;
	else
	  {
	    std::iter_swap(__first, __last);
	    ++__first;
	  }
    }

  /**
   *  This is an uglified reverse(_BidirectionalIterator,
   *                              _BidirectionalIterator)
   *  overloaded for random access iterators.
  */
  template<typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    void
    __reverse(_RandomAccessIterator __first, _RandomAccessIterator __last,
	      random_access_iterator_tag)
    {
      if (__first == __last)
	return;
      --__last;
      while (__first < __last)
	{
	  std::iter_swap(__first, __last);
	  ++__first;
	  --__last;
	}
    }

  /**
   *  @brief Reverse a sequence.
   *  @ingroup mutating_algorithms
   *  @param  __first  A bidirectional iterator.
   *  @param  __last   A bidirectional iterator.
   *  @return   reverse() returns no value.
   *
   *  Reverses the order of the elements in the range @p [__first,__last),
   *  so that the first element becomes the last etc.
   *  For every @c i such that @p 0<=i<=(__last-__first)/2), @p reverse()
   *  swaps @p *(__first+i) and @p *(__last-(i+1))
  */
  template<typename _BidirectionalIterator>
    _GLIBCXX20_CONSTEXPR
    inline void
    reverse(_BidirectionalIterator __first, _BidirectionalIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_BidirectionalIteratorConcept<
				  _BidirectionalIterator>)
      __glibcxx_requires_valid_range(__first, __last);
      std::__reverse(__first, __last, std::__iterator_category(__first));
    }

  /**
   *  @brief Copy a sequence, reversing its elements.
   *  @ingroup mutating_algorithms
   *  @param  __first   A bidirectional iterator.
   *  @param  __last    A bidirectional iterator.
   *  @param  __result  An output iterator.
   *  @return  An iterator designating the end of the resulting sequence.
   *
   *  Copies the elements in the range @p [__first,__last) to the
   *  range @p [__result,__result+(__last-__first)) such that the
   *  order of the elements is reversed.  For every @c i such that @p
   *  0<=i<=(__last-__first), @p reverse_copy() performs the
   *  assignment @p *(__result+(__last-__first)-1-i) = *(__first+i).
   *  The ranges @p [__first,__last) and @p
   *  [__result,__result+(__last-__first)) must not overlap.
  */
  template<typename _BidirectionalIterator, typename _OutputIterator>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    reverse_copy(_BidirectionalIterator __first, _BidirectionalIterator __last,
		 _OutputIterator __result)
    {
      // concept requirements
      __glibcxx_function_requires(_BidirectionalIteratorConcept<
				  _BidirectionalIterator>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
		typename iterator_traits<_BidirectionalIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      while (__first != __last)
	{
	  --__last;
	  *__result = *__last;
	  ++__result;
	}
      return __result;
    }

  /**
   *  This is a helper function for the rotate algorithm specialized on RAIs.
   *  It returns the greatest common divisor of two integer values.
  */
  template<typename _EuclideanRingElement>
    _GLIBCXX20_CONSTEXPR
    _EuclideanRingElement
    __gcd(_EuclideanRingElement __m, _EuclideanRingElement __n)
    {
      while (__n != 0)
	{
	  _EuclideanRingElement __t = __m % __n;
	  __m = __n;
	  __n = __t;
	}
      return __m;
    }

  inline namespace _V2
  {

  /// This is a helper function for the rotate algorithm.
  template<typename _ForwardIterator>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator
    __rotate(_ForwardIterator __first,
	     _ForwardIterator __middle,
	     _ForwardIterator __last,
	     forward_iterator_tag)
    {
      if (__first == __middle)
	return __last;
      else if (__last == __middle)
	return __first;

      _ForwardIterator __first2 = __middle;
      do
	{
	  std::iter_swap(__first, __first2);
	  ++__first;
	  ++__first2;
	  if (__first == __middle)
	    __middle = __first2;
	}
      while (__first2 != __last);

      _ForwardIterator __ret = __first;

      __first2 = __middle;

      while (__first2 != __last)
	{
	  std::iter_swap(__first, __first2);
	  ++__first;
	  ++__first2;
	  if (__first == __middle)
	    __middle = __first2;
	  else if (__first2 == __last)
	    __first2 = __middle;
	}
      return __ret;
    }

   /// This is a helper function for the rotate algorithm.
  template<typename _BidirectionalIterator>
    _GLIBCXX20_CONSTEXPR
    _BidirectionalIterator
    __rotate(_BidirectionalIterator __first,
	     _BidirectionalIterator __middle,
	     _BidirectionalIterator __last,
	      bidirectional_iterator_tag)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_BidirectionalIteratorConcept<
				  _BidirectionalIterator>)

      if (__first == __middle)
	return __last;
      else if (__last == __middle)
	return __first;

      std::__reverse(__first,  __middle, bidirectional_iterator_tag());
      std::__reverse(__middle, __last,   bidirectional_iterator_tag());

      while (__first != __middle && __middle != __last)
	{
	  std::iter_swap(__first, --__last);
	  ++__first;
	}

      if (__first == __middle)
	{
	  std::__reverse(__middle, __last,   bidirectional_iterator_tag());
	  return __last;
	}
      else
	{
	  std::__reverse(__first,  __middle, bidirectional_iterator_tag());
	  return __first;
	}
    }

  /// This is a helper function for the rotate algorithm.
  template<typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    _RandomAccessIterator
    __rotate(_RandomAccessIterator __first,
	     _RandomAccessIterator __middle,
	     _RandomAccessIterator __last,
	     random_access_iterator_tag)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
				  _RandomAccessIterator>)

      if (__first == __middle)
	return __last;
      else if (__last == __middle)
	return __first;

      typedef typename iterator_traits<_RandomAccessIterator>::difference_type
	_Distance;
      typedef typename iterator_traits<_RandomAccessIterator>::value_type
	_ValueType;

      _Distance __n = __last   - __first;
      _Distance __k = __middle - __first;

      if (__k == __n - __k)
	{
	  std::swap_ranges(__first, __middle, __middle);
	  return __middle;
	}

      _RandomAccessIterator __p = __first;
      _RandomAccessIterator __ret = __first + (__last - __middle);

      for (;;)
	{
	  if (__k < __n - __k)
	    {
	      if (__is_pod(_ValueType) && __k == 1)
		{
		  _ValueType __t = _GLIBCXX_MOVE(*__p);
		  _GLIBCXX_MOVE3(__p + 1, __p + __n, __p);
		  *(__p + __n - 1) = _GLIBCXX_MOVE(__t);
		  return __ret;
		}
	      _RandomAccessIterator __q = __p + __k;
	      for (_Distance __i = 0; __i < __n - __k; ++ __i)
		{
		  std::iter_swap(__p, __q);
		  ++__p;
		  ++__q;
		}
	      __n %= __k;
	      if (__n == 0)
		return __ret;
	      std::swap(__n, __k);
	      __k = __n - __k;
	    }
	  else
	    {
	      __k = __n - __k;
	      if (__is_pod(_ValueType) && __k == 1)
		{
		  _ValueType __t = _GLIBCXX_MOVE(*(__p + __n - 1));
		  _GLIBCXX_MOVE_BACKWARD3(__p, __p + __n - 1, __p + __n);
		  *__p = _GLIBCXX_MOVE(__t);
		  return __ret;
		}
	      _RandomAccessIterator __q = __p + __n;
	      __p = __q - __k;
	      for (_Distance __i = 0; __i < __n - __k; ++ __i)
		{
		  --__p;
		  --__q;
		  std::iter_swap(__p, __q);
		}
	      __n %= __k;
	      if (__n == 0)
		return __ret;
	      std::swap(__n, __k);
	    }
	}
    }

   // _GLIBCXX_RESOLVE_LIB_DEFECTS
   // DR 488. rotate throws away useful information
  /**
   *  @brief Rotate the elements of a sequence.
   *  @ingroup mutating_algorithms
   *  @param  __first   A forward iterator.
   *  @param  __middle  A forward iterator.
   *  @param  __last    A forward iterator.
   *  @return  first + (last - middle).
   *
   *  Rotates the elements of the range @p [__first,__last) by 
   *  @p (__middle - __first) positions so that the element at @p __middle
   *  is moved to @p __first, the element at @p __middle+1 is moved to
   *  @p __first+1 and so on for each element in the range
   *  @p [__first,__last).
   *
   *  This effectively swaps the ranges @p [__first,__middle) and
   *  @p [__middle,__last).
   *
   *  Performs
   *   @p *(__first+(n+(__last-__middle))%(__last-__first))=*(__first+n)
   *  for each @p n in the range @p [0,__last-__first).
  */
  template<typename _ForwardIterator>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    rotate(_ForwardIterator __first, _ForwardIterator __middle,
	   _ForwardIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_ForwardIteratorConcept<
				  _ForwardIterator>)
      __glibcxx_requires_valid_range(__first, __middle);
      __glibcxx_requires_valid_range(__middle, __last);

      return std::__rotate(__first, __middle, __last,
			   std::__iterator_category(__first));
    }

  } // namespace _V2

  /**
   *  @brief Copy a sequence, rotating its elements.
   *  @ingroup mutating_algorithms
   *  @param  __first   A forward iterator.
   *  @param  __middle  A forward iterator.
   *  @param  __last    A forward iterator.
   *  @param  __result  An output iterator.
   *  @return   An iterator designating the end of the resulting sequence.
   *
   *  Copies the elements of the range @p [__first,__last) to the
   *  range beginning at @result, rotating the copied elements by 
   *  @p (__middle-__first) positions so that the element at @p __middle
   *  is moved to @p __result, the element at @p __middle+1 is moved
   *  to @p __result+1 and so on for each element in the range @p
   *  [__first,__last).
   *
   *  Performs 
   *  @p *(__result+(n+(__last-__middle))%(__last-__first))=*(__first+n)
   *  for each @p n in the range @p [0,__last-__first).
  */
  template<typename _ForwardIterator, typename _OutputIterator>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    rotate_copy(_ForwardIterator __first, _ForwardIterator __middle,
		_ForwardIterator __last, _OutputIterator __result)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
		typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __middle);
      __glibcxx_requires_valid_range(__middle, __last);

      return std::copy(__first, __middle,
		       std::copy(__middle, __last, __result));
    }

  /// This is a helper function...
  template<typename _ForwardIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator
    __partition(_ForwardIterator __first, _ForwardIterator __last,
		_Predicate __pred, forward_iterator_tag)
    {
      if (__first == __last)
	return __first;

      while (__pred(*__first))
	if (++__first == __last)
	  return __first;

      _ForwardIterator __next = __first;

      while (++__next != __last)
	if (__pred(*__next))
	  {
	    std::iter_swap(__first, __next);
	    ++__first;
	  }

      return __first;
    }

  /// This is a helper function...
  template<typename _BidirectionalIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _BidirectionalIterator
    __partition(_BidirectionalIterator __first, _BidirectionalIterator __last,
		_Predicate __pred, bidirectional_iterator_tag)
    {
      while (true)
	{
	  while (true)
	    if (__first == __last)
	      return __first;
	    else if (__pred(*__first))
	      ++__first;
	    else
	      break;
	  --__last;
	  while (true)
	    if (__first == __last)
	      return __first;
	    else if (!bool(__pred(*__last)))
	      --__last;
	    else
	      break;
	  std::iter_swap(__first, __last);
	  ++__first;
	}
    }

  // partition

  /// This is a helper function...
  /// Requires __first != __last and !__pred(__first)
  /// and __len == distance(__first, __last).
  ///
  /// !__pred(__first) allows us to guarantee that we don't
  /// move-assign an element onto itself.
  template<typename _ForwardIterator, typename _Pointer, typename _Predicate,
	   typename _Distance>
    _ForwardIterator
    __stable_partition_adaptive(_ForwardIterator __first,
				_ForwardIterator __last,
				_Predicate __pred, _Distance __len,
				_Pointer __buffer,
				_Distance __buffer_size)
    {
      if (__len == 1)
	return __first;

      if (__len <= __buffer_size)
	{
	  _ForwardIterator __result1 = __first;
	  _Pointer __result2 = __buffer;

	  // The precondition guarantees that !__pred(__first), so
	  // move that element to the buffer before starting the loop.
	  // This ensures that we only call __pred once per element.
	  *__result2 = _GLIBCXX_MOVE(*__first);
	  ++__result2;
	  ++__first;
	  for (; __first != __last; ++__first)
	    if (__pred(__first))
	      {
		*__result1 = _GLIBCXX_MOVE(*__first);
		++__result1;
	      }
	    else
	      {
		*__result2 = _GLIBCXX_MOVE(*__first);
		++__result2;
	      }

	  _GLIBCXX_MOVE3(__buffer, __result2, __result1);
	  return __result1;
	}

      _ForwardIterator __middle = __first;
      std::advance(__middle, __len / 2);
      _ForwardIterator __left_split =
	std::__stable_partition_adaptive(__first, __middle, __pred,
					 __len / 2, __buffer,
					 __buffer_size);

      // Advance past true-predicate values to satisfy this
      // function's preconditions.
      _Distance __right_len = __len - __len / 2;
      _ForwardIterator __right_split =
	std::__find_if_not_n(__middle, __right_len, __pred);

      if (__right_len)
	__right_split =
	  std::__stable_partition_adaptive(__right_split, __last, __pred,
					   __right_len,
					   __buffer, __buffer_size);

      return std::rotate(__left_split, __middle, __right_split);
    }

  template<typename _ForwardIterator, typename _Predicate>
    _ForwardIterator
    __stable_partition(_ForwardIterator __first, _ForwardIterator __last,
		       _Predicate __pred)
    {
      __first = std::__find_if_not(__first, __last, __pred);

      if (__first == __last)
	return __first;

      typedef typename iterator_traits<_ForwardIterator>::value_type
	_ValueType;
      typedef typename iterator_traits<_ForwardIterator>::difference_type
	_DistanceType;

      _Temporary_buffer<_ForwardIterator, _ValueType>
	__buf(__first, std::distance(__first, __last));
      return
	std::__stable_partition_adaptive(__first, __last, __pred,
					 _DistanceType(__buf.requested_size()),
					 __buf.begin(),
					 _DistanceType(__buf.size()));
    }

  /**
   *  @brief Move elements for which a predicate is true to the beginning
   *         of a sequence, preserving relative ordering.
   *  @ingroup mutating_algorithms
   *  @param  __first   A forward iterator.
   *  @param  __last    A forward iterator.
   *  @param  __pred    A predicate functor.
   *  @return  An iterator @p middle such that @p __pred(i) is true for each
   *  iterator @p i in the range @p [first,middle) and false for each @p i
   *  in the range @p [middle,last).
   *
   *  Performs the same function as @p partition() with the additional
   *  guarantee that the relative ordering of elements in each group is
   *  preserved, so any two elements @p x and @p y in the range
   *  @p [__first,__last) such that @p __pred(x)==__pred(y) will have the same
   *  relative ordering after calling @p stable_partition().
  */
  template<typename _ForwardIterator, typename _Predicate>
    inline _ForwardIterator
    stable_partition(_ForwardIterator __first, _ForwardIterator __last,
		     _Predicate __pred)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_ForwardIteratorConcept<
				  _ForwardIterator>)
      __glibcxx_function_requires(_UnaryPredicateConcept<_Predicate,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__stable_partition(__first, __last,
				     __gnu_cxx::__ops::__pred_iter(__pred));
    }

  /// This is a helper function for the sort routines.
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    __heap_select(_RandomAccessIterator __first,
		  _RandomAccessIterator __middle,
		  _RandomAccessIterator __last, _Compare __comp)
    {
      std::__make_heap(__first, __middle, __comp);
      for (_RandomAccessIterator __i = __middle; __i < __last; ++__i)
	if (__comp(__i, __first))
	  std::__pop_heap(__first, __middle, __i, __comp);
    }

  // partial_sort

  template<typename _InputIterator, typename _RandomAccessIterator,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _RandomAccessIterator
    __partial_sort_copy(_InputIterator __first, _InputIterator __last,
			_RandomAccessIterator __result_first,
			_RandomAccessIterator __result_last,
			_Compare __comp)
    {
      typedef typename iterator_traits<_InputIterator>::value_type
	_InputValueType;
      typedef iterator_traits<_RandomAccessIterator> _RItTraits;
      typedef typename _RItTraits::difference_type _DistanceType;

      if (__result_first == __result_last)
	return __result_last;
      _RandomAccessIterator __result_real_last = __result_first;
      while (__first != __last && __result_real_last != __result_last)
	{
	  *__result_real_last = *__first;
	  ++__result_real_last;
	  ++__first;
	}
      
      std::__make_heap(__result_first, __result_real_last, __comp);
      while (__first != __last)
	{
	  if (__comp(__first, __result_first))
	    std::__adjust_heap(__result_first, _DistanceType(0),
			       _DistanceType(__result_real_last
					     - __result_first),
			       _InputValueType(*__first), __comp);
	  ++__first;
	}
      std::__sort_heap(__result_first, __result_real_last, __comp);
      return __result_real_last;
    }

  /**
   *  @brief Copy the smallest elements of a sequence.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __result_first   A random-access iterator.
   *  @param  __result_last    Another random-access iterator.
   *  @return   An iterator indicating the end of the resulting sequence.
   *
   *  Copies and sorts the smallest N values from the range @p [__first,__last)
   *  to the range beginning at @p __result_first, where the number of
   *  elements to be copied, @p N, is the smaller of @p (__last-__first) and
   *  @p (__result_last-__result_first).
   *  After the sort if @e i and @e j are iterators in the range
   *  @p [__result_first,__result_first+N) such that i precedes j then
   *  *j<*i is false.
   *  The value returned is @p __result_first+N.
  */
  template<typename _InputIterator, typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    inline _RandomAccessIterator
    partial_sort_copy(_InputIterator __first, _InputIterator __last,
		      _RandomAccessIterator __result_first,
		      _RandomAccessIterator __result_last)
    {
#ifdef _GLIBCXX_CONCEPT_CHECKS
      typedef typename iterator_traits<_InputIterator>::value_type
	_InputValueType;
      typedef typename iterator_traits<_RandomAccessIterator>::value_type
	_OutputValueType;
#endif

      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_ConvertibleConcept<_InputValueType,
				  _OutputValueType>)
      __glibcxx_function_requires(_LessThanOpConcept<_InputValueType,
						     _OutputValueType>)
      __glibcxx_function_requires(_LessThanComparableConcept<_OutputValueType>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);
      __glibcxx_requires_valid_range(__result_first, __result_last);

      return std::__partial_sort_copy(__first, __last,
				      __result_first, __result_last,
				      __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief Copy the smallest elements of a sequence using a predicate for
   *         comparison.
   *  @ingroup sorting_algorithms
   *  @param  __first   An input iterator.
   *  @param  __last    Another input iterator.
   *  @param  __result_first   A random-access iterator.
   *  @param  __result_last    Another random-access iterator.
   *  @param  __comp    A comparison functor.
   *  @return   An iterator indicating the end of the resulting sequence.
   *
   *  Copies and sorts the smallest N values from the range @p [__first,__last)
   *  to the range beginning at @p result_first, where the number of
   *  elements to be copied, @p N, is the smaller of @p (__last-__first) and
   *  @p (__result_last-__result_first).
   *  After the sort if @e i and @e j are iterators in the range
   *  @p [__result_first,__result_first+N) such that i precedes j then
   *  @p __comp(*j,*i) is false.
   *  The value returned is @p __result_first+N.
  */
  template<typename _InputIterator, typename _RandomAccessIterator,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _RandomAccessIterator
    partial_sort_copy(_InputIterator __first, _InputIterator __last,
		      _RandomAccessIterator __result_first,
		      _RandomAccessIterator __result_last,
		      _Compare __comp)
    {
#ifdef _GLIBCXX_CONCEPT_CHECKS
      typedef typename iterator_traits<_InputIterator>::value_type
	_InputValueType;
      typedef typename iterator_traits<_RandomAccessIterator>::value_type
	_OutputValueType;
#endif

      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
				  _RandomAccessIterator>)
      __glibcxx_function_requires(_ConvertibleConcept<_InputValueType,
				  _OutputValueType>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
				  _InputValueType, _OutputValueType>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
				  _OutputValueType, _OutputValueType>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);
      __glibcxx_requires_valid_range(__result_first, __result_last);

      return std::__partial_sort_copy(__first, __last,
				      __result_first, __result_last,
				__gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  /// This is a helper function for the sort routine.
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    __unguarded_linear_insert(_RandomAccessIterator __last,
			      _Compare __comp)
    {
      typename iterator_traits<_RandomAccessIterator>::value_type
	__val = _GLIBCXX_MOVE(*__last);
      _RandomAccessIterator __next = __last;
      --__next;
      while (__comp(__val, __next))
	{
	  *__last = _GLIBCXX_MOVE(*__next);
	  __last = __next;
	  --__next;
	}
      *__last = _GLIBCXX_MOVE(__val);
    }

  /// This is a helper function for the sort routine.
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    __insertion_sort(_RandomAccessIterator __first,
		     _RandomAccessIterator __last, _Compare __comp)
    {
      if (__first == __last) return;

      for (_RandomAccessIterator __i = __first + 1; __i != __last; ++__i)
	{
	  if (__comp(__i, __first))
	    {
	      typename iterator_traits<_RandomAccessIterator>::value_type
		__val = _GLIBCXX_MOVE(*__i);
	      _GLIBCXX_MOVE_BACKWARD3(__first, __i, __i + 1);
	      *__first = _GLIBCXX_MOVE(__val);
	    }
	  else
	    std::__unguarded_linear_insert(__i,
				__gnu_cxx::__ops::__val_comp_iter(__comp));
	}
    }

  /// This is a helper function for the sort routine.
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline void
    __unguarded_insertion_sort(_RandomAccessIterator __first,
			       _RandomAccessIterator __last, _Compare __comp)
    {
      for (_RandomAccessIterator __i = __first; __i != __last; ++__i)
	std::__unguarded_linear_insert(__i,
				__gnu_cxx::__ops::__val_comp_iter(__comp));
    }

  /**
   *  @doctodo
   *  This controls some aspect of the sort routines.
  */
  enum { _S_threshold = 16 };

  /// This is a helper function for the sort routine.
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    __final_insertion_sort(_RandomAccessIterator __first,
			   _RandomAccessIterator __last, _Compare __comp)
    {
      if (__last - __first > int(_S_threshold))
	{
	  std::__insertion_sort(__first, __first + int(_S_threshold), __comp);
	  std::__unguarded_insertion_sort(__first + int(_S_threshold), __last,
					  __comp);
	}
      else
	std::__insertion_sort(__first, __last, __comp);
    }

  /// This is a helper function...
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _RandomAccessIterator
    __unguarded_partition(_RandomAccessIterator __first,
			  _RandomAccessIterator __last,
			  _RandomAccessIterator __pivot, _Compare __comp)
    {
      while (true)
	{
	  while (__comp(__first, __pivot))
	    ++__first;
	  --__last;
	  while (__comp(__pivot, __last))
	    --__last;
	  if (!(__first < __last))
	    return __first;
	  std::iter_swap(__first, __last);
	  ++__first;
	}
    }

  /// This is a helper function...
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _RandomAccessIterator
    __unguarded_partition_pivot(_RandomAccessIterator __first,
				_RandomAccessIterator __last, _Compare __comp)
    {
      _RandomAccessIterator __mid = __first + (__last - __first) / 2;
      std::__move_median_to_first(__first, __first + 1, __mid, __last - 1,
				  __comp);
      return std::__unguarded_partition(__first + 1, __last, __first, __comp);
    }

  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline void
    __partial_sort(_RandomAccessIterator __first,
		   _RandomAccessIterator __middle,
		   _RandomAccessIterator __last,
		   _Compare __comp)
    {
      std::__heap_select(__first, __middle, __last, __comp);
      std::__sort_heap(__first, __middle, __comp);
    }

  /// This is a helper function for the sort routine.
  template<typename _RandomAccessIterator, typename _Size, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    __introsort_loop(_RandomAccessIterator __first,
		     _RandomAccessIterator __last,
		     _Size __depth_limit, _Compare __comp)
    {
      while (__last - __first > int(_S_threshold))
	{
	  if (__depth_limit == 0)
	    {
	      std::__partial_sort(__first, __last, __last, __comp);
	      return;
	    }
	  --__depth_limit;
	  _RandomAccessIterator __cut =
	    std::__unguarded_partition_pivot(__first, __last, __comp);
	  std::__introsort_loop(__cut, __last, __depth_limit, __comp);
	  __last = __cut;
	}
    }

  // sort

  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline void
    __sort(_RandomAccessIterator __first, _RandomAccessIterator __last,
	   _Compare __comp)
    {
      if (__first != __last)
	{
	  std::__introsort_loop(__first, __last,
				std::__lg(__last - __first) * 2,
				__comp);
	  std::__final_insertion_sort(__first, __last, __comp);
	}
    }

  template<typename _RandomAccessIterator, typename _Size, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    __introselect(_RandomAccessIterator __first, _RandomAccessIterator __nth,
		  _RandomAccessIterator __last, _Size __depth_limit,
		  _Compare __comp)
    {
      while (__last - __first > 3)
	{
	  if (__depth_limit == 0)
	    {
	      std::__heap_select(__first, __nth + 1, __last, __comp);
	      // Place the nth largest element in its final position.
	      std::iter_swap(__first, __nth);
	      return;
	    }
	  --__depth_limit;
	  _RandomAccessIterator __cut =
	    std::__unguarded_partition_pivot(__first, __last, __comp);
	  if (__cut <= __nth)
	    __first = __cut;
	  else
	    __last = __cut;
	}
      std::__insertion_sort(__first, __last, __comp);
    }

  // nth_element

  // lower_bound moved to stl_algobase.h

  /**
   *  @brief Finds the first position in which @p __val could be inserted
   *         without changing the ordering.
   *  @ingroup binary_search_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __val     The search term.
   *  @param  __comp    A functor to use for comparisons.
   *  @return An iterator pointing to the first element <em>not less
   *           than</em> @p __val, or end() if every element is less
   *           than @p __val.
   *  @ingroup binary_search_algorithms
   *
   *  The comparison function should have the same effects on ordering as
   *  the function used for the initial sort.
  */
  template<typename _ForwardIterator, typename _Tp, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    lower_bound(_ForwardIterator __first, _ForwardIterator __last,
		const _Tp& __val, _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	typename iterator_traits<_ForwardIterator>::value_type, _Tp>)
      __glibcxx_requires_partitioned_lower_pred(__first, __last,
						__val, __comp);

      return std::__lower_bound(__first, __last, __val,
				__gnu_cxx::__ops::__iter_comp_val(__comp));
    }

  template<typename _ForwardIterator, typename _Tp, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator
    __upper_bound(_ForwardIterator __first, _ForwardIterator __last,
		  const _Tp& __val, _Compare __comp)
    {
      typedef typename iterator_traits<_ForwardIterator>::difference_type
	_DistanceType;

      _DistanceType __len = std::distance(__first, __last);

      while (__len > 0)
	{
	  _DistanceType __half = __len >> 1;
	  _ForwardIterator __middle = __first;
	  std::advance(__middle, __half);
	  if (__comp(__val, __middle))
	    __len = __half;
	  else
	    {
	      __first = __middle;
	      ++__first;
	      __len = __len - __half - 1;
	    }
	}
      return __first;
    }

  /**
   *  @brief Finds the last position in which @p __val could be inserted
   *         without changing the ordering.
   *  @ingroup binary_search_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __val     The search term.
   *  @return  An iterator pointing to the first element greater than @p __val,
   *           or end() if no elements are greater than @p __val.
   *  @ingroup binary_search_algorithms
  */
  template<typename _ForwardIterator, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    upper_bound(_ForwardIterator __first, _ForwardIterator __last,
		const _Tp& __val)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_LessThanOpConcept<
	_Tp, typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_partitioned_upper(__first, __last, __val);

      return std::__upper_bound(__first, __last, __val,
				__gnu_cxx::__ops::__val_less_iter());
    }

  /**
   *  @brief Finds the last position in which @p __val could be inserted
   *         without changing the ordering.
   *  @ingroup binary_search_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __val     The search term.
   *  @param  __comp    A functor to use for comparisons.
   *  @return  An iterator pointing to the first element greater than @p __val,
   *           or end() if no elements are greater than @p __val.
   *  @ingroup binary_search_algorithms
   *
   *  The comparison function should have the same effects on ordering as
   *  the function used for the initial sort.
  */
  template<typename _ForwardIterator, typename _Tp, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    upper_bound(_ForwardIterator __first, _ForwardIterator __last,
		const _Tp& __val, _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	_Tp, typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_partitioned_upper_pred(__first, __last,
						__val, __comp);

      return std::__upper_bound(__first, __last, __val,
				__gnu_cxx::__ops::__val_comp_iter(__comp));
    }

  template<typename _ForwardIterator, typename _Tp,
	   typename _CompareItTp, typename _CompareTpIt>
    _GLIBCXX20_CONSTEXPR
    pair<_ForwardIterator, _ForwardIterator>
    __equal_range(_ForwardIterator __first, _ForwardIterator __last,
		  const _Tp& __val,
		  _CompareItTp __comp_it_val, _CompareTpIt __comp_val_it)
    {
      typedef typename iterator_traits<_ForwardIterator>::difference_type
	_DistanceType;

      _DistanceType __len = std::distance(__first, __last);

      while (__len > 0)
	{
	  _DistanceType __half = __len >> 1;
	  _ForwardIterator __middle = __first;
	  std::advance(__middle, __half);
	  if (__comp_it_val(__middle, __val))
	    {
	      __first = __middle;
	      ++__first;
	      __len = __len - __half - 1;
	    }
	  else if (__comp_val_it(__val, __middle))
	    __len = __half;
	  else
	    {
	      _ForwardIterator __left
		= std::__lower_bound(__first, __middle, __val, __comp_it_val);
	      std::advance(__first, __len);
	      _ForwardIterator __right
		= std::__upper_bound(++__middle, __first, __val, __comp_val_it);
	      return pair<_ForwardIterator, _ForwardIterator>(__left, __right);
	    }
	}
      return pair<_ForwardIterator, _ForwardIterator>(__first, __first);
    }

  /**
   *  @brief Finds the largest subrange in which @p __val could be inserted
   *         at any place in it without changing the ordering.
   *  @ingroup binary_search_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __val     The search term.
   *  @return  An pair of iterators defining the subrange.
   *  @ingroup binary_search_algorithms
   *
   *  This is equivalent to
   *  @code
   *    std::make_pair(lower_bound(__first, __last, __val),
   *                   upper_bound(__first, __last, __val))
   *  @endcode
   *  but does not actually call those functions.
  */
  template<typename _ForwardIterator, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline pair<_ForwardIterator, _ForwardIterator>
    equal_range(_ForwardIterator __first, _ForwardIterator __last,
		const _Tp& __val)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_LessThanOpConcept<
	typename iterator_traits<_ForwardIterator>::value_type, _Tp>)
      __glibcxx_function_requires(_LessThanOpConcept<
	_Tp, typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_partitioned_lower(__first, __last, __val);
      __glibcxx_requires_partitioned_upper(__first, __last, __val);

      return std::__equal_range(__first, __last, __val,
				__gnu_cxx::__ops::__iter_less_val(),
				__gnu_cxx::__ops::__val_less_iter());
    }

  /**
   *  @brief Finds the largest subrange in which @p __val could be inserted
   *         at any place in it without changing the ordering.
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __val     The search term.
   *  @param  __comp    A functor to use for comparisons.
   *  @return  An pair of iterators defining the subrange.
   *  @ingroup binary_search_algorithms
   *
   *  This is equivalent to
   *  @code
   *    std::make_pair(lower_bound(__first, __last, __val, __comp),
   *                   upper_bound(__first, __last, __val, __comp))
   *  @endcode
   *  but does not actually call those functions.
  */
  template<typename _ForwardIterator, typename _Tp, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline pair<_ForwardIterator, _ForwardIterator>
    equal_range(_ForwardIterator __first, _ForwardIterator __last,
		const _Tp& __val, _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	typename iterator_traits<_ForwardIterator>::value_type, _Tp>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	_Tp, typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_partitioned_lower_pred(__first, __last,
						__val, __comp);
      __glibcxx_requires_partitioned_upper_pred(__first, __last,
						__val, __comp);

      return std::__equal_range(__first, __last, __val,
				__gnu_cxx::__ops::__iter_comp_val(__comp),
				__gnu_cxx::__ops::__val_comp_iter(__comp));
    }

  /**
   *  @brief Determines whether an element exists in a range.
   *  @ingroup binary_search_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __val     The search term.
   *  @return True if @p __val (or its equivalent) is in [@p
   *  __first,@p __last ].
   *
   *  Note that this does not actually return an iterator to @p __val.  For
   *  that, use std::find or a container's specialized find member functions.
  */
  template<typename _ForwardIterator, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    bool
    binary_search(_ForwardIterator __first, _ForwardIterator __last,
		  const _Tp& __val)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_LessThanOpConcept<
	_Tp, typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_partitioned_lower(__first, __last, __val);
      __glibcxx_requires_partitioned_upper(__first, __last, __val);

      _ForwardIterator __i
	= std::__lower_bound(__first, __last, __val,
			     __gnu_cxx::__ops::__iter_less_val());
      return __i != __last && !(__val < *__i);
    }

  /**
   *  @brief Determines whether an element exists in a range.
   *  @ingroup binary_search_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __val     The search term.
   *  @param  __comp    A functor to use for comparisons.
   *  @return  True if @p __val (or its equivalent) is in @p [__first,__last].
   *
   *  Note that this does not actually return an iterator to @p __val.  For
   *  that, use std::find or a container's specialized find member functions.
   *
   *  The comparison function should have the same effects on ordering as
   *  the function used for the initial sort.
  */
  template<typename _ForwardIterator, typename _Tp, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    bool
    binary_search(_ForwardIterator __first, _ForwardIterator __last,
		  const _Tp& __val, _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	_Tp, typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_partitioned_lower_pred(__first, __last,
						__val, __comp);
      __glibcxx_requires_partitioned_upper_pred(__first, __last,
						__val, __comp);

      _ForwardIterator __i
	= std::__lower_bound(__first, __last, __val,
			     __gnu_cxx::__ops::__iter_comp_val(__comp));
      return __i != __last && !bool(__comp(__val, *__i));
    }

  // merge

  /// This is a helper function for the __merge_adaptive routines.
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator, typename _Compare>
    void
    __move_merge_adaptive(_InputIterator1 __first1, _InputIterator1 __last1,
			  _InputIterator2 __first2, _InputIterator2 __last2,
			  _OutputIterator __result, _Compare __comp)
    {
      while (__first1 != __last1 && __first2 != __last2)
	{
	  if (__comp(__first2, __first1))
	    {
	      *__result = _GLIBCXX_MOVE(*__first2);
	      ++__first2;
	    }
	  else
	    {
	      *__result = _GLIBCXX_MOVE(*__first1);
	      ++__first1;
	    }
	  ++__result;
	}
      if (__first1 != __last1)
	_GLIBCXX_MOVE3(__first1, __last1, __result);
    }

  /// This is a helper function for the __merge_adaptive routines.
  template<typename _BidirectionalIterator1, typename _BidirectionalIterator2,
	   typename _BidirectionalIterator3, typename _Compare>
    void
    __move_merge_adaptive_backward(_BidirectionalIterator1 __first1,
				   _BidirectionalIterator1 __last1,
				   _BidirectionalIterator2 __first2,
				   _BidirectionalIterator2 __last2,
				   _BidirectionalIterator3 __result,
				   _Compare __comp)
    {
      if (__first1 == __last1)
	{
	  _GLIBCXX_MOVE_BACKWARD3(__first2, __last2, __result);
	  return;
	}
      else if (__first2 == __last2)
	return;

      --__last1;
      --__last2;
      while (true)
	{
	  if (__comp(__last2, __last1))
	    {
	      *--__result = _GLIBCXX_MOVE(*__last1);
	      if (__first1 == __last1)
		{
		  _GLIBCXX_MOVE_BACKWARD3(__first2, ++__last2, __result);
		  return;
		}
	      --__last1;
	    }
	  else
	    {
	      *--__result = _GLIBCXX_MOVE(*__last2);
	      if (__first2 == __last2)
		return;
	      --__last2;
	    }
	}
    }

  /// This is a helper function for the merge routines.
  template<typename _BidirectionalIterator1, typename _BidirectionalIterator2,
	   typename _Distance>
    _BidirectionalIterator1
    __rotate_adaptive(_BidirectionalIterator1 __first,
		      _BidirectionalIterator1 __middle,
		      _BidirectionalIterator1 __last,
		      _Distance __len1, _Distance __len2,
		      _BidirectionalIterator2 __buffer,
		      _Distance __buffer_size)
    {
      _BidirectionalIterator2 __buffer_end;
      if (__len1 > __len2 && __len2 <= __buffer_size)
	{
	  if (__len2)
	    {
	      __buffer_end = _GLIBCXX_MOVE3(__middle, __last, __buffer);
	      _GLIBCXX_MOVE_BACKWARD3(__first, __middle, __last);
	      return _GLIBCXX_MOVE3(__buffer, __buffer_end, __first);
	    }
	  else
	    return __first;
	}
      else if (__len1 <= __buffer_size)
	{
	  if (__len1)
	    {
	      __buffer_end = _GLIBCXX_MOVE3(__first, __middle, __buffer);
	      _GLIBCXX_MOVE3(__middle, __last, __first);
	      return _GLIBCXX_MOVE_BACKWARD3(__buffer, __buffer_end, __last);
	    }
	  else
	    return __last;
	}
      else
	return std::rotate(__first, __middle, __last);
    }

  /// This is a helper function for the merge routines.
  template<typename _BidirectionalIterator, typename _Distance, 
	   typename _Pointer, typename _Compare>
    void
    __merge_adaptive(_BidirectionalIterator __first,
		     _BidirectionalIterator __middle,
		     _BidirectionalIterator __last,
		     _Distance __len1, _Distance __len2,
		     _Pointer __buffer, _Distance __buffer_size,
		     _Compare __comp)
    {
      if (__len1 <= __len2 && __len1 <= __buffer_size)
	{
	  _Pointer __buffer_end = _GLIBCXX_MOVE3(__first, __middle, __buffer);
	  std::__move_merge_adaptive(__buffer, __buffer_end, __middle, __last,
				     __first, __comp);
	}
      else if (__len2 <= __buffer_size)
	{
	  _Pointer __buffer_end = _GLIBCXX_MOVE3(__middle, __last, __buffer);
	  std::__move_merge_adaptive_backward(__first, __middle, __buffer,
					      __buffer_end, __last, __comp);
	}
      else
	{
	  _BidirectionalIterator __first_cut = __first;
	  _BidirectionalIterator __second_cut = __middle;
	  _Distance __len11 = 0;
	  _Distance __len22 = 0;
	  if (__len1 > __len2)
	    {
	      __len11 = __len1 / 2;
	      std::advance(__first_cut, __len11);
	      __second_cut
		= std::__lower_bound(__middle, __last, *__first_cut,
				     __gnu_cxx::__ops::__iter_comp_val(__comp));
	      __len22 = std::distance(__middle, __second_cut);
	    }
	  else
	    {
	      __len22 = __len2 / 2;
	      std::advance(__second_cut, __len22);
	      __first_cut
		= std::__upper_bound(__first, __middle, *__second_cut,
				     __gnu_cxx::__ops::__val_comp_iter(__comp));
	      __len11 = std::distance(__first, __first_cut);
	    }

	  _BidirectionalIterator __new_middle
	    = std::__rotate_adaptive(__first_cut, __middle, __second_cut,
				     __len1 - __len11, __len22, __buffer,
				     __buffer_size);
	  std::__merge_adaptive(__first, __first_cut, __new_middle, __len11,
				__len22, __buffer, __buffer_size, __comp);
	  std::__merge_adaptive(__new_middle, __second_cut, __last,
				__len1 - __len11,
				__len2 - __len22, __buffer,
				__buffer_size, __comp);
	}
    }

  /// This is a helper function for the merge routines.
  template<typename _BidirectionalIterator, typename _Distance,
	   typename _Compare>
    void
    __merge_without_buffer(_BidirectionalIterator __first,
			   _BidirectionalIterator __middle,
			   _BidirectionalIterator __last,
			   _Distance __len1, _Distance __len2,
			   _Compare __comp)
    {
      if (__len1 == 0 || __len2 == 0)
	return;

      if (__len1 + __len2 == 2)
	{
	  if (__comp(__middle, __first))
	    std::iter_swap(__first, __middle);
	  return;
	}

      _BidirectionalIterator __first_cut = __first;
      _BidirectionalIterator __second_cut = __middle;
      _Distance __len11 = 0;
      _Distance __len22 = 0;
      if (__len1 > __len2)
	{
	  __len11 = __len1 / 2;
	  std::advance(__first_cut, __len11);
	  __second_cut
	    = std::__lower_bound(__middle, __last, *__first_cut,
				 __gnu_cxx::__ops::__iter_comp_val(__comp));
	  __len22 = std::distance(__middle, __second_cut);
	}
      else
	{
	  __len22 = __len2 / 2;
	  std::advance(__second_cut, __len22);
	  __first_cut
	    = std::__upper_bound(__first, __middle, *__second_cut,
				 __gnu_cxx::__ops::__val_comp_iter(__comp));
	  __len11 = std::distance(__first, __first_cut);
	}

      _BidirectionalIterator __new_middle
	= std::rotate(__first_cut, __middle, __second_cut);
      std::__merge_without_buffer(__first, __first_cut, __new_middle,
				  __len11, __len22, __comp);
      std::__merge_without_buffer(__new_middle, __second_cut, __last,
				  __len1 - __len11, __len2 - __len22, __comp);
    }

  template<typename _BidirectionalIterator, typename _Compare>
    void
    __inplace_merge(_BidirectionalIterator __first,
		    _BidirectionalIterator __middle,
		    _BidirectionalIterator __last,
		    _Compare __comp)
    {
      typedef typename iterator_traits<_BidirectionalIterator>::value_type
	  _ValueType;
      typedef typename iterator_traits<_BidirectionalIterator>::difference_type
	  _DistanceType;

      if (__first == __middle || __middle == __last)
	return;

      const _DistanceType __len1 = std::distance(__first, __middle);
      const _DistanceType __len2 = std::distance(__middle, __last);

      typedef _Temporary_buffer<_BidirectionalIterator, _ValueType> _TmpBuf;
      _TmpBuf __buf(__first, __len1 + __len2);

      if (__buf.begin() == 0)
	std::__merge_without_buffer
	  (__first, __middle, __last, __len1, __len2, __comp);
      else
	std::__merge_adaptive
	  (__first, __middle, __last, __len1, __len2, __buf.begin(),
	   _DistanceType(__buf.size()), __comp);
    }

  /**
   *  @brief Merges two sorted ranges in place.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __middle  Another iterator.
   *  @param  __last    Another iterator.
   *  @return  Nothing.
   *
   *  Merges two sorted and consecutive ranges, [__first,__middle) and
   *  [__middle,__last), and puts the result in [__first,__last).  The
   *  output will be sorted.  The sort is @e stable, that is, for
   *  equivalent elements in the two ranges, elements from the first
   *  range will always come before elements from the second.
   *
   *  If enough additional memory is available, this takes (__last-__first)-1
   *  comparisons.  Otherwise an NlogN algorithm is used, where N is
   *  distance(__first,__last).
  */
  template<typename _BidirectionalIterator>
    inline void
    inplace_merge(_BidirectionalIterator __first,
		  _BidirectionalIterator __middle,
		  _BidirectionalIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_BidirectionalIteratorConcept<
	    _BidirectionalIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_BidirectionalIterator>::value_type>)
      __glibcxx_requires_sorted(__first, __middle);
      __glibcxx_requires_sorted(__middle, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      std::__inplace_merge(__first, __middle, __last,
			   __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief Merges two sorted ranges in place.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __middle  Another iterator.
   *  @param  __last    Another iterator.
   *  @param  __comp    A functor to use for comparisons.
   *  @return  Nothing.
   *
   *  Merges two sorted and consecutive ranges, [__first,__middle) and
   *  [middle,last), and puts the result in [__first,__last).  The output will
   *  be sorted.  The sort is @e stable, that is, for equivalent
   *  elements in the two ranges, elements from the first range will always
   *  come before elements from the second.
   *
   *  If enough additional memory is available, this takes (__last-__first)-1
   *  comparisons.  Otherwise an NlogN algorithm is used, where N is
   *  distance(__first,__last).
   *
   *  The comparison function should have the same effects on ordering as
   *  the function used for the initial sort.
  */
  template<typename _BidirectionalIterator, typename _Compare>
    inline void
    inplace_merge(_BidirectionalIterator __first,
		  _BidirectionalIterator __middle,
		  _BidirectionalIterator __last,
		  _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_BidirectionalIteratorConcept<
	    _BidirectionalIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_BidirectionalIterator>::value_type,
	    typename iterator_traits<_BidirectionalIterator>::value_type>)
      __glibcxx_requires_sorted_pred(__first, __middle, __comp);
      __glibcxx_requires_sorted_pred(__middle, __last, __comp);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      std::__inplace_merge(__first, __middle, __last,
			   __gnu_cxx::__ops::__iter_comp_iter(__comp));
    }


  /// This is a helper function for the __merge_sort_loop routines.
  template<typename _InputIterator, typename _OutputIterator,
	   typename _Compare>
    _OutputIterator
    __move_merge(_InputIterator __first1, _InputIterator __last1,
		 _InputIterator __first2, _InputIterator __last2,
		 _OutputIterator __result, _Compare __comp)
    {
      while (__first1 != __last1 && __first2 != __last2)
	{
	  if (__comp(__first2, __first1))
	    {
	      *__result = _GLIBCXX_MOVE(*__first2);
	      ++__first2;
	    }
	  else
	    {
	      *__result = _GLIBCXX_MOVE(*__first1);
	      ++__first1;
	    }
	  ++__result;
	}
      return _GLIBCXX_MOVE3(__first2, __last2,
			    _GLIBCXX_MOVE3(__first1, __last1,
					   __result));
    }

  template<typename _RandomAccessIterator1, typename _RandomAccessIterator2,
	   typename _Distance, typename _Compare>
    void
    __merge_sort_loop(_RandomAccessIterator1 __first,
		      _RandomAccessIterator1 __last,
		      _RandomAccessIterator2 __result, _Distance __step_size,
		      _Compare __comp)
    {
      const _Distance __two_step = 2 * __step_size;

      while (__last - __first >= __two_step)
	{
	  __result = std::__move_merge(__first, __first + __step_size,
				       __first + __step_size,
				       __first + __two_step,
				       __result, __comp);
	  __first += __two_step;
	}
      __step_size = std::min(_Distance(__last - __first), __step_size);

      std::__move_merge(__first, __first + __step_size,
			__first + __step_size, __last, __result, __comp);
    }

  template<typename _RandomAccessIterator, typename _Distance,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    __chunk_insertion_sort(_RandomAccessIterator __first,
			   _RandomAccessIterator __last,
			   _Distance __chunk_size, _Compare __comp)
    {
      while (__last - __first >= __chunk_size)
	{
	  std::__insertion_sort(__first, __first + __chunk_size, __comp);
	  __first += __chunk_size;
	}
      std::__insertion_sort(__first, __last, __comp);
    }

  enum { _S_chunk_size = 7 };

  template<typename _RandomAccessIterator, typename _Pointer, typename _Compare>
    void
    __merge_sort_with_buffer(_RandomAccessIterator __first,
			     _RandomAccessIterator __last,
			     _Pointer __buffer, _Compare __comp)
    {
      typedef typename iterator_traits<_RandomAccessIterator>::difference_type
	_Distance;

      const _Distance __len = __last - __first;
      const _Pointer __buffer_last = __buffer + __len;

      _Distance __step_size = _S_chunk_size;
      std::__chunk_insertion_sort(__first, __last, __step_size, __comp);

      while (__step_size < __len)
	{
	  std::__merge_sort_loop(__first, __last, __buffer,
				 __step_size, __comp);
	  __step_size *= 2;
	  std::__merge_sort_loop(__buffer, __buffer_last, __first,
				 __step_size, __comp);
	  __step_size *= 2;
	}
    }

  template<typename _RandomAccessIterator, typename _Pointer,
	   typename _Distance, typename _Compare>
    void
    __stable_sort_adaptive(_RandomAccessIterator __first,
			   _RandomAccessIterator __last,
			   _Pointer __buffer, _Distance __buffer_size,
			   _Compare __comp)
    {
      const _Distance __len = (__last - __first + 1) / 2;
      const _RandomAccessIterator __middle = __first + __len;
      if (__len > __buffer_size)
	{
	  std::__stable_sort_adaptive(__first, __middle, __buffer,
				      __buffer_size, __comp);
	  std::__stable_sort_adaptive(__middle, __last, __buffer,
				      __buffer_size, __comp);
	}
      else
	{
	  std::__merge_sort_with_buffer(__first, __middle, __buffer, __comp);
	  std::__merge_sort_with_buffer(__middle, __last, __buffer, __comp);
	}
      std::__merge_adaptive(__first, __middle, __last,
			    _Distance(__middle - __first),
			    _Distance(__last - __middle),
			    __buffer, __buffer_size,
			    __comp);
    }

  /// This is a helper function for the stable sorting routines.
  template<typename _RandomAccessIterator, typename _Compare>
    void
    __inplace_stable_sort(_RandomAccessIterator __first,
			  _RandomAccessIterator __last, _Compare __comp)
    {
      if (__last - __first < 15)
	{
	  std::__insertion_sort(__first, __last, __comp);
	  return;
	}
      _RandomAccessIterator __middle = __first + (__last - __first) / 2;
      std::__inplace_stable_sort(__first, __middle, __comp);
      std::__inplace_stable_sort(__middle, __last, __comp);
      std::__merge_without_buffer(__first, __middle, __last,
				  __middle - __first,
				  __last - __middle,
				  __comp);
    }

  // stable_sort

  // Set algorithms: includes, set_union, set_intersection, set_difference,
  // set_symmetric_difference.  All of these algorithms have the precondition
  // that their input ranges are sorted and the postcondition that their output
  // ranges are sorted.

  template<typename _InputIterator1, typename _InputIterator2,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    bool
    __includes(_InputIterator1 __first1, _InputIterator1 __last1,
	       _InputIterator2 __first2, _InputIterator2 __last2,
	       _Compare __comp)
    {
      while (__first1 != __last1 && __first2 != __last2)
	if (__comp(__first2, __first1))
	  return false;
	else if (__comp(__first1, __first2))
	  ++__first1;
	else
	  {
	    ++__first1;
	    ++__first2;
	  }

      return __first2 == __last2;
    }

  /**
   *  @brief Determines whether all elements of a sequence exists in a range.
   *  @param  __first1  Start of search range.
   *  @param  __last1   End of search range.
   *  @param  __first2  Start of sequence
   *  @param  __last2   End of sequence.
   *  @return  True if each element in [__first2,__last2) is contained in order
   *  within [__first1,__last1).  False otherwise.
   *  @ingroup set_algorithms
   *
   *  This operation expects both [__first1,__last1) and
   *  [__first2,__last2) to be sorted.  Searches for the presence of
   *  each element in [__first2,__last2) within [__first1,__last1).
   *  The iterators over each range only move forward, so this is a
   *  linear algorithm.  If an element in [__first2,__last2) is not
   *  found before the search iterator reaches @p __last2, false is
   *  returned.
  */
  template<typename _InputIterator1, typename _InputIterator2>
    _GLIBCXX20_CONSTEXPR
    inline bool
    includes(_InputIterator1 __first1, _InputIterator1 __last1,
	     _InputIterator2 __first2, _InputIterator2 __last2)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_LessThanOpConcept<
	    typename iterator_traits<_InputIterator1>::value_type,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_LessThanOpConcept<
	    typename iterator_traits<_InputIterator2>::value_type,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_requires_sorted_set(__first1, __last1, __first2);
      __glibcxx_requires_sorted_set(__first2, __last2, __first1);
      __glibcxx_requires_irreflexive2(__first1, __last1);
      __glibcxx_requires_irreflexive2(__first2, __last2);

      return std::__includes(__first1, __last1, __first2, __last2,
			     __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief Determines whether all elements of a sequence exists in a range
   *  using comparison.
   *  @ingroup set_algorithms
   *  @param  __first1  Start of search range.
   *  @param  __last1   End of search range.
   *  @param  __first2  Start of sequence
   *  @param  __last2   End of sequence.
   *  @param  __comp    Comparison function to use.
   *  @return True if each element in [__first2,__last2) is contained
   *  in order within [__first1,__last1) according to comp.  False
   *  otherwise.  @ingroup set_algorithms
   *
   *  This operation expects both [__first1,__last1) and
   *  [__first2,__last2) to be sorted.  Searches for the presence of
   *  each element in [__first2,__last2) within [__first1,__last1),
   *  using comp to decide.  The iterators over each range only move
   *  forward, so this is a linear algorithm.  If an element in
   *  [__first2,__last2) is not found before the search iterator
   *  reaches @p __last2, false is returned.
  */
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline bool
    includes(_InputIterator1 __first1, _InputIterator1 __last1,
	     _InputIterator2 __first2, _InputIterator2 __last2,
	     _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_InputIterator1>::value_type,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_InputIterator2>::value_type,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_requires_sorted_set_pred(__first1, __last1, __first2, __comp);
      __glibcxx_requires_sorted_set_pred(__first2, __last2, __first1, __comp);
      __glibcxx_requires_irreflexive_pred2(__first1, __last1, __comp);
      __glibcxx_requires_irreflexive_pred2(__first2, __last2, __comp);

      return std::__includes(__first1, __last1, __first2, __last2,
			     __gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  // nth_element
  // merge
  // set_difference
  // set_intersection
  // set_union
  // stable_sort
  // set_symmetric_difference
  // min_element
  // max_element

  template<typename _BidirectionalIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    bool
    __next_permutation(_BidirectionalIterator __first,
		       _BidirectionalIterator __last, _Compare __comp)
    {
      if (__first == __last)
	return false;
      _BidirectionalIterator __i = __first;
      ++__i;
      if (__i == __last)
	return false;
      __i = __last;
      --__i;

      for(;;)
	{
	  _BidirectionalIterator __ii = __i;
	  --__i;
	  if (__comp(__i, __ii))
	    {
	      _BidirectionalIterator __j = __last;
	      while (!__comp(__i, --__j))
		{}
	      std::iter_swap(__i, __j);
	      std::__reverse(__ii, __last,
			     std::__iterator_category(__first));
	      return true;
	    }
	  if (__i == __first)
	    {
	      std::__reverse(__first, __last,
			     std::__iterator_category(__first));
	      return false;
	    }
	}
    }

  /**
   *  @brief  Permute range into the next @e dictionary ordering.
   *  @ingroup sorting_algorithms
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @return  False if wrapped to first permutation, true otherwise.
   *
   *  Treats all permutations of the range as a set of @e dictionary sorted
   *  sequences.  Permutes the current sequence into the next one of this set.
   *  Returns true if there are more sequences to generate.  If the sequence
   *  is the largest of the set, the smallest is generated and false returned.
  */
  template<typename _BidirectionalIterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    next_permutation(_BidirectionalIterator __first,
		     _BidirectionalIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_BidirectionalIteratorConcept<
				  _BidirectionalIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_BidirectionalIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      return std::__next_permutation
	(__first, __last, __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief  Permute range into the next @e dictionary ordering using
   *          comparison functor.
   *  @ingroup sorting_algorithms
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @param  __comp   A comparison functor.
   *  @return  False if wrapped to first permutation, true otherwise.
   *
   *  Treats all permutations of the range [__first,__last) as a set of
   *  @e dictionary sorted sequences ordered by @p __comp.  Permutes the current
   *  sequence into the next one of this set.  Returns true if there are more
   *  sequences to generate.  If the sequence is the largest of the set, the
   *  smallest is generated and false returned.
  */
  template<typename _BidirectionalIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline bool
    next_permutation(_BidirectionalIterator __first,
		     _BidirectionalIterator __last, _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_BidirectionalIteratorConcept<
				  _BidirectionalIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_BidirectionalIterator>::value_type,
	    typename iterator_traits<_BidirectionalIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      return std::__next_permutation
	(__first, __last, __gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  template<typename _BidirectionalIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    bool
    __prev_permutation(_BidirectionalIterator __first,
		       _BidirectionalIterator __last, _Compare __comp)
    {
      if (__first == __last)
	return false;
      _BidirectionalIterator __i = __first;
      ++__i;
      if (__i == __last)
	return false;
      __i = __last;
      --__i;

      for(;;)
	{
	  _BidirectionalIterator __ii = __i;
	  --__i;
	  if (__comp(__ii, __i))
	    {
	      _BidirectionalIterator __j = __last;
	      while (!__comp(--__j, __i))
		{}
	      std::iter_swap(__i, __j);
	      std::__reverse(__ii, __last,
			     std::__iterator_category(__first));
	      return true;
	    }
	  if (__i == __first)
	    {
	      std::__reverse(__first, __last,
			     std::__iterator_category(__first));
	      return false;
	    }
	}
    }

  /**
   *  @brief  Permute range into the previous @e dictionary ordering.
   *  @ingroup sorting_algorithms
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @return  False if wrapped to last permutation, true otherwise.
   *
   *  Treats all permutations of the range as a set of @e dictionary sorted
   *  sequences.  Permutes the current sequence into the previous one of this
   *  set.  Returns true if there are more sequences to generate.  If the
   *  sequence is the smallest of the set, the largest is generated and false
   *  returned.
  */
  template<typename _BidirectionalIterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    prev_permutation(_BidirectionalIterator __first,
		     _BidirectionalIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_BidirectionalIteratorConcept<
				  _BidirectionalIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_BidirectionalIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      return std::__prev_permutation(__first, __last,
				     __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief  Permute range into the previous @e dictionary ordering using
   *          comparison functor.
   *  @ingroup sorting_algorithms
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @param  __comp   A comparison functor.
   *  @return  False if wrapped to last permutation, true otherwise.
   *
   *  Treats all permutations of the range [__first,__last) as a set of
   *  @e dictionary sorted sequences ordered by @p __comp.  Permutes the current
   *  sequence into the previous one of this set.  Returns true if there are
   *  more sequences to generate.  If the sequence is the smallest of the set,
   *  the largest is generated and false returned.
  */
  template<typename _BidirectionalIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline bool
    prev_permutation(_BidirectionalIterator __first,
		     _BidirectionalIterator __last, _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_BidirectionalIteratorConcept<
				  _BidirectionalIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_BidirectionalIterator>::value_type,
	    typename iterator_traits<_BidirectionalIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      return std::__prev_permutation(__first, __last,
				__gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  // replace
  // replace_if

  template<typename _InputIterator, typename _OutputIterator,
	   typename _Predicate, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    __replace_copy_if(_InputIterator __first, _InputIterator __last,
		      _OutputIterator __result,
		      _Predicate __pred, const _Tp& __new_value)
    {
      for (; __first != __last; ++__first, (void)++__result)
	if (__pred(__first))
	  *__result = __new_value;
	else
	  *__result = *__first;
      return __result;
    }

  /**
   *  @brief Copy a sequence, replacing each element of one value with another
   *         value.
   *  @param  __first      An input iterator.
   *  @param  __last       An input iterator.
   *  @param  __result     An output iterator.
   *  @param  __old_value  The value to be replaced.
   *  @param  __new_value  The replacement value.
   *  @return   The end of the output sequence, @p result+(last-first).
   *
   *  Copies each element in the input range @p [__first,__last) to the
   *  output range @p [__result,__result+(__last-__first)) replacing elements
   *  equal to @p __old_value with @p __new_value.
  */
  template<typename _InputIterator, typename _OutputIterator, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    replace_copy(_InputIterator __first, _InputIterator __last,
		 _OutputIterator __result,
		 const _Tp& __old_value, const _Tp& __new_value)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_function_requires(_EqualOpConcept<
	    typename iterator_traits<_InputIterator>::value_type, _Tp>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__replace_copy_if(__first, __last, __result,
			__gnu_cxx::__ops::__iter_equals_val(__old_value),
					      __new_value);
    }

  /**
   *  @brief Copy a sequence, replacing each value for which a predicate
   *         returns true with another value.
   *  @ingroup mutating_algorithms
   *  @param  __first      An input iterator.
   *  @param  __last       An input iterator.
   *  @param  __result     An output iterator.
   *  @param  __pred       A predicate.
   *  @param  __new_value  The replacement value.
   *  @return   The end of the output sequence, @p __result+(__last-__first).
   *
   *  Copies each element in the range @p [__first,__last) to the range
   *  @p [__result,__result+(__last-__first)) replacing elements for which
   *  @p __pred returns true with @p __new_value.
  */
  template<typename _InputIterator, typename _OutputIterator,
	   typename _Predicate, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    replace_copy_if(_InputIterator __first, _InputIterator __last,
		    _OutputIterator __result,
		    _Predicate __pred, const _Tp& __new_value)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_function_requires(_UnaryPredicateConcept<_Predicate,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__replace_copy_if(__first, __last, __result,
				__gnu_cxx::__ops::__pred_iter(__pred),
					      __new_value);
    }

#if __cplusplus >= 201103L
  /**
   *  @brief  Determines whether the elements of a sequence are sorted.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @return  True if the elements are sorted, false otherwise.
  */
  template<typename _ForwardIterator>
    _GLIBCXX20_CONSTEXPR
    inline bool
    is_sorted(_ForwardIterator __first, _ForwardIterator __last)
    { return std::is_sorted_until(__first, __last) == __last; }

  /**
   *  @brief  Determines whether the elements of a sequence are sorted
   *          according to a comparison functor.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __comp    A comparison functor.
   *  @return  True if the elements are sorted, false otherwise.
  */
  template<typename _ForwardIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline bool
    is_sorted(_ForwardIterator __first, _ForwardIterator __last,
	      _Compare __comp)
    { return std::is_sorted_until(__first, __last, __comp) == __last; }

  template<typename _ForwardIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _ForwardIterator
    __is_sorted_until(_ForwardIterator __first, _ForwardIterator __last,
		      _Compare __comp)
    {
      if (__first == __last)
	return __last;

      _ForwardIterator __next = __first;
      for (++__next; __next != __last; __first = __next, (void)++__next)
	if (__comp(__next, __first))
	  return __next;
      return __next;
    }

  /**
   *  @brief  Determines the end of a sorted sequence.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @return  An iterator pointing to the last iterator i in [__first, __last)
   *           for which the range [__first, i) is sorted.
  */
  template<typename _ForwardIterator>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    is_sorted_until(_ForwardIterator __first, _ForwardIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      return std::__is_sorted_until(__first, __last,
				    __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief  Determines the end of a sorted sequence using comparison functor.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __comp    A comparison functor.
   *  @return  An iterator pointing to the last iterator i in [__first, __last)
   *           for which the range [__first, i) is sorted.
  */
  template<typename _ForwardIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    is_sorted_until(_ForwardIterator __first, _ForwardIterator __last,
		    _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_ForwardIterator>::value_type,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      return std::__is_sorted_until(__first, __last,
				    __gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  /**
   *  @brief  Determines min and max at once as an ordered pair.
   *  @ingroup sorting_algorithms
   *  @param  __a  A thing of arbitrary type.
   *  @param  __b  Another thing of arbitrary type.
   *  @return A pair(__b, __a) if __b is smaller than __a, pair(__a,
   *  __b) otherwise.
  */
  template<typename _Tp>
    _GLIBCXX14_CONSTEXPR
    inline pair<const _Tp&, const _Tp&>
    minmax(const _Tp& __a, const _Tp& __b)
    {
      // concept requirements
      __glibcxx_function_requires(_LessThanComparableConcept<_Tp>)

      return __b < __a ? pair<const _Tp&, const _Tp&>(__b, __a)
		       : pair<const _Tp&, const _Tp&>(__a, __b);
    }

  /**
   *  @brief  Determines min and max at once as an ordered pair.
   *  @ingroup sorting_algorithms
   *  @param  __a  A thing of arbitrary type.
   *  @param  __b  Another thing of arbitrary type.
   *  @param  __comp  A @link comparison_functors comparison functor @endlink.
   *  @return A pair(__b, __a) if __b is smaller than __a, pair(__a,
   *  __b) otherwise.
  */
  template<typename _Tp, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    inline pair<const _Tp&, const _Tp&>
    minmax(const _Tp& __a, const _Tp& __b, _Compare __comp)
    {
      return __comp(__b, __a) ? pair<const _Tp&, const _Tp&>(__b, __a)
			      : pair<const _Tp&, const _Tp&>(__a, __b);
    }

  template<typename _ForwardIterator, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    pair<_ForwardIterator, _ForwardIterator>
    __minmax_element(_ForwardIterator __first, _ForwardIterator __last,
		     _Compare __comp)
    {
      _ForwardIterator __next = __first;
      if (__first == __last
	  || ++__next == __last)
	return std::make_pair(__first, __first);

      _ForwardIterator __min{}, __max{};
      if (__comp(__next, __first))
	{
	  __min = __next;
	  __max = __first;
	}
      else
	{
	  __min = __first;
	  __max = __next;
	}

      __first = __next;
      ++__first;

      while (__first != __last)
	{
	  __next = __first;
	  if (++__next == __last)
	    {
	      if (__comp(__first, __min))
		__min = __first;
	      else if (!__comp(__first, __max))
		__max = __first;
	      break;
	    }

	  if (__comp(__next, __first))
	    {
	      if (__comp(__next, __min))
		__min = __next;
	      if (!__comp(__first, __max))
		__max = __first;
	    }
	  else
	    {
	      if (__comp(__first, __min))
		__min = __first;
	      if (!__comp(__next, __max))
		__max = __next;
	    }

	  __first = __next;
	  ++__first;
	}

      return std::make_pair(__min, __max);
    }

  /**
   *  @brief  Return a pair of iterators pointing to the minimum and maximum
   *          elements in a range.
   *  @ingroup sorting_algorithms
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @return  make_pair(m, M), where m is the first iterator i in 
   *           [__first, __last) such that no other element in the range is
   *           smaller, and where M is the last iterator i in [__first, __last)
   *           such that no other element in the range is larger.
  */
  template<typename _ForwardIterator>
    _GLIBCXX14_CONSTEXPR
    inline pair<_ForwardIterator, _ForwardIterator>
    minmax_element(_ForwardIterator __first, _ForwardIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      return std::__minmax_element(__first, __last,
				   __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief  Return a pair of iterators pointing to the minimum and maximum
   *          elements in a range.
   *  @ingroup sorting_algorithms
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @param  __comp   Comparison functor.
   *  @return  make_pair(m, M), where m is the first iterator i in 
   *           [__first, __last) such that no other element in the range is
   *           smaller, and where M is the last iterator i in [__first, __last)
   *           such that no other element in the range is larger.
  */
  template<typename _ForwardIterator, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    inline pair<_ForwardIterator, _ForwardIterator>
    minmax_element(_ForwardIterator __first, _ForwardIterator __last,
		   _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_ForwardIterator>::value_type,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      return std::__minmax_element(__first, __last,
				   __gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  // N2722 + DR 915.
  template<typename _Tp>
    _GLIBCXX14_CONSTEXPR
    inline _Tp
    min(initializer_list<_Tp> __l)
    { return *std::min_element(__l.begin(), __l.end()); }

  template<typename _Tp, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    inline _Tp
    min(initializer_list<_Tp> __l, _Compare __comp)
    { return *std::min_element(__l.begin(), __l.end(), __comp); }

  template<typename _Tp>
    _GLIBCXX14_CONSTEXPR
    inline _Tp
    max(initializer_list<_Tp> __l)
    { return *std::max_element(__l.begin(), __l.end()); }

  template<typename _Tp, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    inline _Tp
    max(initializer_list<_Tp> __l, _Compare __comp)
    { return *std::max_element(__l.begin(), __l.end(), __comp); }

  template<typename _Tp>
    _GLIBCXX14_CONSTEXPR
    inline pair<_Tp, _Tp>
    minmax(initializer_list<_Tp> __l)
    {
      pair<const _Tp*, const _Tp*> __p =
	std::minmax_element(__l.begin(), __l.end());
      return std::make_pair(*__p.first, *__p.second);
    }

  template<typename _Tp, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    inline pair<_Tp, _Tp>
    minmax(initializer_list<_Tp> __l, _Compare __comp)
    {
      pair<const _Tp*, const _Tp*> __p =
	std::minmax_element(__l.begin(), __l.end(), __comp);
      return std::make_pair(*__p.first, *__p.second);
    }

  /**
   *  @brief  Checks whether a permutation of the second sequence is equal
   *          to the first sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first1  Start of first range.
   *  @param  __last1   End of first range.
   *  @param  __first2  Start of second range.
   *  @param  __pred    A binary predicate.
   *  @return true if there exists a permutation of the elements in
   *          the range [__first2, __first2 + (__last1 - __first1)),
   *          beginning with ForwardIterator2 begin, such that
   *          equal(__first1, __last1, __begin, __pred) returns true;
   *          otherwise, returns false.
  */
  template<typename _ForwardIterator1, typename _ForwardIterator2,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    is_permutation(_ForwardIterator1 __first1, _ForwardIterator1 __last1,
		   _ForwardIterator2 __first2, _BinaryPredicate __pred)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator1>)
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator2>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_BinaryPredicate,
	    typename iterator_traits<_ForwardIterator1>::value_type,
	    typename iterator_traits<_ForwardIterator2>::value_type>)
      __glibcxx_requires_valid_range(__first1, __last1);

      return std::__is_permutation(__first1, __last1, __first2,
				   __gnu_cxx::__ops::__iter_comp_iter(__pred));
    }

#if __cplusplus > 201103L
  template<typename _ForwardIterator1, typename _ForwardIterator2,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    bool
    __is_permutation(_ForwardIterator1 __first1, _ForwardIterator1 __last1,
		     _ForwardIterator2 __first2, _ForwardIterator2 __last2,
		     _BinaryPredicate __pred)
    {
      using _Cat1
	= typename iterator_traits<_ForwardIterator1>::iterator_category;
      using _Cat2
	= typename iterator_traits<_ForwardIterator2>::iterator_category;
      using _It1_is_RA = is_same<_Cat1, random_access_iterator_tag>;
      using _It2_is_RA = is_same<_Cat2, random_access_iterator_tag>;
      constexpr bool __ra_iters = _It1_is_RA() && _It2_is_RA();
      if (__ra_iters)
	{
	  auto __d1 = std::distance(__first1, __last1);
	  auto __d2 = std::distance(__first2, __last2);
	  if (__d1 != __d2)
	    return false;
	}

      // Efficiently compare identical prefixes:  O(N) if sequences
      // have the same elements in the same order.
      for (; __first1 != __last1 && __first2 != __last2;
	  ++__first1, (void)++__first2)
	if (!__pred(__first1, __first2))
	  break;

      if (__ra_iters)
	{
	  if (__first1 == __last1)
	    return true;
	}
      else
	{
	  auto __d1 = std::distance(__first1, __last1);
	  auto __d2 = std::distance(__first2, __last2);
	  if (__d1 == 0 && __d2 == 0)
	    return true;
	  if (__d1 != __d2)
	    return false;
	}

      for (_ForwardIterator1 __scan = __first1; __scan != __last1; ++__scan)
	{
	  if (__scan != std::__find_if(__first1, __scan,
			__gnu_cxx::__ops::__iter_comp_iter(__pred, __scan)))
	    continue; // We've seen this one before.

	  auto __matches = std::__count_if(__first2, __last2,
		__gnu_cxx::__ops::__iter_comp_iter(__pred, __scan));
	  if (0 == __matches
	      || std::__count_if(__scan, __last1,
			__gnu_cxx::__ops::__iter_comp_iter(__pred, __scan))
	      != __matches)
	    return false;
	}
      return true;
    }

  /**
   *  @brief  Checks whether a permutaion of the second sequence is equal
   *          to the first sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first1  Start of first range.
   *  @param  __last1   End of first range.
   *  @param  __first2  Start of second range.
   *  @param  __last2   End of first range.
   *  @return true if there exists a permutation of the elements in the range
   *          [__first2, __last2), beginning with ForwardIterator2 begin,
   *          such that equal(__first1, __last1, begin) returns true;
   *          otherwise, returns false.
  */
  template<typename _ForwardIterator1, typename _ForwardIterator2>
    _GLIBCXX20_CONSTEXPR
    inline bool
    is_permutation(_ForwardIterator1 __first1, _ForwardIterator1 __last1,
		   _ForwardIterator2 __first2, _ForwardIterator2 __last2)
    {
      __glibcxx_requires_valid_range(__first1, __last1);
      __glibcxx_requires_valid_range(__first2, __last2);

      return
	std::__is_permutation(__first1, __last1, __first2, __last2,
			      __gnu_cxx::__ops::__iter_equal_to_iter());
    }

  /**
   *  @brief  Checks whether a permutation of the second sequence is equal
   *          to the first sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first1  Start of first range.
   *  @param  __last1   End of first range.
   *  @param  __first2  Start of second range.
   *  @param  __last2   End of first range.
   *  @param  __pred    A binary predicate.
   *  @return true if there exists a permutation of the elements in the range
   *          [__first2, __last2), beginning with ForwardIterator2 begin,
   *          such that equal(__first1, __last1, __begin, __pred) returns true;
   *          otherwise, returns false.
  */
  template<typename _ForwardIterator1, typename _ForwardIterator2,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    is_permutation(_ForwardIterator1 __first1, _ForwardIterator1 __last1,
		   _ForwardIterator2 __first2, _ForwardIterator2 __last2,
		   _BinaryPredicate __pred)
    {
      __glibcxx_requires_valid_range(__first1, __last1);
      __glibcxx_requires_valid_range(__first2, __last2);

      return std::__is_permutation(__first1, __last1, __first2, __last2,
				   __gnu_cxx::__ops::__iter_comp_iter(__pred));
    }

#if __cplusplus > 201402L

#define __cpp_lib_clamp 201603

  /**
   *  @brief  Returns the value clamped between lo and hi.
   *  @ingroup sorting_algorithms
   *  @param  __val  A value of arbitrary type.
   *  @param  __lo   A lower limit of arbitrary type.
   *  @param  __hi   An upper limit of arbitrary type.
   *  @return max(__val, __lo) if __val < __hi or min(__val, __hi) otherwise.
   */
  template<typename _Tp>
    constexpr const _Tp&
    clamp(const _Tp& __val, const _Tp& __lo, const _Tp& __hi)
    {
      __glibcxx_assert(!(__hi < __lo));
      return (__val < __lo) ? __lo : (__hi < __val) ? __hi : __val;
    }

  /**
   *  @brief  Returns the value clamped between lo and hi.
   *  @ingroup sorting_algorithms
   *  @param  __val   A value of arbitrary type.
   *  @param  __lo    A lower limit of arbitrary type.
   *  @param  __hi    An upper limit of arbitrary type.
   *  @param  __comp  A comparison functor.
   *  @return max(__val, __lo, __comp) if __comp(__val, __hi)
   *	      or min(__val, __hi, __comp) otherwise.
   */
  template<typename _Tp, typename _Compare>
    constexpr const _Tp&
    clamp(const _Tp& __val, const _Tp& __lo, const _Tp& __hi, _Compare __comp)
    {
      __glibcxx_assert(!__comp(__hi, __lo));
      return __comp(__val, __lo) ? __lo : __comp(__hi, __val) ? __hi : __val;
    }
#endif // C++17
#endif // C++14

#ifdef _GLIBCXX_USE_C99_STDINT_TR1
  /**
   *  @brief Generate two uniformly distributed integers using a
   *         single distribution invocation.
   *  @param  __b0    The upper bound for the first integer.
   *  @param  __b1    The upper bound for the second integer.
   *  @param  __g     A UniformRandomBitGenerator.
   *  @return  A pair (i, j) with i and j uniformly distributed
   *           over [0, __b0) and [0, __b1), respectively.
   *
   *  Requires: __b0 * __b1 <= __g.max() - __g.min().
   *
   *  Using uniform_int_distribution with a range that is very
   *  small relative to the range of the generator ends up wasting
   *  potentially expensively generated randomness, since
   *  uniform_int_distribution does not store leftover randomness
   *  between invocations.
   *
   *  If we know we want two integers in ranges that are sufficiently
   *  small, we can compose the ranges, use a single distribution
   *  invocation, and significantly reduce the waste.
  */
  template<typename _IntType, typename _UniformRandomBitGenerator>
    pair<_IntType, _IntType>
    __gen_two_uniform_ints(_IntType __b0, _IntType __b1,
			   _UniformRandomBitGenerator&& __g)
    {
      _IntType __x
	= uniform_int_distribution<_IntType>{0, (__b0 * __b1) - 1}(__g);
      return std::make_pair(__x / __b1, __x % __b1);
    }

  /**
   *  @brief Shuffle the elements of a sequence using a uniform random
   *         number generator.
   *  @ingroup mutating_algorithms
   *  @param  __first   A forward iterator.
   *  @param  __last    A forward iterator.
   *  @param  __g       A UniformRandomNumberGenerator (26.5.1.3).
   *  @return  Nothing.
   *
   *  Reorders the elements in the range @p [__first,__last) using @p __g to
   *  provide random numbers.
  */
  template<typename _RandomAccessIterator,
	   typename _UniformRandomNumberGenerator>
    void
    shuffle(_RandomAccessIterator __first, _RandomAccessIterator __last,
	    _UniformRandomNumberGenerator&& __g)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_requires_valid_range(__first, __last);

      if (__first == __last)
	return;

      typedef typename iterator_traits<_RandomAccessIterator>::difference_type
	_DistanceType;

      typedef typename std::make_unsigned<_DistanceType>::type __ud_type;
      typedef typename std::uniform_int_distribution<__ud_type> __distr_type;
      typedef typename __distr_type::param_type __p_type;

      typedef typename remove_reference<_UniformRandomNumberGenerator>::type
	_Gen;
      typedef typename common_type<typename _Gen::result_type, __ud_type>::type
	__uc_type;

      const __uc_type __urngrange = __g.max() - __g.min();
      const __uc_type __urange = __uc_type(__last - __first);

      if (__urngrange / __urange >= __urange)
        // I.e. (__urngrange >= __urange * __urange) but without wrap issues.
      {
	_RandomAccessIterator __i = __first + 1;

	// Since we know the range isn't empty, an even number of elements
	// means an uneven number of elements /to swap/, in which case we
	// do the first one up front:

	if ((__urange % 2) == 0)
	{
	  __distr_type __d{0, 1};
	  std::iter_swap(__i++, __first + __d(__g));
	}

	// Now we know that __last - __i is even, so we do the rest in pairs,
	// using a single distribution invocation to produce swap positions
	// for two successive elements at a time:

	while (__i != __last)
	{
	  const __uc_type __swap_range = __uc_type(__i - __first) + 1;

	  const pair<__uc_type, __uc_type> __pospos =
	    __gen_two_uniform_ints(__swap_range, __swap_range + 1, __g);

	  std::iter_swap(__i++, __first + __pospos.first);
	  std::iter_swap(__i++, __first + __pospos.second);
	}

	return;
      }

      __distr_type __d;

      for (_RandomAccessIterator __i = __first + 1; __i != __last; ++__i)
	std::iter_swap(__i, __first + __d(__g, __p_type(0, __i - __first)));
    }
#endif

#endif // C++11

_GLIBCXX_BEGIN_NAMESPACE_ALGO

  /**
   *  @brief Apply a function to every element of a sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first  An input iterator.
   *  @param  __last   An input iterator.
   *  @param  __f      A unary function object.
   *  @return   @p __f
   *
   *  Applies the function object @p __f to each element in the range
   *  @p [first,last).  @p __f must not modify the order of the sequence.
   *  If @p __f has a return value it is ignored.
  */
  template<typename _InputIterator, typename _Function>
    _GLIBCXX20_CONSTEXPR
    _Function
    for_each(_InputIterator __first, _InputIterator __last, _Function __f)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_requires_valid_range(__first, __last);
      for (; __first != __last; ++__first)
	__f(*__first);
      return __f; // N.B. [alg.foreach] says std::move(f) but it's redundant.
    }

#if __cplusplus >= 201703L
  /**
   *  @brief Apply a function to every element of a sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first  An input iterator.
   *  @param  __n      A value convertible to an integer.
   *  @param  __f      A unary function object.
   *  @return   `__first+__n`
   *
   *  Applies the function object `__f` to each element in the range
   *  `[first, first+n)`.  `__f` must not modify the order of the sequence.
   *  If `__f` has a return value it is ignored.
  */
  template<typename _InputIterator, typename _Size, typename _Function>
    _GLIBCXX20_CONSTEXPR
    _InputIterator
    for_each_n(_InputIterator __first, _Size __n, _Function __f)
    {
      auto __n2 = std::__size_to_integer(__n);
      using _Cat = typename iterator_traits<_InputIterator>::iterator_category;
      if constexpr (is_base_of_v<random_access_iterator_tag, _Cat>)
	{
	  if (__n2 <= 0)
	    return __first;
	  auto __last = __first + __n2;
	  std::for_each(__first, __last, std::move(__f));
	  return __last;
	}
      else
	{
	  while (__n2-->0)
	    {
	      __f(*__first);
	      ++__first;
	    }
	  return __first;
	}
    }
#endif // C++17

  /**
   *  @brief Find the first occurrence of a value in a sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first  An input iterator.
   *  @param  __last   An input iterator.
   *  @param  __val    The value to find.
   *  @return   The first iterator @c i in the range @p [__first,__last)
   *  such that @c *i == @p __val, or @p __last if no such iterator exists.
  */
  template<typename _InputIterator, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline _InputIterator
    find(_InputIterator __first, _InputIterator __last,
	 const _Tp& __val)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_EqualOpConcept<
		typename iterator_traits<_InputIterator>::value_type, _Tp>)
      __glibcxx_requires_valid_range(__first, __last);
      return std::__find_if(__first, __last,
			    __gnu_cxx::__ops::__iter_equals_val(__val));
    }

  /**
   *  @brief Find the first element in a sequence for which a
   *         predicate is true.
   *  @ingroup non_mutating_algorithms
   *  @param  __first  An input iterator.
   *  @param  __last   An input iterator.
   *  @param  __pred   A predicate.
   *  @return   The first iterator @c i in the range @p [__first,__last)
   *  such that @p __pred(*i) is true, or @p __last if no such iterator exists.
  */
  template<typename _InputIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline _InputIterator
    find_if(_InputIterator __first, _InputIterator __last,
	    _Predicate __pred)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_UnaryPredicateConcept<_Predicate,
	      typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__find_if(__first, __last,
			    __gnu_cxx::__ops::__pred_iter(__pred));
    }

  /**
   *  @brief  Find element from a set in a sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first1  Start of range to search.
   *  @param  __last1   End of range to search.
   *  @param  __first2  Start of match candidates.
   *  @param  __last2   End of match candidates.
   *  @return   The first iterator @c i in the range
   *  @p [__first1,__last1) such that @c *i == @p *(i2) such that i2 is an
   *  iterator in [__first2,__last2), or @p __last1 if no such iterator exists.
   *
   *  Searches the range @p [__first1,__last1) for an element that is
   *  equal to some element in the range [__first2,__last2).  If
   *  found, returns an iterator in the range [__first1,__last1),
   *  otherwise returns @p __last1.
  */
  template<typename _InputIterator, typename _ForwardIterator>
    _GLIBCXX20_CONSTEXPR
    _InputIterator
    find_first_of(_InputIterator __first1, _InputIterator __last1,
		  _ForwardIterator __first2, _ForwardIterator __last2)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_EqualOpConcept<
	    typename iterator_traits<_InputIterator>::value_type,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first1, __last1);
      __glibcxx_requires_valid_range(__first2, __last2);

      for (; __first1 != __last1; ++__first1)
	for (_ForwardIterator __iter = __first2; __iter != __last2; ++__iter)
	  if (*__first1 == *__iter)
	    return __first1;
      return __last1;
    }

  /**
   *  @brief  Find element from a set in a sequence using a predicate.
   *  @ingroup non_mutating_algorithms
   *  @param  __first1  Start of range to search.
   *  @param  __last1   End of range to search.
   *  @param  __first2  Start of match candidates.
   *  @param  __last2   End of match candidates.
   *  @param  __comp    Predicate to use.
   *  @return   The first iterator @c i in the range
   *  @p [__first1,__last1) such that @c comp(*i, @p *(i2)) is true
   *  and i2 is an iterator in [__first2,__last2), or @p __last1 if no
   *  such iterator exists.
   *

   *  Searches the range @p [__first1,__last1) for an element that is
   *  equal to some element in the range [__first2,__last2).  If
   *  found, returns an iterator in the range [__first1,__last1),
   *  otherwise returns @p __last1.
  */
  template<typename _InputIterator, typename _ForwardIterator,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _InputIterator
    find_first_of(_InputIterator __first1, _InputIterator __last1,
		  _ForwardIterator __first2, _ForwardIterator __last2,
		  _BinaryPredicate __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_BinaryPredicate,
	    typename iterator_traits<_InputIterator>::value_type,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first1, __last1);
      __glibcxx_requires_valid_range(__first2, __last2);

      for (; __first1 != __last1; ++__first1)
	for (_ForwardIterator __iter = __first2; __iter != __last2; ++__iter)
	  if (__comp(*__first1, *__iter))
	    return __first1;
      return __last1;
    }

  /**
   *  @brief Find two adjacent values in a sequence that are equal.
   *  @ingroup non_mutating_algorithms
   *  @param  __first  A forward iterator.
   *  @param  __last   A forward iterator.
   *  @return   The first iterator @c i such that @c i and @c i+1 are both
   *  valid iterators in @p [__first,__last) and such that @c *i == @c *(i+1),
   *  or @p __last if no such iterator exists.
  */
  template<typename _ForwardIterator>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    adjacent_find(_ForwardIterator __first, _ForwardIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_EqualityComparableConcept<
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__adjacent_find(__first, __last,
				  __gnu_cxx::__ops::__iter_equal_to_iter());
    }

  /**
   *  @brief Find two adjacent values in a sequence using a predicate.
   *  @ingroup non_mutating_algorithms
   *  @param  __first         A forward iterator.
   *  @param  __last          A forward iterator.
   *  @param  __binary_pred   A binary predicate.
   *  @return   The first iterator @c i such that @c i and @c i+1 are both
   *  valid iterators in @p [__first,__last) and such that
   *  @p __binary_pred(*i,*(i+1)) is true, or @p __last if no such iterator
   *  exists.
  */
  template<typename _ForwardIterator, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    adjacent_find(_ForwardIterator __first, _ForwardIterator __last,
		  _BinaryPredicate __binary_pred)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_BinaryPredicate,
	    typename iterator_traits<_ForwardIterator>::value_type,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__adjacent_find(__first, __last,
			__gnu_cxx::__ops::__iter_comp_iter(__binary_pred));
    }

  /**
   *  @brief Count the number of copies of a value in a sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first  An input iterator.
   *  @param  __last   An input iterator.
   *  @param  __value  The value to be counted.
   *  @return   The number of iterators @c i in the range @p [__first,__last)
   *  for which @c *i == @p __value
  */
  template<typename _InputIterator, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline typename iterator_traits<_InputIterator>::difference_type
    count(_InputIterator __first, _InputIterator __last, const _Tp& __value)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_EqualOpConcept<
	    typename iterator_traits<_InputIterator>::value_type, _Tp>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__count_if(__first, __last,
			     __gnu_cxx::__ops::__iter_equals_val(__value));
    }

  /**
   *  @brief Count the elements of a sequence for which a predicate is true.
   *  @ingroup non_mutating_algorithms
   *  @param  __first  An input iterator.
   *  @param  __last   An input iterator.
   *  @param  __pred   A predicate.
   *  @return   The number of iterators @c i in the range @p [__first,__last)
   *  for which @p __pred(*i) is true.
  */
  template<typename _InputIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline typename iterator_traits<_InputIterator>::difference_type
    count_if(_InputIterator __first, _InputIterator __last, _Predicate __pred)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_UnaryPredicateConcept<_Predicate,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__count_if(__first, __last,
			     __gnu_cxx::__ops::__pred_iter(__pred));
    }

  /**
   *  @brief Search a sequence for a matching sub-sequence.
   *  @ingroup non_mutating_algorithms
   *  @param  __first1  A forward iterator.
   *  @param  __last1   A forward iterator.
   *  @param  __first2  A forward iterator.
   *  @param  __last2   A forward iterator.
   *  @return The first iterator @c i in the range @p
   *  [__first1,__last1-(__last2-__first2)) such that @c *(i+N) == @p
   *  *(__first2+N) for each @c N in the range @p
   *  [0,__last2-__first2), or @p __last1 if no such iterator exists.
   *
   *  Searches the range @p [__first1,__last1) for a sub-sequence that
   *  compares equal value-by-value with the sequence given by @p
   *  [__first2,__last2) and returns an iterator to the first element
   *  of the sub-sequence, or @p __last1 if the sub-sequence is not
   *  found.
   *
   *  Because the sub-sequence must lie completely within the range @p
   *  [__first1,__last1) it must start at a position less than @p
   *  __last1-(__last2-__first2) where @p __last2-__first2 is the
   *  length of the sub-sequence.
   *
   *  This means that the returned iterator @c i will be in the range
   *  @p [__first1,__last1-(__last2-__first2))
  */
  template<typename _ForwardIterator1, typename _ForwardIterator2>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator1
    search(_ForwardIterator1 __first1, _ForwardIterator1 __last1,
	   _ForwardIterator2 __first2, _ForwardIterator2 __last2)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator1>)
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator2>)
      __glibcxx_function_requires(_EqualOpConcept<
	    typename iterator_traits<_ForwardIterator1>::value_type,
	    typename iterator_traits<_ForwardIterator2>::value_type>)
      __glibcxx_requires_valid_range(__first1, __last1);
      __glibcxx_requires_valid_range(__first2, __last2);

      return std::__search(__first1, __last1, __first2, __last2,
			   __gnu_cxx::__ops::__iter_equal_to_iter());
    }

  /**
   *  @brief Search a sequence for a matching sub-sequence using a predicate.
   *  @ingroup non_mutating_algorithms
   *  @param  __first1     A forward iterator.
   *  @param  __last1      A forward iterator.
   *  @param  __first2     A forward iterator.
   *  @param  __last2      A forward iterator.
   *  @param  __predicate  A binary predicate.
   *  @return   The first iterator @c i in the range
   *  @p [__first1,__last1-(__last2-__first2)) such that
   *  @p __predicate(*(i+N),*(__first2+N)) is true for each @c N in the range
   *  @p [0,__last2-__first2), or @p __last1 if no such iterator exists.
   *
   *  Searches the range @p [__first1,__last1) for a sub-sequence that
   *  compares equal value-by-value with the sequence given by @p
   *  [__first2,__last2), using @p __predicate to determine equality,
   *  and returns an iterator to the first element of the
   *  sub-sequence, or @p __last1 if no such iterator exists.
   *
   *  @see search(_ForwardIter1, _ForwardIter1, _ForwardIter2, _ForwardIter2)
  */
  template<typename _ForwardIterator1, typename _ForwardIterator2,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator1
    search(_ForwardIterator1 __first1, _ForwardIterator1 __last1,
	   _ForwardIterator2 __first2, _ForwardIterator2 __last2,
	   _BinaryPredicate  __predicate)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator1>)
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator2>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_BinaryPredicate,
	    typename iterator_traits<_ForwardIterator1>::value_type,
	    typename iterator_traits<_ForwardIterator2>::value_type>)
      __glibcxx_requires_valid_range(__first1, __last1);
      __glibcxx_requires_valid_range(__first2, __last2);

      return std::__search(__first1, __last1, __first2, __last2,
			   __gnu_cxx::__ops::__iter_comp_iter(__predicate));
    }

  /**
   *  @brief Search a sequence for a number of consecutive values.
   *  @ingroup non_mutating_algorithms
   *  @param  __first  A forward iterator.
   *  @param  __last   A forward iterator.
   *  @param  __count  The number of consecutive values.
   *  @param  __val    The value to find.
   *  @return The first iterator @c i in the range @p
   *  [__first,__last-__count) such that @c *(i+N) == @p __val for
   *  each @c N in the range @p [0,__count), or @p __last if no such
   *  iterator exists.
   *
   *  Searches the range @p [__first,__last) for @p count consecutive elements
   *  equal to @p __val.
  */
  template<typename _ForwardIterator, typename _Integer, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    search_n(_ForwardIterator __first, _ForwardIterator __last,
	     _Integer __count, const _Tp& __val)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_EqualOpConcept<
	    typename iterator_traits<_ForwardIterator>::value_type, _Tp>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__search_n(__first, __last, __count,
			     __gnu_cxx::__ops::__iter_equals_val(__val));
    }


  /**
   *  @brief Search a sequence for a number of consecutive values using a
   *         predicate.
   *  @ingroup non_mutating_algorithms
   *  @param  __first        A forward iterator.
   *  @param  __last         A forward iterator.
   *  @param  __count        The number of consecutive values.
   *  @param  __val          The value to find.
   *  @param  __binary_pred  A binary predicate.
   *  @return The first iterator @c i in the range @p
   *  [__first,__last-__count) such that @p
   *  __binary_pred(*(i+N),__val) is true for each @c N in the range
   *  @p [0,__count), or @p __last if no such iterator exists.
   *
   *  Searches the range @p [__first,__last) for @p __count
   *  consecutive elements for which the predicate returns true.
  */
  template<typename _ForwardIterator, typename _Integer, typename _Tp,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    search_n(_ForwardIterator __first, _ForwardIterator __last,
	     _Integer __count, const _Tp& __val,
	     _BinaryPredicate __binary_pred)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_BinaryPredicate,
	    typename iterator_traits<_ForwardIterator>::value_type, _Tp>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__search_n(__first, __last, __count,
		__gnu_cxx::__ops::__iter_comp_val(__binary_pred, __val));
    }

#if __cplusplus > 201402L
  /** @brief Search a sequence using a Searcher object.
   *
   *  @param  __first        A forward iterator.
   *  @param  __last         A forward iterator.
   *  @param  __searcher     A callable object.
   *  @return @p __searcher(__first,__last).first
  */
  template<typename _ForwardIterator, typename _Searcher>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    search(_ForwardIterator __first, _ForwardIterator __last,
	   const _Searcher& __searcher)
    { return __searcher(__first, __last).first; }
#endif

  /**
   *  @brief Perform an operation on a sequence.
   *  @ingroup mutating_algorithms
   *  @param  __first     An input iterator.
   *  @param  __last      An input iterator.
   *  @param  __result    An output iterator.
   *  @param  __unary_op  A unary operator.
   *  @return   An output iterator equal to @p __result+(__last-__first).
   *
   *  Applies the operator to each element in the input range and assigns
   *  the results to successive elements of the output sequence.
   *  Evaluates @p *(__result+N)=unary_op(*(__first+N)) for each @c N in the
   *  range @p [0,__last-__first).
   *
   *  @p unary_op must not alter its argument.
  */
  template<typename _InputIterator, typename _OutputIterator,
	   typename _UnaryOperation>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    transform(_InputIterator __first, _InputIterator __last,
	      _OutputIterator __result, _UnaryOperation __unary_op)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    // "the type returned by a _UnaryOperation"
	    __typeof__(__unary_op(*__first))>)
      __glibcxx_requires_valid_range(__first, __last);

      for (; __first != __last; ++__first, (void)++__result)
	*__result = __unary_op(*__first);
      return __result;
    }

  /**
   *  @brief Perform an operation on corresponding elements of two sequences.
   *  @ingroup mutating_algorithms
   *  @param  __first1     An input iterator.
   *  @param  __last1      An input iterator.
   *  @param  __first2     An input iterator.
   *  @param  __result     An output iterator.
   *  @param  __binary_op  A binary operator.
   *  @return   An output iterator equal to @p result+(last-first).
   *
   *  Applies the operator to the corresponding elements in the two
   *  input ranges and assigns the results to successive elements of the
   *  output sequence.
   *  Evaluates @p
   *  *(__result+N)=__binary_op(*(__first1+N),*(__first2+N)) for each
   *  @c N in the range @p [0,__last1-__first1).
   *
   *  @p binary_op must not alter either of its arguments.
  */
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator, typename _BinaryOperation>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    transform(_InputIterator1 __first1, _InputIterator1 __last1,
	      _InputIterator2 __first2, _OutputIterator __result,
	      _BinaryOperation __binary_op)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    // "the type returned by a _BinaryOperation"
	    __typeof__(__binary_op(*__first1,*__first2))>)
      __glibcxx_requires_valid_range(__first1, __last1);

      for (; __first1 != __last1; ++__first1, (void)++__first2, ++__result)
	*__result = __binary_op(*__first1, *__first2);
      return __result;
    }

  /**
   *  @brief Replace each occurrence of one value in a sequence with another
   *         value.
   *  @ingroup mutating_algorithms
   *  @param  __first      A forward iterator.
   *  @param  __last       A forward iterator.
   *  @param  __old_value  The value to be replaced.
   *  @param  __new_value  The replacement value.
   *  @return   replace() returns no value.
   *
   *  For each iterator @c i in the range @p [__first,__last) if @c *i ==
   *  @p __old_value then the assignment @c *i = @p __new_value is performed.
  */
  template<typename _ForwardIterator, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    void
    replace(_ForwardIterator __first, _ForwardIterator __last,
	    const _Tp& __old_value, const _Tp& __new_value)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_ForwardIteratorConcept<
				  _ForwardIterator>)
      __glibcxx_function_requires(_EqualOpConcept<
	    typename iterator_traits<_ForwardIterator>::value_type, _Tp>)
      __glibcxx_function_requires(_ConvertibleConcept<_Tp,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      for (; __first != __last; ++__first)
	if (*__first == __old_value)
	  *__first = __new_value;
    }

  /**
   *  @brief Replace each value in a sequence for which a predicate returns
   *         true with another value.
   *  @ingroup mutating_algorithms
   *  @param  __first      A forward iterator.
   *  @param  __last       A forward iterator.
   *  @param  __pred       A predicate.
   *  @param  __new_value  The replacement value.
   *  @return   replace_if() returns no value.
   *
   *  For each iterator @c i in the range @p [__first,__last) if @p __pred(*i)
   *  is true then the assignment @c *i = @p __new_value is performed.
  */
  template<typename _ForwardIterator, typename _Predicate, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    void
    replace_if(_ForwardIterator __first, _ForwardIterator __last,
	       _Predicate __pred, const _Tp& __new_value)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_ForwardIteratorConcept<
				  _ForwardIterator>)
      __glibcxx_function_requires(_ConvertibleConcept<_Tp,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_function_requires(_UnaryPredicateConcept<_Predicate,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      for (; __first != __last; ++__first)
	if (__pred(*__first))
	  *__first = __new_value;
    }

  /**
   *  @brief Assign the result of a function object to each value in a
   *         sequence.
   *  @ingroup mutating_algorithms
   *  @param  __first  A forward iterator.
   *  @param  __last   A forward iterator.
   *  @param  __gen    A function object taking no arguments and returning
   *                 std::iterator_traits<_ForwardIterator>::value_type
   *  @return   generate() returns no value.
   *
   *  Performs the assignment @c *i = @p __gen() for each @c i in the range
   *  @p [__first,__last).
  */
  template<typename _ForwardIterator, typename _Generator>
    _GLIBCXX20_CONSTEXPR
    void
    generate(_ForwardIterator __first, _ForwardIterator __last,
	     _Generator __gen)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_GeneratorConcept<_Generator,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      for (; __first != __last; ++__first)
	*__first = __gen();
    }

  /**
   *  @brief Assign the result of a function object to each value in a
   *         sequence.
   *  @ingroup mutating_algorithms
   *  @param  __first  A forward iterator.
   *  @param  __n      The length of the sequence.
   *  @param  __gen    A function object taking no arguments and returning
   *                 std::iterator_traits<_ForwardIterator>::value_type
   *  @return   The end of the sequence, @p __first+__n
   *
   *  Performs the assignment @c *i = @p __gen() for each @c i in the range
   *  @p [__first,__first+__n).
   *
   * If @p __n is negative, the function does nothing and returns @p __first.
  */
  // _GLIBCXX_RESOLVE_LIB_DEFECTS
  // DR 865. More algorithms that throw away information
  // DR 426. search_n(), fill_n(), and generate_n() with negative n
  template<typename _OutputIterator, typename _Size, typename _Generator>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    generate_n(_OutputIterator __first, _Size __n, _Generator __gen)
    {
      // concept requirements
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    // "the type returned by a _Generator"
	    __typeof__(__gen())>)

      typedef __decltype(std::__size_to_integer(__n)) _IntSize;
      for (_IntSize __niter = std::__size_to_integer(__n);
	   __niter > 0; --__niter, (void) ++__first)
	*__first = __gen();
      return __first;
    }

  /**
   *  @brief Copy a sequence, removing consecutive duplicate values.
   *  @ingroup mutating_algorithms
   *  @param  __first   An input iterator.
   *  @param  __last    An input iterator.
   *  @param  __result  An output iterator.
   *  @return   An iterator designating the end of the resulting sequence.
   *
   *  Copies each element in the range @p [__first,__last) to the range
   *  beginning at @p __result, except that only the first element is copied
   *  from groups of consecutive elements that compare equal.
   *  unique_copy() is stable, so the relative order of elements that are
   *  copied is unchanged.
   *
   *  _GLIBCXX_RESOLVE_LIB_DEFECTS
   *  DR 241. Does unique_copy() require CopyConstructible and Assignable?
   *  
   *  _GLIBCXX_RESOLVE_LIB_DEFECTS
   *  DR 538. 241 again: Does unique_copy() require CopyConstructible and 
   *  Assignable?
  */
  template<typename _InputIterator, typename _OutputIterator>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    unique_copy(_InputIterator __first, _InputIterator __last,
		_OutputIterator __result)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_function_requires(_EqualityComparableConcept<
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      if (__first == __last)
	return __result;
      return std::__unique_copy(__first, __last, __result,
				__gnu_cxx::__ops::__iter_equal_to_iter(),
				std::__iterator_category(__first),
				std::__iterator_category(__result));
    }

  /**
   *  @brief Copy a sequence, removing consecutive values using a predicate.
   *  @ingroup mutating_algorithms
   *  @param  __first        An input iterator.
   *  @param  __last         An input iterator.
   *  @param  __result       An output iterator.
   *  @param  __binary_pred  A binary predicate.
   *  @return   An iterator designating the end of the resulting sequence.
   *
   *  Copies each element in the range @p [__first,__last) to the range
   *  beginning at @p __result, except that only the first element is copied
   *  from groups of consecutive elements for which @p __binary_pred returns
   *  true.
   *  unique_copy() is stable, so the relative order of elements that are
   *  copied is unchanged.
   *
   *  _GLIBCXX_RESOLVE_LIB_DEFECTS
   *  DR 241. Does unique_copy() require CopyConstructible and Assignable?
  */
  template<typename _InputIterator, typename _OutputIterator,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    unique_copy(_InputIterator __first, _InputIterator __last,
		_OutputIterator __result,
		_BinaryPredicate __binary_pred)
    {
      // concept requirements -- predicates checked later
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      if (__first == __last)
	return __result;
      return std::__unique_copy(__first, __last, __result,
			__gnu_cxx::__ops::__iter_comp_iter(__binary_pred),
				std::__iterator_category(__first),
				std::__iterator_category(__result));
    }

#if _GLIBCXX_HOSTED
  /**
   *  @brief Randomly shuffle the elements of a sequence.
   *  @ingroup mutating_algorithms
   *  @param  __first   A forward iterator.
   *  @param  __last    A forward iterator.
   *  @return  Nothing.
   *
   *  Reorder the elements in the range @p [__first,__last) using a random
   *  distribution, so that every possible ordering of the sequence is
   *  equally likely.
  */
  template<typename _RandomAccessIterator>
    inline void
    random_shuffle(_RandomAccessIterator __first, _RandomAccessIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_requires_valid_range(__first, __last);

      if (__first != __last)
	for (_RandomAccessIterator __i = __first + 1; __i != __last; ++__i)
	  {
	    // XXX rand() % N is not uniformly distributed
	    _RandomAccessIterator __j = __first
					+ std::rand() % ((__i - __first) + 1);
	    if (__i != __j)
	      std::iter_swap(__i, __j);
	  }
    }
#endif

  /**
   *  @brief Shuffle the elements of a sequence using a random number
   *         generator.
   *  @ingroup mutating_algorithms
   *  @param  __first   A forward iterator.
   *  @param  __last    A forward iterator.
   *  @param  __rand    The RNG functor or function.
   *  @return  Nothing.
   *
   *  Reorders the elements in the range @p [__first,__last) using @p __rand to
   *  provide a random distribution. Calling @p __rand(N) for a positive
   *  integer @p N should return a randomly chosen integer from the
   *  range [0,N).
  */
  template<typename _RandomAccessIterator, typename _RandomNumberGenerator>
    void
    random_shuffle(_RandomAccessIterator __first, _RandomAccessIterator __last,
#if __cplusplus >= 201103L
		   _RandomNumberGenerator&& __rand)
#else
		   _RandomNumberGenerator& __rand)
#endif
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_requires_valid_range(__first, __last);

      if (__first == __last)
	return;
      for (_RandomAccessIterator __i = __first + 1; __i != __last; ++__i)
	{
	  _RandomAccessIterator __j = __first + __rand((__i - __first) + 1);
	  if (__i != __j)
	    std::iter_swap(__i, __j);
	}
    }


  /**
   *  @brief Move elements for which a predicate is true to the beginning
   *         of a sequence.
   *  @ingroup mutating_algorithms
   *  @param  __first   A forward iterator.
   *  @param  __last    A forward iterator.
   *  @param  __pred    A predicate functor.
   *  @return  An iterator @p middle such that @p __pred(i) is true for each
   *  iterator @p i in the range @p [__first,middle) and false for each @p i
   *  in the range @p [middle,__last).
   *
   *  @p __pred must not modify its operand. @p partition() does not preserve
   *  the relative ordering of elements in each group, use
   *  @p stable_partition() if this is needed.
  */
  template<typename _ForwardIterator, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline _ForwardIterator
    partition(_ForwardIterator __first, _ForwardIterator __last,
	      _Predicate   __pred)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_ForwardIteratorConcept<
				  _ForwardIterator>)
      __glibcxx_function_requires(_UnaryPredicateConcept<_Predicate,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);

      return std::__partition(__first, __last, __pred,
			      std::__iterator_category(__first));
    }


  /**
   *  @brief Sort the smallest elements of a sequence.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __middle  Another iterator.
   *  @param  __last    Another iterator.
   *  @return  Nothing.
   *
   *  Sorts the smallest @p (__middle-__first) elements in the range
   *  @p [first,last) and moves them to the range @p [__first,__middle). The
   *  order of the remaining elements in the range @p [__middle,__last) is
   *  undefined.
   *  After the sort if @e i and @e j are iterators in the range
   *  @p [__first,__middle) such that i precedes j and @e k is an iterator in
   *  the range @p [__middle,__last) then *j<*i and *k<*i are both false.
  */
  template<typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    inline void
    partial_sort(_RandomAccessIterator __first,
		 _RandomAccessIterator __middle,
		 _RandomAccessIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_RandomAccessIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __middle);
      __glibcxx_requires_valid_range(__middle, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      std::__partial_sort(__first, __middle, __last,
			  __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief Sort the smallest elements of a sequence using a predicate
   *         for comparison.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __middle  Another iterator.
   *  @param  __last    Another iterator.
   *  @param  __comp    A comparison functor.
   *  @return  Nothing.
   *
   *  Sorts the smallest @p (__middle-__first) elements in the range
   *  @p [__first,__last) and moves them to the range @p [__first,__middle). The
   *  order of the remaining elements in the range @p [__middle,__last) is
   *  undefined.
   *  After the sort if @e i and @e j are iterators in the range
   *  @p [__first,__middle) such that i precedes j and @e k is an iterator in
   *  the range @p [__middle,__last) then @p *__comp(j,*i) and @p __comp(*k,*i)
   *  are both false.
  */
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline void
    partial_sort(_RandomAccessIterator __first,
		 _RandomAccessIterator __middle,
		 _RandomAccessIterator __last,
		 _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_RandomAccessIterator>::value_type,
	    typename iterator_traits<_RandomAccessIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __middle);
      __glibcxx_requires_valid_range(__middle, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      std::__partial_sort(__first, __middle, __last,
			  __gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  /**
   *  @brief Sort a sequence just enough to find a particular position.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __nth     Another iterator.
   *  @param  __last    Another iterator.
   *  @return  Nothing.
   *
   *  Rearranges the elements in the range @p [__first,__last) so that @p *__nth
   *  is the same element that would have been in that position had the
   *  whole sequence been sorted. The elements either side of @p *__nth are
   *  not completely sorted, but for any iterator @e i in the range
   *  @p [__first,__nth) and any iterator @e j in the range @p [__nth,__last) it
   *  holds that *j < *i is false.
  */
  template<typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    inline void
    nth_element(_RandomAccessIterator __first, _RandomAccessIterator __nth,
		_RandomAccessIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
				  _RandomAccessIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_RandomAccessIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __nth);
      __glibcxx_requires_valid_range(__nth, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      if (__first == __last || __nth == __last)
	return;

      std::__introselect(__first, __nth, __last,
			 std::__lg(__last - __first) * 2,
			 __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief Sort a sequence just enough to find a particular position
   *         using a predicate for comparison.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __nth     Another iterator.
   *  @param  __last    Another iterator.
   *  @param  __comp    A comparison functor.
   *  @return  Nothing.
   *
   *  Rearranges the elements in the range @p [__first,__last) so that @p *__nth
   *  is the same element that would have been in that position had the
   *  whole sequence been sorted. The elements either side of @p *__nth are
   *  not completely sorted, but for any iterator @e i in the range
   *  @p [__first,__nth) and any iterator @e j in the range @p [__nth,__last) it
   *  holds that @p __comp(*j,*i) is false.
  */
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline void
    nth_element(_RandomAccessIterator __first, _RandomAccessIterator __nth,
		_RandomAccessIterator __last, _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
				  _RandomAccessIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_RandomAccessIterator>::value_type,
	    typename iterator_traits<_RandomAccessIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __nth);
      __glibcxx_requires_valid_range(__nth, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      if (__first == __last || __nth == __last)
	return;

      std::__introselect(__first, __nth, __last,
			 std::__lg(__last - __first) * 2,
			 __gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  /**
   *  @brief Sort the elements of a sequence.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @return  Nothing.
   *
   *  Sorts the elements in the range @p [__first,__last) in ascending order,
   *  such that for each iterator @e i in the range @p [__first,__last-1),  
   *  *(i+1)<*i is false.
   *
   *  The relative ordering of equivalent elements is not preserved, use
   *  @p stable_sort() if this is needed.
  */
  template<typename _RandomAccessIterator>
    _GLIBCXX20_CONSTEXPR
    inline void
    sort(_RandomAccessIterator __first, _RandomAccessIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_RandomAccessIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      std::__sort(__first, __last, __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief Sort the elements of a sequence using a predicate for comparison.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __comp    A comparison functor.
   *  @return  Nothing.
   *
   *  Sorts the elements in the range @p [__first,__last) in ascending order,
   *  such that @p __comp(*(i+1),*i) is false for every iterator @e i in the
   *  range @p [__first,__last-1).
   *
   *  The relative ordering of equivalent elements is not preserved, use
   *  @p stable_sort() if this is needed.
  */
  template<typename _RandomAccessIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline void
    sort(_RandomAccessIterator __first, _RandomAccessIterator __last,
	 _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_RandomAccessIterator>::value_type,
	    typename iterator_traits<_RandomAccessIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      std::__sort(__first, __last, __gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    __merge(_InputIterator1 __first1, _InputIterator1 __last1,
	    _InputIterator2 __first2, _InputIterator2 __last2,
	    _OutputIterator __result, _Compare __comp)
    {
      while (__first1 != __last1 && __first2 != __last2)
	{
	  if (__comp(__first2, __first1))
	    {
	      *__result = *__first2;
	      ++__first2;
	    }
	  else
	    {
	      *__result = *__first1;
	      ++__first1;
	    }
	  ++__result;
	}
      return std::copy(__first2, __last2,
		       std::copy(__first1, __last1, __result));
    }

  /**
   *  @brief Merges two sorted ranges.
   *  @ingroup sorting_algorithms
   *  @param  __first1  An iterator.
   *  @param  __first2  Another iterator.
   *  @param  __last1   Another iterator.
   *  @param  __last2   Another iterator.
   *  @param  __result  An iterator pointing to the end of the merged range.
   *  @return   An output iterator equal to @p __result + (__last1 - __first1)
   *            + (__last2 - __first2).
   *
   *  Merges the ranges @p [__first1,__last1) and @p [__first2,__last2) into
   *  the sorted range @p [__result, __result + (__last1-__first1) +
   *  (__last2-__first2)).  Both input ranges must be sorted, and the
   *  output range must not overlap with either of the input ranges.
   *  The sort is @e stable, that is, for equivalent elements in the
   *  two ranges, elements from the first range will always come
   *  before elements from the second.
  */
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    merge(_InputIterator1 __first1, _InputIterator1 __last1,
	  _InputIterator2 __first2, _InputIterator2 __last2,
	  _OutputIterator __result)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_LessThanOpConcept<
	    typename iterator_traits<_InputIterator2>::value_type,
	    typename iterator_traits<_InputIterator1>::value_type>)	
      __glibcxx_requires_sorted_set(__first1, __last1, __first2);
      __glibcxx_requires_sorted_set(__first2, __last2, __first1);
      __glibcxx_requires_irreflexive2(__first1, __last1);
      __glibcxx_requires_irreflexive2(__first2, __last2);

      return _GLIBCXX_STD_A::__merge(__first1, __last1,
				     __first2, __last2, __result,
				     __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief Merges two sorted ranges.
   *  @ingroup sorting_algorithms
   *  @param  __first1  An iterator.
   *  @param  __first2  Another iterator.
   *  @param  __last1   Another iterator.
   *  @param  __last2   Another iterator.
   *  @param  __result  An iterator pointing to the end of the merged range.
   *  @param  __comp    A functor to use for comparisons.
   *  @return   An output iterator equal to @p __result + (__last1 - __first1)
   *            + (__last2 - __first2).
   *
   *  Merges the ranges @p [__first1,__last1) and @p [__first2,__last2) into
   *  the sorted range @p [__result, __result + (__last1-__first1) +
   *  (__last2-__first2)).  Both input ranges must be sorted, and the
   *  output range must not overlap with either of the input ranges.
   *  The sort is @e stable, that is, for equivalent elements in the
   *  two ranges, elements from the first range will always come
   *  before elements from the second.
   *
   *  The comparison function should have the same effects on ordering as
   *  the function used for the initial sort.
  */
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    merge(_InputIterator1 __first1, _InputIterator1 __last1,
	  _InputIterator2 __first2, _InputIterator2 __last2,
	  _OutputIterator __result, _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_InputIterator2>::value_type,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_requires_sorted_set_pred(__first1, __last1, __first2, __comp);
      __glibcxx_requires_sorted_set_pred(__first2, __last2, __first1, __comp);
      __glibcxx_requires_irreflexive_pred2(__first1, __last1, __comp);
      __glibcxx_requires_irreflexive_pred2(__first2, __last2, __comp);

      return _GLIBCXX_STD_A::__merge(__first1, __last1,
				__first2, __last2, __result,
				__gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  template<typename _RandomAccessIterator, typename _Compare>
    inline void
    __stable_sort(_RandomAccessIterator __first, _RandomAccessIterator __last,
		  _Compare __comp)
    {
      typedef typename iterator_traits<_RandomAccessIterator>::value_type
	_ValueType;
      typedef typename iterator_traits<_RandomAccessIterator>::difference_type
	_DistanceType;

      typedef _Temporary_buffer<_RandomAccessIterator, _ValueType> _TmpBuf;
      _TmpBuf __buf(__first, std::distance(__first, __last));

      if (__buf.begin() == 0)
	std::__inplace_stable_sort(__first, __last, __comp);
      else
	std::__stable_sort_adaptive(__first, __last, __buf.begin(),
				    _DistanceType(__buf.size()), __comp);
    }

  /**
   *  @brief Sort the elements of a sequence, preserving the relative order
   *         of equivalent elements.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @return  Nothing.
   *
   *  Sorts the elements in the range @p [__first,__last) in ascending order,
   *  such that for each iterator @p i in the range @p [__first,__last-1),
   *  @p *(i+1)<*i is false.
   *
   *  The relative ordering of equivalent elements is preserved, so any two
   *  elements @p x and @p y in the range @p [__first,__last) such that
   *  @p x<y is false and @p y<x is false will have the same relative
   *  ordering after calling @p stable_sort().
  */
  template<typename _RandomAccessIterator>
    inline void
    stable_sort(_RandomAccessIterator __first, _RandomAccessIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_RandomAccessIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      _GLIBCXX_STD_A::__stable_sort(__first, __last,
				    __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief Sort the elements of a sequence using a predicate for comparison,
   *         preserving the relative order of equivalent elements.
   *  @ingroup sorting_algorithms
   *  @param  __first   An iterator.
   *  @param  __last    Another iterator.
   *  @param  __comp    A comparison functor.
   *  @return  Nothing.
   *
   *  Sorts the elements in the range @p [__first,__last) in ascending order,
   *  such that for each iterator @p i in the range @p [__first,__last-1),
   *  @p __comp(*(i+1),*i) is false.
   *
   *  The relative ordering of equivalent elements is preserved, so any two
   *  elements @p x and @p y in the range @p [__first,__last) such that
   *  @p __comp(x,y) is false and @p __comp(y,x) is false will have the same
   *  relative ordering after calling @p stable_sort().
  */
  template<typename _RandomAccessIterator, typename _Compare>
    inline void
    stable_sort(_RandomAccessIterator __first, _RandomAccessIterator __last,
		_Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_Mutable_RandomAccessIteratorConcept<
	    _RandomAccessIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_RandomAccessIterator>::value_type,
	    typename iterator_traits<_RandomAccessIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      _GLIBCXX_STD_A::__stable_sort(__first, __last,
				    __gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    __set_union(_InputIterator1 __first1, _InputIterator1 __last1,
		_InputIterator2 __first2, _InputIterator2 __last2,
		_OutputIterator __result, _Compare __comp)
    {
      while (__first1 != __last1 && __first2 != __last2)
	{
	  if (__comp(__first1, __first2))
	    {
	      *__result = *__first1;
	      ++__first1;
	    }
	  else if (__comp(__first2, __first1))
	    {
	      *__result = *__first2;
	      ++__first2;
	    }
	  else
	    {
	      *__result = *__first1;
	      ++__first1;
	      ++__first2;
	    }
	  ++__result;
	}
      return std::copy(__first2, __last2,
		       std::copy(__first1, __last1, __result));
    }

  /**
   *  @brief Return the union of two sorted ranges.
   *  @ingroup set_algorithms
   *  @param  __first1  Start of first range.
   *  @param  __last1   End of first range.
   *  @param  __first2  Start of second range.
   *  @param  __last2   End of second range.
   *  @param  __result  Start of output range.
   *  @return  End of the output range.
   *  @ingroup set_algorithms
   *
   *  This operation iterates over both ranges, copying elements present in
   *  each range in order to the output range.  Iterators increment for each
   *  range.  When the current element of one range is less than the other,
   *  that element is copied and the iterator advanced.  If an element is
   *  contained in both ranges, the element from the first range is copied and
   *  both ranges advance.  The output range may not overlap either input
   *  range.
  */
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    set_union(_InputIterator1 __first1, _InputIterator1 __last1,
	      _InputIterator2 __first2, _InputIterator2 __last2,
	      _OutputIterator __result)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_LessThanOpConcept<
	    typename iterator_traits<_InputIterator1>::value_type,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_LessThanOpConcept<
	    typename iterator_traits<_InputIterator2>::value_type,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_requires_sorted_set(__first1, __last1, __first2);
      __glibcxx_requires_sorted_set(__first2, __last2, __first1);
      __glibcxx_requires_irreflexive2(__first1, __last1);
      __glibcxx_requires_irreflexive2(__first2, __last2);

      return _GLIBCXX_STD_A::__set_union(__first1, __last1,
				__first2, __last2, __result,
				__gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief Return the union of two sorted ranges using a comparison functor.
   *  @ingroup set_algorithms
   *  @param  __first1  Start of first range.
   *  @param  __last1   End of first range.
   *  @param  __first2  Start of second range.
   *  @param  __last2   End of second range.
   *  @param  __result  Start of output range.
   *  @param  __comp    The comparison functor.
   *  @return  End of the output range.
   *  @ingroup set_algorithms
   *
   *  This operation iterates over both ranges, copying elements present in
   *  each range in order to the output range.  Iterators increment for each
   *  range.  When the current element of one range is less than the other
   *  according to @p __comp, that element is copied and the iterator advanced.
   *  If an equivalent element according to @p __comp is contained in both
   *  ranges, the element from the first range is copied and both ranges
   *  advance.  The output range may not overlap either input range.
  */
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    set_union(_InputIterator1 __first1, _InputIterator1 __last1,
	      _InputIterator2 __first2, _InputIterator2 __last2,
	      _OutputIterator __result, _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_InputIterator1>::value_type,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_InputIterator2>::value_type,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_requires_sorted_set_pred(__first1, __last1, __first2, __comp);
      __glibcxx_requires_sorted_set_pred(__first2, __last2, __first1, __comp);
      __glibcxx_requires_irreflexive_pred2(__first1, __last1, __comp);
      __glibcxx_requires_irreflexive_pred2(__first2, __last2, __comp);

      return _GLIBCXX_STD_A::__set_union(__first1, __last1,
				__first2, __last2, __result,
				__gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    __set_intersection(_InputIterator1 __first1, _InputIterator1 __last1,
		       _InputIterator2 __first2, _InputIterator2 __last2,
		       _OutputIterator __result, _Compare __comp)
    {
      while (__first1 != __last1 && __first2 != __last2)
	if (__comp(__first1, __first2))
	  ++__first1;
	else if (__comp(__first2, __first1))
	  ++__first2;
	else
	  {
	    *__result = *__first1;
	    ++__first1;
	    ++__first2;
	    ++__result;
	  }
      return __result;
    }

  /**
   *  @brief Return the intersection of two sorted ranges.
   *  @ingroup set_algorithms
   *  @param  __first1  Start of first range.
   *  @param  __last1   End of first range.
   *  @param  __first2  Start of second range.
   *  @param  __last2   End of second range.
   *  @param  __result  Start of output range.
   *  @return  End of the output range.
   *  @ingroup set_algorithms
   *
   *  This operation iterates over both ranges, copying elements present in
   *  both ranges in order to the output range.  Iterators increment for each
   *  range.  When the current element of one range is less than the other,
   *  that iterator advances.  If an element is contained in both ranges, the
   *  element from the first range is copied and both ranges advance.  The
   *  output range may not overlap either input range.
  */
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    set_intersection(_InputIterator1 __first1, _InputIterator1 __last1,
		     _InputIterator2 __first2, _InputIterator2 __last2,
		     _OutputIterator __result)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_function_requires(_LessThanOpConcept<
	    typename iterator_traits<_InputIterator1>::value_type,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_LessThanOpConcept<
	    typename iterator_traits<_InputIterator2>::value_type,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_requires_sorted_set(__first1, __last1, __first2);
      __glibcxx_requires_sorted_set(__first2, __last2, __first1);
      __glibcxx_requires_irreflexive2(__first1, __last1);
      __glibcxx_requires_irreflexive2(__first2, __last2);

      return _GLIBCXX_STD_A::__set_intersection(__first1, __last1,
				     __first2, __last2, __result,
				     __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief Return the intersection of two sorted ranges using comparison
   *  functor.
   *  @ingroup set_algorithms
   *  @param  __first1  Start of first range.
   *  @param  __last1   End of first range.
   *  @param  __first2  Start of second range.
   *  @param  __last2   End of second range.
   *  @param  __result  Start of output range.
   *  @param  __comp    The comparison functor.
   *  @return  End of the output range.
   *  @ingroup set_algorithms
   *
   *  This operation iterates over both ranges, copying elements present in
   *  both ranges in order to the output range.  Iterators increment for each
   *  range.  When the current element of one range is less than the other
   *  according to @p __comp, that iterator advances.  If an element is
   *  contained in both ranges according to @p __comp, the element from the
   *  first range is copied and both ranges advance.  The output range may not
   *  overlap either input range.
  */
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    set_intersection(_InputIterator1 __first1, _InputIterator1 __last1,
		     _InputIterator2 __first2, _InputIterator2 __last2,
		     _OutputIterator __result, _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_InputIterator1>::value_type,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_InputIterator2>::value_type,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_requires_sorted_set_pred(__first1, __last1, __first2, __comp);
      __glibcxx_requires_sorted_set_pred(__first2, __last2, __first1, __comp);
      __glibcxx_requires_irreflexive_pred2(__first1, __last1, __comp);
      __glibcxx_requires_irreflexive_pred2(__first2, __last2, __comp);

      return _GLIBCXX_STD_A::__set_intersection(__first1, __last1,
				__first2, __last2, __result,
				__gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    __set_difference(_InputIterator1 __first1, _InputIterator1 __last1,
		     _InputIterator2 __first2, _InputIterator2 __last2,
		     _OutputIterator __result, _Compare __comp)
    {
      while (__first1 != __last1 && __first2 != __last2)
	if (__comp(__first1, __first2))
	  {
	    *__result = *__first1;
	    ++__first1;
	    ++__result;
	  }
	else if (__comp(__first2, __first1))
	  ++__first2;
	else
	  {
	    ++__first1;
	    ++__first2;
	  }
      return std::copy(__first1, __last1, __result);
    }

  /**
   *  @brief Return the difference of two sorted ranges.
   *  @ingroup set_algorithms
   *  @param  __first1  Start of first range.
   *  @param  __last1   End of first range.
   *  @param  __first2  Start of second range.
   *  @param  __last2   End of second range.
   *  @param  __result  Start of output range.
   *  @return  End of the output range.
   *  @ingroup set_algorithms
   *
   *  This operation iterates over both ranges, copying elements present in
   *  the first range but not the second in order to the output range.
   *  Iterators increment for each range.  When the current element of the
   *  first range is less than the second, that element is copied and the
   *  iterator advances.  If the current element of the second range is less,
   *  the iterator advances, but no element is copied.  If an element is
   *  contained in both ranges, no elements are copied and both ranges
   *  advance.  The output range may not overlap either input range.
  */
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    set_difference(_InputIterator1 __first1, _InputIterator1 __last1,
		   _InputIterator2 __first2, _InputIterator2 __last2,
		   _OutputIterator __result)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_function_requires(_LessThanOpConcept<
	    typename iterator_traits<_InputIterator1>::value_type,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_LessThanOpConcept<
	    typename iterator_traits<_InputIterator2>::value_type,
	    typename iterator_traits<_InputIterator1>::value_type>)	
      __glibcxx_requires_sorted_set(__first1, __last1, __first2);
      __glibcxx_requires_sorted_set(__first2, __last2, __first1);
      __glibcxx_requires_irreflexive2(__first1, __last1);
      __glibcxx_requires_irreflexive2(__first2, __last2);

      return _GLIBCXX_STD_A::__set_difference(__first1, __last1,
				   __first2, __last2, __result,
				   __gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief  Return the difference of two sorted ranges using comparison
   *  functor.
   *  @ingroup set_algorithms
   *  @param  __first1  Start of first range.
   *  @param  __last1   End of first range.
   *  @param  __first2  Start of second range.
   *  @param  __last2   End of second range.
   *  @param  __result  Start of output range.
   *  @param  __comp    The comparison functor.
   *  @return  End of the output range.
   *  @ingroup set_algorithms
   *
   *  This operation iterates over both ranges, copying elements present in
   *  the first range but not the second in order to the output range.
   *  Iterators increment for each range.  When the current element of the
   *  first range is less than the second according to @p __comp, that element
   *  is copied and the iterator advances.  If the current element of the
   *  second range is less, no element is copied and the iterator advances.
   *  If an element is contained in both ranges according to @p __comp, no
   *  elements are copied and both ranges advance.  The output range may not
   *  overlap either input range.
  */
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    set_difference(_InputIterator1 __first1, _InputIterator1 __last1,
		   _InputIterator2 __first2, _InputIterator2 __last2,
		   _OutputIterator __result, _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_InputIterator1>::value_type,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_InputIterator2>::value_type,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_requires_sorted_set_pred(__first1, __last1, __first2, __comp);
      __glibcxx_requires_sorted_set_pred(__first2, __last2, __first1, __comp);
      __glibcxx_requires_irreflexive_pred2(__first1, __last1, __comp);
      __glibcxx_requires_irreflexive_pred2(__first2, __last2, __comp);

      return _GLIBCXX_STD_A::__set_difference(__first1, __last1,
				   __first2, __last2, __result,
				   __gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _OutputIterator
    __set_symmetric_difference(_InputIterator1 __first1,
			       _InputIterator1 __last1,
			       _InputIterator2 __first2,
			       _InputIterator2 __last2,
			       _OutputIterator __result,
			       _Compare __comp)
    {
      while (__first1 != __last1 && __first2 != __last2)
	if (__comp(__first1, __first2))
	  {
	    *__result = *__first1;
	    ++__first1;
	    ++__result;
	  }
	else if (__comp(__first2, __first1))
	  {
	    *__result = *__first2;
	    ++__first2;
	    ++__result;
	  }
	else
	  {
	    ++__first1;
	    ++__first2;
	  }
      return std::copy(__first2, __last2, 
		       std::copy(__first1, __last1, __result));
    }

  /**
   *  @brief  Return the symmetric difference of two sorted ranges.
   *  @ingroup set_algorithms
   *  @param  __first1  Start of first range.
   *  @param  __last1   End of first range.
   *  @param  __first2  Start of second range.
   *  @param  __last2   End of second range.
   *  @param  __result  Start of output range.
   *  @return  End of the output range.
   *  @ingroup set_algorithms
   *
   *  This operation iterates over both ranges, copying elements present in
   *  one range but not the other in order to the output range.  Iterators
   *  increment for each range.  When the current element of one range is less
   *  than the other, that element is copied and the iterator advances.  If an
   *  element is contained in both ranges, no elements are copied and both
   *  ranges advance.  The output range may not overlap either input range.
  */
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    set_symmetric_difference(_InputIterator1 __first1, _InputIterator1 __last1,
			     _InputIterator2 __first2, _InputIterator2 __last2,
			     _OutputIterator __result)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_LessThanOpConcept<
	    typename iterator_traits<_InputIterator1>::value_type,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_LessThanOpConcept<
	    typename iterator_traits<_InputIterator2>::value_type,
	    typename iterator_traits<_InputIterator1>::value_type>)	
      __glibcxx_requires_sorted_set(__first1, __last1, __first2);
      __glibcxx_requires_sorted_set(__first2, __last2, __first1);
      __glibcxx_requires_irreflexive2(__first1, __last1);
      __glibcxx_requires_irreflexive2(__first2, __last2);

      return _GLIBCXX_STD_A::__set_symmetric_difference(__first1, __last1,
					__first2, __last2, __result,
					__gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief  Return the symmetric difference of two sorted ranges using
   *  comparison functor.
   *  @ingroup set_algorithms
   *  @param  __first1  Start of first range.
   *  @param  __last1   End of first range.
   *  @param  __first2  Start of second range.
   *  @param  __last2   End of second range.
   *  @param  __result  Start of output range.
   *  @param  __comp    The comparison functor.
   *  @return  End of the output range.
   *  @ingroup set_algorithms
   *
   *  This operation iterates over both ranges, copying elements present in
   *  one range but not the other in order to the output range.  Iterators
   *  increment for each range.  When the current element of one range is less
   *  than the other according to @p comp, that element is copied and the
   *  iterator advances.  If an element is contained in both ranges according
   *  to @p __comp, no elements are copied and both ranges advance.  The output
   *  range may not overlap either input range.
  */
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _OutputIterator, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    inline _OutputIterator
    set_symmetric_difference(_InputIterator1 __first1, _InputIterator1 __last1,
			     _InputIterator2 __first2, _InputIterator2 __last2,
			     _OutputIterator __result,
			     _Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator1>)
      __glibcxx_function_requires(_InputIteratorConcept<_InputIterator2>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_function_requires(_OutputIteratorConcept<_OutputIterator,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_InputIterator1>::value_type,
	    typename iterator_traits<_InputIterator2>::value_type>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_InputIterator2>::value_type,
	    typename iterator_traits<_InputIterator1>::value_type>)
      __glibcxx_requires_sorted_set_pred(__first1, __last1, __first2, __comp);
      __glibcxx_requires_sorted_set_pred(__first2, __last2, __first1, __comp);
      __glibcxx_requires_irreflexive_pred2(__first1, __last1, __comp);
      __glibcxx_requires_irreflexive_pred2(__first2, __last2, __comp);

      return _GLIBCXX_STD_A::__set_symmetric_difference(__first1, __last1,
				__first2, __last2, __result,
				__gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  template<typename _ForwardIterator, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    _ForwardIterator
    __min_element(_ForwardIterator __first, _ForwardIterator __last,
		  _Compare __comp)
    {
      if (__first == __last)
	return __first;
      _ForwardIterator __result = __first;
      while (++__first != __last)
	if (__comp(__first, __result))
	  __result = __first;
      return __result;
    }

  /**
   *  @brief  Return the minimum element in a range.
   *  @ingroup sorting_algorithms
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @return  Iterator referencing the first instance of the smallest value.
  */
  template<typename _ForwardIterator>
    _GLIBCXX14_CONSTEXPR
    _ForwardIterator
    inline min_element(_ForwardIterator __first, _ForwardIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      return _GLIBCXX_STD_A::__min_element(__first, __last,
				__gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief  Return the minimum element in a range using comparison functor.
   *  @ingroup sorting_algorithms
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @param  __comp   Comparison functor.
   *  @return  Iterator referencing the first instance of the smallest value
   *  according to __comp.
  */
  template<typename _ForwardIterator, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    inline _ForwardIterator
    min_element(_ForwardIterator __first, _ForwardIterator __last,
		_Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_ForwardIterator>::value_type,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      return _GLIBCXX_STD_A::__min_element(__first, __last,
				__gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

  template<typename _ForwardIterator, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    _ForwardIterator
    __max_element(_ForwardIterator __first, _ForwardIterator __last,
		  _Compare __comp)
    {
      if (__first == __last) return __first;
      _ForwardIterator __result = __first;
      while (++__first != __last)
	if (__comp(__result, __first))
	  __result = __first;
      return __result;
    }

  /**
   *  @brief  Return the maximum element in a range.
   *  @ingroup sorting_algorithms
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @return  Iterator referencing the first instance of the largest value.
  */
  template<typename _ForwardIterator>
    _GLIBCXX14_CONSTEXPR
    inline _ForwardIterator
    max_element(_ForwardIterator __first, _ForwardIterator __last)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_LessThanComparableConcept<
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive(__first, __last);

      return _GLIBCXX_STD_A::__max_element(__first, __last,
				__gnu_cxx::__ops::__iter_less_iter());
    }

  /**
   *  @brief  Return the maximum element in a range using comparison functor.
   *  @ingroup sorting_algorithms
   *  @param  __first  Start of range.
   *  @param  __last   End of range.
   *  @param  __comp   Comparison functor.
   *  @return  Iterator referencing the first instance of the largest value
   *  according to __comp.
  */
  template<typename _ForwardIterator, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    inline _ForwardIterator
    max_element(_ForwardIterator __first, _ForwardIterator __last,
		_Compare __comp)
    {
      // concept requirements
      __glibcxx_function_requires(_ForwardIteratorConcept<_ForwardIterator>)
      __glibcxx_function_requires(_BinaryPredicateConcept<_Compare,
	    typename iterator_traits<_ForwardIterator>::value_type,
	    typename iterator_traits<_ForwardIterator>::value_type>)
      __glibcxx_requires_valid_range(__first, __last);
      __glibcxx_requires_irreflexive_pred(__first, __last, __comp);

      return _GLIBCXX_STD_A::__max_element(__first, __last,
				__gnu_cxx::__ops::__iter_comp_iter(__comp));
    }

#if __cplusplus >= 201402L
  /// Reservoir sampling algorithm.
  template<typename _InputIterator, typename _RandomAccessIterator,
           typename _Size, typename _UniformRandomBitGenerator>
    _RandomAccessIterator
    __sample(_InputIterator __first, _InputIterator __last, input_iterator_tag,
	     _RandomAccessIterator __out, random_access_iterator_tag,
	     _Size __n, _UniformRandomBitGenerator&& __g)
    {
      using __distrib_type = uniform_int_distribution<_Size>;
      using __param_type = typename __distrib_type::param_type;
      __distrib_type __d{};
      _Size __sample_sz = 0;
      while (__first != __last && __sample_sz != __n)
	{
	  __out[__sample_sz++] = *__first;
	  ++__first;
	}
      for (auto __pop_sz = __sample_sz; __first != __last;
	  ++__first, (void) ++__pop_sz)
	{
	  const auto __k = __d(__g, __param_type{0, __pop_sz});
	  if (__k < __n)
	    __out[__k] = *__first;
	}
      return __out + __sample_sz;
    }

  /// Selection sampling algorithm.
  template<typename _ForwardIterator, typename _OutputIterator, typename _Cat,
           typename _Size, typename _UniformRandomBitGenerator>
    _OutputIterator
    __sample(_ForwardIterator __first, _ForwardIterator __last,
	     forward_iterator_tag,
	     _OutputIterator __out, _Cat,
	     _Size __n, _UniformRandomBitGenerator&& __g)
    {
      using __distrib_type = uniform_int_distribution<_Size>;
      using __param_type = typename __distrib_type::param_type;
      using _USize = make_unsigned_t<_Size>;
      using _Gen = remove_reference_t<_UniformRandomBitGenerator>;
      using __uc_type = common_type_t<typename _Gen::result_type, _USize>;

      if (__first == __last)
	return __out;

      __distrib_type __d{};
      _Size __unsampled_sz = std::distance(__first, __last);
      __n = std::min(__n, __unsampled_sz);

      // If possible, we use __gen_two_uniform_ints to efficiently produce
      // two random numbers using a single distribution invocation:

      const __uc_type __urngrange = __g.max() - __g.min();
      if (__urngrange / __uc_type(__unsampled_sz) >= __uc_type(__unsampled_sz))
        // I.e. (__urngrange >= __unsampled_sz * __unsampled_sz) but without
	// wrapping issues.
        {
	  while (__n != 0 && __unsampled_sz >= 2)
	    {
	      const pair<_Size, _Size> __p =
		__gen_two_uniform_ints(__unsampled_sz, __unsampled_sz - 1, __g);

	      --__unsampled_sz;
	      if (__p.first < __n)
		{
		  *__out++ = *__first;
		  --__n;
		}

	      ++__first;

	      if (__n == 0) break;

	      --__unsampled_sz;
	      if (__p.second < __n)
		{
		  *__out++ = *__first;
		  --__n;
		}

	      ++__first;
	    }
        }

      // The loop above is otherwise equivalent to this one-at-a-time version:

      for (; __n != 0; ++__first)
	if (__d(__g, __param_type{0, --__unsampled_sz}) < __n)
	  {
	    *__out++ = *__first;
	    --__n;
	  }
      return __out;
    }

#if __cplusplus > 201402L
#define __cpp_lib_sample 201603
  /// Take a random sample from a population.
  template<typename _PopulationIterator, typename _SampleIterator,
           typename _Distance, typename _UniformRandomBitGenerator>
    _SampleIterator
    sample(_PopulationIterator __first, _PopulationIterator __last,
	   _SampleIterator __out, _Distance __n,
	   _UniformRandomBitGenerator&& __g)
    {
      using __pop_cat = typename
	std::iterator_traits<_PopulationIterator>::iterator_category;
      using __samp_cat = typename
	std::iterator_traits<_SampleIterator>::iterator_category;

      static_assert(
	  __or_<is_convertible<__pop_cat, forward_iterator_tag>,
		is_convertible<__samp_cat, random_access_iterator_tag>>::value,
	  "output range must use a RandomAccessIterator when input range"
	  " does not meet the ForwardIterator requirements");

      static_assert(is_integral<_Distance>::value,
		    "sample size must be an integer type");

      typename iterator_traits<_PopulationIterator>::difference_type __d = __n;
      return _GLIBCXX_STD_A::
	__sample(__first, __last, __pop_cat{}, __out, __samp_cat{}, __d,
		 std::forward<_UniformRandomBitGenerator>(__g));
    }
#endif // C++17
#endif // C++14

_GLIBCXX_END_NAMESPACE_ALGO
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#endif /* _STL_ALGO_H */
