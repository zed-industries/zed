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

/** @file parallel/unique_copy.h
 *  @brief Parallel implementations of std::unique_copy().
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Robert Geisberger and Robin Dapp.

#ifndef _GLIBCXX_PARALLEL_UNIQUE_COPY_H
#define _GLIBCXX_PARALLEL_UNIQUE_COPY_H 1

#include <parallel/parallel.h>
#include <parallel/multiseq_selection.h>

namespace __gnu_parallel
{
  /** @brief Parallel std::unique_copy(), w/__o explicit equality predicate.
    *  @param __first Begin iterator of input sequence.
    *  @param __last End iterator of input sequence.
    *  @param __result Begin iterator of result __sequence.
    *  @param __binary_pred Equality predicate.
    *  @return End iterator of result __sequence. */
  template<typename _IIter,
           class _OutputIterator,
           class _BinaryPredicate>
    _OutputIterator
    __parallel_unique_copy(_IIter __first, _IIter __last,
			   _OutputIterator __result,
			   _BinaryPredicate __binary_pred)
    {
      _GLIBCXX_CALL(__last - __first)

      typedef std::iterator_traits<_IIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      _DifferenceType __size = __last - __first;

      if (__size == 0)
	return __result;

      // Let the first thread process two parts.
      _DifferenceType *__counter;
      _DifferenceType *__borders;

      _ThreadIndex __num_threads = __get_max_threads();
      // First part contains at least one element.
#     pragma omp parallel num_threads(__num_threads)
      {
#       pragma omp single
	{
	  __num_threads = omp_get_num_threads();
	  __borders = new _DifferenceType[__num_threads + 2];
	  __equally_split(__size, __num_threads + 1, __borders);
	  __counter = new _DifferenceType[__num_threads + 1];
	}

	_ThreadIndex __iam = omp_get_thread_num();

	_DifferenceType __begin, __end;

	// Check for length without duplicates
	// Needed for position in output
	_DifferenceType __i = 0;
	_OutputIterator __out = __result;

	if (__iam == 0)
          {
            __begin = __borders[0] + 1;   // == 1
            __end = __borders[__iam + 1];

            ++__i;
            *__out++ = *__first;

            for (_IIter __iter = __first + __begin; __iter < __first + __end;
		 ++__iter)
              {
        	if (!__binary_pred(*__iter, *(__iter - 1)))
                  {
                    ++__i;
                    *__out++ = *__iter;
                  }
              }
          }
	else
          {
            __begin = __borders[__iam]; //one part
            __end = __borders[__iam + 1];

            for (_IIter __iter = __first + __begin; __iter < __first + __end;
		 ++__iter)
              {
        	if (!__binary_pred(*__iter, *(__iter - 1)))
                  ++__i;
              }
          }
	__counter[__iam] = __i;

	// Last part still untouched.
	_DifferenceType __begin_output;

#       pragma omp barrier

	// Store result in output on calculated positions.
	__begin_output = 0;

	if (__iam == 0)
          {
            for (_ThreadIndex __t = 0; __t < __num_threads; ++__t)
              __begin_output += __counter[__t];

            __i = 0;

            _OutputIterator __iter_out = __result + __begin_output;

            __begin = __borders[__num_threads];
            __end = __size;

            for (_IIter __iter = __first + __begin; __iter < __first + __end;
		 ++__iter)
              {
        	if (__iter == __first
		    || !__binary_pred(*__iter, *(__iter - 1)))
                  {
                    ++__i;
                    *__iter_out++ = *__iter;
                  }
              }

            __counter[__num_threads] = __i;
          }
	else
          {
            for (_ThreadIndex __t = 0; __t < __iam; __t++)
              __begin_output += __counter[__t];

            _OutputIterator __iter_out = __result + __begin_output;
            for (_IIter __iter = __first + __begin; __iter < __first + __end;
		 ++__iter)
              {
        	if (!__binary_pred(*__iter, *(__iter - 1)))
                  *__iter_out++ = *__iter;
              }
          }
      }

      _DifferenceType __end_output = 0;
      for (_ThreadIndex __t = 0; __t < __num_threads + 1; __t++)
	__end_output += __counter[__t];

      delete[] __borders;

      return __result + __end_output;
    }

  /** @brief Parallel std::unique_copy(), without explicit equality predicate
    *  @param __first Begin iterator of input sequence.
    *  @param __last End iterator of input sequence.
    *  @param __result Begin iterator of result __sequence.
    *  @return End iterator of result __sequence. */
  template<typename _IIter, class _OutputIterator>
    inline _OutputIterator
    __parallel_unique_copy(_IIter __first, _IIter __last,
			   _OutputIterator __result)
    {
      typedef typename std::iterator_traits<_IIter>::value_type
	_ValueType;
      return __parallel_unique_copy(__first, __last, __result,
				    std::equal_to<_ValueType>());
    }

}//namespace __gnu_parallel

#endif /* _GLIBCXX_PARALLEL_UNIQUE_COPY_H */
