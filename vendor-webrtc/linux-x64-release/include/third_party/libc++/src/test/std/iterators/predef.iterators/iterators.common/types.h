//===----------------------------------------------------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef TEST_STD_RANGES_ITERATORS_PREDEF_ITERATORS_ITERATORS_COMMON_TYPES_H
#define TEST_STD_RANGES_ITERATORS_PREDEF_ITERATORS_ITERATORS_COMMON_TYPES_H

#include "test_macros.h"
#include "test_iterators.h"

template <class>
class assignable_iterator;

template <class It>
class simple_iterator
{
    It it_;

public:
    typedef          std::input_iterator_tag                   iterator_category;
    typedef typename std::iterator_traits<It>::value_type      value_type;
    typedef typename std::iterator_traits<It>::difference_type difference_type;
    typedef It                                                 pointer;
    typedef typename std::iterator_traits<It>::reference       reference;

    friend constexpr It base(const simple_iterator& i) {return i.it_;}

    simple_iterator() = default;
    explicit constexpr simple_iterator(It it) : it_(it) {}

    constexpr reference operator*() const {return *it_;}

    constexpr simple_iterator& operator++() {++it_; return *this;}
    constexpr simple_iterator operator++(int)
        {simple_iterator tmp(*this); ++(*this); return tmp;}
};

template <class It>
class value_iterator
{
    It it_;

public:
    typedef          std::input_iterator_tag                   iterator_category;
    typedef typename std::iterator_traits<It>::value_type      value_type;
    typedef typename std::iterator_traits<It>::difference_type difference_type;
    typedef It                                                 pointer;
    typedef typename std::iterator_traits<It>::reference       reference;

    friend constexpr It base(const value_iterator& i) {return i.it_;}

    value_iterator() = default;
    explicit constexpr value_iterator(It it) : it_(it) {}

    constexpr value_type operator*() const {return std::move(*it_);}

    constexpr value_iterator& operator++() {++it_; return *this;}
    constexpr value_iterator operator++(int)
        {value_iterator tmp(*this); ++(*this); return tmp;}
};

template <class It>
class void_plus_plus_iterator
{
    It it_;

public:
    typedef          std::input_iterator_tag                   iterator_category;
    typedef typename std::iterator_traits<It>::value_type      value_type;
    typedef typename std::iterator_traits<It>::difference_type difference_type;
    typedef It                                                 pointer;
    typedef typename std::iterator_traits<It>::reference       reference;

    friend constexpr It base(const void_plus_plus_iterator& i) {return i.it_;}

    void_plus_plus_iterator() = default;
    explicit constexpr void_plus_plus_iterator(It it) : it_(it) {}

    constexpr value_type operator*() const {return std::move(*it_);}

    constexpr void_plus_plus_iterator& operator++() {++it_; return *this;}
    constexpr void operator++(int) {++(*this);}
};

// Not referenceable, constructible, and not move constructible.
template <class It>
class value_type_not_move_constructible_iterator
{
    It it_;

public:
    template<class T>
    struct hold {
      T value_;
      hold(T v) : value_(v) {}
      hold(const hold&) = delete;
      hold(hold&&) = delete;
    };

    typedef          std::input_iterator_tag                   iterator_category;
    typedef typename std::iterator_traits<It>::value_type      underlying_value_type;
    typedef hold<underlying_value_type>                        value_type;
    typedef typename std::iterator_traits<It>::difference_type difference_type;
    typedef It                                                 pointer;
    typedef typename std::iterator_traits<It>::reference       reference;

    friend constexpr It base(const value_type_not_move_constructible_iterator& i) {return i.it_;}

    value_type_not_move_constructible_iterator() = default;
    explicit constexpr value_type_not_move_constructible_iterator(It it) : it_(it) {}

    constexpr underlying_value_type operator*() const {return std::move(*it_);}

    constexpr value_type_not_move_constructible_iterator& operator++() {++it_; return *this;}
    constexpr void operator++(int) {++(*this);}
};

template <class It>
class comparable_iterator
{
    It it_;

public:
    typedef          std::input_iterator_tag                   iterator_category;
    typedef typename std::iterator_traits<It>::value_type      value_type;
    typedef typename std::iterator_traits<It>::difference_type difference_type;
    typedef It                                                 pointer;
    typedef typename std::iterator_traits<It>::reference       reference;

    friend constexpr It base(const comparable_iterator& i) {return i.it_;}

    comparable_iterator() = default;
    explicit constexpr comparable_iterator(It it) : it_(it) {}

    constexpr reference operator*() const {return *it_;}

    constexpr comparable_iterator& operator++() {++it_; return *this;}
    constexpr comparable_iterator operator++(int)
        {comparable_iterator tmp(*this); ++(*this); return tmp;}

