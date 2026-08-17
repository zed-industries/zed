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

/** @file parallel/sort.h
 *  @brief Parallel sorting algorithm switch.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler.

#ifndef _GLIBCXX_PARALLEL_SORT_H
#define _GLIBCXX_PARALLEL_SORT_H 1

#include <parallel/basic_iterator.h>
#include <parallel/features.h>
#include <parallel/parallel.h>

#if _GLIBCXX_PARALLEL_ASSERTIONS
#include <parallel/checkers.h>
#endif

#if _GLIBCXX_MERGESORT
#include <parallel/multiway_mergesort.h>
#endif

#if _GLIBCXX_QUICKSORT
#include <parallel/quicksort.h>
#endif

#if _GLIBCXX_BAL_QUICKSORT
#include <parallel/balanced_quicksort.h>
#endif

namespace __gnu_parallel
{
  //prototype
  template<bool __stable, typename _RAIter,
           typename _Compare, typename _Parallelism>
    void
    __parallel_sort(_RAIter __begin, _RAIter __end,
		    _Compare __comp, _Parallelism __parallelism);
        
  /** 
   *  @brief Choose multiway mergesort, splitting variant at run-time,
   *  for parallel sorting.
   *  @param __begin Begin iterator of input sequence.
   *  @param __end End iterator of input sequence.
   *  @param __comp Comparator.
   *  @tparam __stable Sort stable.
   *  @callgraph 
   */
  template<bool __stable, typename _RAIter, typename _Compare>
    inline void
    __parallel_sort(_RAIter __begin, _RAIter __end,
		    _Compare __comp, multiway_mergesort_tag __parallelism)
    {
      _GLIBCXX_CALL(__end - __begin)

      if(_Settings::get().sort_splitting == EXACT)
	parallel_sort_mwms<__stable, true>
	  (__begin, __end, __comp, __parallelism.__get_num_threads());
      else
	parallel_sort_mwms<__stable, false>
	  (__begin, __end, __comp, __parallelism.__get_num_threads());
    }

  /** 
   *  @brief Choose multiway mergesort with exact splitting,
   *  for parallel sorting.
   *  @param __begin Begin iterator of input sequence.
   *  @param __end End iterator of input sequence.
   *  @param __comp Comparator.
   *  @tparam __stable Sort stable.
   *  @callgraph 
   */
  template<bool __stable, typename _RAIter, typename _Compare>
    inline void
    __parallel_sort(_RAIter __begin, _RAIter __end,
		    _Compare __comp,
		    multiway_mergesort_exact_tag __parallelism)
    {
      _GLIBCXX_CALL(__end - __begin)

      parallel_sort_mwms<__stable, true>
        (__begin, __end, __comp, __parallelism.__get_num_threads());
    }

  /** 
   *  @brief Choose multiway mergesort with splitting by sampling,
   *  for parallel sorting.
   *  @param __begin Begin iterator of input sequence.
   *  @param __end End iterator of input sequence.
   *  @param __comp Comparator.
   *  @tparam __stable Sort stable.
   *  @callgraph 
   */
  template<bool __stable, typename _RAIter, typename _Compare>
    inline void
    __parallel_sort(_RAIter __begin, _RAIter __end,
		    _Compare __comp,
		    multiway_mergesort_sampling_tag __parallelism)
    {
      _GLIBCXX_CALL(__end - __begin)

      parallel_sort_mwms<__stable, false>
      (__begin, __end, __comp, __parallelism.__get_num_threads());
    }

  /**
   *  @brief Choose quicksort for parallel sorting.
   *  @param __begin Begin iterator of input sequence.
   *  @param __end End iterator of input sequence.
   *  @param __comp Comparator.
   *  @tparam __stable Sort stable.
   *  @callgraph 
   */
  template<bool __stable, typename _RAIter, typename _Compare>
    inline void
    __parallel_sort(_RAIter __begin, _RAIter __end,
		    _Compare __comp, quicksort_tag __parallelism)
    {
      _GLIBCXX_CALL(__end - __begin)

      _GLIBCXX_PARALLEL_ASSERT(__stable == false);

      __parallel_sort_qs(__begin, __end, __comp,
			 __parallelism.__get_num_threads());
    }

  /**
   *  @brief Choose balanced quicksort for parallel sorting.
   *  @param __begin Begin iterator of input sequence.
   *  @param __end End iterator of input sequence.
   *  @param __comp Comparator.
   *  @tparam __stable Sort stable.
   *  @callgraph 
   */
   template<bool __stable, typename _RAIter, typename _Compare>
     inline void
     __parallel_sort(_RAIter __begin, _RAIter __end,
		     _Compare __comp, balanced_quicksort_tag __parallelism)
     {
       _GLIBCXX_CALL(__end - __begin)

       _GLIBCXX_PARALLEL_ASSERT(__stable == false);

       __parallel_sort_qsb(__begin, __end, __comp,
			   __parallelism.__get_num_threads());
     }

  /** 
   *  @brief Choose multiway mergesort with exact splitting,
   *  for parallel sorting.
   *  @param __begin Begin iterator of input sequence.
   *  @param __end End iterator of input sequence.
   *  @param __comp Comparator.
   *  @tparam __stable Sort stable.
   *  @callgraph 
   */
  template<bool __stable, typename _RAIter, typename _Compare>
    inline void
    __parallel_sort(_RAIter __begin, _RAIter __end,
		    _Compare __comp, default_parallel_tag __parallelism)
    {
      _GLIBCXX_CALL(__end - __begin)

      __parallel_sort<__stable>
	(__begin, __end, __comp,
	 multiway_mergesort_exact_tag(__parallelism.__get_num_threads()));
    }

  /**
   *  @brief Choose a parallel sorting algorithm.
   *  @param __begin Begin iterator of input sequence.
   *  @param __end End iterator of input sequence.
   *  @param __comp Comparator.
   *  @tparam __stable Sort stable.
   *  @callgraph 
   */
  template<bool __stable, typename _RAIter, typename _Compare>
    inline void
    __parallel_sort(_RAIter __begin, _RAIter __end,
		    _Compare __comp, parallel_tag __parallelism)
    {
      _GLIBCXX_CALL(__end - __begin)
      typedef std::iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;

      if (false) ;
#if _GLIBCXX_MERGESORT
      else if (__stable || _Settings::get().sort_algorithm == MWMS)
        {
          if(_Settings::get().sort_splitting == EXACT)
            parallel_sort_mwms<__stable, true>
              (__begin, __end, __comp, __parallelism.__get_num_threads());
          else
            parallel_sort_mwms<false, false>
              (__begin, __end, __comp, __parallelism.__get_num_threads());
        }
#endif
#if _GLIBCXX_QUICKSORT
      else if (_Settings::get().sort_algorithm == QS)
        __parallel_sort_qs(__begin, __end, __comp,
                           __parallelism.__get_num_threads());
#endif
#if _GLIBCXX_BAL_QUICKSORT
      else if (_Settings::get().sort_algorithm == QS_BALANCED)
        __parallel_sort_qsb(__begin, __end, __comp,
                            __parallelism.__get_num_threads());
#endif
      else
        __gnu_sequential::sort(__begin, __end, __comp);
    }
} // end namespace __gnu_parallel

#endif /* _GLIBCXX_PARALLEL_SORT_H */
