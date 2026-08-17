// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___FLAT_MAP_FLAT_MULTIMAP_H
#define _LIBCPP___FLAT_MAP_FLAT_MULTIMAP_H

#include <__algorithm/lexicographical_compare_three_way.h>
#include <__algorithm/min.h>
#include <__algorithm/ranges_equal.h>
#include <__algorithm/ranges_equal_range.h>
#include <__algorithm/ranges_inplace_merge.h>
#include <__algorithm/ranges_is_sorted.h>
#include <__algorithm/ranges_lower_bound.h>
#include <__algorithm/ranges_partition_point.h>
#include <__algorithm/ranges_sort.h>
#include <__algorithm/ranges_unique.h>
#include <__algorithm/ranges_upper_bound.h>
#include <__algorithm/remove_if.h>
#include <__assert>
#include <__compare/synth_three_way.h>
#include <__concepts/convertible_to.h>
#include <__concepts/swappable.h>
#include <__config>
#include <__cstddef/byte.h>
#include <__cstddef/ptrdiff_t.h>
#include <__flat_map/key_value_iterator.h>
#include <__flat_map/sorted_equivalent.h>
#include <__flat_map/utils.h>
#include <__functional/invoke.h>
#include <__functional/is_transparent.h>
#include <__functional/operations.h>
#include <__fwd/vector.h>
#include <__iterator/concepts.h>
#include <__iterator/distance.h>
#include <__iterator/iterator_traits.h>
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
#include <__ranges/zip_view.h>
#include <__type_traits/conjunction.h>
#include <__type_traits/container_traits.h>
#include <__type_traits/invoke.h>
#include <__type_traits/is_allocator.h>
#include <__type_traits/is_nothrow_constructible.h>
#include <__type_traits/is_same.h>
#include <__type_traits/maybe_const.h>
#include <__utility/exception_guard.h>
#include <__utility/move.h>
#include <__utility/pair.h>
#include <__utility/scope_guard.h>
#include <__vector/vector.h>
#include <initializer_list>
#include <stdexcept>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

#if _LIBCPP_STD_VER >= 23

_LIBCPP_BEGIN_NAMESPACE_STD

template <class _Key,
          class _Tp,
          class _Compare         = less<_Key>,
          class _KeyContainer    = vector<_Key>,
          class _MappedContainer = vector<_Tp>>
class flat_multimap {
  template <class, class, class, class, class>
  friend class flat_multimap;

  static_assert(is_same_v<_Key, typename _KeyContainer::value_type>);
  static_assert(is_same_v<_Tp, typename _MappedContainer::value_type>);
  static_assert(!is_same_v<_KeyContainer, std::vector<bool>>, "vector<bool> is not a sequence container");
  static_assert(!is_same_v<_MappedContainer, std::vector<bool>>, "vector<bool> is not a sequence container");

  template <bool _Const>
  using __iterator _LIBCPP_NODEBUG = __key_value_iterator<flat_multimap, _KeyContainer, _MappedContainer, _Const>;

public:
  // types
  using key_type               = _Key;
  using mapped_type            = _Tp;
  using value_type             = pair<key_type, mapped_type>;
  using key_compare            = __type_identity_t<_Compare>;
  using reference              = pair<const key_type&, mapped_type&>;
  using const_reference        = pair<const key_type&, const mapped_type&>;
  using size_type              = size_t;
  using difference_type        = ptrdiff_t;
  using iterator               = __iterator<false>; // see [container.requirements]
  using const_iterator         = __iterator<true>;  // see [container.requirements]
  using reverse_iterator       = std::reverse_iterator<iterator>;
  using const_reverse_iterator = std::reverse_iterator<const_iterator>;
  using key_container_type     = _KeyContainer;
  using mapped_container_type  = _MappedContainer;

  class value_compare {
  private:
    key_compare __comp_;
    _LIBCPP_HIDE_FROM_ABI value_compare(key_compare __c) : __comp_(__c) {}
    friend flat_multimap;

  public:
    _LIBCPP_HIDE_FROM_ABI bool operator()(const_reference __x, const_reference __y) const {
      return __comp_(__x.first, __y.first);
    }
  };

  struct containers {
    key_container_type keys;
    mapped_container_type values;
  };

private:
  template <class _Allocator>
  _LIBCPP_HIDE_FROM_ABI static constexpr bool __allocator_ctor_constraint =
      _And<uses_allocator<key_container_type, _Allocator>, uses_allocator<mapped_container_type, _Allocator>>::value;

  _LIBCPP_HIDE_FROM_ABI static constexpr bool __is_compare_transparent = __is_transparent_v<_Compare>;

public:
  // [flat.map.cons], construct/copy/destroy
  _LIBCPP_HIDE_FROM_ABI flat_multimap() noexcept(
      is_nothrow_default_constructible_v<_KeyContainer> && is_nothrow_default_constructible_v<_MappedContainer> &&
      is_nothrow_default_constructible_v<_Compare>)
      : __containers_(), __compare_() {}

  _LIBCPP_HIDE_FROM_ABI flat_multimap(const flat_multimap&) = default;

  // The copy/move constructors are not specified in the spec, which means they should be defaulted.
  // However, the move constructor can potentially leave a moved-from object in an inconsistent
  // state if an exception is thrown.
  _LIBCPP_HIDE_FROM_ABI flat_multimap(flat_multimap&& __other) noexcept(
      is_nothrow_move_constructible_v<_KeyContainer> && is_nothrow_move_constructible_v<_MappedContainer> &&
      is_nothrow_move_constructible_v<_Compare>)
#  if _LIBCPP_HAS_EXCEPTIONS
      try
#  endif // _LIBCPP_HAS_EXCEPTIONS
      : __containers_(std::move(__other.__containers_)), __compare_(std::move(__other.__compare_)) {
    __other.clear();
#  if _LIBCPP_HAS_EXCEPTIONS
  } catch (...) {
    __other.clear();
    // gcc does not like the `throw` keyword in a conditionally noexcept function
    if constexpr (!(is_nothrow_move_constructible_v<_KeyContainer> &&
                    is_nothrow_move_constructible_v<_MappedContainer> && is_nothrow_move_constructible_v<_Compare>)) {
      throw;
    }
#  endif // _LIBCPP_HAS_EXCEPTIONS
  }