    friend constexpr bool operator==(const comparable_iterator& lhs, const simple_iterator<It>& rhs) {
      return base(lhs) == base(rhs);
    }
    friend constexpr bool operator==(const simple_iterator<It>& lhs, const comparable_iterator& rhs) {
      return base(lhs) == base(rhs);
    }

    friend constexpr auto operator-(const comparable_iterator& lhs, const simple_iterator<It>& rhs) {
      return base(lhs) - base(rhs);
    }
    friend constexpr auto operator-(const simple_iterator<It>& lhs, const comparable_iterator& rhs) {
      return base(lhs) - base(rhs);
    }
};

template<class T>
struct sentinel_type {
  T base_;

  template<class U>
  friend constexpr bool operator==(const sentinel_type& lhs, const U& rhs) { return lhs.base_ == base(rhs); }
  template<class U>
  friend constexpr bool operator==(const U& lhs, const sentinel_type& rhs) { return base(lhs) == rhs.base_; }
};

template<class T>
struct sized_sentinel_type {
  T base_;

  template<class U>
  friend constexpr bool operator==(const sized_sentinel_type& lhs, const U& rhs) { return lhs.base_ - base(rhs); }
  template<class U>
  friend constexpr bool operator==(const U& lhs, const sized_sentinel_type& rhs) { return base(lhs) - rhs.base_; }
  template<class U>
  friend constexpr auto operator- (const sized_sentinel_type& lhs, const U& rhs) { return lhs.base_ - base(rhs); }
  template<class U>
  friend constexpr auto operator- (const U& lhs, const sized_sentinel_type& rhs) { return base(lhs) - rhs.base_; }
};

template <class It>
class assignable_iterator
{
    It it_;

public:
    typedef          std::input_iterator_tag                   iterator_category;
    typedef typename std::iterator_traits<It>::value_type      value_type;
    typedef typename std::iterator_traits<It>::difference_type difference_type;
    typedef It                                                 pointer;
    typedef typename std::iterator_traits<It>::reference       reference;

    friend constexpr It base(const assignable_iterator& i) {return i.it_;}

    assignable_iterator() = default;
    explicit constexpr assignable_iterator(It it) : it_(it) {}

    assignable_iterator(const forward_iterator<It>& it) : it_(base(it)) {}
    assignable_iterator(const sentinel_type<It>& it) : it_(base(it)) {}

    constexpr reference operator*() const {return *it_;}

    constexpr assignable_iterator& operator++() {++it_; return *this;}
    constexpr assignable_iterator operator++(int)
        {assignable_iterator tmp(*this); ++(*this); return tmp;}

    assignable_iterator& operator=(const forward_iterator<It> &other) {
      it_ = base(other);
      return *this;
    }

    assignable_iterator& operator=(const sentinel_type<It> &other) {
      it_ = base(other);
      return *this;
    }
};

#ifndef TEST_HAS_NO_EXCEPTIONS
template<class T>
struct sentinel_throws_on_convert {
  T base_;

  template<class U>
  friend constexpr bool operator==(const sentinel_throws_on_convert& lhs, const U& rhs) { return lhs.base_ == base(rhs); }
  template<class U>
  friend constexpr bool operator==(const U& lhs, const sentinel_throws_on_convert& rhs) { return base(lhs) == rhs.base_; }

  operator sentinel_type<int*>() const { throw 42; }
};

template <class It>
class maybe_valueless_iterator
{
    It it_;

public:
    typedef          std::input_iterator_tag                   iterator_category;
    typedef typename std::iterator_traits<It>::value_type      value_type;
    typedef typename std::iterator_traits<It>::difference_type difference_type;
    typedef It                                                 pointer;
    typedef typename std::iterator_traits<It>::reference       reference;

    friend constexpr It base(const maybe_valueless_iterator& i) {return i.it_;}

    maybe_valueless_iterator() = default;
    explicit constexpr maybe_valueless_iterator(It it) : it_(it) {}

    maybe_valueless_iterator(const forward_iterator<It>& it) : it_(base(it)) {}

    constexpr reference operator*() const {return *it_;}

    constexpr maybe_valueless_iterator& operator++() {++it_; return *this;}
    constexpr maybe_valueless_iterator operator++(int)
        {maybe_valueless_iterator tmp(*this); ++(*this); return tmp;}

    maybe_valueless_iterator& operator=(const forward_iterator<It> &other) {
      it_ = base(other);
      return *this;
    }
};
#endif // TEST_HAS_NO_EXCEPTIONS

#endif // TEST_STD_RANGES_ITERATORS_PREDEF_ITERATORS_ITERATORS_COMMON_TYPES_H
