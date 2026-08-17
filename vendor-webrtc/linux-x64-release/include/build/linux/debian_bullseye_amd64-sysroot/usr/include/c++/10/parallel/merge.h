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

/** @file parallel/merge.h
 *  @brief Parallel implementation of std::merge().
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_MERGE_H
#define _GLIBCXX_PARALLEL_MERGE_H 1

#include <parallel/basic_iterator.h>
#include <bits/stl_algo.h>

namespace __gnu_parallel
{
  /** @brief Merge routine being able to merge only the @c __max_length
   * smallest elements.
   *
   * The @c __begin iterators are advanced accordingly, they might not
   * reach @c __end, in contrast to the usual variant.
   * @param __begin1 Begin iterator of first sequence.
   * @param __end1 End iterator of first sequence.
   * @param __begin2 Begin iterator of second sequence.
   * @param __end2 End iterator of second sequence.
   * @param __target Target begin iterator.
   * @param __max_length Maximum number of elements to merge.
   * @param __comp Comparator.
   * @return Output end iterator. */
  template<typename _RAIter1, typename _RAIter2,
           typename _OutputIterator, typename _DifferenceTp,
           typename _Compare>
    _OutputIterator
    __merge_advance_usual(_RAIter1& __begin1, _RAIter1 __end1,
			  _RAIter2& __begin2, _RAIter2 __end2,
			  _OutputIterator __target,
			  _DifferenceTp __max_length, _Compare __comp)
    {
      typedef _DifferenceTp _DifferenceType;
      while (__begin1 != __end1 && __begin2 != __end2 && __max_length > 0)
        {
          // array1[__i1] < array0[i0]
          if (__comp(*__begin2, *__begin1))
            *__target++ = *__begin2++;
          else
            *__target++ = *__begin1++;
          --__max_length;
        }

      if (__begin1 != __end1)
        {
          __target = std::copy(__begin1, __begin1 + __max_length, __target);
          __begin1 += __max_length;
        }
      else
        {
          __target = std::copy(__begin2, __begin2 + __max_length, __target);
          __begin2 += __max_length;
        }
      return __target;
    }

  /** @brief Merge routine being able to merge only the @c __max_length
   * smallest elements.
   *
   * The @c __begin iterators are advanced accordingly, they might not
   * reach @c __end, in contrast to the usual variant.
   * Specially designed code should allow the compiler to generate
   * conditional moves instead of branches.
   * @param __begin1 Begin iterator of first sequence.
   * @param __end1 End iterator of first sequence.
   * @param __begin2 Begin iterator of second sequence.
   * @param __end2 End iterator of second sequence.
   * @param __target Target begin iterator.
   * @param __max_length Maximum number of elements to merge.
   * @param __comp Comparator.
   * @return Output end iterator. */
  template<typename _RAIter1, typename _RAIter2,
           typename _OutputIterator, typename _DifferenceTp,
           typename _Compare>
    _OutputIterator
    __merge_advance_movc(_RAIter1& __begin1, _RAIter1 __end1,
			 _RAIter2& __begin2, _RAIter2 __end2,
			 _OutputIterator __target,
			 _DifferenceTp __max_length, _Compare __comp)
    {
      typedef _DifferenceTp _DifferenceType;
      typedef typename std::iterator_traits<_RAIter1>::value_type
        _ValueType1;
      typedef typename std::iterator_traits<_RAIter2>::value_type
        _ValueType2;

#if _GLIBCXX_PARALLEL_ASSERTIONS
      _GLIBCXX_PARALLEL_ASSERT(__max_length >= 0);
#endif

      while (__begin1 != __end1 && __begin2 != __end2 && __max_length > 0)
        {
          _RAIter1 __next1 = __begin1 + 1;
          _RAIter2 __next2 = __begin2 + 1;
          _ValueType1 __element1 = *__begin1;
          _ValueType2 __element2 = *__begin2;

          if (__comp(__element2, __element1))
            {
              __element1 = __element2;
              __begin2 = __next2;
            }
          else
            __begin1 = __next1;

          *__target = __element1;

          ++__target;
          --__max_length;
        }
      if (__begin1 != __end1)
        {
          __target = std::copy(__begin1, __begin1 + __max_length, __target);
          __begin1 += __max_length;
        }
      else
        {
          __target = std::copy(__begin2, __begin2 + __max_length, __target);
          __begin2 += __max_length;
        }
      return __target;
    }

  /** @brief Merge routine being able to merge only the @c __max_length
   * smallest elements.
   *
   *  The @c __begin iterators are advanced accordingly, they might not
   *  reach @c __end, in contrast to the usual variant.
   *  Static switch on whether to use the conditional-move variant.
   *  @param __begin1 Begin iterator of first sequence.
   *  @param __end1 End iterator of first sequence.
   *  @param __begin2 Begin iterator of second sequence.
   *  @param __end2 End iterator of second sequence.
   *  @param __target Target begin iterator.
   *  @param __max_length Maximum number of elements to merge.
   *  @param __comp Comparator.
   *  @return Output end iterator. */
  template<typename _RAIter1, typename _RAIter2,
           typename _OutputIterator, typename _DifferenceTp,
           typename _Compare>
    inline _OutputIterator
    __merge_advance(_RAIter1& __begin1, _RAIter1 __end1,
		    _RAIter2& __begin2, _RAIter2 __end2,
		    _OutputIterator __target, _DifferenceTp __max_length,
		    _Compare __comp)
    {
      _GLIBCXX_CALL(__max_length)

      return __merge_advance_movc(__begin1, __end1, __begin2, __end2,
				  __target, __max_length, __comp);
    }

  /** @brief Merge routine fallback to sequential in case the
      iterators of the two input sequences are of different type.
      *  @param __begin1 Begin iterator of first sequence.
      *  @param __end1 End iterator of first sequence.
      *  @param __begin2 Begin iterator of second sequence.
      *  @param __end2 End iterator of second sequence.
      *  @param __target Target begin iterator.
      *  @param __max_length Maximum number of elements to merge.
      *  @param __comp Comparator.
      *  @return Output end iterator. */
  template<typename _RAIter1, typename _RAIter2,
           typename _RAIter3, typename _Compare>
    inline _RAIter3
    __parallel_merge_advance(_RAIter1& __begin1, _RAIter1 __end1,
			     _RAIter2& __begin2,
			     // different iterators, parallel implementation
			     // not available
			     _RAIter2 __end2, _RAIter3 __target, typename
			     std::iterator_traits<_RAIter1>::
			     difference_type __max_length, _Compare __comp)
    { return __merge_advance(__begin1, __end1, __begin2, __end2, __target,
			     __max_length, __comp); }

  /** @brief Parallel merge routine being able to merge only the @c
   * __max_length smallest elements.
   *
   *  The @c __begin iterators are advanced accordingly, they might not
   *  reach @c __end, in contrast to the usual variant.
   *  The functionality is projected onto parallel_multiway_merge.
   *  @param __begin1 Begin iterator of first sequence.
   *  @param __end1 End iterator of first sequence.
   *  @param __begin2 Begin iterator of second sequence.
   *  @param __end2 End iterator of second sequence.
   *  @param __target Target begin iterator.
   *  @param __max_length Maximum number of elements to merge.
   *  @param __comp Comparator.
   *  @return Output end iterator.
   */
  template<typename _RAIter1, typename _RAIter3,
           typename _Compare>
    inline _RAIter3
    __parallel_merge_advance(_RAIter1& __begin1, _RAIter1 __end1,
			     _RAIter1& __begin2, _RAIter1 __end2,
			     _RAIter3 __target, typename
			     std::iterator_traits<_RAIter1>::
			     difference_type __max_length, _Compare __comp)
    {
      typedef typename
          std::iterator_traits<_RAIter1>::value_type _ValueType;
      typedef typename std::iterator_traits<_RAIter1>::
        difference_type _DifferenceType1 /* == difference_type2 */;
      typedef typename std::iterator_traits<_RAIter3>::
        difference_type _DifferenceType3;
      typedef typename std::pair<_RAIter1, _RAIter1>
        _IteratorPair;

      _IteratorPair __seqs[2] = { std::make_pair(__begin1, __end1),
				  std::make_pair(__begin2, __end2) };
      _RAIter3 __target_end = parallel_multiway_merge
	< /* __stable = */ true, /* __sentinels = */ false>
	(__seqs, __seqs + 2, __target, multiway_merge_exact_splitting
	 < /* __stable = */ true, _IteratorPair*,
	 _Compare, _DifferenceType1>, __max_length, __comp,
	 omp_get_max_threads());

      return __target_end;
    }
}       //namespace __gnu_parallel

#endif /* _GLIBCXX_PARALLEL_MERGE_H */
