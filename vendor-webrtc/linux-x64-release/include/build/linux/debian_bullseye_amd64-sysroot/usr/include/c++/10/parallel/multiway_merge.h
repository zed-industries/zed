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

/** @file parallel/multiway_merge.h
*  @brief Implementation of sequential and parallel multiway merge.
*
*  Explanations on the high-speed merging routines in the appendix of
*
*  P. Sanders.
*  Fast priority queues for cached memory.
*  ACM Journal of Experimental Algorithmics, 5, 2000.
*
*  This file is a GNU parallel extension to the Standard C++ Library.
*/

// Written by Johannes Singler and Manuel Holtgrewe.

#ifndef _GLIBCXX_PARALLEL_MULTIWAY_MERGE_H
#define _GLIBCXX_PARALLEL_MULTIWAY_MERGE_H

#include <vector>

#include <bits/stl_algo.h>
#include <parallel/features.h>
#include <parallel/parallel.h>
#include <parallel/losertree.h>
#include <parallel/multiseq_selection.h>
#if _GLIBCXX_PARALLEL_ASSERTIONS
#include <parallel/checkers.h>
#endif

/** @brief Length of a sequence described by a pair of iterators. */
#define _GLIBCXX_PARALLEL_LENGTH(__s) ((__s).second - (__s).first)

namespace __gnu_parallel
{
  template<typename _RAIter1, typename _RAIter2, typename _OutputIterator,
	   typename _DifferenceTp, typename _Compare>
    _OutputIterator
    __merge_advance(_RAIter1&, _RAIter1, _RAIter2&, _RAIter2,
		    _OutputIterator, _DifferenceTp, _Compare);

  /** @brief _Iterator wrapper supporting an implicit supremum at the end
   *         of the sequence, dominating all comparisons.
   *
   * The implicit supremum comes with a performance cost.
   *
   * Deriving from _RAIter is not possible since
   * _RAIter need not be a class.
   */
  template<typename _RAIter, typename _Compare>
    class _GuardedIterator
    {
    private:
      /** @brief Current iterator __position. */
      _RAIter _M_current;

      /** @brief End iterator of the sequence. */
      _RAIter _M_end;

      /** @brief _Compare. */
      _Compare& __comp;

    public:
      /** @brief Constructor. Sets iterator to beginning of sequence.
      *  @param __begin Begin iterator of sequence.
      *  @param __end End iterator of sequence.
      *  @param __comp Comparator provided for associated overloaded
      *  compare operators. */
      _GuardedIterator(_RAIter __begin, _RAIter __end, _Compare& __comp)
      : _M_current(__begin), _M_end(__end), __comp(__comp)
      { }

      /** @brief Pre-increment operator.
      *  @return This. */
      _GuardedIterator<_RAIter, _Compare>&
      operator++()
      {
	++_M_current;
	return *this;
      }

      /** @brief Dereference operator.
      *  @return Referenced element. */
      typename std::iterator_traits<_RAIter>::value_type&
      operator*()
      { return *_M_current; }

      /** @brief Convert to wrapped iterator.
      *  @return Wrapped iterator. */
      operator _RAIter()
      { return _M_current; }

      /** @brief Compare two elements referenced by guarded iterators.
       *  @param __bi1 First iterator.
       *  @param __bi2 Second iterator.
       *  @return @c true if less. */
      friend bool
      operator<(_GuardedIterator<_RAIter, _Compare>& __bi1,
		_GuardedIterator<_RAIter, _Compare>& __bi2)
      {
	if (__bi1._M_current == __bi1._M_end)       // __bi1 is sup
	  return __bi2._M_current == __bi2._M_end;  // __bi2 is not sup
	if (__bi2._M_current == __bi2._M_end)       // __bi2 is sup
	  return true;
	return (__bi1.__comp)(*__bi1, *__bi2);      // normal compare
      }

      /** @brief Compare two elements referenced by guarded iterators.
       *  @param __bi1 First iterator.
       *  @param __bi2 Second iterator.
       *  @return @c True if less equal. */
      friend bool
      operator<=(_GuardedIterator<_RAIter, _Compare>& __bi1,
		 _GuardedIterator<_RAIter, _Compare>& __bi2)
      {
	if (__bi2._M_current == __bi2._M_end)       // __bi1 is sup
	  return __bi1._M_current != __bi1._M_end;  // __bi2 is not sup
	if (__bi1._M_current == __bi1._M_end)       // __bi2 is sup
	  return false;
	return !(__bi1.__comp)(*__bi2, *__bi1);     // normal compare
      } 
    };

  template<typename _RAIter, typename _Compare>
    class _UnguardedIterator
    {
    private:
      /** @brief Current iterator __position. */
      _RAIter _M_current;
      /** @brief _Compare. */
      _Compare& __comp;

    public:
      /** @brief Constructor. Sets iterator to beginning of sequence.
      *  @param __begin Begin iterator of sequence.
      *  @param __end Unused, only for compatibility.
      *  @param __comp Unused, only for compatibility. */
      _UnguardedIterator(_RAIter __begin,
                	 _RAIter /* __end */, _Compare& __comp)
      : _M_current(__begin), __comp(__comp)
      { }

      /** @brief Pre-increment operator.
      *  @return This. */
      _UnguardedIterator<_RAIter, _Compare>&
      operator++()
      {
	++_M_current;
	return *this;
      }

      /** @brief Dereference operator.
      *  @return Referenced element. */
      typename std::iterator_traits<_RAIter>::value_type&
      operator*()
      { return *_M_current; }

      /** @brief Convert to wrapped iterator.
      *  @return Wrapped iterator. */
      operator _RAIter()
      { return _M_current; }

      /** @brief Compare two elements referenced by unguarded iterators.
       *  @param __bi1 First iterator.
       *  @param __bi2 Second iterator.
       *  @return @c true if less. */
      friend bool
      operator<(_UnguardedIterator<_RAIter, _Compare>& __bi1,
		_UnguardedIterator<_RAIter, _Compare>& __bi2)
      {
	// Normal compare.
	return (__bi1.__comp)(*__bi1, *__bi2);
      }

      /** @brief Compare two elements referenced by unguarded iterators.
       *  @param __bi1 First iterator.
       *  @param __bi2 Second iterator.
       *  @return @c True if less equal. */
      friend bool
      operator<=(_UnguardedIterator<_RAIter, _Compare>& __bi1,
		 _UnguardedIterator<_RAIter, _Compare>& __bi2)
      {
	// Normal compare.
	return !(__bi1.__comp)(*__bi2, *__bi1);
      }
    };

