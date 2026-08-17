// <parallel/algorithm> Forward declarations -*- C++ -*-

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

/** @file parallel/algorithmfwd.h
 *  This file is a GNU parallel extension to the Standard C++ Library.
 */

#ifndef _GLIBCXX_PARALLEL_ALGORITHMFWD_H
#define _GLIBCXX_PARALLEL_ALGORITHMFWD_H 1

#pragma GCC system_header

#include <parallel/tags.h>
#include <parallel/settings.h>

namespace std _GLIBCXX_VISIBILITY(default)
{
namespace __parallel
{
  template<typename _FIter>
    _FIter
    adjacent_find(_FIter, _FIter);

  template<typename _FIter>
    _FIter
    adjacent_find(_FIter, _FIter, __gnu_parallel::sequential_tag);

  template<typename _FIter, typename _IterTag>
    _FIter
    __adjacent_find_switch(_FIter, _FIter, _IterTag);

  template<typename _RAIter>
    _RAIter
    __adjacent_find_switch(_RAIter, _RAIter, random_access_iterator_tag);


  template<typename _FIter, typename _BiPredicate>
    _FIter
    adjacent_find(_FIter, _FIter, _BiPredicate);

  template<typename _FIter, typename _BiPredicate>
    _FIter
    adjacent_find(_FIter, _FIter, _BiPredicate,
                  __gnu_parallel::sequential_tag);

  template<typename _FIter, typename _BiPredicate, typename _IterTag>
    _FIter
    __adjacent_find_switch(_FIter, _FIter, _BiPredicate, _IterTag);

  template<typename _RAIter, typename _BiPredicate>
    _RAIter
    __adjacent_find_switch(_RAIter, _RAIter, _BiPredicate, 
                         random_access_iterator_tag);


  template<typename _IIter, typename _Tp>
    typename iterator_traits<_IIter>::difference_type
    count(_IIter, _IIter, const _Tp&);

  template<typename _IIter, typename _Tp>
    typename iterator_traits<_IIter>::difference_type
    count(_IIter, _IIter, const _Tp&, __gnu_parallel::sequential_tag);

  template<typename _IIter, typename _Tp>
    typename iterator_traits<_IIter>::difference_type
    count(_IIter, _IIter, const _Tp&, __gnu_parallel::_Parallelism);

  template<typename _IIter, typename _Tp, typename _IterTag>
    typename iterator_traits<_IIter>::difference_type
    __count_switch(_IIter, _IIter, const _Tp&, _IterTag);

  template<typename _RAIter, typename _Tp>
    typename iterator_traits<_RAIter>::difference_type
    __count_switch(_RAIter, _RAIter, const _Tp&, random_access_iterator_tag,
                 __gnu_parallel::_Parallelism __parallelism
                 = __gnu_parallel::parallel_unbalanced);


  template<typename _IIter, typename _Predicate>
    typename iterator_traits<_IIter>::difference_type
    count_if(_IIter, _IIter, _Predicate);

  template<typename _IIter, typename _Predicate>
    typename iterator_traits<_IIter>::difference_type
    count_if(_IIter, _IIter, _Predicate, __gnu_parallel::sequential_tag);

  template<typename _IIter, typename _Predicate>
    typename iterator_traits<_IIter>::difference_type
    count_if(_IIter, _IIter, _Predicate, __gnu_parallel::_Parallelism);

  template<typename _IIter, typename _Predicate, typename _IterTag>
    typename iterator_traits<_IIter>::difference_type
    __count_if_switch(_IIter, _IIter, _Predicate, _IterTag);

  template<typename _RAIter, typename _Predicate>
    typename iterator_traits<_RAIter>::difference_type
    __count_if_switch(_RAIter, _RAIter, _Predicate, random_access_iterator_tag,
                    __gnu_parallel::_Parallelism __parallelism
                    = __gnu_parallel::parallel_unbalanced);

