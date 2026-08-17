// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___FLAT_SET_FLAT_SET_H
#define _LIBCPP___FLAT_SET_FLAT_SET_H

#include <__algorithm/lexicographical_compare_three_way.h>
#include <__algorithm/lower_bound.h>
#include <__algorithm/min.h>
#include <__algorithm/ranges_adjacent_find.h>
#include <__algorithm/ranges_equal.h>
#include <__algorithm/ranges_inplace_merge.h>
#include <__algorithm/ranges_partition_point.h>
#include <__algorithm/ranges_sort.h>
#include <__algorithm/ranges_unique.h>
#include <__algorithm/remove_if.h>
#include <__algorithm/upper_bound.h>
#include <__assert>
#include <__compare/synth_three_way.h>
#include <__concepts/swappable.h>
#include <__config>
#include <__cstddef/ptrdiff_t.h>
#include <__flat_map/sorted_unique.h>
#include <__flat_set/ra_iterator.h>
#include <__flat_set/utils.h>
#include <__functional/invoke.h>
#include <__functional/is_transparent.h>
#include <__functional/operations.h>
#include <__fwd/vector.h>
#include <__iterator/concepts.h>
#include <__iterator/distance.h>
#include <__iterator/iterator_traits.h>
#include <__iterator/next.h>
#include <__iterator/prev.h>
#include <__iterator/ranges_iterator_traits.h>
#include <__iterator/reverse_iterator.h>
#include <__memory/allocator_traits.h>
#include <__memory/uses_allocator.h>
#include <__memory/uses_allocator_construction.h>
#include <__ranges/access.h>
#include <__ranges/concepts.h>
#include <__ranges/container_compatible_range.h>
#include <__ranges/drop_view.h>
#include <__ranges/from_range.h>
#include <__ranges/ref_view.h>
#include <__ranges/size.h>
#include <__ranges/subrange.h>
#include <__type_traits/conjunction.h>
#include <__type_traits/container_traits.h>
#include <__type_traits/invoke.h>
#include <__type_traits/is_allocator.h>
#include <__type_traits/is_const.h>
#include <__type_traits/is_nothrow_constructible.h>
#include <__type_traits/is_same.h>
#include <__type_traits/remove_reference.h>
#include <__utility/as_const.h>
#include <__utility/exception_guard.h>
#include <__utility/move.h>
#include <__utility/pair.h>
#include <__utility/scope_guard.h>
#include <__vector/vector.h>
#include <initializer_list>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

#if _LIBCPP_STD_VER >= 23

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Key, class _Compare = less<_Key>, class _KeyContainer = vector<_Key>>
class flat_set {
  template <class, class, class>
  friend class flat_set;

  friend __flat_set_utils;

  static_assert(is_same_v<_Key, typename _KeyContainer::value_type>);
  static_assert(!is_same_v<_KeyContainer, std::vector<bool>>, "vector<bool> is not a sequence container");

  using __key_iterator _LIBCPP_NODEBUG = typename _KeyContainer::const_iterator;

public:
  // types
  using key_type               = _Key;
  using value_type             = _Key;
  using key_compare            = __type_identity_t<_Compare>;
  using value_compare          = _Compare;
  using reference              = value_type&;
  using const_reference        = const value_type&;
  using size_type              = typename _KeyContainer::size_type;
  using difference_type        = typename _KeyContainer::difference_type;
  using iterator               = __ra_iterator<flat_set, typename _KeyContainer::const_iterator>;
  using const_iterator         = iterator;
  using reverse_iterator       = std::reverse_iterator<iterator>;
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;
  using container_type         = _KeyContainer;

public:
  // [flat.set.cons], construct/copy/destroy
  _LIBCPP_HIDE_FROM_ABI
  flat_set() noexcept(is_nothrow_default_constructible_v<_KeyContainer> && is_nothrow_default_constructible_v<_Compare>)
      : __keys_(), __compare_() {}

  _LIBCPP_HIDE_FROM_ABI flat_set(const flat_set&) = default;

  _LIBCPP_HIDE_FROM_ABI flat_set(flat_set&& __other) noexcept(
      is_nothrow_move_constructible_v<_KeyContainer> && is_nothrow_move_constructible_v<_Compare>)
#  if _LIBCPP_HAS_EXCEPTIONS
      try
#  endif // _LIBCPP_HAS_EXCEPTIONS
      : __keys_(std::move(__other.__keys_)), __compare_(std::move(__other.__compare_)) {
    __other.clear();
#  if _LIBCPP_HAS_EXCEPTIONS
  } catch (...) {
    __other.clear();
    // gcc does not like the `throw` keyword in a conditionally noexcept function
    if constexpr (!(is_nothrow_move_constructible_v<_KeyContainer> && is_nothrow_move_constructible_v<_Compare>)) {
      throw;
    }
#  endif // _LIBCPP_HAS_EXCEPTIONS
  }

