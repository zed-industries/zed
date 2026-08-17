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

/** @file parallel/find.h
 *  @brief Parallel implementation base for std::find(), std::equal()
 *  and related functions.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Felix Putze and Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_FIND_H
#define _GLIBCXX_PARALLEL_FIND_H 1

#include <bits/stl_algobase.h>

#include <parallel/features.h>
#include <parallel/parallel.h>
#include <parallel/compatibility.h>
#include <parallel/equally_split.h>

namespace __gnu_parallel
{
  /**
   *  @brief Parallel std::find, switch for different algorithms.
   *  @param __begin1 Begin iterator of first sequence.
   *  @param __end1 End iterator of first sequence.
   *  @param __begin2 Begin iterator of second sequence. Must have same
   *  length as first sequence.
   *  @param __pred Find predicate.
   *  @param __selector _Functionality (e. g. std::find_if(), std::equal(),...)
   *  @return Place of finding in both sequences.
   */
  template<typename _RAIter1,
	   typename _RAIter2,
	   typename _Pred,
           typename _Selector>
    inline std::pair<_RAIter1, _RAIter2>
    __find_template(_RAIter1 __begin1, _RAIter1 __end1,
		    _RAIter2 __begin2, _Pred __pred, _Selector __selector)
    {
      switch (_Settings::get().find_algorithm)
	{
	case GROWING_BLOCKS:
          return __find_template(__begin1, __end1, __begin2, __pred,
				 __selector, growing_blocks_tag());
	case CONSTANT_SIZE_BLOCKS:
          return __find_template(__begin1, __end1, __begin2, __pred,
				 __selector, constant_size_blocks_tag());
	case EQUAL_SPLIT:
          return __find_template(__begin1, __end1, __begin2, __pred,
				 __selector, equal_split_tag());
	default:
          _GLIBCXX_PARALLEL_ASSERT(false);
          return std::make_pair(__begin1, __begin2);
	}
    }

#if _GLIBCXX_FIND_EQUAL_SPLIT

  /**
   *  @brief Parallel std::find, equal splitting variant.
   *  @param __begin1 Begin iterator of first sequence.
   *  @param __end1 End iterator of first sequence.
   *  @param __begin2 Begin iterator of second sequence. Second __sequence
   *  must have same length as first sequence.
   *  @param __pred Find predicate.
   *  @param __selector _Functionality (e. g. std::find_if(), std::equal(),...)
   *  @return Place of finding in both sequences.
   */
  template<typename _RAIter1,
           typename _RAIter2,
           typename _Pred,
           typename _Selector>
    std::pair<_RAIter1, _RAIter2>
    __find_template(_RAIter1 __begin1, _RAIter1 __end1,
		    _RAIter2 __begin2, _Pred __pred,
		    _Selector __selector, equal_split_tag)
    {
      _GLIBCXX_CALL(__end1 - __begin1)

      typedef std::iterator_traits<_RAIter1> _TraitsType;
      typedef typename _TraitsType::difference_type _DifferenceType;
      typedef typename _TraitsType::value_type _ValueType;

      _DifferenceType __length = __end1 - __begin1;
      _DifferenceType __result = __length;
      _DifferenceType* __borders;

      omp_lock_t __result_lock;
      omp_init_lock(&__result_lock);

      _ThreadIndex __num_threads = __get_max_threads();
#     pragma omp parallel num_threads(__num_threads)
      {
#     pragma omp single
	{
	  __num_threads = omp_get_num_threads();
	  __borders = new _DifferenceType[__num_threads + 1];
	  __equally_split(__length, __num_threads, __borders);
	} //single

	_ThreadIndex __iam = omp_get_thread_num();
	_DifferenceType __start = __borders[__iam],
	                 __stop = __borders[__iam + 1];

	_RAIter1 __i1 = __begin1 + __start;
	_RAIter2 __i2 = __begin2 + __start;
	for (_DifferenceType __pos = __start; __pos < __stop; ++__pos)
	  {
#           pragma omp flush(__result)
	    // Result has been set to something lower.
	    if (__result < __pos)
	      break;

	    if (__selector(__i1, __i2, __pred))
	      {
		omp_set_lock(&__result_lock);
		if (__pos < __result)
		  __result = __pos;
		omp_unset_lock(&__result_lock);
		break;
	      }
	    ++__i1;
	    ++__i2;
	  }
      } //parallel

      omp_destroy_lock(&__result_lock);
      delete[] __borders;

      return std::pair<_RAIter1, _RAIter2>(__begin1 + __result,
					   __begin2 + __result);
    }

