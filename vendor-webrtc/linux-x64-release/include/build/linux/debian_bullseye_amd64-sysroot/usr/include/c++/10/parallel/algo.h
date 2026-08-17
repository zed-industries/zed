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

/** @file parallel/algo.h
 *  @brief Parallel STL function calls corresponding to the stl_algo.h header.
 *
 *  The functions defined here mainly do case switches and
 *  call the actual parallelized versions in other files.
 *  Inlining policy: Functions that basically only contain one function call,
 *  are declared inline.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler and Felix Putze.

#ifndef _GLIBCXX_PARALLEL_ALGO_H
#define _GLIBCXX_PARALLEL_ALGO_H 1

#include <parallel/algorithmfwd.h>
#include <bits/stl_algobase.h>
#include <bits/stl_algo.h>
#include <parallel/iterator.h>
#include <parallel/base.h>
#include <parallel/sort.h>
#include <parallel/workstealing.h>
#include <parallel/par_loop.h>
#include <parallel/omp_loop.h>
#include <parallel/omp_loop_static.h>
#include <parallel/for_each_selectors.h>
#include <parallel/for_each.h>
#include <parallel/find.h>
#include <parallel/find_selectors.h>
#include <parallel/search.h>
#include <parallel/random_shuffle.h>
#include <parallel/partition.h>
#include <parallel/merge.h>
#include <parallel/unique_copy.h>
#include <parallel/set_operations.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
namespace __parallel
{
  // Sequential fallback
  template<typename _IIter, typename _Function>
    inline _Function
    for_each(_IIter __begin, _IIter __end, _Function __f,
	     __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::for_each(__begin, __end, __f); }

  // Sequential fallback for input iterator case
  template<typename _IIter, typename _Function, typename _IteratorTag>
    inline _Function
    __for_each_switch(_IIter __begin, _IIter __end, _Function __f,
		      _IteratorTag)
    { return for_each(__begin, __end, __f, __gnu_parallel::sequential_tag()); }

  // Parallel algorithm for random access iterators
  template<typename _RAIter, typename _Function>
    _Function
    __for_each_switch(_RAIter __begin, _RAIter __end,
		      _Function __f, random_access_iterator_tag,
		      __gnu_parallel::_Parallelism __parallelism_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().for_each_minimal_n
	    && __gnu_parallel::__is_parallel(__parallelism_tag)))
	{
	  bool __dummy;
	  __gnu_parallel::__for_each_selector<_RAIter> __functionality;

	  return __gnu_parallel::
	    __for_each_template_random_access(
	      __begin, __end, __f, __functionality,
	      __gnu_parallel::_DummyReduct(), true, __dummy, -1,
	      __parallelism_tag);
	}
      else
	return for_each(__begin, __end, __f, __gnu_parallel::sequential_tag());
    }

  // Public interface
  template<typename _Iterator, typename _Function>
    inline _Function
    for_each(_Iterator __begin, _Iterator __end, _Function __f,
	     __gnu_parallel::_Parallelism __parallelism_tag)
    {
      return __for_each_switch(__begin, __end, __f,
			       std::__iterator_category(__begin),
			       __parallelism_tag);
    }

  template<typename _Iterator, typename _Function>
    inline _Function
    for_each(_Iterator __begin, _Iterator __end, _Function __f)
    {
      return __for_each_switch(__begin, __end, __f,
			       std::__iterator_category(__begin));
    }

  // Sequential fallback
  template<typename _IIter, typename _Tp>
    inline _IIter
    find(_IIter __begin, _IIter __end, const _Tp& __val,
	 __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::find(__begin, __end, __val); }

  // Sequential fallback for input iterator case
  template<typename _IIter, typename _Tp, typename _IteratorTag>
    inline _IIter
    __find_switch(_IIter __begin, _IIter __end, const _Tp& __val,
		_IteratorTag)
    { return _GLIBCXX_STD_A::find(__begin, __end, __val); }

  // Parallel find for random access iterators
  template<typename _RAIter, typename _Tp>
    _RAIter
    __find_switch(_RAIter __begin, _RAIter __end,
		const _Tp& __val, random_access_iterator_tag)
    {
      typedef iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;

      if (_GLIBCXX_PARALLEL_CONDITION(true))
	{
	  __gnu_parallel::__binder2nd<__gnu_parallel::_EqualTo<_ValueType,
							       const _Tp&>,
				      _ValueType, const _Tp&, bool>
	    __comp(__gnu_parallel::_EqualTo<_ValueType, const _Tp&>(), __val);
	  return __gnu_parallel::__find_template(
		   __begin, __end, __begin, __comp,
		   __gnu_parallel::__find_if_selector()).first;
	}
      else
	return _GLIBCXX_STD_A::find(__begin, __end, __val);
    }

  // Public interface
  template<typename _IIter, typename _Tp>
    inline _IIter
    find(_IIter __begin, _IIter __end, const _Tp& __val)
    {
      return __find_switch(__begin, __end, __val,
			   std::__iterator_category(__begin));
    }

  // Sequential fallback
  template<typename _IIter, typename _Predicate>
    inline _IIter
    find_if(_IIter __begin, _IIter __end, _Predicate __pred,
	    __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::find_if(__begin, __end, __pred); }

  // Sequential fallback for input iterator case
  template<typename _IIter, typename _Predicate, typename _IteratorTag>
    inline _IIter
    __find_if_switch(_IIter __begin, _IIter __end, _Predicate __pred,
		   _IteratorTag)
    { return _GLIBCXX_STD_A::find_if(__begin, __end, __pred); }

  // Parallel find_if for random access iterators
  template<typename _RAIter, typename _Predicate>
    _RAIter
    __find_if_switch(_RAIter __begin, _RAIter __end,
		   _Predicate __pred, random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(true))
	return __gnu_parallel::__find_template(__begin, __end, __begin, __pred,
					     __gnu_parallel::
					     __find_if_selector()).first;
      else
	return _GLIBCXX_STD_A::find_if(__begin, __end, __pred);
    }

  // Public interface
  template<typename _IIter, typename _Predicate>
    inline _IIter
    find_if(_IIter __begin, _IIter __end, _Predicate __pred)
    {
      return __find_if_switch(__begin, __end, __pred,
			      std::__iterator_category(__begin));
    }

  // Sequential fallback
  template<typename _IIter, typename _FIterator>
    inline _IIter
    find_first_of(_IIter __begin1, _IIter __end1,
		  _FIterator __begin2, _FIterator __end2,
		  __gnu_parallel::sequential_tag)
    {
      return _GLIBCXX_STD_A::find_first_of(__begin1, __end1, __begin2, __end2);
    }

  // Sequential fallback
  template<typename _IIter, typename _FIterator,
	   typename _BinaryPredicate>
    inline _IIter
    find_first_of(_IIter __begin1, _IIter __end1,
		  _FIterator __begin2, _FIterator __end2,
		  _BinaryPredicate __comp, __gnu_parallel::sequential_tag)
  { return _GLIBCXX_STD_A::find_first_of(
	     __begin1, __end1, __begin2, __end2, __comp); }

  // Sequential fallback for input iterator type
  template<typename _IIter, typename _FIterator,
	   typename _IteratorTag1, typename _IteratorTag2>
    inline _IIter
    __find_first_of_switch(_IIter __begin1, _IIter __end1,
			   _FIterator __begin2, _FIterator __end2,
			   _IteratorTag1, _IteratorTag2)
    { return find_first_of(__begin1, __end1, __begin2, __end2,
			   __gnu_parallel::sequential_tag()); }

  // Parallel algorithm for random access iterators
  template<typename _RAIter, typename _FIterator,
	   typename _BinaryPredicate, typename _IteratorTag>
    inline _RAIter
    __find_first_of_switch(_RAIter __begin1,
			   _RAIter __end1,
			   _FIterator __begin2, _FIterator __end2,
			   _BinaryPredicate __comp, random_access_iterator_tag,
			   _IteratorTag)
    {
      return __gnu_parallel::
	__find_template(__begin1, __end1, __begin1, __comp,
		      __gnu_parallel::__find_first_of_selector
		      <_FIterator>(__begin2, __end2)).first;
    }

  // Sequential fallback for input iterator type
  template<typename _IIter, typename _FIterator,
	   typename _BinaryPredicate, typename _IteratorTag1,
	   typename _IteratorTag2>
    inline _IIter
    __find_first_of_switch(_IIter __begin1, _IIter __end1,
			   _FIterator __begin2, _FIterator __end2,
			   _BinaryPredicate __comp, _IteratorTag1, _IteratorTag2)
    { return find_first_of(__begin1, __end1, __begin2, __end2, __comp,
			   __gnu_parallel::sequential_tag()); }

  // Public interface
  template<typename _IIter, typename _FIterator,
	   typename _BinaryPredicate>
    inline _IIter
    find_first_of(_IIter __begin1, _IIter __end1,
		  _FIterator __begin2, _FIterator __end2,
		  _BinaryPredicate __comp)
    {
      return __find_first_of_switch(__begin1, __end1, __begin2, __end2, __comp,
				    std::__iterator_category(__begin1),
				    std::__iterator_category(__begin2));
    }

  // Public interface, insert default comparator
  template<typename _IIter, typename _FIterator>
    inline _IIter
    find_first_of(_IIter __begin1, _IIter __end1,
		  _FIterator __begin2, _FIterator __end2)
    {
      typedef typename std::iterator_traits<_IIter>::value_type _IValueType;
      typedef typename std::iterator_traits<_FIterator>::value_type _FValueType;

      return __gnu_parallel::find_first_of(__begin1, __end1, __begin2, __end2,
			 __gnu_parallel::_EqualTo<_IValueType, _FValueType>());
    }

  // Sequential fallback
  template<typename _IIter, typename _OutputIterator>
    inline _OutputIterator
    unique_copy(_IIter __begin1, _IIter __end1, _OutputIterator __out,
		__gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::unique_copy(__begin1, __end1, __out); }

  // Sequential fallback
  template<typename _IIter, typename _OutputIterator,
	   typename _Predicate>
    inline _OutputIterator
    unique_copy(_IIter __begin1, _IIter __end1, _OutputIterator __out,
		_Predicate __pred, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::unique_copy(__begin1, __end1, __out, __pred); }

  // Sequential fallback for input iterator case
  template<typename _IIter, typename _OutputIterator,
	   typename _Predicate, typename _IteratorTag1, typename _IteratorTag2>
    inline _OutputIterator
    __unique_copy_switch(_IIter __begin, _IIter __last,
		       _OutputIterator __out, _Predicate __pred,
		       _IteratorTag1, _IteratorTag2)
    { return _GLIBCXX_STD_A::unique_copy(__begin, __last, __out, __pred); }

  // Parallel unique_copy for random access iterators
  template<typename _RAIter, typename _RandomAccessOutputIterator,
	   typename _Predicate>
    _RandomAccessOutputIterator
    __unique_copy_switch(_RAIter __begin, _RAIter __last,
			 _RandomAccessOutputIterator __out, _Predicate __pred,
			 random_access_iterator_tag, random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__last - __begin)
	    > __gnu_parallel::_Settings::get().unique_copy_minimal_n))
	return __gnu_parallel::__parallel_unique_copy(
		 __begin, __last, __out, __pred);
      else
	return _GLIBCXX_STD_A::unique_copy(__begin, __last, __out, __pred);
    }

  // Public interface
  template<typename _IIter, typename _OutputIterator>
    inline _OutputIterator
    unique_copy(_IIter __begin1, _IIter __end1, _OutputIterator __out)
    {
      typedef typename std::iterator_traits<_IIter>::value_type _ValueType;

      return __unique_copy_switch(
	       __begin1, __end1, __out, equal_to<_ValueType>(),
	       std::__iterator_category(__begin1),
	       std::__iterator_category(__out));
    }

  // Public interface
  template<typename _IIter, typename _OutputIterator, typename _Predicate>
    inline _OutputIterator
    unique_copy(_IIter __begin1, _IIter __end1, _OutputIterator __out,
		_Predicate __pred)
    {
      return __unique_copy_switch(
	       __begin1, __end1, __out, __pred,
	       std::__iterator_category(__begin1),
	       std::__iterator_category(__out));
    }

  // Sequential fallback
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator>
    inline _OutputIterator
    set_union(_IIter1 __begin1, _IIter1 __end1,
	      _IIter2 __begin2, _IIter2 __end2,
	      _OutputIterator __out, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::set_union(
	       __begin1, __end1, __begin2, __end2, __out); }

  // Sequential fallback
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _Predicate>
    inline _OutputIterator
    set_union(_IIter1 __begin1, _IIter1 __end1,
	      _IIter2 __begin2, _IIter2 __end2,
	      _OutputIterator __out, _Predicate __pred,
	      __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::set_union(__begin1, __end1,
				       __begin2, __end2, __out, __pred); }

  // Sequential fallback for input iterator case
  template<typename _IIter1, typename _IIter2, typename _Predicate,
	   typename _OutputIterator, typename _IteratorTag1,
	   typename _IteratorTag2, typename _IteratorTag3>
    inline _OutputIterator
    __set_union_switch(
      _IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2, _IIter2 __end2,
      _OutputIterator __result, _Predicate __pred,
      _IteratorTag1, _IteratorTag2, _IteratorTag3)
    { return _GLIBCXX_STD_A::set_union(__begin1, __end1,
				       __begin2, __end2, __result, __pred); }

  // Parallel set_union for random access iterators
  template<typename _RAIter1, typename _RAIter2,
	   typename _Output_RAIter, typename _Predicate>
    _Output_RAIter
    __set_union_switch(_RAIter1 __begin1, _RAIter1 __end1,
		       _RAIter2 __begin2, _RAIter2 __end2,
		       _Output_RAIter __result, _Predicate __pred,
		       random_access_iterator_tag, random_access_iterator_tag,
		       random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end1 - __begin1)
	    >= __gnu_parallel::_Settings::get().set_union_minimal_n
	    || static_cast<__gnu_parallel::_SequenceIndex>(__end2 - __begin2)
	    >= __gnu_parallel::_Settings::get().set_union_minimal_n))
	return __gnu_parallel::__parallel_set_union(
		 __begin1, __end1, __begin2, __end2, __result, __pred);
      else
	return _GLIBCXX_STD_A::set_union(__begin1, __end1,
					 __begin2, __end2, __result, __pred);
    }

  // Public interface
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator>
    inline _OutputIterator
    set_union(_IIter1 __begin1, _IIter1 __end1,
	      _IIter2 __begin2, _IIter2 __end2, _OutputIterator __out)
    {
      typedef typename std::iterator_traits<_IIter1>::value_type _ValueType1;
      typedef typename std::iterator_traits<_IIter2>::value_type _ValueType2;

      return __set_union_switch(
	       __begin1, __end1, __begin2, __end2, __out,
	       __gnu_parallel::_Less<_ValueType1, _ValueType2>(),
	       std::__iterator_category(__begin1),
	       std::__iterator_category(__begin2),
	       std::__iterator_category(__out));
    }

  // Public interface
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _Predicate>
    inline _OutputIterator
    set_union(_IIter1 __begin1, _IIter1 __end1,
	      _IIter2 __begin2, _IIter2 __end2,
	      _OutputIterator __out, _Predicate __pred)
    {
      return __set_union_switch(
	       __begin1, __end1, __begin2, __end2, __out, __pred,
	       std::__iterator_category(__begin1),
	       std::__iterator_category(__begin2),
	       std::__iterator_category(__out));
    }

  // Sequential fallback.
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator>
    inline _OutputIterator
    set_intersection(_IIter1 __begin1, _IIter1 __end1,
		     _IIter2 __begin2, _IIter2 __end2,
		     _OutputIterator __out, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::set_intersection(__begin1, __end1,
					      __begin2, __end2, __out); }

  // Sequential fallback.
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _Predicate>
    inline _OutputIterator
    set_intersection(_IIter1 __begin1, _IIter1 __end1,
		     _IIter2 __begin2, _IIter2 __end2,
		     _OutputIterator __out, _Predicate __pred,
		     __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::set_intersection(
	       __begin1, __end1, __begin2, __end2, __out, __pred); }

  // Sequential fallback for input iterator case
  template<typename _IIter1, typename _IIter2,
	   typename _Predicate, typename _OutputIterator,
	   typename _IteratorTag1, typename _IteratorTag2,
	   typename _IteratorTag3>
    inline _OutputIterator
    __set_intersection_switch(_IIter1 __begin1, _IIter1 __end1,
			      _IIter2 __begin2, _IIter2 __end2,
			      _OutputIterator __result, _Predicate __pred,
			      _IteratorTag1, _IteratorTag2, _IteratorTag3)
    { return _GLIBCXX_STD_A::set_intersection(__begin1, __end1, __begin2,
					      __end2, __result, __pred); }

  // Parallel set_intersection for random access iterators
  template<typename _RAIter1, typename _RAIter2,
	   typename _Output_RAIter, typename _Predicate>
    _Output_RAIter
    __set_intersection_switch(_RAIter1 __begin1,
			      _RAIter1 __end1,
			      _RAIter2 __begin2,
			      _RAIter2 __end2,
			      _Output_RAIter __result,
			      _Predicate __pred,
			      random_access_iterator_tag,
			      random_access_iterator_tag,
			      random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end1 - __begin1)
	    >= __gnu_parallel::_Settings::get().set_union_minimal_n
	    || static_cast<__gnu_parallel::_SequenceIndex>(__end2 - __begin2)
	    >= __gnu_parallel::_Settings::get().set_union_minimal_n))
	return __gnu_parallel::__parallel_set_intersection(
		 __begin1, __end1, __begin2, __end2, __result, __pred);
      else
	return _GLIBCXX_STD_A::set_intersection(
		 __begin1, __end1, __begin2, __end2, __result, __pred);
    }

  // Public interface
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator>
    inline _OutputIterator
    set_intersection(_IIter1 __begin1, _IIter1 __end1,
		     _IIter2 __begin2, _IIter2 __end2,
		     _OutputIterator __out)
    {
      typedef typename std::iterator_traits<_IIter1>::value_type _ValueType1;
      typedef typename std::iterator_traits<_IIter2>::value_type _ValueType2;

      return __set_intersection_switch(
	       __begin1, __end1, __begin2, __end2, __out,
	       __gnu_parallel::_Less<_ValueType1, _ValueType2>(),
	       std::__iterator_category(__begin1),
	       std::__iterator_category(__begin2),
	       std::__iterator_category(__out));
    }

  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _Predicate>
    inline _OutputIterator
    set_intersection(_IIter1 __begin1, _IIter1 __end1,
		     _IIter2 __begin2, _IIter2 __end2,
		     _OutputIterator __out, _Predicate __pred)
    {
      return __set_intersection_switch(
	       __begin1, __end1, __begin2, __end2, __out, __pred,
	       std::__iterator_category(__begin1),
	       std::__iterator_category(__begin2),
	       std::__iterator_category(__out));
    }

  // Sequential fallback
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator>
    inline _OutputIterator
    set_symmetric_difference(_IIter1 __begin1, _IIter1 __end1,
			     _IIter2 __begin2, _IIter2 __end2,
			     _OutputIterator __out,
			     __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::set_symmetric_difference(
	       __begin1, __end1, __begin2, __end2, __out); }

  // Sequential fallback
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _Predicate>
    inline _OutputIterator
    set_symmetric_difference(_IIter1 __begin1, _IIter1 __end1,
			     _IIter2 __begin2, _IIter2 __end2,
			     _OutputIterator __out, _Predicate __pred,
			     __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::set_symmetric_difference(
	       __begin1, __end1, __begin2, __end2, __out, __pred); }

  // Sequential fallback for input iterator case
  template<typename _IIter1, typename _IIter2,
	   typename _Predicate, typename _OutputIterator,
	   typename _IteratorTag1, typename _IteratorTag2,
	   typename _IteratorTag3>
    inline _OutputIterator
    __set_symmetric_difference_switch(
	_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2, _IIter2 __end2,
	_OutputIterator __result, _Predicate __pred,
	_IteratorTag1, _IteratorTag2, _IteratorTag3)
    { return _GLIBCXX_STD_A::set_symmetric_difference(
	       __begin1, __end1, __begin2, __end2, __result, __pred); }

  // Parallel set_symmetric_difference for random access iterators
  template<typename _RAIter1, typename _RAIter2,
	   typename _Output_RAIter, typename _Predicate>
    _Output_RAIter
    __set_symmetric_difference_switch(_RAIter1 __begin1,
				      _RAIter1 __end1,
				      _RAIter2 __begin2,
				      _RAIter2 __end2,
				      _Output_RAIter __result,
				      _Predicate __pred,
				      random_access_iterator_tag,
				      random_access_iterator_tag,
				      random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
      static_cast<__gnu_parallel::_SequenceIndex>(__end1 - __begin1)
      >= __gnu_parallel::_Settings::get().set_symmetric_difference_minimal_n
      || static_cast<__gnu_parallel::_SequenceIndex>(__end2 - __begin2)
      >= __gnu_parallel::_Settings::get().set_symmetric_difference_minimal_n))
  return __gnu_parallel::__parallel_set_symmetric_difference(
	   __begin1, __end1, __begin2, __end2, __result, __pred);
      else
	return _GLIBCXX_STD_A::set_symmetric_difference(
		 __begin1, __end1, __begin2, __end2, __result, __pred);
    }

  // Public interface.
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator>
    inline _OutputIterator
    set_symmetric_difference(_IIter1 __begin1, _IIter1 __end1,
			     _IIter2 __begin2, _IIter2 __end2,
			     _OutputIterator __out)
    {
      typedef typename std::iterator_traits<_IIter1>::value_type _ValueType1;
      typedef typename std::iterator_traits<_IIter2>::value_type _ValueType2;

      return __set_symmetric_difference_switch(
	       __begin1, __end1, __begin2, __end2, __out,
	       __gnu_parallel::_Less<_ValueType1, _ValueType2>(),
	       std::__iterator_category(__begin1),
	       std::__iterator_category(__begin2),
	       std::__iterator_category(__out));
    }

  // Public interface.
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _Predicate>
    inline _OutputIterator
    set_symmetric_difference(_IIter1 __begin1, _IIter1 __end1,
			     _IIter2 __begin2, _IIter2 __end2,
			     _OutputIterator __out, _Predicate __pred)
    {
      return __set_symmetric_difference_switch(
	       __begin1, __end1, __begin2, __end2, __out, __pred,
	       std::__iterator_category(__begin1),
	       std::__iterator_category(__begin2),
	       std::__iterator_category(__out));
    }

  // Sequential fallback.
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator>
    inline _OutputIterator
    set_difference(_IIter1 __begin1, _IIter1 __end1,
		   _IIter2 __begin2, _IIter2 __end2,
		   _OutputIterator __out, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::set_difference(
	       __begin1,__end1, __begin2, __end2, __out); }

  // Sequential fallback.
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _Predicate>
    inline _OutputIterator
    set_difference(_IIter1 __begin1, _IIter1 __end1,
		   _IIter2 __begin2, _IIter2 __end2,
		   _OutputIterator __out, _Predicate __pred,
		   __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::set_difference(__begin1, __end1,
					    __begin2, __end2, __out, __pred); }

  // Sequential fallback for input iterator case.
  template<typename _IIter1, typename _IIter2, typename _Predicate,
	   typename _OutputIterator, typename _IteratorTag1,
	   typename _IteratorTag2, typename _IteratorTag3>
    inline _OutputIterator
    __set_difference_switch(_IIter1 __begin1, _IIter1 __end1,
			    _IIter2 __begin2, _IIter2 __end2,
			    _OutputIterator __result, _Predicate __pred,
			    _IteratorTag1, _IteratorTag2, _IteratorTag3)
    { return _GLIBCXX_STD_A::set_difference(
	       __begin1, __end1, __begin2, __end2, __result, __pred); }

  // Parallel set_difference for random access iterators
  template<typename _RAIter1, typename _RAIter2,
	   typename _Output_RAIter, typename _Predicate>
    _Output_RAIter
    __set_difference_switch(_RAIter1 __begin1,
			    _RAIter1 __end1,
			    _RAIter2 __begin2,
			    _RAIter2 __end2,
			    _Output_RAIter __result, _Predicate __pred,
			    random_access_iterator_tag,
			    random_access_iterator_tag,
			    random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end1 - __begin1)
	    >= __gnu_parallel::_Settings::get().set_difference_minimal_n
	    || static_cast<__gnu_parallel::_SequenceIndex>(__end2 - __begin2)
	    >= __gnu_parallel::_Settings::get().set_difference_minimal_n))
	return __gnu_parallel::__parallel_set_difference(
		 __begin1, __end1, __begin2, __end2, __result, __pred);
      else
	return _GLIBCXX_STD_A::set_difference(
		 __begin1, __end1, __begin2, __end2, __result, __pred);
    }

  // Public interface
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator>
    inline _OutputIterator
    set_difference(_IIter1 __begin1, _IIter1 __end1,
		   _IIter2 __begin2, _IIter2 __end2,
		   _OutputIterator __out)
    {
      typedef typename std::iterator_traits<_IIter1>::value_type _ValueType1;
      typedef typename std::iterator_traits<_IIter2>::value_type _ValueType2;

      return __set_difference_switch(
	       __begin1, __end1, __begin2, __end2, __out,
	       __gnu_parallel::_Less<_ValueType1, _ValueType2>(),
	       std::__iterator_category(__begin1),
	       std::__iterator_category(__begin2),
	       std::__iterator_category(__out));
    }

  // Public interface
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _Predicate>
    inline _OutputIterator
    set_difference(_IIter1 __begin1, _IIter1 __end1,
		   _IIter2 __begin2, _IIter2 __end2,
		   _OutputIterator __out, _Predicate __pred)
    {
      return __set_difference_switch(
	       __begin1, __end1, __begin2, __end2, __out, __pred,
	       std::__iterator_category(__begin1),
	       std::__iterator_category(__begin2),
	       std::__iterator_category(__out));
    }

  // Sequential fallback
  template<typename _FIterator>
    inline _FIterator
    adjacent_find(_FIterator __begin, _FIterator __end,
		  __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::adjacent_find(__begin, __end); }

  // Sequential fallback
  template<typename _FIterator, typename _BinaryPredicate>
    inline _FIterator
    adjacent_find(_FIterator __begin, _FIterator __end,
		  _BinaryPredicate __binary_pred,
		  __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::adjacent_find(__begin, __end, __binary_pred); }

  // Parallel algorithm for random access iterators
  template<typename _RAIter>
    _RAIter
    __adjacent_find_switch(_RAIter __begin, _RAIter __end,
			   random_access_iterator_tag)
    {
      typedef iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;

      if (_GLIBCXX_PARALLEL_CONDITION(true))
	{
	  _RAIter __spot = __gnu_parallel::
	      __find_template(
		__begin, __end - 1, __begin, equal_to<_ValueType>(),
		__gnu_parallel::__adjacent_find_selector())
	    .first;
	  if (__spot == (__end - 1))
	    return __end;
	  else
	    return __spot;
	}
      else
	return adjacent_find(__begin, __end, __gnu_parallel::sequential_tag());
    }

  // Sequential fallback for input iterator case
  template<typename _FIterator, typename _IteratorTag>
    inline _FIterator
    __adjacent_find_switch(_FIterator __begin, _FIterator __end,
			   _IteratorTag)
    { return adjacent_find(__begin, __end, __gnu_parallel::sequential_tag()); }

  // Public interface
  template<typename _FIterator>
    inline _FIterator
    adjacent_find(_FIterator __begin, _FIterator __end)
    {
      return __adjacent_find_switch(__begin, __end,
				    std::__iterator_category(__begin));
    }

  // Sequential fallback for input iterator case
  template<typename _FIterator, typename _BinaryPredicate,
	   typename _IteratorTag>
    inline _FIterator
    __adjacent_find_switch(_FIterator __begin, _FIterator __end,
			   _BinaryPredicate __pred, _IteratorTag)
    { return adjacent_find(__begin, __end, __pred,
			   __gnu_parallel::sequential_tag()); }

  // Parallel algorithm for random access iterators
  template<typename _RAIter, typename _BinaryPredicate>
    _RAIter
    __adjacent_find_switch(_RAIter __begin, _RAIter __end,
			   _BinaryPredicate __pred, random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(true))
	return __gnu_parallel::__find_template(__begin, __end, __begin, __pred,
					     __gnu_parallel::
					     __adjacent_find_selector()).first;
      else
	return adjacent_find(__begin, __end, __pred,
			     __gnu_parallel::sequential_tag());
    }

  // Public interface
  template<typename _FIterator, typename _BinaryPredicate>
    inline _FIterator
    adjacent_find(_FIterator __begin, _FIterator __end,
		  _BinaryPredicate __pred)
    {
      return __adjacent_find_switch(__begin, __end, __pred,
				    std::__iterator_category(__begin));
    }

  // Sequential fallback
  template<typename _IIter, typename _Tp>
    inline typename iterator_traits<_IIter>::difference_type
    count(_IIter __begin, _IIter __end, const _Tp& __value,
	  __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::count(__begin, __end, __value); }

  // Parallel code for random access iterators
  template<typename _RAIter, typename _Tp>
    typename iterator_traits<_RAIter>::difference_type
    __count_switch(_RAIter __begin, _RAIter __end,
		   const _Tp& __value, random_access_iterator_tag,
		   __gnu_parallel::_Parallelism __parallelism_tag)
    {
      typedef iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;
      typedef __gnu_parallel::_SequenceIndex _SequenceIndex;

      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().count_minimal_n
	    && __gnu_parallel::__is_parallel(__parallelism_tag)))
	{
	  __gnu_parallel::__count_selector<_RAIter, _DifferenceType>
	    __functionality;
	  _DifferenceType __res = 0;
	  __gnu_parallel::
	    __for_each_template_random_access(
	      __begin, __end, __value, __functionality,
	      std::plus<_SequenceIndex>(), __res, __res, -1,
	      __parallelism_tag);
	  return __res;
	}
      else
	return count(__begin, __end, __value,
		     __gnu_parallel::sequential_tag());
    }

  // Sequential fallback for input iterator case.
  template<typename _IIter, typename _Tp, typename _IteratorTag>
    inline typename iterator_traits<_IIter>::difference_type
    __count_switch(_IIter __begin, _IIter __end, const _Tp& __value,
		   _IteratorTag)
    { return count(__begin, __end, __value, __gnu_parallel::sequential_tag());
      }

  // Public interface.
  template<typename _IIter, typename _Tp>
    inline typename iterator_traits<_IIter>::difference_type
    count(_IIter __begin, _IIter __end, const _Tp& __value,
	  __gnu_parallel::_Parallelism __parallelism_tag)
    {
      return __count_switch(__begin, __end, __value,
			    std::__iterator_category(__begin),
			    __parallelism_tag);
    }

  template<typename _IIter, typename _Tp>
    inline typename iterator_traits<_IIter>::difference_type
    count(_IIter __begin, _IIter __end, const _Tp& __value)
    {
      return __count_switch(__begin, __end, __value,
			    std::__iterator_category(__begin));
    }


  // Sequential fallback.
  template<typename _IIter, typename _Predicate>
    inline typename iterator_traits<_IIter>::difference_type
    count_if(_IIter __begin, _IIter __end, _Predicate __pred,
	     __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::count_if(__begin, __end, __pred); }

  // Parallel count_if for random access iterators
  template<typename _RAIter, typename _Predicate>
    typename iterator_traits<_RAIter>::difference_type
    __count_if_switch(_RAIter __begin, _RAIter __end,
		      _Predicate __pred, random_access_iterator_tag,
		      __gnu_parallel::_Parallelism __parallelism_tag)
    {
      typedef iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      typedef typename _TraitsType::difference_type _DifferenceType;
      typedef __gnu_parallel::_SequenceIndex _SequenceIndex;

      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().count_minimal_n
	    && __gnu_parallel::__is_parallel(__parallelism_tag)))
	{
	  _DifferenceType __res = 0;
	  __gnu_parallel::
	    __count_if_selector<_RAIter, _DifferenceType>
	    __functionality;
	  __gnu_parallel::
	    __for_each_template_random_access(
	      __begin, __end, __pred, __functionality,
	      std::plus<_SequenceIndex>(), __res, __res, -1,
	      __parallelism_tag);
	  return __res;
	}
      else
	return count_if(__begin, __end, __pred,
			__gnu_parallel::sequential_tag());
    }

  // Sequential fallback for input iterator case.
  template<typename _IIter, typename _Predicate, typename _IteratorTag>
    inline typename iterator_traits<_IIter>::difference_type
    __count_if_switch(_IIter __begin, _IIter __end, _Predicate __pred,
		      _IteratorTag)
    { return count_if(__begin, __end, __pred,
		      __gnu_parallel::sequential_tag()); }

  // Public interface.
  template<typename _IIter, typename _Predicate>
    inline typename iterator_traits<_IIter>::difference_type
    count_if(_IIter __begin, _IIter __end, _Predicate __pred,
	     __gnu_parallel::_Parallelism __parallelism_tag)
    {
      return __count_if_switch(__begin, __end, __pred,
			       std::__iterator_category(__begin),
			       __parallelism_tag);
    }

  template<typename _IIter, typename _Predicate>
    inline typename iterator_traits<_IIter>::difference_type
    count_if(_IIter __begin, _IIter __end, _Predicate __pred)
    {
      return __count_if_switch(__begin, __end, __pred,
			       std::__iterator_category(__begin));
    }


  // Sequential fallback.
  template<typename _FIterator1, typename _FIterator2>
    inline _FIterator1
    search(_FIterator1 __begin1, _FIterator1 __end1,
	   _FIterator2 __begin2, _FIterator2 __end2,
	   __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::search(__begin1, __end1, __begin2, __end2); }

  // Parallel algorithm for random access iterator
  template<typename _RAIter1, typename _RAIter2>
    _RAIter1
    __search_switch(_RAIter1 __begin1, _RAIter1 __end1,
		    _RAIter2 __begin2, _RAIter2 __end2,
		    random_access_iterator_tag, random_access_iterator_tag)
    {
      typedef typename std::iterator_traits<_RAIter1>::value_type _ValueType1;
      typedef typename std::iterator_traits<_RAIter2>::value_type _ValueType2;

      if (_GLIBCXX_PARALLEL_CONDITION(
		static_cast<__gnu_parallel::_SequenceIndex>(__end1 - __begin1)
	    >= __gnu_parallel::_Settings::get().search_minimal_n))
	return __gnu_parallel::
	  __search_template(
	    __begin1, __end1, __begin2, __end2,
	    __gnu_parallel::_EqualTo<_ValueType1, _ValueType2>());
      else
	return search(__begin1, __end1, __begin2, __end2,
		      __gnu_parallel::sequential_tag());
    }

  // Sequential fallback for input iterator case
  template<typename _FIterator1, typename _FIterator2,
	   typename _IteratorTag1, typename _IteratorTag2>
    inline _FIterator1
    __search_switch(_FIterator1 __begin1, _FIterator1 __end1,
		    _FIterator2 __begin2, _FIterator2 __end2,
		    _IteratorTag1, _IteratorTag2)
    { return search(__begin1, __end1, __begin2, __end2,
		    __gnu_parallel::sequential_tag()); }

  // Public interface.
  template<typename _FIterator1, typename _FIterator2>
    inline _FIterator1
    search(_FIterator1 __begin1, _FIterator1 __end1,
	   _FIterator2 __begin2, _FIterator2 __end2)
    {
      return __search_switch(__begin1, __end1, __begin2, __end2,
			     std::__iterator_category(__begin1),
			     std::__iterator_category(__begin2));
    }

  // Public interface.
  template<typename _FIterator1, typename _FIterator2,
	   typename _BinaryPredicate>
    inline _FIterator1
    search(_FIterator1 __begin1, _FIterator1 __end1,
	   _FIterator2 __begin2, _FIterator2 __end2,
	   _BinaryPredicate __pred, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::search(
			       __begin1, __end1, __begin2, __end2, __pred); }

  // Parallel algorithm for random access iterator.
  template<typename _RAIter1, typename _RAIter2,
	   typename _BinaryPredicate>
    _RAIter1
    __search_switch(_RAIter1 __begin1, _RAIter1 __end1,
		    _RAIter2 __begin2, _RAIter2 __end2,
		    _BinaryPredicate __pred,
		    random_access_iterator_tag, random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
		static_cast<__gnu_parallel::_SequenceIndex>(__end1 - __begin1)
	    >= __gnu_parallel::_Settings::get().search_minimal_n))
	return __gnu_parallel::__search_template(__begin1, __end1,
					       __begin2, __end2, __pred);
      else
	return search(__begin1, __end1, __begin2, __end2, __pred,
		      __gnu_parallel::sequential_tag());
    }

  // Sequential fallback for input iterator case
  template<typename _FIterator1, typename _FIterator2,
	   typename _BinaryPredicate, typename _IteratorTag1,
	   typename _IteratorTag2>
    inline _FIterator1
    __search_switch(_FIterator1 __begin1, _FIterator1 __end1,
		    _FIterator2 __begin2, _FIterator2 __end2,
		    _BinaryPredicate __pred, _IteratorTag1, _IteratorTag2)
    { return search(__begin1, __end1, __begin2, __end2, __pred,
		    __gnu_parallel::sequential_tag()); }

  // Public interface
  template<typename _FIterator1, typename _FIterator2,
	   typename _BinaryPredicate>
    inline _FIterator1
    search(_FIterator1 __begin1, _FIterator1 __end1,
	   _FIterator2 __begin2, _FIterator2 __end2,
	   _BinaryPredicate  __pred)
    {
      return __search_switch(__begin1, __end1, __begin2, __end2, __pred,
			     std::__iterator_category(__begin1),
			     std::__iterator_category(__begin2));
    }

  // Sequential fallback
  template<typename _FIterator, typename _Integer, typename _Tp>
    inline _FIterator
    search_n(_FIterator __begin, _FIterator __end, _Integer __count,
	     const _Tp& __val, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::search_n(__begin, __end, __count, __val); }

  // Sequential fallback
  template<typename _FIterator, typename _Integer, typename _Tp,
	   typename _BinaryPredicate>
    inline _FIterator
    search_n(_FIterator __begin, _FIterator __end, _Integer __count,
	     const _Tp& __val, _BinaryPredicate __binary_pred,
	     __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::search_n(
	       __begin, __end, __count, __val, __binary_pred); }

  // Public interface.
  template<typename _FIterator, typename _Integer, typename _Tp>
    inline _FIterator
    search_n(_FIterator __begin, _FIterator __end, _Integer __count,
	     const _Tp& __val)
    {
      typedef typename iterator_traits<_FIterator>::value_type _ValueType;
      return __gnu_parallel::search_n(__begin, __end, __count, __val,
		      __gnu_parallel::_EqualTo<_ValueType, _Tp>());
    }

  // Parallel algorithm for random access iterators.
  template<typename _RAIter, typename _Integer,
	   typename _Tp, typename _BinaryPredicate>
    _RAIter
    __search_n_switch(_RAIter __begin, _RAIter __end, _Integer __count,
		      const _Tp& __val, _BinaryPredicate __binary_pred,
		      random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
		static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().search_minimal_n))
	{
	  __gnu_parallel::_PseudoSequence<_Tp, _Integer> __ps(__val, __count);
	  return __gnu_parallel::__search_template(
		   __begin, __end, __ps.begin(), __ps.end(), __binary_pred);
	}
      else
	return _GLIBCXX_STD_A::search_n(__begin, __end, __count, __val,
					__binary_pred);
    }

  // Sequential fallback for input iterator case.
  template<typename _FIterator, typename _Integer, typename _Tp,
	   typename _BinaryPredicate, typename _IteratorTag>
    inline _FIterator
    __search_n_switch(_FIterator __begin, _FIterator __end, _Integer __count,
		      const _Tp& __val, _BinaryPredicate __binary_pred,
		      _IteratorTag)
    { return _GLIBCXX_STD_A::search_n(__begin, __end, __count, __val,
				      __binary_pred); }

  // Public interface.
  template<typename _FIterator, typename _Integer, typename _Tp,
	   typename _BinaryPredicate>
    inline _FIterator
    search_n(_FIterator __begin, _FIterator __end, _Integer __count,
	     const _Tp& __val, _BinaryPredicate __binary_pred)
    {
      return __search_n_switch(__begin, __end, __count, __val, __binary_pred,
			       std::__iterator_category(__begin));
    }


  // Sequential fallback.
  template<typename _IIter, typename _OutputIterator,
	   typename _UnaryOperation>
    inline _OutputIterator
    transform(_IIter __begin, _IIter __end, _OutputIterator __result,
	      _UnaryOperation __unary_op, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::transform(__begin, __end, __result, __unary_op); }

  // Parallel unary transform for random access iterators.
  template<typename _RAIter1, typename _RAIter2,
	   typename _UnaryOperation>
    _RAIter2
    __transform1_switch(_RAIter1 __begin, _RAIter1 __end,
			_RAIter2 __result, _UnaryOperation __unary_op,
			random_access_iterator_tag, random_access_iterator_tag,
			__gnu_parallel::_Parallelism __parallelism_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().transform_minimal_n
	    && __gnu_parallel::__is_parallel(__parallelism_tag)))
	{
	  bool __dummy = true;
	  typedef __gnu_parallel::_IteratorPair<_RAIter1,
	    _RAIter2, random_access_iterator_tag> _ItTrip;
	  _ItTrip __begin_pair(__begin, __result),
		  __end_pair(__end, __result + (__end - __begin));
	  __gnu_parallel::__transform1_selector<_ItTrip> __functionality;
	  __gnu_parallel::
	    __for_each_template_random_access(
	      __begin_pair, __end_pair, __unary_op, __functionality,
	      __gnu_parallel::_DummyReduct(),
	      __dummy, __dummy, -1, __parallelism_tag);
	  return __functionality._M_finish_iterator;
	}
      else
	return transform(__begin, __end, __result, __unary_op,
			 __gnu_parallel::sequential_tag());
    }

  // Sequential fallback for input iterator case.
  template<typename _RAIter1, typename _RAIter2,
	   typename _UnaryOperation, typename _IteratorTag1,
	   typename _IteratorTag2>
    inline _RAIter2
    __transform1_switch(_RAIter1 __begin, _RAIter1 __end,
			_RAIter2 __result, _UnaryOperation __unary_op,
			_IteratorTag1, _IteratorTag2)
    { return transform(__begin, __end, __result, __unary_op,
		       __gnu_parallel::sequential_tag()); }

  // Public interface.
  template<typename _IIter, typename _OutputIterator,
	   typename _UnaryOperation>
    inline _OutputIterator
    transform(_IIter __begin, _IIter __end, _OutputIterator __result,
	      _UnaryOperation __unary_op,
	      __gnu_parallel::_Parallelism __parallelism_tag)
    {
      return __transform1_switch(__begin, __end, __result, __unary_op,
				 std::__iterator_category(__begin),
				 std::__iterator_category(__result),
				 __parallelism_tag);
    }

  template<typename _IIter, typename _OutputIterator,
	   typename _UnaryOperation>
    inline _OutputIterator
    transform(_IIter __begin, _IIter __end, _OutputIterator __result,
	      _UnaryOperation __unary_op)
    {
      return __transform1_switch(__begin, __end, __result, __unary_op,
				 std::__iterator_category(__begin),
				 std::__iterator_category(__result));
    }


  // Sequential fallback
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _BinaryOperation>
    inline _OutputIterator
    transform(_IIter1 __begin1, _IIter1 __end1,
	      _IIter2 __begin2, _OutputIterator __result,
	      _BinaryOperation __binary_op, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::transform(__begin1, __end1,
				       __begin2, __result, __binary_op); }

  // Parallel binary transform for random access iterators.
  template<typename _RAIter1, typename _RAIter2,
	   typename _RAIter3, typename _BinaryOperation>
    _RAIter3
    __transform2_switch(_RAIter1 __begin1, _RAIter1 __end1,
			_RAIter2 __begin2,
			_RAIter3 __result, _BinaryOperation __binary_op,
			random_access_iterator_tag, random_access_iterator_tag,
			random_access_iterator_tag,
			__gnu_parallel::_Parallelism __parallelism_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    (__end1 - __begin1) >=
		__gnu_parallel::_Settings::get().transform_minimal_n
	    && __gnu_parallel::__is_parallel(__parallelism_tag)))
	{
	  bool __dummy = true;
	  typedef __gnu_parallel::_IteratorTriple<_RAIter1,
	    _RAIter2, _RAIter3,
	    random_access_iterator_tag> _ItTrip;
	  _ItTrip __begin_triple(__begin1, __begin2, __result),
	    __end_triple(__end1, __begin2 + (__end1 - __begin1),
		       __result + (__end1 - __begin1));
	  __gnu_parallel::__transform2_selector<_ItTrip> __functionality;
	  __gnu_parallel::
	    __for_each_template_random_access(__begin_triple, __end_triple,
					      __binary_op, __functionality,
					      __gnu_parallel::_DummyReduct(),
					      __dummy, __dummy, -1,
					      __parallelism_tag);
	  return __functionality._M_finish_iterator;
	}
      else
	return transform(__begin1, __end1, __begin2, __result, __binary_op,
			 __gnu_parallel::sequential_tag());
    }

  // Sequential fallback for input iterator case.
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _BinaryOperation,
	   typename _Tag1, typename _Tag2, typename _Tag3>
    inline _OutputIterator
    __transform2_switch(_IIter1 __begin1, _IIter1 __end1,
			_IIter2 __begin2, _OutputIterator __result,
			_BinaryOperation __binary_op, _Tag1, _Tag2, _Tag3)
    { return transform(__begin1, __end1, __begin2, __result, __binary_op,
		       __gnu_parallel::sequential_tag()); }

  // Public interface.
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _BinaryOperation>
    inline _OutputIterator
    transform(_IIter1 __begin1, _IIter1 __end1,
	      _IIter2 __begin2, _OutputIterator __result,
	      _BinaryOperation __binary_op,
	      __gnu_parallel::_Parallelism __parallelism_tag)
    {
      return __transform2_switch(
	       __begin1, __end1, __begin2, __result, __binary_op,
	       std::__iterator_category(__begin1),
	       std::__iterator_category(__begin2),
	       std::__iterator_category(__result),
	       __parallelism_tag);
    }

  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _BinaryOperation>
    inline _OutputIterator
    transform(_IIter1 __begin1, _IIter1 __end1,
	      _IIter2 __begin2, _OutputIterator __result,
	      _BinaryOperation __binary_op)
    {
      return __transform2_switch(
	       __begin1, __end1, __begin2, __result, __binary_op,
	       std::__iterator_category(__begin1),
	       std::__iterator_category(__begin2),
	       std::__iterator_category(__result));
    }

  // Sequential fallback
  template<typename _FIterator, typename _Tp>
    inline void
    replace(_FIterator __begin, _FIterator __end, const _Tp& __old_value,
	    const _Tp& __new_value, __gnu_parallel::sequential_tag)
    { _GLIBCXX_STD_A::replace(__begin, __end, __old_value, __new_value); }

  // Sequential fallback for input iterator case
  template<typename _FIterator, typename _Tp, typename _IteratorTag>
    inline void
    __replace_switch(_FIterator __begin, _FIterator __end,
		     const _Tp& __old_value, const _Tp& __new_value,
		     _IteratorTag)
    { replace(__begin, __end, __old_value, __new_value,
	      __gnu_parallel::sequential_tag()); }

  // Parallel replace for random access iterators
  template<typename _RAIter, typename _Tp>
    inline void
    __replace_switch(_RAIter __begin, _RAIter __end,
		     const _Tp& __old_value, const _Tp& __new_value,
		     random_access_iterator_tag,
		     __gnu_parallel::_Parallelism __parallelism_tag)
    {
      // XXX parallel version is where?
      replace(__begin, __end, __old_value, __new_value,
	      __gnu_parallel::sequential_tag());
    }

  // Public interface
  template<typename _FIterator, typename _Tp>
    inline void
    replace(_FIterator __begin, _FIterator __end, const _Tp& __old_value,
	    const _Tp& __new_value,
	    __gnu_parallel::_Parallelism __parallelism_tag)
    {
      __replace_switch(__begin, __end, __old_value, __new_value,
		       std::__iterator_category(__begin),
		       __parallelism_tag);
    }

  template<typename _FIterator, typename _Tp>
    inline void
    replace(_FIterator __begin, _FIterator __end, const _Tp& __old_value,
	    const _Tp& __new_value)
    {
      __replace_switch(__begin, __end, __old_value, __new_value,
		       std::__iterator_category(__begin));
    }


  // Sequential fallback
  template<typename _FIterator, typename _Predicate, typename _Tp>
    inline void
    replace_if(_FIterator __begin, _FIterator __end, _Predicate __pred,
	       const _Tp& __new_value, __gnu_parallel::sequential_tag)
    { _GLIBCXX_STD_A::replace_if(__begin, __end, __pred, __new_value); }

  // Sequential fallback for input iterator case
  template<typename _FIterator, typename _Predicate, typename _Tp,
	   typename _IteratorTag>
    inline void
    __replace_if_switch(_FIterator __begin, _FIterator __end,
			_Predicate __pred, const _Tp& __new_value, _IteratorTag)
    { replace_if(__begin, __end, __pred, __new_value,
		 __gnu_parallel::sequential_tag()); }

  // Parallel algorithm for random access iterators.
  template<typename _RAIter, typename _Predicate, typename _Tp>
    void
    __replace_if_switch(_RAIter __begin, _RAIter __end,
			_Predicate __pred, const _Tp& __new_value,
			random_access_iterator_tag,
			__gnu_parallel::_Parallelism __parallelism_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().replace_minimal_n
	    && __gnu_parallel::__is_parallel(__parallelism_tag)))
	{
	  bool __dummy;
	  __gnu_parallel::
	    __replace_if_selector<_RAIter, _Predicate, _Tp>
	    __functionality(__new_value);
	  __gnu_parallel::
	    __for_each_template_random_access(
	      __begin, __end, __pred, __functionality,
	      __gnu_parallel::_DummyReduct(),
	      true, __dummy, -1, __parallelism_tag);
	}
      else
	replace_if(__begin, __end, __pred, __new_value,
		   __gnu_parallel::sequential_tag());
    }

  // Public interface.
  template<typename _FIterator, typename _Predicate, typename _Tp>
    inline void
    replace_if(_FIterator __begin, _FIterator __end,
	       _Predicate __pred, const _Tp& __new_value,
	       __gnu_parallel::_Parallelism __parallelism_tag)
    {
      __replace_if_switch(__begin, __end, __pred, __new_value,
			  std::__iterator_category(__begin),
			  __parallelism_tag);
    }

  template<typename _FIterator, typename _Predicate, typename _Tp>
    inline void
    replace_if(_FIterator __begin, _FIterator __end,
	       _Predicate __pred, const _Tp& __new_value)
    {
      __replace_if_switch(__begin, __end, __pred, __new_value,
			  std::__iterator_category(__begin));
    }

  // Sequential fallback
  template<typename _FIterator, typename _Generator>
    inline void
    generate(_FIterator __begin, _FIterator __end, _Generator __gen,
	     __gnu_parallel::sequential_tag)
    { _GLIBCXX_STD_A::generate(__begin, __end, __gen); }

  // Sequential fallback for input iterator case.
  template<typename _FIterator, typename _Generator, typename _IteratorTag>
    inline void
    __generate_switch(_FIterator __begin, _FIterator __end, _Generator __gen,
		      _IteratorTag)
    { generate(__begin, __end, __gen, __gnu_parallel::sequential_tag()); }

  // Parallel algorithm for random access iterators.
  template<typename _RAIter, typename _Generator>
    void
    __generate_switch(_RAIter __begin, _RAIter __end,
		      _Generator __gen, random_access_iterator_tag,
		      __gnu_parallel::_Parallelism __parallelism_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().generate_minimal_n
	    && __gnu_parallel::__is_parallel(__parallelism_tag)))
	{
	  bool __dummy;
	  __gnu_parallel::__generate_selector<_RAIter>
	    __functionality;
	  __gnu_parallel::
	    __for_each_template_random_access(
	      __begin, __end, __gen, __functionality,
	      __gnu_parallel::_DummyReduct(),
	      true, __dummy, -1, __parallelism_tag);
	}
      else
	generate(__begin, __end, __gen, __gnu_parallel::sequential_tag());
    }

  // Public interface.
  template<typename _FIterator, typename _Generator>
    inline void
    generate(_FIterator __begin, _FIterator __end,
	     _Generator __gen, __gnu_parallel::_Parallelism __parallelism_tag)
    {
      __generate_switch(__begin, __end, __gen,
			std::__iterator_category(__begin),
			__parallelism_tag);
    }

  template<typename _FIterator, typename _Generator>
    inline void
    generate(_FIterator __begin, _FIterator __end, _Generator __gen)
    {
      __generate_switch(__begin, __end, __gen,
			std::__iterator_category(__begin));
    }


  // Sequential fallback.
  template<typename _OutputIterator, typename _Size, typename _Generator>
    inline _OutputIterator
    generate_n(_OutputIterator __begin, _Size __n, _Generator __gen,
	       __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::generate_n(__begin, __n, __gen); }

  // Sequential fallback for input iterator case.
  template<typename _OutputIterator, typename _Size, typename _Generator,
	   typename _IteratorTag>
    inline _OutputIterator
    __generate_n_switch(_OutputIterator __begin, _Size __n, _Generator __gen,
			_IteratorTag)
    { return generate_n(__begin, __n, __gen,
			__gnu_parallel::sequential_tag()); }

  // Parallel algorithm for random access iterators.
  template<typename _RAIter, typename _Size, typename _Generator>
    inline _RAIter
    __generate_n_switch(_RAIter __begin, _Size __n, _Generator __gen,
			random_access_iterator_tag,
			__gnu_parallel::_Parallelism __parallelism_tag)
    {
      // XXX parallel version is where?
      return generate_n(__begin, __n, __gen, __gnu_parallel::sequential_tag());
    }

  // Public interface.
  template<typename _OutputIterator, typename _Size, typename _Generator>
    inline _OutputIterator
    generate_n(_OutputIterator __begin, _Size __n, _Generator __gen,
	       __gnu_parallel::_Parallelism __parallelism_tag)
    {
      return __generate_n_switch(__begin, __n, __gen,
				 std::__iterator_category(__begin),
				 __parallelism_tag);
    }

  template<typename _OutputIterator, typename _Size, typename _Generator>
    inline _OutputIterator
    generate_n(_OutputIterator __begin, _Size __n, _Generator __gen)
    {
      return __generate_n_switch(__begin, __n, __gen,
				 std::__iterator_category(__begin));
    }


  // Sequential fallback.
  template<typename _RAIter>
    inline void
    random_shuffle(_RAIter __begin, _RAIter __end,
		   __gnu_parallel::sequential_tag)
    { _GLIBCXX_STD_A::random_shuffle(__begin, __end); }

  // Sequential fallback.
  template<typename _RAIter, typename _RandomNumberGenerator>
    inline void
    random_shuffle(_RAIter __begin, _RAIter __end,
		   _RandomNumberGenerator& __rand,
		   __gnu_parallel::sequential_tag)
    { _GLIBCXX_STD_A::random_shuffle(__begin, __end, __rand); }


  /** @brief Functor wrapper for std::rand(). */
  template<typename _MustBeInt = int>
    struct _CRandNumber
    {
      int
      operator()(int __limit)
      { return rand() % __limit; }
    };

  // Fill in random number generator.
  template<typename _RAIter>
    inline void
    random_shuffle(_RAIter __begin, _RAIter __end)
    {
      _CRandNumber<> __r;
      // Parallelization still possible.
      __gnu_parallel::random_shuffle(__begin, __end, __r);
    }

  // Parallel algorithm for random access iterators.
  template<typename _RAIter, typename _RandomNumberGenerator>
    void
    random_shuffle(_RAIter __begin, _RAIter __end,
#if __cplusplus >= 201103L
		   _RandomNumberGenerator&& __rand)