  /** @brief Highly efficient 3-way merging procedure.
   *
   * Merging is done with the algorithm implementation described by Peter
   * Sanders.  Basically, the idea is to minimize the number of necessary
   * comparison after merging an element.  The implementation trick
   * that makes this fast is that the order of the sequences is stored
   * in the instruction pointer (translated into labels in C++).
   *
   * This works well for merging up to 4 sequences.
   *
   * Note that making the merging stable does @a not come at a
   * performance hit.
   *
   * Whether the merging is done guarded or unguarded is selected by the
   * used iterator class.
   *
   * @param __seqs_begin Begin iterator of iterator pair input sequence.
   * @param __seqs_end End iterator of iterator pair input sequence.
   * @param __target Begin iterator of output sequence.
   * @param __comp Comparator.
   * @param __length Maximum length to merge, less equal than the
   * total number of elements available.
   *
   * @return End iterator of output sequence.
   */
  template<template<typename _RAI, typename _Cp> class iterator,
           typename _RAIterIterator,
           typename _RAIter3,
           typename _DifferenceTp,
           typename _Compare>
    _RAIter3
    multiway_merge_3_variant(_RAIterIterator __seqs_begin,
			     _RAIterIterator __seqs_end,
			     _RAIter3 __target,
			     _DifferenceTp __length, _Compare __comp)
    {
      _GLIBCXX_CALL(__length);

      typedef _DifferenceTp _DifferenceType;

      typedef typename std::iterator_traits<_RAIterIterator>
	::value_type::first_type
	_RAIter1;
      typedef typename std::iterator_traits<_RAIter1>::value_type
	_ValueType;

      if (__length == 0)
	return __target;

#if _GLIBCXX_PARALLEL_ASSERTIONS
      _DifferenceTp __orig_length = __length;
#endif

      iterator<_RAIter1, _Compare>
	__seq0(__seqs_begin[0].first, __seqs_begin[0].second, __comp),
	__seq1(__seqs_begin[1].first, __seqs_begin[1].second, __comp),
	__seq2(__seqs_begin[2].first, __seqs_begin[2].second, __comp);

      if (__seq0 <= __seq1)
	{
          if (__seq1 <= __seq2)
            goto __s012;
          else
            if (__seq2 <  __seq0)
              goto __s201;
            else
              goto __s021;
	}
      else
	{
          if (__seq1 <= __seq2)
            {
              if (__seq0 <= __seq2)
        	goto __s102;
              else
        	goto __s120;
            }
          else
            goto __s210;
	}
#define _GLIBCXX_PARALLEL_MERGE_3_CASE(__a, __b, __c, __c0, __c1) \
      __s ## __a ## __b ## __c :                            \
	*__target = *__seq ## __a;                          \
	++__target;                                         \
	--__length;                                         \
	++__seq ## __a;                                     \
	if (__length == 0) goto __finish;                   \
	if (__seq ## __a __c0 __seq ## __b) goto __s ## __a ## __b ## __c; \
	if (__seq ## __a __c1 __seq ## __c) goto __s ## __b ## __a ## __c; \
	goto __s ## __b ## __c ## __a;

      _GLIBCXX_PARALLEL_MERGE_3_CASE(0, 1, 2, <=, <=);
      _GLIBCXX_PARALLEL_MERGE_3_CASE(1, 2, 0, <=, < );
      _GLIBCXX_PARALLEL_MERGE_3_CASE(2, 0, 1, < , < );
      _GLIBCXX_PARALLEL_MERGE_3_CASE(1, 0, 2, < , <=);
      _GLIBCXX_PARALLEL_MERGE_3_CASE(0, 2, 1, <=, <=);
      _GLIBCXX_PARALLEL_MERGE_3_CASE(2, 1, 0, < , < );

#undef _GLIBCXX_PARALLEL_MERGE_3_CASE

    __finish:
      ;

#if _GLIBCXX_PARALLEL_ASSERTIONS
    _GLIBCXX_PARALLEL_ASSERT(
	((_RAIter1)__seq0 - __seqs_begin[0].first) +
	((_RAIter1)__seq1 - __seqs_begin[1].first) +
	((_RAIter1)__seq2 - __seqs_begin[2].first)
	== __orig_length);
#endif

      __seqs_begin[0].first = __seq0;
      __seqs_begin[1].first = __seq1;
      __seqs_begin[2].first = __seq2;

      return __target;
    }

  /**
   * @brief Highly efficient 4-way merging procedure.
   *
   * Merging is done with the algorithm implementation described by Peter
   * Sanders. Basically, the idea is to minimize the number of necessary
   * comparison after merging an element.  The implementation trick
   * that makes this fast is that the order of the sequences is stored
   * in the instruction pointer (translated into goto labels in C++).
   *
   * This works well for merging up to 4 sequences.
   *
   * Note that making the merging stable does @a not come at a
   * performance hit.
   *
   * Whether the merging is done guarded or unguarded is selected by the
   * used iterator class.
   *
   * @param __seqs_begin Begin iterator of iterator pair input sequence.
   * @param __seqs_end End iterator of iterator pair input sequence.
   * @param __target Begin iterator of output sequence.
   * @param __comp Comparator.
   * @param __length Maximum length to merge, less equal than the
   * total number of elements available.
   *
   * @return End iterator of output sequence.
   */
  template<template<typename _RAI, typename _Cp> class iterator,
           typename _RAIterIterator,
           typename _RAIter3,
           typename _DifferenceTp,
           typename _Compare>
    _RAIter3
    multiway_merge_4_variant(_RAIterIterator __seqs_begin,
                             _RAIterIterator __seqs_end,
                             _RAIter3 __target,
                             _DifferenceTp __length, _Compare __comp)
    {
      _GLIBCXX_CALL(__length);
      typedef _DifferenceTp _DifferenceType;

      typedef typename std::iterator_traits<_RAIterIterator>
	::value_type::first_type
	_RAIter1;
      typedef typename std::iterator_traits<_RAIter1>::value_type
	_ValueType;

      iterator<_RAIter1, _Compare>
	__seq0(__seqs_begin[0].first, __seqs_begin[0].second, __comp),
	__seq1(__seqs_begin[1].first, __seqs_begin[1].second, __comp),
	__seq2(__seqs_begin[2].first, __seqs_begin[2].second, __comp),
	__seq3(__seqs_begin[3].first, __seqs_begin[3].second, __comp);

#define _GLIBCXX_PARALLEL_DECISION(__a, __b, __c, __d) {  \
	if (__seq ## __d < __seq ## __a)		  \
	  goto __s ## __d ## __a ## __b ## __c;		  \
	if (__seq ## __d < __seq ## __b)		  \
	  goto __s ## __a ## __d ## __b ## __c;		  \
	if (__seq ## __d < __seq ## __c)		  \
	  goto __s ## __a ## __b ## __d ## __c;		  \
	goto __s ## __a ## __b ## __c ## __d;  }

      if (__seq0 <= __seq1)
	{
          if (__seq1 <= __seq2)
            _GLIBCXX_PARALLEL_DECISION(0,1,2,3)
            else
              if (__seq2 < __seq0)
        	_GLIBCXX_PARALLEL_DECISION(2,0,1,3)
        	else
                  _GLIBCXX_PARALLEL_DECISION(0,2,1,3)
                    }
      else
	{
          if (__seq1 <= __seq2)
            {
              if (__seq0 <= __seq2)
        	_GLIBCXX_PARALLEL_DECISION(1,0,2,3)
        	else
                  _GLIBCXX_PARALLEL_DECISION(1,2,0,3)
                    }
          else
            _GLIBCXX_PARALLEL_DECISION(2,1,0,3)
              }

#define _GLIBCXX_PARALLEL_MERGE_4_CASE(__a, __b, __c, __d,  \
				       __c0, __c1, __c2)    \
      __s ## __a ## __b ## __c ## __d:                      \
      if (__length == 0) goto __finish;                     \
      *__target = *__seq ## __a;                            \
      ++__target;                                           \
      --__length;                                           \
      ++__seq ## __a;                                       \
      if (__seq ## __a __c0 __seq ## __b)      \
	goto __s ## __a ## __b ## __c ## __d;  \
      if (__seq ## __a __c1 __seq ## __c)      \
	goto __s ## __b ## __a ## __c ## __d;  \
      if (__seq ## __a __c2 __seq ## __d)      \
	goto __s ## __b ## __c ## __a ## __d;  \
      goto __s ## __b ## __c ## __d ## __a;

      _GLIBCXX_PARALLEL_MERGE_4_CASE(0, 1, 2, 3, <=, <=, <=);
      _GLIBCXX_PARALLEL_MERGE_4_CASE(0, 1, 3, 2, <=, <=, <=);
      _GLIBCXX_PARALLEL_MERGE_4_CASE(0, 2, 1, 3, <=, <=, <=);
      _GLIBCXX_PARALLEL_MERGE_4_CASE(0, 2, 3, 1, <=, <=, <=);
      _GLIBCXX_PARALLEL_MERGE_4_CASE(0, 3, 1, 2, <=, <=, <=);
      _GLIBCXX_PARALLEL_MERGE_4_CASE(0, 3, 2, 1, <=, <=, <=);
      _GLIBCXX_PARALLEL_MERGE_4_CASE(1, 0, 2, 3, < , <=, <=);
      _GLIBCXX_PARALLEL_MERGE_4_CASE(1, 0, 3, 2, < , <=, <=);
      _GLIBCXX_PARALLEL_MERGE_4_CASE(1, 2, 0, 3, <=, < , <=);
      _GLIBCXX_PARALLEL_MERGE_4_CASE(1, 2, 3, 0, <=, <=, < );
      _GLIBCXX_PARALLEL_MERGE_4_CASE(1, 3, 0, 2, <=, < , <=);
      _GLIBCXX_PARALLEL_MERGE_4_CASE(1, 3, 2, 0, <=, <=, < );
      _GLIBCXX_PARALLEL_MERGE_4_CASE(2, 0, 1, 3, < , < , <=);
      _GLIBCXX_PARALLEL_MERGE_4_CASE(2, 0, 3, 1, < , <=, < );
      _GLIBCXX_PARALLEL_MERGE_4_CASE(2, 1, 0, 3, < , < , <=);
      _GLIBCXX_PARALLEL_MERGE_4_CASE(2, 1, 3, 0, < , <=, < );
      _GLIBCXX_PARALLEL_MERGE_4_CASE(2, 3, 0, 1, <=, < , < );
      _GLIBCXX_PARALLEL_MERGE_4_CASE(2, 3, 1, 0, <=, < , < );
      _GLIBCXX_PARALLEL_MERGE_4_CASE(3, 0, 1, 2, < , < , < );
      _GLIBCXX_PARALLEL_MERGE_4_CASE(3, 0, 2, 1, < , < , < );
      _GLIBCXX_PARALLEL_MERGE_4_CASE(3, 1, 0, 2, < , < , < );
      _GLIBCXX_PARALLEL_MERGE_4_CASE(3, 1, 2, 0, < , < , < );
      _GLIBCXX_PARALLEL_MERGE_4_CASE(3, 2, 0, 1, < , < , < );
      _GLIBCXX_PARALLEL_MERGE_4_CASE(3, 2, 1, 0, < , < , < );

#undef _GLIBCXX_PARALLEL_MERGE_4_CASE
#undef _GLIBCXX_PARALLEL_DECISION

    __finish:
      ;

      __seqs_begin[0].first = __seq0;
      __seqs_begin[1].first = __seq1;
      __seqs_begin[2].first = __seq2;
      __seqs_begin[3].first = __seq3;

      return __target;
    }

  /** @brief Multi-way merging procedure for a high branching factor,
   *         guarded case.
   *
   * This merging variant uses a LoserTree class as selected by <tt>_LT</tt>.
   *
   * Stability is selected through the used LoserTree class <tt>_LT</tt>.
   *
   * At least one non-empty sequence is required.
   *
   * @param __seqs_begin Begin iterator of iterator pair input sequence.
   * @param __seqs_end End iterator of iterator pair input sequence.
   * @param __target Begin iterator of output sequence.
   * @param __comp Comparator.
   * @param __length Maximum length to merge, less equal than the
   * total number of elements available.
   *
   * @return End iterator of output sequence.
   */
  template<typename _LT,
           typename _RAIterIterator,
           typename _RAIter3,
           typename _DifferenceTp,
           typename _Compare>
    _RAIter3
    multiway_merge_loser_tree(_RAIterIterator __seqs_begin,
                              _RAIterIterator __seqs_end,
                              _RAIter3 __target,
                              _DifferenceTp __length, _Compare __comp)
    {
      _GLIBCXX_CALL(__length)

      typedef _DifferenceTp _DifferenceType;
      typedef typename std::iterator_traits<_RAIterIterator>
	::difference_type _SeqNumber;
      typedef typename std::iterator_traits<_RAIterIterator>
	::value_type::first_type
	_RAIter1;
      typedef typename std::iterator_traits<_RAIter1>::value_type
	_ValueType;

      _SeqNumber __k = static_cast<_SeqNumber>(__seqs_end - __seqs_begin);

      _LT __lt(__k, __comp);

      // Default value for potentially non-default-constructible types.
      _ValueType* __arbitrary_element = 0;

      for (_SeqNumber __t = 0; __t < __k; ++__t)
	{
          if(!__arbitrary_element
	     && _GLIBCXX_PARALLEL_LENGTH(__seqs_begin[__t]) > 0)
            __arbitrary_element = &(*__seqs_begin[__t].first);
	}

      for (_SeqNumber __t = 0; __t < __k; ++__t)
	{
          if (__seqs_begin[__t].first == __seqs_begin[__t].second)
            __lt.__insert_start(*__arbitrary_element, __t, true);
          else
            __lt.__insert_start(*__seqs_begin[__t].first, __t, false);
	}

      __lt.__init();

      _SeqNumber __source;

      for (_DifferenceType __i = 0; __i < __length; ++__i)
	{
          //take out
          __source = __lt.__get_min_source();

          *(__target++) = *(__seqs_begin[__source].first++);

          // Feed.
          if (__seqs_begin[__source].first == __seqs_begin[__source].second)
            __lt.__delete_min_insert(*__arbitrary_element, true);
          else
            // Replace from same __source.
            __lt.__delete_min_insert(*__seqs_begin[__source].first, false);
	}

      return __target;
    }

  /** @brief Multi-way merging procedure for a high branching factor,
   *         unguarded case.
   *
   * Merging is done using the LoserTree class <tt>_LT</tt>.
   *
   * Stability is selected by the used LoserTrees.
   *
   * @pre No input will run out of elements during the merge.
   *
   * @param __seqs_begin Begin iterator of iterator pair input sequence.
   * @param __seqs_end End iterator of iterator pair input sequence.
   * @param __target Begin iterator of output sequence.
   * @param __comp Comparator.
   * @param __length Maximum length to merge, less equal than the
   * total number of elements available.
   *
   * @return End iterator of output sequence.
   */
  template<typename _LT,
	   typename _RAIterIterator,
	   typename _RAIter3,
	   typename _DifferenceTp, typename _Compare>
    _RAIter3
    multiway_merge_loser_tree_unguarded(_RAIterIterator __seqs_begin,
					_RAIterIterator __seqs_end,
					_RAIter3 __target,
       const typename std::iterator_traits<typename std::iterator_traits<
	  _RAIterIterator>::value_type::first_type>::value_type&
					__sentinel,
					_DifferenceTp __length,
					_Compare __comp)
    {
      _GLIBCXX_CALL(__length)
      typedef _DifferenceTp _DifferenceType;

      typedef typename std::iterator_traits<_RAIterIterator>
	::difference_type _SeqNumber;
      typedef typename std::iterator_traits<_RAIterIterator>
	::value_type::first_type
	_RAIter1;
      typedef typename std::iterator_traits<_RAIter1>::value_type
	_ValueType;

      _SeqNumber __k = __seqs_end - __seqs_begin;

      _LT __lt(__k, __sentinel, __comp);

      for (_SeqNumber __t = 0; __t < __k; ++__t)
	{
#if _GLIBCXX_PARALLEL_ASSERTIONS
          _GLIBCXX_PARALLEL_ASSERT(__seqs_begin[__t].first
                                   != __seqs_begin[__t].second);
#endif
          __lt.__insert_start(*__seqs_begin[__t].first, __t, false);
	}

      __lt.__init();

      _SeqNumber __source;

#if _GLIBCXX_PARALLEL_ASSERTIONS
      _DifferenceType __i = 0;
#endif

      _RAIter3 __target_end = __target + __length;
      while (__target < __target_end)
	{
          // Take out.
          __source = __lt.__get_min_source();

#if _GLIBCXX_PARALLEL_ASSERTIONS
          _GLIBCXX_PARALLEL_ASSERT(0 <= __source && __source < __k);
          _GLIBCXX_PARALLEL_ASSERT(__i == 0
              || !__comp(*(__seqs_begin[__source].first), *(__target - 1)));
#endif

          // Feed.
          *(__target++) = *(__seqs_begin[__source].first++);

#if _GLIBCXX_PARALLEL_ASSERTIONS
          ++__i;
#endif
          // Replace from same __source.
          __lt.__delete_min_insert(*__seqs_begin[__source].first, false);
	}

      return __target;
    }


  /** @brief Multi-way merging procedure for a high branching factor,
   *         requiring sentinels to exist.
   *
   * @tparam _UnguardedLoserTree Loser Tree variant to use for the unguarded
   *				 merging.
   *
   * @param __seqs_begin Begin iterator of iterator pair input sequence.
   * @param __seqs_end End iterator of iterator pair input sequence.
   * @param __target Begin iterator of output sequence.
   * @param __comp Comparator.
   * @param __length Maximum length to merge, less equal than the
   * total number of elements available.
   *
   * @return End iterator of output sequence.
   */
  template<typename _UnguardedLoserTree,
	   typename _RAIterIterator,
	   typename _RAIter3,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIter3
    multiway_merge_loser_tree_sentinel(_RAIterIterator __seqs_begin,
				       _RAIterIterator __seqs_end,
				       _RAIter3 __target,
      const typename std::iterator_traits<typename std::iterator_traits<
	_RAIterIterator>::value_type::first_type>::value_type&
				       __sentinel,
				       _DifferenceTp __length,
				       _Compare __comp)
    {
      _GLIBCXX_CALL(__length)

      typedef _DifferenceTp _DifferenceType;
      typedef std::iterator_traits<_RAIterIterator> _TraitsType;
      typedef typename std::iterator_traits<_RAIterIterator>
	::value_type::first_type
	_RAIter1;
      typedef typename std::iterator_traits<_RAIter1>::value_type
	_ValueType;

      _RAIter3 __target_end;

      for (_RAIterIterator __s = __seqs_begin; __s != __seqs_end; ++__s)
	// Move the sequence ends to the sentinel.  This has the
	// effect that the sentinel appears to be within the sequence. Then,
	// we can use the unguarded variant if we merge out as many
	// non-sentinel elements as we have.
	++((*__s).second);

      __target_end = multiway_merge_loser_tree_unguarded<_UnguardedLoserTree>
	(__seqs_begin, __seqs_end, __target, __sentinel, __length, __comp);

#if _GLIBCXX_PARALLEL_ASSERTIONS
      _GLIBCXX_PARALLEL_ASSERT(__target_end == __target + __length);
      _GLIBCXX_PARALLEL_ASSERT(__is_sorted(__target, __target_end, __comp));
#endif

      // Restore the sequence ends so the sentinels are not contained in the
      // sequence any more (see comment in loop above).
      for (_RAIterIterator __s = __seqs_begin; __s != __seqs_end; ++__s)
	--((*__s).second);

      return __target_end;
    }

  /**
   * @brief Traits for determining whether the loser tree should
   *   use pointers or copies.
   *
   * The field "_M_use_pointer" is used to determine whether to use pointers
   * in he loser trees or whether to copy the values into the loser tree.
   *
   * The default behavior is to use pointers if the data type is 4 times as
   * big as the pointer to it.
   *
   * Specialize for your data type to customize the behavior.
   *
   * Example:
   *
   *   template<>
   *   struct _LoserTreeTraits<int>
   *   { static const bool _M_use_pointer = false; };
   *
   *   template<>
   *   struct _LoserTreeTraits<heavyweight_type>
   *   { static const bool _M_use_pointer = true; };
   *
   * @param _Tp type to give the loser tree traits for.
   */
  template <typename _Tp>
    struct _LoserTreeTraits
    {
      /**
       * @brief True iff to use pointers instead of values in loser trees.
       *
       * The default behavior is to use pointers if the data type is four
       * times as big as the pointer to it.
       */
      static const bool _M_use_pointer = (sizeof(_Tp) > 4 * sizeof(_Tp*));
    };

  /**
   * @brief Switch for 3-way merging with __sentinels turned off.
   *
   * Note that 3-way merging is always stable!
   */
  template<bool __sentinels /*default == false*/,
	   typename _RAIterIterator,
	   typename _RAIter3,
	   typename _DifferenceTp,
	   typename _Compare>
    struct __multiway_merge_3_variant_sentinel_switch
    {
      _RAIter3
      operator()(_RAIterIterator __seqs_begin,
		 _RAIterIterator __seqs_end,
		 _RAIter3 __target,
		 _DifferenceTp __length, _Compare __comp)
      { return multiway_merge_3_variant<_GuardedIterator>
	  (__seqs_begin, __seqs_end, __target, __length, __comp); }
    };

  /**
   * @brief Switch for 3-way merging with __sentinels turned on.
   *
   * Note that 3-way merging is always stable!
   */
  template<typename _RAIterIterator,
	   typename _RAIter3,
	   typename _DifferenceTp,
	   typename _Compare>
    struct __multiway_merge_3_variant_sentinel_switch<true, _RAIterIterator,
						      _RAIter3, _DifferenceTp,
						      _Compare>
    {
      _RAIter3
      operator()(_RAIterIterator __seqs_begin,
		 _RAIterIterator __seqs_end,
		 _RAIter3 __target,
		 _DifferenceTp __length, _Compare __comp)
      { return multiway_merge_3_variant<_UnguardedIterator>
	  (__seqs_begin, __seqs_end, __target, __length, __comp); }
    };

  /**
   * @brief Switch for 4-way merging with __sentinels turned off.
   *
   * Note that 4-way merging is always stable!
   */
  template<bool __sentinels /*default == false*/,
	   typename _RAIterIterator,
	   typename _RAIter3,
	   typename _DifferenceTp,
	   typename _Compare>
    struct __multiway_merge_4_variant_sentinel_switch
    {
      _RAIter3
      operator()(_RAIterIterator __seqs_begin,
		 _RAIterIterator __seqs_end,
		 _RAIter3 __target,
		 _DifferenceTp __length, _Compare __comp)
      { return multiway_merge_4_variant<_GuardedIterator>
	  (__seqs_begin, __seqs_end, __target, __length, __comp); }
    };

  /**
   * @brief Switch for 4-way merging with __sentinels turned on.
   *
   * Note that 4-way merging is always stable!
   */
  template<typename _RAIterIterator,
	   typename _RAIter3,
	   typename _DifferenceTp,
	   typename _Compare>
    struct __multiway_merge_4_variant_sentinel_switch<true, _RAIterIterator,
						      _RAIter3, _DifferenceTp,
						      _Compare>
    {
      _RAIter3
      operator()(_RAIterIterator __seqs_begin,
		 _RAIterIterator __seqs_end,
		 _RAIter3 __target,
		 _DifferenceTp __length, _Compare __comp)
      { return multiway_merge_4_variant<_UnguardedIterator>
	  (__seqs_begin, __seqs_end, __target, __length, __comp); }
    };

  /**
   * @brief Switch for k-way merging with __sentinels turned on.
   */
  template<bool __sentinels,
	   bool __stable,
	   typename _RAIterIterator,
	   typename _RAIter3,
	   typename _DifferenceTp,
	   typename _Compare>
    struct __multiway_merge_k_variant_sentinel_switch
    {
      _RAIter3
      operator()(_RAIterIterator __seqs_begin,
		 _RAIterIterator __seqs_end,
		 _RAIter3 __target,
      const typename std::iterator_traits<typename std::iterator_traits<
      _RAIterIterator>::value_type::first_type>::value_type&
		 __sentinel,
		 _DifferenceTp __length, _Compare __comp)
      {
	typedef typename std::iterator_traits<_RAIterIterator>
	  ::value_type::first_type
	  _RAIter1;
	typedef typename std::iterator_traits<_RAIter1>::value_type
	  _ValueType;

	return multiway_merge_loser_tree_sentinel<
	typename __gnu_cxx::__conditional_type<
	_LoserTreeTraits<_ValueType>::_M_use_pointer,
	  _LoserTreePointerUnguarded<__stable, _ValueType, _Compare>,
	  _LoserTreeUnguarded<__stable, _ValueType, _Compare>
          >::__type>
	  (__seqs_begin, __seqs_end, __target, __sentinel, __length, __comp);
      }
    };

  /**
   * @brief Switch for k-way merging with __sentinels turned off.
   */
  template<bool __stable,
	   typename _RAIterIterator,
	   typename _RAIter3,
	   typename _DifferenceTp,
	   typename _Compare>
    struct __multiway_merge_k_variant_sentinel_switch<false, __stable,
						      _RAIterIterator,
						      _RAIter3, _DifferenceTp,
						      _Compare>
    {
      _RAIter3
      operator()(_RAIterIterator __seqs_begin,
		 _RAIterIterator __seqs_end,
		 _RAIter3 __target,
       const typename std::iterator_traits<typename std::iterator_traits<
       _RAIterIterator>::value_type::first_type>::value_type&
		 __sentinel,
		 _DifferenceTp __length, _Compare __comp)
      {
	typedef typename std::iterator_traits<_RAIterIterator>
	  ::value_type::first_type
	  _RAIter1;
	typedef typename std::iterator_traits<_RAIter1>::value_type
	  _ValueType;

	return multiway_merge_loser_tree<
	typename __gnu_cxx::__conditional_type<
	_LoserTreeTraits<_ValueType>::_M_use_pointer,
	  _LoserTreePointer<__stable, _ValueType, _Compare>,
	  _LoserTree<__stable, _ValueType, _Compare>
          >::__type >(__seqs_begin, __seqs_end, __target, __length, __comp);
      }
    };

  /** @brief Sequential multi-way merging switch.
   *
   *  The _GLIBCXX_PARALLEL_DECISION is based on the branching factor and
   *  runtime settings.
   *  @param __seqs_begin Begin iterator of iterator pair input sequence.
   *  @param __seqs_end End iterator of iterator pair input sequence.
   *  @param __target Begin iterator of output sequence.
   *  @param __comp Comparator.
   *  @param __length Maximum length to merge, possibly larger than the
   *  number of elements available.
   *  @param __sentinel The sequences have __a __sentinel element.
   *  @return End iterator of output sequence. */
  template<bool __stable,
	   bool __sentinels,
	   typename _RAIterIterator,
	   typename _RAIter3,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIter3
    __sequential_multiway_merge(_RAIterIterator __seqs_begin,
				_RAIterIterator __seqs_end,
				_RAIter3 __target,
      const typename std::iterator_traits<typename std::iterator_traits<
	_RAIterIterator>::value_type::first_type>::value_type&
				__sentinel,
				_DifferenceTp __length, _Compare __comp)
    {
      _GLIBCXX_CALL(__length)

      typedef _DifferenceTp _DifferenceType;
      typedef typename std::iterator_traits<_RAIterIterator>
	::difference_type _SeqNumber;
      typedef typename std::iterator_traits<_RAIterIterator>
	::value_type::first_type
	_RAIter1;
      typedef typename std::iterator_traits<_RAIter1>::value_type
	_ValueType;

#if _GLIBCXX_PARALLEL_ASSERTIONS
      for (_RAIterIterator __s = __seqs_begin; __s != __seqs_end; ++__s)
	{
          _GLIBCXX_PARALLEL_ASSERT(__is_sorted((*__s).first,
					       (*__s).second, __comp));
	}
#endif

      _DifferenceTp __total_length = 0;
      for (_RAIterIterator __s = __seqs_begin; __s != __seqs_end; ++__s)
	__total_length += _GLIBCXX_PARALLEL_LENGTH(*__s);

      __length = std::min<_DifferenceTp>(__length, __total_length);

      if(__length == 0)
	return __target;

      _RAIter3 __return_target = __target;
      _SeqNumber __k = static_cast<_SeqNumber>(__seqs_end - __seqs_begin);

      switch (__k)
	{
	case 0:
          break;
	case 1:
          __return_target = std::copy(__seqs_begin[0].first,
				      __seqs_begin[0].first + __length,
				      __target);
          __seqs_begin[0].first += __length;
          break;
	case 2:
          __return_target = __merge_advance(__seqs_begin[0].first,
					    __seqs_begin[0].second,
					    __seqs_begin[1].first,
					    __seqs_begin[1].second,
					    __target, __length, __comp);
          break;
	case 3:
          __return_target = __multiway_merge_3_variant_sentinel_switch
	    <__sentinels, _RAIterIterator, _RAIter3, _DifferenceTp, _Compare>()
	    (__seqs_begin, __seqs_end, __target, __length, __comp);
          break;
	case 4:
          __return_target = __multiway_merge_4_variant_sentinel_switch
	    <__sentinels, _RAIterIterator, _RAIter3, _DifferenceTp, _Compare>()
	    (__seqs_begin, __seqs_end, __target, __length, __comp);
          break;
	default:
	  __return_target = __multiway_merge_k_variant_sentinel_switch
	    <__sentinels, __stable, _RAIterIterator, _RAIter3, _DifferenceTp,
	     _Compare>()
	    (__seqs_begin, __seqs_end, __target, __sentinel, __length, __comp);
	  break;
	}
#if _GLIBCXX_PARALLEL_ASSERTIONS
      _GLIBCXX_PARALLEL_ASSERT(
	__is_sorted(__target, __target + __length, __comp));
#endif

      return __return_target;
    }

  /**
   * @brief Stable sorting functor.
   *
   * Used to reduce code instanciation in multiway_merge_sampling_splitting.
   */
  template<bool __stable, class _RAIter, class _StrictWeakOrdering>
    struct _SamplingSorter
    {
      void
      operator()(_RAIter __first, _RAIter __last, _StrictWeakOrdering __comp)
      { __gnu_sequential::stable_sort(__first, __last, __comp); }
    };

  /**
   * @brief Non-__stable sorting functor.
   *
   * Used to reduce code instantiation in multiway_merge_sampling_splitting.
   */
  template<class _RAIter, class _StrictWeakOrdering>
    struct _SamplingSorter<false, _RAIter, _StrictWeakOrdering>
    {
      void
      operator()(_RAIter __first, _RAIter __last, _StrictWeakOrdering __comp)
      { __gnu_sequential::sort(__first, __last, __comp); }
    };

  /**
   * @brief Sampling based splitting for parallel multiway-merge routine.
   */
  template<bool __stable,
	   typename _RAIterIterator,
	   typename _Compare,
	   typename _DifferenceType>
    void
    multiway_merge_sampling_splitting(_RAIterIterator __seqs_begin,
				      _RAIterIterator __seqs_end,
				      _DifferenceType __length,
				      _DifferenceType __total_length,
				      _Compare __comp,
     std::vector<std::pair<_DifferenceType, _DifferenceType> > *__pieces)
    {
      typedef typename std::iterator_traits<_RAIterIterator>
	::difference_type _SeqNumber;
      typedef typename std::iterator_traits<_RAIterIterator>
	::value_type::first_type
	_RAIter1;
      typedef typename std::iterator_traits<_RAIter1>::value_type
	_ValueType;

      // __k sequences.
      const _SeqNumber __k
	= static_cast<_SeqNumber>(__seqs_end - __seqs_begin);

      const _ThreadIndex __num_threads = omp_get_num_threads();

      const _DifferenceType __num_samples =
	__gnu_parallel::_Settings::get().merge_oversampling * __num_threads;

      _ValueType* __samples = static_cast<_ValueType*>
	(::operator new(sizeof(_ValueType) * __k * __num_samples));
      // Sample.
      for (_SeqNumber __s = 0; __s < __k; ++__s)
	for (_DifferenceType __i = 0; __i < __num_samples; ++__i)
	  {
	    _DifferenceType sample_index = static_cast<_DifferenceType>
	      (_GLIBCXX_PARALLEL_LENGTH(__seqs_begin[__s])
	       * (double(__i + 1) / (__num_samples + 1))
	       * (double(__length) / __total_length));
	    new(&(__samples[__s * __num_samples + __i]))
              _ValueType(__seqs_begin[__s].first[sample_index]);
	  }

      // Sort stable or non-stable, depending on value of template parameter
      // "__stable".
      _SamplingSorter<__stable, _ValueType*, _Compare>()
	(__samples, __samples + (__num_samples * __k), __comp);

      for (_ThreadIndex __slab = 0; __slab < __num_threads; ++__slab)
	// For each slab / processor.
	for (_SeqNumber __seq = 0; __seq < __k; ++__seq)
	  {
	    // For each sequence.
	    if (__slab > 0)
	      __pieces[__slab][__seq].first = std::upper_bound
		(__seqs_begin[__seq].first, __seqs_begin[__seq].second,
		 __samples[__num_samples * __k * __slab / __num_threads],
		 __comp)
		- __seqs_begin[__seq].first;
	    else
	      // Absolute beginning.
	      __pieces[__slab][__seq].first = 0;
	    if ((__slab + 1) < __num_threads)
	      __pieces[__slab][__seq].second = std::upper_bound
		(__seqs_begin[__seq].first, __seqs_begin[__seq].second,
		 __samples[__num_samples * __k * (__slab + 1) / __num_threads],
		 __comp)
		- __seqs_begin[__seq].first;
	    else
              // Absolute end.
	      __pieces[__slab][__seq].second =
		_GLIBCXX_PARALLEL_LENGTH(__seqs_begin[__seq]);
	  }

      for (_SeqNumber __s = 0; __s < __k; ++__s)
	for (_DifferenceType __i = 0; __i < __num_samples; ++__i)
	  __samples[__s * __num_samples + __i].~_ValueType();
      ::operator delete(__samples);
    }

  /**
   * @brief Exact splitting for parallel multiway-merge routine.
   *
   * None of the passed sequences may be empty.
   */
  template<bool __stable,
	   typename _RAIterIterator,
	   typename _Compare,
	   typename _DifferenceType>
    void
    multiway_merge_exact_splitting(_RAIterIterator __seqs_begin,
				   _RAIterIterator __seqs_end,
				   _DifferenceType __length,
				   _DifferenceType __total_length,
				   _Compare __comp,
       std::vector<std::pair<_DifferenceType, _DifferenceType> > *__pieces)
    {
      typedef typename std::iterator_traits<_RAIterIterator>
	::difference_type _SeqNumber;
      typedef typename std::iterator_traits<_RAIterIterator>
	::value_type::first_type
	_RAIter1;

      const bool __tight = (__total_length == __length);

      // __k sequences.
      const _SeqNumber __k = __seqs_end - __seqs_begin;

      const _ThreadIndex __num_threads = omp_get_num_threads();

      // (Settings::multiway_merge_splitting
      //  == __gnu_parallel::_Settings::EXACT).
      std::vector<_RAIter1>* __offsets = 
	new std::vector<_RAIter1>[__num_threads];
      std::vector<std::pair<_RAIter1, _RAIter1> > __se(__k);

      copy(__seqs_begin, __seqs_end, __se.begin());

      _DifferenceType* __borders =
	new _DifferenceType[__num_threads + 1];
      __equally_split(__length, __num_threads, __borders);

      for (_ThreadIndex __s = 0; __s < (__num_threads - 1); ++__s)
	{
	  __offsets[__s].resize(__k);
	  multiseq_partition(__se.begin(), __se.end(), __borders[__s + 1],
			     __offsets[__s].begin(), __comp);

	  // Last one also needed and available.
	  if (!__tight)
	    {
	      __offsets[__num_threads - 1].resize(__k);
	      multiseq_partition(__se.begin(), __se.end(),
				 _DifferenceType(__length),
				 __offsets[__num_threads - 1].begin(),
				 __comp);
	    }
	}
      delete[] __borders;

      for (_ThreadIndex __slab = 0; __slab < __num_threads; ++__slab)
	{
	  // For each slab / processor.
	  for (_SeqNumber __seq = 0; __seq < __k; ++__seq)
	    {
	      // For each sequence.
	      if (__slab == 0)
		{
		  // Absolute beginning.
		  __pieces[__slab][__seq].first = 0;
		}
	      else
		__pieces[__slab][__seq].first =
		  __pieces[__slab - 1][__seq].second;
	      if (!__tight || __slab < (__num_threads - 1))
		__pieces[__slab][__seq].second =
		  __offsets[__slab][__seq] - __seqs_begin[__seq].first;
	      else
		{
		  // __slab == __num_threads - 1
		  __pieces[__slab][__seq].second =
                    _GLIBCXX_PARALLEL_LENGTH(__seqs_begin[__seq]);
		}
	    }
	}
      delete[] __offsets;
    }

  /** @brief Parallel multi-way merge routine.
   *
   * The _GLIBCXX_PARALLEL_DECISION is based on the branching factor
   * and runtime settings.
   *
   * Must not be called if the number of sequences is 1.
   *
   * @tparam _Splitter functor to split input (either __exact or sampling based)
   * @tparam __stable Stable merging incurs a performance penalty.
   * @tparam __sentinel Ignored.
   *
   * @param __seqs_begin Begin iterator of iterator pair input sequence.
   * @param __seqs_end End iterator of iterator pair input sequence.
   * @param __target Begin iterator of output sequence.
   * @param __comp Comparator.
   * @param __length Maximum length to merge, possibly larger than the
   * number of elements available.
   * @return End iterator of output sequence.
   */
  template<bool __stable,
	   bool __sentinels,
	   typename _RAIterIterator,
	   typename _RAIter3,
	   typename _DifferenceTp,
	   typename _Splitter,
	   typename _Compare>
    _RAIter3
    parallel_multiway_merge(_RAIterIterator __seqs_begin,
                            _RAIterIterator __seqs_end,
                            _RAIter3 __target,
                            _Splitter __splitter,
                            _DifferenceTp __length,
                            _Compare __comp,
                            _ThreadIndex __num_threads)
      {
#if _GLIBCXX_PARALLEL_ASSERTIONS
	_GLIBCXX_PARALLEL_ASSERT(__seqs_end - __seqs_begin > 1);
#endif

	_GLIBCXX_CALL(__length)

	typedef _DifferenceTp _DifferenceType;
        typedef typename std::iterator_traits<_RAIterIterator>
	  ::difference_type _SeqNumber;
	typedef typename std::iterator_traits<_RAIterIterator>
          ::value_type::first_type
          _RAIter1;
	typedef typename
          std::iterator_traits<_RAIter1>::value_type _ValueType;

	// Leave only non-empty sequences.
	typedef std::pair<_RAIter1, _RAIter1> seq_type;
	seq_type* __ne_seqs = new seq_type[__seqs_end - __seqs_begin];
	_SeqNumber __k = 0;
	_DifferenceType __total_length = 0;
	for (_RAIterIterator __raii = __seqs_begin;
             __raii != __seqs_end; ++__raii)
          {
            _DifferenceTp __seq_length = _GLIBCXX_PARALLEL_LENGTH(*__raii);
            if(__seq_length > 0)
              {
        	__total_length += __seq_length;
        	__ne_seqs[__k++] = *__raii;
              }
          }

	_GLIBCXX_CALL(__total_length)

	__length = std::min<_DifferenceTp>(__length, __total_length);

	if (__total_length == 0 || __k == 0)
	  {
	    delete[] __ne_seqs;
	    return __target;
	  }

	std::vector<std::pair<_DifferenceType, _DifferenceType> >* __pieces;

	__num_threads = static_cast<_ThreadIndex>
          (std::min<_DifferenceType>(__num_threads, __total_length));

#       pragma omp parallel num_threads (__num_threads)
	{
#         pragma omp single
	  {
	    __num_threads = omp_get_num_threads();
	    // Thread __t will have to merge pieces[__iam][0..__k - 1]
	    __pieces = new std::vector<
	    std::pair<_DifferenceType, _DifferenceType> >[__num_threads];
	    for (_ThreadIndex __s = 0; __s < __num_threads; ++__s)
	      __pieces[__s].resize(__k);

	    _DifferenceType __num_samples =
	      __gnu_parallel::_Settings::get().merge_oversampling
	      * __num_threads;

	    __splitter(__ne_seqs, __ne_seqs + __k, __length, __total_length,
		       __comp, __pieces);
	  } //single

	  _ThreadIndex __iam = omp_get_thread_num();

	  _DifferenceType __target_position = 0;

	  for (_SeqNumber __c = 0; __c < __k; ++__c)
	    __target_position += __pieces[__iam][__c].first;

	  seq_type* __chunks = new seq_type[__k];

	  for (_SeqNumber __s = 0; __s < __k; ++__s)
	    __chunks[__s] = std::make_pair(__ne_seqs[__s].first
					   + __pieces[__iam][__s].first,
					   __ne_seqs[__s].first
					   + __pieces[__iam][__s].second);

	  if(__length > __target_position)
	    __sequential_multiway_merge<__stable, __sentinels>
	      (__chunks, __chunks + __k, __target + __target_position,
	       *(__seqs_begin->second), __length - __target_position, __comp);

	  delete[] __chunks;
	} // parallel

#if _GLIBCXX_PARALLEL_ASSERTIONS
	_GLIBCXX_PARALLEL_ASSERT(
          __is_sorted(__target, __target + __length, __comp));
#endif

	__k = 0;
	// Update ends of sequences.
	for (_RAIterIterator __raii = __seqs_begin;
             __raii != __seqs_end; ++__raii)
          {
            _DifferenceTp __length = _GLIBCXX_PARALLEL_LENGTH(*__raii);
            if(__length > 0)
              (*__raii).first += __pieces[__num_threads - 1][__k++].second;
          }

	delete[] __pieces;
	delete[] __ne_seqs;

	return __target + __length;
      }

  /**
   * @brief Multiway Merge Frontend.
   *
   * Merge the sequences specified by seqs_begin and __seqs_end into
   * __target.  __seqs_begin and __seqs_end must point to a sequence of
   * pairs.  These pairs must contain an iterator to the beginning
   * of a sequence in their first entry and an iterator the _M_end of
   * the same sequence in their second entry.
   *
   * Ties are broken arbitrarily.  See stable_multiway_merge for a variant
   * that breaks ties by sequence number but is slower.
   *
   * The first entries of the pairs (i.e. the begin iterators) will be moved
   * forward.
   *
   * The output sequence has to provide enough space for all elements
   * that are written to it.
   *
   * This function will merge the input sequences:
   *
   * - not stable
   * - parallel, depending on the input size and Settings
   * - using sampling for splitting
   * - not using sentinels
   *
   * Example:
   *
   * <pre>
   *   int sequences[10][10];
   *   for (int __i = 0; __i < 10; ++__i)
   *     for (int __j = 0; __i < 10; ++__j)
   *       sequences[__i][__j] = __j;
   *
   *   int __out[33];
   *   std::vector<std::pair<int*> > seqs;
   *   for (int __i = 0; __i < 10; ++__i)
   *     { seqs.push(std::make_pair<int*>(sequences[__i],
   *                                      sequences[__i] + 10)) }
   *
   *   multiway_merge(seqs.begin(), seqs.end(), __target, std::less<int>(), 33);
   * </pre>
   *
   * @see stable_multiway_merge
   *
   * @pre All input sequences must be sorted.
   * @pre Target must provide enough space to merge out length elements or
   *    the number of elements in all sequences, whichever is smaller.
   *
   * @post [__target, return __value) contains merged __elements from the
   *    input sequences.
   * @post return __value - __target = min(__length, number of elements in all
   *    sequences).
   *
   * @tparam _RAIterPairIterator iterator over sequence
   *    of pairs of iterators
   * @tparam _RAIterOut iterator over target sequence
   * @tparam _DifferenceTp difference type for the sequence
   * @tparam _Compare strict weak ordering type to compare elements
   *    in sequences
   *
   * @param __seqs_begin  __begin of sequence __sequence
   * @param __seqs_end    _M_end of sequence __sequence
   * @param __target      target sequence to merge to.
   * @param __comp        strict weak ordering to use for element comparison.
   * @param __length Maximum length to merge, possibly larger than the
   * number of elements available.
   *
   * @return _M_end iterator of output sequence
   */
  // multiway_merge
  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    multiway_merge(_RAIterPairIterator __seqs_begin,
		   _RAIterPairIterator __seqs_end,
		   _RAIterOut __target,
		   _DifferenceTp __length, _Compare __comp,
		   __gnu_parallel::sequential_tag)
    {
      typedef _DifferenceTp _DifferenceType;
      _GLIBCXX_CALL(__seqs_end - __seqs_begin)

      // catch special case: no sequences
      if (__seqs_begin == __seqs_end)
	return __target;

      // Execute multiway merge *sequentially*.
      return __sequential_multiway_merge
	</* __stable = */ false, /* __sentinels = */ false>
	(__seqs_begin, __seqs_end, __target,
	 *(__seqs_begin->second), __length, __comp);
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    multiway_merge(_RAIterPairIterator __seqs_begin,
		   _RAIterPairIterator __seqs_end,
		   _RAIterOut __target,
		   _DifferenceTp __length, _Compare __comp,
		   __gnu_parallel::exact_tag __tag)
    {
      typedef _DifferenceTp _DifferenceType;
      _GLIBCXX_CALL(__seqs_end - __seqs_begin)

      // catch special case: no sequences
      if (__seqs_begin == __seqs_end)
	return __target;

      // Execute merge; maybe parallel, depending on the number of merged
      // elements and the number of sequences and global thresholds in
      // Settings.
      if ((__seqs_end - __seqs_begin > 1)
	  && _GLIBCXX_PARALLEL_CONDITION(
            ((__seqs_end - __seqs_begin) >=
               __gnu_parallel::_Settings::get().multiway_merge_minimal_k)
            && ((_SequenceIndex)__length >=
              __gnu_parallel::_Settings::get().multiway_merge_minimal_n)))
	return parallel_multiway_merge
	  </* __stable = */ false, /* __sentinels = */ false>
	  (__seqs_begin, __seqs_end, __target,
	   multiway_merge_exact_splitting</* __stable = */ false,
	   typename std::iterator_traits<_RAIterPairIterator>
	   ::value_type*, _Compare, _DifferenceTp>,
	   static_cast<_DifferenceType>(__length), __comp,
	   __tag.__get_num_threads());
      else
	return __sequential_multiway_merge
	  </* __stable = */ false, /* __sentinels = */ false>
	  (__seqs_begin, __seqs_end, __target,
	   *(__seqs_begin->second), __length, __comp);
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    multiway_merge(_RAIterPairIterator __seqs_begin,
		   _RAIterPairIterator __seqs_end,
		   _RAIterOut __target,
		   _DifferenceTp __length, _Compare __comp,
		   __gnu_parallel::sampling_tag __tag)
    {
      typedef _DifferenceTp _DifferenceType;
      _GLIBCXX_CALL(__seqs_end - __seqs_begin)

      // catch special case: no sequences
      if (__seqs_begin == __seqs_end)
	return __target;

      // Execute merge; maybe parallel, depending on the number of merged
      // elements and the number of sequences and global thresholds in
      // Settings.
      if ((__seqs_end - __seqs_begin > 1)
	  && _GLIBCXX_PARALLEL_CONDITION(
            ((__seqs_end - __seqs_begin) >=
               __gnu_parallel::_Settings::get().multiway_merge_minimal_k)
            && ((_SequenceIndex)__length >=
              __gnu_parallel::_Settings::get().multiway_merge_minimal_n)))
	return parallel_multiway_merge
	  </* __stable = */ false, /* __sentinels = */ false>
	  (__seqs_begin, __seqs_end, __target,
	   multiway_merge_exact_splitting</* __stable = */ false,
	   typename std::iterator_traits<_RAIterPairIterator>
	   ::value_type*, _Compare, _DifferenceTp>,
	   static_cast<_DifferenceType>(__length), __comp,
	   __tag.__get_num_threads());
      else
	return __sequential_multiway_merge
	  </* __stable = */ false, /* __sentinels = */ false>
	  (__seqs_begin, __seqs_end, __target,
	   *(__seqs_begin->second), __length, __comp);
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    multiway_merge(_RAIterPairIterator __seqs_begin,
		   _RAIterPairIterator __seqs_end,
		   _RAIterOut __target,
		   _DifferenceTp __length, _Compare __comp,
		   parallel_tag __tag = parallel_tag(0))
    { return multiway_merge(__seqs_begin, __seqs_end, __target, __length,
			    __comp, exact_tag(__tag.__get_num_threads())); }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    multiway_merge(_RAIterPairIterator __seqs_begin,
		   _RAIterPairIterator __seqs_end,
		   _RAIterOut __target,
		   _DifferenceTp __length, _Compare __comp,
		   default_parallel_tag __tag)
    { return multiway_merge(__seqs_begin, __seqs_end, __target, __length,
			    __comp, exact_tag(__tag.__get_num_threads())); }

  // stable_multiway_merge
  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    stable_multiway_merge(_RAIterPairIterator __seqs_begin,
			  _RAIterPairIterator __seqs_end,
			  _RAIterOut __target,
			  _DifferenceTp __length, _Compare __comp,
			  __gnu_parallel::sequential_tag)
    {
      typedef _DifferenceTp _DifferenceType;
      _GLIBCXX_CALL(__seqs_end - __seqs_begin)

      // catch special case: no sequences
      if (__seqs_begin == __seqs_end)
	return __target;

      // Execute multiway merge *sequentially*.
      return __sequential_multiway_merge
	</* __stable = */ true, /* __sentinels = */ false>
          (__seqs_begin, __seqs_end, __target,
	   *(__seqs_begin->second), __length, __comp);
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    stable_multiway_merge(_RAIterPairIterator __seqs_begin,
			  _RAIterPairIterator __seqs_end,
			  _RAIterOut __target,
			  _DifferenceTp __length, _Compare __comp,
			  __gnu_parallel::exact_tag __tag)
    {
      typedef _DifferenceTp _DifferenceType;
      _GLIBCXX_CALL(__seqs_end - __seqs_begin)

      // catch special case: no sequences
      if (__seqs_begin == __seqs_end)
	return __target;

      // Execute merge; maybe parallel, depending on the number of merged
      // elements and the number of sequences and global thresholds in
      // Settings.
      if ((__seqs_end - __seqs_begin > 1)
	  && _GLIBCXX_PARALLEL_CONDITION(
            ((__seqs_end - __seqs_begin) >=
              __gnu_parallel::_Settings::get().multiway_merge_minimal_k)
            && ((_SequenceIndex)__length >=
              __gnu_parallel::_Settings::get().multiway_merge_minimal_n)))
	return parallel_multiway_merge
          </* __stable = */ true, /* __sentinels = */ false>
	  (__seqs_begin, __seqs_end, __target,
	   multiway_merge_exact_splitting</* __stable = */ true,
	   typename std::iterator_traits<_RAIterPairIterator>
	   ::value_type*, _Compare, _DifferenceTp>,
	   static_cast<_DifferenceType>(__length), __comp,
	   __tag.__get_num_threads());
      else
	return __sequential_multiway_merge
	  </* __stable = */ true, /* __sentinels = */ false>
	  (__seqs_begin, __seqs_end, __target,
	   *(__seqs_begin->second), __length, __comp);
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    stable_multiway_merge(_RAIterPairIterator __seqs_begin,
			  _RAIterPairIterator __seqs_end,
			  _RAIterOut __target,
			  _DifferenceTp __length, _Compare __comp,
			  sampling_tag __tag)
    {
      typedef _DifferenceTp _DifferenceType;
      _GLIBCXX_CALL(__seqs_end - __seqs_begin)

      // catch special case: no sequences
      if (__seqs_begin == __seqs_end)
	return __target;

      // Execute merge; maybe parallel, depending on the number of merged
      // elements and the number of sequences and global thresholds in
      // Settings.
      if ((__seqs_end - __seqs_begin > 1)
	  && _GLIBCXX_PARALLEL_CONDITION(
            ((__seqs_end - __seqs_begin) >=
              __gnu_parallel::_Settings::get().multiway_merge_minimal_k)
            && ((_SequenceIndex)__length >=
              __gnu_parallel::_Settings::get().multiway_merge_minimal_n)))
	return parallel_multiway_merge
          </* __stable = */ true, /* __sentinels = */ false>
	  (__seqs_begin, __seqs_end, __target,
	   multiway_merge_sampling_splitting</* __stable = */ true,
	   typename std::iterator_traits<_RAIterPairIterator>
	   ::value_type*, _Compare, _DifferenceTp>,
	   static_cast<_DifferenceType>(__length), __comp,
	   __tag.__get_num_threads());
      else
	return __sequential_multiway_merge
          </* __stable = */ true, /* __sentinels = */ false>
	  (__seqs_begin, __seqs_end, __target,
	   *(__seqs_begin->second), __length, __comp);
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    stable_multiway_merge(_RAIterPairIterator __seqs_begin,
			  _RAIterPairIterator __seqs_end,
			  _RAIterOut __target,
			  _DifferenceTp __length, _Compare __comp,
			  parallel_tag __tag = parallel_tag(0))
    {
      return stable_multiway_merge
	(__seqs_begin, __seqs_end, __target, __length, __comp,
	 exact_tag(__tag.__get_num_threads()));
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    stable_multiway_merge(_RAIterPairIterator __seqs_begin,
			  _RAIterPairIterator __seqs_end,
			  _RAIterOut __target,
			  _DifferenceTp __length, _Compare __comp,
			  default_parallel_tag __tag)
    {
      return stable_multiway_merge
	(__seqs_begin, __seqs_end, __target, __length, __comp,
	 exact_tag(__tag.__get_num_threads()));
    }

  /**
   * @brief Multiway Merge Frontend.
   *
   * Merge the sequences specified by seqs_begin and __seqs_end into
   * __target.  __seqs_begin and __seqs_end must point to a sequence of
   * pairs.  These pairs must contain an iterator to the beginning
   * of a sequence in their first entry and an iterator the _M_end of
   * the same sequence in their second entry.
   *
   * Ties are broken arbitrarily.  See stable_multiway_merge for a variant
   * that breaks ties by sequence number but is slower.
   *
   * The first entries of the pairs (i.e. the begin iterators) will be moved
   * forward accordingly.
   *
   * The output sequence has to provide enough space for all elements
   * that are written to it.
   *
   * This function will merge the input sequences:
   *
   * - not stable
   * - parallel, depending on the input size and Settings
   * - using sampling for splitting
   * - using sentinels
   *
   * You have to take care that the element the _M_end iterator points to is
   * readable and contains a value that is greater than any other non-sentinel
   * value in all sequences.
   *
   * Example:
   *
   * <pre>
   *   int sequences[10][11];
   *   for (int __i = 0; __i < 10; ++__i)
   *     for (int __j = 0; __i < 11; ++__j)
   *       sequences[__i][__j] = __j; // __last one is sentinel!
   *
   *   int __out[33];
   *   std::vector<std::pair<int*> > seqs;
   *   for (int __i = 0; __i < 10; ++__i)
   *     { seqs.push(std::make_pair<int*>(sequences[__i],
   *                                      sequences[__i] + 10)) }
   *
   *   multiway_merge(seqs.begin(), seqs.end(), __target, std::less<int>(), 33);
   * </pre>
   *
   * @pre All input sequences must be sorted.
   * @pre Target must provide enough space to merge out length elements or
   *    the number of elements in all sequences, whichever is smaller.
   * @pre For each @c __i, @c __seqs_begin[__i].second must be the end
   *    marker of the sequence, but also reference the one more __sentinel
   *    element.
   *
   * @post [__target, return __value) contains merged __elements from the
   *    input sequences.
   * @post return __value - __target = min(__length, number of elements in all
   *    sequences).
   *
   * @see stable_multiway_merge_sentinels
   *
   * @tparam _RAIterPairIterator iterator over sequence
   *    of pairs of iterators
   * @tparam _RAIterOut iterator over target sequence
   * @tparam _DifferenceTp difference type for the sequence
   * @tparam _Compare strict weak ordering type to compare elements
   *    in sequences
   *
   * @param __seqs_begin  __begin of sequence __sequence
   * @param __seqs_end    _M_end of sequence __sequence
   * @param __target      target sequence to merge to.
   * @param __comp        strict weak ordering to use for element comparison.
   * @param __length Maximum length to merge, possibly larger than the
   * number of elements available.
   *
   * @return _M_end iterator of output sequence
   */
  // multiway_merge_sentinels
  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    multiway_merge_sentinels(_RAIterPairIterator __seqs_begin,
			     _RAIterPairIterator __seqs_end,
			     _RAIterOut __target,
			     _DifferenceTp __length, _Compare __comp,
			     __gnu_parallel::sequential_tag)
    {
      typedef _DifferenceTp _DifferenceType;
      _GLIBCXX_CALL(__seqs_end - __seqs_begin)

      // catch special case: no sequences
      if (__seqs_begin == __seqs_end)
	return __target;

      // Execute multiway merge *sequentially*.
      return __sequential_multiway_merge
	</* __stable = */ false, /* __sentinels = */ true>
          (__seqs_begin, __seqs_end,
           __target, *(__seqs_begin->second), __length, __comp);
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    multiway_merge_sentinels(_RAIterPairIterator __seqs_begin,
			     _RAIterPairIterator __seqs_end,
			     _RAIterOut __target,
			     _DifferenceTp __length, _Compare __comp,
			     __gnu_parallel::exact_tag __tag)
    {
      typedef _DifferenceTp _DifferenceType;
      _GLIBCXX_CALL(__seqs_end - __seqs_begin)

      // catch special case: no sequences
      if (__seqs_begin == __seqs_end)
	return __target;

      // Execute merge; maybe parallel, depending on the number of merged
      // elements and the number of sequences and global thresholds in
      // Settings.
      if ((__seqs_end - __seqs_begin > 1)
	  && _GLIBCXX_PARALLEL_CONDITION(
            ((__seqs_end - __seqs_begin) >=
              __gnu_parallel::_Settings::get().multiway_merge_minimal_k)
            && ((_SequenceIndex)__length >=
              __gnu_parallel::_Settings::get().multiway_merge_minimal_n)))
	return parallel_multiway_merge
          </* __stable = */ false, /* __sentinels = */ true>
	  (__seqs_begin, __seqs_end, __target,
	   multiway_merge_exact_splitting</* __stable = */ false,
	   typename std::iterator_traits<_RAIterPairIterator>
	   ::value_type*, _Compare, _DifferenceTp>,
	   static_cast<_DifferenceType>(__length), __comp,
	   __tag.__get_num_threads());
      else
	return __sequential_multiway_merge
          </* __stable = */ false, /* __sentinels = */ true>
	  (__seqs_begin, __seqs_end, __target,
	   *(__seqs_begin->second), __length, __comp);
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    multiway_merge_sentinels(_RAIterPairIterator __seqs_begin,
			     _RAIterPairIterator __seqs_end,
			     _RAIterOut __target,
			     _DifferenceTp __length, _Compare __comp,
			     sampling_tag __tag)
    {
      typedef _DifferenceTp _DifferenceType;
      _GLIBCXX_CALL(__seqs_end - __seqs_begin)

      // catch special case: no sequences
      if (__seqs_begin == __seqs_end)
	return __target;

      // Execute merge; maybe parallel, depending on the number of merged
      // elements and the number of sequences and global thresholds in
      // Settings.
      if ((__seqs_end - __seqs_begin > 1)
	  && _GLIBCXX_PARALLEL_CONDITION(
            ((__seqs_end - __seqs_begin) >=
              __gnu_parallel::_Settings::get().multiway_merge_minimal_k)
            && ((_SequenceIndex)__length >=
              __gnu_parallel::_Settings::get().multiway_merge_minimal_n)))
	return parallel_multiway_merge
          </* __stable = */ false, /* __sentinels = */ true>
	  (__seqs_begin, __seqs_end, __target,
	   multiway_merge_sampling_splitting</* __stable = */ false,
	   typename std::iterator_traits<_RAIterPairIterator>
	   ::value_type*, _Compare, _DifferenceTp>,
	   static_cast<_DifferenceType>(__length), __comp,
	   __tag.__get_num_threads());
      else
	return __sequential_multiway_merge
          </* __stable = */false, /* __sentinels = */ true>(
            __seqs_begin, __seqs_end, __target,
	    *(__seqs_begin->second), __length, __comp);
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    multiway_merge_sentinels(_RAIterPairIterator __seqs_begin,
			     _RAIterPairIterator __seqs_end,
			     _RAIterOut __target,
			     _DifferenceTp __length, _Compare __comp,
			     parallel_tag __tag = parallel_tag(0))
    {
      return multiway_merge_sentinels
	(__seqs_begin, __seqs_end, __target, __length, __comp,
	 exact_tag(__tag.__get_num_threads()));
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    multiway_merge_sentinels(_RAIterPairIterator __seqs_begin,
			     _RAIterPairIterator __seqs_end,
			     _RAIterOut __target,
			     _DifferenceTp __length, _Compare __comp,
			     default_parallel_tag __tag)
    {
      return multiway_merge_sentinels
	(__seqs_begin, __seqs_end, __target, __length, __comp,
	 exact_tag(__tag.__get_num_threads()));
    }

  // stable_multiway_merge_sentinels
  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    stable_multiway_merge_sentinels(_RAIterPairIterator __seqs_begin,
				    _RAIterPairIterator __seqs_end,
				    _RAIterOut __target,
				    _DifferenceTp __length, _Compare __comp,
				    __gnu_parallel::sequential_tag)
    {
      typedef _DifferenceTp _DifferenceType;
      _GLIBCXX_CALL(__seqs_end - __seqs_begin)

      // catch special case: no sequences
      if (__seqs_begin == __seqs_end)
	return __target;

      // Execute multiway merge *sequentially*.
      return __sequential_multiway_merge
	</* __stable = */ true, /* __sentinels = */ true>
	(__seqs_begin, __seqs_end, __target,
	 *(__seqs_begin->second), __length, __comp);
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    stable_multiway_merge_sentinels(_RAIterPairIterator __seqs_begin,
				    _RAIterPairIterator __seqs_end,
				    _RAIterOut __target,
				    _DifferenceTp __length, _Compare __comp,
				    __gnu_parallel::exact_tag __tag)
    {
      typedef _DifferenceTp _DifferenceType;
      _GLIBCXX_CALL(__seqs_end - __seqs_begin)

      // catch special case: no sequences
      if (__seqs_begin == __seqs_end)
	return __target;

      // Execute merge; maybe parallel, depending on the number of merged
      // elements and the number of sequences and global thresholds in
      // Settings.
      if ((__seqs_end - __seqs_begin > 1)
	  && _GLIBCXX_PARALLEL_CONDITION(
            ((__seqs_end - __seqs_begin) >=
            __gnu_parallel::_Settings::get().multiway_merge_minimal_k)
            && ((_SequenceIndex)__length >=
            __gnu_parallel::_Settings::get().multiway_merge_minimal_n)))
	return parallel_multiway_merge
          </* __stable = */ true, /* __sentinels = */ true>
	  (__seqs_begin, __seqs_end, __target,
	   multiway_merge_exact_splitting</* __stable = */ true,
	   typename std::iterator_traits<_RAIterPairIterator>
	   ::value_type*, _Compare, _DifferenceTp>,
	   static_cast<_DifferenceType>(__length), __comp,
	   __tag.__get_num_threads());
      else
	return __sequential_multiway_merge
          </* __stable = */ true, /* __sentinels = */ true>
	  (__seqs_begin, __seqs_end, __target,
	   *(__seqs_begin->second), __length, __comp);
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    stable_multiway_merge_sentinels(_RAIterPairIterator __seqs_begin,
				    _RAIterPairIterator __seqs_end,
				    _RAIterOut __target,
				    _DifferenceTp __length,
				    _Compare __comp,
				    sampling_tag __tag)
    {
      typedef _DifferenceTp _DifferenceType;
      _GLIBCXX_CALL(__seqs_end - __seqs_begin)

      // catch special case: no sequences
      if (__seqs_begin == __seqs_end)
	return __target;

      // Execute merge; maybe parallel, depending on the number of merged
      // elements and the number of sequences and global thresholds in
      // Settings.
      if ((__seqs_end - __seqs_begin > 1)
	  && _GLIBCXX_PARALLEL_CONDITION(
            ((__seqs_end - __seqs_begin) >=
              __gnu_parallel::_Settings::get().multiway_merge_minimal_k)
            && ((_SequenceIndex)__length >=
              __gnu_parallel::_Settings::get().multiway_merge_minimal_n)))
	return parallel_multiway_merge
          </* __stable = */ true, /* __sentinels = */ true>
	  (__seqs_begin, __seqs_end, __target,
	   multiway_merge_sampling_splitting</* __stable = */ true,
	   typename std::iterator_traits<_RAIterPairIterator>
	   ::value_type*, _Compare, _DifferenceTp>,
	   static_cast<_DifferenceType>(__length), __comp,
	   __tag.__get_num_threads());
      else
	return __sequential_multiway_merge
          </* __stable = */ true, /* __sentinels = */ true>
	  (__seqs_begin, __seqs_end, __target,
	   *(__seqs_begin->second), __length, __comp);
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    stable_multiway_merge_sentinels(_RAIterPairIterator __seqs_begin,
				    _RAIterPairIterator __seqs_end,
				    _RAIterOut __target,
				    _DifferenceTp __length,
				    _Compare __comp,
				    parallel_tag __tag = parallel_tag(0))
    {
      return stable_multiway_merge_sentinels
	(__seqs_begin, __seqs_end, __target, __length, __comp,
	 exact_tag(__tag.__get_num_threads()));
    }

  // public interface
  template<typename _RAIterPairIterator,
	   typename _RAIterOut,
	   typename _DifferenceTp,
	   typename _Compare>
    _RAIterOut
    stable_multiway_merge_sentinels(_RAIterPairIterator __seqs_begin,
				    _RAIterPairIterator __seqs_end,
				    _RAIterOut __target,
				    _DifferenceTp __length, _Compare __comp,
				    default_parallel_tag __tag)
    {
      return stable_multiway_merge_sentinels
	(__seqs_begin, __seqs_end, __target, __length, __comp,
	 exact_tag(__tag.__get_num_threads()));
    }
}; // namespace __gnu_parallel

#endif /* _GLIBCXX_PARALLEL_MULTIWAY_MERGE_H */