  template <class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI flat_multimap(const flat_multimap& __other, const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_tag{},
                      __alloc,
                      __other.__containers_.keys,
                      __other.__containers_.values,
                      __other.__compare_) {}

  template <class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI flat_multimap(flat_multimap&& __other, const _Allocator& __alloc)
#  if _LIBCPP_HAS_EXCEPTIONS
      try
#  endif // _LIBCPP_HAS_EXCEPTIONS
      : flat_multimap(__ctor_uses_allocator_tag{},
                      __alloc,
                      std::move(__other.__containers_.keys),
                      std::move(__other.__containers_.values),
                      std::move(__other.__compare_)) {
    __other.clear();
#  if _LIBCPP_HAS_EXCEPTIONS
  } catch (...) {
    __other.clear();
    throw;
#  endif // _LIBCPP_HAS_EXCEPTIONS
  }

  _LIBCPP_HIDE_FROM_ABI flat_multimap(
      key_container_type __key_cont, mapped_container_type __mapped_cont, const key_compare& __comp = key_compare())
      : __containers_{.keys = std::move(__key_cont), .values = std::move(__mapped_cont)}, __compare_(__comp) {
    _LIBCPP_ASSERT_VALID_INPUT_RANGE(__containers_.keys.size() == __containers_.values.size(),
                                     "flat_multimap keys and mapped containers have different size");
    __sort();
  }

  template <class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI flat_multimap(
      const key_container_type& __key_cont, const mapped_container_type& __mapped_cont, const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_tag{}, __alloc, __key_cont, __mapped_cont) {
    _LIBCPP_ASSERT_VALID_INPUT_RANGE(__containers_.keys.size() == __containers_.values.size(),
                                     "flat_multimap keys and mapped containers have different size");
    __sort();
  }

  template <class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI
  flat_multimap(const key_container_type& __key_cont,
                const mapped_container_type& __mapped_cont,
                const key_compare& __comp,
                const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_tag{}, __alloc, __key_cont, __mapped_cont, __comp) {
    _LIBCPP_ASSERT_VALID_INPUT_RANGE(__containers_.keys.size() == __containers_.values.size(),
                                     "flat_multimap keys and mapped containers have different size");
    __sort();
  }

  _LIBCPP_HIDE_FROM_ABI
  flat_multimap(sorted_equivalent_t,
                key_container_type __key_cont,
                mapped_container_type __mapped_cont,
                const key_compare& __comp = key_compare())
      : __containers_{.keys = std::move(__key_cont), .values = std::move(__mapped_cont)}, __compare_(__comp) {
    _LIBCPP_ASSERT_VALID_INPUT_RANGE(__containers_.keys.size() == __containers_.values.size(),
                                     "flat_multimap keys and mapped containers have different size");
    _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(__is_sorted(__containers_.keys), "Key container is not sorted");
  }

  template <class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI
  flat_multimap(sorted_equivalent_t,
                const key_container_type& __key_cont,
                const mapped_container_type& __mapped_cont,
                const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_tag{}, __alloc, __key_cont, __mapped_cont) {
    _LIBCPP_ASSERT_VALID_INPUT_RANGE(__containers_.keys.size() == __containers_.values.size(),
                                     "flat_multimap keys and mapped containers have different size");
    _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(__is_sorted(__containers_.keys), "Key container is not sorted");
  }

  template <class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI
  flat_multimap(sorted_equivalent_t,
                const key_container_type& __key_cont,
                const mapped_container_type& __mapped_cont,
                const key_compare& __comp,
                const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_tag{}, __alloc, __key_cont, __mapped_cont, __comp) {
    _LIBCPP_ASSERT_VALID_INPUT_RANGE(__containers_.keys.size() == __containers_.values.size(),
                                     "flat_multimap keys and mapped containers have different size");
    _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(__is_sorted(__containers_.keys), "Key container is not sorted");
  }

  _LIBCPP_HIDE_FROM_ABI explicit flat_multimap(const key_compare& __comp) : __containers_(), __compare_(__comp) {}

  template <class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI flat_multimap(const key_compare& __comp, const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_empty_tag{}, __alloc, __comp) {}

  template <class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI explicit flat_multimap(const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_empty_tag{}, __alloc) {}

  template <class _InputIterator>
    requires __has_input_iterator_category<_InputIterator>::value
  _LIBCPP_HIDE_FROM_ABI
  flat_multimap(_InputIterator __first, _InputIterator __last, const key_compare& __comp = key_compare())
      : __containers_(), __compare_(__comp) {
    insert(__first, __last);
  }

  template <class _InputIterator, class _Allocator>
    requires(__has_input_iterator_category<_InputIterator>::value && __allocator_ctor_constraint<_Allocator>)
  _LIBCPP_HIDE_FROM_ABI
  flat_multimap(_InputIterator __first, _InputIterator __last, const key_compare& __comp, const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_empty_tag{}, __alloc, __comp) {
    insert(__first, __last);
  }

  template <class _InputIterator, class _Allocator>
    requires(__has_input_iterator_category<_InputIterator>::value && __allocator_ctor_constraint<_Allocator>)
  _LIBCPP_HIDE_FROM_ABI flat_multimap(_InputIterator __first, _InputIterator __last, const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_empty_tag{}, __alloc) {
    insert(__first, __last);
  }

  template <_ContainerCompatibleRange<value_type> _Range>
  _LIBCPP_HIDE_FROM_ABI flat_multimap(from_range_t __fr, _Range&& __rg)
      : flat_multimap(__fr, std::forward<_Range>(__rg), key_compare()) {}

  template <_ContainerCompatibleRange<value_type> _Range, class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI flat_multimap(from_range_t, _Range&& __rg, const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_empty_tag{}, __alloc) {
    insert_range(std::forward<_Range>(__rg));
  }