  _LIBCPP_HIDE_FROM_ABI explicit flat_set(const key_compare& __comp) : __keys_(), __compare_(__comp) {}

  _LIBCPP_HIDE_FROM_ABI explicit flat_set(container_type __keys, const key_compare& __comp = key_compare())
      : __keys_(std::move(__keys)), __compare_(__comp) {
    __sort_and_unique();
  }

  _LIBCPP_HIDE_FROM_ABI flat_set(sorted_unique_t, container_type __keys, const key_compare& __comp = key_compare())
      : __keys_(std::move(__keys)), __compare_(__comp) {
    _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(
        __is_sorted_and_unique(__keys_), "Either the key container is not sorted or it contains duplicates");
  }

  template <class _InputIterator>
    requires __has_input_iterator_category<_InputIterator>::value
  _LIBCPP_HIDE_FROM_ABI
  flat_set(_InputIterator __first, _InputIterator __last, const key_compare& __comp = key_compare())
      : __keys_(), __compare_(__comp) {
    insert(__first, __last);
  }

  template <class _InputIterator>
    requires __has_input_iterator_category<_InputIterator>::value
  _LIBCPP_HIDE_FROM_ABI
  flat_set(sorted_unique_t, _InputIterator __first, _InputIterator __last, const key_compare& __comp = key_compare())
      : __keys_(__first, __last), __compare_(__comp) {
    _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(
        __is_sorted_and_unique(__keys_), "Either the key container is not sorted or it contains duplicates");
  }

  template <_ContainerCompatibleRange<value_type> _Range>
  _LIBCPP_HIDE_FROM_ABI flat_set(from_range_t, _Range&& __rg)
      : flat_set(from_range, std::forward<_Range>(__rg), key_compare()) {}

  template <_ContainerCompatibleRange<value_type> _Range>
  _LIBCPP_HIDE_FROM_ABI flat_set(from_range_t, _Range&& __rg, const key_compare& __comp) : flat_set(__comp) {
    insert_range(std::forward<_Range>(__rg));
  }

  _LIBCPP_HIDE_FROM_ABI flat_set(initializer_list<value_type> __il, const key_compare& __comp = key_compare())
      : flat_set(__il.begin(), __il.end(), __comp) {}

  _LIBCPP_HIDE_FROM_ABI
  flat_set(sorted_unique_t, initializer_list<value_type> __il, const key_compare& __comp = key_compare())
      : flat_set(sorted_unique, __il.begin(), __il.end(), __comp) {}

  template <class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI explicit flat_set(const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc)), __compare_() {}

  template <class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI flat_set(const key_compare& __comp, const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc)), __compare_(__comp) {}

  template <class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI flat_set(const container_type& __keys, const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc, __keys)), __compare_() {
    __sort_and_unique();
  }

  template <class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI flat_set(const container_type& __keys, const key_compare& __comp, const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc, __keys)), __compare_(__comp) {
    __sort_and_unique();
  }

  template <class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI flat_set(sorted_unique_t, const container_type& __keys, const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc, __keys)), __compare_() {
    _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(
        __is_sorted_and_unique(__keys_), "Either the key container is not sorted or it contains duplicates");
  }

  template <class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI
  flat_set(sorted_unique_t, const container_type& __keys, const key_compare& __comp, const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc, __keys)), __compare_(__comp) {
    _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(
        __is_sorted_and_unique(__keys_), "Either the key container is not sorted or it contains duplicates");
  }

  template <class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI flat_set(const flat_set& __other, const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc, __other.__keys_)),
        __compare_(__other.__compare_) {}

  template <class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI flat_set(flat_set&& __other, const _Allocator& __alloc)
#  if _LIBCPP_HAS_EXCEPTIONS
      try
