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

/** @file parallel/algobase.h
 *  @brief Parallel STL function calls corresponding to the
 *  stl_algobase.h header.  The functions defined here mainly do case
 *  switches and call the actual parallelized versions in other files.
 *  Inlining policy: Functions that basically only contain one
 *  function call, are declared inline.
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

// Written by Johannes Singler and Felix Putze.

#ifndef _GLIBCXX_PARALLEL_ALGOBASE_H
#define _GLIBCXX_PARALLEL_ALGOBASE_H 1

#include <bits/stl_algobase.h>
#include <parallel/base.h>
#include <parallel/algorithmfwd.h>
#include <parallel/find.h>
#include <parallel/find_selectors.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
namespace __parallel
{
  // NB: equal and lexicographical_compare require mismatch.

  // Sequential fallback
  template<typename _IIter1, typename _IIter2>
    inline pair<_IIter1, _IIter2>
    mismatch(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2,
             __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::mismatch(__begin1, __end1, __begin2); }

  // Sequential fallback
  template<typename _IIter1, typename _IIter2, typename _Predicate>
    inline pair<_IIter1, _IIter2>
    mismatch(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2,
             _Predicate __pred, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::mismatch(__begin1, __end1, __begin2, __pred); }

  // Sequential fallback for input iterator case
  template<typename _IIter1, typename _IIter2,
           typename _Predicate, typename _IteratorTag1, typename _IteratorTag2>
    inline pair<_IIter1, _IIter2>
    __mismatch_switch(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2,
                      _Predicate __pred, _IteratorTag1, _IteratorTag2)
    { return _GLIBCXX_STD_A::mismatch(__begin1, __end1, __begin2, __pred); }

  // Parallel mismatch for random access iterators
  template<typename _RAIter1, typename _RAIter2, typename _Predicate>
    pair<_RAIter1, _RAIter2>
    __mismatch_switch(_RAIter1 __begin1, _RAIter1 __end1,
                      _RAIter2 __begin2, _Predicate __pred, 
                      random_access_iterator_tag, random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(true))
        {
          _RAIter1 __res =
            __gnu_parallel::__find_template(__begin1, __end1, __begin2, __pred,
                                            __gnu_parallel::
                                            __mismatch_selector()).first;
          return make_pair(__res , __begin2 + (__res - __begin1));
        }
      else
        return _GLIBCXX_STD_A::mismatch(__begin1, __end1, __begin2, __pred);
    }

  // Public interface
  template<typename _IIter1, typename _IIter2>
    inline pair<_IIter1, _IIter2>
    mismatch(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2)
    {
      typedef __gnu_parallel::_EqualTo<
	typename std::iterator_traits<_IIter1>::value_type,
	typename std::iterator_traits<_IIter2>::value_type> _EqualTo;

      return __mismatch_switch(__begin1, __end1, __begin2, _EqualTo(),
                               std::__iterator_category(__begin1),
			       std::__iterator_category(__begin2));
    }

  // Public interface
  template<typename _IIter1, typename _IIter2, typename _Predicate>
    inline pair<_IIter1, _IIter2>
    mismatch(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2,
             _Predicate __pred)
    {
      return __mismatch_switch(__begin1, __end1, __begin2, __pred,
                               std::__iterator_category(__begin1),
			       std::__iterator_category(__begin2));
    }

#if __cplusplus > 201103L
  // Sequential fallback.
  template<typename _InputIterator1, typename _InputIterator2>
    inline pair<_InputIterator1, _InputIterator2>
    mismatch(_InputIterator1 __first1, _InputIterator1 __last1,
	     _InputIterator2 __first2, _InputIterator2 __last2,
	     __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::mismatch(__first1, __last1, __first2, __last2); }

  // Sequential fallback.
  template<typename _InputIterator1, typename _InputIterator2,
	   typename _BinaryPredicate>
    inline pair<_InputIterator1, _InputIterator2>
    mismatch(_InputIterator1 __first1, _InputIterator1 __last1,
	     _InputIterator2 __first2, _InputIterator2 __last2,
	     _BinaryPredicate __binary_pred,
	     __gnu_parallel::sequential_tag)
    {
      return _GLIBCXX_STD_A::mismatch(__first1, __last1, __first2, __last2,
				      __binary_pred);
    }

  // Sequential fallback for input iterator case
  template<typename _IIter1, typename _IIter2,
           typename _Predicate, typename _IteratorTag1, typename _IteratorTag2>
    inline pair<_IIter1, _IIter2>
    __mismatch_switch(_IIter1 __begin1, _IIter1 __end1,
		      _IIter2 __begin2, _IIter2 __end2, _Predicate __pred,
		      _IteratorTag1, _IteratorTag2)
    {
      return _GLIBCXX_STD_A::mismatch(__begin1, __end1,
				      __begin2, __end2, __pred);
    }

  // Parallel mismatch for random access iterators
  template<typename _RAIter1, typename _RAIter2, typename _Predicate>
    pair<_RAIter1, _RAIter2>
    __mismatch_switch(_RAIter1 __begin1, _RAIter1 __end1,
                      _RAIter2 __begin2, _RAIter2 __end2, _Predicate __pred, 
                      random_access_iterator_tag, random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(true))
        {
	  if ((__end2 - __begin2) < (__end1 - __begin1))
	    __end1 = __begin1 + (__end2 - __begin2);

          _RAIter1 __res =
            __gnu_parallel::__find_template(__begin1, __end1, __begin2, __pred,
                                            __gnu_parallel::
                                            __mismatch_selector()).first;
          return make_pair(__res , __begin2 + (__res - __begin1));
        }
      else
        return _GLIBCXX_STD_A::mismatch(__begin1, __end1,
					__begin2, __end2, __pred);
    }

  template<typename _IIter1, typename _IIter2>
    inline pair<_IIter1, _IIter2>
    mismatch(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2, _IIter2 __end2)
    {
      typedef __gnu_parallel::_EqualTo<
	typename std::iterator_traits<_IIter1>::value_type,
	typename std::iterator_traits<_IIter2>::value_type> _EqualTo;

      return __mismatch_switch(__begin1, __end1, __begin2, __end2, _EqualTo(),
			       std::__iterator_category(__begin1),
			       std::__iterator_category(__begin2));
    }

  template<typename _InputIterator1, typename _InputIterator2,
	   typename _BinaryPredicate>
    inline pair<_InputIterator1, _InputIterator2>
    mismatch(_InputIterator1 __begin1, _InputIterator1 __end1,
	     _InputIterator2 __begin2, _InputIterator2 __end2,
	     _BinaryPredicate __binary_pred)
    {
      return __mismatch_switch(__begin1, __end1, __begin2, __end2,
			       __binary_pred,
			       std::__iterator_category(__begin1),
			       std::__iterator_category(__begin2));
    }
#endif

  // Sequential fallback
  template<typename _IIter1, typename _IIter2>
    inline bool
    equal(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2, 
          __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::equal(__begin1, __end1, __begin2); }

  // Sequential fallback
  template<typename _IIter1, typename _IIter2, typename _Predicate>
    inline bool
    equal(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2, 
          _Predicate __pred, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::equal(__begin1, __end1, __begin2, __pred); }

  // Public interface
  template<typename _IIter1, typename _IIter2>
    _GLIBCXX20_CONSTEXPR
    inline bool
    equal(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2)
    {
#if __cplusplus > 201703L
      if (std::is_constant_evaluated())
	return _GLIBCXX_STD_A::equal(__begin1, __end1, __begin2);
#endif

      return __gnu_parallel::mismatch(__begin1, __end1, __begin2).first
              == __end1;
    }

  // Public interface
  template<typename _IIter1, typename _IIter2, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    equal(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2, 
          _Predicate __pred)
    {
#if __cplusplus > 201703L
      if (std::is_constant_evaluated())
	return _GLIBCXX_STD_A::equal(__begin1, __end1, __begin2, __pred);
#endif

      return __gnu_parallel::mismatch(__begin1, __end1, __begin2, __pred).first
              == __end1;
    }

#if __cplusplus > 201103L
  // Sequential fallback
  template<typename _IIter1, typename _IIter2>
    inline bool
    equal(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2, _IIter2 __end2,
	  __gnu_parallel::sequential_tag)
    {
      return _GLIBCXX_STD_A::equal(__begin1, __end1, __begin2, __end2);
    }

  // Sequential fallback
  template<typename _IIter1, typename _IIter2, typename _BinaryPredicate>
    inline bool
    equal(_IIter1 __begin1, _IIter1 __end1,
	  _IIter2 __begin2, _IIter2 __end2, _BinaryPredicate __binary_pred,
	  __gnu_parallel::sequential_tag)
    {
      return _GLIBCXX_STD_A::equal(__begin1, __end1, __begin2, __end2,
				   __binary_pred);
    }

  // Sequential fallback for input iterator case
  template<typename _IIter1, typename _IIter2,
           typename _Predicate, typename _IteratorTag1, typename _IteratorTag2>
    inline bool
    __equal_switch(_IIter1 __begin1, _IIter1 __end1,
		   _IIter2 __begin2, _IIter2 __end2, _Predicate __pred,
		   _IteratorTag1, _IteratorTag2)
    {
      return _GLIBCXX_STD_A::equal(__begin1, __end1,
				   __begin2, __end2, __pred);
    }

  // Parallel equal for random access iterators
  template<typename _RAIter1, typename _RAIter2, typename _Predicate>
    inline bool
    __equal_switch(_RAIter1 __begin1, _RAIter1 __end1,
		   _RAIter2 __begin2, _RAIter2 __end2, _Predicate __pred, 
		   random_access_iterator_tag, random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(true))
        {
	  if (std::distance(__begin1, __end1)
	      != std::distance(__begin2, __end2))
	    return false;

	  return __gnu_parallel::mismatch(__begin1, __end1, __begin2, __end2,
					  __pred).first == __end1;
        }
      else
        return _GLIBCXX_STD_A::equal(__begin1, __end1,
				     __begin2, __end2, __pred);
    }

  template<typename _IIter1, typename _IIter2>
    _GLIBCXX20_CONSTEXPR
    inline bool
    equal(_IIter1 __begin1, _IIter1 __end1, _IIter2 __begin2, _IIter2 __end2)
    {
#if __cplusplus > 201703L
      if (std::is_constant_evaluated())
	return _GLIBCXX_STD_A::equal(__begin1, __end1, __begin2, __end2);
#endif

      typedef __gnu_parallel::_EqualTo<
	typename std::iterator_traits<_IIter1>::value_type,
	typename std::iterator_traits<_IIter2>::value_type> _EqualTo;

      return __equal_switch(__begin1, __end1, __begin2, __end2, _EqualTo(),
			    std::__iterator_category(__begin1),
			    std::__iterator_category(__begin2));
    }

  template<typename _IIter1, typename _IIter2, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    equal(_IIter1 __begin1, _IIter1 __end1,
	  _IIter2 __begin2, _IIter2 __end2, _BinaryPredicate __binary_pred)
    {
#if __cplusplus > 201703L
      if (std::is_constant_evaluated())
	return _GLIBCXX_STD_A::equal(__begin1, __end1, __begin2, __end2,
				     __binary_pred);
#endif

      return __equal_switch(__begin1, __end1, __begin2, __end2, __binary_pred,
			    std::__iterator_category(__begin1),
			    std::__iterator_category(__begin2));
    }
#endif // C++14

  // Sequential fallback
  template<typename _IIter1, typename _IIter2>
    inline bool
    lexicographical_compare(_IIter1 __begin1, _IIter1 __end1, 
                            _IIter2 __begin2, _IIter2 __end2, 
                            __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::lexicographical_compare(__begin1, __end1,
                                                     __begin2, __end2); }

  // Sequential fallback
  template<typename _IIter1, typename _IIter2, typename _Predicate>
    inline bool
    lexicographical_compare(_IIter1 __begin1, _IIter1 __end1, 
                            _IIter2 __begin2, _IIter2 __end2, 
                            _Predicate __pred, __gnu_parallel::sequential_tag)
    { return _GLIBCXX_STD_A::lexicographical_compare(
               __begin1, __end1, __begin2, __end2, __pred); }

  // Sequential fallback for input iterator case
  template<typename _IIter1, typename _IIter2,
           typename _Predicate, typename _IteratorTag1, typename _IteratorTag2>
    inline bool
    __lexicographical_compare_switch(_IIter1 __begin1, _IIter1 __end1,
                                     _IIter2 __begin2, _IIter2 __end2, 
                                     _Predicate __pred,
                                     _IteratorTag1, _IteratorTag2)
    { return _GLIBCXX_STD_A::lexicographical_compare(
               __begin1, __end1, __begin2, __end2, __pred); }

  // Parallel lexicographical_compare for random access iterators
  // Limitation: Both valuetypes must be the same
  template<typename _RAIter1, typename _RAIter2, typename _Predicate>
    bool
    __lexicographical_compare_switch(_RAIter1 __begin1, _RAIter1 __end1,
                                     _RAIter2 __begin2, _RAIter2 __end2,
                                     _Predicate __pred,
                                     random_access_iterator_tag, 
                                     random_access_iterator_tag)
    {
      if (_GLIBCXX_PARALLEL_CONDITION(true))
        {
          typedef iterator_traits<_RAIter1> _TraitsType1;
          typedef typename _TraitsType1::value_type _ValueType1;

          typedef iterator_traits<_RAIter2> _TraitsType2;
          typedef typename _TraitsType2::value_type _ValueType2;

          typedef __gnu_parallel::
                  _EqualFromLess<_ValueType1, _ValueType2, _Predicate>
                  _EqualFromLessCompare;

          // Longer sequence in first place.
          if ((__end1 - __begin1) < (__end2 - __begin2))
            {
              typedef pair<_RAIter1, _RAIter2> _SpotType;
              _SpotType __mm = __mismatch_switch(__begin1, __end1, __begin2, 
                                             _EqualFromLessCompare(__pred), 
                                             random_access_iterator_tag(), 
                                             random_access_iterator_tag());

              return (__mm.first == __end1)
                        || bool(__pred(*__mm.first, *__mm.second));
            }
          else
            {
              typedef pair<_RAIter2, _RAIter1> _SpotType;
              _SpotType __mm = __mismatch_switch(__begin2, __end2, __begin1, 
                                             _EqualFromLessCompare(__pred), 
                                             random_access_iterator_tag(), 
                                             random_access_iterator_tag());

              return (__mm.first != __end2)
                        && bool(__pred(*__mm.second, *__mm.first));
            }
        }
      else
        return _GLIBCXX_STD_A::lexicographical_compare(
                 __begin1, __end1, __begin2, __end2, __pred);
    }

  // Public interface
  template<typename _IIter1, typename _IIter2>
    _GLIBCXX20_CONSTEXPR
    inline bool
    lexicographical_compare(_IIter1 __begin1, _IIter1 __end1,
                            _IIter2 __begin2, _IIter2 __end2)
    {
#if __cplusplus > 201703L
      if (std::is_constant_evaluated())
	return _GLIBCXX_STD_A::lexicographical_compare(__begin1, __end1,
						       __begin2, __end2);
#endif

      typedef iterator_traits<_IIter1> _TraitsType1;
      typedef typename _TraitsType1::value_type _ValueType1;
      typedef typename _TraitsType1::iterator_category _IteratorCategory1;

      typedef iterator_traits<_IIter2> _TraitsType2;
      typedef typename _TraitsType2::value_type _ValueType2;
      typedef typename _TraitsType2::iterator_category _IteratorCategory2;
      typedef __gnu_parallel::_Less<_ValueType1, _ValueType2> _LessType;

      return __lexicographical_compare_switch(
               __begin1, __end1, __begin2, __end2, _LessType(),
               _IteratorCategory1(), _IteratorCategory2());
    }

  // Public interface
  template<typename _IIter1, typename _IIter2, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    inline bool
    lexicographical_compare(_IIter1 __begin1, _IIter1 __end1,
                            _IIter2 __begin2, _IIter2 __end2,
                            _Predicate __pred)
    {
#if __cplusplus > 201703L
      if (std::is_constant_evaluated())
	return _GLIBCXX_STD_A::lexicographical_compare(__begin1, __end1,
						       __begin2, __end2,
						       __pred);
#endif

      typedef iterator_traits<_IIter1> _TraitsType1;
      typedef typename _TraitsType1::iterator_category _IteratorCategory1;

      typedef iterator_traits<_IIter2> _TraitsType2;
      typedef typename _TraitsType2::iterator_category _IteratorCategory2;

      return __lexicographical_compare_switch(
               __begin1, __end1, __begin2, __end2, __pred,
               _IteratorCategory1(), _IteratorCategory2());
    }
} // end namespace
} // end namespace

#endif /* _GLIBCXX_PARALLEL_ALGOBASE_H */