  template <_ContainerCompatibleRange<value_type> _Range>
  _LIBCPP_HIDE_FROM_ABI flat_multimap(from_range_t, _Range&& __rg, const key_compare& __comp) : flat_multimap(__comp) {
    insert_range(std::forward<_Range>(__rg));
  }

  template <_ContainerCompatibleRange<value_type> _Range, class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI flat_multimap(from_range_t, _Range&& __rg, const key_compare& __comp, const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_empty_tag{}, __alloc, __comp) {
    insert_range(std::forward<_Range>(__rg));
  }

  template <class _InputIterator>
    requires __has_input_iterator_category<_InputIterator>::value
  _LIBCPP_HIDE_FROM_ABI flat_multimap(
      sorted_equivalent_t, _InputIterator __first, _InputIterator __last, const key_compare& __comp = key_compare())
      : __containers_(), __compare_(__comp) {
    insert(sorted_equivalent, __first, __last);
  }
  template <class _InputIterator, class _Allocator>
    requires(__has_input_iterator_category<_InputIterator>::value && __allocator_ctor_constraint<_Allocator>)
  _LIBCPP_HIDE_FROM_ABI
  flat_multimap(sorted_equivalent_t,
                _InputIterator __first,
                _InputIterator __last,
                const key_compare& __comp,
                const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_empty_tag{}, __alloc, __comp) {
    insert(sorted_equivalent, __first, __last);
  }

  template <class _InputIterator, class _Allocator>
    requires(__has_input_iterator_category<_InputIterator>::value && __allocator_ctor_constraint<_Allocator>)
  _LIBCPP_HIDE_FROM_ABI
  flat_multimap(sorted_equivalent_t, _InputIterator __first, _InputIterator __last, const _Allocator& __alloc)
      : flat_multimap(__ctor_uses_allocator_empty_tag{}, __alloc) {
    insert(sorted_equivalent, __first, __last);
  }

  _LIBCPP_HIDE_FROM_ABI flat_multimap(initializer_list<value_type> __il, const key_compare& __comp = key_compare())
      : flat_multimap(__il.begin(), __il.end(), __comp) {}

  template <class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI
  flat_multimap(initializer_list<value_type> __il, const key_compare& __comp, const _Allocator& __alloc)
      : flat_multimap(__il.begin(), __il.end(), __comp, __alloc) {}

  template <class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI flat_multimap(initializer_list<value_type> __il, const _Allocator& __alloc)
      : flat_multimap(__il.begin(), __il.end(), __alloc) {}

  _LIBCPP_HIDE_FROM_ABI
  flat_multimap(sorted_equivalent_t, initializer_list<value_type> __il, const key_compare& __comp = key_compare())
      : flat_multimap(sorted_equivalent, __il.begin(), __il.end(), __comp) {}

  template <class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI flat_multimap(
      sorted_equivalent_t, initializer_list<value_type> __il, const key_compare& __comp, const _Allocator& __alloc)
      : flat_multimap(sorted_equivalent, __il.begin(), __il.end(), __comp, __alloc) {}

  template <class _Allocator>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI flat_multimap(sorted_equivalent_t, initializer_list<value_type> __il, const _Allocator& __alloc)
      : flat_multimap(sorted_equivalent, __il.begin(), __il.end(), __alloc) {}

  _LIBCPP_HIDE_FROM_ABI flat_multimap& operator=(initializer_list<value_type> __il) {
    clear();
    insert(__il);
    return *this;
  }

  // copy/move assignment are not specified in the spec (defaulted)
  // but move assignment can potentially leave moved from object in an inconsistent
  // state if an exception is thrown
  _LIBCPP_HIDE_FROM_ABI flat_multimap& operator=(const flat_multimap&) = default;

  _LIBCPP_HIDE_FROM_ABI flat_multimap& operator=(flat_multimap&& __other) noexcept(
      is_nothrow_move_assignable_v<_KeyContainer> && is_nothrow_move_assignable_v<_MappedContainer> &&
      is_nothrow_move_assignable_v<_Compare>) {
    auto __clear_other_guard = std::__make_scope_guard([&]() noexcept { __other.clear() /* noexcept */; });
    auto __clear_self_guard  = std::__make_exception_guard([&]() noexcept { clear() /* noexcept */; });
    __containers_            = std::move(__other.__containers_);
    __compare_               = std::move(__other.__compare_);
    __clear_self_guard.__complete();
    return *this;
  }

  // iterators
  _LIBCPP_HIDE_FROM_ABI iterator begin() noexcept {
    return iterator(__containers_.keys.begin(), __containers_.values.begin());
  }

  _LIBCPP_HIDE_FROM_ABI const_iterator begin() const noexcept {
    return const_iterator(__containers_.keys.begin(), __containers_.values.begin());
  }

  _LIBCPP_HIDE_FROM_ABI iterator end() noexcept {
    return iterator(__containers_.keys.end(), __containers_.values.end());
  }

  _LIBCPP_HIDE_FROM_ABI const_iterator end() const noexcept {
    return const_iterator(__containers_.keys.end(), __containers_.values.end());
  }

  _LIBCPP_HIDE_FROM_ABI reverse_iterator rbegin() noexcept { return reverse_iterator(end()); }
  _LIBCPP_HIDE_FROM_ABI const_reverse_iterator rbegin() const noexcept { return const_reverse_iterator(end()); }
  _LIBCPP_HIDE_FROM_ABI reverse_iterator rend() noexcept { return reverse_iterator(begin()); }
  _LIBCPP_HIDE_FROM_ABI const_reverse_iterator rend() const noexcept { return const_reverse_iterator(begin()); }

  _LIBCPP_HIDE_FROM_ABI const_iterator cbegin() const noexcept { return begin(); }
  _LIBCPP_HIDE_FROM_ABI const_iterator cend() const noexcept { return end(); }
  _LIBCPP_HIDE_FROM_ABI const_reverse_iterator crbegin() const noexcept { return const_reverse_iterator(end()); }
  _LIBCPP_HIDE_FROM_ABI const_reverse_iterator crend() const noexcept { return const_reverse_iterator(begin()); }