#  endif // _LIBCPP_HAS_EXCEPTIONS
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc, std::move(__other.__keys_))),
        __compare_(std::move(__other.__compare_)) {
    __other.clear();
#  if _LIBCPP_HAS_EXCEPTIONS
  } catch (...) {
    __other.clear();
    throw;
#  endif // _LIBCPP_HAS_EXCEPTIONS
  }

  template <class _InputIterator, class _Allocator>
    requires(__has_input_iterator_category<_InputIterator>::value && uses_allocator<container_type, _Allocator>::value)
  _LIBCPP_HIDE_FROM_ABI flat_set(_InputIterator __first, _InputIterator __last, const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc)), __compare_() {
    insert(__first, __last);
  }

  template <class _InputIterator, class _Allocator>
    requires(__has_input_iterator_category<_InputIterator>::value && uses_allocator<container_type, _Allocator>::value)
  _LIBCPP_HIDE_FROM_ABI
  flat_set(_InputIterator __first, _InputIterator __last, const key_compare& __comp, const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc)), __compare_(__comp) {
    insert(__first, __last);
  }

  template <class _InputIterator, class _Allocator>
    requires(__has_input_iterator_category<_InputIterator>::value && uses_allocator<container_type, _Allocator>::value)
  _LIBCPP_HIDE_FROM_ABI
  flat_set(sorted_unique_t, _InputIterator __first, _InputIterator __last, const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc, __first, __last)), __compare_() {
    _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(
        __is_sorted_and_unique(__keys_), "Either the key container is not sorted or it contains duplicates");
  }

  template <class _InputIterator, class _Allocator>
    requires(__has_input_iterator_category<_InputIterator>::value && uses_allocator<container_type, _Allocator>::value)
  _LIBCPP_HIDE_FROM_ABI
  flat_set(sorted_unique_t,
           _InputIterator __first,
           _InputIterator __last,
           const key_compare& __comp,
           const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc, __first, __last)), __compare_(__comp) {
    _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(
        __is_sorted_and_unique(__keys_), "Either the key container is not sorted or it contains duplicates");
  }

  template <_ContainerCompatibleRange<value_type> _Range, class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI flat_set(from_range_t, _Range&& __rg, const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc)), __compare_() {
    insert_range(std::forward<_Range>(__rg));
  }

  template <_ContainerCompatibleRange<value_type> _Range, class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI flat_set(from_range_t, _Range&& __rg, const key_compare& __comp, const _Allocator& __alloc)
      : __keys_(std::make_obj_using_allocator<container_type>(__alloc)), __compare_(__comp) {
    insert_range(std::forward<_Range>(__rg));
  }

  template <class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI flat_set(initializer_list<value_type> __il, const _Allocator& __alloc)
      : flat_set(__il.begin(), __il.end(), __alloc) {}

  template <class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI
  flat_set(initializer_list<value_type> __il, const key_compare& __comp, const _Allocator& __alloc)
      : flat_set(__il.begin(), __il.end(), __comp, __alloc) {}

  template <class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI flat_set(sorted_unique_t, initializer_list<value_type> __il, const _Allocator& __alloc)
      : flat_set(sorted_unique, __il.begin(), __il.end(), __alloc) {}

  template <class _Allocator>
    requires uses_allocator<container_type, _Allocator>::value
  _LIBCPP_HIDE_FROM_ABI
  flat_set(sorted_unique_t, initializer_list<value_type> __il, const key_compare& __comp, const _Allocator& __alloc)
      : flat_set(sorted_unique, __il.begin(), __il.end(), __comp, __alloc) {}

  _LIBCPP_HIDE_FROM_ABI flat_set& operator=(initializer_list<value_type> __il) {
    clear();
    insert(__il);
    return *this;
  }

  _LIBCPP_HIDE_FROM_ABI flat_set& operator=(const flat_set&) = default;

  _LIBCPP_HIDE_FROM_ABI flat_set& operator=(flat_set&& __other) noexcept(
      is_nothrow_move_assignable_v<_KeyContainer> && is_nothrow_move_assignable_v<_Compare>) {
    // No matter what happens, we always want to clear the other container before returning
    // since we moved from it
    auto __clear_other_guard = std::__make_scope_guard([&]() noexcept { __other.clear() /* noexcept */; });
    {
      // If an exception is thrown, we have no choice but to clear *this to preserve invariants
      auto __on_exception = std::__make_exception_guard([&]() noexcept { clear() /* noexcept */; });
      __keys_             = std::move(__other.__keys_);
      __compare_          = std::move(__other.__compare_);
      __on_exception.__complete();
    }
    return *this;
  }

  // iterators
  _LIBCPP_HIDE_FROM_ABI iterator begin() noexcept { return iterator(std::as_const(__keys_).begin()); }
  _LIBCPP_HIDE_FROM_ABI const_iterator begin() const noexcept { return const_iterator(__keys_.begin()); }
  _LIBCPP_HIDE_FROM_ABI iterator end() noexcept { return iterator(std::as_const(__keys_).end()); }
  _LIBCPP_HIDE_FROM_ABI const_iterator end() const noexcept { return const_iterator(__keys_.end()); }

  _LIBCPP_HIDE_FROM_ABI reverse_iterator rbegin() noexcept { return reverse_iterator(end()); }
  _LIBCPP_HIDE_FROM_ABI const_reverse_iterator rbegin() const noexcept { return const_reverse_iterator(end()); }
  _LIBCPP_HIDE_FROM_ABI reverse_iterator rend() noexcept { return reverse_iterator(begin()); }
  _LIBCPP_HIDE_FROM_ABI const_reverse_iterator rend() const noexcept { return const_reverse_iterator(begin()); }

  _LIBCPP_HIDE_FROM_ABI const_iterator cbegin() const noexcept { return begin(); }
  _LIBCPP_HIDE_FROM_ABI const_iterator cend() const noexcept { return end(); }
  _LIBCPP_HIDE_FROM_ABI const_reverse_iterator crbegin() const noexcept { return const_reverse_iterator(end()); }
  _LIBCPP_HIDE_FROM_ABI const_reverse_iterator crend() const noexcept { return const_reverse_iterator(begin()); }

  // [flat.set.capacity], capacity
  [[nodiscard]] _LIBCPP_HIDE_FROM_ABI bool empty() const noexcept { return __keys_.empty(); }

  _LIBCPP_HIDE_FROM_ABI size_type size() const noexcept { return __keys_.size(); }

  _LIBCPP_HIDE_FROM_ABI size_type max_size() const noexcept { return __keys_.max_size(); }

  // [flat.set.modifiers], modifiers
  template <class... _Args>
  _LIBCPP_HIDE_FROM_ABI pair<iterator, bool> emplace(_Args&&... __args) {
    if constexpr (sizeof...(__args) == 1 && (is_same_v<remove_cvref_t<_Args>, _Key> && ...)) {
      return __emplace(std::forward<_Args>(__args)...);
    } else {
      return __emplace(_Key(std::forward<_Args>(__args)...));
    }
  }

  template <class... _Args>
  _LIBCPP_HIDE_FROM_ABI iterator emplace_hint(const_iterator __hint, _Args&&... __args) {
    if constexpr (sizeof...(__args) == 1 && (is_same_v<remove_cvref_t<_Args>, _Key> && ...)) {
      return __emplace_hint(std::move(__hint), std::forward<_Args>(__args)...);
    } else {
      return __emplace_hint(std::move(__hint), _Key(std::forward<_Args>(__args)...));
    }
  }

  _LIBCPP_HIDE_FROM_ABI pair<iterator, bool> insert(const value_type& __x) { return emplace(__x); }

  _LIBCPP_HIDE_FROM_ABI pair<iterator, bool> insert(value_type&& __x) { return emplace(std::move(__x)); }

  template <class _Kp>
    requires(__is_transparent_v<_Compare> && is_constructible_v<value_type, _Kp>)
  _LIBCPP_HIDE_FROM_ABI pair<iterator, bool> insert(_Kp&& __x) {
    return __emplace(std::forward<_Kp>(__x));
  }
  _LIBCPP_HIDE_FROM_ABI iterator insert(const_iterator __hint, const value_type& __x) {
    return emplace_hint(__hint, __x);
  }

  _LIBCPP_HIDE_FROM_ABI iterator insert(const_iterator __hint, value_type&& __x) {
    return emplace_hint(__hint, std::move(__x));
  }

  template <class _Kp>
    requires(__is_transparent_v<_Compare> && is_constructible_v<value_type, _Kp>)
  _LIBCPP_HIDE_FROM_ABI iterator insert(const_iterator __hint, _Kp&& __x) {
    return __emplace_hint(__hint, std::forward<_Kp>(__x));
  }

  template <class _InputIterator>
    requires __has_input_iterator_category<_InputIterator>::value
  _LIBCPP_HIDE_FROM_ABI void insert(_InputIterator __first, _InputIterator __last) {
    if constexpr (sized_sentinel_for<_InputIterator, _InputIterator>) {
      __reserve(__last - __first);
    }
    __append_sort_merge_unique</*WasSorted = */ false>(std::move(__first), std::move(__last));
  }

  template <class _InputIterator>
    requires __has_input_iterator_category<_InputIterator>::value
  _LIBCPP_HIDE_FROM_ABI void insert(sorted_unique_t, _InputIterator __first, _InputIterator __last) {
    if constexpr (sized_sentinel_for<_InputIterator, _InputIterator>) {
      __reserve(__last - __first);
    }

    __append_sort_merge_unique</*WasSorted = */ true>(std::move(__first), std::move(__last));
  }

  template <_ContainerCompatibleRange<value_type> _Range>
  _LIBCPP_HIDE_FROM_ABI void insert_range(_Range&& __range) {
    if constexpr (ranges::sized_range<_Range>) {
      __reserve(ranges::size(__range));
    }

    __append_sort_merge_unique</*WasSorted = */ false>(std::forward<_Range>(__range));
  }

  _LIBCPP_HIDE_FROM_ABI void insert(initializer_list<value_type> __il) { insert(__il.begin(), __il.end()); }

  _LIBCPP_HIDE_FROM_ABI void insert(sorted_unique_t, initializer_list<value_type> __il) {
    insert(sorted_unique, __il.begin(), __il.end());
  }

  _LIBCPP_HIDE_FROM_ABI container_type extract() && {
    auto __guard = std::__make_scope_guard([&]() noexcept { clear() /* noexcept */; });
    auto __ret   = std::move(__keys_);
    return __ret;
  }

  _LIBCPP_HIDE_FROM_ABI void replace(container_type&& __keys) {
    _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(
        __is_sorted_and_unique(__keys), "Either the key container is not sorted or it contains duplicates");
    auto __guard = std::__make_exception_guard([&]() noexcept { clear() /* noexcept */; });
    __keys_      = std::move(__keys);
    __guard.__complete();
  }

  _LIBCPP_HIDE_FROM_ABI iterator erase(iterator __position) {
    auto __on_failure = std::__make_exception_guard([&]() noexcept { clear() /* noexcept */; });
    auto __key_iter   = __keys_.erase(__position.__base());
    __on_failure.__complete();
    return iterator(__key_iter);
  }

  // The following overload is the same as the iterator overload
  // iterator erase(const_iterator __position);

  _LIBCPP_HIDE_FROM_ABI size_type erase(const key_type& __x) {
    auto __iter = find(__x);
    if (__iter != end()) {
      erase(__iter);
      return 1;
    }
    return 0;
  }

  template <class _Kp>
    requires(__is_transparent_v<_Compare> && !is_convertible_v<_Kp &&, iterator> &&
             !is_convertible_v<_Kp &&, const_iterator>)
  _LIBCPP_HIDE_FROM_ABI size_type erase(_Kp&& __x) {
    auto [__first, __last] = equal_range(__x);
    auto __res             = __last - __first;
    erase(__first, __last);
    return __res;
  }

  _LIBCPP_HIDE_FROM_ABI iterator erase(const_iterator __first, const_iterator __last) {
    auto __on_failure = std::__make_exception_guard([&]() noexcept { clear() /* noexcept */; });
    auto __key_it     = __keys_.erase(__first.__base(), __last.__base());
    __on_failure.__complete();
    return iterator(std::move(__key_it));
  }

  _LIBCPP_HIDE_FROM_ABI void swap(flat_set& __y) noexcept {
    // warning: The spec has unconditional noexcept, which means that
    // if any of the following functions throw an exception,
    // std::terminate will be called.
    // This is discussed in P2767, which hasn't been voted on yet.
    ranges::swap(__compare_, __y.__compare_);
    ranges::swap(__keys_, __y.__keys_);
  }

  _LIBCPP_HIDE_FROM_ABI void clear() noexcept { __keys_.clear(); }

  // observers
  _LIBCPP_HIDE_FROM_ABI key_compare key_comp() const { return __compare_; }
  _LIBCPP_HIDE_FROM_ABI value_compare value_comp() const { return __compare_; }

  // set operations
  _LIBCPP_HIDE_FROM_ABI iterator find(const key_type& __x) { return __find_impl(*this, __x); }

  _LIBCPP_HIDE_FROM_ABI const_iterator find(const key_type& __x) const { return __find_impl(*this, __x); }

  template <class _Kp>
    requires __is_transparent_v<_Compare>
  _LIBCPP_HIDE_FROM_ABI iterator find(const _Kp& __x) {
    return __find_impl(*this, __x);
  }

  template <class _Kp>
    requires __is_transparent_v<_Compare>
  _LIBCPP_HIDE_FROM_ABI const_iterator find(const _Kp& __x) const {
    return __find_impl(*this, __x);
  }

  _LIBCPP_HIDE_FROM_ABI size_type count(const key_type& __x) const { return contains(__x) ? 1 : 0; }

  template <class _Kp>
    requires __is_transparent_v<_Compare>
  _LIBCPP_HIDE_FROM_ABI size_type count(const _Kp& __x) const {
    return contains(__x) ? 1 : 0;
  }

  _LIBCPP_HIDE_FROM_ABI bool contains(const key_type& __x) const { return find(__x) != end(); }

  template <class _Kp>
    requires __is_transparent_v<_Compare>
  _LIBCPP_HIDE_FROM_ABI bool contains(const _Kp& __x) const {
    return find(__x) != end();
  }

  _LIBCPP_HIDE_FROM_ABI iterator lower_bound(const key_type& __x) {
    const auto& __keys = __keys_;
    return iterator(std::lower_bound(__keys.begin(), __keys.end(), __x, __compare_));
  }

  _LIBCPP_HIDE_FROM_ABI const_iterator lower_bound(const key_type& __x) const {
    return const_iterator(std::lower_bound(__keys_.begin(), __keys_.end(), __x, __compare_));
  }

  template <class _Kp>
    requires __is_transparent_v<_Compare>
  _LIBCPP_HIDE_FROM_ABI iterator lower_bound(const _Kp& __x) {
    const auto& __keys = __keys_;
    return iterator(std::lower_bound(__keys.begin(), __keys.end(), __x, __compare_));
  }

  template <class _Kp>
    requires __is_transparent_v<_Compare>
  _LIBCPP_HIDE_FROM_ABI const_iterator lower_bound(const _Kp& __x) const {
    return const_iterator(std::lower_bound(__keys_.begin(), __keys_.end(), __x, __compare_));
  }

  _LIBCPP_HIDE_FROM_ABI iterator upper_bound(const key_type& __x) {
    const auto& __keys = __keys_;
    return iterator(std::upper_bound(__keys.begin(), __keys.end(), __x, __compare_));
  }

  _LIBCPP_HIDE_FROM_ABI const_iterator upper_bound(const key_type& __x) const {
    return const_iterator(std::upper_bound(__keys_.begin(), __keys_.end(), __x, __compare_));
  }

  template <class _Kp>
    requires __is_transparent_v<_Compare>
  _LIBCPP_HIDE_FROM_ABI iterator upper_bound(const _Kp& __x) {
    const auto& __keys = __keys_;
    return iterator(std::upper_bound(__keys.begin(), __keys.end(), __x, __compare_));
  }

  template <class _Kp>
    requires __is_transparent_v<_Compare>
  _LIBCPP_HIDE_FROM_ABI const_iterator upper_bound(const _Kp& __x) const {
    return const_iterator(std::upper_bound(__keys_.begin(), __keys_.end(), __x, __compare_));
  }

  _LIBCPP_HIDE_FROM_ABI pair<iterator, iterator> equal_range(const key_type& __x) {
    return __equal_range_impl(*this, __x);
  }

  _LIBCPP_HIDE_FROM_ABI pair<const_iterator, const_iterator> equal_range(const key_type& __x) const {
    return __equal_range_impl(*this, __x);
  }

  template <class _Kp>
    requires __is_transparent_v<_Compare>
  _LIBCPP_HIDE_FROM_ABI pair<iterator, iterator> equal_range(const _Kp& __x) {
    return __equal_range_impl(*this, __x);
  }
  template <class _Kp>
    requires __is_transparent_v<_Compare>
  _LIBCPP_HIDE_FROM_ABI pair<const_iterator, const_iterator> equal_range(const _Kp& __x) const {
    return __equal_range_impl(*this, __x);
  }

  friend _LIBCPP_HIDE_FROM_ABI bool operator==(const flat_set& __x, const flat_set& __y) {
    return ranges::equal(__x, __y);
  }

  friend _LIBCPP_HIDE_FROM_ABI auto operator<=>(const flat_set& __x, const flat_set& __y) {
    return std::lexicographical_compare_three_way(
        __x.begin(), __x.end(), __y.begin(), __y.end(), std::__synth_three_way);
  }

  friend _LIBCPP_HIDE_FROM_ABI void swap(flat_set& __x, flat_set& __y) noexcept { __x.swap(__y); }