  // algobase.h
  template<typename _IIter1, typename _IIter2>
    bool
    equal(_IIter1, _IIter1, _IIter2, __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _Predicate>
    bool
    equal(_IIter1, _IIter1, _IIter2, _Predicate,
          __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2>
    _GLIBCXX20_CONSTEXPR
    bool
    equal(_IIter1, _IIter1, _IIter2);

  template<typename _IIter1, typename _IIter2, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    bool
    equal(_IIter1, _IIter1, _IIter2, _Predicate);

  template<typename _IIter, typename _Tp>
    _IIter
    find(_IIter, _IIter, const _Tp&, __gnu_parallel::sequential_tag);

  template<typename _IIter, typename _Tp>
    _IIter
    find(_IIter, _IIter, const _Tp& __val);

  template<typename _IIter, typename _Tp, typename _IterTag>
    _IIter
    __find_switch(_IIter, _IIter, const _Tp&, _IterTag);

  template<typename _RAIter, typename _Tp>
    _RAIter
    __find_switch(_RAIter, _RAIter, const _Tp&, random_access_iterator_tag);

  template<typename _IIter, typename _Predicate>
    _IIter
    find_if(_IIter, _IIter, _Predicate, __gnu_parallel::sequential_tag);

  template<typename _IIter, typename _Predicate>
    _IIter
    find_if(_IIter, _IIter, _Predicate);

  template<typename _IIter, typename _Predicate, typename _IterTag>
    _IIter
    __find_if_switch(_IIter, _IIter, _Predicate, _IterTag);

  template<typename _RAIter, typename _Predicate>
    _RAIter
    __find_if_switch(_RAIter, _RAIter, _Predicate, random_access_iterator_tag);

  template<typename _IIter, typename _FIter>
    _IIter
    find_first_of(_IIter, _IIter, _FIter, _FIter,
                  __gnu_parallel::sequential_tag);

  template<typename _IIter, typename _FIter, typename _BiPredicate>
    _IIter
    find_first_of(_IIter, _IIter, _FIter, _FIter, _BiPredicate,
                  __gnu_parallel::sequential_tag);

  template<typename _IIter, typename _FIter, typename _BiPredicate>
    _IIter
    find_first_of(_IIter, _IIter, _FIter, _FIter, _BiPredicate);

  template<typename _IIter, typename _FIter>
    _IIter
    find_first_of(_IIter, _IIter, _FIter, _FIter);

  template<typename _IIter, typename _FIter,
           typename _IterTag1, typename _IterTag2>
    _IIter
    __find_first_of_switch(
      _IIter, _IIter, _FIter, _FIter, _IterTag1, _IterTag2);

  template<typename _RAIter, typename _FIter, typename _BiPredicate,
           typename _IterTag>
    _RAIter
    __find_first_of_switch(_RAIter, _RAIter, _FIter, _FIter, _BiPredicate,
                         random_access_iterator_tag, _IterTag);

  template<typename _IIter, typename _FIter, typename _BiPredicate,
           typename _IterTag1, typename _IterTag2>
    _IIter
    __find_first_of_switch(_IIter, _IIter, _FIter, _FIter, _BiPredicate,
                         _IterTag1, _IterTag2);


  template<typename _IIter, typename _Function>
    _Function
    for_each(_IIter, _IIter, _Function);

  template<typename _IIter, typename _Function>
    _Function
    for_each(_IIter, _IIter, _Function, __gnu_parallel::sequential_tag);

  template<typename _Iterator, typename _Function>
    _Function
    for_each(_Iterator, _Iterator, _Function, __gnu_parallel::_Parallelism);

  template<typename _IIter, typename _Function, typename _IterTag>
    _Function
    __for_each_switch(_IIter, _IIter, _Function, _IterTag);

  template<typename _RAIter, typename _Function>
    _Function
    __for_each_switch(_RAIter, _RAIter, _Function, random_access_iterator_tag,
                    __gnu_parallel::_Parallelism  __parallelism
                    = __gnu_parallel::parallel_balanced);


  template<typename _FIter, typename _Generator>
    void
    generate(_FIter, _FIter, _Generator);

  template<typename _FIter, typename _Generator>
    void
    generate(_FIter, _FIter, _Generator, __gnu_parallel::sequential_tag);

  template<typename _FIter, typename _Generator>
    void
    generate(_FIter, _FIter, _Generator, __gnu_parallel::_Parallelism);

  template<typename _FIter, typename _Generator, typename _IterTag>
    void
    __generate_switch(_FIter, _FIter, _Generator, _IterTag);

  template<typename _RAIter, typename _Generator>
    void
    __generate_switch(_RAIter, _RAIter, _Generator, random_access_iterator_tag,
                    __gnu_parallel::_Parallelism __parallelism
                    = __gnu_parallel::parallel_balanced);

  template<typename _OIter, typename _Size, typename _Generator>
    _OIter
    generate_n(_OIter, _Size, _Generator);

  template<typename _OIter, typename _Size, typename _Generator>
    _OIter
    generate_n(_OIter, _Size, _Generator, __gnu_parallel::sequential_tag);

  template<typename _OIter, typename _Size, typename _Generator>
    _OIter
    generate_n(_OIter, _Size, _Generator, __gnu_parallel::_Parallelism);

  template<typename _OIter, typename _Size, typename _Generator,
           typename _IterTag>
    _OIter
    __generate_n_switch(_OIter, _Size, _Generator, _IterTag);

  template<typename _RAIter, typename _Size, typename _Generator>
    _RAIter
    __generate_n_switch(_RAIter, _Size, _Generator, random_access_iterator_tag,
                      __gnu_parallel::_Parallelism __parallelism
                      = __gnu_parallel::parallel_balanced);

  template<typename _IIter1, typename _IIter2>
    bool
    lexicographical_compare(_IIter1, _IIter1, _IIter2, _IIter2,
                            __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _Predicate>
    bool
    lexicographical_compare(_IIter1, _IIter1, _IIter2, _IIter2, _Predicate,
                            __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2>
    _GLIBCXX20_CONSTEXPR
    bool
    lexicographical_compare(_IIter1, _IIter1, _IIter2, _IIter2);

  template<typename _IIter1, typename _IIter2, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    bool
    lexicographical_compare(_IIter1, _IIter1, _IIter2, _IIter2, _Predicate);

  template<typename _IIter1, typename _IIter2,
           typename _Predicate, typename _IterTag1, typename _IterTag2>
    bool
    __lexicographical_compare_switch(_IIter1, _IIter1, _IIter2, _IIter2,
                                   _Predicate, _IterTag1, _IterTag2);

  template<typename _RAIter1, typename _RAIter2, typename _Predicate>
    bool
    __lexicographical_compare_switch(_RAIter1, _RAIter1, _RAIter2, _RAIter2,
                                   _Predicate, random_access_iterator_tag,
                                   random_access_iterator_tag);

  // algo.h
  template<typename _IIter1, typename _IIter2>
    pair<_IIter1, _IIter2>
    mismatch(_IIter1, _IIter1, _IIter2, __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _Predicate>
    pair<_IIter1, _IIter2>
    mismatch(_IIter1, _IIter1, _IIter2, _Predicate,
             __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2>
    pair<_IIter1, _IIter2>
    mismatch(_IIter1, _IIter1, _IIter2);

  template<typename _IIter1, typename _IIter2, typename _Predicate>
    pair<_IIter1, _IIter2>
    mismatch(_IIter1, _IIter1, _IIter2, _Predicate);

  template<typename _IIter1, typename _IIter2, typename _Predicate,
           typename _IterTag1, typename _IterTag2>
    pair<_IIter1, _IIter2>
    __mismatch_switch(_IIter1, _IIter1, _IIter2, _Predicate,
                    _IterTag1, _IterTag2);

  template<typename _RAIter1, typename _RAIter2, typename _Predicate>
    pair<_RAIter1, _RAIter2>
    __mismatch_switch(_RAIter1, _RAIter1, _RAIter2, _Predicate,
                    random_access_iterator_tag, random_access_iterator_tag);

  template<typename _FIter1, typename _FIter2>
    _FIter1
    search(_FIter1, _FIter1, _FIter2, _FIter2, __gnu_parallel::sequential_tag);

  template<typename _FIter1, typename _FIter2>
    _FIter1
    search(_FIter1, _FIter1, _FIter2, _FIter2);

  template<typename _FIter1, typename _FIter2, typename _BiPredicate>
    _FIter1
    search(_FIter1, _FIter1, _FIter2, _FIter2, _BiPredicate,
           __gnu_parallel::sequential_tag);

  template<typename _FIter1, typename _FIter2, typename _BiPredicate>
    _FIter1
    search(_FIter1, _FIter1, _FIter2, _FIter2, _BiPredicate);

  template<typename _RAIter1, typename _RAIter2>
    _RAIter1
    __search_switch(_RAIter1, _RAIter1, _RAIter2, _RAIter2,
                  random_access_iterator_tag, random_access_iterator_tag);

  template<typename _FIter1, typename _FIter2, typename _IterTag1,
           typename _IterTag2>
    _FIter1
    __search_switch(_FIter1, _FIter1, _FIter2, _FIter2, _IterTag1, _IterTag2);

  template<typename _RAIter1, typename _RAIter2, typename _BiPredicate>
    _RAIter1
    __search_switch(_RAIter1, _RAIter1, _RAIter2, _RAIter2, _BiPredicate,
                  random_access_iterator_tag, random_access_iterator_tag);

  template<typename _FIter1, typename _FIter2, typename _BiPredicate,
           typename _IterTag1, typename _IterTag2>
    _FIter1
    __search_switch(_FIter1, _FIter1, _FIter2, _FIter2, _BiPredicate,
                  _IterTag1, _IterTag2);

  template<typename _FIter, typename _Integer, typename _Tp>
    _FIter
    search_n(_FIter, _FIter, _Integer, const _Tp&,
             __gnu_parallel::sequential_tag);

  template<typename _FIter, typename _Integer, typename _Tp,
           typename _BiPredicate>
    _FIter
    search_n(_FIter, _FIter, _Integer, const _Tp&, _BiPredicate,
             __gnu_parallel::sequential_tag);
    
  template<typename _FIter, typename _Integer, typename _Tp>
    _FIter
    search_n(_FIter, _FIter, _Integer, const _Tp&);

  template<typename _FIter, typename _Integer, typename _Tp,
           typename _BiPredicate>
    _FIter
    search_n(_FIter, _FIter, _Integer, const _Tp&, _BiPredicate);

  template<typename _RAIter, typename _Integer, typename _Tp,
           typename _BiPredicate>
    _RAIter
    __search_n_switch(_RAIter, _RAIter, _Integer, const _Tp&,
                    _BiPredicate, random_access_iterator_tag);

  template<typename _FIter, typename _Integer, typename _Tp,
           typename _BiPredicate, typename _IterTag>
    _FIter
    __search_n_switch(_FIter, _FIter, _Integer, const _Tp&,
                    _BiPredicate, _IterTag);


  template<typename _IIter, typename _OIter, typename _UnaryOperation>
    _OIter
    transform(_IIter, _IIter, _OIter, _UnaryOperation);

  template<typename _IIter, typename _OIter, typename _UnaryOperation>
    _OIter
    transform(_IIter, _IIter, _OIter, _UnaryOperation, 
              __gnu_parallel::sequential_tag);

  template<typename _IIter, typename _OIter, typename _UnaryOperation>
    _OIter
    transform(_IIter, _IIter, _OIter, _UnaryOperation, 
              __gnu_parallel::_Parallelism);

  template<typename _IIter, typename _OIter, typename _UnaryOperation,
           typename _IterTag1, typename _IterTag2>
    _OIter
    __transform1_switch(_IIter, _IIter, _OIter, _UnaryOperation, 
                      _IterTag1, _IterTag2);
    

  template<typename _RAIIter, typename _RAOIter, typename _UnaryOperation>
    _RAOIter
    __transform1_switch(_RAIIter, _RAIIter, _RAOIter, _UnaryOperation, 
                      random_access_iterator_tag, random_access_iterator_tag, 
                      __gnu_parallel::_Parallelism __parallelism
                      = __gnu_parallel::parallel_balanced);


  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _BiOperation>
    _OIter
    transform(_IIter1, _IIter1, _IIter2, _OIter, _BiOperation);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _BiOperation>
    _OIter
    transform(_IIter1, _IIter1, _IIter2, _OIter, _BiOperation, 
              __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _BiOperation>
    _OIter
    transform(_IIter1, _IIter1, _IIter2, _OIter, _BiOperation, 
              __gnu_parallel::_Parallelism);

  template<typename _RAIter1, typename _RAIter2, typename _RAIter3,
           typename _BiOperation>
    _RAIter3
    __transform2_switch(_RAIter1, _RAIter1, _RAIter2, _RAIter3, _BiOperation, 
                      random_access_iterator_tag, random_access_iterator_tag, 
                      random_access_iterator_tag,
                      __gnu_parallel::_Parallelism __parallelism
                      = __gnu_parallel::parallel_balanced);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _BiOperation, typename _Tag1,
           typename _Tag2, typename _Tag3>
    _OIter
    __transform2_switch(_IIter1, _IIter1, _IIter2, _OIter, _BiOperation, 
                      _Tag1, _Tag2, _Tag3);


  template<typename _FIter, typename _Tp>
    void
    replace(_FIter, _FIter, const _Tp&, const _Tp&);

  template<typename _FIter, typename _Tp>
    void
    replace(_FIter, _FIter, const _Tp&, const _Tp&, 
            __gnu_parallel::sequential_tag);

  template<typename _FIter, typename _Tp>
    void
    replace(_FIter, _FIter, const _Tp&, const _Tp&,
            __gnu_parallel::_Parallelism);

  template<typename _FIter, typename _Tp, typename _IterTag>
    void
    __replace_switch(_FIter, _FIter, const _Tp&, const _Tp&, _IterTag);

  template<typename _RAIter, typename _Tp>
    void
    __replace_switch(_RAIter, _RAIter, const _Tp&, const _Tp&, 
                   random_access_iterator_tag, __gnu_parallel::_Parallelism);


  template<typename _FIter, typename _Predicate, typename _Tp>
    void
    replace_if(_FIter, _FIter, _Predicate, const _Tp&);

  template<typename _FIter, typename _Predicate, typename _Tp>
    void
    replace_if(_FIter, _FIter, _Predicate, const _Tp&,
               __gnu_parallel::sequential_tag);

  template<typename _FIter, typename _Predicate, typename _Tp>
    void
    replace_if(_FIter, _FIter, _Predicate, const _Tp&,
               __gnu_parallel::_Parallelism);

  template<typename _FIter, typename _Predicate, typename _Tp,
           typename _IterTag>
    void
    __replace_if_switch(_FIter, _FIter, _Predicate, const _Tp&, _IterTag);
 
  template<typename _RAIter, typename _Predicate, typename _Tp>
    void
    __replace_if_switch(_RAIter, _RAIter, _Predicate, const _Tp&,
                      random_access_iterator_tag,
                      __gnu_parallel::_Parallelism);


  template<typename _FIter>
    _FIter
    max_element(_FIter, _FIter);

  template<typename _FIter>
    _FIter
    max_element(_FIter, _FIter, __gnu_parallel::sequential_tag);

  template<typename _FIter>
    _FIter
    max_element(_FIter, _FIter, __gnu_parallel::_Parallelism);

  template<typename _FIter, typename _Compare>
    _FIter
    max_element(_FIter, _FIter, _Compare);

  template<typename _FIter, typename _Compare>
    _FIter
    max_element(_FIter, _FIter, _Compare, __gnu_parallel::sequential_tag);

  template<typename _FIter, typename _Compare>
    _FIter
    max_element(_FIter, _FIter, _Compare, __gnu_parallel::_Parallelism);

  template<typename _FIter, typename _Compare, typename _IterTag>
    _FIter
    __max_element_switch(_FIter, _FIter, _Compare, _IterTag);

  template<typename _RAIter, typename _Compare>
    _RAIter
    __max_element_switch(
      _RAIter, _RAIter, _Compare, random_access_iterator_tag,
      __gnu_parallel::_Parallelism __parallelism
      = __gnu_parallel::parallel_balanced);


  template<typename _IIter1, typename _IIter2, typename _OIter>
    _OIter
    merge(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, 
          __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _Compare>
    _OIter
    merge(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Compare, 
          __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _Compare>
    _OIter
    merge(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Compare);

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _OIter
    merge(_IIter1, _IIter1, _IIter2, _IIter2, _OIter);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _Compare, typename _IterTag1, typename _IterTag2,
           typename _IterTag3>
    _OIter
    __merge_switch(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Compare, 
                 _IterTag1, _IterTag2, _IterTag3);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _Compare>
    _OIter
    __merge_switch(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Compare, 
                 random_access_iterator_tag, random_access_iterator_tag, 
                 random_access_iterator_tag);


  template<typename _FIter>
    _FIter
    min_element(_FIter, _FIter);

  template<typename _FIter>
    _FIter
    min_element(_FIter, _FIter, __gnu_parallel::sequential_tag);

  template<typename _FIter>
    _FIter
    min_element(_FIter, _FIter,
                __gnu_parallel::_Parallelism __parallelism_tag);

  template<typename _FIter, typename _Compare>
    _FIter
    min_element(_FIter, _FIter, _Compare);

  template<typename _FIter, typename _Compare>
    _FIter
    min_element(_FIter, _FIter, _Compare, __gnu_parallel::sequential_tag);

  template<typename _FIter, typename _Compare>
    _FIter
    min_element(_FIter, _FIter, _Compare, __gnu_parallel::_Parallelism);

  template<typename _FIter, typename _Compare, typename _IterTag>
    _FIter
    __min_element_switch(_FIter, _FIter, _Compare, _IterTag);

  template<typename _RAIter, typename _Compare>
    _RAIter
    __min_element_switch(
      _RAIter, _RAIter, _Compare, random_access_iterator_tag,
      __gnu_parallel::_Parallelism __parallelism
      = __gnu_parallel::parallel_balanced);

  template<typename _RAIter>
    void
    nth_element(_RAIter, _RAIter, _RAIter, __gnu_parallel::sequential_tag);

  template<typename _RAIter, typename _Compare>
    void
    nth_element(_RAIter, _RAIter, _RAIter, _Compare,
                __gnu_parallel::sequential_tag);

  template<typename _RAIter, typename _Compare>
    void
    nth_element(_RAIter, _RAIter, _RAIter, _Compare);

  template<typename _RAIter>
    void
    nth_element(_RAIter, _RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    void
    partial_sort(_RAIter, _RAIter, _RAIter, _Compare,
                 __gnu_parallel::sequential_tag);

  template<typename _RAIter>
    void
    partial_sort(_RAIter, _RAIter, _RAIter, __gnu_parallel::sequential_tag);

  template<typename _RAIter, typename _Compare>
    void
    partial_sort(_RAIter, _RAIter, _RAIter, _Compare);

  template<typename _RAIter>
    void
    partial_sort(_RAIter, _RAIter, _RAIter);

  template<typename _FIter, typename _Predicate>
    _FIter
    partition(_FIter, _FIter, _Predicate, __gnu_parallel::sequential_tag);
    
  template<typename _FIter, typename _Predicate>
    _FIter
    partition(_FIter, _FIter, _Predicate);

  template<typename _FIter, typename _Predicate, typename _IterTag>
    _FIter
    __partition_switch(_FIter, _FIter, _Predicate, _IterTag);
    
  template<typename _RAIter, typename _Predicate>
    _RAIter
    __partition_switch(
      _RAIter, _RAIter, _Predicate, random_access_iterator_tag);

  template<typename _RAIter>
    void
    random_shuffle(_RAIter, _RAIter, __gnu_parallel::sequential_tag);

  template<typename _RAIter, typename _RandomNumberGenerator>
    void
    random_shuffle(_RAIter, _RAIter, _RandomNumberGenerator&,
                   __gnu_parallel::sequential_tag);

  template<typename _RAIter>
    void
    random_shuffle(_RAIter, _RAIter);

  template<typename _RAIter, typename _RandomNumberGenerator>
    void
    random_shuffle(_RAIter, _RAIter,
#if __cplusplus >= 201103L
		   _RandomNumberGenerator&&);
#else
		   _RandomNumberGenerator&);
#endif

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _OIter
    set_union(_IIter1, _IIter1, _IIter2, _IIter2, _OIter,
            __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _Predicate>
    _OIter
    set_union(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Predicate,
              __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _OIter
    set_union(_IIter1, _IIter1, _IIter2, _IIter2, _OIter);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _Predicate>
    _OIter 
    set_union(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Predicate);

  template<typename _IIter1, typename _IIter2, typename _Predicate,
           typename _OIter, typename _IterTag1, typename _IterTag2,
           typename _IterTag3>
    _OIter
    __set_union_switch(_IIter1, _IIter1, _IIter2, _IIter2, _OIter,
                     _Predicate, _IterTag1, _IterTag2, _IterTag3);

  template<typename _RAIter1, typename _RAIter2, typename _Output_RAIter,
           typename _Predicate>
    _Output_RAIter
    __set_union_switch(_RAIter1, _RAIter1, _RAIter2, _RAIter2, _Output_RAIter,
                     _Predicate, random_access_iterator_tag,
                     random_access_iterator_tag, random_access_iterator_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _OIter
    set_intersection(_IIter1, _IIter1, _IIter2, _IIter2, _OIter,
                     __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _Predicate>
    _OIter
    set_intersection(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Predicate,
                     __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _OIter
    set_intersection(_IIter1, _IIter1, _IIter2, _IIter2, _OIter);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _Predicate>
    _OIter 
    set_intersection(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Predicate);

  template<typename _IIter1, typename _IIter2, typename _Predicate,
           typename _OIter, typename _IterTag1, typename _IterTag2,
           typename _IterTag3>
    _OIter
    __set_intersection_switch(_IIter1, _IIter1, _IIter2, _IIter2, _OIter,
                            _Predicate, _IterTag1, _IterTag2, _IterTag3);

  template<typename _RAIter1, typename _RAIter2, typename _Output_RAIter,
           typename _Predicate>
    _Output_RAIter
    __set_intersection_switch(_RAIter1, _RAIter1, _RAIter2, _RAIter2,
                            _Output_RAIter, _Predicate,
                            random_access_iterator_tag,
                            random_access_iterator_tag,
                            random_access_iterator_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _OIter
    set_symmetric_difference(_IIter1, _IIter1, _IIter2, _IIter2, _OIter,
                             __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _Predicate>
    _OIter
    set_symmetric_difference(_IIter1, _IIter1, _IIter2, _IIter2, _OIter,
                             _Predicate, __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _OIter 
    set_symmetric_difference(_IIter1, _IIter1, _IIter2, _IIter2, _OIter);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _Predicate>
    _OIter 
    set_symmetric_difference(_IIter1, _IIter1, _IIter2, _IIter2, _OIter,
                             _Predicate);

  template<typename _IIter1, typename _IIter2, typename _Predicate,
           typename _OIter, typename _IterTag1, typename _IterTag2,
           typename _IterTag3>
    _OIter
    __set_symmetric_difference_switch(_IIter1, _IIter1, _IIter2, _IIter2,
                                    _OIter, _Predicate, _IterTag1, _IterTag2,
                                    _IterTag3);

  template<typename _RAIter1, typename _RAIter2, typename _Output_RAIter,
           typename _Predicate>
    _Output_RAIter
    __set_symmetric_difference_switch(_RAIter1, _RAIter1, _RAIter2, _RAIter2,
                                    _Output_RAIter, _Predicate,
                                    random_access_iterator_tag,
                                    random_access_iterator_tag,
                                    random_access_iterator_tag);


  template<typename _IIter1, typename _IIter2, typename _OIter>
    _OIter
    set_difference(_IIter1, _IIter1, _IIter2, _IIter2, _OIter,
                   __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _Predicate>
    _OIter
    set_difference(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Predicate,
                   __gnu_parallel::sequential_tag);

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _OIter
    set_difference(_IIter1, _IIter1, _IIter2, _IIter2, _OIter);

  template<typename _IIter1, typename _IIter2, typename _OIter,
           typename _Predicate>
    _OIter
    set_difference(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Predicate);

  template<typename _IIter1, typename _IIter2, typename _Predicate,
           typename _OIter, typename _IterTag1, typename _IterTag2,
           typename _IterTag3>
    _OIter
    __set_difference_switch(_IIter1, _IIter1, _IIter2, _IIter2, _OIter,
                          _Predicate, _IterTag1, _IterTag2, _IterTag3);

  template<typename _RAIter1, typename _RAIter2, typename _Output_RAIter,
           typename _Predicate>
    _Output_RAIter
    __set_difference_switch(_RAIter1, _RAIter1, _RAIter2, _RAIter2,
                          _Output_RAIter, _Predicate,
                          random_access_iterator_tag,
                          random_access_iterator_tag,
                          random_access_iterator_tag);


  template<typename _RAIter>
    void
    sort(_RAIter, _RAIter, __gnu_parallel::sequential_tag);

  template<typename _RAIter, typename _Compare>
    void
    sort(_RAIter, _RAIter, _Compare, __gnu_parallel::sequential_tag);

  template<typename _RAIter>
    void
    sort(_RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    void
    sort(_RAIter, _RAIter, _Compare);

  template<typename _RAIter>
    void
    stable_sort(_RAIter, _RAIter, __gnu_parallel::sequential_tag);

  template<typename _RAIter, typename _Compare>
    void
    stable_sort(_RAIter, _RAIter, _Compare, __gnu_parallel::sequential_tag);

  template<typename _RAIter>
    void
    stable_sort(_RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    void
    stable_sort(_RAIter, _RAIter, _Compare);

  template<typename _IIter, typename _OIter>
    _OIter
    unique_copy(_IIter, _IIter, _OIter, __gnu_parallel::sequential_tag);

  template<typename _IIter, typename _OIter, typename _Predicate>
    _OIter
    unique_copy(_IIter, _IIter, _OIter, _Predicate,
                __gnu_parallel::sequential_tag);

  template<typename _IIter, typename _OIter>
    _OIter
    unique_copy(_IIter, _IIter, _OIter);

  template<typename _IIter, typename _OIter, typename _Predicate>
    _OIter
    unique_copy(_IIter, _IIter, _OIter, _Predicate);

  template<typename _IIter, typename _OIter, typename _Predicate,
           typename _IterTag1, typename _IterTag2>
    _OIter
    __unique_copy_switch(_IIter, _IIter, _OIter, _Predicate,
                       _IterTag1, _IterTag2);

  template<typename _RAIter, typename _RandomAccess_OIter, typename _Predicate>
    _RandomAccess_OIter
    __unique_copy_switch(_RAIter, _RAIter, _RandomAccess_OIter, _Predicate,
                       random_access_iterator_tag, random_access_iterator_tag);
} // end namespace __parallel
} // end namespace std

#endif /* _GLIBCXX_PARALLEL_ALGORITHMFWD_H */
