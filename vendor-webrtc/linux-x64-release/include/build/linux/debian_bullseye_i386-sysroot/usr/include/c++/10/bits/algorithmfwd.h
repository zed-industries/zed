// <algorithm> Forward declarations  -*- C++ -*-

// Copyright (C) 2007-2020 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the
// Free Software Foundation; either version 3, or (at your option)
// any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/** @file bits/algorithmfwd.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{algorithm}
 */

#ifndef _GLIBCXX_ALGORITHMFWD_H
#define _GLIBCXX_ALGORITHMFWD_H 1

#pragma GCC system_header

#include <bits/c++config.h>
#include <bits/stl_pair.h>
#include <bits/stl_iterator_base_types.h>
#if __cplusplus >= 201103L
#include <initializer_list>
#endif

namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION

  /*
    adjacent_find
    all_of (C++11)
    any_of (C++11)
    binary_search
    clamp (C++17)
    copy
    copy_backward
    copy_if (C++11)
    copy_n (C++11)
    count
    count_if
    equal
    equal_range
    fill
    fill_n
    find
    find_end
    find_first_of
    find_if
    find_if_not (C++11)
    for_each
    generate
    generate_n
    includes
    inplace_merge
    is_heap (C++11)
    is_heap_until (C++11)
    is_partitioned (C++11)
    is_sorted (C++11)
    is_sorted_until (C++11)
    iter_swap
    lexicographical_compare
    lower_bound
    make_heap
    max
    max_element
    merge
    min
    min_element
    minmax (C++11)
    minmax_element (C++11)
    mismatch
    next_permutation
    none_of (C++11)
    nth_element
    partial_sort
    partial_sort_copy
    partition
    partition_copy (C++11)
    partition_point (C++11)
    pop_heap
    prev_permutation
    push_heap
    random_shuffle
    remove
    remove_copy
    remove_copy_if
    remove_if
    replace
    replace_copy
    replace_copy_if
    replace_if
    reverse
    reverse_copy
    rotate
    rotate_copy
    search
    search_n
    set_difference
    set_intersection
    set_symmetric_difference
    set_union
    shuffle (C++11)
    sort
    sort_heap
    stable_partition
    stable_sort
    swap
    swap_ranges
    transform
    unique
    unique_copy
    upper_bound
  */

  /**
   * @defgroup algorithms Algorithms
   *
   * Components for performing algorithmic operations. Includes
   * non-modifying sequence, modifying (mutating) sequence, sorting,
   * searching, merge, partition, heap, set, minima, maxima, and
   * permutation operations.
   */

  /**
   * @defgroup mutating_algorithms Mutating
   * @ingroup algorithms
   */

  /**
   * @defgroup non_mutating_algorithms Non-Mutating
   * @ingroup algorithms
   */

  /**
   * @defgroup sorting_algorithms Sorting
   * @ingroup algorithms
   */

  /**
   * @defgroup set_algorithms Set Operations
   * @ingroup sorting_algorithms
   *
   * These algorithms are common set operations performed on sequences
   * that are already sorted. The number of comparisons will be
   * linear.
   */

  /**
   * @defgroup binary_search_algorithms Binary Search
   * @ingroup sorting_algorithms
   *
   * These algorithms are variations of a classic binary search, and
   * all assume that the sequence being searched is already sorted.
   *
   * The number of comparisons will be logarithmic (and as few as
   * possible).  The number of steps through the sequence will be
   * logarithmic for random-access iterators (e.g., pointers), and
   * linear otherwise.
   *
   * The LWG has passed Defect Report 270, which notes: <em>The
   * proposed resolution reinterprets binary search. Instead of
   * thinking about searching for a value in a sorted range, we view
   * that as an important special case of a more general algorithm:
   * searching for the partition point in a partitioned range.  We
   * also add a guarantee that the old wording did not: we ensure that
   * the upper bound is no earlier than the lower bound, that the pair
   * returned by equal_range is a valid range, and that the first part
   * of that pair is the lower bound.</em>
   *
   * The actual effect of the first sentence is that a comparison
   * functor passed by the user doesn't necessarily need to induce a
   * strict weak ordering relation.  Rather, it partitions the range.
   */

  // adjacent_find