private:
  _LIBCPP_HIDE_FROM_ABI bool __is_sorted_and_unique(auto&& __key_container) const {
    auto __greater_or_equal_to = [this](const auto& __x, const auto& __y) { return !__compare_(__x, __y); };
    return ranges::adjacent_find(__key_container, __greater_or_equal_to) == ranges::end(__key_container);
  }

  // This function is only used in constructors. So there is not exception handling in this function.
  // If the function exits via an exception, there will be no flat_set object constructed, thus, there
  // is no invariant state to preserve
  _LIBCPP_HIDE_FROM_ABI void __sort_and_unique() {
    ranges::sort(__keys_, __compare_);
    auto __dup_start = ranges::unique(__keys_, __key_equiv(__compare_)).begin();
    __keys_.erase(__dup_start, __keys_.end());
  }

  template <bool _WasSorted, class... _Args>
  _LIBCPP_HIDE_FROM_ABI void __append_sort_merge_unique(_Args&&... __args) {
    auto __on_failure    = std::__make_exception_guard([&]() noexcept { clear() /* noexcept */; });
    size_type __old_size = size();
    __flat_set_utils::__append(*this, std::forward<_Args>(__args)...);
    if (size() != __old_size) {
      if constexpr (!_WasSorted) {
        ranges::sort(__keys_.begin() + __old_size, __keys_.end(), __compare_);
      } else {
        _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(__is_sorted_and_unique(__keys_ | ranges::views::drop(__old_size)),
                                            "Either the key container is not sorted or it contains duplicates");
      }
      ranges::inplace_merge(__keys_.begin(), __keys_.begin() + __old_size, __keys_.end(), __compare_);

      auto __dup_start = ranges::unique(__keys_, __key_equiv(__compare_)).begin();
      __keys_.erase(__dup_start, __keys_.end());
    }
    __on_failure.__complete();
  }

  template <class _Self, class _Kp>
  _LIBCPP_HIDE_FROM_ABI static auto __find_impl(_Self&& __self, const _Kp& __key) {
    auto __it   = __self.lower_bound(__key);
    auto __last = __self.end();
    if (__it == __last || __self.__compare_(__key, *__it)) {
      return __last;
    }
    return __it;
  }

  template <class _Self, class _Kp>
  _LIBCPP_HIDE_FROM_ABI static auto __equal_range_impl(_Self&& __self, const _Kp& __key) {
    using __iter = _If<is_const_v<__libcpp_remove_reference_t<_Self>>, const_iterator, iterator>;
    auto __it    = std::lower_bound(__self.__keys_.begin(), __self.__keys_.end(), __key, __self.__compare_);
    auto __last  = __self.__keys_.end();
    if (__it == __last || __self.__compare_(__key, *__it)) {
      return std::make_pair(__iter(__it), __iter(__it));
    }
    return std::make_pair(__iter(__it), __iter(std::next(__it)));
  }

  template <class _Kp>
  _LIBCPP_HIDE_FROM_ABI pair<iterator, bool> __emplace(_Kp&& __key) {
    auto __it = lower_bound(__key);
    if (__it == end() || __compare_(__key, *__it)) {
      return pair<iterator, bool>(__flat_set_utils::__emplace_exact_pos(*this, __it, std::forward<_Kp>(__key)), true);
    } else {
      return pair<iterator, bool>(std::move(__it), false);
    }
  }

  template <class _Kp>
  _LIBCPP_HIDE_FROM_ABI bool __is_hint_correct(const_iterator __hint, _Kp&& __key) {
    if (__hint != cbegin() && !__compare_(*std::prev(__hint), __key)) {
      return false;
    }
    if (__hint != cend() && __compare_(*__hint, __key)) {
      return false;
    }
    return true;
  }

  template <class _Kp>
  _LIBCPP_HIDE_FROM_ABI iterator __emplace_hint(const_iterator __hint, _Kp&& __key) {
    if (__is_hint_correct(__hint, __key)) {
      if (__hint == cend() || __compare_(__key, *__hint)) {
        return __flat_set_utils::__emplace_exact_pos(*this, __hint, std::forward<_Kp>(__key));
      } else {
        // we already have an equal key
        return __hint;
      }
    } else {
      return __emplace(std::forward<_Kp>(__key)).first;
    }
  }

  _LIBCPP_HIDE_FROM_ABI void __reserve(size_t __size) {
    if constexpr (requires { __keys_.reserve(__size); }) {
      __keys_.reserve(__size);
    }
  }

  template <class _Key2, class _Compare2, class _KeyContainer2, class _Predicate>
  friend typename flat_set<_Key2, _Compare2, _KeyContainer2>::size_type
  erase_if(flat_set<_Key2, _Compare2, _KeyContainer2>&, _Predicate);

  _KeyContainer __keys_;
  _LIBCPP_NO_UNIQUE_ADDRESS key_compare __compare_;

  struct __key_equiv {
    _LIBCPP_HIDE_FROM_ABI __key_equiv(key_compare __c) : __comp_(__c) {}
    _LIBCPP_HIDE_FROM_ABI bool operator()(const_reference __x, const_reference __y) const {
      return !__comp_(__x, __y) && !__comp_(__y, __x);
    }
    key_compare __comp_;
  };
};

