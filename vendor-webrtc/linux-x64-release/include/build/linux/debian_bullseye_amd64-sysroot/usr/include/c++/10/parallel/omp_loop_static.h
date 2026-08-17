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

/** @file parallel/omp_loop_static.h
 *  @brief Parallelization of embarrassingly parallel execution by
 *  means of an OpenMP for loop with static scheduling.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Felix Putze.

#ifndef _GLIBCXX_PARALLEL_OMP_LOOP_STATIC_H
#define _GLIBCXX_PARALLEL_OMP_LOOP_STATIC_H 1

#include <omp.h>

#include <parallel/settings.h>
#include <parallel/basic_iterator.h>

namespace __gnu_parallel
{
  /** @brief Embarrassingly parallel algorithm for random access
   * iterators, using an OpenMP for loop with static scheduling.
   *
   *  @param __begin Begin iterator of element sequence.
   *  @param __end End iterator of element sequence.
   *  @param __o User-supplied functor (comparator, predicate, adding
   *  functor, ...).
   *  @param __f Functor to @a process an element with __op (depends on
   *  desired functionality, e. g. for std::for_each(), ...).
   *  @param __r Functor to @a add a single __result to the already processed
   *  __elements (depends on functionality).
   *  @param __base Base value for reduction.
   *  @param __output Pointer to position where final result is written to
   *  @param __bound Maximum number of elements processed (e. g. for
   *  std::count_n()).
   *  @return User-supplied functor (that may contain a part of the result).
   */
  template<typename _RAIter,
	   typename _Op,
	   typename _Fu,
	   typename _Red,
	   typename _Result>
    _Op
    __for_each_template_random_access_omp_loop_static(_RAIter __begin,
						      _RAIter __end, _Op __o,
						      _Fu& __f, _Red __r,
						      _Result __base,
						      _Result& __output,
      typename std::iterator_traits<_RAIter>::difference_type __bound)
    {
      typedef typename std::iterator_traits<_RAIter>::difference_type
	_DifferenceType;

      _DifferenceType __length = __end - __begin;
      _ThreadIndex __num_threads = std::min<_DifferenceType>
	(__get_max_threads(), __length);

      _Result *__thread_results;

#     pragma omp parallel num_threads(__num_threads)
      {
#       pragma omp single
	{
	  __num_threads = omp_get_num_threads();
	  __thread_results = new _Result[__num_threads];

	  for (_ThreadIndex __i = 0; __i < __num_threads; ++__i)
	    __thread_results[__i] = _Result();
	}

        _ThreadIndex __iam = omp_get_thread_num();

#pragma omp for schedule(static, _Settings::get().workstealing_chunk_size)
        for (_DifferenceType __pos = 0; __pos < __length; ++__pos)
          __thread_results[__iam] = __r(__thread_results[__iam],
                                        __f(__o, __begin+__pos));
      } //parallel

      for (_ThreadIndex __i = 0; __i < __num_threads; ++__i)
	__output = __r(__output, __thread_results[__i]);

      delete [] __thread_results;

      // Points to last element processed (needed as return value for
      // some algorithms like transform).
      __f.finish_iterator = __begin + __length;

      return __o;
    }

} // end namespace

#endif /* _GLIBCXX_PARALLEL_OMP_LOOP_STATIC_H */
