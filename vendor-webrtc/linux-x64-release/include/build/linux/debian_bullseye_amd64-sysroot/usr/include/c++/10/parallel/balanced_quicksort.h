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

/** @file parallel/balanced_quicksort.h
 *  @brief Implementation of a dynamically load-balanced parallel quicksort.
 *
 *  It works in-place and needs only logarithmic extra memory.
 *  The algorithm is similar to the one proposed in
 *
 *  P. Tsigas and Y. Zhang.
 *  A simple, fast parallel implementation of quicksort and
 *  its performance evaluation on SUN enterprise 10000.
 *  In 11th Euromicro Conference on Parallel, Distributed and
 *  Network-Based Processing, page 372, 2003.
 *
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_BALANCED_QUICKSORT_H
#define _GLIBCXX_PARALLEL_BALANCED_QUICKSORT_H 1

#include <parallel/basic_iterator.h>
#include <bits/stl_algo.h>
#include <bits/stl_function.h>

#include <parallel/settings.h>
#include <parallel/partition.h>
#include <parallel/random_number.h>
#include <parallel/queue.h>

#if _GLIBCXX_PARALLEL_ASSERTIONS
#include <parallel/checkers.h>
#ifdef _GLIBCXX_HAVE_UNISTD_H
#include <unistd.h>
#endif
#endif

namespace __gnu_parallel
{
  /** @brief Information local to one thread in the parallel quicksort run. */
  template<typename _RAIter>
    struct _QSBThreadLocal
    {
      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      /** @brief Continuous part of the sequence, described by an
      iterator pair. */
      typedef std::pair<_RAIter, _RAIter> _Piece;

      /** @brief Initial piece to work on. */
      _Piece _M_initial;

      /** @brief Work-stealing queue. */
      _RestrictedBoundedConcurrentQueue<_Piece> _M_leftover_parts;

      /** @brief Number of threads involved in this algorithm. */
      _ThreadIndex _M_num_threads;

      /** @brief Pointer to a counter of elements left over to sort. */
      volatile _DifferenceType* _M_elements_leftover;

      /** @brief The complete sequence to sort. */
      _Piece _M_global;

      /** @brief Constructor.
       *  @param __queue_size size of the work-stealing queue. */
      _QSBThreadLocal(int __queue_size) : _M_leftover_parts(__queue_size) { }
    };

  /** @brief Balanced quicksort divide step.
    *  @param __begin Begin iterator of subsequence.
    *  @param __end End iterator of subsequence.
    *  @param __comp Comparator.
    *  @param __num_threads Number of threads that are allowed to work on
    *  this part.
    *  @pre @c (__end-__begin)>=1 */
  template<typename _RAIter, typename _Compare>
    typename std::iterator_traits<_RAIter>::difference_type
    __qsb_divide(_RAIter __begin, _RAIter __end,
		 _Compare __comp, _ThreadIndex __num_threads)
    {
      _GLIBCXX_PARALLEL_ASSERT(__num_threads > 0);

      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      _RAIter __pivot_pos =
	__median_of_three_iterators(__begin, __begin + (__end - __begin) / 2,
				    __end  - 1, __comp);

#if defined(_GLIBCXX_PARALLEL_ASSERTIONS)
      // Must be in between somewhere.
      _DifferenceType __n = __end - __begin;

      _GLIBCXX_PARALLEL_ASSERT((!__comp(*__pivot_pos, *__begin)
				&& !__comp(*(__begin + __n / 2),
					   *__pivot_pos))
			       || (!__comp(*__pivot_pos, *__begin)
				   && !__comp(*(__end - 1), *__pivot_pos))
			       || (!__comp(*__pivot_pos, *(__begin + __n / 2))
				   && !__comp(*__begin, *__pivot_pos))
			       || (!__comp(*__pivot_pos, *(__begin + __n / 2))
				   && !__comp(*(__end - 1), *__pivot_pos))
			       || (!__comp(*__pivot_pos, *(__end - 1))
				   && !__comp(*__begin, *__pivot_pos))
			       || (!__comp(*__pivot_pos, *(__end - 1))
				   && !__comp(*(__begin + __n / 2),
					      *__pivot_pos)));
#endif

      // Swap pivot value to end.
      if (__pivot_pos != (__end - 1))
	std::iter_swap(__pivot_pos, __end - 1);
      __pivot_pos = __end - 1;

      __gnu_parallel::__binder2nd<_Compare, _ValueType, _ValueType, bool>
	__pred(__comp, *__pivot_pos);

      // Divide, returning __end - __begin - 1 in the worst case.
      _DifferenceType __split_pos = __parallel_partition(__begin, __end - 1,
							 __pred,
							 __num_threads);

      // Swap back pivot to middle.
      std::iter_swap(__begin + __split_pos, __pivot_pos);
      __pivot_pos = __begin + __split_pos;

#if _GLIBCXX_PARALLEL_ASSERTIONS
      _RAIter __r;
      for (__r = __begin; __r != __pivot_pos; ++__r)
	_GLIBCXX_PARALLEL_ASSERT(__comp(*__r, *__pivot_pos));
      for (; __r != __end; ++__r)
	_GLIBCXX_PARALLEL_ASSERT(!__comp(*__r, *__pivot_pos));
#endif

      return __split_pos;
    }

  /** @brief Quicksort conquer step.
    *  @param __tls Array of thread-local storages.
    *  @param __begin Begin iterator of subsequence.
    *  @param __end End iterator of subsequence.
    *  @param __comp Comparator.
    *  @param __iam Number of the thread processing this function.
    *  @param __num_threads
    *          Number of threads that are allowed to work on this part. */
  template<typename _RAIter, typename _Compare>
    void
    __qsb_conquer(_QSBThreadLocal<_RAIter>** __tls,
		  _RAIter __begin, _RAIter __end,
		  _Compare __comp,
		  _ThreadIndex __iam, _ThreadIndex __num_threads,
		  bool __parent_wait)
    {
      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      _DifferenceType __n = __end - __begin;

      if (__num_threads <= 1 || __n <= 1)
	{
          __tls[__iam]->_M_initial.first  = __begin;
          __tls[__iam]->_M_initial.second = __end;

          __qsb_local_sort_with_helping(__tls, __comp, __iam, __parent_wait);

          return;
	}

      // Divide step.
      _DifferenceType __split_pos =
	__qsb_divide(__begin, __end, __comp, __num_threads);

#if _GLIBCXX_PARALLEL_ASSERTIONS
      _GLIBCXX_PARALLEL_ASSERT(0 <= __split_pos &&
                               __split_pos < (__end - __begin));
#endif

      _ThreadIndex
	__num_threads_leftside = std::max<_ThreadIndex>
	(1, std::min<_ThreadIndex>(__num_threads - 1, __split_pos
				   * __num_threads / __n));

#     pragma omp atomic
      *__tls[__iam]->_M_elements_leftover -= (_DifferenceType)1;

      // Conquer step.
#     pragma omp parallel num_threads(2)
      {
	bool __wait;
	if(omp_get_num_threads() < 2)
          __wait = false;
	else
          __wait = __parent_wait;

#       pragma omp sections
	{
#         pragma omp section
	  {
	    __qsb_conquer(__tls, __begin, __begin + __split_pos, __comp,
			  __iam, __num_threads_leftside, __wait);
	    __wait = __parent_wait;
	  }
	  // The pivot_pos is left in place, to ensure termination.
#         pragma omp section
	  {
	    __qsb_conquer(__tls, __begin + __split_pos + 1, __end, __comp,
			  __iam + __num_threads_leftside,
			  __num_threads - __num_threads_leftside, __wait);
	    __wait = __parent_wait;
	  }
	}
      }
    }

  /**
    *  @brief Quicksort step doing load-balanced local sort.
    *  @param __tls Array of thread-local storages.
    *  @param __comp Comparator.
    *  @param __iam Number of the thread processing this function.
    */
  template<typename _RAIter, typename _Compare>
    void
    __qsb_local_sort_with_helping(_QSBThreadLocal<_RAIter>** __tls,
				  _Compare& __comp, _ThreadIndex __iam,
				  bool __wait)
    {
      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;
      typedef std::pair<_RAIter, _RAIter> _Piece;

      _QSBThreadLocal<_RAIter>& __tl = *__tls[__iam];

      _DifferenceType
	__base_case_n = _Settings::get().sort_qsb_base_case_maximal_n;
      if (__base_case_n < 2)
	__base_case_n = 2;
      _ThreadIndex __num_threads = __tl._M_num_threads;

      // Every thread has its own random number generator.
      _RandomNumber __rng(__iam + 1);

      _Piece __current = __tl._M_initial;

      _DifferenceType __elements_done = 0;
#if _GLIBCXX_PARALLEL_ASSERTIONS
      _DifferenceType __total_elements_done = 0;
#endif

      for (;;)
	{
          // Invariant: __current must be a valid (maybe empty) range.
          _RAIter __begin = __current.first, __end = __current.second;
          _DifferenceType __n = __end - __begin;

          if (__n > __base_case_n)
            {
              // Divide.
              _RAIter __pivot_pos = __begin +  __rng(__n);

              // Swap __pivot_pos value to end.
              if (__pivot_pos != (__end - 1))
        	std::iter_swap(__pivot_pos, __end - 1);
              __pivot_pos = __end - 1;

              __gnu_parallel::__binder2nd
		<_Compare, _ValueType, _ValueType, bool>
		__pred(__comp, *__pivot_pos);

              // Divide, leave pivot unchanged in last place.
              _RAIter __split_pos1, __split_pos2;
              __split_pos1 = __gnu_sequential::partition(__begin, __end - 1,
							 __pred);

              // Left side: < __pivot_pos; __right side: >= __pivot_pos.
#if _GLIBCXX_PARALLEL_ASSERTIONS
              _GLIBCXX_PARALLEL_ASSERT(__begin <= __split_pos1
                                       && __split_pos1 < __end);
#endif
              // Swap pivot back to middle.
              if (__split_pos1 != __pivot_pos)
        	std::iter_swap(__split_pos1, __pivot_pos);
              __pivot_pos = __split_pos1;

              // In case all elements are equal, __split_pos1 == 0.
              if ((__split_pos1 + 1 - __begin) < (__n >> 7)
		  || (__end - __split_pos1) < (__n >> 7))
        	{
                  // Very unequal split, one part smaller than one 128th
                  // elements not strictly larger than the pivot.
                  __gnu_parallel::__unary_negate<__gnu_parallel::__binder1st
                    <_Compare, _ValueType, _ValueType, bool>, _ValueType>
                    __pred(__gnu_parallel::__binder1st
                	 <_Compare, _ValueType, _ValueType, bool>
			   (__comp, *__pivot_pos));

                  // Find other end of pivot-equal range.
                  __split_pos2 = __gnu_sequential::partition(__split_pos1 + 1,
							     __end, __pred);
        	}
              else
        	// Only skip the pivot.
        	__split_pos2 = __split_pos1 + 1;

              // Elements equal to pivot are done.
              __elements_done += (__split_pos2 - __split_pos1);
#if _GLIBCXX_PARALLEL_ASSERTIONS
              __total_elements_done += (__split_pos2 - __split_pos1);
#endif
              // Always push larger part onto stack.
              if (((__split_pos1 + 1) - __begin) < (__end - (__split_pos2)))
        	{
                  // Right side larger.
                  if ((__split_pos2) != __end)
                    __tl._M_leftover_parts.push_front
		      (std::make_pair(__split_pos2, __end));

                  //__current.first = __begin;    //already set anyway
                  __current.second = __split_pos1;
                  continue;
        	}
              else
        	{
                  // Left side larger.
                  if (__begin != __split_pos1)
                    __tl._M_leftover_parts.push_front(std::make_pair
						      (__begin, __split_pos1));

                  __current.first = __split_pos2;
                  //__current.second = __end;     //already set anyway
                  continue;
        	}
            }
          else
            {
              __gnu_sequential::sort(__begin, __end, __comp);
              __elements_done += __n;
#if _GLIBCXX_PARALLEL_ASSERTIONS
              __total_elements_done += __n;
#endif

              // Prefer own stack, small pieces.
              if (__tl._M_leftover_parts.pop_front(__current))
        	continue;

#             pragma omp atomic
              *__tl._M_elements_leftover -= __elements_done;

              __elements_done = 0;

#if _GLIBCXX_PARALLEL_ASSERTIONS
              double __search_start = omp_get_wtime();
#endif

              // Look for new work.
              bool __successfully_stolen = false;
              while (__wait && *__tl._M_elements_leftover > 0
                     && !__successfully_stolen
#if _GLIBCXX_PARALLEL_ASSERTIONS
                      // Possible dead-lock.
                     && (omp_get_wtime() < (__search_start + 1.0))
#endif
		     )
        	{
                  _ThreadIndex __victim;
                  __victim = __rng(__num_threads);

                  // Large pieces.
                  __successfully_stolen = (__victim != __iam)
		    && __tls[__victim]->_M_leftover_parts.pop_back(__current);
                  if (!__successfully_stolen)
                    __yield();
#if !defined(__ICC) && !defined(__ECC)
#                 pragma omp flush
#endif
        	}

#if _GLIBCXX_PARALLEL_ASSERTIONS
              if (omp_get_wtime() >= (__search_start + 1.0))
        	{
                  sleep(1);
                  _GLIBCXX_PARALLEL_ASSERT(omp_get_wtime()
                                           < (__search_start + 1.0));
        	}
#endif
              if (!__successfully_stolen)
        	{
#if _GLIBCXX_PARALLEL_ASSERTIONS
                  _GLIBCXX_PARALLEL_ASSERT(*__tl._M_elements_leftover == 0);
#endif
                  return;
        	}
            }
	}
    }

  /** @brief Top-level quicksort routine.
    *  @param __begin Begin iterator of sequence.
    *  @param __end End iterator of sequence.
    *  @param __comp Comparator.
    *  @param __num_threads Number of threads that are allowed to work on
    *  this part.
    */
  template<typename _RAIter, typename _Compare>
    void
    __parallel_sort_qsb(_RAIter __begin, _RAIter __end,
			_Compare __comp, _ThreadIndex __num_threads)
    {
      _GLIBCXX_CALL(__end - __begin)

      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;
      typedef std::pair<_RAIter, _RAIter> _Piece;

      typedef _QSBThreadLocal<_RAIter> _TLSType;

      _DifferenceType __n = __end - __begin;

      if (__n <= 1)
	return;

      // At least one element per processor.
      if (__num_threads > __n)
	__num_threads = static_cast<_ThreadIndex>(__n);

      // Initialize thread local storage
      _TLSType** __tls = new _TLSType*[__num_threads];
      _DifferenceType __queue_size = (__num_threads
				      * (_ThreadIndex)(__rd_log2(__n) + 1));
      for (_ThreadIndex __t = 0; __t < __num_threads; ++__t)
	__tls[__t] = new _QSBThreadLocal<_RAIter>(__queue_size);

      // There can never be more than ceil(__rd_log2(__n)) ranges on the
      // stack, because
      // 1. Only one processor pushes onto the stack
      // 2. The largest range has at most length __n
      // 3. Each range is larger than half of the range remaining
      volatile _DifferenceType __elements_leftover = __n;
      for (_ThreadIndex __i = 0; __i < __num_threads; ++__i)
	{
          __tls[__i]->_M_elements_leftover = &__elements_leftover;
          __tls[__i]->_M_num_threads = __num_threads;
          __tls[__i]->_M_global = std::make_pair(__begin, __end);

          // Just in case nothing is left to assign.
          __tls[__i]->_M_initial = std::make_pair(__end, __end);
	}

      // Main recursion call.
      __qsb_conquer(__tls, __begin, __begin + __n, __comp, 0,
		    __num_threads, true);

#if _GLIBCXX_PARALLEL_ASSERTIONS
      // All stack must be empty.
      _Piece __dummy;
      for (_ThreadIndex __i = 1; __i < __num_threads; ++__i)
	_GLIBCXX_PARALLEL_ASSERT(
          !__tls[__i]->_M_leftover_parts.pop_back(__dummy));
#endif

      for (_ThreadIndex __i = 0; __i < __num_threads; ++__i)
	delete __tls[__i];
      delete[] __tls;
    }
} // namespace __gnu_parallel

#endif /* _GLIBCXX_PARALLEL_BALANCED_QUICKSORT_H */
