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

/** @file parallel/quicksort.h
 *  @brief Implementation of a unbalanced parallel quicksort (in-place).
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_QUICKSORT_H
#define _GLIBCXX_PARALLEL_QUICKSORT_H 1

#include <parallel/parallel.h>
#include <parallel/partition.h>

namespace __gnu_parallel
{
  /** @brief Unbalanced quicksort divide step.
   *  @param __begin Begin iterator of subsequence.
   *  @param __end End iterator of subsequence.
   *  @param __comp Comparator.
   *  @param __pivot_rank Desired __rank of the pivot.
   *  @param __num_samples Choose pivot from that many samples.
   *  @param __num_threads Number of threads that are allowed to work on
   *  this part.
   */
  template<typename _RAIter, typename _Compare>
    typename std::iterator_traits<_RAIter>::difference_type
    __parallel_sort_qs_divide(_RAIter __begin, _RAIter __end,
			      _Compare __comp, typename std::iterator_traits
			      <_RAIter>::difference_type __pivot_rank,
			      typename std::iterator_traits
			      <_RAIter>::difference_type
			      __num_samples, _ThreadIndex __num_threads)
    {
      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      _DifferenceType __n = __end - __begin;
      __num_samples = std::min(__num_samples, __n);

      // Allocate uninitialized, to avoid default constructor.
      _ValueType* __samples = static_cast<_ValueType*>
	(::operator new(__num_samples * sizeof(_ValueType)));

      for (_DifferenceType __s = 0; __s < __num_samples; ++__s)
        {
          const unsigned long long __index = static_cast<unsigned long long>
	    (__s) * __n / __num_samples;
          ::new(&(__samples[__s])) _ValueType(__begin[__index]);
        }

      __gnu_sequential::sort(__samples, __samples + __num_samples, __comp);

      _ValueType& __pivot = __samples[__pivot_rank * __num_samples / __n];

      __gnu_parallel::__binder2nd<_Compare, _ValueType, _ValueType, bool>
        __pred(__comp, __pivot);
      _DifferenceType __split = __parallel_partition(__begin, __end,
						     __pred, __num_threads);

      for (_DifferenceType __s = 0; __s < __num_samples; ++__s)
	__samples[__s].~_ValueType();
      ::operator delete(__samples);

      return __split;
    }

  /** @brief Unbalanced quicksort conquer step.
   *  @param __begin Begin iterator of subsequence.
   *  @param __end End iterator of subsequence.
   *  @param __comp Comparator.
   *  @param __num_threads Number of threads that are allowed to work on
   *  this part.
   */
  template<typename _RAIter, typename _Compare>
    void
    __parallel_sort_qs_conquer(_RAIter __begin, _RAIter __end,
			       _Compare __comp,
			       _ThreadIndex __num_threads)
    {
      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      if (__num_threads <= 1)
        {
          __gnu_sequential::sort(__begin, __end, __comp);
          return;
        }

      _DifferenceType __n = __end - __begin, __pivot_rank;

      if (__n <= 1)
        return;

      _ThreadIndex __num_threads_left;

      if ((__num_threads % 2) == 1)
        __num_threads_left = __num_threads / 2 + 1;
      else
        __num_threads_left = __num_threads / 2;

      __pivot_rank = __n * __num_threads_left / __num_threads;

      _DifferenceType __split = __parallel_sort_qs_divide
	(__begin, __end, __comp, __pivot_rank,
	 _Settings::get().sort_qs_num_samples_preset, __num_threads);

#pragma omp parallel sections num_threads(2)
      {
#pragma omp section
        __parallel_sort_qs_conquer(__begin, __begin + __split,
				   __comp, __num_threads_left);
#pragma omp section
        __parallel_sort_qs_conquer(__begin + __split, __end,
				   __comp, __num_threads - __num_threads_left);
      }
    }


  /** @brief Unbalanced quicksort main call.
   *  @param __begin Begin iterator of input sequence.
   *  @param __end End iterator input sequence, ignored.
   *  @param __comp Comparator.
   *  @param __num_threads Number of threads that are allowed to work on
   *  this part.
   */
  template<typename _RAIter, typename _Compare>
    void
    __parallel_sort_qs(_RAIter __begin, _RAIter __end,
		       _Compare __comp,
		       _ThreadIndex __num_threads)
    {
      _GLIBCXX_CALL(__n)

      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      _DifferenceType __n = __end - __begin;

      // At least one element per processor.
      if (__num_threads > __n)
        __num_threads = static_cast<_ThreadIndex>(__n);

      __parallel_sort_qs_conquer(
        __begin, __begin + __n, __comp, __num_threads);
    }

} //namespace __gnu_parallel

#endif /* _GLIBCXX_PARALLEL_QUICKSORT_H */