#if __cplusplus > 201703L
#  define __cpp_lib_constexpr_algorithms 201806L
#endif

#if __cplusplus >= 201103L
  template<typename _IIter, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    bool
    all_of(_IIter, _IIter, _Predicate);

  template<typename _IIter, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    bool
    any_of(_IIter, _IIter, _Predicate);
#endif

  template<typename _FIter, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    bool
    binary_search(_FIter, _FIter, const _Tp&);

  template<typename _FIter, typename _Tp, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    bool
    binary_search(_FIter, _FIter, const _Tp&, _Compare);

#if __cplusplus > 201402L
  template<typename _Tp>
    _GLIBCXX14_CONSTEXPR
    const _Tp&
    clamp(const _Tp&, const _Tp&, const _Tp&);

  template<typename _Tp, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    const _Tp&
    clamp(const _Tp&, const _Tp&, const _Tp&, _Compare);
#endif

  template<typename _IIter, typename _OIter>
    _GLIBCXX20_CONSTEXPR
    _OIter
    copy(_IIter, _IIter, _OIter);

  template<typename _BIter1, typename _BIter2>
    _GLIBCXX20_CONSTEXPR
    _BIter2
    copy_backward(_BIter1, _BIter1, _BIter2);

#if __cplusplus >= 201103L
  template<typename _IIter, typename _OIter, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _OIter
    copy_if(_IIter, _IIter, _OIter, _Predicate);

  template<typename _IIter, typename _Size, typename _OIter>
    _GLIBCXX20_CONSTEXPR
    _OIter
    copy_n(_IIter, _Size, _OIter);
#endif

  // count
  // count_if

  template<typename _FIter, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    pair<_FIter, _FIter>
    equal_range(_FIter, _FIter, const _Tp&);

  template<typename _FIter, typename _Tp, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    pair<_FIter, _FIter>
    equal_range(_FIter, _FIter, const _Tp&, _Compare);

  template<typename _FIter, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    void
    fill(_FIter, _FIter, const _Tp&);

  template<typename _OIter, typename _Size, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    _OIter
    fill_n(_OIter, _Size, const _Tp&);

  // find

  template<typename _FIter1, typename _FIter2>
    _GLIBCXX20_CONSTEXPR
    _FIter1
    find_end(_FIter1, _FIter1, _FIter2, _FIter2);

  template<typename _FIter1, typename _FIter2, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _FIter1
    find_end(_FIter1, _FIter1, _FIter2, _FIter2, _BinaryPredicate);

  // find_first_of
  // find_if

#if __cplusplus >= 201103L
  template<typename _IIter, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _IIter
    find_if_not(_IIter, _IIter, _Predicate);
#endif

  // for_each
  // generate
  // generate_n

  template<typename _IIter1, typename _IIter2>
    _GLIBCXX20_CONSTEXPR
    bool
    includes(_IIter1, _IIter1, _IIter2, _IIter2);

  template<typename _IIter1, typename _IIter2, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    bool
    includes(_IIter1, _IIter1, _IIter2, _IIter2, _Compare);

  template<typename _BIter>
    void
    inplace_merge(_BIter, _BIter, _BIter);

  template<typename _BIter, typename _Compare>
    void
    inplace_merge(_BIter, _BIter, _BIter, _Compare);