  // [flat.map.capacity], capacity
  [[nodiscard]] _LIBCPP_HIDE_FROM_ABI bool empty() const noexcept { return __containers_.keys.empty(); }

  _LIBCPP_HIDE_FROM_ABI size_type size() const noexcept { return __containers_.keys.size(); }

  _LIBCPP_HIDE_FROM_ABI size_type max_size() const noexcept {
    return std::min<size_type>(__containers_.keys.max_size(), __containers_.values.max_size());
  }

  // [flat.map.modifiers], modifiers
  template <class... _Args>
    requires is_constructible_v<pair<key_type, mapped_type>, _Args...> && is_move_constructible_v<key_type> &&
             is_move_constructible_v<mapped_type>
  _LIBCPP_HIDE_FROM_ABI iterator emplace(_Args&&... __args) {
    std::pair<key_type, mapped_type> __pair(std::forward<_Args>(__args)...);
    auto __key_it    = ranges::upper_bound(__containers_.keys, __pair.first, __compare_);
    auto __mapped_it = __corresponding_mapped_it(*this, __key_it);

    return __flat_map_utils::__emplace_exact_pos(
        *this, std::move(__key_it), std::move(__mapped_it), std::move(__pair.first), std::move(__pair.second));
  }

  template <class... _Args>
    requires is_constructible_v<pair<key_type, mapped_type>, _Args...>
  _LIBCPP_HIDE_FROM_ABI iterator emplace_hint(const_iterator __hint, _Args&&... __args) {
    std::pair<key_type, mapped_type> __pair(std::forward<_Args>(__args)...);

    auto __prev_larger  = __hint != cbegin() && __compare_(__pair.first, (__hint - 1)->first);
    auto __next_smaller = __hint != cend() && __compare_(__hint->first, __pair.first);

    auto __hint_distance = __hint.__key_iter_ - __containers_.keys.cbegin();
    auto __key_iter      = __containers_.keys.begin() + __hint_distance;
    auto __mapped_iter   = __containers_.values.begin() + __hint_distance;

    if (!__prev_larger && !__next_smaller) [[likely]] {
      // hint correct, just use exact hint iterators
    } else if (__prev_larger && !__next_smaller) {
      // the hint position is more to the right than the key should have been.
      // we want to emplace the element to a position as right as possible
      // e.g. Insert new element "2" in the following range
      // 1, 1, 2, 2, 2, 3, 4, 6
      //                   ^
      //                   |
      //                  hint
      // We want to insert "2" after the last existing "2"
      __key_iter    = ranges::upper_bound(__containers_.keys.begin(), __key_iter, __pair.first, __compare_);
      __mapped_iter = __corresponding_mapped_it(*this, __key_iter);
    } else {
      _LIBCPP_ASSERT_INTERNAL(!__prev_larger && __next_smaller, "this means that the multimap is not sorted");

      // the hint position is more to the left than the key should have been.
      // we want to emplace the element to a position as left as possible
      //  1, 1, 2, 2, 2, 3, 4, 6
      //  ^
      //  |
      // hint
      // We want to insert "2" before the first existing "2"
      __key_iter    = ranges::lower_bound(__key_iter, __containers_.keys.end(), __pair.first, __compare_);
      __mapped_iter = __corresponding_mapped_it(*this, __key_iter);
    }
    return __flat_map_utils::__emplace_exact_pos(
        *this, __key_iter, __mapped_iter, std::move(__pair.first), std::move(__pair.second));
  }

  _LIBCPP_HIDE_FROM_ABI iterator insert(const value_type& __x) { return emplace(__x); }

  _LIBCPP_HIDE_FROM_ABI iterator insert(value_type&& __x) { return emplace(std::move(__x)); }

  _LIBCPP_HIDE_FROM_ABI iterator insert(const_iterator __hint, const value_type& __x) {
    return emplace_hint(__hint, __x);
  }

  _LIBCPP_HIDE_FROM_ABI iterator insert(const_iterator __hint, value_type&& __x) {
    return emplace_hint(__hint, std::move(__x));
  }

  template <class _PairLike>
    requires is_constructible_v<pair<key_type, mapped_type>, _PairLike>
  _LIBCPP_HIDE_FROM_ABI iterator insert(_PairLike&& __x) {
    return emplace(std::forward<_PairLike>(__x));
  }

  template <class _PairLike>
    requires is_constructible_v<pair<key_type, mapped_type>, _PairLike>
  _LIBCPP_HIDE_FROM_ABI iterator insert(const_iterator __hint, _PairLike&& __x) {
    return emplace_hint(__hint, std::forward<_PairLike>(__x));
  }

  template <class _InputIterator>
    requires __has_input_iterator_category<_InputIterator>::value
  _LIBCPP_HIDE_FROM_ABI void insert(_InputIterator __first, _InputIterator __last) {
    if constexpr (sized_sentinel_for<_InputIterator, _InputIterator>) {
      __reserve(__last - __first);
    }
    __append_sort_merge</*WasSorted = */ false>(std::move(__first), std::move(__last));
  }

  template <class _InputIterator>
    requires __has_input_iterator_category<_InputIterator>::value
  _LIBCPP_HIDE_FROM_ABI void insert(sorted_equivalent_t, _InputIterator __first, _InputIterator __last) {
    if constexpr (sized_sentinel_for<_InputIterator, _InputIterator>) {
      __reserve(__last - __first);
    }

    __append_sort_merge</*WasSorted = */ true>(std::move(__first), std::move(__last));
  }

  template <_ContainerCompatibleRange<value_type> _Range>
  _LIBCPP_HIDE_FROM_ABI void insert_range(_Range&& __range) {
    if constexpr (ranges::sized_range<_Range>) {
      __reserve(ranges::size(__range));
    }

    __append_sort_merge</*WasSorted = */ false>(ranges::begin(__range), ranges::end(__range));
  }

  _LIBCPP_HIDE_FROM_ABI void insert(initializer_list<value_type> __il) { insert(__il.begin(), __il.end()); }