template <class _KeyContainer, class _Compare = less<typename _KeyContainer::value_type>>
  requires(!__is_allocator<_Compare>::value && !__is_allocator<_KeyContainer>::value &&
           is_invocable_v<const _Compare&,
                          const typename _KeyContainer::value_type&,
                          const typename _KeyContainer::value_type&>)
flat_set(_KeyContainer, _Compare = _Compare()) -> flat_set<typename _KeyContainer::value_type, _Compare, _KeyContainer>;

template <class _KeyContainer, class _Allocator>
  requires(uses_allocator_v<_KeyContainer, _Allocator> && !__is_allocator<_KeyContainer>::value)
flat_set(_KeyContainer, _Allocator)
    -> flat_set<typename _KeyContainer::value_type, less<typename _KeyContainer::value_type>, _KeyContainer>;

template <class _KeyContainer, class _Compare, class _Allocator>
  requires(!__is_allocator<_Compare>::value && !__is_allocator<_KeyContainer>::value &&
           uses_allocator_v<_KeyContainer, _Allocator> &&
           is_invocable_v<const _Compare&,
                          const typename _KeyContainer::value_type&,
                          const typename _KeyContainer::value_type&>)
flat_set(_KeyContainer, _Compare, _Allocator) -> flat_set<typename _KeyContainer::value_type, _Compare, _KeyContainer>;

