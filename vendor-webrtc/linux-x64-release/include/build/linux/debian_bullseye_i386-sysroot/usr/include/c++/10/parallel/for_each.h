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

/** @file parallel/for_each.h
 *  @brief Main interface for embarrassingly parallel functions.
 *
 *  The explicit implementation are in other header files, like
 *  workstealing.h, par_loop.h, omp_loop.h, and omp_loop_static.h.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Felix Putze.

#ifndef _GLIBCXX_PARALLEL_FOR_EACH_H
#define _GLIBCXX_PARALLEL_FOR_EACH_H 1

#include <parallel/settings.h>
#include <parallel/par_loop.h>
#include <parallel/omp_loop.h>
#include <parallel/workstealing.h>

namespace __gnu_parallel
{
  /** @brief Chose the desired algorithm by evaluating @c __parallelism_tag.
   *  @param __begin Begin iterator of input sequence.
   *  @param __end End iterator of input sequence.
   *  @param __user_op A user-specified functor (comparator, predicate,
   *  associative operator,...)
   *  @param __functionality functor to @a process an element with
   *  __user_op (depends on desired functionality, e. g. accumulate,
   *  for_each,...
   *  @param __reduction Reduction functor.
   *  @param __reduction_start Initial value for reduction.
   *  @param __output Output iterator.
   *  @param __bound Maximum number of elements processed.
   *  @param __parallelism_tag Parallelization method */
  template<typename _IIter, typename _UserOp,
           typename _Functionality, typename _Red, typename _Result>
    _UserOp
    __for_each_template_random_access(_IIter __begin, _IIter __end,
                                      _UserOp __user_op,
                                      _Functionality& __functionality,
                                      _Red __reduction,
                                      _Result __reduction_start,
                                      _Result& __output, typename
                                      std::iterator_traits<_IIter>::
                                      difference_type __bound,
                                      _Parallelism __parallelism_tag)
    {
      if (__parallelism_tag == parallel_unbalanced)
        return __for_each_template_random_access_ed
	  (__begin, __end, __user_op, __functionality, __reduction,
	   __reduction_start, __output, __bound);
      else if (__parallelism_tag == parallel_omp_loop)
        return __for_each_template_random_access_omp_loop
	  (__begin, __end, __user_op, __functionality, __reduction,
	   __reduction_start, __output, __bound);
      else if (__parallelism_tag == parallel_omp_loop_static)
        return __for_each_template_random_access_omp_loop
	  (__begin, __end, __user_op, __functionality, __reduction,
	   __reduction_start, __output, __bound);
      else      //e. g. parallel_balanced
        return __for_each_template_random_access_workstealing
	  (__begin, __end, __user_op, __functionality, __reduction,
	   __reduction_start, __output, __bound);
  }
}

#endif /* _GLIBCXX_PARALLEL_FOR_EACH_H */