  _LIBCPP_HIDE_FROM_ABI void insert(sorted_equivalent_t, initializer_list<value_type> __il) {
    insert(sorted_equivalent, __il.begin(), __il.end());
  }

  _LIBCPP_HIDE_FROM_ABI containers extract() && {
    auto __guard = std::__make_scope_guard([&]() noexcept { clear() /* noexcept */; });
    auto __ret   = std::move(__containers_);
    return __ret;
  }

  _LIBCPP_HIDE_FROM_ABI void replace(key_container_type&& __key_cont, mapped_container_type&& __mapped_cont) {
    _LIBCPP_ASSERT_VALID_INPUT_RANGE(
        __key_cont.size() == __mapped_cont.size(), "flat_multimap keys and mapped containers have different size");

    _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(__is_sorted(__key_cont), "Key container is not sorted");
    auto __guard         = std::__make_exception_guard([&]() noexcept { clear() /* noexcept */; });
    __containers_.keys   = std::move(__key_cont);
    __containers_.values = std::move(__mapped_cont);
    __guard.__complete();
  }

  _LIBCPP_HIDE_FROM_ABI iterator erase(iterator __position) {
    return __erase(__position.__key_iter_, __position.__mapped_iter_);
  }

  _LIBCPP_HIDE_FROM_ABI iterator erase(const_iterator __position) {
    return __erase(__position.__key_iter_, __position.__mapped_iter_);
  }

  _LIBCPP_HIDE_FROM_ABI size_type erase(const key_type& __x) {
    auto [__first, __last] = equal_range(__x);
    auto __res             = __last - __first;
    erase(__first, __last);
    return __res;
  }

  template <class _Kp>
    requires(__is_compare_transparent && !is_convertible_v<_Kp &&, iterator> &&
             !is_convertible_v<_Kp &&, const_iterator>)
  _LIBCPP_HIDE_FROM_ABI size_type erase(_Kp&& __x) {
    auto [__first, __last] = equal_range(__x);
    auto __res             = __last - __first;
    erase(__first, __last);
    return __res;
  }

  _LIBCPP_HIDE_FROM_ABI iterator erase(const_iterator __first, const_iterator __last) {
    auto __on_failure = std::__make_exception_guard([&]() noexcept { clear() /* noexcept */; });
    auto __key_it     = __containers_.keys.erase(__first.__key_iter_, __last.__key_iter_);
    auto __mapped_it  = __containers_.values.erase(__first.__mapped_iter_, __last.__mapped_iter_);
    __on_failure.__complete();
    return iterator(std::move(__key_it), std::move(__mapped_it));
  }

  _LIBCPP_HIDE_FROM_ABI void swap(flat_multimap& __y) noexcept {
    // warning: The spec has unconditional noexcept, which means that
    // if any of the following functions throw an exception,
    // std::terminate will be called
    ranges::swap(__compare_, __y.__compare_);
    ranges::swap(__containers_.keys, __y.__containers_.keys);
    ranges::swap(__containers_.values, __y.__containers_.values);
  }

  _LIBCPP_HIDE_FROM_ABI void clear() noexcept {
    __containers_.keys.clear();
    __containers_.values.clear();
  }

  // observers
  _LIBCPP_HIDE_FROM_ABI key_compare key_comp() const { return __compare_; }
  _LIBCPP_HIDE_FROM_ABI value_compare value_comp() const { return value_compare(__compare_); }

  _LIBCPP_HIDE_FROM_ABI const key_container_type& keys() const noexcept { return __containers_.keys; }
  _LIBCPP_HIDE_FROM_ABI const mapped_container_type& values() const noexcept { return __containers_.values; }

  // map operations
  _LIBCPP_HIDE_FROM_ABI iterator find(const key_type& __x) { return __find_impl(*this, __x); }

  _LIBCPP_HIDE_FROM_ABI const_iterator find(const key_type& __x) const { return __find_impl(*this, __x); }

  template <class _Kp>
    requires __is_compare_transparent
  _LIBCPP_HIDE_FROM_ABI iterator find(const _Kp& __x) {
    return __find_impl(*this, __x);
  }

  template <class _Kp>
    requires __is_compare_transparent
  _LIBCPP_HIDE_FROM_ABI const_iterator find(const _Kp& __x) const {
    return __find_impl(*this, __x);
  }

  _LIBCPP_HIDE_FROM_ABI size_type count(const key_type& __x) const {
    auto [__first, __last] = equal_range(__x);
    return __last - __first;
  }

  template <class _Kp>
    requires __is_compare_transparent
  _LIBCPP_HIDE_FROM_ABI size_type count(const _Kp& __x) const {
    auto [__first, __last] = equal_range(__x);
    return __last - __first;
  }

  _LIBCPP_HIDE_FROM_ABI bool contains(const key_type& __x) const { return find(__x) != end(); }

  template <class _Kp>
    requires __is_compare_transparent
  _LIBCPP_HIDE_FROM_ABI bool contains(const _Kp& __x) const {
    return find(__x) != end();
  }

  _LIBCPP_HIDE_FROM_ABI iterator lower_bound(const key_type& __x) { return __lower_bound<iterator>(*this, __x); }

  _LIBCPP_HIDE_FROM_ABI const_iterator lower_bound(const key_type& __x) const {
    return __lower_bound<const_iterator>(*this, __x);
  }

  template <class _Kp>
    requires __is_compare_transparent
  _LIBCPP_HIDE_FROM_ABI iterator lower_bound(const _Kp& __x) {
    return __lower_bound<iterator>(*this, __x);
  }

  template <class _Kp>
    requires __is_compare_transparent
  _LIBCPP_HIDE_FROM_ABI const_iterator lower_bound(const _Kp& __x) const {
    return __lower_bound<const_iterator>(*this, __x);
  }

  _LIBCPP_HIDE_FROM_ABI iterator upper_bound(const key_type& __x) { return __upper_bound<iterator>(*this, __x); }

  _LIBCPP_HIDE_FROM_ABI const_iterator upper_bound(const key_type& __x) const {
    return __upper_bound<const_iterator>(*this, __x);
  }