template <class _KeyContainer, class _Compare = less<typename _KeyContainer::value_type>>
  requires(!__is_allocator<_Compare>::value && !__is_allocator<_KeyContainer>::value &&
           is_invocable_v<const _Compare&,
                          const typename _KeyContainer::value_type&,
                          const typename _KeyContainer::value_type&>)
flat_set(sorted_unique_t, _KeyContainer, _Compare = _Compare())
    -> flat_set<typename _KeyContainer::value_type, _Compare, _KeyContainer>;

template <class _KeyContainer, class _Allocator>
  requires(uses_allocator_v<_KeyContainer, _Allocator> && !__is_allocator<_KeyContainer>::value)
flat_set(sorted_unique_t, _KeyContainer, _Allocator)
    -> flat_set<typename _KeyContainer::value_type, less<typename _KeyContainer::value_type>, _KeyContainer>;

template <class _KeyContainer, class _Compare, class _Allocator>
  requires(!__is_allocator<_Compare>::value && !__is_allocator<_KeyContainer>::value &&
           uses_allocator_v<_KeyContainer, _Allocator> &&
           is_invocable_v<const _Compare&,
                          const typename _KeyContainer::value_type&,
                          const typename _KeyContainer::value_type&>)
flat_set(sorted_unique_t, _KeyContainer, _Compare, _Allocator)
    -> flat_set<typename _KeyContainer::value_type, _Compare, _KeyContainer>;

