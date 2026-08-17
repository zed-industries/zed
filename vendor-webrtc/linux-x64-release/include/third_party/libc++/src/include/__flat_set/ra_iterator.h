// -*- C++ -*-
//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef _LIBCPP___FLAT_SET_RA_ITERATOR_H
#define _LIBCPP___FLAT_SET_RA_ITERATOR_H

#include "__type_traits/is_same.h"
#include <__compare/three_way_comparable.h>
#include <__config>
#include <__iterator/incrementable_traits.h>
#include <__iterator/iterator_traits.h>
#include <__type_traits/is_constructible.h>
#include <__utility/move.h>

#if !defined(_LIBCPP_HAS_NO_PRAGMA_SYSTEM_HEADER)
#  pragma GCC system_header
#endif

_LIBCPP_PUSH_MACROS
#include <__undef_macros>

#if _LIBCPP_STD_VER >= 23

_LIBCPP_BEGIN_NAMESPACE_STD

/**
 * __ra_iterator is a random access iterator that wraps an underlying iterator.
 * It also stores the underlying container type in its type so that algorithms
 * can optimize based on the underlying container type, and to avoid inadvertently
 * mixing iterators coming from different containers..
 */
template <class _Container, class _Iterator>
struct __ra_iterator {
private:
  _Iterator __iter_;

  friend _Container;

  // note: checking the concept random_access_iterator does not work for incomplete types
  static_assert(_IsSame<typename iterator_traits<_Iterator>::iterator_category, random_access_iterator_tag>::value,
                "Underlying iterator must be a random access iterator");

public:
  using iterator_concept  = random_access_iterator_tag; // deliberately lower contiguous_iterator
  using iterator_category = random_access_iterator_tag;
  using value_type        = iter_value_t<_Iterator>;
  using difference_type   = iter_difference_t<_Iterator>;

  _LIBCPP_HIDE_FROM_ABI __ra_iterator()
    requires is_default_constructible_v<_Iterator>
  = default;

  _LIBCPP_HIDE_FROM_ABI explicit constexpr __ra_iterator(_Iterator __iter) : __iter_(std::move(__iter)) {}

  _LIBCPP_HIDE_FROM_ABI constexpr _Iterator __base() const noexcept(noexcept(_Iterator(__iter_))) { return __iter_; }

  _LIBCPP_HIDE_FROM_ABI constexpr decltype(auto) operator*() const { return *__iter_; }
  _LIBCPP_HIDE_FROM_ABI constexpr decltype(auto) operator->() const
    requires requires { __iter_.operator->(); }
  {
    return __iter_.operator->();
  }

  _LIBCPP_HIDE_FROM_ABI constexpr __ra_iterator& operator++() {
    ++__iter_;
    return *this;
  }

  _LIBCPP_HIDE_FROM_ABI constexpr __ra_iterator operator++(int) {
    __ra_iterator __tmp(*this);
    ++*this;
    return __tmp;
  }

  _LIBCPP_HIDE_FROM_ABI constexpr __ra_iterator& operator--() {
    --__iter_;
    return *this;
  }

  _LIBCPP_HIDE_FROM_ABI constexpr __ra_iterator operator--(int) {
    __ra_iterator __tmp(*this);
    --*this;
    return __tmp;
  }

  _LIBCPP_HIDE_FROM_ABI constexpr __ra_iterator& operator+=(difference_type __x) {
    __iter_ += __x;
    return *this;
  }

  _LIBCPP_HIDE_FROM_ABI constexpr __ra_iterator& operator-=(difference_type __x) {
    __iter_ -= __x;
    return *this;
  }

  _LIBCPP_HIDE_FROM_ABI constexpr decltype(auto) operator[](difference_type __n) const { return *(*this + __n); }

  _LIBCPP_HIDE_FROM_ABI friend constexpr bool operator==(const __ra_iterator& __x, const __ra_iterator& __y) {
    return __x.__iter_ == __y.__iter_;
  }

  _LIBCPP_HIDE_FROM_ABI friend constexpr bool operator<(const __ra_iterator& __x, const __ra_iterator& __y) {
    return __x.__iter_ < __y.__iter_;
  }

  _LIBCPP_HIDE_FROM_ABI friend constexpr bool operator>(const __ra_iterator& __x, const __ra_iterator& __y) {
    return __y < __x;
  }

  _LIBCPP_HIDE_FROM_ABI friend constexpr bool operator<=(const __ra_iterator& __x, const __ra_iterator& __y) {
    return !(__y < __x);
  }

  _LIBCPP_HIDE_FROM_ABI friend constexpr bool operator>=(const __ra_iterator& __x, const __ra_iterator& __y) {
    return !(__x < __y);
  }

  _LIBCPP_HIDE_FROM_ABI friend constexpr auto operator<=>(const __ra_iterator& __x, const __ra_iterator& __y)
    requires three_way_comparable<_Iterator>
  {
    return __x.__iter_ <=> __y.__iter_;
  }

  _LIBCPP_HIDE_FROM_ABI friend constexpr __ra_iterator operator+(const __ra_iterator& __i, difference_type __n) {
    auto __tmp = __i;
    __tmp += __n;
    return __tmp;
  }

  _LIBCPP_HIDE_FROM_ABI friend constexpr __ra_iterator operator+(difference_type __n, const __ra_iterator& __i) {
    return __i + __n;
  }

  _LIBCPP_HIDE_FROM_ABI friend constexpr __ra_iterator operator-(const __ra_iterator& __i, difference_type __n) {
    auto __tmp = __i;
    __tmp -= __n;
    return __tmp;
  }

  _LIBCPP_HIDE_FROM_ABI friend constexpr difference_type operator-(const __ra_iterator& __x, const __ra_iterator& __y) {
    return __x.__iter_ - __y.__iter_;
  }
};

_LIBCPP_END_NAMESPACE_STD

#endif // _LIBCPP_STD_VER >= 23

_LIBCPP_POP_MACROS

#endif // _LIBCPP___FLAT_SET_RA_ITERATOR_H
