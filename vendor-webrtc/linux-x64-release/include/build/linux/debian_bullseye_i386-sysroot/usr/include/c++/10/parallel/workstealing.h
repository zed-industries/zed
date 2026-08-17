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

/** @file parallel/workstealing.h
 *  @brief Parallelization of embarrassingly parallel execution by
 *  means of work-stealing.
 *
 *  Work stealing is described in
 *
 *  R. D. Blumofe and C. E. Leiserson.
 *  Scheduling multithreaded computations by work stealing.
 *  Journal of the ACM, 46(5):720-748, 1999.
 *
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Felix Putze.

#ifndef _GLIBCXX_PARALLEL_WORKSTEALING_H
#define _GLIBCXX_PARALLEL_WORKSTEALING_H 1

#include <parallel/parallel.h>
#include <parallel/random_number.h>
#include <parallel/compatibility.h>

namespace __gnu_parallel
{

#define _GLIBCXX_JOB_VOLATILE volatile

  /** @brief One __job for a certain thread. */
  template<typename _DifferenceTp>
    struct _Job
    {
      typedef _DifferenceTp _DifferenceType;

      /** @brief First element.
       *
       *  Changed by owning and stealing thread. By stealing thread,
       *  always incremented. */
      _GLIBCXX_JOB_VOLATILE _DifferenceType _M_first;

      /** @brief Last element.
       *
       *  Changed by owning thread only. */
      _GLIBCXX_JOB_VOLATILE _DifferenceType _M_last;

      /** @brief Number of elements, i.e. @c _M_last-_M_first+1.
       *
       *  Changed by owning thread only. */
      _GLIBCXX_JOB_VOLATILE _DifferenceType _M_load;
    };

  /** @brief Work stealing algorithm for random access iterators.
    *
    *  Uses O(1) additional memory. Synchronization at job lists is
    *  done with atomic operations.
    *  @param __begin Begin iterator of element sequence.
    *  @param __end End iterator of element sequence.
    *  @param __op User-supplied functor (comparator, predicate, adding
    *  functor, ...).
    *  @param __f Functor to @a process an element with __op (depends on
    *  desired functionality, e. g. for std::for_each(), ...).
    *  @param __r Functor to @a add a single __result to the already
    *  processed elements (depends on functionality).
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
    __for_each_template_random_access_workstealing(_RAIter __begin,
						   _RAIter __end, _Op __op,
						   _Fu& __f, _Red __r,
						   _Result __base,
						   _Result& __output,
      typename std::iterator_traits<_RAIter>::difference_type __bound)
    {
      _GLIBCXX_CALL(__end - __begin)

      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      const _Settings& __s = _Settings::get();

      _DifferenceType __chunk_size =
          static_cast<_DifferenceType>(__s.workstealing_chunk_size);

      // How many jobs?
      _DifferenceType __length = (__bound < 0) ? (__end - __begin) : __bound;

      // To avoid false sharing in a cache line.
      const int __stride = (__s.cache_line_size * 10
			    / sizeof(_Job<_DifferenceType>) + 1);

      // Total number of threads currently working.
      _ThreadIndex __busy = 0;

      _Job<_DifferenceType> *__job;

      omp_lock_t __output_lock;
      omp_init_lock(&__output_lock);

      // Write base value to output.
      __output = __base;

      // No more threads than jobs, at least one thread.
      _ThreadIndex __num_threads = __gnu_parallel::max<_ThreadIndex>
	(1, __gnu_parallel::min<_DifferenceType>(__length,
						 __get_max_threads()));

#     pragma omp parallel shared(__busy) num_threads(__num_threads)
      {
#       pragma omp single
	{
	  __num_threads = omp_get_num_threads();

	  // Create job description array.
	  __job = new _Job<_DifferenceType>[__num_threads * __stride];
	}

	// Initialization phase.

	// Flags for every thread if it is doing productive work.
	bool __iam_working = false;

	// Thread id.
	_ThreadIndex __iam = omp_get_thread_num();

	// This job.
	_Job<_DifferenceType>& __my_job = __job[__iam * __stride];

	// Random number (for work stealing).
	_ThreadIndex __victim;

	// Local value for reduction.
	_Result __result = _Result();

	// Number of elements to steal in one attempt.
	_DifferenceType __steal;

	// Every thread has its own random number generator
	// (modulo __num_threads).
	_RandomNumber __rand_gen(__iam, __num_threads);

	// This thread is currently working.
#       pragma omp atomic
	++__busy;

	__iam_working = true;

	// How many jobs per thread? last thread gets the rest.
	__my_job._M_first = static_cast<_DifferenceType>
	  (__iam * (__length / __num_threads));

	__my_job._M_last = (__iam == (__num_threads - 1)
			    ? (__length - 1)
			    : ((__iam + 1) * (__length / __num_threads) - 1));
	__my_job._M_load = __my_job._M_last - __my_job._M_first + 1;

	// Init result with _M_first value (to have a base value for reduction)
	if (__my_job._M_first <= __my_job._M_last)
	  {
	    // Cannot use volatile variable directly.
	    _DifferenceType __my_first = __my_job._M_first;
	    __result = __f(__op, __begin + __my_first);
	    ++__my_job._M_first;
	    --__my_job._M_load;
	  }

	_RAIter __current;

#       pragma omp barrier

	// Actual work phase
	// Work on own or stolen current start
	while (__busy > 0)
	  {
	    // Work until no productive thread left.
#           pragma omp flush(__busy)

	    // Thread has own work to do
	    while (__my_job._M_first <= __my_job._M_last)
	      {
		// fetch-and-add call
		// Reserve current job block (size __chunk_size) in my queue.
		_DifferenceType __current_job =
		  __fetch_and_add<_DifferenceType>(&(__my_job._M_first),
						   __chunk_size);

		// Update _M_load, to make the three values consistent,
		// _M_first might have been changed in the meantime
		__my_job._M_load = __my_job._M_last - __my_job._M_first + 1;
		for (_DifferenceType __job_counter = 0;
		     __job_counter < __chunk_size
		       && __current_job <= __my_job._M_last;
		     ++__job_counter)
		  {
		    // Yes: process it!
		    __current = __begin + __current_job;
		    ++__current_job;

		    // Do actual work.
		    __result = __r(__result, __f(__op, __current));
		  }

#               pragma omp flush(__busy)
	      }

	    // After reaching this point, a thread's __job list is empty.
	    if (__iam_working)
	      {
		// This thread no longer has work.
#               pragma omp atomic
		--__busy;

		__iam_working = false;
	      }

	    _DifferenceType __supposed_first, __supposed_last,
	                    __supposed_load;
	    do
	      {
		// Find random nonempty deque (not own), do consistency check.
		__yield();
#               pragma omp flush(__busy)
		__victim = __rand_gen();
		__supposed_first = __job[__victim * __stride]._M_first;
		__supposed_last = __job[__victim * __stride]._M_last;
		__supposed_load = __job[__victim * __stride]._M_load;
	      }
	    while (__busy > 0
		   && ((__supposed_load <= 0)
		       || ((__supposed_first + __supposed_load - 1)
			   != __supposed_last)));

	    if (__busy == 0)
	      break;

	    if (__supposed_load > 0)
	      {
		// Has work and work to do.
		// Number of elements to steal (at least one).
		__steal = (__supposed_load < 2) ? 1 : __supposed_load / 2;

		// Push __victim's current start forward.
		_DifferenceType __stolen_first =
		  __fetch_and_add<_DifferenceType>
		  (&(__job[__victim * __stride]._M_first), __steal);
		_DifferenceType __stolen_try = (__stolen_first + __steal
						- _DifferenceType(1));

		__my_job._M_first = __stolen_first;
		__my_job._M_last = __gnu_parallel::min(__stolen_try,
						       __supposed_last);
		__my_job._M_load = __my_job._M_last - __my_job._M_first + 1;

		// Has potential work again.
#               pragma omp atomic
		++__busy;
		__iam_working = true;

#               pragma omp flush(__busy)
	      }
#           pragma omp flush(__busy)
	  } // end while __busy > 0
	// Add accumulated result to output.
	omp_set_lock(&__output_lock);
	__output = __r(__output, __result);
	omp_unset_lock(&__output_lock);
      }

      delete[] __job;

      // Points to last element processed (needed as return value for
      // some algorithms like transform)
      __f._M_finish_iterator = __begin + __length;

      omp_destroy_lock(&__output_lock);

      return __op;
    }
} // end namespace

#endif /* _GLIBCXX_PARALLEL_WORKSTEALING_H */