#if __cplusplus >= 201103L
  template<typename _RAIter>
    _GLIBCXX20_CONSTEXPR
    bool
    is_heap(_RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    bool
    is_heap(_RAIter, _RAIter, _Compare);

  template<typename _RAIter>
    _GLIBCXX20_CONSTEXPR
    _RAIter
    is_heap_until(_RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _RAIter
    is_heap_until(_RAIter, _RAIter, _Compare);

  template<typename _IIter, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    bool
    is_partitioned(_IIter, _IIter, _Predicate);

  template<typename _FIter1, typename _FIter2>
    _GLIBCXX20_CONSTEXPR
    bool
    is_permutation(_FIter1, _FIter1, _FIter2);

  template<typename _FIter1, typename _FIter2,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    bool
    is_permutation(_FIter1, _FIter1, _FIter2, _BinaryPredicate);

  template<typename _FIter>
    _GLIBCXX20_CONSTEXPR
    bool
    is_sorted(_FIter, _FIter);

  template<typename _FIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    bool
    is_sorted(_FIter, _FIter, _Compare);

  template<typename _FIter>
    _GLIBCXX20_CONSTEXPR
    _FIter
    is_sorted_until(_FIter, _FIter);

  template<typename _FIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _FIter
    is_sorted_until(_FIter, _FIter, _Compare);
#endif

  template<typename _FIter1, typename _FIter2>
    _GLIBCXX20_CONSTEXPR
    void
    iter_swap(_FIter1, _FIter2);

  template<typename _FIter, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    _FIter
    lower_bound(_FIter, _FIter, const _Tp&);

  template<typename _FIter, typename _Tp, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _FIter
    lower_bound(_FIter, _FIter, const _Tp&, _Compare);

  template<typename _RAIter>
    _GLIBCXX20_CONSTEXPR
    void
    make_heap(_RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    make_heap(_RAIter, _RAIter, _Compare);

  template<typename _Tp>
    _GLIBCXX14_CONSTEXPR
    const _Tp&
    max(const _Tp&, const _Tp&);

  template<typename _Tp, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    const _Tp&
    max(const _Tp&, const _Tp&, _Compare);

  // max_element
  // merge

  template<typename _Tp>
    _GLIBCXX14_CONSTEXPR
    const _Tp&
    min(const _Tp&, const _Tp&);

  template<typename _Tp, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    const _Tp&
    min(const _Tp&, const _Tp&, _Compare);

  // min_element

#if __cplusplus >= 201103L
  template<typename _Tp>
    _GLIBCXX14_CONSTEXPR
    pair<const _Tp&, const _Tp&>
    minmax(const _Tp&, const _Tp&);

  template<typename _Tp, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    pair<const _Tp&, const _Tp&>
    minmax(const _Tp&, const _Tp&, _Compare);

  template<typename _FIter>
    _GLIBCXX14_CONSTEXPR
    pair<_FIter, _FIter>
    minmax_element(_FIter, _FIter);

  template<typename _FIter, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    pair<_FIter, _FIter>
    minmax_element(_FIter, _FIter, _Compare);

  template<typename _Tp>
    _GLIBCXX14_CONSTEXPR
    _Tp
    min(initializer_list<_Tp>);

  template<typename _Tp, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    _Tp
    min(initializer_list<_Tp>, _Compare);

  template<typename _Tp>
    _GLIBCXX14_CONSTEXPR
    _Tp
    max(initializer_list<_Tp>);

  template<typename _Tp, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    _Tp
    max(initializer_list<_Tp>, _Compare);

  template<typename _Tp>
    _GLIBCXX14_CONSTEXPR
    pair<_Tp, _Tp>
    minmax(initializer_list<_Tp>);

  template<typename _Tp, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    pair<_Tp, _Tp>
    minmax(initializer_list<_Tp>, _Compare);
#endif

  // mismatch

  template<typename _BIter>
    _GLIBCXX20_CONSTEXPR
    bool
    next_permutation(_BIter, _BIter);

  template<typename _BIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    bool
    next_permutation(_BIter, _BIter, _Compare);

#if __cplusplus >= 201103L
  template<typename _IIter, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    bool
    none_of(_IIter, _IIter, _Predicate);
#endif

  // nth_element
  // partial_sort

  template<typename _IIter, typename _RAIter>
    _GLIBCXX20_CONSTEXPR
    _RAIter
    partial_sort_copy(_IIter, _IIter, _RAIter, _RAIter);

  template<typename _IIter, typename _RAIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _RAIter
    partial_sort_copy(_IIter, _IIter, _RAIter, _RAIter, _Compare);

  // partition

#if __cplusplus >= 201103L
  template<typename _IIter, typename _OIter1,
	   typename _OIter2, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    pair<_OIter1, _OIter2>
    partition_copy(_IIter, _IIter, _OIter1, _OIter2, _Predicate);

  template<typename _FIter, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _FIter
    partition_point(_FIter, _FIter, _Predicate);
#endif

  template<typename _RAIter>
    _GLIBCXX20_CONSTEXPR
    void
    pop_heap(_RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    pop_heap(_RAIter, _RAIter, _Compare);

  template<typename _BIter>
    _GLIBCXX20_CONSTEXPR
    bool
    prev_permutation(_BIter, _BIter);

  template<typename _BIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    bool
    prev_permutation(_BIter, _BIter, _Compare);

  template<typename _RAIter>
    _GLIBCXX20_CONSTEXPR
    void
    push_heap(_RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    push_heap(_RAIter, _RAIter, _Compare);

  // random_shuffle

  template<typename _FIter, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    _FIter
    remove(_FIter, _FIter, const _Tp&);

  template<typename _FIter, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _FIter
    remove_if(_FIter, _FIter, _Predicate);

  template<typename _IIter, typename _OIter, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    _OIter
    remove_copy(_IIter, _IIter, _OIter, const _Tp&);

  template<typename _IIter, typename _OIter, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _OIter
    remove_copy_if(_IIter, _IIter, _OIter, _Predicate);

  // replace

  template<typename _IIter, typename _OIter, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    _OIter
    replace_copy(_IIter, _IIter, _OIter, const _Tp&, const _Tp&);

  template<typename _Iter, typename _OIter, typename _Predicate, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    _OIter
    replace_copy_if(_Iter, _Iter, _OIter, _Predicate, const _Tp&);

  // replace_if

  template<typename _BIter>
    _GLIBCXX20_CONSTEXPR
    void
    reverse(_BIter, _BIter);

  template<typename _BIter, typename _OIter>
    _GLIBCXX20_CONSTEXPR
    _OIter
    reverse_copy(_BIter, _BIter, _OIter);

  inline namespace _V2
  {
    template<typename _FIter>
      _GLIBCXX20_CONSTEXPR
      _FIter
      rotate(_FIter, _FIter, _FIter);
  }

  template<typename _FIter, typename _OIter>
    _GLIBCXX20_CONSTEXPR
    _OIter
    rotate_copy(_FIter, _FIter, _FIter, _OIter);

  // search
  // search_n
  // set_difference
  // set_intersection
  // set_symmetric_difference
  // set_union

#if (__cplusplus >= 201103L) && defined(_GLIBCXX_USE_C99_STDINT_TR1)
  template<typename _RAIter, typename _UGenerator>
    void
    shuffle(_RAIter, _RAIter, _UGenerator&&);
#endif

  template<typename _RAIter>
    _GLIBCXX20_CONSTEXPR
    void
    sort_heap(_RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    sort_heap(_RAIter, _RAIter, _Compare);

  template<typename _BIter, typename _Predicate>
    _BIter
    stable_partition(_BIter, _BIter, _Predicate);

#if __cplusplus < 201103L
  // For C++11 swap() is declared in <type_traits>.

  template<typename _Tp, size_t _Nm>
    _GLIBCXX20_CONSTEXPR
    inline void
    swap(_Tp& __a, _Tp& __b);

  template<typename _Tp, size_t _Nm>
    _GLIBCXX20_CONSTEXPR
    inline void
    swap(_Tp (&__a)[_Nm], _Tp (&__b)[_Nm]);
#endif

  template<typename _FIter1, typename _FIter2>
    _GLIBCXX20_CONSTEXPR
    _FIter2
    swap_ranges(_FIter1, _FIter1, _FIter2);

  // transform

  template<typename _FIter>
    _GLIBCXX20_CONSTEXPR
    _FIter
    unique(_FIter, _FIter);

  template<typename _FIter, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _FIter
    unique(_FIter, _FIter, _BinaryPredicate);

  // unique_copy

  template<typename _FIter, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    _FIter
    upper_bound(_FIter, _FIter, const _Tp&);

  template<typename _FIter, typename _Tp, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _FIter
    upper_bound(_FIter, _FIter, const _Tp&, _Compare);

_GLIBCXX_BEGIN_NAMESPACE_ALGO

  template<typename _FIter>
    _GLIBCXX20_CONSTEXPR
    _FIter
    adjacent_find(_FIter, _FIter);

  template<typename _FIter, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _FIter
    adjacent_find(_FIter, _FIter, _BinaryPredicate);

  template<typename _IIter, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    typename iterator_traits<_IIter>::difference_type
    count(_IIter, _IIter, const _Tp&);

  template<typename _IIter, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    typename iterator_traits<_IIter>::difference_type
    count_if(_IIter, _IIter, _Predicate);

  template<typename _IIter1, typename _IIter2>
    _GLIBCXX20_CONSTEXPR
    bool
    equal(_IIter1, _IIter1, _IIter2);

  template<typename _IIter1, typename _IIter2, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    bool
    equal(_IIter1, _IIter1, _IIter2, _BinaryPredicate);

  template<typename _IIter, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    _IIter
    find(_IIter, _IIter, const _Tp&);

  template<typename _FIter1, typename _FIter2>
    _GLIBCXX20_CONSTEXPR
    _FIter1
    find_first_of(_FIter1, _FIter1, _FIter2, _FIter2);

  template<typename _FIter1, typename _FIter2, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _FIter1
    find_first_of(_FIter1, _FIter1, _FIter2, _FIter2, _BinaryPredicate);

  template<typename _IIter, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _IIter
    find_if(_IIter, _IIter, _Predicate);

  template<typename _IIter, typename _Funct>
    _GLIBCXX20_CONSTEXPR
    _Funct
    for_each(_IIter, _IIter, _Funct);

  template<typename _FIter, typename _Generator>
    _GLIBCXX20_CONSTEXPR
    void
    generate(_FIter, _FIter, _Generator);

  template<typename _OIter, typename _Size, typename _Generator>
    _GLIBCXX20_CONSTEXPR
    _OIter
    generate_n(_OIter, _Size, _Generator);

  template<typename _IIter1, typename _IIter2>
    _GLIBCXX20_CONSTEXPR
    bool
    lexicographical_compare(_IIter1, _IIter1, _IIter2, _IIter2);

  template<typename _IIter1, typename _IIter2, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    bool
    lexicographical_compare(_IIter1, _IIter1, _IIter2, _IIter2, _Compare);

  template<typename _FIter>
    _GLIBCXX14_CONSTEXPR
    _FIter
    max_element(_FIter, _FIter);

  template<typename _FIter, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    _FIter
    max_element(_FIter, _FIter, _Compare);

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _GLIBCXX20_CONSTEXPR
    _OIter
    merge(_IIter1, _IIter1, _IIter2, _IIter2, _OIter);

  template<typename _IIter1, typename _IIter2, typename _OIter,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _OIter
    merge(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Compare);

  template<typename _FIter>
    _GLIBCXX14_CONSTEXPR
    _FIter
    min_element(_FIter, _FIter);

  template<typename _FIter, typename _Compare>
    _GLIBCXX14_CONSTEXPR
    _FIter
    min_element(_FIter, _FIter, _Compare);

  template<typename _IIter1, typename _IIter2>
    _GLIBCXX20_CONSTEXPR
    pair<_IIter1, _IIter2>
    mismatch(_IIter1, _IIter1, _IIter2);

  template<typename _IIter1, typename _IIter2, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    pair<_IIter1, _IIter2>
    mismatch(_IIter1, _IIter1, _IIter2, _BinaryPredicate);

  template<typename _RAIter>
    _GLIBCXX20_CONSTEXPR
    void
    nth_element(_RAIter, _RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    nth_element(_RAIter, _RAIter, _RAIter, _Compare);

  template<typename _RAIter>
    _GLIBCXX20_CONSTEXPR
    void
    partial_sort(_RAIter, _RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    partial_sort(_RAIter, _RAIter, _RAIter, _Compare);

  template<typename _BIter, typename _Predicate>
    _GLIBCXX20_CONSTEXPR
    _BIter
    partition(_BIter, _BIter, _Predicate);

  template<typename _RAIter>
    void
    random_shuffle(_RAIter, _RAIter);

  template<typename _RAIter, typename _Generator>
    void
    random_shuffle(_RAIter, _RAIter,
#if __cplusplus >= 201103L
		   _Generator&&);
#else
		   _Generator&);
#endif

  template<typename _FIter, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    void
    replace(_FIter, _FIter, const _Tp&, const _Tp&);

  template<typename _FIter, typename _Predicate, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    void
    replace_if(_FIter, _FIter, _Predicate, const _Tp&);

  template<typename _FIter1, typename _FIter2>
    _GLIBCXX20_CONSTEXPR
    _FIter1
    search(_FIter1, _FIter1, _FIter2, _FIter2);

  template<typename _FIter1, typename _FIter2, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _FIter1
    search(_FIter1, _FIter1, _FIter2, _FIter2, _BinaryPredicate);

  template<typename _FIter, typename _Size, typename _Tp>
    _GLIBCXX20_CONSTEXPR
    _FIter
    search_n(_FIter, _FIter, _Size, const _Tp&);

  template<typename _FIter, typename _Size, typename _Tp,
	   typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _FIter
    search_n(_FIter, _FIter, _Size, const _Tp&, _BinaryPredicate);

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _GLIBCXX20_CONSTEXPR
    _OIter
    set_difference(_IIter1, _IIter1, _IIter2, _IIter2, _OIter);

  template<typename _IIter1, typename _IIter2, typename _OIter,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _OIter
    set_difference(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Compare);

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _GLIBCXX20_CONSTEXPR
    _OIter
    set_intersection(_IIter1, _IIter1, _IIter2, _IIter2, _OIter);

  template<typename _IIter1, typename _IIter2, typename _OIter,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _OIter
    set_intersection(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Compare);

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _GLIBCXX20_CONSTEXPR
    _OIter
    set_symmetric_difference(_IIter1, _IIter1, _IIter2, _IIter2, _OIter);

  template<typename _IIter1, typename _IIter2, typename _OIter,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _OIter
    set_symmetric_difference(_IIter1, _IIter1, _IIter2, _IIter2,
			     _OIter, _Compare);

  template<typename _IIter1, typename _IIter2, typename _OIter>
    _GLIBCXX20_CONSTEXPR
    _OIter
    set_union(_IIter1, _IIter1, _IIter2, _IIter2, _OIter);

  template<typename _IIter1, typename _IIter2, typename _OIter,
	   typename _Compare>
    _GLIBCXX20_CONSTEXPR
    _OIter
    set_union(_IIter1, _IIter1, _IIter2, _IIter2, _OIter, _Compare);

  template<typename _RAIter>
    _GLIBCXX20_CONSTEXPR
    void
    sort(_RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    _GLIBCXX20_CONSTEXPR
    void
    sort(_RAIter, _RAIter, _Compare);

  template<typename _RAIter>
    void
    stable_sort(_RAIter, _RAIter);

  template<typename _RAIter, typename _Compare>
    void
    stable_sort(_RAIter, _RAIter, _Compare);

  template<typename _IIter, typename _OIter, typename _UnaryOperation>
    _GLIBCXX20_CONSTEXPR
    _OIter
    transform(_IIter, _IIter, _OIter, _UnaryOperation);

  template<typename _IIter1, typename _IIter2, typename _OIter,
	   typename _BinaryOperation>
    _GLIBCXX20_CONSTEXPR
    _OIter
    transform(_IIter1, _IIter1, _IIter2, _OIter, _BinaryOperation);

  template<typename _IIter, typename _OIter>
    _GLIBCXX20_CONSTEXPR
    _OIter
    unique_copy(_IIter, _IIter, _OIter);

  template<typename _IIter, typename _OIter, typename _BinaryPredicate>
    _GLIBCXX20_CONSTEXPR
    _OIter
    unique_copy(_IIter, _IIter, _OIter, _BinaryPredicate);

_GLIBCXX_END_NAMESPACE_ALGO
_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std

#ifdef _GLIBCXX_PARALLEL
# include <parallel/algorithmfwd.h>
#endif

#endif

