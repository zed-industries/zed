// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___FLAT_MAP_KEY_VALUE_ITERATOR_H
#define _LIBCPP___FLAT_MAP_KEY_VALUE_ITERATOR_H

#include <__compare/three_way_comparable.h>
#include <__concepts/convertible_to.h>
#include <__config>
#include <__iterator/iterator_traits.h>
#include <__memory/addressof.h>
#include <__type_traits/conditional.h>
#include <__utility/move.h>
#include <__utility/pair.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

#if _LIBCPP_STD_VER >= 23

_LIBCPP_BEGIN_NAMESPACE_STD

/**
 * __key_value_iterator is a proxy iterator which zips the underlying
 * _KeyContainer::iterator and the underlying _MappedContainer::iterator.
 * The two underlying iterators will be incremented/decremented together.
 * And the reference is a pair of the const key reference and the value reference.
 */
template <class _Owner, class _KeyContainer, class _MappedContainer, bool _Const>
struct __key_value_iterator {
private:
  using __key_iterator _LIBCPP_NODEBUG = typename _KeyContainer::const_iterator;
  using __mapped_iterator _LIBCPP_NODEBUG =
      _If<_Const, typename _MappedContainer::const_iterator, typename _MappedContainer::iterator>;
  using __reference _LIBCPP_NODEBUG = _If<_Const, typename _Owner::const_reference, typename _Owner::reference>;

  struct __arrow_proxy {
    __reference __ref_;
    _LIBCPP_HIDE_FROM_ABI __reference* operator->() { return std::addressof(__ref_); }
  };

  __key_iterator __key_iter_;
  __mapped_iterator __mapped_iter_;

  friend _Owner;

  template <class, class, class, bool>
  friend struct __key_value_iterator;

public:
  using iterator_concept = random_access_iterator_tag;
  // `__key_value_iterator` only satisfy "Cpp17InputIterator" named requirements, because
  // its `reference` is not a reference type.
  // However, to avoid surprising runtime behaviour when it is used with the
  // Cpp17 algorithms or operations, iterator_category is set to random_access_iterator_tag.
  using iterator_category = random_access_iterator_tag;
  using value_type        = typename _Owner::value_type;
  using difference_type   = typename _Owner::difference_type;

  _LIBCPP_HIDE_FROM_ABI __key_value_iterator() = default;

  _LIBCPP_HIDE_FROM_ABI __key_value_iterator(__key_value_iterator<_Owner, _KeyContainer, _MappedContainer, !_Const> __i)
    requires _Const && convertible_to<typename _KeyContainer::iterator, __key_iterator> &&
                 convertible_to<typename _MappedContainer::iterator, __mapped_iterator>
      : __key_iter_(std::move(__i.__key_iter_)), __mapped_iter_(std::move(__i.__mapped_iter_)) {}

  _LIBCPP_HIDE_FROM_ABI __key_value_iterator(__key_iterator __key_iter, __mapped_iterator __mapped_iter)
      : __key_iter_(std::move(__key_iter)), __mapped_iter_(std::move(__mapped_iter)) {}

  _LIBCPP_HIDE_FROM_ABI __reference operator*() const { return __reference(*__key_iter_, *__mapped_iter_); }
  _LIBCPP_HIDE_FROM_ABI __arrow_proxy operator->() const { return __arrow_proxy{**this}; }

  _LIBCPP_HIDE_FROM_ABI __key_value_iterator& operator++() {
    ++__key_iter_;
    ++__mapped_iter_;
    return *this;
  }

  _LIBCPP_HIDE_FROM_ABI __key_value_iterator operator++(int) {
    __key_value_iterator __tmp(*this);
    ++*this;
    return __tmp;
  }

  _LIBCPP_HIDE_FROM_ABI __key_value_iterator& operator--() {
    --__key_iter_;
    --__mapped_iter_;
    return *this;
  }

  _LIBCPP_HIDE_FROM_ABI __key_value_iterator operator--(int) {
    __key_value_iterator __tmp(*this);
    --*this;
    return __tmp;
  }

  _LIBCPP_HIDE_FROM_ABI __key_value_iterator& operator+=(difference_type __x) {
    __key_iter_ += __x;
    __mapped_iter_ += __x;
    return *this;
  }

  _LIBCPP_HIDE_FROM_ABI __key_value_iterator& operator-=(difference_type __x) {
    __key_iter_ -= __x;
    __mapped_iter_ -= __x;
    return *this;
  }

  _LIBCPP_HIDE_FROM_ABI __reference operator[](difference_type __n) const { return *(*this + __n); }

  _LIBCPP_HIDE_FROM_ABI friend constexpr bool
  operator==(const __key_value_iterator& __x, const __key_value_iterator& __y) {
    return __x.__key_iter_ == __y.__key_iter_;
  }

  _LIBCPP_HIDE_FROM_ABI friend bool operator<(const __key_value_iterator& __x, const __key_value_iterator& __y) {
    return __x.__key_iter_ < __y.__key_iter_;
  }

  _LIBCPP_HIDE_FROM_ABI friend bool operator>(const __key_value_iterator& __x, const __key_value_iterator& __y) {
    return __y < __x;
  }

  _LIBCPP_HIDE_FROM_ABI friend bool operator<=(const __key_value_iterator& __x, const __key_value_iterator& __y) {
    return !(__y < __x);
  }

  _LIBCPP_HIDE_FROM_ABI friend bool operator>=(const __key_value_iterator& __x, const __key_value_iterator& __y) {
    return !(__x < __y);
  }

  _LIBCPP_HIDE_FROM_ABI friend auto operator<=>(const __key_value_iterator& __x, const __key_value_iterator& __y)
    requires three_way_comparable<__key_iterator>
  {
    return __x.__key_iter_ <=> __y.__key_iter_;
  }

  _LIBCPP_HIDE_FROM_ABI friend __key_value_iterator operator+(const __key_value_iterator& __i, difference_type __n) {
    auto __tmp = __i;
    __tmp += __n;
    return __tmp;
  }

  _LIBCPP_HIDE_FROM_ABI friend __key_value_iterator operator+(difference_type __n, const __key_value_iterator& __i) {
    return __i + __n;
  }

  _LIBCPP_HIDE_FROM_ABI friend __key_value_iterator operator-(const __key_value_iterator& __i, difference_type __n) {
    auto __tmp = __i;
    __tmp -= __n;
    return __tmp;
  }

  _LIBCPP_HIDE_FROM_ABI friend difference_type
  operator-(const __key_value_iterator& __x, const __key_value_iterator& __y) {
    return difference_type(__x.__key_iter_ - __y.__key_iter_);
  }
};

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_STD_VER >= 23

_LIBCPP_POP_MACROS

#endif // _LIBCPP___FLAT_MAP_KEY_VALUE_ITERATOR_H
