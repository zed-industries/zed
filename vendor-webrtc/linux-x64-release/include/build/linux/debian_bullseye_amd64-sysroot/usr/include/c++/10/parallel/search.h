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

/** @file parallel/search.h
 *  @brief Parallel implementation base for std::search() and
 *  std::search_n().
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Felix Putze.

#ifndef _GLIBCXX_PARALLEL_SEARCH_H
#define _GLIBCXX_PARALLEL_SEARCH_H 1

#include <bits/stl_algobase.h>

#include <parallel/parallel.h>
#include <parallel/equally_split.h>

namespace __gnu_parallel
{
  /**
   *  @brief Precalculate __advances for Knuth-Morris-Pratt algorithm.
   *  @param __elements Begin iterator of sequence to search for.
   *  @param __length Length of sequence to search for.
   *  @param __off Returned __offsets. 
   */
  template<typename _RAIter, typename _DifferenceTp>
    void
    __calc_borders(_RAIter __elements, _DifferenceTp __length, 
		   _DifferenceTp* __off)
    {
      typedef _DifferenceTp _DifferenceType;

      __off[0] = -1;
      if (__length > 1)
	__off[1] = 0;
      _DifferenceType __k = 0;
      for (_DifferenceType __j = 2; __j <= __length; __j++)
	{
          while ((__k >= 0) && !(__elements[__k] == __elements[__j-1]))
            __k = __off[__k];
          __off[__j] = ++__k;
	}
    }

  // Generic parallel find algorithm (requires random access iterator).

  /** @brief Parallel std::search.
   *  @param __begin1 Begin iterator of first sequence.
   *  @param __end1 End iterator of first sequence.
   *  @param __begin2 Begin iterator of second sequence.
   *  @param __end2 End iterator of second sequence.
   *  @param __pred Find predicate.
   *  @return Place of finding in first sequences. */
  template<typename __RAIter1,
           typename __RAIter2,
           typename _Pred>
    __RAIter1
    __search_template(__RAIter1 __begin1, __RAIter1 __end1,
		      __RAIter2 __begin2, __RAIter2 __end2,
		      _Pred __pred)
    {
      typedef std::iterator_traits<__RAIter1> _TraitsType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      _GLIBCXX_CALL((__end1 - __begin1) + (__end2 - __begin2));

      _DifferenceType __pattern_length = __end2 - __begin2;

      // Pattern too short.
      if(__pattern_length <= 0)
	return __end1;

      // Last point to start search.
      _DifferenceType __input_length = (__end1 - __begin1) - __pattern_length;

      // Where is first occurrence of pattern? defaults to end.
      _DifferenceType __result = (__end1 - __begin1);
      _DifferenceType *__splitters;

      // Pattern too long.
      if (__input_length < 0)
	return __end1;

      omp_lock_t __result_lock;
      omp_init_lock(&__result_lock);

      _ThreadIndex __num_threads = std::max<_DifferenceType>
	(1, std::min<_DifferenceType>(__input_length,
				      __get_max_threads()));

      _DifferenceType __advances[__pattern_length];
      __calc_borders(__begin2, __pattern_length, __advances);

#     pragma omp parallel num_threads(__num_threads)
      {
#       pragma omp single
	{
	  __num_threads = omp_get_num_threads();
	  __splitters = new _DifferenceType[__num_threads + 1];
	  __equally_split(__input_length, __num_threads, __splitters);
	}

	_ThreadIndex __iam = omp_get_thread_num();

	_DifferenceType __start = __splitters[__iam],
	                 __stop = __splitters[__iam + 1];

	_DifferenceType __pos_in_pattern = 0;
	bool __found_pattern = false;

	while (__start <= __stop && !__found_pattern)
	  {
	    // Get new value of result.
#pragma omp flush(__result)
	    // No chance for this thread to find first occurrence.
	    if (__result < __start)
	      break;
	    while (__pred(__begin1[__start + __pos_in_pattern],
			  __begin2[__pos_in_pattern]))
	      {
		++__pos_in_pattern;
		if (__pos_in_pattern == __pattern_length)
		  {
		    // Found new candidate for result.
		    omp_set_lock(&__result_lock);
		    __result = std::min(__result, __start);
		    omp_unset_lock(&__result_lock);

		    __found_pattern = true;
		    break;
		  }
	      }
	    // Make safe jump.
	    __start += (__pos_in_pattern - __advances[__pos_in_pattern]);
	    __pos_in_pattern = (__advances[__pos_in_pattern] < 0
				? 0 : __advances[__pos_in_pattern]);
	  }
      } //parallel

      omp_destroy_lock(&__result_lock);

      delete[] __splitters;
      
      // Return iterator on found element.
      return (__begin1 + __result);
    }
} // end namespace

#endif /* _GLIBCXX_PARALLEL_SEARCH_H */