template <class _InputIterator, class _Compare = less<__iter_value_type<_InputIterator>>>
  requires(__has_input_iterator_category<_InputIterator>::value && !__is_allocator<_Compare>::value)
flat_set(_InputIterator, _InputIterator, _Compare = _Compare())
    -> flat_set<__iter_value_type<_InputIterator>, _Compare>;

template <class _InputIterator, class _Compare = less<__iter_value_type<_InputIterator>>>
  requires(__has_input_iterator_category<_InputIterator>::value && !__is_allocator<_Compare>::value)
flat_set(sorted_unique_t, _InputIterator, _InputIterator, _Compare = _Compare())
    -> flat_set<__iter_value_type<_InputIterator>, _Compare>;

template <ranges::input_range _Range,
          class _Compare   = less<ranges::range_value_t<_Range>>,
          class _Allocator = allocator<ranges::range_value_t<_Range>>,
          class            = __enable_if_t<!__is_allocator<_Compare>::value && __is_allocator<_Allocator>::value>>
flat_set(from_range_t, _Range&&, _Compare = _Compare(), _Allocator = _Allocator()) -> flat_set<
    ranges::range_value_t<_Range>,
    _Compare,
    vector<ranges::range_value_t<_Range>, __allocator_traits_rebind_t<_Allocator, ranges::range_value_t<_Range>>>>;