  template <class _Kp>
    requires __is_compare_transparent
  _LIBCPP_HIDE_FROM_ABI iterator upper_bound(const _Kp& __x) {
    return __upper_bound<iterator>(*this, __x);
  }

  template <class _Kp>
    requires __is_compare_transparent
  _LIBCPP_HIDE_FROM_ABI const_iterator upper_bound(const _Kp& __x) const {
    return __upper_bound<const_iterator>(*this, __x);
  }

  _LIBCPP_HIDE_FROM_ABI pair<iterator, iterator> equal_range(const key_type& __x) {
    return __equal_range_impl(*this, __x);
  }

  _LIBCPP_HIDE_FROM_ABI pair<const_iterator, const_iterator> equal_range(const key_type& __x) const {
    return __equal_range_impl(*this, __x);
  }

  template <class _Kp>
    requires __is_compare_transparent
  _LIBCPP_HIDE_FROM_ABI pair<iterator, iterator> equal_range(const _Kp& __x) {
    return __equal_range_impl(*this, __x);
  }
  template <class _Kp>
    requires __is_compare_transparent
  _LIBCPP_HIDE_FROM_ABI pair<const_iterator, const_iterator> equal_range(const _Kp& __x) const {
    return __equal_range_impl(*this, __x);
  }

  friend _LIBCPP_HIDE_FROM_ABI bool operator==(const flat_multimap& __x, const flat_multimap& __y) {
    return ranges::equal(__x, __y);
  }

  friend _LIBCPP_HIDE_FROM_ABI auto operator<=>(const flat_multimap& __x, const flat_multimap& __y) {
    return std::lexicographical_compare_three_way(
        __x.begin(), __x.end(), __y.begin(), __y.end(), std::__synth_three_way);
  }

  friend _LIBCPP_HIDE_FROM_ABI void swap(flat_multimap& __x, flat_multimap& __y) noexcept { __x.swap(__y); }

private:
  struct __ctor_uses_allocator_tag {
    explicit _LIBCPP_HIDE_FROM_ABI __ctor_uses_allocator_tag() = default;
  };
  struct __ctor_uses_allocator_empty_tag {
    explicit _LIBCPP_HIDE_FROM_ABI __ctor_uses_allocator_empty_tag() = default;
  };

  template <class _Allocator, class _KeyCont, class _MappedCont, class... _CompArg>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI
  flat_multimap(__ctor_uses_allocator_tag,
                const _Allocator& __alloc,
                _KeyCont&& __key_cont,
                _MappedCont&& __mapped_cont,
                _CompArg&&... __comp)
      : __containers_{.keys = std::make_obj_using_allocator<key_container_type>(
                          __alloc, std::forward<_KeyCont>(__key_cont)),
                      .values = std::make_obj_using_allocator<mapped_container_type>(
                          __alloc, std::forward<_MappedCont>(__mapped_cont))},
        __compare_(std::forward<_CompArg>(__comp)...) {}

  template <class _Allocator, class... _CompArg>
    requires __allocator_ctor_constraint<_Allocator>
  _LIBCPP_HIDE_FROM_ABI flat_multimap(__ctor_uses_allocator_empty_tag, const _Allocator& __alloc, _CompArg&&... __comp)
      : __containers_{.keys   = std::make_obj_using_allocator<key_container_type>(__alloc),
                      .values = std::make_obj_using_allocator<mapped_container_type>(__alloc)},
        __compare_(std::forward<_CompArg>(__comp)...) {}

  _LIBCPP_HIDE_FROM_ABI bool __is_sorted(auto&& __key_container) const {
    return ranges::is_sorted(__key_container, __compare_);
  }

  _LIBCPP_HIDE_FROM_ABI void __sort() {
    auto __zv = ranges::views::zip(__containers_.keys, __containers_.values);
    ranges::sort(__zv, __compare_, [](const auto& __p) -> decltype(auto) { return std::get<0>(__p); });
  }

  template <class _Self, class _KeyIter>
  _LIBCPP_HIDE_FROM_ABI static auto __corresponding_mapped_it(_Self&& __self, _KeyIter&& __key_iter) {
    return __self.__containers_.values.begin() +
           static_cast<ranges::range_difference_t<mapped_container_type>>(
               ranges::distance(__self.__containers_.keys.begin(), __key_iter));
  }

  template <bool _WasSorted, class _InputIterator, class _Sentinel>
  _LIBCPP_HIDE_FROM_ABI void __append_sort_merge(_InputIterator __first, _Sentinel __last) {
    auto __on_failure     = std::__make_exception_guard([&]() noexcept { clear() /* noexcept */; });
    size_t __num_appended = __flat_map_utils::__append(*this, std::move(__first), std::move(__last));
    if (__num_appended != 0) {
      auto __zv                  = ranges::views::zip(__containers_.keys, __containers_.values);
      auto __append_start_offset = __containers_.keys.size() - __num_appended;
      auto __end                 = __zv.end();
      auto __compare_key         = [this](const auto& __p1, const auto& __p2) {
        return __compare_(std::get<0>(__p1), std::get<0>(__p2));
      };
      if constexpr (!_WasSorted) {
        ranges::sort(__zv.begin() + __append_start_offset, __end, __compare_key);
      } else {
        _LIBCPP_ASSERT_SEMANTIC_REQUIREMENT(
            __is_sorted(__containers_.keys | ranges::views::drop(__append_start_offset)),
            "Key container is not sorted");
      }
      ranges::inplace_merge(__zv.begin(), __zv.begin() + __append_start_offset, __end, __compare_key);
    }
    __on_failure.__complete();
  }

  template <class _Self, class _Kp>
  _LIBCPP_HIDE_FROM_ABI static auto __find_impl(_Self&& __self, const _Kp& __key) {
    auto __it   = __self.lower_bound(__key);
    auto __last = __self.end();
    if (__it == __last || __self.__compare_(__key, __it->first)) {
      return __last;
    }
    return __it;
  }

