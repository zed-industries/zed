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

/** @file parallel/partial_sum.h
 *  @brief Parallel implementation of std::partial_sum(), i.e. prefix
*  sums.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_PARTIAL_SUM_H
#define _GLIBCXX_PARALLEL_PARTIAL_SUM_H 1

#include <omp.h>
#include <new>
#include <bits/stl_algobase.h>
#include <parallel/parallel.h>
#include <parallel/numericfwd.h>

namespace __gnu_parallel
{
  // Problem: there is no 0-element given.

  /** @brief Base case prefix sum routine.
   *  @param __begin Begin iterator of input sequence.
   *  @param __end End iterator of input sequence.
   *  @param __result Begin iterator of output sequence.
   *  @param __bin_op Associative binary function.
   *  @param __value Start value. Must be passed since the neutral
   *  element is unknown in general.
   *  @return End iterator of output sequence. */
  template<typename _IIter,
	   typename _OutputIterator,
	   typename _BinaryOperation>
    _OutputIterator
    __parallel_partial_sum_basecase(_IIter __begin, _IIter __end,
				    _OutputIterator __result,
				    _BinaryOperation __bin_op,
      typename std::iterator_traits <_IIter>::value_type __value)
    {
      if (__begin == __end)
	return __result;

      while (__begin != __end)
	{
	  __value = __bin_op(__value, *__begin);
	  *__result = __value;
	  ++__result;
	  ++__begin;
	}
      return __result;
    }

  /** @brief Parallel partial sum implementation, two-phase approach,
      no recursion.
      *  @param __begin Begin iterator of input sequence.
      *  @param __end End iterator of input sequence.
      *  @param __result Begin iterator of output sequence.
      *  @param __bin_op Associative binary function.
      *  @param __n Length of sequence.
      *  @return End iterator of output sequence.
      */
  template<typename _IIter,
	   typename _OutputIterator,
	   typename _BinaryOperation>
    _OutputIterator
    __parallel_partial_sum_linear(_IIter __begin, _IIter __end,
				  _OutputIterator __result,
				  _BinaryOperation __bin_op,
      typename std::iterator_traits<_IIter>::difference_type __n)
    {
      typedef std::iterator_traits<_IIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      if (__begin == __end)
	return __result;

      _ThreadIndex __num_threads =
        std::min<_DifferenceType>(__get_max_threads(), __n - 1);

      if (__num_threads < 2)
	{
	  *__result = *__begin;
	  return __parallel_partial_sum_basecase(__begin + 1, __end,
						 __result + 1, __bin_op,
						 *__begin);
	}

      _DifferenceType* __borders;
      _ValueType* __sums;

      const _Settings& __s = _Settings::get();

#     pragma omp parallel num_threads(__num_threads)
      {
#       pragma omp single
	{
	  __num_threads = omp_get_num_threads();
	    
	  __borders = new _DifferenceType[__num_threads + 2];

	  if (__s.partial_sum_dilation == 1.0f)
	    __equally_split(__n, __num_threads + 1, __borders);
	  else
	    {
	      _DifferenceType __first_part_length =
		  std::max<_DifferenceType>(1,
		    __n / (1.0f + __s.partial_sum_dilation * __num_threads));
	      _DifferenceType __chunk_length =
		  (__n - __first_part_length) / __num_threads;
	      _DifferenceType __borderstart =
		  __n - __num_threads * __chunk_length;
	      __borders[0] = 0;
	      for (_ThreadIndex __i = 1; __i < (__num_threads + 1); ++__i)
		{
		  __borders[__i] = __borderstart;
		  __borderstart += __chunk_length;
		}
	      __borders[__num_threads + 1] = __n;
	    }

	  __sums = static_cast<_ValueType*>(::operator new(sizeof(_ValueType)
                                                           * __num_threads));
	  _OutputIterator __target_end;
	} //single

        _ThreadIndex __iam = omp_get_thread_num();
        if (__iam == 0)
          {
            *__result = *__begin;
            __parallel_partial_sum_basecase(__begin + 1,
					    __begin + __borders[1],
					    __result + 1,
					    __bin_op, *__begin);
            ::new(&(__sums[__iam])) _ValueType(*(__result + __borders[1] - 1));
          }
        else
          {
            ::new(&(__sums[__iam]))
              _ValueType(__gnu_parallel::accumulate(
                                         __begin + __borders[__iam] + 1,
                                         __begin + __borders[__iam + 1],
                                         *(__begin + __borders[__iam]),
                                         __bin_op,
                                         __gnu_parallel::sequential_tag()));
          }

#       pragma omp barrier

#       pragma omp single
	__parallel_partial_sum_basecase(__sums + 1, __sums + __num_threads,
					__sums + 1, __bin_op, __sums[0]);

#       pragma omp barrier

	// Still same team.
        __parallel_partial_sum_basecase(__begin + __borders[__iam + 1],
					__begin + __borders[__iam + 2],
					__result + __borders[__iam + 1],
					__bin_op, __sums[__iam]);
      } //parallel

      for (_ThreadIndex __i = 0; __i < __num_threads; ++__i)
	__sums[__i].~_ValueType();
      ::operator delete(__sums);

      delete[] __borders;

      return __result + __n;
    }

  /** @brief Parallel partial sum front-__end.
   *  @param __begin Begin iterator of input sequence.
   *  @param __end End iterator of input sequence.
   *  @param __result Begin iterator of output sequence.
   *  @param __bin_op Associative binary function.
   *  @return End iterator of output sequence. */
  template<typename _IIter,
	   typename _OutputIterator,
	   typename _BinaryOperation>
    _OutputIterator
    __parallel_partial_sum(_IIter __begin, _IIter __end,
			   _OutputIterator __result, _BinaryOperation __bin_op)
    {
      _GLIBCXX_CALL(__begin - __end)

      typedef std::iterator_traits<_IIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      _DifferenceType __n = __end - __begin;

      switch (_Settings::get().partial_sum_algorithm)
	{
	case LINEAR:
	  // Need an initial offset.
	  return __parallel_partial_sum_linear(__begin, __end, __result,
					       __bin_op, __n);
	default:
	  // Partial_sum algorithm not implemented.
	  _GLIBCXX_PARALLEL_ASSERT(0);
	  return __result + __n;
	}
    }
}

#endif /* _GLIBCXX_PARALLEL_PARTIAL_SUM_H */