#endif

#if _GLIBCXX_FIND_GROWING_BLOCKS

  /**
   *  @brief Parallel std::find, growing block size variant.
   *  @param __begin1 Begin iterator of first sequence.
   *  @param __end1 End iterator of first sequence.
   *  @param __begin2 Begin iterator of second sequence. Second __sequence
   *  must have same length as first sequence.
   *  @param __pred Find predicate.
   *  @param __selector _Functionality (e. g. std::find_if(), std::equal(),...)
   *  @return Place of finding in both sequences.
   *  @see __gnu_parallel::_Settings::find_sequential_search_size
   *  @see __gnu_parallel::_Settings::find_scale_factor
   *
   *  There are two main differences between the growing blocks and
   *  the constant-size blocks variants.
   *  1. For GB, the block size grows; for CSB, the block size is fixed.
   *  2. For GB, the blocks are allocated dynamically;
   *     for CSB, the blocks are allocated in a predetermined manner,
   *     namely spacial round-robin.
   */
  template<typename _RAIter1,
           typename _RAIter2,
           typename _Pred,
           typename _Selector>
    std::pair<_RAIter1, _RAIter2>
    __find_template(_RAIter1 __begin1, _RAIter1 __end1,
		    _RAIter2 __begin2, _Pred __pred, _Selector __selector,
		    growing_blocks_tag)
    {
      _GLIBCXX_CALL(__end1 - __begin1)

      typedef std::iterator_traits<_RAIter1> _TraitsType;
      typedef typename _TraitsType::difference_type _DifferenceType;
      typedef typename _TraitsType::value_type _ValueType;

      const _Settings& __s = _Settings::get();

      _DifferenceType __length = __end1 - __begin1;

      _DifferenceType
	__sequential_search_size = std::min<_DifferenceType>
	(__length, __s.find_sequential_search_size);

      // Try it sequentially first.
      std::pair<_RAIter1, _RAIter2>
	__find_seq_result = __selector._M_sequential_algorithm
	(__begin1, __begin1 + __sequential_search_size,
	 __begin2, __pred);

      if (__find_seq_result.first != (__begin1 + __sequential_search_size))
	return __find_seq_result;

      // Index of beginning of next free block (after sequential find).
      _DifferenceType __next_block_start = __sequential_search_size;
      _DifferenceType __result = __length;

      omp_lock_t __result_lock;
      omp_init_lock(&__result_lock);

      const float __scale_factor = __s.find_scale_factor;

      _ThreadIndex __num_threads = __get_max_threads();
#     pragma omp parallel shared(__result) num_threads(__num_threads)
      {
#       pragma omp single
	__num_threads = omp_get_num_threads();

	// Not within first __k elements -> start parallel.
	_ThreadIndex __iam = omp_get_thread_num();

	_DifferenceType __block_size =
	  std::max<_DifferenceType>(1, __scale_factor * __next_block_start);
	_DifferenceType __start = __fetch_and_add<_DifferenceType>
	  (&__next_block_start, __block_size);

	// Get new block, update pointer to next block.
	_DifferenceType __stop =
	  std::min<_DifferenceType>(__length, __start + __block_size);

	std::pair<_RAIter1, _RAIter2> __local_result;

	while (__start < __length)
	  {
#           pragma omp flush(__result)
	    // Get new value of result.
	    if (__result < __start)
	      {
		// No chance to find first element.
		break;
	      }

	    __local_result = __selector._M_sequential_algorithm
	      (__begin1 + __start, __begin1 + __stop,
	       __begin2 + __start, __pred);

	    if (__local_result.first != (__begin1 + __stop))
	      {
		omp_set_lock(&__result_lock);
		if ((__local_result.first - __begin1) < __result)
		  {
		    __result = __local_result.first - __begin1;

		    // Result cannot be in future blocks, stop algorithm.
		    __fetch_and_add<_DifferenceType>(&__next_block_start,
						     __length);
		  }
		omp_unset_lock(&__result_lock);
	      }

	    _DifferenceType __block_size =
	     std::max<_DifferenceType>(1, __scale_factor * __next_block_start);

	    // Get new block, update pointer to next block.
	    __start = __fetch_and_add<_DifferenceType>(&__next_block_start,
						       __block_size);
	    __stop =
	      std::min<_DifferenceType>(__length, __start + __block_size);
	  }
      } //parallel

      omp_destroy_lock(&__result_lock);

      // Return iterator on found element.
      return
	std::pair<_RAIter1, _RAIter2>(__begin1 + __result,
				      __begin2 + __result);
    }