template <ranges::input_range _Range, class _Allocator, class = __enable_if_t<__is_allocator<_Allocator>::value>>
flat_set(from_range_t, _Range&&, _Allocator) -> flat_set<
    ranges::range_value_t<_Range>,
    less<ranges::range_value_t<_Range>>,
    vector<ranges::range_value_t<_Range>, __allocator_traits_rebind_t<_Allocator, ranges::range_value_t<_Range>>>>;

template <class _Key, class _Compare = less<_Key>>
  requires(!__is_allocator<_Compare>::value)
flat_set(initializer_list<_Key>, _Compare = _Compare()) -> flat_set<_Key, _Compare>;

template <class _Key, class _Compare = less<_Key>>
  requires(!__is_allocator<_Compare>::value)
flat_set(sorted_unique_t, initializer_list<_Key>, _Compare = _Compare()) -> flat_set<_Key, _Compare>;

template <class _Key, class _Compare, class _KeyContainer, class _Allocator>
struct uses_allocator<flat_set<_Key, _Compare, _KeyContainer>, _Allocator>
    : bool_constant<uses_allocator_v<_KeyContainer, _Allocator>> {};

template <class _Key, class _Compare, class _KeyContainer, class _Predicate>
_LIBCPP_HIDE_FROM_ABI typename flat_set<_Key, _Compare, _KeyContainer>::size_type
erase_if(flat_set<_Key, _Compare, _KeyContainer>& __flat_set, _Predicate __pred) {
  auto __guard = std::__make_exception_guard([&] { __flat_set.clear(); });
  auto __it    = std::remove_if(__flat_set.__keys_.begin(), __flat_set.__keys_.end(), [&](const auto& __e) -> bool {
    return static_cast<bool>(__pred(__e));
  });
  auto __res   = __flat_set.__keys_.end() - __it;
  __flat_set.__keys_.erase(__it, __flat_set.__keys_.end());
  __guard.__complete();
  return __res;
}

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_STD_VER >= 23

_LIBCPP_POP_MACROS

#endif // _LIBCPP___FLAT_SET_FLAT_SET_H
