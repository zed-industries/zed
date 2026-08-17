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

/** @file parallel/find_selectors.h
 *  @brief _Function objects representing different tasks to be plugged
 *  into the parallel find algorithm.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Felix Putze.

#ifndef _GLIBCXX_PARALLEL_FIND_SELECTORS_H
#define _GLIBCXX_PARALLEL_FIND_SELECTORS_H 1

#include <parallel/tags.h>
#include <parallel/basic_iterator.h>
#include <bits/stl_pair.h>

namespace __gnu_parallel
{
  /** @brief Base class of all __gnu_parallel::__find_template selectors. */
  struct __generic_find_selector
  { };

  /** 
   *  @brief Test predicate on a single element, used for std::find()
   *  and std::find_if ().
   */
  struct __find_if_selector : public __generic_find_selector
  {
    /** @brief Test on one position.
     * @param __i1 _Iterator on first sequence.
     * @param __i2 _Iterator on second sequence (unused).
     * @param __pred Find predicate.
     */
    template<typename _RAIter1, typename _RAIter2,
             typename _Pred>
      bool 
      operator()(_RAIter1 __i1, _RAIter2 __i2, _Pred __pred)
      { return __pred(*__i1); }

    /** @brief Corresponding sequential algorithm on a sequence.
     *  @param __begin1 Begin iterator of first sequence.
     *  @param __end1 End iterator of first sequence.
     *  @param __begin2 Begin iterator of second sequence.
     *  @param __pred Find predicate.
     */
    template<typename _RAIter1, typename _RAIter2,
             typename _Pred>
      std::pair<_RAIter1, _RAIter2> 
      _M_sequential_algorithm(_RAIter1 __begin1,
                           _RAIter1 __end1,
                           _RAIter2 __begin2, _Pred __pred)
      { return std::make_pair(find_if(__begin1, __end1, __pred,
                                      sequential_tag()), __begin2); }
  };

  /** @brief Test predicate on two adjacent elements. */
  struct __adjacent_find_selector : public __generic_find_selector
  {
    /** @brief Test on one position.
     *  @param __i1 _Iterator on first sequence.
     *  @param __i2 _Iterator on second sequence (unused).
     *  @param __pred Find predicate.
     */
    template<typename _RAIter1, typename _RAIter2,
             typename _Pred>
      bool 
      operator()(_RAIter1 __i1, _RAIter2 __i2, _Pred __pred)
      {
        // Passed end iterator is one short.
        return __pred(*__i1, *(__i1 + 1));
      }

    /** @brief Corresponding sequential algorithm on a sequence.
     *  @param __begin1 Begin iterator of first sequence.
     *  @param __end1 End iterator of first sequence.
     *  @param __begin2 Begin iterator of second sequence.
     *  @param __pred Find predicate.
     */
    template<typename _RAIter1, typename _RAIter2,
             typename _Pred>
      std::pair<_RAIter1, _RAIter2>
      _M_sequential_algorithm(_RAIter1 __begin1,
			      _RAIter1 __end1,
			      _RAIter2 __begin2, _Pred __pred)
      {
        // Passed end iterator is one short.
        _RAIter1 __spot = adjacent_find(__begin1, __end1 + 1,
					__pred, sequential_tag());
        if (__spot == (__end1 + 1))
          __spot = __end1;
        return std::make_pair(__spot, __begin2);
      }
  };

  /** @brief Test inverted predicate on a single element. */
  struct __mismatch_selector : public __generic_find_selector
  {
    /** 
     *  @brief Test on one position.
     *  @param __i1 _Iterator on first sequence.
     *  @param __i2 _Iterator on second sequence (unused).
     *  @param __pred Find predicate. 
     */
    template<typename _RAIter1, typename _RAIter2,
             typename _Pred>
      bool 
      operator()(_RAIter1 __i1, _RAIter2 __i2, _Pred __pred)
      { return !__pred(*__i1, *__i2); }

    /** 
     *  @brief Corresponding sequential algorithm on a sequence.
     *  @param __begin1 Begin iterator of first sequence.
     *  @param __end1 End iterator of first sequence.
     *  @param __begin2 Begin iterator of second sequence.
     *  @param __pred Find predicate. 
     */
    template<typename _RAIter1, typename _RAIter2,
             typename _Pred>
      std::pair<_RAIter1, _RAIter2>
      _M_sequential_algorithm(_RAIter1 __begin1,
			      _RAIter1 __end1,
			      _RAIter2 __begin2, _Pred __pred)
      { return mismatch(__begin1, __end1, __begin2,
			__pred, sequential_tag()); }
  };


  /** @brief Test predicate on several elements. */
  template<typename _FIterator>
    struct __find_first_of_selector : public __generic_find_selector
    {
      _FIterator _M_begin;
      _FIterator _M_end;

      explicit __find_first_of_selector(_FIterator __begin,
					_FIterator __end)
      : _M_begin(__begin), _M_end(__end) { }

      /** @brief Test on one position.
       *  @param __i1 _Iterator on first sequence.
       *  @param __i2 _Iterator on second sequence (unused).
       *  @param __pred Find predicate. */
      template<typename _RAIter1, typename _RAIter2,
	       typename _Pred>
        bool
        operator()(_RAIter1 __i1, _RAIter2 __i2, _Pred __pred)
        {
	  for (_FIterator __pos_in_candidates = _M_begin;
	       __pos_in_candidates != _M_end; ++__pos_in_candidates)
	    if (__pred(*__i1, *__pos_in_candidates))
	      return true;
	  return false;
	}

      /** @brief Corresponding sequential algorithm on a sequence.
       *  @param __begin1 Begin iterator of first sequence.
       *  @param __end1 End iterator of first sequence.
       *  @param __begin2 Begin iterator of second sequence.
       *  @param __pred Find predicate. */
      template<typename _RAIter1, typename _RAIter2,
	       typename _Pred>
        std::pair<_RAIter1, _RAIter2>
        _M_sequential_algorithm(_RAIter1 __begin1,
				_RAIter1 __end1,
				_RAIter2 __begin2, _Pred __pred)
        {
	  return std::make_pair(find_first_of(__begin1, __end1,
					      _M_begin, _M_end, __pred,
					      sequential_tag()), __begin2);
	}
     };
}

#endif /* _GLIBCXX_PARALLEL_FIND_SELECTORS_H */