  template <class _Self, class _Kp>
  _LIBCPP_HIDE_FROM_ABI static auto __equal_range_impl(_Self&& __self, const _Kp& __key) {
    auto [__key_first, __key_last] = ranges::equal_range(__self.__containers_.keys, __key, __self.__compare_);

    using __iterator_type = ranges::iterator_t<decltype(__self)>;
    return std::make_pair(__iterator_type(__key_first, __corresponding_mapped_it(__self, __key_first)),
                          __iterator_type(__key_last, __corresponding_mapped_it(__self, __key_last)));
  }

  template <class _Res, class _Self, class _Kp>
  _LIBCPP_HIDE_FROM_ABI static _Res __lower_bound(_Self&& __self, _Kp& __x) {
    auto __key_iter    = ranges::lower_bound(__self.__containers_.keys, __x, __self.__compare_);
    auto __mapped_iter = __corresponding_mapped_it(__self, __key_iter);
    return _Res(std::move(__key_iter), std::move(__mapped_iter));
  }

  template <class _Res, class _Self, class _Kp>
  _LIBCPP_HIDE_FROM_ABI static _Res __upper_bound(_Self&& __self, _Kp& __x) {
    auto __key_iter    = ranges::upper_bound(__self.__containers_.keys, __x, __self.__compare_);
    auto __mapped_iter = __corresponding_mapped_it(__self, __key_iter);
    return _Res(std::move(__key_iter), std::move(__mapped_iter));
  }

  _LIBCPP_HIDE_FROM_ABI void __reserve(size_t __size) {
    if constexpr (requires { __containers_.keys.reserve(__size); }) {
      __containers_.keys.reserve(__size);
    }

    if constexpr (requires { __containers_.values.reserve(__size); }) {
      __containers_.values.reserve(__size);
    }
  }

  template <class _KIter, class _MIter>
  _LIBCPP_HIDE_FROM_ABI iterator __erase(_KIter __key_iter_to_remove, _MIter __mapped_iter_to_remove) {
    auto __on_failure  = std::__make_exception_guard([&]() noexcept { clear() /* noexcept */; });
    auto __key_iter    = __containers_.keys.erase(__key_iter_to_remove);
    auto __mapped_iter = __containers_.values.erase(__mapped_iter_to_remove);
    __on_failure.__complete();
    return iterator(std::move(__key_iter), std::move(__mapped_iter));
  }

  template <class _Key2, class _Tp2, class _Compare2, class _KeyContainer2, class _MappedContainer2, class _Predicate>
  friend typename flat_multimap<_Key2, _Tp2, _Compare2, _KeyContainer2, _MappedContainer2>::size_type
  erase_if(flat_multimap<_Key2, _Tp2, _Compare2, _KeyContainer2, _MappedContainer2>&, _Predicate);

  friend __flat_map_utils;

  containers __containers_;
  _LIBCPP_NO_UNIQUE_ADDRESS key_compare __compare_;

  struct __key_equiv {
    _LIBCPP_HIDE_FROM_ABI __key_equiv(key_compare __c) : __comp_(__c) {}
    _LIBCPP_HIDE_FROM_ABI bool operator()(const_reference __x, const_reference __y) const {
      return !__comp_(std::get<0>(__x), std::get<0>(__y)) && !__comp_(std::get<0>(__y), std::get<0>(__x));
    }
    key_compare __comp_;
  };
};

template <class _KeyContainer, class _MappedContainer, class _Compare = less<typename _KeyContainer::value_type>>
  requires(!__is_allocator<_Compare>::value && !__is_allocator<_KeyContainer>::value &&
           !__is_allocator<_MappedContainer>::value &&
           is_invocable_v<const _Compare&,
                          const typename _KeyContainer::value_type&,
                          const typename _KeyContainer::value_type&>)
flat_multimap(_KeyContainer, _MappedContainer, _Compare = _Compare())
    -> flat_multimap<typename _KeyContainer::value_type,
                     typename _MappedContainer::value_type,
                     _Compare,
                     _KeyContainer,
                     _MappedContainer>;

template <class _KeyContainer, class _MappedContainer, class _Allocator>
  requires(uses_allocator_v<_KeyContainer, _Allocator> && uses_allocator_v<_MappedContainer, _Allocator> &&
           !__is_allocator<_KeyContainer>::value && !__is_allocator<_MappedContainer>::value)
flat_multimap(_KeyContainer, _MappedContainer, _Allocator)
    -> flat_multimap<typename _KeyContainer::value_type,
                     typename _MappedContainer::value_type,
                     less<typename _KeyContainer::value_type>,
                     _KeyContainer,
                     _MappedContainer>;

template <class _KeyContainer, class _MappedContainer, class _Compare, class _Allocator>
  requires(!__is_allocator<_Compare>::value && !__is_allocator<_KeyContainer>::value &&
           !__is_allocator<_MappedContainer>::value && uses_allocator_v<_KeyContainer, _Allocator> &&
           uses_allocator_v<_MappedContainer, _Allocator> &&
           is_invocable_v<const _Compare&,
                          const typename _KeyContainer::value_type&,
                          const typename _KeyContainer::value_type&>)
flat_multimap(_KeyContainer, _MappedContainer, _Compare, _Allocator)
    -> flat_multimap<typename _KeyContainer::value_type,
                     typename _MappedContainer::value_type,
                     _Compare,
                     _KeyContainer,
                     _MappedContainer>;

template <class _KeyContainer, class _MappedContainer, class _Compare = less<typename _KeyContainer::value_type>>
  requires(!__is_allocator<_Compare>::value && !__is_allocator<_KeyContainer>::value &&
           !__is_allocator<_MappedContainer>::value &&
           is_invocable_v<const _Compare&,
                          const typename _KeyContainer::value_type&,
                          const typename _KeyContainer::value_type&>)
flat_multimap(sorted_equivalent_t, _KeyContainer, _MappedContainer, _Compare = _Compare())
    -> flat_multimap<typename _KeyContainer::value_type,
                     typename _MappedContainer::value_type,
                     _Compare,
                     _KeyContainer,
                     _MappedContainer>;