#endif

#if _GLIBCXX_FIND_CONSTANT_SIZE_BLOCKS

  /**
   *   @brief Parallel std::find, constant block size variant.
   *  @param __begin1 Begin iterator of first sequence.
   *  @param __end1 End iterator of first sequence.
   *  @param __begin2 Begin iterator of second sequence. Second __sequence
   *  must have same length as first sequence.
   *  @param __pred Find predicate.
   *  @param __selector _Functionality (e. g. std::find_if(), std::equal(),...)
   *  @return Place of finding in both sequences.
   *  @see __gnu_parallel::_Settings::find_sequential_search_size
   *  @see __gnu_parallel::_Settings::find_block_size
   *  There are two main differences between the growing blocks and the
   *  constant-size blocks variants.
   *  1. For GB, the block size grows; for CSB, the block size is fixed.
   *  2. For GB, the blocks are allocated dynamically; for CSB, the
   *  blocks are allocated in a predetermined manner, namely spacial
   *  round-robin.
   */
  template<typename _RAIter1,
           typename _RAIter2,
           typename _Pred,
           typename _Selector>
    std::pair<_RAIter1, _RAIter2>
    __find_template(_RAIter1 __begin1, _RAIter1 __end1,
                  _RAIter2 __begin2, _Pred __pred, _Selector __selector,
                  constant_size_blocks_tag)
    {
      _GLIBCXX_CALL(__end1 - __begin1)
      typedef std::iterator_traits<_RAIter1> _TraitsType;
      typedef typename _TraitsType::difference_type _DifferenceType;
      typedef typename _TraitsType::value_type _ValueType;

      const _Settings& __s = _Settings::get();

      _DifferenceType __length = __end1 - __begin1;

      _DifferenceType __sequential_search_size = std::min<_DifferenceType>
	(__length, __s.find_sequential_search_size);

      // Try it sequentially first.
      std::pair<_RAIter1, _RAIter2>
	__find_seq_result = __selector._M_sequential_algorithm
	(__begin1, __begin1 + __sequential_search_size, __begin2, __pred);

      if (__find_seq_result.first != (__begin1 + __sequential_search_size))
	return __find_seq_result;

      _DifferenceType __result = __length;
      omp_lock_t __result_lock;
      omp_init_lock(&__result_lock);

      // Not within first __sequential_search_size elements -> start parallel.

      _ThreadIndex __num_threads = __get_max_threads();
#     pragma omp parallel shared(__result) num_threads(__num_threads)
      {
#       pragma omp single
	__num_threads = omp_get_num_threads();

	_ThreadIndex __iam = omp_get_thread_num();
	_DifferenceType __block_size = __s.find_initial_block_size;

	// First element of thread's current iteration.
	_DifferenceType __iteration_start = __sequential_search_size;

	// Where to work (initialization).
	_DifferenceType __start = __iteration_start + __iam * __block_size;
	_DifferenceType __stop = std::min<_DifferenceType>(__length,
							   __start
							   + __block_size);

	std::pair<_RAIter1, _RAIter2> __local_result;

	while (__start < __length)
	  {
	    // Get new value of result.
#           pragma omp flush(__result)
	    // No chance to find first element.
	    if (__result < __start)
	      break;

	    __local_result = __selector._M_sequential_algorithm
	      (__begin1 + __start, __begin1 + __stop,
	       __begin2 + __start, __pred);

	    if (__local_result.first != (__begin1 + __stop))
	      {
		omp_set_lock(&__result_lock);
		if ((__local_result.first - __begin1) < __result)
		  __result = __local_result.first - __begin1;
		omp_unset_lock(&__result_lock);
		// Will not find better value in its interval.
		break;
	      }

	    __iteration_start += __num_threads * __block_size;

	    // Where to work.
	    __start = __iteration_start + __iam * __block_size;
	    __stop = std::min<_DifferenceType>(__length,
					       __start + __block_size);
	  }
      } //parallel

      omp_destroy_lock(&__result_lock);

      // Return iterator on found element.
      return std::pair<_RAIter1, _RAIter2>(__begin1 + __result,
					   __begin2 + __result);
    }
#endif
} // end namespace

#endif /* _GLIBCXX_PARALLEL_FIND_H */