#else
		   _RandomNumberGenerator& __rand)
#endif
    {
      if (__begin == __end)
	return;
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().random_shuffle_minimal_n))
	__gnu_parallel::__parallel_random_shuffle(__begin, __end, __rand);
      else
	__gnu_parallel::__sequential_random_shuffle(__begin, __end, __rand);
    }

  // Sequential fallback.
  template<typename _FIterator, typename _Predicate>
    inline _FIterator
    partition(_FIterator __begin, _FIterator __end,
	      _Predicate __pred, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::partition(__begin, __end, __pred); }

  // Sequential fallback for input iterator case.
  template<typename _FIterator, typename _Predicate, typename _IteratorTag>
    inline _FIterator
    __partition_switch(_FIterator __begin, _FIterator __end,
		       _Predicate __pred, _IteratorTag)
    { return partition(__begin, __end, __pred,
		       __gnu_parallel::sequential_tag()); }

  // Parallel algorithm for random access iterators.
  template<typename _RAIter, typename _Predicate>
    _RAIter
    __partition_switch(_RAIter __begin, _RAIter __end,
		       _Predicate __pred, random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().partition_minimal_n))
	{
	  typedef typename std::iterator_traits<_RAIter>::
	    difference_type _DifferenceType;
	  _DifferenceType __middle = __gnu_parallel::
	    __parallel_partition(__begin, __end, __pred,
			       __gnu_parallel::__get_max_threads());
	  return __begin + __middle;
	}
      else
	return partition(__begin, __end, __pred,
			 __gnu_parallel::sequential_tag());
    }

  // Public interface.
  template<typename _FIterator, typename _Predicate>
    inline _FIterator
    partition(_FIterator __begin, _FIterator __end, _Predicate __pred)
    {
      return __partition_switch(__begin, __end, __pred,
				std::__iterator_category(__begin));
    }

  // sort interface

  // Sequential fallback
  template<typename _RAIter>
    inline void
    sort(_RAIter __begin, _RAIter __end,
	 __gnu_parallel::sequential_tag)
    { _GLIBCXX_STD_A::sort(__begin, __end); }

  // Sequential fallback
  template<typename _RAIter, typename _Compare>
    inline void
    sort(_RAIter __begin, _RAIter __end, _Compare __comp,
	 __gnu_parallel::sequential_tag)
    { _GLIBCXX_STD_A::sort<_RAIter, _Compare>(__begin, __end,
							     __comp); }

  // Public interface
  template<typename _RAIter, typename _Compare,
	   typename _Parallelism>
    void
    sort(_RAIter __begin, _RAIter __end, _Compare __comp,
	 _Parallelism __parallelism)
  {
    typedef typename iterator_traits<_RAIter>::value_type _ValueType;

    if (__begin != __end)
      {
	if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin) >=
	      __gnu_parallel::_Settings::get().sort_minimal_n))
	  __gnu_parallel::__parallel_sort<false>(
			    __begin, __end, __comp, __parallelism);
	else
	  sort(__begin, __end, __comp, __gnu_parallel::sequential_tag());
      }
  }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    sort(_RAIter __begin, _RAIter __end)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      sort(__begin, __end, std::less<_ValueType>(),
	   __gnu_parallel::default_parallel_tag());
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    sort(_RAIter __begin, _RAIter __end,
	 __gnu_parallel::default_parallel_tag __parallelism)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      sort(__begin, __end, std::less<_ValueType>(), __parallelism);
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    sort(_RAIter __begin, _RAIter __end,
	 __gnu_parallel::parallel_tag __parallelism)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      sort(__begin, __end, std::less<_ValueType>(), __parallelism);
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    sort(_RAIter __begin, _RAIter __end,
	 __gnu_parallel::multiway_mergesort_tag __parallelism)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      sort(__begin, __end, std::less<_ValueType>(), __parallelism);
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    sort(_RAIter __begin, _RAIter __end,
	 __gnu_parallel::multiway_mergesort_sampling_tag __parallelism)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      sort(__begin, __end, std::less<_ValueType>(), __parallelism);
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    sort(_RAIter __begin, _RAIter __end,
	 __gnu_parallel::multiway_mergesort_exact_tag __parallelism)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      sort(__begin, __end, std::less<_ValueType>(), __parallelism);
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    sort(_RAIter __begin, _RAIter __end,
	 __gnu_parallel::quicksort_tag __parallelism)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      sort(__begin, __end, std::less<_ValueType>(), __parallelism);
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    sort(_RAIter __begin, _RAIter __end,
	 __gnu_parallel::balanced_quicksort_tag __parallelism)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      sort(__begin, __end, std::less<_ValueType>(), __parallelism);
    }

  // Public interface
  template<typename _RAIter, typename _Compare>
    void
    sort(_RAIter __begin, _RAIter __end, _Compare __comp)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      sort(__begin, __end, __comp, __gnu_parallel::default_parallel_tag());
    }

  // stable_sort interface


  // Sequential fallback
  template<typename _RAIter>
    inline void
    stable_sort(_RAIter __begin, _RAIter __end,
		__gnu_parallel::sequential_tag)
    { _GLIBCXX_STD_A::stable_sort(__begin, __end); }

  // Sequential fallback
  template<typename _RAIter, typename _Compare>
    inline void
    stable_sort(_RAIter __begin, _RAIter __end,
		_Compare __comp, __gnu_parallel::sequential_tag)
    { _GLIBCXX_STD_A::stable_sort<_RAIter, _Compare>(__begin, __end, __comp); }

  // Public interface
  template<typename _RAIter, typename _Compare,
	   typename _Parallelism>
    void
    stable_sort(_RAIter __begin, _RAIter __end,
		_Compare __comp, _Parallelism __parallelism)
    {
      typedef iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;

      if (__begin != __end)
	{
	  if (_GLIBCXX_PARALLEL_CONDITION(
		static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin) >=
		__gnu_parallel::_Settings::get().sort_minimal_n))
	    __gnu_parallel::__parallel_sort<true>(__begin, __end,
						  __comp, __parallelism);
	  else
	    stable_sort(__begin, __end, __comp,
			__gnu_parallel::sequential_tag());
	}
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    stable_sort(_RAIter __begin, _RAIter __end)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      stable_sort(__begin, __end, std::less<_ValueType>(),
		  __gnu_parallel::default_parallel_tag());
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    stable_sort(_RAIter __begin, _RAIter __end,
		__gnu_parallel::default_parallel_tag __parallelism)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      stable_sort(__begin, __end, std::less<_ValueType>(), __parallelism);
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    stable_sort(_RAIter __begin, _RAIter __end,
		__gnu_parallel::parallel_tag __parallelism)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      stable_sort(__begin, __end, std::less<_ValueType>(), __parallelism);
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    stable_sort(_RAIter __begin, _RAIter __end,
		__gnu_parallel::multiway_mergesort_tag __parallelism)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      stable_sort(__begin, __end, std::less<_ValueType>(), __parallelism);
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    stable_sort(_RAIter __begin, _RAIter __end,
		__gnu_parallel::quicksort_tag __parallelism)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      stable_sort(__begin, __end, std::less<_ValueType>(), __parallelism);
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    stable_sort(_RAIter __begin, _RAIter __end,
		__gnu_parallel::balanced_quicksort_tag __parallelism)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      stable_sort(__begin, __end, std::less<_ValueType>(), __parallelism);
    }

  // Public interface
  template<typename _RAIter, typename _Compare>
    void
    stable_sort(_RAIter __begin, _RAIter __end, _Compare __comp)
    {
      stable_sort(
	__begin, __end, __comp, __gnu_parallel::default_parallel_tag());
    }

  // Sequential fallback
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator>
    inline _OutputIterator
    merge(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2,
	  _IIter2 __end2, _OutputIterator __result,
	  __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::merge(
	       __begin1, __end1, __begin2, __end2, __result); }

  // Sequential fallback
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _Compare>
    inline _OutputIterator
    merge(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2,
	  _IIter2 __end2, _OutputIterator __result, _Compare __comp,
	  __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::merge(
		__begin1, __end1, __begin2, __end2, __result, __comp); }

  // Sequential fallback for input iterator case
  template<typename _IIter1, typename _IIter2, typename _OutputIterator,
	   typename _Compare, typename _IteratorTag1,
	   typename _IteratorTag2, typename _IteratorTag3>
    inline _OutputIterator
    __merge_switch(_IIter1 __begin1, _IIter1 __end1,
		   _IIter2 __begin2, _IIter2 __end2,
		   _OutputIterator __result, _Compare __comp,
		   _IteratorTag1, _IteratorTag2, _IteratorTag3)
     { return _GLIBCXX_STD_A::merge(__begin1, __end1, __begin2, __end2,
				    __result, __comp); }

  // Parallel algorithm for random access iterators
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _Compare>
    _OutputIterator
    __merge_switch(_IIter1 __begin1, _IIter1 __end1,
		   _IIter2 __begin2, _IIter2 __end2,
		   _OutputIterator __result, _Compare __comp,
		   random_access_iterator_tag, random_access_iterator_tag,
		   random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    (static_cast<__gnu_parallel::_SequenceIndex>(__end1 - __begin1)
	     >= __gnu_parallel::_Settings::get().merge_minimal_n
	     || static_cast<__gnu_parallel::_SequenceIndex>(__end2 - __begin2)
	     >= __gnu_parallel::_Settings::get().merge_minimal_n)))
	return __gnu_parallel::__parallel_merge_advance(
		 __begin1, __end1, __begin2, __end2, __result,
		 (__end1 - __begin1) + (__end2 - __begin2), __comp);
      else
	return __gnu_parallel::__merge_advance(
		 __begin1, __end1, __begin2, __end2, __result,
		 (__end1 - __begin1) + (__end2 - __begin2), __comp);
  }

  // Public interface
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator, typename _Compare>
    inline _OutputIterator
    merge(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2,
	  _IIter2 __end2, _OutputIterator __result, _Compare __comp)
    {
      return __merge_switch(
		__begin1, __end1, __begin2, __end2, __result, __comp,
		std::__iterator_category(__begin1),
		std::__iterator_category(__begin2),
		std::__iterator_category(__result));
  }

  // Public interface, insert default comparator
  template<typename _IIter1, typename _IIter2,
	   typename _OutputIterator>
    inline _OutputIterator
    merge(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2,
	  _IIter2 __end2, _OutputIterator __result)
    {
      typedef typename std::iterator_traits<_IIter1>::value_type _ValueType1;
      typedef typename std::iterator_traits<_IIter2>::value_type _ValueType2;

      return __gnu_parallel::merge(__begin1, __end1, __begin2, __end2,
		__result, __gnu_parallel::_Less<_ValueType1, _ValueType2>());
    }

  // Sequential fallback
  template<typename _RAIter>
    inline void
    nth_element(_RAIter __begin, _RAIter __nth,
		_RAIter __end, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::nth_element(__begin, __nth, __end); }

  // Sequential fallback
  template<typename _RAIter, typename _Compare>
    inline void
    nth_element(_RAIter __begin, _RAIter __nth,
		_RAIter __end, _Compare __comp,
		__gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::nth_element(__begin, __nth, __end, __comp); }

  // Public interface
  template<typename _RAIter, typename _Compare>
    inline void
    nth_element(_RAIter __begin, _RAIter __nth,
		_RAIter __end, _Compare __comp)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().nth_element_minimal_n))
	__gnu_parallel::__parallel_nth_element(__begin, __nth, __end, __comp);
      else
	nth_element(__begin, __nth, __end, __comp,
		    __gnu_parallel::sequential_tag());
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    nth_element(_RAIter __begin, _RAIter __nth,
		_RAIter __end)
    {
      typedef typename iterator_traits<_RAIter>::value_type _ValueType;
      __gnu_parallel::nth_element(__begin, __nth, __end,
				  std::less<_ValueType>());
    }

  // Sequential fallback
  template<typename _RAIter, typename _Compare>
    inline void
    partial_sort(_RAIter __begin, _RAIter __middle,
		 _RAIter __end, _Compare __comp,
		 __gnu_parallel::sequential_tag)
    { _GLIBCXX_STD_A::partial_sort(__begin, __middle, __end, __comp); }

  // Sequential fallback
  template<typename _RAIter>
    inline void
    partial_sort(_RAIter __begin, _RAIter __middle,
		 _RAIter __end, __gnu_parallel::sequential_tag)
    { _GLIBCXX_STD_A::partial_sort(__begin, __middle, __end); }

  // Public interface, parallel algorithm for random access iterators
  template<typename _RAIter, typename _Compare>
    void
    partial_sort(_RAIter __begin, _RAIter __middle,
		 _RAIter __end, _Compare __comp)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().partial_sort_minimal_n))
	__gnu_parallel::
	  __parallel_partial_sort(__begin, __middle, __end, __comp);
      else
	partial_sort(__begin, __middle, __end, __comp,
		     __gnu_parallel::sequential_tag());
    }

  // Public interface, insert default comparator
  template<typename _RAIter>
    inline void
    partial_sort(_RAIter __begin, _RAIter __middle,
		 _RAIter __end)
    {
      typedef iterator_traits<_RAIter> _TraitsType;
      typedef typename _TraitsType::value_type _ValueType;
      __gnu_parallel::partial_sort(__begin, __middle, __end,
				   std::less<_ValueType>());
    }

  // Sequential fallback
  template<typename _FIterator>
    inline _FIterator
    max_element(_FIterator __begin, _FIterator __end,
		__gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::max_element(__begin, __end); }

  // Sequential fallback
  template<typename _FIterator, typename _Compare>
    inline _FIterator
    max_element(_FIterator __begin, _FIterator __end, _Compare __comp,
		__gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::max_element(__begin, __end, __comp); }

  // Sequential fallback for input iterator case
  template<typename _FIterator, typename _Compare, typename _IteratorTag>
    inline _FIterator
    __max_element_switch(_FIterator __begin, _FIterator __end,
		       _Compare __comp, _IteratorTag)
    { return max_element(__begin, __end, __comp,
			 __gnu_parallel::sequential_tag()); }

  // Parallel algorithm for random access iterators
  template<typename _RAIter, typename _Compare>
    _RAIter
    __max_element_switch(_RAIter __begin, _RAIter __end,
			 _Compare __comp, random_access_iterator_tag,
			 __gnu_parallel::_Parallelism __parallelism_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().max_element_minimal_n
	    && __gnu_parallel::__is_parallel(__parallelism_tag)))
	{
	  _RAIter __res(__begin);
	  __gnu_parallel::__identity_selector<_RAIter>
	    __functionality;
	  __gnu_parallel::
	    __for_each_template_random_access(
	      __begin, __end, __gnu_parallel::_Nothing(), __functionality,
	      __gnu_parallel::__max_element_reduct<_Compare, _RAIter>(__comp),
	      __res, __res, -1, __parallelism_tag);
	  return __res;
	}
      else
	return max_element(__begin, __end, __comp,
			   __gnu_parallel::sequential_tag());
    }

  // Public interface, insert default comparator
  template<typename _FIterator>
    inline _FIterator
    max_element(_FIterator __begin, _FIterator __end,
		__gnu_parallel::_Parallelism __parallelism_tag)
    {
      typedef typename iterator_traits<_FIterator>::value_type _ValueType;
      return max_element(__begin, __end, std::less<_ValueType>(),
			 __parallelism_tag);
    }

  template<typename _FIterator>
    inline _FIterator
    max_element(_FIterator __begin, _FIterator __end)
    {
      typedef typename iterator_traits<_FIterator>::value_type _ValueType;
      return __gnu_parallel::max_element(__begin, __end,
					 std::less<_ValueType>());
    }

  // Public interface
  template<typename _FIterator, typename _Compare>
    inline _FIterator
    max_element(_FIterator __begin, _FIterator __end, _Compare __comp,
		__gnu_parallel::_Parallelism __parallelism_tag)
    {
      return __max_element_switch(__begin, __end, __comp,
				  std::__iterator_category(__begin),
				  __parallelism_tag);
    }

  template<typename _FIterator, typename _Compare>
    inline _FIterator
    max_element(_FIterator __begin, _FIterator __end, _Compare __comp)
    {
      return __max_element_switch(__begin, __end, __comp,
				  std::__iterator_category(__begin));
    }


  // Sequential fallback
  template<typename _FIterator>
    inline _FIterator
    min_element(_FIterator __begin, _FIterator __end,
		__gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::min_element(__begin, __end); }

  // Sequential fallback
  template<typename _FIterator, typename _Compare>
    inline _FIterator
    min_element(_FIterator __begin, _FIterator __end, _Compare __comp,
		__gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::min_element(__begin, __end, __comp); }

  // Sequential fallback for input iterator case
  template<typename _FIterator, typename _Compare, typename _IteratorTag>
    inline _FIterator
    __min_element_switch(_FIterator __begin, _FIterator __end,
		       _Compare __comp, _IteratorTag)
    { return min_element(__begin, __end, __comp,
			 __gnu_parallel::sequential_tag()); }

  // Parallel algorithm for random access iterators
  template<typename _RAIter, typename _Compare>
    _RAIter
    __min_element_switch(_RAIter __begin, _RAIter __end,
			 _Compare __comp, random_access_iterator_tag,
			 __gnu_parallel::_Parallelism __parallelism_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(
	    static_cast<__gnu_parallel::_SequenceIndex>(__end - __begin)
	    >= __gnu_parallel::_Settings::get().min_element_minimal_n
	    && __gnu_parallel::__is_parallel(__parallelism_tag)))
	{
	  _RAIter __res(__begin);
	  __gnu_parallel::__identity_selector<_RAIter>
	    __functionality;
	  __gnu_parallel::
	    __for_each_template_random_access(
	      __begin, __end, __gnu_parallel::_Nothing(), __functionality,
	      __gnu_parallel::__min_element_reduct<_Compare, _RAIter>(__comp),
	      __res, __res, -1, __parallelism_tag);
	  return __res;
	}
      else
	return min_element(__begin, __end, __comp,
			   __gnu_parallel::sequential_tag());
    }

  // Public interface, insert default comparator
  template<typename _FIterator>
    inline _FIterator
    min_element(_FIterator __begin, _FIterator __end,
		__gnu_parallel::_Parallelism __parallelism_tag)
    {
      typedef typename iterator_traits<_FIterator>::value_type _ValueType;
      return min_element(__begin, __end, std::less<_ValueType>(),
			 __parallelism_tag);
    }

  template<typename _FIterator>
    inline _FIterator
    min_element(_FIterator __begin, _FIterator __end)
    {
      typedef typename iterator_traits<_FIterator>::value_type _ValueType;
      return __gnu_parallel::min_element(__begin, __end,
					 std::less<_ValueType>());
    }

  // Public interface
  template<typename _FIterator, typename _Compare>
    inline _FIterator
    min_element(_FIterator __begin, _FIterator __end, _Compare __comp,
		__gnu_parallel::_Parallelism __parallelism_tag)
    {
      return __min_element_switch(__begin, __end, __comp,
				  std::__iterator_category(__begin),
				  __parallelism_tag);
    }

  template<typename _FIterator, typename _Compare>
    inline _FIterator
    min_element(_FIterator __begin, _FIterator __end, _Compare __comp)
    {
      return __min_element_switch(__begin, __end, __comp,
				  std::__iterator_category(__begin));
    }

#if __cplusplus >= 201703L
  using _GLIBCXX_STD_A::for_each_n;
  using _GLIBCXX_STD_A::sample;
#endif
} // end namespace
} // end namespace

#endif /* _GLIBCXX_PARALLEL_ALGO_H */
