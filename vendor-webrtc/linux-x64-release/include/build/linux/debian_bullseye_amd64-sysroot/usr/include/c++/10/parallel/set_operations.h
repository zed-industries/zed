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

/**
 * @file parallel/set_operations.h
 * @brief Parallel implementations of set operations for random-access
 * iterators.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Marius Elvert and Felix Bondarenko.

#ifndef _GLIBCXX_PARALLEL_SET_OPERATIONS_H
#define _GLIBCXX_PARALLEL_SET_OPERATIONS_H 1

#include <omp.h>

#include <parallel/settings.h>
#include <parallel/multiseq_selection.h>

namespace __gnu_parallel
{
  template<typename _IIter, typename _OutputIterator>
    _OutputIterator
    __copy_tail(std::pair<_IIter, _IIter> __b,
		std::pair<_IIter, _IIter> __e, _OutputIterator __r)
    {
      if (__b.first != __e.first)
	{
          do
            {
              *__r++ = *__b.first++;
            }
          while (__b.first != __e.first);
	}
      else
	{
          while (__b.second != __e.second)
            *__r++ = *__b.second++;
	}
      return __r;
    }

  template<typename _IIter,
           typename _OutputIterator,
           typename _Compare>
    struct __symmetric_difference_func
    {
      typedef std::iterator_traits<_IIter> _TraitsType;
      typedef typename _TraitsType::difference_type _DifferenceType;
      typedef typename std::pair<_IIter, _IIter> _IteratorPair;

      __symmetric_difference_func(_Compare __comp) : _M_comp(__comp) {}

      _Compare _M_comp;

      _OutputIterator
      _M_invoke(_IIter __a, _IIter __b, _IIter __c, _IIter __d,
		_OutputIterator __r) const
      {
	while (__a != __b && __c != __d)
          {
            if (_M_comp(*__a, *__c))
              {
        	*__r = *__a;
        	++__a;
        	++__r;
              }
            else if (_M_comp(*__c, *__a))
              {
        	*__r = *__c;
        	++__c;
        	++__r;
              }
            else
              {
        	++__a;
        	++__c;
              }
          }
	return std::copy(__c, __d, std::copy(__a, __b, __r));
      }

      _DifferenceType
      __count(_IIter __a, _IIter __b, _IIter __c, _IIter __d) const
      {
	_DifferenceType __counter = 0;

	while (__a != __b && __c != __d)
          {
            if (_M_comp(*__a, *__c))
              {
        	++__a;
        	++__counter;
              }
            else if (_M_comp(*__c, *__a))
              {
        	++__c;
        	++__counter;
              }
            else
              {
        	++__a;
        	++__c;
              }
          }

	return __counter + (__b - __a) + (__d - __c);
      }

      _OutputIterator
      __first_empty(_IIter __c, _IIter __d, _OutputIterator __out) const
      { return std::copy(__c, __d, __out); }

      _OutputIterator
      __second_empty(_IIter __a, _IIter __b, _OutputIterator __out) const
      { return std::copy(__a, __b, __out); }
    };


  template<typename _IIter,
           typename _OutputIterator,
           typename _Compare>
    struct __difference_func
    {
      typedef std::iterator_traits<_IIter> _TraitsType;
      typedef typename _TraitsType::difference_type _DifferenceType;
      typedef typename std::pair<_IIter, _IIter> _IteratorPair;

      __difference_func(_Compare __comp) : _M_comp(__comp) {}

      _Compare _M_comp;

      _OutputIterator
      _M_invoke(_IIter __a, _IIter __b, _IIter __c, _IIter __d,
		_OutputIterator __r) const
      {
	while (__a != __b && __c != __d)
          {
            if (_M_comp(*__a, *__c))
              {
        	*__r = *__a;
        	++__a;
        	++__r;
              }
            else if (_M_comp(*__c, *__a))
              { ++__c; }
            else
              {
        	++__a;
        	++__c;
              }
          }
	return std::copy(__a, __b, __r);
      }

      _DifferenceType
      __count(_IIter __a, _IIter __b,
	      _IIter __c, _IIter __d) const
      {
	_DifferenceType __counter = 0;

	while (__a != __b && __c != __d)
          {
            if (_M_comp(*__a, *__c))
              {
        	++__a;
        	++__counter;
              }
            else if (_M_comp(*__c, *__a))
              { ++__c; }
            else
              { ++__a; ++__c; }
          }

	return __counter + (__b - __a);
      }

      _OutputIterator
      __first_empty(_IIter, _IIter, _OutputIterator __out) const
      { return __out; }

      _OutputIterator
      __second_empty(_IIter __a, _IIter __b, _OutputIterator __out) const
      { return std::copy(__a, __b, __out); }
    };


  template<typename _IIter,
           typename _OutputIterator,
           typename _Compare>
    struct __intersection_func
    {
      typedef std::iterator_traits<_IIter> _TraitsType;
      typedef typename _TraitsType::difference_type _DifferenceType;
      typedef typename std::pair<_IIter, _IIter> _IteratorPair;

      __intersection_func(_Compare __comp) : _M_comp(__comp) {}

      _Compare _M_comp;

      _OutputIterator
      _M_invoke(_IIter __a, _IIter __b, _IIter __c, _IIter __d,
		_OutputIterator __r) const
      {
	while (__a != __b && __c != __d)
          {
            if (_M_comp(*__a, *__c))
              { ++__a; }
            else if (_M_comp(*__c, *__a))
              { ++__c; }
            else
              {
        	*__r = *__a;
        	++__a;
        	++__c;
        	++__r;
              }
          }

	return __r;
      }

      _DifferenceType
      __count(_IIter __a, _IIter __b, _IIter __c, _IIter __d) const
      {
	_DifferenceType __counter = 0;

	while (__a != __b && __c != __d)
          {
            if (_M_comp(*__a, *__c))
              { ++__a; }
            else if (_M_comp(*__c, *__a))
              { ++__c; }
            else
              {
        	++__a;
        	++__c;
        	++__counter;
              }
          }

	return __counter;
      }

      _OutputIterator
      __first_empty(_IIter, _IIter, _OutputIterator __out) const
      { return __out; }

      _OutputIterator
      __second_empty(_IIter, _IIter, _OutputIterator __out) const
      { return __out; }
    };

  template<class _IIter, class _OutputIterator, class _Compare>
    struct __union_func
    {
      typedef typename std::iterator_traits<_IIter>::difference_type
      _DifferenceType;

      __union_func(_Compare __comp) : _M_comp(__comp) {}

      _Compare _M_comp;

      _OutputIterator
      _M_invoke(_IIter __a, const _IIter __b, _IIter __c,
		const _IIter __d, _OutputIterator __r) const
      {
	while (__a != __b && __c != __d)
          {
            if (_M_comp(*__a, *__c))
              {
        	*__r = *__a;
        	++__a;
              }
            else if (_M_comp(*__c, *__a))
              {
        	*__r = *__c;
        	++__c;
              }
            else
              {
        	*__r = *__a;
        	++__a;
        	++__c;
              }
            ++__r;
          }
	return std::copy(__c, __d, std::copy(__a, __b, __r));
      }

      _DifferenceType
      __count(_IIter __a, _IIter __b, _IIter __c, _IIter __d) const
      {
	_DifferenceType __counter = 0;

	while (__a != __b && __c != __d)
          {
            if (_M_comp(*__a, *__c))
              { ++__a; }
            else if (_M_comp(*__c, *__a))
              { ++__c; }
            else
              {
        	++__a;
        	++__c;
              }
            ++__counter;
          }

	__counter += (__b - __a);
	__counter += (__d - __c);
	return __counter;
      }

      _OutputIterator
      __first_empty(_IIter __c, _IIter __d, _OutputIterator __out) const
      { return std::copy(__c, __d, __out); }

      _OutputIterator
      __second_empty(_IIter __a, _IIter __b, _OutputIterator __out) const
      { return std::copy(__a, __b, __out); }
    };

  template<typename _IIter,
           typename _OutputIterator,
           typename _Operation>
    _OutputIterator
    __parallel_set_operation(_IIter __begin1, _IIter __end1,
			     _IIter __begin2, _IIter __end2,
			     _OutputIterator __result, _Operation __op)
    {
      _GLIBCXX_CALL((__end1 - __begin1) + (__end2 - __begin2))

      typedef std::iterator_traits<_IIter> _TraitsType;
      typedef typename _TraitsType::difference_type _DifferenceType;
      typedef typename std::pair<_IIter, _IIter> _IteratorPair;

      if (__begin1 == __end1)
	return __op.__first_empty(__begin2, __end2, __result);

      if (__begin2 == __end2)
	return __op.__second_empty(__begin1, __end1, __result);

      const _DifferenceType __size = (__end1 - __begin1) + (__end2 - __begin2);

      const _IteratorPair __sequence[2] = { std::make_pair(__begin1, __end1),
					    std::make_pair(__begin2, __end2) };
      _OutputIterator __return_value = __result;
      _DifferenceType *__borders;
      _IteratorPair *__block_begins;
      _DifferenceType* __lengths;

      _ThreadIndex __num_threads =
          std::min<_DifferenceType>(__get_max_threads(),
              std::min(__end1 - __begin1, __end2 - __begin2));

#     pragma omp parallel num_threads(__num_threads)
      {
#       pragma omp single
	{
	  __num_threads = omp_get_num_threads();

	  __borders = new _DifferenceType[__num_threads + 2];
	  __equally_split(__size, __num_threads + 1, __borders);
	  __block_begins = new _IteratorPair[__num_threads + 1];
	  // Very __start.
	  __block_begins[0] = std::make_pair(__begin1, __begin2);
	  __lengths = new _DifferenceType[__num_threads];
	} //single

	_ThreadIndex __iam = omp_get_thread_num();

	// _Result from multiseq_partition.
	_IIter __offset[2];
	const _DifferenceType __rank = __borders[__iam + 1];

	multiseq_partition(__sequence, __sequence + 2,
			   __rank, __offset, __op._M_comp);

	// allowed to read?
	// together
	// *(__offset[ 0 ] - 1) == *__offset[ 1 ]
	if (__offset[ 0 ] != __begin1 && __offset[1] != __end2
	    && !__op._M_comp(*(__offset[0] - 1), *__offset[1])
	    && !__op._M_comp(*__offset[1], *(__offset[0] - 1)))
	  {
	    // Avoid split between globally equal elements: move one to
	    // front in first sequence.
              --__offset[0];
	  }

	_IteratorPair __block_end = __block_begins[__iam + 1] =
	  _IteratorPair(__offset[0], __offset[1]);

	// Make sure all threads have their block_begin result written out.
#       pragma omp barrier

	_IteratorPair __block_begin = __block_begins[__iam];

	// Begin working for the first block, while the others except
	// the last start to count.
	if (__iam == 0)
	  {
	    // The first thread can copy already.
	    __lengths[ __iam ] =
	      __op._M_invoke(__block_begin.first, __block_end.first,
			     __block_begin.second, __block_end.second,
			     __result) - __result;
	  }
	else
	  {
	    __lengths[ __iam ] =
	      __op.__count(__block_begin.first, __block_end.first,
			   __block_begin.second, __block_end.second);
	  }

	// Make sure everyone wrote their lengths.
#       pragma omp barrier

	_OutputIterator __r = __result;

	if (__iam == 0)
	  {
	    // Do the last block.
	    for (_ThreadIndex __i = 0; __i < __num_threads; ++__i)
	      __r += __lengths[__i];

	    __block_begin = __block_begins[__num_threads];

	    // Return the result iterator of the last block.
	    __return_value =
	      __op._M_invoke(__block_begin.first, __end1,
			     __block_begin.second, __end2, __r);

	  }
          else
            {
              for (_ThreadIndex __i = 0; __i < __iam; ++__i)
        	__r += __lengths[ __i ];

              // Reset begins for copy pass.
              __op._M_invoke(__block_begin.first, __block_end.first,
			     __block_begin.second, __block_end.second, __r);
            }
	}
      return __return_value;
    }

  template<typename _IIter,
           typename _OutputIterator,
           typename _Compare>
    inline _OutputIterator
    __parallel_set_union(_IIter __begin1, _IIter __end1,
			 _IIter __begin2, _IIter __end2,
			 _OutputIterator __result, _Compare __comp)
    {
      return __parallel_set_operation(__begin1, __end1, __begin2, __end2,
				      __result,
				      __union_func< _IIter, _OutputIterator,
				      _Compare>(__comp));
    }

  template<typename _IIter,
           typename _OutputIterator,
           typename _Compare>
    inline _OutputIterator
    __parallel_set_intersection(_IIter __begin1, _IIter __end1,
                        	_IIter __begin2, _IIter __end2,
                        	_OutputIterator __result, _Compare __comp)
    {
      return __parallel_set_operation(__begin1, __end1, __begin2, __end2,
				      __result,
				      __intersection_func<_IIter,
				      _OutputIterator, _Compare>(__comp));
    }

  template<typename _IIter,
           typename _OutputIterator,
           typename _Compare>
    inline _OutputIterator
    __parallel_set_difference(_IIter __begin1, _IIter __end1,
                              _IIter __begin2, _IIter __end2,
                              _OutputIterator __result, _Compare __comp)
    {
      return __parallel_set_operation(__begin1, __end1, __begin2, __end2,
				      __result,
				      __difference_func<_IIter,
				      _OutputIterator, _Compare>(__comp));
    }

  template<typename _IIter,
           typename _OutputIterator,
           typename _Compare>
    inline _OutputIterator
    __parallel_set_symmetric_difference(_IIter __begin1, _IIter __end1,
                                	_IIter __begin2, _IIter __end2,
                                	_OutputIterator __result,
                                	_Compare __comp)
    {
      return __parallel_set_operation(__begin1, __end1, __begin2, __end2,
				      __result,
				      __symmetric_difference_func<_IIter,
				      _OutputIterator, _Compare>(__comp));
    }
}

#endif /* _GLIBCXX_PARALLEL_SET_OPERATIONS_H */