template <class _KeyContainer, class _MappedContainer, class _Allocator>
  requires(uses_allocator_v<_KeyContainer, _Allocator> && uses_allocator_v<_MappedContainer, _Allocator> &&
           !__is_allocator<_KeyContainer>::value && !__is_allocator<_MappedContainer>::value)
flat_multimap(sorted_equivalent_t, _KeyContainer, _MappedContainer, _Allocator)
    -> flat_multimap<typename _KeyContainer::value_type,
                     typename _MappedContainer::value_type,
                     less<typename _KeyContainer::value_type>,
                     _KeyContainer,
                     _MappedContainer>;

template <class _KeyContainer, class _MappedContainer, class _Compare, class _Allocator>
  requires(!__is_allocator<_Compare>::value && !__is_allocator<_KeyContainer>::value &&
           !__is_allocator<_MappedContainer>::value && uses_allocator_v<_KeyContainer, _Allocator> &&
           uses_allocator_v<_MappedContainer, _Allocator> &&
           is_invocable_v<const _Compare&,
                          const typename _KeyContainer::value_type&,
                          const typename _KeyContainer::value_type&>)
flat_multimap(sorted_equivalent_t, _KeyContainer, _MappedContainer, _Compare, _Allocator)
    -> flat_multimap<typename _KeyContainer::value_type,
                     typename _MappedContainer::value_type,
                     _Compare,
                     _KeyContainer,
                     _MappedContainer>;

template <class _InputIterator, class _Compare = less<__iter_key_type<_InputIterator>>>
  requires(__has_input_iterator_category<_InputIterator>::value && !__is_allocator<_Compare>::value)
flat_multimap(_InputIterator, _InputIterator, _Compare = _Compare())
    -> flat_multimap<__iter_key_type<_InputIterator>, __iter_mapped_type<_InputIterator>, _Compare>;

template <class _InputIterator, class _Compare = less<__iter_key_type<_InputIterator>>>
  requires(__has_input_iterator_category<_InputIterator>::value && !__is_allocator<_Compare>::value)
flat_multimap(sorted_equivalent_t, _InputIterator, _InputIterator, _Compare = _Compare())
    -> flat_multimap<__iter_key_type<_InputIterator>, __iter_mapped_type<_InputIterator>, _Compare>;

template <ranges::input_range _Range,
          class _Compare   = less<__range_key_type<_Range>>,
          class _Allocator = allocator<byte>,
          class            = __enable_if_t<!__is_allocator<_Compare>::value && __is_allocator<_Allocator>::value>>
flat_multimap(from_range_t, _Range&&, _Compare = _Compare(), _Allocator = _Allocator()) -> flat_multimap<
    __range_key_type<_Range>,
    __range_mapped_type<_Range>,
    _Compare,
    vector<__range_key_type<_Range>, __allocator_traits_rebind_t<_Allocator, __range_key_type<_Range>>>,
    vector<__range_mapped_type<_Range>, __allocator_traits_rebind_t<_Allocator, __range_mapped_type<_Range>>>>;

template <ranges::input_range _Range, class _Allocator, class = __enable_if_t<__is_allocator<_Allocator>::value>>
flat_multimap(from_range_t, _Range&&, _Allocator) -> flat_multimap<
    __range_key_type<_Range>,
    __range_mapped_type<_Range>,
    less<__range_key_type<_Range>>,
    vector<__range_key_type<_Range>, __allocator_traits_rebind_t<_Allocator, __range_key_type<_Range>>>,
    vector<__range_mapped_type<_Range>, __allocator_traits_rebind_t<_Allocator, __range_mapped_type<_Range>>>>;

template <class _Key, class _Tp, class _Compare = less<_Key>>
  requires(!__is_allocator<_Compare>::value)
flat_multimap(initializer_list<pair<_Key, _Tp>>, _Compare = _Compare()) -> flat_multimap<_Key, _Tp, _Compare>;

template <class _Key, class _Tp, class _Compare = less<_Key>>
  requires(!__is_allocator<_Compare>::value)
flat_multimap(sorted_equivalent_t, initializer_list<pair<_Key, _Tp>>, _Compare = _Compare())
    -> flat_multimap<_Key, _Tp, _Compare>;

template <class _Key, class _Tp, class _Compare, class _KeyContainer, class _MappedContainer, class _Allocator>
struct uses_allocator<flat_multimap<_Key, _Tp, _Compare, _KeyContainer, _MappedContainer>, _Allocator>
    : bool_constant<uses_allocator_v<_KeyContainer, _Allocator> && uses_allocator_v<_MappedContainer, _Allocator>> {};

template <class _Key, class _Tp, class _Compare, class _KeyContainer, class _MappedContainer, class _Predicate>
_LIBCPP_HIDE_FROM_ABI typename flat_multimap<_Key, _Tp, _Compare, _KeyContainer, _MappedContainer>::size_type
erase_if(flat_multimap<_Key, _Tp, _Compare, _KeyContainer, _MappedContainer>& __flat_multimap, _Predicate __pred) {
  auto __zv     = ranges::views::zip(__flat_multimap.__containers_.keys, __flat_multimap.__containers_.values);
  auto __first  = __zv.begin();
  auto __last   = __zv.end();
  auto __guard  = std::__make_exception_guard([&] { __flat_multimap.clear(); });
  auto __it     = std::remove_if(__first, __last, [&](auto&& __zipped) -> bool {
    using _Ref = typename flat_multimap<_Key, _Tp, _Compare, _KeyContainer, _MappedContainer>::const_reference;
    return __pred(_Ref(std::get<0>(__zipped), std::get<1>(__zipped)));
  });
  auto __res    = __last - __it;
  auto __offset = __it - __first;

  const auto __erase_container = [&](auto& __cont) { __cont.erase(__cont.begin() + __offset, __cont.end()); };

  __erase_container(__flat_multimap.__containers_.keys);
  __erase_container(__flat_multimap.__containers_.values);

  __guard.__complete();
  return __res;
}

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_STD_VER >= 23

_LIBCPP_POP_MACROS

#endif // _LIBCPP___FLAT_MAP_FLAT_MULTIMAP_H
